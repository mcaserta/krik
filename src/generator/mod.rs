//! Site generation functionality for Krik
//!
//! This module provides focused components for better maintainability:
//! 
//! - `core`: Main SiteGenerator struct and orchestration
//! - `markdown`: Markdown processing and content parsing  
//! - `assets`: Asset copying and file management
//! - `templates`: HTML template rendering and page generation
//! - `feeds`: Atom feed generation
//! - `sitemap`: XML sitemap generation
//! - `robots`: robots.txt generation
//! - `pdf`: PDF generation using pandoc and typst

pub mod core;
pub mod markdown;
pub mod assets;
pub mod templates;
pub mod feeds;
pub mod sitemap;
pub mod robots;
pub mod pdf;

// Re-export the main SiteGenerator for backwards compatibility
pub use core::SiteGenerator;

