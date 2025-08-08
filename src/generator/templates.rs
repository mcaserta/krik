use crate::parser::Document;
use crate::theme::Theme;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::Context;
use pathdiff::diff_paths;
use serde_json::json;

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

/// Calculate relative path using pathdiff for consistent handling across nested paths
fn calculate_relative_path(file_path: &str, target: &str) -> String {
    let current_path = PathBuf::from(file_path);
    let target_path = PathBuf::from(target.trim_start_matches('/'));
    
    // Get the directory of the current file
    let current_dir = current_path.parent().unwrap_or_else(|| Path::new(""));
    
    // Use pathdiff to calculate the relative path from current directory to target
    if let Some(relative_path) = diff_paths(&target_path, current_dir) {
        // Convert to string and ensure forward slashes for consistency
        relative_path.to_string_lossy().replace('\\', "/")
    } else {
        // Fallback: if pathdiff fails, use the target path as-is
        target.trim_start_matches('/').to_string()
    }
}

/// Generate a clean description from document content
fn generate_description(content: &str, frontmatter_description: Option<&String>) -> String {
    if let Some(desc) = frontmatter_description {
        // Use frontmatter description if available, clean it up
        desc.trim().replace('\n', " ").replace('\r', " ")
    } else {
        // Generate from content: strip ALL HTML tags, truncate, and clean up
        let mut text_content = content.to_string();
        
        // Remove all HTML tags using a simple regex-like approach
        while let Some(start) = text_content.find('<') {
            if let Some(end) = text_content[start..].find('>') {
                text_content.replace_range(start..start + end + 1, " ");
            } else {
                break;
            }
        }
        
        // Remove extra whitespace and truncate
        let cleaned = text_content
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");
        
        if cleaned.len() > 160 {
            format!("{}...", &cleaned[..157])
        } else {
            cleaned
        }
    }
}

/// Add common site and path context to a Tera context
fn add_site_context(context: &mut Context, site_config: &SiteConfig, language: &str, file_path: &str) {
    context.insert("site_title", &site_config.get_site_title());
    context.insert("file_path", file_path);
    if let Some(ref base_url) = site_config.base_url {
        context.insert("base_url", base_url);
    }
    
    // Calculate relative paths based on file depth
    let assets_path = calculate_relative_path(file_path, "/assets");
    let home_path = calculate_relative_path(file_path, "/index.html");
    let feed_path = calculate_relative_path(file_path, "/feed.xml");
    
    context.insert("assets_path", &assets_path);
    context.insert("home_path", &home_path);
    context.insert("feed_path", &feed_path);
    context.insert("lang", language);
}

/// Create page link HashMap for navigation with relative paths
fn create_page_link(document: &Document, current_file_path: &str) -> HashMap<String, String> {
    let mut page_link = HashMap::new();
    let target_path = format!("/{}", document.file_path.replace(".md", ".html"));
    let relative_url = calculate_relative_path(current_file_path, &target_path);
    
    page_link.insert("title".to_string(), document.front_matter.title.as_deref().unwrap_or("Untitled").to_string());
    page_link.insert("url".to_string(), relative_url);
    page_link
}

