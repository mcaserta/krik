use crate::theme::Theme;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copy non-markdown files from source to output directory
pub fn copy_non_markdown_files(source_dir: &Path, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(source_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Skip directories and markdown files
        if path.is_dir() || path.extension().map_or(false, |ext| ext == "md") {
            continue;
        }

        // Skip site.toml (site configuration file)
        if path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
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

