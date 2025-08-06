use crate::parser::Document;
use crate::site::SiteConfig;
use chrono::Utc;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generate sitemap.xml for the website
pub fn generate_sitemap(
    documents: &[Document],
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Group documents by base name to find language variants
    let document_groups = group_documents_by_base_name(documents);
    let sitemap_content = generate_sitemap_xml(documents, &document_groups, site_config)?;

    // Write sitemap file
    let sitemap_path = output_dir.join("sitemap.xml");
    let mut file = File::create(&sitemap_path)?;
    file.write_all(sitemap_content.as_bytes())?;

    Ok(())
}

/// Generate sitemap XML content
fn generate_sitemap_xml(
    documents: &[Document], 
    document_groups: &HashMap<String, Vec<&Document>>, 
    site_config: &SiteConfig
) -> Result<String, Box<dyn std::error::Error>> {
    let mut sitemap = String::new();
    
    // XML declaration and urlset opening with xhtml namespace and schema location
    sitemap.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    sitemap.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:xhtml=\"http://www.w3.org/1999/xhtml\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd\">\n");

    // Add home page entry
    if let Some(ref base_url) = site_config.base_url {
        sitemap.push_str("  <url>\n");
        sitemap.push_str(&format!("    <loc>{}</loc>\n", escape_xml_url(base_url)));
        
        // Use most recent post date or current time for home page
        let most_recent_date = documents.iter()
            .filter(|doc| should_include_in_sitemap(doc))
            .filter_map(|doc| doc.front_matter.date)
            .max()
            .unwrap_or_else(Utc::now);
        
        sitemap.push_str(&format!("    <lastmod>{}</lastmod>\n", most_recent_date.format("%Y-%m-%d")));
        sitemap.push_str("    <changefreq>weekly</changefreq>\n");
        sitemap.push_str("    <priority>1.0</priority>\n");
        sitemap.push_str("  </url>\n");
    }

    // Add document entries (one per base name, not per language)
    let mut processed_base_names: HashSet<String> = HashSet::new();
    
    for document in documents {
        if should_include_in_sitemap(document) && !processed_base_names.contains(&document.base_name) {
            processed_base_names.insert(document.base_name.clone());
            
            // Get all language variants for this base name
            if let Some(language_variants) = document_groups.get(&document.base_name) {
                sitemap.push_str(&generate_sitemap_entry_for_group(language_variants, site_config)?);
            }
        }
    }

    sitemap.push_str("</urlset>\n");

    Ok(sitemap)
}

/// Group documents by base name to find language variants
fn group_documents_by_base_name(documents: &[Document]) -> HashMap<String, Vec<&Document>> {
    let mut groups: HashMap<String, Vec<&Document>> = HashMap::new();
    
    for doc in documents {
        if should_include_in_sitemap(doc) {
            groups.entry(doc.base_name.clone()).or_default().push(doc);
        }
    }
    
    groups
}

/// Generate a single sitemap entry for a group of language variants
fn generate_sitemap_entry_for_group(
    language_variants: &[&Document], 
    site_config: &SiteConfig
) -> Result<String, Box<dyn std::error::Error>> {
    let mut entry = String::new();
    
    entry.push_str("  <url>\n");
    
    // Choose canonical document (prefer English, fall back to first available)
    let canonical_doc = language_variants.iter()
        .find(|doc| doc.language == "en")
        .unwrap_or(&language_variants[0]);
    
    // URL for canonical version
    let canonical_url = generate_document_url(canonical_doc, site_config);
    entry.push_str(&format!("    <loc>{}</loc>\n", escape_xml_url(&canonical_url)));

    // Last modification date (use most recent date across all variants)
    let most_recent_date = language_variants.iter()
        .filter_map(|doc| doc.front_matter.date)
        .max();
    if let Some(date) = most_recent_date {
        entry.push_str(&format!("    <lastmod>{}</lastmod>\n", date.format("%Y-%m-%d")));
    }

    // Change frequency and priority based on document type
    if is_post(canonical_doc) {
        entry.push_str("    <changefreq>monthly</changefreq>\n");
        entry.push_str("    <priority>0.8</priority>\n");
    } else {
        // Pages
        entry.push_str("    <changefreq>monthly</changefreq>\n");
        entry.push_str("    <priority>0.6</priority>\n");
    }

    // Add xhtml:link elements for all language versions (including canonical)
    if language_variants.len() > 1 {
        for variant in language_variants {
            let variant_url = generate_document_url(variant, site_config);
            let hreflang = map_language_to_hreflang(&variant.language);
            entry.push_str(&format!(
                "    <xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}\" />\n",
                hreflang,
                escape_xml_url(&variant_url)
            ));
        }
    }

    entry.push_str("  </url>\n");

    Ok(entry)
}

/// Generate URL for a document
fn generate_document_url(document: &Document, site_config: &SiteConfig) -> String {
    let mut path = std::path::PathBuf::from(&document.file_path);
    path.set_extension("html");
    
    if let Some(ref base_url) = site_config.base_url {
        format!("{}/{}", base_url.trim_end_matches('/'), path.to_string_lossy())
    } else {
        path.to_string_lossy().to_string()
    }
}

/// Check if document should be included in sitemap
fn should_include_in_sitemap(document: &Document) -> bool {
    // Exclude drafts
    if document.front_matter.draft.unwrap_or(false) {
        return false;
    }

    // Include all languages - each language variant should be discoverable
    true
}

/// Check if document is a post (vs page)
fn is_post(document: &Document) -> bool {
    document.front_matter.extra.get("layout").and_then(|v| v.as_str()) == Some("post") || 
    document.file_path.starts_with("posts/")
}

/// Map language codes to hreflang values
fn map_language_to_hreflang(language: &str) -> &str {
    match language {
        "en" => "en",
        "it" => "it", 
        "es" => "es",
        "fr" => "fr",
        "de" => "de",
        "pt" => "pt",
        "ja" => "ja",
        "zh" => "zh",
        "ru" => "ru",
        "ar" => "ar",
        _ => language, // Fallback to the original language code
    }
}

/// Escape XML special characters in URLs
fn escape_xml_url(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_escape_xml_url() {
        assert_eq!(escape_xml_url("https://example.com/page?a=1&b=2"), "https://example.com/page?a=1&amp;b=2");
    }

    #[test]
    fn test_should_include_in_sitemap() {
        use crate::parser::FrontMatter;
        
        let document = Document {
            file_path: "posts/test.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "test".to_string(),
        };
        assert!(should_include_in_sitemap(&document));

        // Test draft exclusion
        let draft_document = Document {
            file_path: "posts/draft.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: Some(true),
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "draft".to_string(),
        };
        assert!(!should_include_in_sitemap(&draft_document));

        // Test non-English inclusion
        let non_english_document = Document {
            file_path: "posts/test.it.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "it".to_string(),
            base_name: "test".to_string(),
        };
        assert!(should_include_in_sitemap(&non_english_document));
    }

    #[test]
    fn test_is_post() {
        use crate::parser::FrontMatter;
        
        // Test post by layout
        let mut post_extra = HashMap::new();
        post_extra.insert("layout".to_string(), serde_yaml::Value::String("post".to_string()));
        
        let post_by_layout = Document {
            file_path: "content/some-post.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra: post_extra,
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "some-post".to_string(),
        };
        assert!(is_post(&post_by_layout));

        // Test post by path
        let post_by_path = Document {
            file_path: "posts/test.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "test".to_string(),
        };
        assert!(is_post(&post_by_path));

        // Test page
        let page = Document {
            file_path: "pages/about.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "about".to_string(),
        };
        assert!(!is_post(&page));
    }
}