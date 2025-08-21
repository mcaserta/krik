use std::fs;
use std::path::Path;

use krik::error::KrikError;
use krik::generator::templates;
use krik::i18n::I18nManager;
use krik::parser::{Document, FrontMatter};
use krik::site::SiteConfig;
use krik::theme::{Theme, ThemeConfig};

fn make_doc(path: &str) -> Document {
    Document {
        front_matter: FrontMatter {
            title: Some("X".into()),
            date: None,
            tags: None,
            lang: None,
            draft: None,
            pdf: None,
            extra: Default::default(),
        },
        content: "<p>content</p>".into(),
        file_path: path.into(),
        language: "en".into(),
        base_name: "x".into(),
        toc: None,
    }
}

fn build_empty_theme() -> Theme {
    // Construct a Theme with an empty Tera so any render will fail
    let config = ThemeConfig {
        name: "test".into(),
        version: "0.0.0".into(),
        author: None,
        description: None,
        templates: Default::default(),
    };
    Theme {
        config,
        templates: tera::Tera::default(),
        theme_path: std::path::PathBuf::from("<test>"),
    }
}

#[test]
fn render_page_maps_template_error() {
    let theme = build_empty_theme();
    let site = SiteConfig::default();
    let out = std::env::temp_dir().join(format!("krik_test_out_{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();

    let doc = make_doc("posts/missing.md");
    let err = templates::generate_page(&doc, &[doc.clone()], &theme, &site, Path::new(&out))
        .expect_err("expected template render to fail");

    match err {
        KrikError::Template(t) => {
            // Ensure context contains the attempted template name or file path
            assert!(t.context.contains("Rendering page"));
        }
        other => panic!("expected KrikError::Template, got {:?}", other),
    }
}

#[test]
fn render_index_maps_template_error() {
    let theme = build_empty_theme();
    let i18n = I18nManager::new("en".to_string());
    let site = SiteConfig::default();
    let out = std::env::temp_dir().join(format!("krik_test_out_idx_{}", std::process::id()));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();

    let docs: Vec<Document> = vec![];
    let err = templates::generate_index(&docs, &theme, &site, &i18n, Path::new(&out))
        .expect_err("expected index render to fail");

    match err {
        KrikError::Template(t) => {
            assert!(t.template.contains("index"));
            assert!(t.context.contains("Rendering index page"));
        }
        other => panic!("expected KrikError::Template, got {:?}", other),
    }
}
