use crate::parser::Document;
use crate::theme::Theme;
use crate::i18n::I18nManager;
use crate::site::SiteConfig;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let source_dir = source_dir.as_ref().to_path_buf();
        let output_dir = output_dir.as_ref().to_path_buf();
        
        let theme = if let Some(theme_path) = theme_dir {
            Theme::load_from_path(theme_path)?
        } else {
            Theme::load_from_path("themes/default").unwrap_or_else(|_| {
                Theme {
                    config: crate::theme::ThemeConfig {
                        name: "default".to_string(),
                        version: "1.0.0".to_string(),
                        author: None,
                        description: None,
                        templates: HashMap::new(),
                    },
                    templates: tera::Tera::new("themes/default/templates/**/*").unwrap_or_default(),
                    theme_path: PathBuf::from("themes/default"),
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
    pub fn scan_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        super::markdown::scan_files(&self.source_dir, &mut self.documents)
    }

    /// Generate the complete static site
    ///
    /// This orchestrates the entire site generation process:
    /// 1. Copy non-markdown files and theme assets
    /// 2. Generate HTML pages from documents
    /// 3. Generate index page with post listings
    /// 4. Generate Atom feed
    /// 5. Generate XML sitemap
    pub fn generate_site(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure output directory exists
        if !self.output_dir.exists() {
            std::fs::create_dir_all(&self.output_dir)?;
        }

        // Copy assets and non-markdown files
        super::assets::copy_non_markdown_files(&self.source_dir, &self.output_dir)?;
        super::assets::copy_theme_assets(&self.theme, &self.output_dir)?;

        // Generate HTML pages
        super::templates::generate_pages(&self.documents, &self.theme, &self.i18n, &self.site_config, &self.output_dir)?;

        // Generate index page
        super::templates::generate_index(&self.documents, &self.theme, &self.site_config, &self.output_dir)?;

        // Generate Atom feed
        super::feeds::generate_feed(&self.documents, &self.site_config, &self.output_dir)?;

        // Generate XML sitemap
        super::sitemap::generate_sitemap(&self.documents, &self.site_config, &self.output_dir)?;

        Ok(())
    }
}