/// Add page links context for navigation
fn add_page_links_context(context: &mut Context, all_documents: &[Document], current_file_path: &str) {
    let mut filtered_docs: Vec<_> = all_documents.iter()
        .filter(|doc| !is_post(doc) && doc.language == "en")
        .collect();
    
    // Sort pages alphabetically by title
    filtered_docs.sort_by(|a, b| {
        a.front_matter.title.as_deref().unwrap_or("")
            .cmp(b.front_matter.title.as_deref().unwrap_or(""))
    });
    
    let page_links: Vec<HashMap<String, String>> = filtered_docs.iter()
        .map(|doc| create_page_link(doc, current_file_path))
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
    context.insert("pdf", &document.front_matter.pdf);
    
    // Generate and add description
    let frontmatter_desc = document.front_matter.extra.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let description = generate_description(&document.content, frontmatter_desc.as_ref());
    context.insert("description", &description);
    
    // Add site configuration and common paths
    add_site_context(&mut context, site_config, &document.language, &document.file_path);

    // Add custom frontmatter fields
    for (key, value) in &document.front_matter.extra {
        context.insert(key, value);
    }

    // Remove duplicate title from content if it matches frontmatter title
    let content_without_title = super::markdown::remove_duplicate_title(&document.content, document.front_matter.title.as_deref());
    context.insert("content", &content_without_title);

    // Add TOC if available
    if let Some(toc_html) = &document.toc {
        context.insert("toc", toc_html);
    }

    // Process footnotes
    let processed_content = match context.get("content").and_then(|v| v.as_str()) {
        Some(s) => super::markdown::process_footnotes(s),
        None => super::markdown::process_footnotes(""),
    };
    context.insert("content", &processed_content);

    // Add navigation data
    add_navigation_context(&mut context, document, i18n);

    // Add language translations
    add_language_context(&mut context, document, all_documents);

    // Add sidebar pages and page links for navigation
    add_sidebar_context(&mut context, all_documents);
    add_page_links_context(&mut context, all_documents, &document.file_path);

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

/// Create post object for index page template with relative paths
fn create_post_object(
    document: &Document,
    current_file_path: &str,
) -> HashMap<String, serde_json::Value> {
    let mut post: HashMap<String, serde_json::Value> = HashMap::new();
    let target_path = format!("/{}", document.file_path.replace(".md", ".html"));
    let relative_url = calculate_relative_path(current_file_path, &target_path);

    post.insert(
        "title".to_string(),
        json!(document
            .front_matter
            .title
            .as_deref()
            .unwrap_or("Untitled")),
    );
    post.insert("url".to_string(), json!(relative_url));
    if let Some(date) = document.front_matter.date {
        post.insert("date".to_string(), json!(date.to_rfc3339()));
    }
    if let Some(tags) = &document.front_matter.tags {
        post.insert("tags".to_string(), json!(tags));
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
    
    // Add site configuration and common paths (index is always at root)
    add_site_context(&mut context, site_config, "en", "index.html");
    
    // Generate site description
    let site_description = format!("{} - Latest posts and articles", site_config.get_site_title());
    context.insert("site_description", &site_description);

    // Filter and sort posts (only documents with 'post' layout or in posts directory, default language only)
    let mut post_docs: Vec<&Document> = documents.iter()
        .filter(|doc| is_post(doc) && doc.language == "en")
        .collect();
    
    post_docs.sort_by(|a, b| {
        b.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC)
            .cmp(&a.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC))
    });

    // Create post objects with URL, date, and tags for template
    let posts: Vec<HashMap<String, serde_json::Value>> = post_docs
        .iter()
        .map(|doc| create_post_object(doc, "index.html"))
        .collect();

    context.insert("posts", &posts);

    // Add sidebar pages and page links for navigation
    add_sidebar_context(&mut context, documents);
    add_page_links_context(&mut context, documents, "index.html");

    
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
fn add_navigation_context(context: &mut Context, document: &Document, i18n: &I18nManager) {
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
            let target_path = format!("/{}", doc.file_path.replace(".md", ".html"));
            let relative_path = calculate_relative_path(&document.file_path, &target_path);
            translation.insert("path", relative_path);
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
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let parent = path.parent().map(|p| p.to_string_lossy()).unwrap_or_default();
    
    // Remove language suffix if present (e.g., "file.en" -> "file")
    let base_stem = if let Some(dot_pos) = stem.rfind('.') {
        let (base, lang) = stem.split_at(dot_pos);
        // Check if the suffix looks like a language code
        if lang.len() == 3 && lang.chars().nth(1).unwrap_or('.') != '.' {
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
                pdf: None,
                extra: HashMap::new(),
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "test".to_string(),
            toc: None,
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
                pdf: None,
                extra,
            },
            content: String::new(),
            language: "en".to_string(),
            base_name: "about".to_string(),
            toc: None,
        };
        assert!(!is_post(&page_doc));
    }

    #[test]
    fn test_get_base_path() {
        assert_eq!(get_base_path(&std::path::Path::new("posts/hello.en.md")), "posts/hello");
        assert_eq!(get_base_path(&std::path::Path::new("about.md")), "about");
    }

    #[test]
    fn test_calculate_relative_path() {
        // Test basic cases
        assert_eq!(calculate_relative_path("index.html", "/assets/css/main.css"), "assets/css/main.css");
        assert_eq!(calculate_relative_path("about.html", "/assets/css/main.css"), "assets/css/main.css");
        
        // Test nested paths
        assert_eq!(calculate_relative_path("posts/article.html", "/assets/css/main.css"), "../assets/css/main.css");
        assert_eq!(calculate_relative_path("posts/2023/article.html", "/assets/css/main.css"), "../../assets/css/main.css");
        assert_eq!(calculate_relative_path("posts/2023/12/article.html", "/assets/css/main.css"), "../../../assets/css/main.css");
        
        // Test same directory
        assert_eq!(calculate_relative_path("posts/article1.html", "/posts/article2.html"), "article2.html");
        assert_eq!(calculate_relative_path("posts/2023/article1.html", "/posts/2023/article2.html"), "article2.html");
        
        // Test cross-directory navigation
        assert_eq!(calculate_relative_path("posts/article.html", "/pages/about.html"), "../pages/about.html");
        assert_eq!(calculate_relative_path("posts/2023/article.html", "/pages/about.html"), "../../pages/about.html");
        assert_eq!(calculate_relative_path("pages/about.html", "/posts/article.html"), "../posts/article.html");
        
        // Test deep nesting
        assert_eq!(calculate_relative_path("posts/2023/12/25/article.html", "/assets/css/main.css"), "../../../../assets/css/main.css");
        assert_eq!(calculate_relative_path("posts/2023/12/25/article.html", "/index.html"), "../../../../index.html");
        
        // Test root to nested
        assert_eq!(calculate_relative_path("index.html", "/posts/article.html"), "posts/article.html");
        assert_eq!(calculate_relative_path("index.html", "/posts/2023/article.html"), "posts/2023/article.html");
        
        // Test complex cross-navigation - these are the actual results from pathdiff
        assert_eq!(calculate_relative_path("posts/2023/12/article.html", "/pages/contact.html"), "../../../pages/contact.html");
        assert_eq!(calculate_relative_path("pages/contact.html", "/posts/2023/12/article.html"), "../posts/2023/12/article.html");
    }
}