use crate::parser::Document;
use crate::theme::Theme;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::Context;

/// Generate HTML pages for all documents
pub fn generate_pages(
    documents: &[Document],
    theme: &Theme,
    i18n: &I18nManager,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    for document in documents {
        generate_page(document, documents, theme, i18n, site_config, output_dir)?;
    }
    Ok(())
}

/// Generate a single HTML page from a document
pub fn generate_page(
    document: &Document,
    all_documents: &[Document],
    theme: &Theme,
    i18n: &I18nManager,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    
    // Add document data
    context.insert("title", &document.front_matter.title);
    context.insert("content", &document.content);
    context.insert("date", &document.front_matter.date);
    context.insert("tags", &document.front_matter.tags);
    context.insert("language", &document.language);
    
    // Add site configuration
    context.insert("site_title", &site_config.get_site_title());
    if let Some(ref base_url) = site_config.base_url {
        context.insert("base_url", base_url);
    }

    // Add custom frontmatter fields
    for (key, value) in &document.front_matter.extra {
        context.insert(key, value);
    }

    // Process TOC if enabled (check for toc field in frontmatter)
    if document.front_matter.extra.get("toc").and_then(|v| v.as_bool()).unwrap_or(false) {
        let (toc_html, processed_content) = super::markdown::generate_toc_and_content(&document.content, document.front_matter.title.as_deref());
        context.insert("toc", &toc_html);
        context.insert("content", &processed_content);
    }

    // Process footnotes
    let processed_content = super::markdown::process_footnotes(context.get("content").unwrap().as_str().unwrap());
    context.insert("content", &processed_content);

    // Add navigation data
    add_navigation_context(&mut context, document, all_documents, i18n);

    // Add language translations
    add_language_context(&mut context, document, all_documents);

    // Add sidebar pages
    add_sidebar_context(&mut context, all_documents);

    // Determine template to use
    let template_name = determine_template_name(document);
    
    // Render template
    let rendered = theme.templates.render(&template_name, &context)
        .map_err(|e| format!("Failed to render template '{}': {}", template_name, e))?;

    // Determine output path
    let output_path = determine_output_path(document, output_dir);
    
    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Write file
    let mut file = File::create(&output_path)?;
    file.write_all(rendered.as_bytes())?;

    Ok(())
}

/// Generate the index page with post listings
pub fn generate_index(
    documents: &[Document],
    theme: &Theme,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    
    // Add site configuration
    context.insert("site_title", &site_config.title);
    if let Some(ref base_url) = site_config.base_url {
        context.insert("base_url", base_url);
    }

    // Filter and sort posts (only documents with 'post' layout or in posts directory)
    let mut posts: Vec<&Document> = documents.iter()
        .filter(|doc| is_post(doc))
        .collect();
    
    posts.sort_by(|a, b| {
        b.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC)
            .cmp(&a.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC))
    });

    context.insert("posts", &posts);

    // Add sidebar pages
    add_sidebar_context(&mut context, documents);

    // Render index template
    let rendered = theme.templates.render("index.html", &context)
        .map_err(|e| format!("Failed to render index template: {}", e))?;

    // Write index file
    let index_path = output_dir.join("index.html");
    let mut file = File::create(&index_path)?;
    file.write_all(rendered.as_bytes())?;

    Ok(())
}

/// Add navigation context for back links and breadcrumbs
fn add_navigation_context(context: &mut Context, document: &Document, _all_documents: &[Document], i18n: &I18nManager) {
    // Add "Back to Home" link for posts
    if is_post(document) {
        context.insert("show_back_to_home", &true);
    }

    // Add language selector info
    context.insert("language_name", &i18n.get_language_name(&document.language));
}

/// Add language context for translations
fn add_language_context(context: &mut Context, document: &Document, all_documents: &[Document]) {
    // Find other language versions of this document
    let base_path = get_base_path(&std::path::Path::new(&document.file_path));
    let translations: Vec<_> = all_documents.iter()
        .filter(|doc| get_base_path(&std::path::Path::new(&doc.file_path)) == base_path && doc.language != document.language)
        .map(|doc| {
            let mut translation = HashMap::new();
            translation.insert("language", doc.language.clone());
            translation.insert("path", format_output_path(&std::path::Path::new(&doc.file_path), &doc.language));
            translation
        })
        .collect();

    if !translations.is_empty() {
        context.insert("translations", &translations);
    }
}

/// Add sidebar context with page links
fn add_sidebar_context(context: &mut Context, all_documents: &[Document]) {
    // Get all pages (non-posts) for sidebar
    let mut pages: Vec<_> = all_documents.iter()
        .filter(|doc| !is_post(doc))
        .collect();
    
    // Sort pages alphabetically by title
    pages.sort_by(|a, b| {
        a.front_matter.title.as_deref().unwrap_or("")
            .cmp(b.front_matter.title.as_deref().unwrap_or(""))
    });

    context.insert("sidebar_pages", &pages);
}

/// Determine if a document is a post
fn is_post(document: &Document) -> bool {
    document.front_matter.extra.get("layout").and_then(|v| v.as_str()) == Some("post") || 
    document.file_path.starts_with("posts/")
}

/// Determine template name based on document layout or path
fn determine_template_name(document: &Document) -> String {
    if let Some(layout) = document.front_matter.extra.get("layout").and_then(|v| v.as_str()) {
        format!("{}.html", layout)
    } else if is_post(document) {
        "post.html".to_string()
    } else {
        "page.html".to_string()
    }
}

/// Determine output path for a document
fn determine_output_path(document: &Document, output_dir: &Path) -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(&document.file_path);
    path.set_extension("html");
    output_dir.join(path)
}

/// Get base path without language suffix
fn get_base_path(path: &Path) -> String {
    let stem = path.file_stem().unwrap().to_string_lossy();
    let parent = path.parent().map(|p| p.to_string_lossy()).unwrap_or_default();
    
    // Remove language suffix if present (e.g., "file.en" -> "file")
    let base_stem = if let Some(dot_pos) = stem.rfind('.') {
        let (base, lang) = stem.split_at(dot_pos);
        // Check if the suffix looks like a language code
        if lang.len() == 3 && lang.chars().nth(1).unwrap() != '.' {
            base
        } else {
            &stem
        }
    } else {
        &stem
    };
    
    if parent.is_empty() {
        base_stem.to_string()
    } else {
        format!("{}/{}", parent, base_stem)
    }
}

/// Format output path for a document with language
fn format_output_path(path: &Path, language: &str) -> String {
    let mut html_path = path.with_extension("html");
    
    if language != "en" {
        // Add language suffix to filename
        let stem = html_path.file_stem().unwrap().to_string_lossy();
        let new_name = format!("{}.{}.html", stem, language);
        html_path.set_file_name(new_name);
    }
    
    html_path.to_string_lossy().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::FrontMatter;
    use std::collections::HashMap;

    #[test]
    fn test_is_post() {
        let post_doc = Document {
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
        assert!(is_post(&post_doc));

        let mut extra = HashMap::new();
        extra.insert("layout".to_string(), serde_yaml::Value::String("page".to_string()));
        let page_doc = Document {
            file_path: "pages/about.md".to_string(),
            front_matter: FrontMatter {
                title: None,
                date: None,
                tags: None,
                lang: None,
                draft: None,
                extra,
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "about".to_string(),
        };
        assert!(!is_post(&page_doc));
    }

    #[test]
    fn test_get_base_path() {
        assert_eq!(get_base_path(&std::path::Path::new("posts/hello.en.md")), "posts/hello");
        assert_eq!(get_base_path(&std::path::Path::new("about.md")), "about");
    }
}