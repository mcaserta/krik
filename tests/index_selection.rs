use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use krik::generator::SiteGenerator;

fn write_file(path: &PathBuf, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    let mut f = File::create(path).unwrap();
    f.write_all(contents.as_bytes()).unwrap();
}

#[test]
fn index_selection_prefers_default_language_when_both_exist(
) -> Result<(), Box<dyn std::error::Error>> {
    // Prepare temporary workspace
    let mut tmp_dir: PathBuf = std::env::temp_dir();
    tmp_dir.push(format!("krik_test_index_selection_{}", std::process::id()));
    // Start with a clean directory if it exists from a previous run
    let _ = fs::remove_dir_all(&tmp_dir);
    fs::create_dir_all(&tmp_dir)?;

    let content_dir = tmp_dir.join("content");
    let output_dir = tmp_dir.join("_site");

    // Create two variants for the same post base name: default 'en' and 'it'
    let post_en = content_dir.join("posts/foo.md");
    let post_it = content_dir.join("posts/foo.it.md");

    // Minimal markdown with front matter titles to distinguish variants
    write_file(
        &post_en,
        r#"---
title: Foo EN
---

# English Post

Content.
"#,
    );

    write_file(
        &post_it,
        r#"---
title: Foo IT
---

# Post Italiano

Contenuto.
"#,
    );

    // Generate site
    let generator = SiteGenerator::new(&content_dir, &output_dir, None::<&PathBuf>)?;
    generator.generate_site()?;

    // Read generated index and assert the default-language variant is chosen
    let index_path = output_dir.join("index.html");
    let index_html = fs::read_to_string(index_path)?;

    // Should link to posts/foo.html (default language) and not posts/foo.it.html
    assert!(
        index_html.contains("href=\"posts/foo.html\""),
        "index should link to default-language post"
    );
    assert!(
        !index_html.contains("href=\"posts/foo.it.html\""),
        "index should not link to non-default variant when default exists"
    );

    Ok(())
}
