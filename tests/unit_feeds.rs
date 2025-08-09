use krik::generator::feeds::generate_feed;
use krik::parser::{Document, FrontMatter};
use std::collections::HashMap;
use krik::site::SiteConfig;
use std::fs;
use std::path::Path;

#[test]
fn feed_generation_smoke() {
    let mut post_extra = HashMap::new();
    post_extra.insert("layout".to_string(), serde_yaml::Value::String("post".to_string()));
    let post = Document { file_path: "posts/test.md".into(), front_matter: FrontMatter { title: None, date: None, tags: None, lang: None, draft: None, pdf: None, extra: post_extra }, content: String::new(), language: "en".into(), base_name: "test".into(), toc: None };
    let docs = vec![post];
    let mut cfg = SiteConfig::default();
    cfg.base_url = Some("https://example.com".into());
    let out = std::env::temp_dir().join(format!("krik_test_feed_{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    generate_feed(&docs, &cfg, Path::new(&out)).unwrap();
    assert!(out.join("feed.xml").exists());
}

