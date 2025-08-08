use pulldown_cmark::{Event, Tag, TagEnd, Options, Parser, HeadingLevel};
use std::collections::HashMap;
use regex::Regex;

/// Represents a heading in the document structure
#[derive(Debug, Clone)]
pub struct Heading {
    pub level: HeadingLevel,
    pub text: String,
    pub id: String,
    pub line_number: usize,
}

/// Represents a footnote reference or definition
#[derive(Debug, Clone)]
pub struct Footnote {
    pub id: String,
    pub reference_text: String,
    pub definition_text: String,
    pub reference_line: usize,
    pub definition_line: usize,
}

/// AST parsing result containing headings and footnotes
#[derive(Debug, Clone)]
pub struct AstParseResult {
    pub headings: Vec<Heading>,
    pub footnotes: HashMap<String, Footnote>,
    pub html_content: String,
}

/// Parse markdown content using AST to extract headings and footnotes
pub fn parse_markdown_ast(markdown: &str) -> AstParseResult {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown, options);
    let mut ast_parser = AstParser::new();
    
    // Collect headings and footnotes
    let events: Vec<_> = parser.collect();
    for event in &events {
        ast_parser.process_event(event.clone());
    }
    
    // Generate HTML using default pulldown-cmark HTML generation
    let mut html_output = String::new();
    use pulldown_cmark::html::push_html;
    push_html(&mut html_output, events.into_iter());
    
    // Post-process HTML to add IDs to headings
    let processed_html = add_heading_ids_to_html(&html_output, &ast_parser.headings);
    
    AstParseResult {
        headings: ast_parser.headings,
        footnotes: ast_parser.footnotes,
        html_content: processed_html,
    }
}

/// AST parser that collects headings and footnotes
struct AstParser {
    headings: Vec<Heading>,
    footnotes: HashMap<String, Footnote>,
    current_heading_text: String,
    current_heading_level: Option<HeadingLevel>,
    current_footnote_id: Option<String>,
    current_footnote_text: String,
    line_number: usize,
    in_heading: bool,
    in_footnote_definition: bool,
}

impl AstParser {
    fn new() -> Self {
        Self {
            headings: Vec::new(),
            footnotes: HashMap::new(),
            current_heading_text: String::new(),
            current_heading_level: None,
            current_footnote_id: None,
            current_footnote_text: String::new(),
            line_number: 1,
            in_heading: false,
            in_footnote_definition: false,
        }
    }
    
    fn process_event(&mut self, event: Event) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                self.current_heading_level = Some(level);
                self.current_heading_text.clear();
                self.in_heading = true;
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(heading_level) = self.current_heading_level {
                    let heading_id = self.generate_heading_id(&self.current_heading_text);
                    let heading = Heading {
                        level: heading_level,
                        text: self.current_heading_text.clone(),
                        id: heading_id,
                        line_number: self.line_number,
                    };
                    self.headings.push(heading);
                    self.current_heading_level = None;
                    self.in_heading = false;
                }
            }
            Event::Text(text) => {
                if self.in_heading {
                    self.current_heading_text.push_str(&text);
                } else if self.in_footnote_definition {
                    self.current_footnote_text.push_str(&text);
                }
            }
            Event::Start(Tag::FootnoteDefinition(footnote_id)) => {
                self.current_footnote_id = Some(footnote_id.to_string());
                self.current_footnote_text.clear();
                self.in_footnote_definition = true;
            }
            Event::End(TagEnd::FootnoteDefinition) => {
                if let Some(footnote_id) = self.current_footnote_id.take() {
                    let footnote = Footnote {
                        id: footnote_id.clone(),
                        reference_text: String::new(), // Will be filled by reference processing
                        definition_text: self.current_footnote_text.clone(),
                        reference_line: 0, // Will be filled by reference processing
                        definition_line: self.line_number,
                    };
                    self.footnotes.insert(footnote_id, footnote);
                    self.in_footnote_definition = false;
                }
            }
            Event::FootnoteReference(footnote_id) => {
                // Process footnote reference
                let footnote_id_str = footnote_id.to_string();
                if let Some(footnote) = self.footnotes.get_mut(&footnote_id_str) {
                    footnote.reference_line = self.line_number;
                } else {
                    // Create footnote entry if it doesn't exist yet
                    let footnote = Footnote {
                        id: footnote_id_str.clone(),
                        reference_text: String::new(),
                        definition_text: String::new(),
                        reference_line: self.line_number,
                        definition_line: 0,
                    };
                    self.footnotes.insert(footnote_id_str, footnote);
                }
            }
            Event::HardBreak | Event::SoftBreak => {
                self.line_number += 1;
            }
            _ => {}
        }
    }
    
    fn generate_heading_id(&self, text: &str) -> String {
        // Generate a URL-friendly ID from heading text
        let mut id = text
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
        
        // Replace spaces with hyphens and remove multiple hyphens
        id = id.replace(' ', "-");
        while id.contains("--") {
            id = id.replace("--", "-");
        }
        
        // Remove leading/trailing hyphens
        id = id.trim_matches('-').to_string();
        
        // Ensure uniqueness by adding counter if needed
        let base_id = id.clone();
        let mut counter = 1;
        while self.headings.iter().any(|h| h.id == id) {
            id = format!("{base_id}-{counter}");
            counter += 1;
        }
        
        id
    }
}

