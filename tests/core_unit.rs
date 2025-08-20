use krik::generator::core::*;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_analyze_change_type_theme_related() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = theme_path.join("template.html");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    assert!(matches!(result, ChangeType::ThemeRelated));
}

#[test]
fn test_analyze_change_type_html_template() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = temp_dir.path().join("templates/page.html");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::create_dir_all(changed_path.parent().unwrap()).unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    assert!(matches!(result, ChangeType::ThemeRelated));
}

#[test]
fn test_analyze_change_type_site_config() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = source_dir.join("site.toml");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();
    // Create the file so canonicalization works
    std::fs::write(&changed_path, "").unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    assert!(matches!(result, ChangeType::SiteConfig));
}

#[test]
fn test_analyze_change_type_markdown() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = source_dir.join("posts/hello.md");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::create_dir_all(changed_path.parent().unwrap()).unwrap();
    // Create the file so canonicalization works
    std::fs::write(&changed_path, "").unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    match result {
        ChangeType::Markdown { relative_path } => {
            assert_eq!(relative_path, "posts/hello.md");
        }
        _ => panic!("Expected Markdown change type"),
    }
}

#[test]
fn test_analyze_change_type_asset() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = source_dir.join("images/photo.jpg");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::create_dir_all(changed_path.parent().unwrap()).unwrap();
    // Create the file so canonicalization works
    std::fs::write(&changed_path, "").unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    assert!(matches!(result, ChangeType::Asset));
}

#[test]
fn test_analyze_change_type_unrelated() {
    let temp_dir = TempDir::new().unwrap();
    let theme_path = temp_dir.path().join("themes/custom");
    let source_dir = temp_dir.path().join("content");
    let changed_path = temp_dir.path().join("other/file.txt");

    std::fs::create_dir_all(&theme_path).unwrap();
    std::fs::create_dir_all(&source_dir).unwrap();

    let result = analyze_change_type(&changed_path, &theme_path, &source_dir).unwrap();
    assert!(matches!(result, ChangeType::Unrelated));
}

#[test]
fn test_is_html_template_true() {
    let path = Path::new("templates/page.html");
    assert!(is_html_template(path));
}

#[test]
fn test_is_html_template_false_wrong_extension() {
    let path = Path::new("templates/page.md");
    assert!(!is_html_template(path));
}

#[test]
fn test_is_html_template_false_no_templates_dir() {
    let path = Path::new("pages/page.html");
    assert!(!is_html_template(path));
}

#[test]
fn test_is_html_template_case_insensitive() {
    let path = Path::new("templates/page.HTML");
    assert!(is_html_template(path));
}

#[test]
fn test_create_asset_error() {
    let source_dir = Path::new("/source");
    let output_dir = Path::new("/output");
    let error = Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "test error",
    ));

    let result = create_asset_error("test context", source_dir, output_dir, error);

    match result {
        krik::error::KrikError::Generation(gen_err) => {
            assert_eq!(gen_err.context, "test context");
            match gen_err.kind {
                krik::error::GenerationErrorKind::AssetCopyError { source, target, .. } => {
                    assert_eq!(source, source_dir);
                    assert_eq!(target, output_dir);
                }
                _ => panic!("Expected AssetCopyError"),
            }
        }
        _ => panic!("Expected Generation error"),
    }
}
