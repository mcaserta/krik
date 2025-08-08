use crate::parser::{Document, extract_language_from_filename, parse_markdown_with_frontmatter_for_file};
use crate::generator::ast_parser::{parse_markdown_ast, generate_toc_from_headings};
use crate::error::{KrikResult, KrikError, IoError, IoErrorKind, MarkdownError, MarkdownErrorKind};
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use tracing::{info, debug, warn};

use pulldown_cmark::{html, Options, Parser};

/// Scan files in the source directory and parse markdown documents
pub fn scan_files(source_dir: &Path, documents: &mut Vec<Document>) -> KrikResult<()> {
    info!("Starting file scan in: {}", source_dir.display());
    let mut processed_files = 0;
    let mut skipped_files = 0;
    let mut error_files = 0;

    let entries: Vec<_> = WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .filter(|e| e.path().file_name() != Some(std::ffi::OsStr::new("site.toml")))
        .collect();

    // Parse files in parallel, then merge results deterministically by file path
    let mut results: Vec<(String, Result<Document, KrikError>)> = entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            let rel_path = path
                .strip_prefix(source_dir)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());

            let rel_path_clone = rel_path.clone();
            let res = (|| {
                debug!("Processing file: {}", path.display());
                let content = std::fs::read_to_string(path).map_err(|e| KrikError::Io(IoError {
                    kind: IoErrorKind::ReadFailed(e),
                    path: path.to_path_buf(),
                    context: "Reading markdown file".to_string(),
                }))?;

                let (frontmatter, markdown_content) = parse_markdown_with_frontmatter_for_file(&content, path)?;
                if frontmatter.draft.unwrap_or(false) {
                    return Err(KrikError::Markdown(MarkdownError {
                        kind: MarkdownErrorKind::ParseError("Draft skipped".to_string()),
                        file: path.to_path_buf(),
                        line: None,
                        column: None,
                        context: "Skipping draft file".to_string(),
                    }));
                }

                let filename_without_ext = path
                    .file_stem()
                    .ok_or_else(|| KrikError::Io(IoError {
                        kind: IoErrorKind::InvalidPath,
                        path: path.to_path_buf(),
                        context: "Extracting filename stem".to_string(),
                    }))?
                    .to_string_lossy();
                let (base_name, language) = extract_language_from_filename(&filename_without_ext)?;

                let (html_content, toc_html) = if frontmatter
                    .extra
                    .get("toc")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
                {
                    markdown_to_html_with_toc(&markdown_content, frontmatter.title.as_deref())
                } else {
                    (markdown_to_html(&markdown_content), String::new())
                };

                Ok(Document {
                    front_matter: frontmatter,
                    content: html_content,
                    file_path: rel_path_clone,
                    language,
                    base_name,
                    toc: if toc_html.is_empty() { None } else { Some(toc_html) },
                })
            })();
            (rel_path, res)
        })
        .collect();

    // Sort by path to keep deterministic order when pushing into documents
    results.sort_by(|a, b| a.0.cmp(&b.0));
    for (path_str, res) in results.into_iter() {
        match res {
            Ok(doc) => {
                documents.push(doc);
                processed_files += 1;
                debug!("Successfully processed: {}", path_str);
            }
            Err(e) => {
                // Treat a draft skip as a soft skip; others are errors
                if let KrikError::Markdown(MarkdownError { kind: MarkdownErrorKind::ParseError(msg), .. }) = &e {
                    if msg == "Draft skipped" {
                        skipped_files += 1;
                        continue;
                    }
                }
                warn!("Failed to parse {}: {}", path_str, e);
                error_files += 1;
            }
        }
    }

    info!("File scan completed: {} processed, {} skipped, {} errors", 
          processed_files, skipped_files, error_files);
    Ok(())
}

/// Convert markdown content to HTML with proper options
pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Convert markdown content to HTML and extract TOC data
pub fn markdown_to_html_with_toc(markdown: &str, title: Option<&str>) -> (String, String) {
    // Use AST-based parsing for robust heading IDs and TOC generation
    let result = parse_markdown_ast(markdown);
    let toc_html = generate_toc_from_headings(&result.headings, title);
    (result.html_content, toc_html)
}

