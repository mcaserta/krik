use krik::generator::pdf::PdfGenerator;
use std::path::{Path, PathBuf};

#[test]
fn pdf_availability_check_does_not_panic() {
    // This test does not require pandoc/typst to be present; it only exercises the availability check.
    let _ = PdfGenerator::is_available();
}

#[test]
fn test_path_normalization() {
    let generator = PdfGenerator::new().unwrap();

    // Test basic parent directory resolution
    let path = Path::new("posts/../images/logo.png");
    let normalized = generator.normalize_path(path);
    assert_eq!(normalized, PathBuf::from("images/logo.png"));

    // Test multiple parent directories
    let path = Path::new("posts/deep/../../images/logo.png");
    let normalized = generator.normalize_path(path);
    assert_eq!(normalized, PathBuf::from("images/logo.png"));

    // Test current directory references
    let path = Path::new("posts/./images/logo.png");
    let normalized = generator.normalize_path(path);
    assert_eq!(normalized, PathBuf::from("posts/images/logo.png"));
}

#[test]
fn test_relative_path_resolution() {
    let generator = PdfGenerator::new().unwrap();

    let source_root = Path::new("/project");

    // Test path resolution from posts directory (inside source root)
    let input_dir = Path::new("/project/content/posts");
    let resolved =
        generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "content/images/logo.png");

    // Test path resolution from pages directory (inside source root)
    let input_dir = Path::new("/project/content/pages");
    let resolved =
        generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "content/images/logo.png");

    // Test path resolution with deeper nesting (inside source root)
    let input_dir = Path::new("/project/content/posts/year/month");
    let resolved =
        generator.resolve_relative_path("../../../../images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "images/logo.png");

    // Test path resolution with input directory outside source root
    let input_dir = Path::new("/other/project/content/posts");
    let resolved =
        generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "/other/project/content/images/logo.png");

    // Test path resolution with input directory as parent of source root
    let input_dir = Path::new("/other/content");
    let resolved =
        generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "/other/images/logo.png");

    // Test path resolution with complex relative paths (inside source root)
    let input_dir = Path::new("/project/content/posts");
    let resolved =
        generator.resolve_relative_path("../../other/images/logo.png", input_dir, source_root);
    assert_eq!(resolved, "other/images/logo.png");
}

#[test]
fn test_pdf_url_generation() {
    let generator = PdfGenerator::new().unwrap();

    // Test absolute URL generation
    let output_path = Path::new("/project/_site/posts/document.pdf");
    let base_url = "https://example.com";
    let absolute_url = generator.generate_absolute_pdf_url(output_path, base_url);
    assert_eq!(absolute_url, "https://example.com/posts/document.pdf");

    // Test absolute URL generation with trailing slash
    let output_path = Path::new("/project/_site/pages/about.pdf");
    let base_url = "https://example.com/";
    let absolute_url = generator.generate_absolute_pdf_url(output_path, base_url);
    assert_eq!(absolute_url, "https://example.com/pages/about.pdf");
}

