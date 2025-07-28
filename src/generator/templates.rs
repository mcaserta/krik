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

/// Add common site and path context to a Tera context
fn add_site_context(context: &mut Context, site_config: &SiteConfig, language: &str) {
    context.insert("site_title", &site_config.get_site_title());
    if let Some(ref base_url) = site_config.base_url {
        context.insert("base_url", base_url);
    }
    context.insert("assets_path", "/assets");
    context.insert("home_path", "/");
    context.insert("feed_path", "/feed.xml");
    context.insert("lang", language);
}

/// Create page link HashMap for navigation
fn create_page_link(document: &Document) -> HashMap<&str, String> {
    let mut page_link = HashMap::new();
    page_link.insert("title", document.front_matter.title.as_deref().unwrap_or("Untitled").to_string());
    page_link.insert("url", format!("/{}", document.file_path.replace(".md", ".html")));
    page_link
}

/// Add page links context for navigation
fn add_page_links_context(context: &mut Context, all_documents: &[Document]) {
    let page_links: Vec<HashMap<&str, String>> = all_documents.iter()
        .filter(|doc| !is_post(doc) && doc.language == "en")
        .map(create_page_link)
        .collect();
    context.insert("page_links", &page_links);
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
    context.insert("base_name", &document.base_name);
    
    // Add site configuration and common paths
    add_site_context(&mut context, site_config, &document.language);

    // Add custom frontmatter fields
    for (key, value) in &document.front_matter.extra {
        context.insert(key, value);
    }

    // Remove duplicate title from content if it matches frontmatter title
    let content_without_title = super::markdown::remove_duplicate_title(&document.content, document.front_matter.title.as_deref());
    context.insert("content", &content_without_title);

    // Process TOC if enabled (check for toc field in frontmatter)
    if document.front_matter.extra.get("toc").and_then(|v| v.as_bool()).unwrap_or(false) {
        let current_content = context.get("content").unwrap().as_str().unwrap();
        let (toc_html, processed_content) = super::markdown::generate_toc_and_content(current_content, document.front_matter.title.as_deref());
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

    // Add sidebar pages and page links for navigation
    add_sidebar_context(&mut context, all_documents);
    add_page_links_context(&mut context, all_documents);

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

/// Create post object for index page template
fn create_post_object(document: &Document) -> HashMap<&str, String> {
    let mut post = HashMap::new();
    post.insert("title", document.front_matter.title.as_deref().unwrap_or("Untitled").to_string());
    post.insert("url", format!("/{}", document.file_path.replace(".md", ".html")));
    if let Some(date) = document.front_matter.date {
        post.insert("date", date.to_rfc3339());
    }
    post
}

/// Generate the index page with post listings
pub fn generate_index(
    documents: &[Document],
    theme: &Theme,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    
    // Add site configuration and common paths
    add_site_context(&mut context, site_config, "en");

    // Filter and sort posts (only documents with 'post' layout or in posts directory, default language only)
    let mut post_docs: Vec<&Document> = documents.iter()
        .filter(|doc| is_post(doc) && doc.language == "en")
        .collect();
    
    post_docs.sort_by(|a, b| {
        b.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC)
            .cmp(&a.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC))
    });

    // Create post objects with URL and other template fields
    let posts: Vec<HashMap<&str, String>> = post_docs.iter()
        .map(|doc| create_post_object(doc))
        .collect();

    context.insert("posts", &posts);

    // Add sidebar pages and page links for navigation
    add_sidebar_context(&mut context, documents);
    add_page_links_context(&mut context, documents);

    
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
    // Find all language versions of this document (including current)
    let base_path = get_base_path(&std::path::Path::new(&document.file_path));
    let mut available_translations: Vec<_> = all_documents.iter()
        .filter(|doc| get_base_path(&std::path::Path::new(&doc.file_path)) == base_path)
        .map(|doc| {
            let mut translation = HashMap::new();
            translation.insert("lang", doc.language.clone());
            translation.insert("lang_name", get_language_name(&doc.language));
            translation.insert("path", format!("/{}", doc.file_path.replace(".md", ".html")));
            translation.insert("is_current", if doc.language == document.language { "true".to_string() } else { "false".to_string() });
            translation
        })
        .collect();

    // Sort by language code for consistent order
    available_translations.sort_by(|a, b| a.get("lang").cmp(&b.get("lang")));

    if available_translations.len() > 1 {
        context.insert("available_translations", &available_translations);
    }
}

/// Get display name for language code
fn get_language_name(lang_code: &str) -> String {
    match lang_code {
        "en" => "English".to_string(),
        "it" => "Italiano".to_string(),
        "es" => "Español".to_string(),
        "fr" => "Français".to_string(),
        "de" => "Deutsch".to_string(),
        "pt" => "Português".to_string(),
        "ja" => "日本語".to_string(),
        "zh" => "中文".to_string(),
        "ru" => "Русский".to_string(),
        "ar" => "العربية".to_string(),
        _ => lang_code.to_uppercase(),
    }
}

/// Add sidebar context with page links
fn add_sidebar_context(context: &mut Context, all_documents: &[Document]) {
    // Get all pages (non-posts) for sidebar (default language only)
    let mut pages: Vec<_> = all_documents.iter()
        .filter(|doc| !is_post(doc) && doc.language == "en")
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