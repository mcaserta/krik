use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Front matter metadata extracted from the YAML header of Markdown files.
///
/// Front matter appears at the beginning of Markdown files between `---` delimiters
/// and contains metadata about the document in YAML format.
///
/// # Example
///
/// ```yaml
/// ---
/// title: "My Blog Post"
/// date: 2024-01-15T10:30:00Z
/// tags: ["rust", "web"]
/// draft: false
/// custom_field: "custom value"
/// ---
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    /// The title of the document (used in HTML title and navigation)
    pub title: Option<String>,
    /// Publication date in ISO 8601 format (falls back to file modification time)
    pub date: Option<DateTime<Utc>>,
    /// Array of tags for categorization (displayed on post templates)
    pub tags: Option<Vec<String>>,
    /// Language code for this document (usually auto-detected from filename)
    pub lang: Option<String>,
    /// Whether this document should be skipped during generation
    pub draft: Option<bool>,
    /// Additional custom fields accessible in templates
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

/// A parsed Markdown document with its metadata and content.
///
/// Represents a single Markdown file that has been parsed and is ready
/// for processing into HTML. Contains both the front matter metadata
/// and the Markdown content body.
#[derive(Debug, Clone, Serialize)]
pub struct Document {
    /// Parsed YAML front matter containing metadata
    pub front_matter: FrontMatter,
    /// Raw Markdown content (without front matter)
    pub content: String,
    /// Original file path of the source Markdown file
    pub file_path: String,
    /// Detected language code (e.g., "en", "it", "es")
    pub language: String,
    /// Base filename without language suffix or extension
    pub base_name: String,
}

/// Parses a Markdown document with YAML front matter.
///
/// Extracts YAML front matter from the beginning of a Markdown document and returns
/// both the parsed metadata and the remaining Markdown content.
///
/// # Arguments
///
/// * `content` - The raw Markdown content including front matter
///
/// # Returns
///
/// Returns a tuple of `(FrontMatter, String)` where the first element contains
/// parsed metadata and the second contains the Markdown content.
///
/// # Example
///
/// ```rust
/// use krik::parser::parse_markdown_with_frontmatter;
///
/// let content = r#"---
/// title: "Hello World"
/// date: 2024-01-15T10:30:00Z
/// ---
///
/// # Hello World
///
/// This is the content.
/// "#;
///
/// let (front_matter, markdown) = parse_markdown_with_frontmatter(content)?;
/// assert_eq!(front_matter.title, Some("Hello World".to_string()));
/// assert!(markdown.contains("Hello World"));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_markdown_with_frontmatter(content: &str) -> Result<(FrontMatter, String), Box<dyn std::error::Error>> {
    if let Some(stripped) = content.strip_prefix("---\n") {
        if let Some(end_pos) = stripped.find("\n---\n") {
            let yaml_content = &stripped[..end_pos];
            let markdown_content = &stripped[end_pos + 5..];
            
            let front_matter: FrontMatter = serde_yaml::from_str(yaml_content)?;
            return Ok((front_matter, markdown_content.to_string()));
        }
    }
    
    Ok((FrontMatter {
        title: None,
        date: None,
        tags: None,
        lang: None,
        draft: None,
        extra: HashMap::new(),
    }, content.to_string()))
}

pub fn extract_language_from_filename(filename: &str) -> (String, String) {
    // filename is already without extension (e.g., "sample.it" or "sample")
    if let Some(dot_pos) = filename.rfind('.') {
        let base_part = &filename[..dot_pos];
        let potential_lang = &filename[dot_pos + 1..];
        if potential_lang.len() == 2 {
            return (base_part.to_string(), potential_lang.to_string());
        }
    }
    (filename.to_string(), "en".to_string())
}