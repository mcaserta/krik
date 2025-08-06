use crate::error::{KrikResult, MarkdownError, MarkdownErrorKind, IoError, IoErrorKind};
use std::path::Path;

/// Error recovery utilities for graceful degradation
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Attempt to recover from markdown parsing errors by providing fallback content
    pub fn recover_markdown_error(
        error: &MarkdownError,
        file_path: &Path,
        content: &str,
    ) -> Option<(crate::parser::FrontMatter, String)> {
        match &error.kind {
            MarkdownErrorKind::InvalidFrontMatter(_) => {
                // If front matter is invalid, try to extract just the content without front matter
                eprintln!("Warning: Invalid front matter in {}, using content without metadata", 
                          file_path.display());
                
                // Skip the front matter section and use remaining content
                if let Some(stripped) = content.strip_prefix("---\n") {
                    if let Some(end_pos) = stripped.find("\n---\n") {
                        let markdown_content = &stripped[end_pos + 5..];
                        return Some((Self::create_default_frontmatter(), markdown_content.to_string()));
                    }
                }
                
                // If we can't find the end of front matter, use entire content
                Some((Self::create_default_frontmatter(), content.to_string()))
            }
            MarkdownErrorKind::InvalidDate(_) => {
                // For invalid dates, we can still process the content with a warning
                eprintln!("Warning: Invalid date in {}, using current date", file_path.display());
                Some((Self::create_default_frontmatter(), content.to_string()))
            }
            _ => None, // Can't recover from other errors
        }
    }

    /// Attempt to recover from I/O errors by suggesting alternatives
    pub fn recover_io_error(error: &IoError) -> Option<String> {
        match &error.kind {
            IoErrorKind::NotFound => {
                Some(format!(
                    "File not found: {}\n  Suggestions:\n  - Check if the path is correct\n  - Ensure the file exists\n  - Check file permissions",
                    error.path.display()
                ))
            }
            IoErrorKind::PermissionDenied => {
                Some(format!(
                    "Permission denied: {}\n  Suggestions:\n  - Check file/directory permissions\n  - Run with appropriate user privileges\n  - Check if file is locked by another process",
                    error.path.display()
                ))
            }
            IoErrorKind::InvalidPath => {
                Some(format!(
                    "Invalid path: {}\n  Suggestions:\n  - Check for invalid characters in path\n  - Ensure path length is within limits\n  - Use forward slashes (/) for path separators",
                    error.path.display()
                ))
            }
            _ => None,
        }
    }

    /// Create a default front matter when recovery is needed
    fn create_default_frontmatter() -> crate::parser::FrontMatter {
        use chrono::Utc;
        use std::collections::HashMap;
        
        crate::parser::FrontMatter {
            title: Some("Untitled".to_string()),
            date: Some(Utc::now()),
            tags: None,
            lang: Some("en".to_string()),
            draft: Some(false),
            extra: HashMap::new(),
        }
    }

    /// Try to continue processing even when some files fail
    pub fn with_error_tolerance<F, T>(
        operation: F,
        description: &str,
        continue_on_error: bool,
    ) -> KrikResult<Vec<T>>
    where
        F: Fn() -> KrikResult<Vec<T>>,
    {
        match operation() {
            Ok(results) => Ok(results),
            Err(e) if continue_on_error => {
                eprintln!("Warning: {description} failed with error: {e}");
                eprintln!("Continuing with partial results...");
                Ok(Vec::new()) // Return empty results but don't fail
            }
            Err(e) => Err(e),
        }
    }

    /// Validate and suggest fixes for common configuration issues
    pub fn validate_project_structure(content_dir: &Path) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check if content directory exists
        if !content_dir.exists() {
            suggestions.push(format!(
                "Content directory '{}' does not exist. Create it with: mkdir -p {}",
                content_dir.display(),
                content_dir.display()
            ));
            return suggestions;
        }

        // Check for common structure issues
        let posts_dir = content_dir.join("posts");
        let pages_dir = content_dir.join("pages");
        let site_config = content_dir.join("site.toml");

        if !posts_dir.exists() {
            suggestions.push(format!(
                "Consider creating a 'posts' directory: mkdir -p {}",
                posts_dir.display()
            ));
        }

        if !pages_dir.exists() {
            suggestions.push(format!(
                "Consider creating a 'pages' directory: mkdir -p {}",
                pages_dir.display()
            ));
        }

        if !site_config.exists() {
            suggestions.push(format!(
                "Consider creating a site configuration: echo 'title = \"My Site\"' > {}",
                site_config.display()
            ));
        }

        // Check for markdown files
        let has_markdown = walkdir::WalkDir::new(content_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .any(|entry| {
                entry.path().is_file() &&
                entry.path().extension().is_some_and(|ext| ext == "md")
            });

        if !has_markdown {
            suggestions.push(
                "No markdown files found. Create your first post with: kk post \"My First Post\"".to_string()
            );
        }

        suggestions
    }

    /// Auto-fix common issues when possible
    pub fn auto_fix_issues(content_dir: &Path) -> KrikResult<Vec<String>> {
        let mut fixes_applied = Vec::new();
        
        // Create basic directory structure if missing
        let dirs_to_create = ["posts", "pages"];
        
        for dir_name in &dirs_to_create {
            let dir_path = content_dir.join(dir_name);
            if !dir_path.exists() {
                std::fs::create_dir_all(&dir_path)
                    .map_err(|e| crate::io_error!(
                        IoErrorKind::WriteFailed(e),
                        &dir_path,
                        "Creating directory structure"
                    ))?;
                fixes_applied.push(format!("Created directory: {}", dir_path.display()));
            }
        }

        // Create basic site.toml if missing
        let site_config = content_dir.join("site.toml");
        if !site_config.exists() {
            let default_config = r#"title = "My Krik Site"
# base_url = "https://example.com"  # Uncomment and update for production
"#;
            std::fs::write(&site_config, default_config)
                .map_err(|e| crate::io_error!(
                    IoErrorKind::WriteFailed(e),
                    &site_config,
                    "Creating default site configuration"
                ))?;
            fixes_applied.push(format!("Created default site.toml: {}", site_config.display()));
        }

        Ok(fixes_applied)
    }
}

/// Trait for adding recovery methods to Result types
pub trait ErrorRecoverable<T> {
    /// Continue processing even if this operation fails
    fn continue_on_error(self, description: &str) -> KrikResult<Option<T>>;
    
    /// Provide a default value if this operation fails
    fn or_default_with_warning(self, default: T, description: &str) -> KrikResult<T>;
}

impl<T> ErrorRecoverable<T> for KrikResult<T> {
    fn continue_on_error(self, description: &str) -> KrikResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(e) => {
                eprintln!("Warning: {} failed: {}", description, e);
                eprintln!("Continuing...");
                Ok(None)
            }
        }
    }
    
    fn or_default_with_warning(self, default: T, description: &str) -> KrikResult<T> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => {
                eprintln!("Warning: {} failed: {}", description, e);
                eprintln!("Using default value");
                Ok(default)
            }
        }
    }
}