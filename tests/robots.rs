use krik::generator::robots::generate_robots;
use krik::site::SiteConfig;
use std::fs;
use std::path::Path;

#[test]
fn robots_includes_sitemap_and_defaults() {
    let mut cfg = SiteConfig::default();
    cfg.base_url = Some("https://example.com".into());
    let out = std::env::temp_dir().join(format!("krik_test_robots_{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    generate_robots(&cfg, Path::new(&out)).unwrap();
    let txt = fs::read_to_string(out.join("robots.txt")).unwrap();
    assert!(txt.contains("Sitemap: https://example.com/sitemap.xml"));
}

