use crate::parser::Document;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use crate::theme::Theme;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::Context;

use super::context::{add_page_links_context, add_site_context, create_post_object, is_post};
use super::paths::get_base_path;

pub fn generate_index(
    documents: &[Document],
    theme: &Theme,
    site_config: &SiteConfig,
    i18n: &I18nManager,
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    add_site_context(&mut context, site_config, i18n.default_language(), "index.html");

    let site_description = format!("{} - Latest posts and articles", site_config.get_site_title());
    context.insert("site_description", &site_description);

    // Choose one document per post base path, prefer default language if available
    let default_lang = i18n.default_language();
    use std::collections::HashMap;
    let mut chosen: HashMap<String, &Document> = HashMap::new();
    for doc in documents.iter().filter(|d| is_post(d)) {
        let base = get_base_path(std::path::Path::new(&doc.file_path));
        match chosen.get(&base) {
            None => {
                chosen.insert(base, doc);
            }
            Some(existing) => {
                // Prefer default language over non-default
                if existing.language != default_lang && doc.language == default_lang {
                    chosen.insert(base, doc);
                }
            }
        }
    }
    let mut post_docs: Vec<&Document> = chosen.values().cloned().collect();
    post_docs.sort_by(|a, b| b.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC).cmp(&a.front_matter.date.unwrap_or(DateTime::<Utc>::MIN_UTC)));

    let posts: Vec<std::collections::HashMap<String, serde_json::Value>> = post_docs
        .iter()
        .map(|doc| create_post_object(doc, "index.html"))
        .collect();
    context.insert("posts", &posts);

    add_page_links_context(&mut context, documents, "index.html");

    let rendered = theme
        .templates
        .render("index.html", &context)
        .map_err(|e| format!("Failed to render index template: {}", e))?;
    let index_path = output_dir.join("index.html");
    let mut file = File::create(&index_path)?;
    file.write_all(rendered.as_bytes())?;
    Ok(())
}


