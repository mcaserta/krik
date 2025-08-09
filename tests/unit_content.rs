use krik::content::{create_page, create_post};
use std::fs;
use std::path::Path;

#[test]
fn create_page_and_post_smoke() {
    let tmp = std::env::temp_dir().join(format!("krik_test_content_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();

    create_post(Path::new(&tmp), "Hello World", None).unwrap();
    create_page(Path::new(&tmp), "About", None).unwrap();

    assert!(tmp.join("posts/hello-world.md").exists());
    assert!(tmp.join("pages/about.md").exists());
}