/// Add heading IDs to HTML content
fn add_heading_ids_to_html(html: &str, headings: &[Heading]) -> String {
    let mut result = html.to_string();
    
    // Use regex to find and replace heading tags
    let heading_regex = Regex::new(r"<h([1-6])([^>]*)>([^<]*)</h[1-6]>").unwrap();
    
    result = heading_regex.replace_all(&result, |caps: &regex::Captures| {
        let level = &caps[1];
        let attrs = &caps[2];
        let text = &caps[3];
        
        // Find matching heading by text content
        if let Some(heading) = headings.iter().find(|h| h.text.trim() == text.trim()) {
            format!("<h{}{} id=\"{}\">{}</h{}>", level, attrs, heading.id, text, level)
        } else {
            // If no match found, just return the original
            caps[0].to_string()
        }
    }).to_string();
    
    result
}

/// Generate table of contents from parsed headings
pub fn generate_toc_from_headings(headings: &[Heading], title: Option<&str>) -> String {
    let mut toc_html = String::new();
    
    for heading in headings {
        // Skip h1 if it matches the title
        if !(heading.level == HeadingLevel::H1 && title.is_some_and(|t| t.trim() == heading.text.trim())) {
            let indent = "  ".repeat((heading.level as u8).saturating_sub(1) as usize);
            toc_html.push_str(&format!(
                "{}<li><a href=\"#{}\">{}</a></li>\n",
                indent, heading.id, heading.text
            ));
        }
    }
    
    if !toc_html.is_empty() {
        toc_html = format!("<ul class=\"toc\">\n{toc_html}</ul>");
    }
    
    toc_html
}

/// Process footnotes to add proper navigation
pub fn process_footnotes_ast(footnotes: &HashMap<String, Footnote>) -> HashMap<String, Footnote> {
    let mut processed_footnotes = footnotes.clone();
    
    for (id, footnote) in processed_footnotes.iter_mut() {
        // Ensure proper ID format
        if !id.starts_with("fn:") {
            footnote.id = format!("fn:{id}");
        }
    }
    
    processed_footnotes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_markdown_ast() {
        let markdown = "# Title\n\n## Section 1\n\nThis has a footnote[^1].\n\n[^1]: This is the footnote.";
        let result = parse_markdown_ast(markdown);
        
        assert_eq!(result.headings.len(), 2);
        assert_eq!(result.headings[0].text, "Title");
        assert_eq!(result.headings[1].text, "Section 1");
        assert_eq!(result.footnotes.len(), 1);
        assert!(result.html_content.contains("id=\"title\""));
        assert!(result.html_content.contains("id=\"section-1\""));
    }

    #[test]
    fn test_generate_toc_from_headings() {
        let headings = vec![
            Heading { level: HeadingLevel::H1, text: "Title".to_string(), id: "title".to_string(), line_number: 1 },
            Heading { level: HeadingLevel::H2, text: "Section 1".to_string(), id: "section-1".to_string(), line_number: 3 },
            Heading { level: HeadingLevel::H3, text: "Subsection".to_string(), id: "subsection".to_string(), line_number: 5 },
        ];
        
        let toc = generate_toc_from_headings(&headings, Some("Title"));
        assert!(toc.contains("Section 1"));
        assert!(toc.contains("Subsection"));
        assert!(!toc.contains("Title")); // Should be excluded
    }

    #[test]
    fn test_heading_id_generation() {
        let markdown = "# My Heading\n\n## Another Heading\n\n# My Heading";
        let result = parse_markdown_ast(markdown);
        
        assert_eq!(result.headings.len(), 3);
        assert_eq!(result.headings[0].id, "my-heading");
        assert_eq!(result.headings[1].id, "another-heading");
        assert_eq!(result.headings[2].id, "my-heading-1"); // Should be unique
    }
}
