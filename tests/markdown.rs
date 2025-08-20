use krik::generator::markdown::{generate_toc_and_content, markdown_to_html};

#[test]
fn markdown_to_html_basic() {
    let (html, _toc) = markdown_to_html("# Hello\n\nThis is **bold**.", false, None);
    assert!(html.contains("<h1"));
    assert!(html.contains("<strong>"));
}

#[test]
fn toc_generation_excludes_title() {
    let content = "<h1>Title</h1>\n<h2>Section 1</h2>\n<h3>Subsection</h3>";
    let (toc, processed) = generate_toc_and_content(content, Some("Title"));
    assert!(toc.contains("Section 1"));
    assert!(toc.contains("Subsection"));
    assert!(!toc.contains("Title"));
    assert_eq!(processed, content);
}