/// Generate table of contents and process content for TOC-enabled documents
pub fn generate_toc_and_content(content: &str, title: Option<&str>) -> (String, String) {
    // Generate a TOC directly from HTML using a lightweight regex, to preserve existing behavior.
    let mut toc_items = Vec::new();
    let heading_regex = match Regex::new(r"<h([1-6])[^>]*>([^<]+)</h[1-6]>") {
        Ok(r) => r,
        Err(_) => return (String::new(), content.to_string()),
    };

    for caps in heading_regex.captures_iter(content) {
        let level: u8 = caps[1].parse().unwrap_or(1);
        let text = caps[2].trim();

        // Skip h1 if it matches the title
        if !(level == 1 && title.map_or(false, |t| t.trim() == text)) {
            let id = text
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect::<String>()
                .replace(' ', "-")
                .trim_matches('-')
                .to_string();

            let indent = "  ".repeat((level - 1) as usize);
            toc_items.push(format!("{}<li><a href=\"#{}\">{}</a></li>", indent, id, text));
        }
    }

    let toc = if toc_items.is_empty() {
        String::new()
    } else {
        format!("<ul class=\"toc\">\n{}\n</ul>", toc_items.join("\n"))
    };

    (toc, content.to_string())
}

/// Remove duplicate H1 title from content if it matches the frontmatter title
pub fn remove_duplicate_title(content: &str, title: Option<&str>) -> String {
    if let Some(title) = title {
    let h1_regex = match Regex::new(r"<h1[^>]*>([^<]+)</h1>") {
        Ok(r) => r,
        Err(_) => return content.to_string(),
    };
        if let Some(cap) = h1_regex.captures(content) {
            let heading_text = &cap[1];
            if heading_text.trim() == title.trim() {
                // Remove the first H1 that matches the title
                return h1_regex.replace(content, "").to_string();
            }
        }
    }
    content.to_string()
}

/// Process footnotes to add return navigation
pub fn process_footnotes(content: &str) -> String {
    // For now, just return the content as-is since pulldown-cmark handles footnotes
    content.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write as _;
    use std::path::PathBuf;
    use crate::theme::Theme;
    use crate::i18n::I18nManager;
    use crate::site::SiteConfig;
    use crate::generator::templates;

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<h1")); // Changed to include ID attribute
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_generate_toc_and_content() {
        let content = "<h1>Title</h1>\n<h2>Section 1</h2>\n<h3>Subsection</h3>";
        let (toc, processed) = generate_toc_and_content(content, Some("Title"));
        
        assert!(toc.contains("Section 1"));
        assert!(toc.contains("Subsection"));
        assert!(!toc.contains("Title")); // h1 matching title should be excluded
        assert_eq!(processed, content); // Should return content as-is
    }

    #[test]
    fn test_scan_and_generate_non_default_language_post() {
        // Prepare a temporary content directory with posts/foo.it.md only
        let tmp_root = std::env::temp_dir().join(format!("krik_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp_root);
        fs::create_dir_all(tmp_root.join("content/posts")).unwrap();

        let post_path = tmp_root.join("content/posts/foo.it.md");
        let mut f = fs::File::create(&post_path).unwrap();
        writeln!(
            f,
            "---\ntitle: \"Ciao\"\nlayout: post\n---\n\n# Ciao\n\nContenuto in italiano."
        )
        .unwrap();

        // Scan files
        let source_dir = tmp_root.join("content");
        let mut documents: Vec<crate::parser::Document> = Vec::new();
        scan_files(&source_dir, &mut documents).unwrap();

        assert_eq!(documents.len(), 1);
        let doc = &documents[0];
        assert_eq!(doc.language, "it");
        assert_eq!(doc.base_name, "foo");
        assert_eq!(doc.file_path, "posts/foo.it.md");

        // Generate HTML for the document
        let output_dir = tmp_root.join("_site");
        fs::create_dir_all(&output_dir).unwrap();
        let theme = Theme::load_from_path(Path::new("themes/default")).unwrap();
        let i18n = I18nManager::new("en".to_string());
        let site = SiteConfig::default();

        templates::generate_pages(&documents, &theme, &i18n, &site, &output_dir).unwrap();

        // Verify generated file exists
        let generated = output_dir.join("posts/foo.it.html");
        assert!(generated.exists(), "expected generated file: {}", generated.display());

        // Cleanup
        let _ = fs::remove_dir_all(&tmp_root);
    }
}