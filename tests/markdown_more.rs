use krik::error::{IoError, IoErrorKind, KrikError, MarkdownError, MarkdownErrorKind};
use krik::generator::markdown::*;
use krik::parser::{Document, FrontMatter};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

#[test]
fn test_calculate_relative_path() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("content");
    let file_path = source_dir.join("posts/hello.md");

    fs::create_dir_all(&source_dir).unwrap();

    let result = calculate_relative_path(&source_dir, &file_path);
    assert_eq!(result, "posts/hello.md");
}

#[test]
fn test_calculate_relative_path_fallback() {
    let source_dir = Path::new("/nonexistent/source");
    let file_path = Path::new("/different/path/file.md");

    let result = calculate_relative_path(source_dir, file_path);
    assert_eq!(result, "/different/path/file.md");
}

#[test]
fn test_validate_not_draft_success() {
    let frontmatter = FrontMatter {
        title: Some("Test".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: Some(false),
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let result = validate_not_draft(&frontmatter, Path::new("test.md"));
    assert!(result.is_ok());
}

#[test]
fn test_validate_not_draft_none() {
    let frontmatter = FrontMatter {
        title: Some("Test".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let result = validate_not_draft(&frontmatter, Path::new("test.md"));
    assert!(result.is_ok());
}

#[test]
fn test_validate_not_draft_fails() {
    let frontmatter = FrontMatter {
        title: Some("Test".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: Some(true),
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let result = validate_not_draft(&frontmatter, Path::new("test.md"));
    assert!(result.is_err());
    assert!(is_draft_skip_error(&result.unwrap_err()));
}

#[test]
fn test_extract_file_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("hello.en.md");

    let result = extract_file_metadata(&file_path).unwrap();
    assert_eq!(result.0, "hello"); // base_name
    assert_eq!(result.1, "en"); // language
}

#[test]
fn test_extract_file_metadata_no_language() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("hello.md");

    let result = extract_file_metadata(&file_path).unwrap();
    assert_eq!(result.0, "hello"); // base_name
    assert_eq!(result.1, "en"); // default language
}

#[test]
fn test_process_markdown_content_with_toc() {
    let frontmatter = FrontMatter {
        title: Some("Test Title".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: {
            let mut map = std::collections::HashMap::new();
            map.insert("toc".to_string(), serde_yaml::Value::Bool(true));
            map
        },
    };

    let markdown = "# Heading 1\n\n## Heading 2\n\nContent here.";
    let (html, toc) = process_markdown_content(markdown, &frontmatter);

    assert!(!html.is_empty());
    assert!(!toc.is_empty());
    assert!(toc.contains("Heading 2")); // Title should be excluded
    assert!(!toc.contains("Test Title")); // Title should be excluded
}

#[test]
fn test_process_markdown_content_no_toc() {
    let frontmatter = FrontMatter {
        title: Some("Test Title".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let markdown = "# Heading 1\n\nContent here.";
    let (html, toc) = process_markdown_content(markdown, &frontmatter);

    assert!(!html.is_empty());
    assert!(toc.is_empty());
}

#[test]
fn test_create_document() {
    let frontmatter = FrontMatter {
        title: Some("Test".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let doc = create_document(
        frontmatter.clone(),
        "<h1>Test</h1>".to_string(),
        "test.md".to_string(),
        "en".to_string(),
        "test".to_string(),
        "<ul>toc</ul>".to_string(),
    );

    assert_eq!(doc.front_matter.title, Some("Test".to_string()));
    assert_eq!(doc.content, "<h1>Test</h1>");
    assert_eq!(doc.file_path, "test.md");
    assert_eq!(doc.language, "en");
    assert_eq!(doc.base_name, "test");
    assert_eq!(doc.toc, Some("<ul>toc</ul>".to_string()));
}

#[test]
fn test_create_document_empty_toc() {
    let frontmatter = FrontMatter {
        title: Some("Test".to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    let doc = create_document(
        frontmatter.clone(),
        "<h1>Test</h1>".to_string(),
        "test.md".to_string(),
        "en".to_string(),
        "test".to_string(),
        "".to_string(),
    );

    assert_eq!(doc.toc, None);
}

#[test]
fn test_is_draft_skip_error_true() {
    let error = KrikError::Markdown(MarkdownError {
        kind: MarkdownErrorKind::ParseError("Draft skipped".to_string()),
        file: PathBuf::from("test.md"),
        line: None,
        column: None,
        context: "test".to_string(),
    });

    assert!(is_draft_skip_error(&error));
}

#[test]
fn test_is_draft_skip_error_false() {
    let error = KrikError::Markdown(MarkdownError {
        kind: MarkdownErrorKind::ParseError("Other error".to_string()),
        file: PathBuf::from("test.md"),
        line: None,
        column: None,
        context: "test".to_string(),
    });

    assert!(!is_draft_skip_error(&error));
}

#[test]
fn test_is_draft_skip_error_different_error_type() {
    let error = KrikError::Io(IoError {
        kind: IoErrorKind::InvalidPath,
        path: PathBuf::from("test.md"),
        context: "test".to_string(),
    });

    assert!(!is_draft_skip_error(&error));
}

#[test]
fn test_collect_results_all_success() {
    let results = vec![
        (
            "file1.md".to_string(),
            Ok(create_test_document("File 1", "file1.md")),
        ),
        (
            "file2.md".to_string(),
            Ok(create_test_document("File 2", "file2.md")),
        ),
    ];

    let mut documents = Vec::new();
    let stats = collect_results(results, &mut documents);

    assert_eq!(stats.processed, 2);
    assert_eq!(stats.skipped, 0);
    assert_eq!(stats.errors, 0);
    assert_eq!(documents.len(), 2);
}

#[test]
fn test_collect_results_with_draft_skip() {
    let draft_error = KrikError::Markdown(MarkdownError {
        kind: MarkdownErrorKind::ParseError("Draft skipped".to_string()),
        file: PathBuf::from("draft.md"),
        line: None,
        column: None,
        context: "test".to_string(),
    });

    let results = vec![
        (
            "file1.md".to_string(),
            Ok(create_test_document("File 1", "file1.md")),
        ),
        ("draft.md".to_string(), Err(draft_error)),
    ];

    let mut documents = Vec::new();
    let stats = collect_results(results, &mut documents);

    assert_eq!(stats.processed, 1);
    assert_eq!(stats.skipped, 1);
    assert_eq!(stats.errors, 0);
    assert_eq!(documents.len(), 1);
}

#[test]
fn test_collect_results_with_error() {
    let error = KrikError::Io(IoError {
        kind: IoErrorKind::InvalidPath,
        path: PathBuf::from("error.md"),
        context: "test".to_string(),
    });

    let results = vec![
        (
            "file1.md".to_string(),
            Ok(create_test_document("File 1", "file1.md")),
        ),
        ("error.md".to_string(), Err(error)),
    ];

    let mut documents = Vec::new();
    let stats = collect_results(results, &mut documents);

    assert_eq!(stats.processed, 1);
    assert_eq!(stats.skipped, 0);
    assert_eq!(stats.errors, 1);
    assert_eq!(documents.len(), 1);
}

// Helper function to create test documents
fn create_test_document(title: &str, file_path: &str) -> Document {
    let frontmatter = FrontMatter {
        title: Some(title.to_string()),
        date: None,
        tags: None,
        lang: None,
        draft: None,
        pdf: None,
        extra: std::collections::HashMap::new(),
    };

    create_document(
        frontmatter,
        format!("<h1>{}</h1>", title),
        file_path.to_string(),
        "en".to_string(),
        title.to_lowercase().replace(' ', "_"),
        String::new(),
    )
}
