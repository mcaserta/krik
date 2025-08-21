use crate::error::{KrikError, KrikResult, ThemeError, ThemeErrorKind};
use crate::i18n::I18nManager;
use crate::parser::Document;
use crate::site::SiteConfig;
use crate::theme::Theme;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};
use crate::i18n;

#[derive(Debug)]
pub enum ChangeType {
    ThemeRelated,
    SiteConfig,
    Markdown { relative_path: String },
    Asset,
    Unrelated,
}

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
        if let Ok(abs) = std::fs::canonicalize(&source_dir) {
            source_dir = abs;
        }

        let mut output_dir = output_dir.as_ref().to_path_buf();
        // Output directory might not exist yet; canonicalize only if it does
        if output_dir.exists() {
            if let Ok(abs) = std::fs::canonicalize(&output_dir) {
                output_dir = abs;
            }
        }

        let theme = if let Some(theme_path) = theme_dir {
            let mut path = theme_path.as_ref().to_path_buf();
            if let Ok(abs) = std::fs::canonicalize(&path) {
                path = abs;
            }
            Theme::builder()
                .theme_path(&path)
                .autoescape_html(false)
                .enable_reload(false)
                .build()
                .map_err(|_| {
                    KrikError::Theme(Box::new(ThemeError {
                        kind: ThemeErrorKind::NotFound,
                        theme_path: path.clone(),
                        context: format!("Loading custom theme from {}", path.display()),
                    }))
                })?
        } else {
            let default_path = PathBuf::from("themes/default");
            let default_path = if let Ok(abs) = std::fs::canonicalize(&default_path) {
                abs
            } else {
                default_path
            };
            Theme::builder()
                .theme_path(&default_path)
                .autoescape_html(false)
                .enable_reload(false)
                .build()
                .unwrap_or_else(|_| Theme {
                    config: crate::theme::ThemeConfig {
                        name: "default".to_string(),
                        version: "1.0.0".to_string(),
                        author: None,
                        description: None,
                        templates: HashMap::new(),
                    },
                    templates: tera::Tera::new("themes/default/templates/**/*").unwrap_or_default(),
                    theme_path: default_path,
                })
        };

        let i18n = I18nManager::new("en".to_string());

        // Load site configuration with proper error handling
        let site_config = match SiteConfig::load_from_path(&source_dir) {
            Ok(cfg) => cfg,
            Err(e) => {
                warn!(
                    "Failed to load site configuration: {}. Falling back to defaults.",
                    e
                );
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
        info!(
            "Scanning for markdown files in: {}",
            self.source_dir.display()
        );
        // Full scan rebuilds the cache
        self.document_cache.clear();
        self.documents.clear();
        let result = super::markdown::scan_files(&self.source_dir, &mut self.documents).map_err(
            |e| match e {
                KrikError::Generation(gen_err) => KrikError::Generation(gen_err),
                other => other,
            },
        );

        match &result {
            Ok(_) => {
                // Populate cache from documents
                for doc in &self.documents {
                    self.document_cache
                        .insert(doc.file_path.clone(), doc.clone());
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
        render.render_pages(
            &documents,
            &self.theme,
            &self.i18n,
            &self.site_config,
            &self.output_dir,
        )?;
        render.render_index(
            &documents,
            &self.theme,
            &self.site_config,
            &self.i18n,
            &self.output_dir,
        )?;

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
                    match pdf_generator.generate_pdfs(
                        &documents,
                        &self.source_dir,
                        &self.output_dir,
                        &self.site_config,
                    ) {
                        Ok(generated_pdfs) => {
                            if !generated_pdfs.is_empty() {
                                info!(
                                    "Generated {} PDF files alongside their HTML counterparts",
                                    generated_pdfs.len()
                                );
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
        let changed_path = changed_path.as_ref();
        let change_type =
            analyze_change_type(changed_path, &self.theme.theme_path, &self.source_dir)?;

        match change_type {
            ChangeType::ThemeRelated | ChangeType::SiteConfig => {
                debug!("Theme or site config change detected, triggering full regeneration");
                self.generate_site()
            }
            ChangeType::Markdown { relative_path } => {
                self.handle_markdown_change(&relative_path, changed_path, is_removed)
            }
            ChangeType::Asset => self.handle_asset_change(changed_path, is_removed),
            ChangeType::Unrelated => {
                debug!("Change not related to content or theme, triggering full regeneration");
                self.generate_site()
            }
        }
    }

    /// Find all documents that are language variants of the given document.
    /// Language variants share the same base name but have different language extensions.
    /// For example: "welcome.md", "welcome.it.md", "welcome.fr.md" are all variants.
    fn find_language_variants(&self, target_path: &str) -> Vec<String> {
        let mut variants = Vec::new();

        // Extract base name by removing language and extension
        // Example: "pages/welcome.it.md" -> "pages/welcome"
        let path_buf = std::path::PathBuf::from(target_path);
        let parent = path_buf
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        if let Some(_filename) = path_buf.file_name().and_then(|n| n.to_str()) {
            let base_name = if let Some(stem) = path_buf.file_stem().and_then(|s| s.to_str()) {
                // Check if stem contains a language code (e.g., "welcome.it")
                if let Some(dot_pos) = stem.rfind('.') {
                    let potential_lang = &stem[dot_pos + 1..];
                    // Check if it's a known language code
                    if ["en", "it", "es", "fr", "de", "pt", "ja", "zh", "ru", "ar"]
                        .contains(&potential_lang)
                    {
                        &stem[..dot_pos] // Remove language part
                    } else {
                        stem // No language code found
                    }
                } else {
                    stem // No dots in stem
                }
            } else {
                return variants;
            };

            // Find all documents with the same base name
            for doc in &self.documents {
                let doc_path_buf = std::path::PathBuf::from(&doc.file_path);
                let doc_parent = doc_path_buf
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Must be in same directory
                if doc_parent != parent {
                    continue;
                }

                if let Some(_doc_filename) = doc_path_buf.file_name().and_then(|n| n.to_str()) {
                    if let Some(doc_stem) = doc_path_buf.file_stem().and_then(|s| s.to_str()) {
                        let doc_base_name = if let Some(dot_pos) = doc_stem.rfind('.') {
                            let potential_lang = &doc_stem[dot_pos + 1..];
                            if i18n::SUPPORTED_LANGUAGES.contains_key(&potential_lang) {
                                &doc_stem[..dot_pos]
                            } else {
                                doc_stem
                            }
                        } else {
                            doc_stem
                        };

                        // If base names match, this is a language variant
                        if doc_base_name == base_name {
                            variants.push(doc.file_path.clone());
                        }
                    }
                }
            }
        }

        variants
    }

    /// Handle markdown file changes by updating the document cache and re-rendering affected pages
    fn handle_markdown_change(
        &mut self,
        relative_path: &str,
        changed_path: &Path,
        is_removed: bool,
    ) -> KrikResult<()> {
        use super::pipeline::{EmitPhase, RenderPhase, TransformPhase};

        let transform = TransformPhase;
        let render = RenderPhase;
        let emit = EmitPhase;

        emit.ensure_output_dir(&self.output_dir)?;

        let mut documents = self.documents.clone();

        if is_removed {
            self.handle_markdown_removal(relative_path, &mut documents)
        } else {
            self.handle_markdown_update(relative_path, changed_path, &mut documents)?
        };

        // Transform documents for correct dates before rendering
        transform.transform(&mut documents, &self.source_dir);

        if !is_removed {
            self.render_language_variants(relative_path, &documents)?;
        }

        // Persist updated working set back into generator state
        self.documents = documents;

        // Update global artifacts that depend on full document set
        debug!("updating global artifacts (index/feed/sitemap/robots) after single-page change");
        render.render_index(
            &self.documents,
            &self.theme,
            &self.site_config,
            &self.i18n,
            &self.output_dir,
        )?;
        emit.emit_feed(&self.documents, &self.site_config, &self.output_dir)?;
        emit.emit_sitemap(&self.documents, &self.site_config, &self.output_dir)?;
        emit.emit_robots(&self.site_config, &self.output_dir)?;

        Ok(())
    }

    /// Handle asset file changes by copying or removing the file
    fn handle_asset_change(&self, changed_path: &Path, is_removed: bool) -> KrikResult<()> {
        use super::pipeline::EmitPhase;

        let emit = EmitPhase;
        emit.ensure_output_dir(&self.output_dir)?;

        if is_removed {
            debug!("removing single asset {}", changed_path.display());
            super::assets::remove_single_asset(&self.source_dir, &self.output_dir, changed_path)
                .map_err(|e| {
                    create_asset_error(
                        "Removing single changed asset",
                        &self.source_dir,
                        &self.output_dir,
                        Box::new(e),
                    )
                })
        } else {
            debug!("copying single asset {}", changed_path.display());
            super::assets::copy_single_asset(&self.source_dir, &self.output_dir, changed_path)
                .map_err(|e| {
                    create_asset_error(
                        "Copying single changed asset",
                        &self.source_dir,
                        &self.output_dir,
                        Box::new(e),
                    )
                })
        }
    }

    /// Handle markdown file removal by cleaning up cache and output files
    fn handle_markdown_removal(&mut self, relative_path: &str, documents: &mut Vec<Document>) {
        debug!("removing generated page for {}", relative_path);
        // Remove from cache and the mirrored HTML file
        if let Some(rel_removed) = self.document_cache.remove(relative_path) {
            // Also remove from current documents vector copy
            documents.retain(|d| d.file_path != rel_removed.file_path);
            // If removed document had a PDF generated, remove corresponding PDF file
            if rel_removed.front_matter.pdf.unwrap_or(false) {
                let mut pdf_rel_path = std::path::PathBuf::from(relative_path);
                pdf_rel_path.set_extension("pdf");
                let pdf_output_path = self.output_dir.join(pdf_rel_path);
                if pdf_output_path.exists() {
                    let _ = std::fs::remove_file(pdf_output_path);
                }
            }
        }
        let mut output_path = std::path::PathBuf::from(relative_path);
        output_path.set_extension("html");
        let output_path = self.output_dir.join(output_path);
        if output_path.exists() {
            let _ = std::fs::remove_file(output_path);
        }
    }

    /// Handle markdown file update by parsing and updating cache
    fn handle_markdown_update(
        &mut self,
        relative_path: &str,
        changed_path: &Path,
        documents: &mut Vec<Document>,
    ) -> KrikResult<()> {
        match super::markdown::parse_single_file(&self.source_dir, changed_path) {
            Ok(doc) => {
                let prev_pdf = self
                    .document_cache
                    .get(&doc.file_path)
                    .and_then(|d| d.front_matter.pdf)
                    .unwrap_or(false);

                // Replace or insert into cache and working set
                self.document_cache
                    .insert(doc.file_path.clone(), doc.clone());
                if let Some(slot) = documents.iter_mut().find(|d| d.file_path == doc.file_path) {
                    *slot = doc;
                } else {
                    documents.push(doc);
                }

                // Handle PDF generation/removal
                self.handle_pdf_change(relative_path, documents, prev_pdf)?;
                Ok(())
            }
            Err(e) => {
                warn!(
                    "Failed to parse changed file {}: {}. Falling back to full rescan.",
                    changed_path.display(),
                    e
                );
                documents.clear();
                super::markdown::scan_files(&self.source_dir, documents)?;
                // rebuild cache from full scan
                self.document_cache.clear();
                for d in documents {
                    self.document_cache.insert(d.file_path.clone(), d.clone());
                }
                Ok(())
            }
        }
    }

    /// Handle PDF generation or removal based on document settings
    fn handle_pdf_change(
        &self,
        relative_path: &str,
        documents: &[Document],
        prev_pdf: bool,
    ) -> KrikResult<()> {
        if let Some(current_doc) = documents.iter().find(|d| d.file_path == relative_path) {
            let current_pdf = current_doc.front_matter.pdf.unwrap_or(false);
            let mut pdf_rel_path = std::path::PathBuf::from(&current_doc.file_path);
            pdf_rel_path.set_extension("pdf");
            let pdf_output_path = self.output_dir.join(pdf_rel_path);

            if current_pdf {
                // Generate or regenerate PDF
                if super::pdf::PdfGenerator::is_available() {
                    match super::pdf::PdfGenerator::new() {
                        Ok(pdf_gen) => {
                            let input_path = self.source_dir.join(&current_doc.file_path);
                            let _ = pdf_gen.generate_pdf_from_file(
                                &input_path,
                                &pdf_output_path,
                                &self.source_dir,
                                &self.site_config,
                                &current_doc.language,
                            );
                        }
                        Err(e) => {
                            warn!("PDF generator init failed during incremental: {}", e);
                        }
                    }
                } else if prev_pdf && pdf_output_path.exists() {
                    // Tools no longer available; remove stale PDF to reflect state
                    let _ = std::fs::remove_file(&pdf_output_path);
                }
            } else if prev_pdf {
                // PDF flag was removed; delete existing PDF if present
                if pdf_output_path.exists() {
                    let _ = std::fs::remove_file(&pdf_output_path);
                }
            }
        }
        Ok(())
    }

    /// Render all language variants of a document
    fn render_language_variants(
        &self,
        relative_path: &str,
        documents: &[Document],
    ) -> KrikResult<()> {
        let variant_paths = self.find_language_variants(relative_path);
        debug!(
            "found {} language variants for {}: {:?}",
            variant_paths.len(),
            relative_path,
            variant_paths
        );

        // Render all language variants (including the changed one)
        let mut rendered_any = false;
        for variant_path in &variant_paths {
            if let Some(doc) = documents.iter().find(|d| d.file_path == *variant_path) {
                debug!("re-rendering language variant page {}", variant_path);
                super::templates::generate_page(
                    doc,
                    documents,
                    &self.theme,
                    &self.i18n,
                    &self.site_config,
                    &self.output_dir,
                )
                .map_err(|e| {
                    KrikError::Generation(Box::new(crate::error::GenerationError {
                        kind: crate::error::GenerationErrorKind::OutputDirError(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("Page generation failed for {}: {e}", variant_path),
                            ),
                        ),
                        context: "Incremental language variant page generation".to_string(),
                    }))
                })?;
                rendered_any = true;
            }
        }

        if !rendered_any {
            debug!(
                "changed page {} and its variants not present in scanned documents; triggering full regeneration",
                relative_path
            );
            return Err(KrikError::Generation(Box::new(crate::error::GenerationError {
                kind: crate::error::GenerationErrorKind::OutputDirError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Document variants not found",
                )),
                context: "Language variant rendering".to_string(),
            })));
        }

        Ok(())
    }
}

/// Analyze the type of change and determine how to handle it
pub fn analyze_change_type(
    changed_path: &Path,
    theme_path: &Path,
    source_dir: &Path,
) -> KrikResult<ChangeType> {
    // If change is inside theme dir or is an HTML template, do full regen.
    let is_theme_related = theme_path.is_dir() && changed_path.starts_with(theme_path);
    let is_template_ext = is_html_template(changed_path);

    if is_theme_related || is_template_ext {
        return Ok(ChangeType::ThemeRelated);
    }

    // Changes within content directory
    let canonical_changed =
        std::fs::canonicalize(changed_path).unwrap_or_else(|_| changed_path.to_path_buf());
    let canonical_source =
        std::fs::canonicalize(source_dir).unwrap_or_else(|_| source_dir.to_path_buf());

    if canonical_changed.starts_with(&canonical_source) {
        let is_markdown = changed_path.extension().is_some_and(|ext| ext == "md");
        let is_site_toml = changed_path.file_name() == Some(OsStr::new("site.toml"));

        if is_site_toml {
            return Ok(ChangeType::SiteConfig);
        }

        if is_markdown {
            let relative_path = canonical_changed
                .strip_prefix(&canonical_source)
                .map(|rel| rel.to_string_lossy().to_string())
                .map_err(|_| {
                    debug!(
                        "failed to compute relative path for {}",
                        changed_path.display()
                    );
                    KrikError::Generation(Box::new(crate::error::GenerationError {
                        kind: crate::error::GenerationErrorKind::OutputDirError(
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "Failed to compute relative path",
                            ),
                        ),
                        context: "Path canonicalization".to_string(),
                    }))
                })?;
            return Ok(ChangeType::Markdown { relative_path });
        } else {
            return Ok(ChangeType::Asset);
        }
    }

    Ok(ChangeType::Unrelated)
}

/// Check if a path is an HTML template
pub fn is_html_template(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("html"))
        .unwrap_or(false)
        && path
            .components()
            .any(|c| c.as_os_str() == OsStr::new("templates"))
}

/// Create a standardized asset error
pub fn create_asset_error(
    context: &str,
    source_dir: &Path,
    output_dir: &Path,
    error: Box<dyn std::error::Error + Send + Sync>,
) -> KrikError {
    KrikError::Generation(Box::new(crate::error::GenerationError {
        kind: crate::error::GenerationErrorKind::AssetCopyError {
            source: source_dir.to_path_buf(),
            target: output_dir.to_path_buf(),
            error: std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Asset operation failed: {error}"),
            ),
        },
        context: context.to_string(),
    }))
}
