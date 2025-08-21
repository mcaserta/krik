use crate::error::{GenerationError, GenerationErrorKind, KrikError, KrikResult};
use crate::parser::Document;
use crate::site::SiteConfig;
use crate::theme::Theme;
use chrono::{DateTime, Utc};
use std::path::Path;

/// Phase: scan the content directory and build the in-memory document list
pub struct ScanPhase;

impl ScanPhase {
    pub fn scan(&self, source_dir: &Path) -> KrikResult<Vec<Document>> {
        let mut documents = Vec::new();
        super::markdown::scan_files(source_dir, &mut documents).map_err(|e| match e {
            KrikError::Generation(gen_err) => KrikError::Generation(gen_err),
            other => other,
        })?;
        Ok(documents)
    }
}

/// Phase: apply transformations and enrichments to parsed documents before rendering
pub struct TransformPhase;

impl TransformPhase {
    /// Apply non-rendering transformations and return new immutable documents
    /// Currently: set missing dates from file modification time when available
    pub fn transform(&self, documents: Vec<Document>, source_dir: &Path) -> Vec<Document> {
        documents
            .into_iter()
            .map(|mut doc| {
                if doc.front_matter.date.is_none() {
                    let file_path = source_dir.join(&doc.file_path);
                    if let Ok(metadata) = std::fs::metadata(&file_path) {
                        if let Ok(modified) = metadata.modified() {
                            let dt: DateTime<Utc> = modified.into();
                            doc.front_matter.date = Some(dt);
                        }
                    }
                }
                doc
            })
            .collect()
    }
}

/// Phase: render HTML for pages and index using the theme
pub struct RenderPhase;

impl RenderPhase {
    pub fn render_pages(
        &self,
        documents: &[Document],
        theme: &Theme,
        site_config: &SiteConfig,
        output_dir: &Path,
    ) -> KrikResult<()> {
        super::templates::generate_pages(documents, theme, site_config, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Page generation failed: {e}"),
                )),
                context: "Generating HTML pages from documents".to_string(),
            }))
        })
    }

    pub fn render_index(
        &self,
        documents: &[Document],
        theme: &Theme,
        site_config: &SiteConfig,
        output_dir: &Path,
    ) -> KrikResult<()> {
        super::templates::generate_index(documents, theme, site_config, output_dir).map_err(
            |e| {
                KrikError::Generation(Box::new(GenerationError {
                    kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Index page generation failed: {e}"),
                    )),
                    context: "Generating index page with post listings".to_string(),
                }))
            },
        )
    }
}

/// Phase: emit non-HTML artifacts and copy assets
pub struct EmitPhase;

impl EmitPhase {
    pub fn ensure_output_dir(&self, output_dir: &Path) -> KrikResult<()> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir).map_err(|e| {
                KrikError::Generation(Box::new(GenerationError {
                    kind: GenerationErrorKind::OutputDirError(e),
                    context: format!("Creating output directory: {}", output_dir.display()),
                }))
            })?;
        }
        Ok(())
    }

    pub fn copy_assets(
        &self,
        source_dir: &Path,
        theme: &Theme,
        output_dir: &Path,
    ) -> KrikResult<()> {
        super::assets::copy_non_markdown_files(source_dir, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::AssetCopyError {
                    source: source_dir.to_path_buf(),
                    target: output_dir.to_path_buf(),
                    error: std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Asset copy failed: {e}"),
                    ),
                },
                context: "Copying non-markdown assets".to_string(),
            }))
        })?;

        super::assets::copy_theme_assets(theme, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::AssetCopyError {
                    source: theme.theme_path.clone(),
                    target: output_dir.to_path_buf(),
                    error: std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Theme asset copy failed: {e}"),
                    ),
                },
                context: "Copying theme assets".to_string(),
            }))
        })
    }

    pub fn emit_feed(
        &self,
        documents: &[Document],
        site_config: &SiteConfig,
        output_dir: &Path,
    ) -> KrikResult<()> {
        super::feeds::generate_feed(documents, site_config, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::FeedError(format!("Atom feed generation failed: {e}")),
                context: "Generating Atom feed for posts".to_string(),
            }))
        })
    }

    pub fn emit_sitemap(
        &self,
        documents: &[Document],
        site_config: &SiteConfig,
        output_dir: &Path,
    ) -> KrikResult<()> {
        super::sitemap::generate_sitemap(documents, site_config, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::SitemapError(format!(
                    "XML sitemap generation failed: {e}"
                )),
                context: "Generating XML sitemap with multilingual support".to_string(),
            }))
        })
    }

    pub fn emit_robots(&self, site_config: &SiteConfig, output_dir: &Path) -> KrikResult<()> {
        super::robots::generate_robots(site_config, output_dir).map_err(|e| {
            KrikError::Generation(Box::new(GenerationError {
                kind: GenerationErrorKind::OutputDirError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("robots.txt generation failed: {e}"),
                )),
                context: "Generating robots.txt with sitemap reference".to_string(),
            }))
        })
    }
}
