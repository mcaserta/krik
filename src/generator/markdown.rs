use crate::parser::{Document, extract_language_from_filename, parse_markdown_with_frontmatter_for_file};
use crate::generator::ast_parser::{parse_markdown_ast, generate_toc_from_headings};
use crate::error::{KrikResult, KrikError, IoError, IoErrorKind};
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;
use tracing::{info, debug, warn};

use pulldown_cmark::{html, Options, Parser};

/// Scan files in the source directory and parse markdown documents
pub fn scan_files(source_dir: &Path, documents: &mut Vec<Document>) -> KrikResult<()> {
    info!("Starting file scan in: {}", source_dir.display());
    let mut processed_files = 0;
    let mut skipped_files = 0;
    let mut error_files = 0;

    for entry in WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories and non-markdown files
        if !path.is_file() || !path.extension().map_or(false, |ext| ext == "md") {
            continue;
        }

        // Skip site.toml (site configuration)
        if path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
            continue;
        }

        debug!("Processing file: {}", path.display());

        // Read and parse the file
        let content = std::fs::read_to_string(path)
            .map_err(|e| KrikError::Io(IoError {
                kind: IoErrorKind::ReadFailed(e),
                path: path.to_path_buf(),
                context: "Reading markdown file".to_string(),
            }))?;
        
        match parse_markdown_with_frontmatter_for_file(&content, path) {
            Ok((frontmatter, markdown_content)) => {
                // Skip drafts
                if frontmatter.draft.unwrap_or(false) {
                    debug!("Skipping draft: {}", path.display());
                    skipped_files += 1;
                    continue;
                }

                // Extract language from filename
                let filename_without_ext = path.file_stem()
                    .ok_or_else(|| KrikError::Io(IoError {
                        kind: IoErrorKind::InvalidPath,
                        path: path.to_path_buf(),
                        context: "Extracting filename stem".to_string(),
                    }))?
                    .to_string_lossy();
                let (base_name, language) = extract_language_from_filename(&filename_without_ext)?;
                
                // Determine relative path from source directory
                let relative_path = path.strip_prefix(source_dir)
                    .map_err(|_| KrikError::Io(IoError {
                        kind: IoErrorKind::InvalidPath,
                        path: path.to_path_buf(),
                        context: "Getting relative path from source directory".to_string(),
                    }))?;

                // Convert to HTML content and generate TOC if enabled
                let (html_content, toc_html) = if frontmatter.extra.get("toc").and_then(|v| v.as_bool()).unwrap_or(false) {
                    markdown_to_html_with_toc(&markdown_content, frontmatter.title.as_deref())
                } else {
                    (markdown_to_html(&markdown_content), String::new())
                };

                let document = Document {
                    front_matter: frontmatter,
                    content: html_content,
                    file_path: relative_path.to_string_lossy().to_string(),
                    language,
                    base_name,
                    toc: if toc_html.is_empty() { None } else { Some(toc_html) },
                };

                documents.push(document);
                processed_files += 1;
                debug!("Successfully processed: {}", path.display());
            }
            Err(e) => {
                // Log error but continue processing other files
                warn!("Failed to parse {}: {}", path.display(), e);
                error_files += 1;
                continue;
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
}