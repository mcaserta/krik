use pathdiff::diff_paths;
use std::path::{Path, PathBuf};

pub fn calculate_relative_path(file_path: &str, target: &str) -> String {
    let current_path = PathBuf::from(file_path);
    let target_path = PathBuf::from(target.trim_start_matches('/'));

    let current_dir = current_path.parent().unwrap_or_else(|| Path::new(""));
    if let Some(relative_path) = diff_paths(&target_path, current_dir) {
        relative_path.to_string_lossy().replace('\\', "/")
    } else {
        target.trim_start_matches('/').to_string()
    }
}

pub fn determine_output_path(document_file_path: &str, output_dir: &Path) -> PathBuf {
    let mut path = PathBuf::from(document_file_path);
    path.set_extension("html");
    output_dir.join(path)
}

pub fn get_base_path(path: &Path) -> String {
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let parent = path
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_default();

    let base_stem = if let Some(dot_pos) = stem.rfind('.') {
        let (base, lang) = stem.split_at(dot_pos);
        if lang.len() == 3 && lang.chars().nth(1).unwrap_or('.') != '.' {
            base
        } else {
            &stem
        }
    } else {
        &stem
    };

    if parent.is_empty() {
        base_stem.to_string()
    } else {
        format!("{parent}/{base_stem}")
    }
}
