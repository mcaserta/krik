use crate::error::{KrikResult, KrikError, GenerationError, GenerationErrorKind, IoError, IoErrorKind};
use crate::parser::Document;
use crate::site::SiteConfig;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use chrono::Utc;
use which::which;
use tracing::{info, warn};

/// PDF generation using pandoc with typst engine
pub struct PdfGenerator {
    pandoc_path: PathBuf,
    typst_path: PathBuf,
}

impl PdfGenerator {
    /// Create a new PDF generator, checking for required tools
    pub fn new() -> KrikResult<Self> {
        let pandoc_path = Self::find_executable("pandoc")
            .ok_or_else(|| KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError("Pandoc not found in PATH. Install pandoc to enable PDF generation.".to_string()),
                context: "Initializing PDF generator".to_string(),
            }))?;
        
        let typst_path = Self::find_executable("typst")
            .ok_or_else(|| KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError("Typst not found in PATH. Install typst to enable PDF generation.".to_string()),
                context: "Initializing PDF generator".to_string(),
            }))?;

        Ok(Self {
            pandoc_path,
            typst_path,
        })
    }

    /// Check if PDF generation is available (both tools present)
    pub fn is_available() -> bool {
        Self::find_executable("pandoc").is_some() && Self::find_executable("typst").is_some()
    }

    /// Find executable in PATH using the `which` crate (cross-platform)
    fn find_executable(name: &str) -> Option<PathBuf> {
        which(name).ok()
    }

    /// Generate PDF from a markdown file path
    pub fn generate_pdf_from_file(&self, input_path: &Path, output_path: &Path, source_root: &Path, site_config: &SiteConfig, document_language: &str) -> KrikResult<()> {
        // Ensure the output directory exists
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| KrikError::Io(IoError {
                    kind: IoErrorKind::WriteFailed(e),
                    path: parent.to_path_buf(),
                    context: "Creating PDF output directory".to_string(),
                }))?;
        }

        // Create a temporary filtered markdown file
        let temp_md_file = self.create_filtered_markdown(input_path, output_path, source_root, site_config, document_language)?;

        // Run pandoc with typst engine on the temporary file
        let mut cmd = Command::new(&self.pandoc_path);
        cmd.arg(&temp_md_file)
            .arg("--pdf-engine=typst")
            .arg("--output")
            .arg(output_path)
            .arg("--standalone")
            .current_dir(source_root);
        
        // Execute pandoc
        let output = cmd.output()
            .map_err(|e| KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Failed to execute pandoc: {}", e)),
                context: "Running pandoc to generate PDF".to_string(),
            }))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Pandoc failed: {}", stderr)),
                context: "Converting markdown to PDF with pandoc".to_string(),
            }));
        }

        // Clean up temporary markdown file
        let _ = fs::remove_file(&temp_md_file);

        Ok(())
    }

    /// Create a filtered markdown file for PDF generation
    fn create_filtered_markdown(&self, input_path: &Path, output_path: &Path, source_root: &Path, site_config: &SiteConfig, document_language: &str) -> KrikResult<PathBuf> {
        // Read the original markdown content
        let content = fs::read_to_string(input_path)
            .map_err(|e| KrikError::Io(IoError {
                kind: IoErrorKind::ReadFailed(e),
                path: input_path.to_path_buf(),
                context: "Reading markdown file for PDF generation".to_string(),
            }))?;

        // Parse front matter and content
        let (front_matter, markdown_content) = self.parse_front_matter(&content)?;

        // Fix relative image paths by resolving them to absolute paths from source root
        let content_with_fixed_paths = self.resolve_relative_image_paths(&markdown_content, input_path, source_root)?;

        // Build the filtered markdown content
        let mut filtered_content = String::new();

        // Add title heading if it exists in front matter and not already in content
        if let Some(title) = &front_matter.title {
            if !content_with_fixed_paths.trim_start().starts_with("# ") {
                filtered_content.push_str(&format!("# {}\n\n", title));
            }
        }

        // Add the main content
        filtered_content.push_str(&content_with_fixed_paths);

        // Add appendix with download information (only if base_url is configured)
        if let Some(base_url) = site_config.get_base_url() {
            let absolute_pdf_url = self.generate_absolute_pdf_url(output_path, &base_url);
            
            filtered_content.push_str("\n\n---\n\n");
            
            // Document Information heading
            let doc_info_heading = self.translate_string("document_information", document_language);
            filtered_content.push_str(&format!("## {}\n\n", doc_info_heading));
            
            // Download URL line
            let download_text = self.translate_string("document_downloaded_from", document_language);
            filtered_content.push_str(&format!("{} {}\n\n", download_text, absolute_pdf_url));
            
            // Generation timestamp line
            let generated_text = self.translate_string("generated_at", document_language);
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            filtered_content.push_str(&format!("{} {}\n", generated_text, timestamp));
        }

        // Create temporary file
        let temp_file = std::env::temp_dir().join(format!(
            "krik_pdf_{}_{}.md",
            input_path.file_stem().unwrap().to_string_lossy(),
            std::process::id()
        ));

        // Write the filtered content to temporary file
        fs::write(&temp_file, filtered_content)
            .map_err(|e| KrikError::Io(IoError {
                kind: IoErrorKind::WriteFailed(e),
                path: temp_file.clone(),
                context: "Writing temporary filtered markdown file".to_string(),
            }))?;

        Ok(temp_file)
    }

    /// Parse front matter from markdown content
    fn parse_front_matter(&self, content: &str) -> KrikResult<(crate::parser::FrontMatter, String)> {
        crate::parser::parse_markdown_with_frontmatter(content)
    }

    /// Generate absolute PDF URL for the appendix
    fn generate_absolute_pdf_url(&self, output_path: &Path, base_url: &str) -> String {
        let relative_path = self.generate_relative_pdf_path(output_path);
        let base_url_trimmed = base_url.trim_end_matches('/');
        format!("{}{}", base_url_trimmed, relative_path)
    }

    /// Generate the relative PDF path
    fn generate_relative_pdf_path(&self, output_path: &Path) -> String {
        // Extract just the filename and create a relative URL
        if let Some(filename) = output_path.file_name() {
            // Get the directory path relative to _site
            if let Some(parent) = output_path.parent() {
                if let Some(parent_name) = parent.file_name() {
                    // Skip if parent is "_site" (use filename only)
                    if parent_name == "_site" {
                        return format!("/{}", filename.to_string_lossy());
                    }
                    return format!("/{}/{}", parent_name.to_string_lossy(), filename.to_string_lossy());
                }
            }
            // Fallback to just filename
            format!("/{}", filename.to_string_lossy())
        } else {
            // Fallback URL
            "/document.pdf".to_string()
        }
    }

    /// Translate strings based on document language
    fn translate_string(&self, key: &str, language: &str) -> String {
        match (key, language) {
            // Document Information
            ("document_information", "it") => "Informazioni sul Documento".to_string(),
            ("document_information", "es") => "Información del Documento".to_string(),
            ("document_information", "fr") => "Informations sur le Document".to_string(),
            ("document_information", "de") => "Dokumentinformationen".to_string(),
            ("document_information", "pt") => "Informações do Documento".to_string(),
            ("document_information", "ja") => "ドキュメント情報".to_string(),
            ("document_information", "zh") => "文档信息".to_string(),
            ("document_information", "ru") => "Информация о документе".to_string(),
            ("document_information", "ar") => "معلومات الوثيقة".to_string(),
            ("document_information", _) => "Document Information".to_string(),

            // Document downloaded from
            ("document_downloaded_from", "it") => "Questo documento è stato scaricato da".to_string(),
            ("document_downloaded_from", "es") => "Este documento fue descargado desde".to_string(),
            ("document_downloaded_from", "fr") => "Ce document a été téléchargé depuis".to_string(),
            ("document_downloaded_from", "de") => "Dieses Dokument wurde heruntergeladen von".to_string(),
            ("document_downloaded_from", "pt") => "Este documento foi baixado de".to_string(),
            ("document_downloaded_from", "ja") => "このドキュメントはダウンロードされました".to_string(),
            ("document_downloaded_from", "zh") => "此文档下载自".to_string(),
            ("document_downloaded_from", "ru") => "Этот документ был загружен с".to_string(),
            ("document_downloaded_from", "ar") => "تم تحميل هذه الوثيقة من".to_string(),
            ("document_downloaded_from", _) => "This document was downloaded from".to_string(),

            // Generated at
            ("generated_at", "it") => "Generato il".to_string(),
            ("generated_at", "es") => "Generado el".to_string(),
            ("generated_at", "fr") => "Généré le".to_string(),
            ("generated_at", "de") => "Erstellt am".to_string(),
            ("generated_at", "pt") => "Gerado em".to_string(),
            ("generated_at", "ja") => "生成日時".to_string(),
            ("generated_at", "zh") => "生成时间".to_string(),
            ("generated_at", "ru") => "Создано".to_string(),
            ("generated_at", "ar") => "تم الإنشاء في".to_string(),
            ("generated_at", _) => "Generated at".to_string(),

            _ => key.to_string(),
        }
    }

    /// Resolve relative image paths in markdown content
    fn resolve_relative_image_paths(&self, content: &str, input_path: &Path, source_root: &Path) -> KrikResult<String> {
        // Use regex to find all markdown image patterns: ![alt](path) and ![alt](path "title")
        use regex::Regex;
        
        let img_regex = Regex::new(r#"!\[([^]]*)]\(([^)]+?)(?:\s+["']([^"']*?)["'])?\)"#)
            .map_err(|e| KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Failed to compile image regex: {}", e)),
                context: "Processing markdown image paths".to_string(),
            }))?;

        let mut fixed_content = content.to_string();
        let input_dir = input_path.parent().unwrap_or(Path::new(""));

        // Find all matches and replace them
        let matches: Vec<_> = img_regex.find_iter(content).collect();
        
        // Process matches in reverse order to avoid offset issues when replacing
        for img_match in matches.iter().rev() {
            let full_match = img_match.as_str();
            
            // Parse the match to extract components
            if let Some(caps) = img_regex.captures(full_match) {
                let alt_text = caps.get(1).map_or("", |m| m.as_str());
                let original_path = caps.get(2).map_or("", |m| m.as_str());
                let title = caps.get(3).map(|m| m.as_str());

                // Only process relative paths (not URLs or absolute paths)
                if !original_path.starts_with("http") && !original_path.starts_with("/") && !original_path.is_empty() {
                    // Resolve the relative path from the markdown file's directory
                    let resolved_path = self.resolve_relative_path(original_path, input_dir, source_root);
                    
                    // Create the replacement markdown image syntax
                    let replacement = if let Some(title_text) = title {
                        format!("![{}]({} \"{}\")", alt_text, resolved_path, title_text)
                    } else {
                        format!("![{}]({})", alt_text, resolved_path)
                    };
                    
                    // Replace in the content
                    let start = img_match.start();
                    let end = img_match.end();
                    fixed_content.replace_range(start..end, &replacement);
                }
            }
        }

        Ok(fixed_content)
    }

    /// Resolve a relative path from a markdown file to an absolute path from source root
    fn resolve_relative_path(&self, relative_path: &str, input_dir: &Path, source_root: &Path) -> String {
        // First canonicalize both paths to handle any .. or . components
        let canonical_input_dir = fs::canonicalize(input_dir)
            .unwrap_or_else(|_| input_dir.to_path_buf());
        let canonical_source_root = fs::canonicalize(source_root)
            .unwrap_or_else(|_| source_root.to_path_buf());

        // Try to make input_dir relative to source_root
        let input_relative_to_source = match canonical_input_dir.strip_prefix(&canonical_source_root) {
            Ok(relative) => relative.to_path_buf(),
            Err(_) => {
                // If outside source root, normalize the path but keep it absolute
                let joined = canonical_input_dir.join(relative_path);
                return self.normalize_path(&joined)
                    .to_string_lossy()
                    .replace('\\', "/");
            }
        };

        // For paths within source root, resolve relative to source root
        let resolved_path = input_relative_to_source.join(relative_path);
        let normalized = self.normalize_path(&resolved_path);
        
        // Convert back to string with forward slashes for consistency
        normalized.to_string_lossy().replace('\\', "/")
    }


    /// Normalize a path by resolving . and .. components
    fn normalize_path(&self, path: &Path) -> PathBuf {
        let mut result = PathBuf::new();
        
        for component in path.components() {
            match component {
                std::path::Component::Normal(name) => result.push(name),
                std::path::Component::ParentDir => {
                    result.pop();
                }
                std::path::Component::CurDir => {
                    // Skip current directory references
                }
                _ => result.push(component),
            }
        }
        
        result
    }

    /// Generate PDFs for documents that have pdf: true in their front matter
    pub fn generate_pdfs(&self, documents: &[Document], source_dir: &Path, output_dir: &Path, site_config: &SiteConfig) -> KrikResult<Vec<PathBuf>> {
        let mut generated_pdfs = Vec::new();

        // Filter documents that have pdf: true
        let pdf_documents: Vec<&Document> = documents
            .iter()
            .filter(|doc| doc.front_matter.pdf.unwrap_or(false))
            .collect();

        if pdf_documents.is_empty() {
            info!("No documents marked for PDF generation (pdf: true)");
            return Ok(generated_pdfs);
        }

        info!("Generating PDFs for {} documents marked with pdf: true", pdf_documents.len());

        // Determine project root by canonicalizing the source directory path
        let canonical_source_dir = fs::canonicalize(source_dir)
            .map_err(|e| KrikError::Io(IoError {
                kind: IoErrorKind::ReadFailed(e),
                path: source_dir.to_path_buf(),
                context: "Canonicalizing source directory path".to_string(),
            }))?;
            
        let project_root = canonical_source_dir.parent()
            .unwrap_or(&canonical_source_dir)
            .to_path_buf();

        for document in pdf_documents {
            // Construct input path (source file) and output path (PDF file)
            let input_path = source_dir.join(&document.file_path);
            let output_path = self.determine_pdf_output_path(document, output_dir);

            match self.generate_pdf_from_file(&input_path, &output_path, &project_root, site_config, &document.language) {
                Ok(()) => {
                    info!("Generated PDF: {}", output_path.display());
                    generated_pdfs.push(output_path);
                }
                Err(e) => {
                    warn!("Warning: Failed to generate PDF for {}: {}", 
                             document.file_path, e);
                }
            }
        }

        Ok(generated_pdfs)
    }

    /// Determine the output path for a PDF file (same directory as HTML)
    fn determine_pdf_output_path(&self, document: &Document, output_dir: &Path) -> PathBuf {
        let mut path = PathBuf::from(&document.file_path);
        path.set_extension("pdf");
        output_dir.join(path)
    }


    /// Get version information for diagnostics
    pub fn version_info(&self) -> KrikResult<(String, String)> {
        let pandoc_version = self.get_tool_version(&self.pandoc_path, &["--version"])?;
        let typst_version = self.get_tool_version(&self.typst_path, &["--version"])?;
        
        Ok((pandoc_version, typst_version))
    }

    fn get_tool_version(&self, tool_path: &Path, args: &[&str]) -> KrikResult<String> {
        let output = Command::new(tool_path)
            .args(args)
            .output()
            .map_err(|e| KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Failed to get version for {}: {}", tool_path.display(), e)),
                context: "Getting tool version information".to_string(),
            }))?;

        if output.status.success() {
            let version_output = String::from_utf8_lossy(&output.stdout);
            let first_line = version_output.lines().next().unwrap_or("Unknown").trim();
            Ok(first_line.to_string())
        } else {
            Err(KrikError::Generation(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Failed to get version for {}", tool_path.display())),
                context: "Getting tool version information".to_string(),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_normalization() {
        let generator = PdfGenerator {
            pandoc_path: PathBuf::from("pandoc"),
            typst_path: PathBuf::from("typst"),
        };

        // Test basic parent directory resolution
        let path = Path::new("posts/../images/logo.png");
        let normalized = generator.normalize_path(path);
        assert_eq!(normalized, PathBuf::from("images/logo.png"));

        // Test multiple parent directories
        let path = Path::new("posts/deep/../../images/logo.png");
        let normalized = generator.normalize_path(path);
        assert_eq!(normalized, PathBuf::from("images/logo.png"));

        // Test current directory references
        let path = Path::new("posts/./images/logo.png");
        let normalized = generator.normalize_path(path);
        assert_eq!(normalized, PathBuf::from("posts/images/logo.png"));
    }

    #[test]
    fn test_relative_path_resolution() {
        let generator = PdfGenerator {
            pandoc_path: PathBuf::from("pandoc"),
            typst_path: PathBuf::from("typst"),
        };

        let source_root = Path::new("/project");
        
        // Test path resolution from posts directory (inside source root)
        let input_dir = Path::new("/project/content/posts");
        let resolved = generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "content/images/logo.png");

        // Test path resolution from pages directory (inside source root)
        let input_dir = Path::new("/project/content/pages");
        let resolved = generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "content/images/logo.png");

        // Test path resolution with deeper nesting (inside source root)
        let input_dir = Path::new("/project/content/posts/year/month");
        let resolved = generator.resolve_relative_path("../../../../images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "images/logo.png");

        // Test path resolution with input directory outside source root
        let input_dir = Path::new("/other/project/content/posts");
        let resolved = generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "/other/project/content/images/logo.png");

        // Test path resolution with input directory as parent of source root
        let input_dir = Path::new("/other/content");
        let resolved = generator.resolve_relative_path("../images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "/other/images/logo.png");

        // Test path resolution with complex relative paths (inside source root)
        let input_dir = Path::new("/project/content/posts");
        let resolved = generator.resolve_relative_path("../../other/images/logo.png", input_dir, source_root);
        assert_eq!(resolved, "other/images/logo.png");
    }

    #[test]
    fn test_pdf_url_generation() {
        let generator = PdfGenerator {
            pandoc_path: PathBuf::from("pandoc"),
            typst_path: PathBuf::from("typst"),
        };

        // Test absolute URL generation
        let output_path = Path::new("/project/_site/posts/document.pdf");
        let base_url = "https://example.com";
        let absolute_url = generator.generate_absolute_pdf_url(output_path, base_url);
        assert_eq!(absolute_url, "https://example.com/posts/document.pdf");

        // Test absolute URL generation with trailing slash
        let output_path = Path::new("/project/_site/pages/about.pdf");
        let base_url = "https://example.com/";
        let absolute_url = generator.generate_absolute_pdf_url(output_path, base_url);
        assert_eq!(absolute_url, "https://example.com/pages/about.pdf");
    }

    #[test]
    fn test_translation_system() {
        let generator = PdfGenerator {
            pandoc_path: PathBuf::from("pandoc"),
            typst_path: PathBuf::from("typst"),
        };

        // Test English (default)
        assert_eq!(generator.translate_string("document_information", "en"), "Document Information");
        assert_eq!(generator.translate_string("document_downloaded_from", "en"), "This document was downloaded from");
        assert_eq!(generator.translate_string("generated_at", "en"), "Generated at");

        // Test Italian
        assert_eq!(generator.translate_string("document_information", "it"), "Informazioni sul Documento");
        assert_eq!(generator.translate_string("document_downloaded_from", "it"), "Questo documento è stato scaricato da");
        assert_eq!(generator.translate_string("generated_at", "it"), "Generato il");

        // Test Spanish
        assert_eq!(generator.translate_string("document_information", "es"), "Información del Documento");
        assert_eq!(generator.translate_string("generated_at", "es"), "Generado el");

        // Test unknown language defaults to English
        assert_eq!(generator.translate_string("document_information", "unknown"), "Document Information");
    }
}
