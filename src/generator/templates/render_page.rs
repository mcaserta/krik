use crate::i18n::I18nManager;
use crate::parser::Document;
use crate::site::SiteConfig;
use crate::theme::Theme;
use crate::error::{KrikError, KrikResult, TemplateError, TemplateErrorKind};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tera::Context;

use super::context::{
    add_language_context, add_navigation_context, add_page_links_context, add_sidebar_context, add_site_context,
    generate_description,
};
use super::paths::{determine_output_path};
use super::select::determine_template_name;
use rayon::prelude::*;
use std::sync::Mutex;

pub fn generate_pages(
    documents: &[Document],
    theme: &Theme,
    i18n: &I18nManager,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> KrikResult<()> {
    // Render pages in parallel. File writes target distinct paths, so no shared file contention.
    // Aggregate errors to avoid partial silent failures.
    let first_error: Mutex<Option<KrikError>> = Mutex::new(None);

    documents.par_iter().for_each(|document| {
        if let Err(e) = generate_page(document, documents, theme, i18n, site_config, output_dir) {
            if let Ok(mut guard) = first_error.lock() {
                if guard.is_none() {
                    *guard = Some(e);
                }
            }
        }
    });

    if let Ok(guard) = first_error.into_inner() {
        if let Some(err) = guard { return Err(err); }
    }
    Ok(())
}

pub fn generate_page(
    document: &Document,
    all_documents: &[Document],
    theme: &Theme,
    i18n: &I18nManager,
    site_config: &SiteConfig,
    output_dir: &Path,
) -> KrikResult<()> {
    let context = build_page_context(document, all_documents, site_config, i18n);
    let rendered_content = render_template(theme, document, &context)?;
    write_output_file(document, output_dir, &rendered_content)
}

/// Build the template context for a page
fn build_page_context(
    document: &Document,
    all_documents: &[Document],
    site_config: &SiteConfig,
    i18n: &I18nManager,
) -> Context {
    let mut context = create_base_context(document);
    add_processed_content(&mut context, document);
    add_all_contexts(&mut context, document, all_documents, site_config, i18n);
    context
}

/// Create the base template context with document fields
fn create_base_context(document: &Document) -> Context {
    let mut context = Context::new();
    context.insert("title", &document.front_matter.title);
    context.insert("content", &document.content);
    context.insert("date", &document.front_matter.date);
    context.insert("tags", &document.front_matter.tags);
    context.insert("language", &document.language);
    context.insert("base_name", &document.base_name);
    context.insert("pdf", &document.front_matter.pdf);

    let frontmatter_desc = document
        .front_matter
        .extra
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let description = generate_description(&document.content, frontmatter_desc.as_ref());
    context.insert("description", &description);

    // Add extra frontmatter fields
    for (key, value) in &document.front_matter.extra {
        context.insert(key, value);
    }

    context
}

/// Add processed content (without duplicate title, with footnotes) to context
fn add_processed_content(context: &mut Context, document: &Document) {
    let content_without_title = crate::generator::markdown::remove_duplicate_title(
        &document.content,
        document.front_matter.title.as_deref(),
    );
    context.insert("content", &content_without_title);

    if let Some(toc_html) = &document.toc {
        context.insert("toc", toc_html);
    }

    // footnotes pass-through for now
    let processed_content = crate::generator::markdown::process_footnotes(
        context.get("content").and_then(|v| v.as_str()).unwrap_or("")
    );
    context.insert("content", &processed_content);
}

/// Add all context helpers (site, navigation, language, sidebar, page links)
fn add_all_contexts(
    context: &mut Context,
    document: &Document,
    all_documents: &[Document],
    site_config: &SiteConfig,
    i18n: &I18nManager,
) {
    add_site_context(context, site_config, &document.language, &document.file_path);
    add_navigation_context(context, document, i18n);
    add_language_context(context, document, all_documents, i18n);
    add_sidebar_context(context, all_documents);
    add_page_links_context(context, all_documents, &document.file_path);
}

/// Render the template with the given context
fn render_template(theme: &Theme, document: &Document, context: &Context) -> KrikResult<String> {
    let template_name = determine_template_name(document);
    theme
        .templates
        .render(&template_name, context)
        .map_err(|e| KrikError::Template(TemplateError {
            kind: TemplateErrorKind::RenderError(e),
            template: template_name.clone(),
            context: format!("Rendering page for {}", document.file_path),
        }))
}

/// Write the rendered content to the output file
fn write_output_file(document: &Document, output_dir: &Path, rendered_content: &str) -> KrikResult<()> {
    let output_path = determine_output_path(&document.file_path, output_dir);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&output_path)?;
    file.write_all(rendered_content.as_bytes())?;
    Ok(())
}
