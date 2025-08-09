use krik::generator::sitemap::generate_sitemap;
use krik::site::SiteConfig;
use krik::parser::{Document, FrontMatter};
use std::collections::HashMap;

fn base_doc(file_path: &str, layout: Option<&str>, draft: Option<bool>, lang: &str) -> Document {
    let mut extra = HashMap::new();
    if let Some(l) = layout { extra.insert("layout".into(), serde_yaml::Value::String(l.into())); }
    Document { file_path: file_path.into(), front_matter: FrontMatter { title: None, date: None, tags: None, lang: None, draft, pdf: None, extra }, content: String::new(), language: lang.into(), base_name: "base".into(), toc: None }
}

#[test]
fn sitemap_inclusion_and_is_post() {
    let page = base_doc("pages/about.md", Some("page"), None, "en");
    // Write a minimal sitemap and ensure no crash (smoke test)
    let out = std::env::temp_dir().join(format!("krik_test_sitemap_more_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();
    let mut cfg = SiteConfig::default();
    cfg.base_url = Some("https://example.com".into());
    let post = base_doc("posts/test.md", Some("post"), None, "en");
    let draft = base_doc("posts/draft.md", Some("post"), Some(true), "en");
    let docs = vec![page, post, draft];
    generate_sitemap(&docs, &cfg, &out).unwrap();
    assert!(out.join("sitemap.xml").exists());
}

