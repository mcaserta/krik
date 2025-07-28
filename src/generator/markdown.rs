use crate::parser::{Document, extract_language_from_filename, parse_markdown_with_frontmatter};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::path::Path;
use walkdir::WalkDir;

/// Scan files in the source directory and parse markdown documents
pub fn scan_files(source_dir: &Path, documents: &mut Vec<Document>) -> Result<(), Box<dyn std::error::Error>> {
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

        // Read and parse the file
        let content = std::fs::read_to_string(path)?;
        
        match parse_markdown_with_frontmatter(&content) {
            Ok((frontmatter, markdown_content)) => {
                // Skip drafts
                if frontmatter.draft.unwrap_or(false) {
                    continue;
                }

                // Extract language from filename
                let filename_without_ext = path.file_stem().unwrap().to_string_lossy();
                let (base_name, language) = extract_language_from_filename(&filename_without_ext);
                
                // Determine relative path from source directory
                let relative_path = path.strip_prefix(source_dir)
                    .map_err(|_| format!("Failed to get relative path for: {}", path.display()))?;

                // Convert to HTML content
                let html_content = markdown_to_html(&markdown_content);

                let document = Document {
                    front_matter: frontmatter,
                    content: html_content,
                    file_path: relative_path.to_string_lossy().to_string(),
                    language,
                    base_name,
                };

                documents.push(document);
            }
            Err(e) => {
                eprintln!("Warning: Failed to parse {}: {}", path.display(), e);
                continue;
            }
        }
    }

    Ok(())
}

/// Convert markdown content to HTML with proper options
pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Generate table of contents and process content for TOC-enabled documents
pub fn generate_toc_and_content(content: &str, title: Option<&str>) -> (String, String) {
    let heading_regex = Regex::new(r"<h([1-6])(?:[^>]*)>([^<]+)</h[1-6]>").unwrap();
    let mut toc_html = String::new();
    let mut processed_content = content.to_string();
    let mut heading_count = 0;

    // Generate TOC and add IDs to headings
    for cap in heading_regex.captures_iter(content) {
        let level = cap[1].parse::<u8>().unwrap_or(1);
        let heading_text = &cap[2];
        let heading_id = format!("heading-{}", heading_count);
        
        // Add to TOC (skip h1 if it matches the title)
        if !(level == 1 && title.map_or(false, |t| t.trim() == heading_text.trim())) {
            let indent = "  ".repeat((level.saturating_sub(1)) as usize);
            toc_html.push_str(&format!(
                "{}<li><a href=\"#{}\">{}</a></li>\n",
                indent, heading_id, heading_text
            ));
        }

        // Replace heading in content with ID
        let original_heading = &cap[0];
        let new_heading = original_heading.replace(
            &format!("<h{}", level),
            &format!("<h{} id=\"{}\"", level, heading_id)
        );
        processed_content = processed_content.replace(original_heading, &new_heading);
        
        heading_count += 1;
    }

    if !toc_html.is_empty() {
        toc_html = format!("<ul class=\"toc\">\n{}</ul>", toc_html);
    }

    (toc_html, processed_content)
}

/// Remove duplicate H1 title from content if it matches the frontmatter title
pub fn remove_duplicate_title(content: &str, title: Option<&str>) -> String {
    if let Some(title) = title {
        let h1_regex = Regex::new(r"<h1[^>]*>([^<]+)</h1>").unwrap();
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
    let footnote_regex = Regex::new(r#"<li id="([^"]+)"><p>([^<]+)(?:<a href="[^"]+">↩</a>)?</p></li>"#).unwrap();
    
    footnote_regex.replace_all(content, |caps: &regex::Captures| {
        let footnote_id = &caps[1];
        let footnote_text = &caps[2];
        let return_link_id = footnote_id.replace("fn:", "fnref:");
        
        format!(
            r#"<li id="{}"><p>{}<a href="{}" class="footnote-return">↩</a></p></li>"#,
            footnote_id, footnote_text, format!("#{}", return_link_id)
        )
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let html = markdown_to_html(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>"));
    }

    #[test]
    fn test_generate_toc_and_content() {
        let content = "<h1>Title</h1><h2>Section 1</h2><h3>Subsection</h3>";
        let (toc, processed) = generate_toc_and_content(content, Some("Title"));
        
        assert!(toc.contains("Section 1"));
        assert!(toc.contains("Subsection"));
        assert!(!toc.contains("Title")); // h1 matching title should be excluded
        assert!(processed.contains("id=\"heading-"));
    }
}