use crate::i18n::I18nManager;
use crate::parser::Document;
use crate::site::SiteConfig;
use serde_json::json;
use std::collections::HashMap;
use tera::Context;

use super::paths::{calculate_relative_path, get_base_path};

pub fn add_site_context(
    context: &mut Context,
    site_config: &SiteConfig,
    language: &str,
    file_path: &str,
) {
    context.insert("site_title", &site_config.get_site_title());
    context.insert("file_path", file_path);
    if let Some(ref base_url) = site_config.base_url {
        context.insert("base_url", base_url);
    }
    let assets_path = calculate_relative_path(file_path, "/assets");
    let home_path = calculate_relative_path(file_path, "/index.html");
    let feed_path = calculate_relative_path(file_path, "/feed.xml");
    context.insert("assets_path", &assets_path);
    context.insert("home_path", &home_path);
    context.insert("feed_path", &feed_path);
    context.insert("lang", language);
}

pub fn add_navigation_context(context: &mut Context, document: &Document) {
    if is_post(document) {
        context.insert("show_back_to_home", &true);
    }
    context.insert(
        "language_name",
        &I18nManager::get_language_name(&document.language),
    );
}

pub fn add_language_context(
    context: &mut Context,
    document: &Document,
    all_documents: &[Document],
) {
    let base_path = get_base_path(std::path::Path::new(&document.file_path));
    let mut available_translations: Vec<_> = all_documents
        .iter()
        .filter(|doc| get_base_path(std::path::Path::new(&doc.file_path)) == base_path)
        .map(|doc| {
            let mut translation = HashMap::new();
            translation.insert("lang", doc.language.clone());
            translation.insert("lang_name", I18nManager::get_language_name(&doc.language));
            let target_path = format!("/{}", doc.file_path.replace(".md", ".html"));
            let relative_path = calculate_relative_path(&document.file_path, &target_path);
            translation.insert("path", relative_path);
            translation.insert(
                "is_current",
                if doc.language == document.language {
                    "true".to_string()
                } else {
                    "false".to_string()
                },
            );
            translation
        })
        .collect();

    available_translations.sort_by(|a, b| a.get("lang").cmp(&b.get("lang")));
    if available_translations.len() > 1 {
        context.insert("available_translations", &available_translations);
    }
}

pub fn add_sidebar_context(context: &mut Context, all_documents: &[Document]) {
    let mut pages: Vec<_> = all_documents
        .iter()
        .filter(|doc| !is_post(doc) && doc.language == "en")
        .collect();
    pages.sort_by(|a, b| {
        a.front_matter
            .title
            .as_deref()
            .unwrap_or("")
            .cmp(b.front_matter.title.as_deref().unwrap_or(""))
    });
    context.insert("sidebar_pages", &pages);
}

pub fn add_page_links_context(
    context: &mut Context,
    all_documents: &[Document],
    current_file_path: &str,
) {
    let mut filtered_docs: Vec<_> = all_documents
        .iter()
        .filter(|doc| !is_post(doc) && doc.language == "en")
        .collect();
    filtered_docs.sort_by(|a, b| {
        a.front_matter
            .title
            .as_deref()
            .unwrap_or("")
            .cmp(b.front_matter.title.as_deref().unwrap_or(""))
    });
    let page_links: Vec<HashMap<String, String>> = filtered_docs
        .iter()
        .map(|doc| create_page_link(doc, current_file_path))
        .collect();
    context.insert("page_links", &page_links);
}

pub fn create_post_object(
    document: &Document,
    current_file_path: &str,
) -> HashMap<String, serde_json::Value> {
    let mut post: HashMap<String, serde_json::Value> = HashMap::new();
    let target_path = format!("/{}", document.file_path.replace(".md", ".html"));
    let relative_url = calculate_relative_path(current_file_path, &target_path);
    post.insert(
        "title".to_string(),
        json!(document.front_matter.title.as_deref().unwrap_or("Untitled")),
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

pub fn create_page_link(document: &Document, current_file_path: &str) -> HashMap<String, String> {
    let mut page_link = HashMap::new();
    let target_path = format!("/{}", document.file_path.replace(".md", ".html"));
    let relative_url = calculate_relative_path(current_file_path, &target_path);
    page_link.insert(
        "title".to_string(),
        document
            .front_matter
            .title
            .as_deref()
            .unwrap_or("Untitled")
            .to_string(),
    );
    page_link.insert("url".to_string(), relative_url);
    page_link
}

pub fn is_post(document: &Document) -> bool {
    document
        .front_matter
        .extra
        .get("layout")
        .and_then(|v| v.as_str())
        == Some("post")
        || document.file_path.starts_with("posts/")
}

pub fn generate_description(content: &str, frontmatter_description: Option<&String>) -> String {
    frontmatter_description
        .map(|desc| clean_frontmatter_description(desc))
        .unwrap_or_else(|| extract_description_from_content(content))
}

/// Clean frontmatter description by removing line breaks
pub fn clean_frontmatter_description(desc: &str) -> String {
    desc.trim().replace(['\n', '\r'], " ")
}

/// Extract description from HTML content by removing tags and truncating
pub fn extract_description_from_content(content: &str) -> String {
    let text_content = strip_html_tags(content);
    let cleaned = normalize_whitespace(&text_content);
    truncate_description(&cleaned, 160)
}

/// Remove HTML tags from content
pub fn strip_html_tags(content: &str) -> String {
    let mut text_content = content.to_string();
    while let Some(start) = text_content.find('<') {
        if let Some(end) = text_content[start..].find('>') {
            text_content.replace_range(start..start + end + 1, " ");
        } else {
            break;
        }
    }
    text_content
}

/// Normalize whitespace by joining with single spaces
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

/// Truncate description to specified length with ellipsis
pub fn truncate_description(text: &str, max_len: usize) -> String {
    if text.len() > max_len {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    } else {
        text.to_string()
    }
}
