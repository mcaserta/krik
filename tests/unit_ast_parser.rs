use krik::generator::ast_parser::{parse_markdown_ast, generate_toc_from_headings, Heading};
use pulldown_cmark::HeadingLevel;

#[test]
fn parse_markdown_ast_extracts_headings_and_ids() {
    let md = "# Title\n\n## Section 1\n\nText";
    let result = parse_markdown_ast(md);
    assert_eq!(result.headings.len(), 2);
    assert_eq!(result.headings[0].text, "Title");
    assert!(result.html_content.contains("id=\"title\""));
}

#[test]
fn heading_id_uniqueness() {
    let md = "# My Heading\n\n# My Heading";
    let result = parse_markdown_ast(md);
    assert_eq!(result.headings[0].id, "my-heading");
    assert_eq!(result.headings[1].id, "my-heading-1");
}

#[test]
fn toc_generation_skips_title() {
    let headings = vec![
        Heading { level: HeadingLevel::H1, text: "Title".into(), id: "title".into(), line_number: 1 },
        Heading { level: HeadingLevel::H2, text: "Section".into(), id: "section".into(), line_number: 2 },
    ];
    let toc = generate_toc_from_headings(&headings, Some("Title"));
    assert!(toc.contains("Section"));
    assert!(!toc.contains("Title"));
}

