use crate::theme::Theme;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Return true if the asset should be ignored (not copied)
fn is_ignored_asset(path: &Path) -> bool {
    if let Some(file_name_os) = path.file_name() {
        if let Some(file_name) = file_name_os.to_str() {
            // Ignore dotfiles like .DS_Store and hidden files
            if file_name.starts_with('.') {
                return true;
            }

            let lower = file_name.to_lowercase();
            // Common OS/editor temp files
            if lower == "thumbs.db" {
                return true;
            }
            if lower.ends_with('~') {
                return true;
            }
            if lower.starts_with('#') && lower.ends_with('#') {
                return true;
            }

            // Temporary/backup extensions
            if let Some(ext) = path.extension().and_then(|s| s.to_str()).map(|s| s.to_lowercase()) {
                match ext.as_str() {
                    "swp" | "swo" | "swx" | "tmp" | "bak" | "orig" | "part" | "crdownload" => return true,
                    _ => {}
                }
            }
        }
    }
    false
}

/// Copy non-markdown files from source to output directory
pub fn copy_non_markdown_files(source_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories and markdown files
        if path.is_dir() || path.extension().is_some_and(|ext| ext == "md") {
            continue;
        }

        // Skip site.toml (site configuration file)
        if path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
            continue;
        }

        // Skip ignored assets (dotfiles, editor temp files, backups)
        if is_ignored_asset(path) {
            continue;
        }

        // Calculate relative path and destination
        let relative_path = path.strip_prefix(source_dir)
            .map_err(|_| format!("Failed to get relative path for: {}", path.display()))?;
        let dest_path = output_dir.join(relative_path);

        // Create parent directories if they don't exist
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Copy the file
        fs::copy(path, &dest_path)?;
    }

    Ok(())
}

/// Copy theme assets to the output directory
pub fn copy_theme_assets(theme: &Theme, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let asset_dir = theme.theme_path.join("assets");
    if asset_dir.exists() {
        let dest_assets_dir = output_dir.join("assets");
        
        // Create assets directory if it doesn't exist
        if !dest_assets_dir.exists() {
            fs::create_dir_all(&dest_assets_dir)?;
        }

        // Copy all files from theme assets
        copy_directory_contents(&asset_dir, &dest_assets_dir)?;
    }

    Ok(())
}

/// Recursively copy directory contents
fn copy_directory_contents(src: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(src)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.is_file() {
            // Skip ignored assets (dotfiles, editor temp files, backups)
            if is_ignored_asset(path) {
                continue;
            }

            let relative_path = path.strip_prefix(src)
                .map_err(|_| format!("Failed to get relative path for: {}", path.display()))?;
            let dest_path = dest.join(relative_path);

            // Create parent directories if they don't exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Copy the file
            fs::copy(path, &dest_path)?;
        }
    }

    Ok(())
}

/// Copy a single asset file from `source_dir` into the mirrored path under `output_dir`.
/// Skips markdown files and ignored assets. Returns Ok even if the path is not a regular file.
pub fn copy_single_asset(
    source_dir: &Path,
    output_dir: &Path,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file_path.exists() || file_path.is_dir() {
        return Ok(());
    }
    // Skip markdown and site.toml
    if file_path.extension().is_some_and(|ext| ext == "md") {
        return Ok(());
    }
    if file_path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
        return Ok(());
    }
    if is_ignored_asset(file_path) {
        return Ok(());
    }

    let relative_path = file_path.strip_prefix(source_dir)
        .map_err(|_| format!("Failed to get relative path for: {}", file_path.display()))?;
    let dest_path = output_dir.join(relative_path);

    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(file_path, &dest_path)?;
    Ok(())
}

/// Remove a single asset file from the mirrored path under `output_dir`.
/// Safe to call even if the destination file does not exist.
pub fn remove_single_asset(
    source_dir: &Path,
    output_dir: &Path,
    file_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let relative_path = match file_path.strip_prefix(source_dir) {
        Ok(rel) => rel,
        Err(_) => return Ok(()),
    };
    let dest_path = output_dir.join(relative_path);
    if dest_path.exists() && dest_path.is_file() {
        let _ = fs::remove_file(dest_path);
    }
    Ok(())
}

