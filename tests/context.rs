use krik::generator::templates::context::*;

#[test]
fn test_clean_frontmatter_description() {
    let desc = "Line 1\nLine 2\r\nLine 3".to_string();
    let result = clean_frontmatter_description(&desc);
    assert_eq!(result, "Line 1 Line 2  Line 3");
}

#[test]
fn test_clean_frontmatter_description_with_trim() {
    let desc = "  \n  Trimmed content  \r\n  ".to_string();
    let result = clean_frontmatter_description(&desc);
    assert_eq!(result, "Trimmed content");
}

#[test]
fn test_strip_html_tags() {
    let content = "<h1>Title</h1><p>This is <strong>bold</strong> text.</p>";
    let result = strip_html_tags(content);
    assert_eq!(result, " Title  This is  bold  text. ");
}

#[test]
fn test_strip_html_tags_malformed() {
    let content = "<h1>Title<p>Missing closing tag";
    let result = strip_html_tags(content);
    assert_eq!(result, " Title Missing closing tag");
}

#[test]
fn test_strip_html_tags_no_tags() {
    let content = "Plain text with no tags";
    let result = strip_html_tags(content);
    assert_eq!(result, "Plain text with no tags");
}

#[test]
fn test_normalize_whitespace() {
    let text = "  Multiple   spaces    and\n\ttabs  ";
    let result = normalize_whitespace(text);
    assert_eq!(result, "Multiple spaces and tabs");
}

#[test]
fn test_normalize_whitespace_empty() {
    let text = "";
    let result = normalize_whitespace(text);
    assert_eq!(result, "");
}

#[test]
fn test_normalize_whitespace_only_spaces() {
    let text = "   \t\n  ";
    let result = normalize_whitespace(text);
    assert_eq!(result, "");
}

#[test]
fn test_truncate_description_short() {
    let text = "Short text";
    let result = truncate_description(text, 160);
    assert_eq!(result, "Short text");
}

#[test]
fn test_truncate_description_long() {
    let text = "a".repeat(200);
    let result = truncate_description(&text, 160);
    assert_eq!(result.len(), 160);
    assert!(result.ends_with("..."));
    assert_eq!(result, format!("{}...", "a".repeat(157)));
}

#[test]
fn test_truncate_description_exact_length() {
    let text = "a".repeat(160);
    let result = truncate_description(&text, 160);
    assert_eq!(result, text);
}

#[test]
fn test_truncate_description_zero_length() {
    let text = "Some text";
    let result = truncate_description(text, 0);
    assert_eq!(result, "...");
}

#[test]
fn test_extract_description_from_content() {
    let content = "<h1>Title</h1><p>This is the first paragraph with some content.</p><p>Second paragraph.</p>";
    let result = extract_description_from_content(content);

    assert!(!result.contains('<'));
    assert!(!result.contains('>'));
    assert!(result.contains("Title"));
    assert!(result.contains("first paragraph"));

    if result.len() > 160 {
        assert!(result.ends_with("..."));
    }
}

#[test]
fn test_extract_description_from_content_short() {
    let content = "<p>Short content</p>";
    let result = extract_description_from_content(content);
    assert_eq!(result, "Short content");
}

#[test]
fn test_generate_description_with_frontmatter() {
    let content = "<h1>Title</h1><p>Content here</p>";
    let frontmatter_desc = Some("Custom description from frontmatter".to_string());

    let result = generate_description(content, frontmatter_desc.as_ref());
    assert_eq!(result, "Custom description from frontmatter");
}

#[test]
fn test_generate_description_without_frontmatter() {
    let content = "<h1>Title</h1><p>Content extracted from HTML</p>";

    let result = generate_description(content, None);
    assert_eq!(result, "Title Content extracted from HTML");
}

#[test]
fn test_generate_description_frontmatter_with_newlines() {
    let content = "<h1>Title</h1><p>Content here</p>";
    let frontmatter_desc = Some("Description\nwith\rnewlines".to_string());

    let result = generate_description(content, frontmatter_desc.as_ref());
    assert_eq!(result, "Description with newlines");
}

#[test]
fn test_generate_description_long_content() {
    let long_content = format!("<p>{}</p>", "word ".repeat(50));

    let result = generate_description(&long_content, None);
    assert!(result.len() <= 160);
    if result.len() == 160 {
        assert!(result.ends_with("..."));
    }
}
