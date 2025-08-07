use crate::parser::Document;
use crate::theme::Theme;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use crate::error::{KrikResult, KrikError, ThemeError, ThemeErrorKind};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{info, debug, warn, error};

/// The main site generator that processes Markdown files and creates a static website.
///
/// `SiteGenerator` handles the entire process of creating a static site:
/// - Scanning and parsing Markdown files with front matter
/// - Processing internationalization and translations
/// - Applying themes and templates
/// - Generating HTML output with navigation features
/// - Creating Atom feeds
///
/// # Example
///
/// ```rust,no_run
/// use krik::generator::SiteGenerator;
/// use std::path::PathBuf;
///
/// let mut generator = SiteGenerator::new(
///     "content",           // Source directory
///     "_site",            // Output directory  
///     Some("themes/custom") // Optional theme directory
/// )?;
///
/// generator.scan_files()?;
/// generator.generate_site()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct SiteGenerator {
    /// Source directory containing Markdown files and assets
    pub source_dir: PathBuf,
    /// Output directory where the generated site will be written
    pub output_dir: PathBuf,
    /// Theme configuration and templates
    pub theme: Theme,
    /// Internationalization manager for multi-language support
    pub i18n: I18nManager,
    /// Site-wide configuration loaded from site.toml
    pub site_config: SiteConfig,
    /// Parsed documents ready for processing
    pub documents: Vec<Document>,
}

impl SiteGenerator {
    /// Creates a new `SiteGenerator` instance.
    ///
    /// # Arguments
    ///
    /// * `source_dir` - Directory containing Markdown files and content
    /// * `output_dir` - Directory where generated HTML will be written  
    /// * `theme_dir` - Optional custom theme directory (defaults to `themes/default`)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `SiteGenerator` or an error if initialization fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use krik::generator::SiteGenerator;
    ///
    /// // Using default theme
    /// let generator = SiteGenerator::new("content", "_site", None::<&str>)?;
    ///
    /// // Using custom theme
    /// let generator = SiteGenerator::new("content", "_site", Some("my-theme"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<P: AsRef<Path>>(
        source_dir: P,
        output_dir: P,
        theme_dir: Option<P>,
    ) -> KrikResult<Self> {
        let source_dir = source_dir.as_ref().to_path_buf();
        let output_dir = output_dir.as_ref().to_path_buf();
        
        let theme = if let Some(theme_path) = theme_dir {
            let path = theme_path.as_ref().to_path_buf();
            Theme::load_from_path(&path)
                .map_err(|_| KrikError::Theme(ThemeError {
                    kind: ThemeErrorKind::NotFound,
                    theme_path: path.clone(),
                    context: format!("Loading custom theme from {}", path.display()),
                }))?
        } else {
            let default_path = PathBuf::from("themes/default");
            Theme::load_from_path(&default_path).unwrap_or_else(|_| {
                Theme {
                    config: crate::theme::ThemeConfig {
                        name: "default".to_string(),
                        version: "1.0.0".to_string(),
                        author: None,
                        description: None,
                        templates: HashMap::new(),
                    },
                    templates: tera::Tera::new("themes/default/templates/**/*").unwrap_or_default(),
                    theme_path: default_path,
                }
            })
        };

        let i18n = I18nManager::new("en".to_string());
        
        // Load site configuration
        let site_config = SiteConfig::load_from_path(&source_dir);

        Ok(Self {
            source_dir,
            output_dir,
            theme,
            i18n,
            site_config,
            documents: Vec::new(),
        })
    }

    /// Scan files in the source directory and parse markdown documents
    pub fn scan_files(&mut self) -> KrikResult<()> {
        info!("Scanning for markdown files in: {}", self.source_dir.display());
        let result = super::markdown::scan_files(&self.source_dir, &mut self.documents)
            .map_err(|e| match e {
                KrikError::Generation(gen_err) => KrikError::Generation(gen_err),
                other => other,
            });
        
        match &result {
            Ok(_) => info!("Successfully scanned {} documents", self.documents.len()),
            Err(e) => error!("Failed to scan files: {}", e),
        }
        
        result
    }

    /// Generate the complete static site
    ///
    /// This orchestrates the entire site generation process:
    /// 1. Copy non-markdown files and theme assets
    /// 2. Generate HTML pages from documents
    /// 3. Generate index page with post listings
    /// 4. Generate Atom feed
    /// 5. Generate XML sitemap
    /// 6. Generate robots.txt
    /// 7. Generate PDFs (if pandoc and typst are available)
    pub fn generate_site(&self) -> KrikResult<()> {
        use super::pipeline::{EmitPhase, RenderPhase, ScanPhase, TransformPhase};

        info!("Starting site generation");
        debug!("Source directory: {}", self.source_dir.display());
        debug!("Output directory: {}", self.output_dir.display());

        let scan = ScanPhase;
        let transform = TransformPhase;
        let render = RenderPhase;
        let emit = EmitPhase;

        // Prepare output
        info!("Preparing output directory");
        emit.ensure_output_dir(&self.output_dir)?;

        // Scan
        info!("Scanning source files");
        let mut documents = scan.scan(&self.source_dir)?;
        debug!("Found {} documents to process", documents.len());

        // Transform
        info!("Transforming documents");
        transform.transform(&mut documents, &self.source_dir);

        // Assets
        info!("Copying assets");
        emit.copy_assets(&self.source_dir, &self.theme, &self.output_dir)?;

        // Render
        info!("Rendering pages");
        render.render_pages(&documents, &self.theme, &self.i18n, &self.site_config, &self.output_dir)?;
        render.render_index(&documents, &self.theme, &self.site_config, &self.output_dir)?;

        // Emit ancillary artifacts
        info!("Generating ancillary files");
        emit.emit_feed(&documents, &self.site_config, &self.output_dir)?;
        emit.emit_sitemap(&documents, &self.site_config, &self.output_dir)?;
        emit.emit_robots(&self.site_config, &self.output_dir)?;

        // Generate PDFs if tools are available
        if super::pdf::PdfGenerator::is_available() {
            info!("PDF generation tools available, generating PDFs");
            match super::pdf::PdfGenerator::new() {
                Ok(pdf_generator) => {
                    match pdf_generator.generate_pdfs(&documents, &self.source_dir, &self.output_dir, &self.site_config) {
                        Ok(generated_pdfs) => {
                            if !generated_pdfs.is_empty() {
                                info!("Generated {} PDF files alongside their HTML counterparts", generated_pdfs.len());
                            }
                        }
                        Err(e) => warn!("PDF generation failed: {}", e),
                    }
                }
                Err(e) => warn!("Could not initialize PDF generator: {}", e),
            }
        } else {
            debug!("PDF generation skipped: pandoc and/or typst not available in PATH");
        }

        info!("Site generation completed successfully");
        Ok(())
    }
}