use krik::generator::templates::render_page::*;
use krik::parser::{Document, FrontMatter};
use std::collections::HashMap;
use tempfile::TempDir;
use tera::Context;

fn create_test_document() -> Document {
    let mut extra = HashMap::new();
    extra.insert(
        "author".to_string(),
        serde_yaml::Value::String("Test Author".to_string()),
    );

    Document {
        front_matter: FrontMatter {
            title: Some("Test Document".to_string()),
            date: Some(chrono::Utc::now()),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            lang: None,
            draft: None,
            pdf: Some(true),
            extra,
        },
        content: "<h1>Test Content</h1><p>This is test content.</p>".to_string(),
        file_path: "posts/test.md".to_string(),
        language: "en".to_string(),
        base_name: "test".to_string(),
        toc: Some("<ul><li><a href=\"#section\">Section</a></li></ul>".to_string()),
    }
}

#[test]
fn test_create_base_context() {
    let document = create_test_document();
    let context = create_base_context(&document);

    // Test basic fields
    assert_eq!(
        context.get("title").unwrap().as_str().unwrap(),
        "Test Document"
    );
    assert_eq!(context.get("language").unwrap().as_str().unwrap(), "en");
    assert_eq!(context.get("base_name").unwrap().as_str().unwrap(), "test");
    assert_eq!(context.get("pdf").unwrap().as_bool().unwrap(), true);

    // Test content
    assert!(context.get("content").is_some());

    // Test tags
    let tags = context.get("tags").unwrap().as_array().unwrap();
    assert_eq!(tags.len(), 2);
    assert_eq!(tags[0].as_str().unwrap(), "tag1");
    assert_eq!(tags[1].as_str().unwrap(), "tag2");

    // Test extra fields
    assert_eq!(
        context.get("author").unwrap().as_str().unwrap(),
        "Test Author"
    );

    // Test description is present
    assert!(context.get("description").is_some());
}

#[test]
fn test_create_base_context_minimal() {
    let document = Document {
        front_matter: FrontMatter {
            title: None,
            date: None,
            tags: None,
            lang: None,
            draft: None,
            pdf: None,
            extra: HashMap::new(),
        },
        content: "Simple content".to_string(),
        file_path: "simple.md".to_string(),
        language: "en".to_string(),
        base_name: "simple".to_string(),
        toc: None,
    };

    let context = create_base_context(&document);

    // Test that None values are handled correctly
    assert!(context.get("title").unwrap().is_null());
    assert!(context.get("date").unwrap().is_null());
    assert!(context.get("tags").unwrap().is_null());
    assert!(context.get("pdf").unwrap().is_null());

    // Test non-null fields
    assert_eq!(context.get("language").unwrap().as_str().unwrap(), "en");
    assert_eq!(
        context.get("base_name").unwrap().as_str().unwrap(),
        "simple"
    );
    assert_eq!(
        context.get("content").unwrap().as_str().unwrap(),
        "Simple content"
    );
}

#[test]
fn test_add_processed_content() {
    let mut document = create_test_document();
    document.front_matter.title = Some("Test Title".to_string());
    document.content = "<h1>Test Title</h1><p>Content here.</p>".to_string();

    let mut context = Context::new();
    context.insert("content", &document.content);

    add_processed_content(&mut context, &document);

    // Test that duplicate title is removed
    let processed_content = context.get("content").unwrap().as_str().unwrap();
    assert!(!processed_content.contains("<h1>Test Title</h1>"));
    assert!(processed_content.contains("<p>Content here.</p>"));
}

#[test]
fn test_add_processed_content_with_toc() {
    let document = create_test_document();
    let mut context = Context::new();
    context.insert("content", &document.content);

    add_processed_content(&mut context, &document);

    // Test that TOC is added
    assert_eq!(
        context.get("toc").unwrap().as_str().unwrap(),
        "<ul><li><a href=\"#section\">Section</a></li></ul>"
    );
}

#[test]
fn test_add_processed_content_no_toc() {
    let mut document = create_test_document();
    document.toc = None;

    let mut context = Context::new();
    context.insert("content", &document.content);

    add_processed_content(&mut context, &document);

    // Test that no TOC key is set when toc is None
    assert!(context.get("toc").is_none());
}

#[test]
fn test_write_output_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path();
    let document = create_test_document();
    let rendered_content = "<html><body>Test</body></html>";

    let result = write_output_file(&document, output_dir, rendered_content);
    assert!(result.is_ok());

    // Check that file was created
    let output_path = output_dir.join("posts/test.html");
    assert!(output_path.exists());

    // Check file contents
    let file_content = std::fs::read_to_string(&output_path).unwrap();
    assert_eq!(file_content, rendered_content);
}

#[test]
fn test_write_output_file_creates_directories() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path();

    let document = Document {
        front_matter: FrontMatter {
            title: Some("Test".to_string()),
            date: None,
            tags: None,
            lang: None,
            draft: None,
            pdf: None,
            extra: HashMap::new(),
        },
        content: "content".to_string(),
        file_path: "deep/nested/path/test.md".to_string(),
        language: "en".to_string(),
        base_name: "test".to_string(),
        toc: None,
    };

    let rendered_content = "<html>test</html>";

    let result = write_output_file(&document, output_dir, rendered_content);
    assert!(result.is_ok());

    // Check that nested directories were created
    let output_path = output_dir.join("deep/nested/path/test.html");
    assert!(output_path.exists());
    assert!(output_path.parent().unwrap().is_dir());
}
