use crate::parser::Document;
use crate::site::SiteConfig;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generate Atom feed for blog posts
pub fn generate_feed(
    documents: &[Document],
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Filter and sort posts
    let mut posts: Vec<&Document> = documents.iter()
        .filter(|doc| is_post_for_feed(doc))
        .collect();
    
    posts.sort_by(|a, b| {
        b.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC)
            .cmp(&a.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC))
    });

    // Limit to 20 most recent posts
    posts.truncate(20);

    let feed_content = generate_atom_feed(&posts, site_config)?;

    // Write feed file
    let feed_path = output_dir.join("feed.xml");
    let mut file = File::create(&feed_path)?;
    file.write_all(feed_content.as_bytes())?;

    Ok(())
}

/// Generate Atom feed XML content
fn generate_atom_feed(posts: &[&Document], site_config: &SiteConfig) -> Result<String, Box<dyn std::error::Error>> {
    let mut feed = String::new();
    
    // XML declaration and feed opening
    feed.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    feed.push_str("<feed xmlns=\"http://www.w3.org/2005/Atom\"");
    
    // Add xml:base if base_url is configured
    if let Some(ref base_url) = site_config.base_url {
        feed.push_str(&format!(" xml:base=\"{}\"", escape_xml_url(base_url)));
    }
    
    feed.push_str(">\n");

    // Feed metadata
    feed.push_str(&format!("  <title>{}</title>\n", escape_xml(&site_config.get_site_title())));
    
    if let Some(ref base_url) = site_config.base_url {
        feed.push_str(&format!("  <link href=\"{}/feed.xml\" rel=\"self\" />\n", escape_xml_url(base_url)));
        feed.push_str(&format!("  <link href=\"{}\" />\n", escape_xml_url(base_url)));
        feed.push_str(&format!("  <id>{}</id>\n", escape_xml_url(base_url)));
    }

    // Updated time (most recent post date or current time)
    let updated = posts.first()
        .and_then(|post| post.front_matter.date)
        .unwrap_or_else(Utc::now);
    feed.push_str(&format!("  <updated>{}</updated>\n", updated.to_rfc3339()));

    // Generator
    feed.push_str("  <generator uri=\"https://github.com/mcaserta/krik\">Krik</generator>\n");

    // Feed entries
    for post in posts {
        feed.push_str(&generate_feed_entry(post, site_config)?);
    }

    feed.push_str("</feed>\n");

    Ok(feed)
}

/// Generate a single feed entry
fn generate_feed_entry(post: &Document, site_config: &SiteConfig) -> Result<String, Box<dyn std::error::Error>> {
    let mut entry = String::new();
    
    entry.push_str("  <entry>\n");
    
    // Title
    if let Some(ref title) = post.front_matter.title {
        entry.push_str(&format!("    <title>{}</title>\n", escape_xml(title)));
    }

    // Link and ID
    let post_url = generate_post_url(post, site_config);
    entry.push_str(&format!("    <link href=\"{}\" />\n", escape_xml_url(&post_url)));
    entry.push_str(&format!("    <id>{}</id>\n", escape_xml_url(&post_url)));

    // Date
    if let Some(date) = post.front_matter.date {
        entry.push_str(&format!("    <updated>{}</updated>\n", date.to_rfc3339()));
        entry.push_str(&format!("    <published>{}</published>\n", date.to_rfc3339()));
    }

    // Content
    entry.push_str("    <content type=\"html\"><![CDATA[\n");
    entry.push_str(&post.content);
    entry.push_str("\n    ]]></content>\n");

    // Tags as categories
    if let Some(ref tags) = post.front_matter.tags {
        for tag in tags {
            entry.push_str(&format!("    <category term=\"{}\" />\n", escape_xml(tag)));
        }
    }

    entry.push_str("  </entry>\n");

    Ok(entry)
}

/// Generate URL for a post
fn generate_post_url(post: &Document, site_config: &SiteConfig) -> String {
    let mut path = std::path::PathBuf::from(&post.file_path);
    path.set_extension("html");
    
    if let Some(ref base_url) = site_config.base_url {
        format!("{}/{}", base_url.trim_end_matches('/'), path.to_string_lossy())
    } else {
        path.to_string_lossy().to_string()
    }
}

/// Check if document should be included in feed
fn is_post_for_feed(document: &Document) -> bool {
    // Include only posts (not pages) and only default language
    (document.front_matter.extra.get("layout").and_then(|v| v.as_str()) == Some("post") || document.file_path.starts_with("posts/")) &&
    document.language == "en"
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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
    fn test_escape_xml() {
        assert_eq!(escape_xml("Hello & <world>"), "Hello &amp; &lt;world&gt;");
    }

    #[test]
    fn test_is_post_for_feed() {
        use crate::parser::FrontMatter;
        
        let mut post_extra = HashMap::new();
        post_extra.insert("layout".to_string(), serde_yaml::Value::String("post".to_string()));
        
        let post = Document {
            file_path: "posts/test.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                pdf: None,
                extra: post_extra,
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "test".to_string(),
        };
        assert!(is_post_for_feed(&post));

        let mut page_extra = HashMap::new();
        page_extra.insert("layout".to_string(), serde_yaml::Value::String("page".to_string()));
        
        let page = Document {
            file_path: "pages/about.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                pdf: None,
                extra: page_extra,
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "about".to_string(),
        };
        assert!(!is_post_for_feed(&page));
    }
}