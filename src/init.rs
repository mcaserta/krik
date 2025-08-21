use crate::error::{
    GenerationError, GenerationErrorKind, IoError, IoErrorKind, KrikError, KrikResult,
};
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

// Embed the content and themes directories at compile time
static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");
static THEMES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/themes");

pub fn init_site(target_dir: &Path, force: bool) -> KrikResult<()> {
    info!("🚀 Initializing new Krik site in: {}", target_dir.display());

    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).map_err(|e| {
            KrikError::Io(Box::new(IoError {
                kind: IoErrorKind::WriteFailed(e),
                path: target_dir.to_path_buf(),
                context: "Creating target directory for site initialization".to_string(),
            }))
        })?;
        info!("📁 Created directory: {}", target_dir.display());
    }

    // Check if directory is empty (unless force is specified)
    if !force && is_directory_not_empty(target_dir)? {
        return Err(KrikError::Generation(Box::new(GenerationError {
            kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Directory is not empty",
            )),
            context: format!(
                "Directory '{}' is not empty. Use --force to overwrite existing files.",
                target_dir.display()
            ),
        })));
    }

    // Extract content directory
    let content_target = target_dir.join("content");
    extract_embedded_dir(&CONTENT_DIR, &content_target, force).map_err(|e| {
        KrikError::Generation(Box::new(GenerationError {
            kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to extract content directory: {e}"),
            )),
            context: "Extracting embedded content directory".to_string(),
        }))
    })?;
    info!("📝 Created content directory with sample posts and pages");

    // Extract themes directory
    let themes_target = target_dir.join("themes");
    extract_embedded_dir(&THEMES_DIR, &themes_target, force).map_err(|e| {
        KrikError::Generation(Box::new(GenerationError {
            kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to extract themes directory: {e}"),
            )),
            context: "Extracting embedded themes directory".to_string(),
        }))
    })?;
    info!("🎨 Created themes directory with default theme");

    info!("\n✅ Site initialized successfully!");
    info!("\n🔧 Next steps:");
    info!("   cd {}", target_dir.display());
    info!("   kk server          # Start development server");
    info!("   kk                 # Generate static site");

    Ok(())
}

fn is_directory_not_empty(dir: &Path) -> KrikResult<bool> {
    if !dir.exists() {
        return Ok(false);
    }

    let entries = fs::read_dir(dir).map_err(|e| {
        KrikError::Io(Box::new(IoError {
            kind: IoErrorKind::ReadFailed(e),
            path: dir.to_path_buf(),
            context: "Checking if directory is empty".to_string(),
        }))
    })?;
    Ok(entries.count() > 0)
}

fn extract_embedded_dir(embedded_dir: &Dir, target_path: &Path, force: bool) -> KrikResult<()> {
    // Create target directory
    fs::create_dir_all(target_path).map_err(|e| {
        KrikError::Io(Box::new(IoError {
            kind: IoErrorKind::WriteFailed(e),
            path: target_path.to_path_buf(),
            context: "Creating directory for embedded file extraction".to_string(),
        }))
    })?;

    // Extract all files in this directory level
    for file in embedded_dir.files() {
        let file_name = match file.path().file_name() {
            Some(n) => n,
            None => continue,
        };
        let file_path = target_path.join(file_name);

        // Check if file exists and force is not specified
        if file_path.exists() && !force {
            warn!("⚠️  Skipping existing file: {}", file_path.display());
            continue;
        }

        // Write file contents
        fs::write(&file_path, file.contents()).map_err(|e| {
            KrikError::Io(Box::new(IoError {
                kind: IoErrorKind::WriteFailed(e),
                path: file_path.clone(),
                context: "Writing embedded file contents".to_string(),
            }))
        })?;
        info!(
            "📄 Created: {}",
            file_path
                .strip_prefix(target_path.parent().unwrap_or(target_path))
                .unwrap_or(&file_path)
                .display()
        );
    }

    // Recursively extract subdirectories
    for subdir in embedded_dir.dirs() {
        let Some(subdir_name) = subdir.path().file_name() else {
            continue;
        };
        let subdir_path = target_path.join(subdir_name);
        extract_embedded_dir(subdir, &subdir_path, force)?;
    }

    Ok(())
}
