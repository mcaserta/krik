use crate::parser::{Document, extract_language_from_filename, parse_markdown_with_frontmatter_for_file};
use crate::generator::ast_parser::{parse_markdown_ast, generate_toc_from_headings};
use crate::error::{KrikResult, KrikError, IoError, IoErrorKind, MarkdownError, MarkdownErrorKind};
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use tracing::{info, debug, warn};


/// Scan files in the source directory and parse markdown documents
pub fn scan_files(source_dir: &Path, documents: &mut Vec<Document>) -> KrikResult<()> {
    info!("Starting file scan in: {}", source_dir.display());
    
    let entries = collect_markdown_files(source_dir);
    let results = process_files_parallel(&entries, source_dir);
    let scan_stats = collect_results(results, documents);
    
    info!("File scan completed: {} processed, {} skipped, {} errors", 
          scan_stats.processed, scan_stats.skipped, scan_stats.errors);
    Ok(())
}

/// Convert markdown content to HTML with optional TOC generation
/// Uses AST-based parsing for consistent heading IDs and robust processing
pub fn markdown_to_html(markdown: &str, with_toc: bool, title: Option<&str>) -> (String, String) {
    let result = parse_markdown_ast(markdown);
    let toc_html = if with_toc {
        generate_toc_from_headings(&result.headings, title)
    } else {
        String::new()
    };
    (result.html_content, toc_html)
}

/// Parse a single markdown file given the site `source_dir` and the file's absolute path
pub fn parse_single_file(source_dir: &Path, path: &Path) -> KrikResult<Document> {
    let rel_path = calculate_relative_path(source_dir, path);
    let content = read_file_content(path)?;
    let (frontmatter, markdown_content) = parse_markdown_with_frontmatter_for_file(&content, path)?;
    
    validate_not_draft(&frontmatter, path)?;
    
    let (base_name, language) = extract_file_metadata(path)?;
    let (html_content, toc_html) = process_markdown_content(&markdown_content, &frontmatter);
    
    Ok(create_document(
        frontmatter,
        html_content,
        rel_path,
        language,
        base_name,
        toc_html,
    ))
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
        if !(level == 1 && title.is_some_and(|t| t.trim() == text)) {
            let id = text
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                .collect::<String>()
                .replace(' ', "-")
                .trim_matches('-')
                .to_string();

            let indent = "  ".repeat((level - 1) as usize);
            toc_items.push(format!("{indent}<li><a href=\"#{id}\">{text}</a></li>"));
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

/// Statistics for file scanning operations
#[derive(Debug, Default)]
struct ScanStats {
    processed: usize,
    skipped: usize,
    errors: usize,
}

/// Collect all markdown files from the source directory
fn collect_markdown_files(source_dir: &Path) -> Vec<walkdir::DirEntry> {
    WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .collect()
}

/// Process files in parallel and return results
fn process_files_parallel(entries: &[walkdir::DirEntry], source_dir: &Path) -> Vec<(String, Result<Document, KrikError>)> {
    let mut results: Vec<(String, Result<Document, KrikError>)> = entries
        .par_iter()
        .map(|entry| {
            let path = entry.path();
            let rel_path = calculate_relative_path(source_dir, path);
            let result = process_single_markdown_file(path, &rel_path);
            (rel_path, result)
        })
        .collect();

    // Sort by path to keep deterministic order when pushing into documents
    results.sort_by(|a, b| a.0.cmp(&b.0));
    results
}

/// Process a single markdown file and return a Document
fn process_single_markdown_file(path: &Path, rel_path: &str) -> Result<Document, KrikError> {
    debug!("Processing file: {}", path.display());
    
    let content = read_file_content(path)?;
    let (frontmatter, markdown_content) = parse_markdown_with_frontmatter_for_file(&content, path)?;
    
    validate_not_draft(&frontmatter, path)?;
    
    let (base_name, language) = extract_file_metadata(path)?;
    let (html_content, toc_html) = process_markdown_content(&markdown_content, &frontmatter);
    
    Ok(create_document(
        frontmatter,
        html_content,
        rel_path.to_string(),
        language,
        base_name,
        toc_html,
    ))
}

/// Collect results from file processing and update documents vector
fn collect_results(results: Vec<(String, Result<Document, KrikError>)>, documents: &mut Vec<Document>) -> ScanStats {
    let mut stats = ScanStats::default();
    
    for (path_str, res) in results {
        match res {
            Ok(doc) => {
                documents.push(doc);
                stats.processed += 1;
                debug!("Successfully processed: {}", path_str);
            }
            Err(e) => {
                // Treat a draft skip as a soft skip; others are errors
                if is_draft_skip_error(&e) {
                    stats.skipped += 1;
                } else {
                    warn!("Failed to parse {}: {}", path_str, e);
                    stats.errors += 1;
                }
            }
        }
    }
    
    stats
}

/// Calculate relative path from source directory to file
fn calculate_relative_path(source_dir: &Path, path: &Path) -> String {
    path.strip_prefix(source_dir)
        .ok()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}

/// Read file content with error handling
fn read_file_content(path: &Path) -> KrikResult<String> {
    std::fs::read_to_string(path).map_err(|e| KrikError::Io(IoError {
        kind: IoErrorKind::ReadFailed(e),
        path: path.to_path_buf(),
        context: "Reading markdown file".to_string(),
    }))
}

/// Validate that a document is not a draft
fn validate_not_draft(frontmatter: &crate::parser::FrontMatter, path: &Path) -> KrikResult<()> {
    if frontmatter.draft.unwrap_or(false) {
        return Err(KrikError::Markdown(MarkdownError {
            kind: MarkdownErrorKind::ParseError("Draft skipped".to_string()),
            file: path.to_path_buf(),
            line: None,
            column: None,
            context: "Skipping draft file".to_string(),
        }));
    }
    Ok(())
}

/// Extract base name and language from file path
fn extract_file_metadata(path: &Path) -> KrikResult<(String, String)> {
    let filename_without_ext = path
        .file_stem()
        .ok_or_else(|| KrikError::Io(IoError {
            kind: IoErrorKind::InvalidPath,
            path: path.to_path_buf(),
            context: "Extracting filename stem".to_string(),
        }))?
        .to_string_lossy();
    
    extract_language_from_filename(&filename_without_ext)
}

/// Process markdown content and generate HTML with optional TOC
fn process_markdown_content(markdown_content: &str, frontmatter: &crate::parser::FrontMatter) -> (String, String) {
    let with_toc = frontmatter
        .extra
        .get("toc")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    markdown_to_html(markdown_content, with_toc, frontmatter.title.as_deref())
}

/// Create a Document with the provided components
fn create_document(
    front_matter: crate::parser::FrontMatter,
    content: String,
    file_path: String,
    language: String,
    base_name: String,
    toc_html: String,
) -> Document {
    Document {
        front_matter,
        content,
        file_path,
        language,
        base_name,
        toc: if toc_html.is_empty() { None } else { Some(toc_html) },
    }
}

/// Check if an error is a draft skip error
fn is_draft_skip_error(error: &KrikError) -> bool {
    matches!(error, KrikError::Markdown(MarkdownError { 
        kind: MarkdownErrorKind::ParseError(msg), .. 
    }) if msg == "Draft skipped")
}

// tests moved to tests/ directory