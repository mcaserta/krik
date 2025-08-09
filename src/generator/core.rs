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
    /// Incremental cache: map from relative file path to Document
    pub document_cache: HashMap<String, Document>,
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
        // Normalize paths to avoid mismatches between absolute and relative paths
        let mut source_dir = source_dir.as_ref().to_path_buf();
        if let Ok(abs) = std::fs::canonicalize(&source_dir) { source_dir = abs; }

        let mut output_dir = output_dir.as_ref().to_path_buf();
        // Output directory might not exist yet; canonicalize only if it does
        if output_dir.exists() {
            if let Ok(abs) = std::fs::canonicalize(&output_dir) { output_dir = abs; }
        }
        
        let theme = if let Some(theme_path) = theme_dir {
            let mut path = theme_path.as_ref().to_path_buf();
            if let Ok(abs) = std::fs::canonicalize(&path) { path = abs; }
            Theme::builder()
                .theme_path(&path)
                .autoescape_html(false)
                .enable_reload(false)
                .build()
                .map_err(|_| KrikError::Theme(ThemeError {
                    kind: ThemeErrorKind::NotFound,
                    theme_path: path.clone(),
                    context: format!("Loading custom theme from {}", path.display()),
                }))?
        } else {
            let default_path = PathBuf::from("themes/default");
            let default_path = if let Ok(abs) = std::fs::canonicalize(&default_path) { abs } else { default_path };
            Theme::builder()
                .theme_path(&default_path)
                .autoescape_html(false)
                .enable_reload(false)
                .build()
                .unwrap_or_else(|_| {
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
        
        // Load site configuration with proper error handling
        let site_config = match SiteConfig::load_from_path(&source_dir) {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!("Failed to load site configuration: {}. Falling back to defaults.", e);
                SiteConfig::default()
            }
        };

        Ok(Self {
            source_dir,
            output_dir,
            theme,
            i18n,
            site_config,
            documents: Vec::new(),
            document_cache: HashMap::new(),
        })
    }

    /// Scan files in the source directory and parse markdown documents
    pub fn scan_files(&mut self) -> KrikResult<()> {
        info!("Scanning for markdown files in: {}", self.source_dir.display());
        // Full scan rebuilds the cache
        self.document_cache.clear();
        self.documents.clear();
        let result = super::markdown::scan_files(&self.source_dir, &mut self.documents)
            .map_err(|e| match e {
                KrikError::Generation(gen_err) => KrikError::Generation(gen_err),
                other => other,
            });
        
        match &result {
            Ok(_) => {
                // Populate cache from documents
                for doc in &self.documents {
                    self.document_cache.insert(doc.file_path.clone(), doc.clone());
                }
                info!("Successfully scanned {} documents", self.documents.len())
            }
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
        render.render_index(&documents, &self.theme, &self.site_config, &self.i18n, &self.output_dir)?;

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

    /// Incrementally (re)generate outputs affected by a single changed content or asset file.
    ///
    /// Behavior:
    /// - If a markdown file changed: re-scan just that file, update/emit its HTML, and re-render index/feed/sitemap.
    /// - If a non-markdown content asset changed: copy that single asset into the output.
    /// - If a content file was removed: remove the mirrored output file and refresh index/feed/sitemap.
    /// - If a theme file changed (templates/assets), fall back to full regeneration as templates affect many pages.
    pub fn generate_incremental_for_path<P: AsRef<Path>>(
        &mut self,
        changed_path: P,
        is_removed: bool,
    ) -> KrikResult<()> {
        use super::pipeline::{EmitPhase, RenderPhase, TransformPhase};
        use std::ffi::OsStr;

        let changed_path = changed_path.as_ref();
        // no need to instantiate ScanPhase here
        let transform = TransformPhase;
        let render = RenderPhase;
        let emit = EmitPhase;

        emit.ensure_output_dir(&self.output_dir)?;

        // If change is inside theme dir or is an HTML template, do full regen.
        let is_theme_related = self.theme.theme_path.as_path().is_dir()
            && changed_path.starts_with(&self.theme.theme_path);
        let is_template_ext = changed_path
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| ext.eq_ignore_ascii_case("html"))
            .unwrap_or(false)
            && changed_path
                .components()
                .any(|c| c.as_os_str() == OsStr::new("templates"));
        if is_theme_related || is_template_ext {
            debug!(
                "theme-related change detected at {} (template_ext={}), triggering full regeneration",
                changed_path.display(),
                is_template_ext
            );
            // Templates affect all pages; safest to rebuild fully.
            return self.generate_site();
        }

        // Changes within content directory
        // Compare using canonicalized paths to avoid absolute/relative mismatches
        let canonical_changed = std::fs::canonicalize(changed_path).unwrap_or_else(|_| changed_path.to_path_buf());
        let canonical_source = std::fs::canonicalize(&self.source_dir).unwrap_or_else(|_| self.source_dir.clone());

        if canonical_changed.starts_with(&canonical_source) {
            let is_markdown = changed_path.extension().is_some_and(|ext| ext == "md");
            let is_site_toml = changed_path.file_name() == Some(OsStr::new("site.toml"));
            if is_site_toml {
                debug!("site.toml changed ({}), triggering full regeneration", changed_path.display());
                // Site configuration changes can affect all pages
                return self.generate_site();
            }

            if is_markdown {
                // Determine relative path for matching Document
                let relative_path = match canonical_changed.strip_prefix(&canonical_source) {
                    Ok(rel) => rel.to_string_lossy().to_string(),
                    Err(_) => {
                        debug!(
                            "failed to compute relative path for {}, falling back to full regeneration",
                            changed_path.display()
                        );
                        return self.generate_site();
                    }
                };

                // Update cache for this single file
                let mut documents = self.documents.clone();
                match super::markdown::parse_single_file(&self.source_dir, &canonical_changed) {
                    Ok(doc) => {
                        // Replace or insert into cache and working set
                        self.document_cache.insert(doc.file_path.clone(), doc.clone());
                        if let Some(slot) = documents.iter_mut().find(|d| d.file_path == doc.file_path) {
                            *slot = doc;
                        } else {
                            documents.push(doc);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse changed file {}: {}. Falling back to full rescan.", changed_path.display(), e);
                        documents.clear();
                        super::markdown::scan_files(&self.source_dir, &mut documents)?;
                        // rebuild cache from full scan
                        self.document_cache.clear();
                        for d in &documents { self.document_cache.insert(d.file_path.clone(), d.clone()); }
                    }
                }

                // Transform documents for correct dates before rendering
                transform.transform(&mut documents, &self.source_dir);

                if is_removed {
                    debug!("removing generated page for {}", relative_path);
                    // Remove from cache and the mirrored HTML file
                    if let Some(rel_removed) = self.document_cache.remove(&relative_path) {
                        // Also remove from current documents vector copy
                        documents.retain(|d| d.file_path != rel_removed.file_path);
                    }
                    let mut output_path = std::path::PathBuf::from(&relative_path);
                    output_path.set_extension("html");
                    let output_path = self.output_dir.join(output_path);
                    if output_path.exists() {
                        let _ = std::fs::remove_file(output_path);
                    }
                } else {
                    // Render only the changed page
                    if let Some(doc) = documents.iter().find(|d| d.file_path == relative_path) {
                        debug!("re-rendering single page {}", relative_path);
                        super::templates::generate_page(
                            doc,
                            &documents,
                            &self.theme,
                            &self.i18n,
                            &self.site_config,
                            &self.output_dir,
                        )
                        .map_err(|e| KrikError::Generation(crate::error::GenerationError {
                            kind: crate::error::GenerationErrorKind::OutputDirError(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Page generation failed: {e}"),
                            )),
                            context: "Incremental page generation".to_string(),
                        }))?;
                    } else {
                        debug!(
                            "changed page {} not present in scanned documents; triggering full regeneration",
                            relative_path
                        );
                        // If not found, fall back to full regeneration
                        return self.generate_site();
                    }
                }

                // Persist updated working set back into generator state
                self.documents = documents.clone();

                // Update global artifacts that depend on full document set
                debug!("updating global artifacts (index/feed/sitemap/robots) after single-page change");
                render.render_index(&self.documents, &self.theme, &self.site_config, &self.i18n, &self.output_dir)?;
                emit.emit_feed(&self.documents, &self.site_config, &self.output_dir)?;
                emit.emit_sitemap(&self.documents, &self.site_config, &self.output_dir)?;
                emit.emit_robots(&self.site_config, &self.output_dir)?;
                return Ok(());
            } else {
                // Non-markdown asset changed under content: copy or remove the single file
                if is_removed {
                    debug!("removing single asset {}", changed_path.display());
                    super::assets::remove_single_asset(&self.source_dir, &self.output_dir, changed_path)
                        .map_err(|e| KrikError::Generation(crate::error::GenerationError {
                            kind: crate::error::GenerationErrorKind::AssetCopyError {
                                source: self.source_dir.clone(),
                                target: self.output_dir.clone(),
                                error: std::io::Error::new(std::io::ErrorKind::Other, format!("Asset remove failed: {e}")),
                            },
                            context: "Removing single changed asset".to_string(),
                        }))?;
                } else {
                    debug!("copying single asset {}", changed_path.display());
                    super::assets::copy_single_asset(&self.source_dir, &self.output_dir, changed_path)
                        .map_err(|e| KrikError::Generation(crate::error::GenerationError {
                            kind: crate::error::GenerationErrorKind::AssetCopyError {
                                source: self.source_dir.clone(),
                                target: self.output_dir.clone(),
                                error: std::io::Error::new(std::io::ErrorKind::Other, format!("Asset copy failed: {e}")),
                            },
                            context: "Copying single changed asset".to_string(),
                        }))?;
                }
                return Ok(());
            }
        }

        debug!(
            "change at {} not under source/theme (or canonicalization mismatch); triggering full regeneration",
            changed_path.display()
        );
        // Default: fall back to full regeneration
        self.generate_site()
    }
}