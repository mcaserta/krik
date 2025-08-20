use krik::generator::sitemap::generate_sitemap;
use krik::site::SiteConfig;
use std::fs;
use std::path::Path;

#[test]
fn sitemap_escapes_home_url() {
    let docs: Vec<krik::parser::Document> = vec![];
    let mut cfg = SiteConfig::default();
    cfg.base_url = Some("https://example.com/page?a=1&b=2".into());
    let out = std::env::temp_dir().join(format!("krik_test_site_{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    generate_sitemap(&docs, &cfg, Path::new(&out)).unwrap();
    let xml = fs::read_to_string(out.join("sitemap.xml")).unwrap();
    assert!(xml.contains("<loc>https://example.com/page?a=1&amp;b=2</loc>"));
}
