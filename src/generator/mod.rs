//! Site generation functionality for Krik
//!
//! This module provides focused components for better maintainability:
//! 
//! - `core`: Main SiteGenerator struct and orchestration
//! - `markdown`: Markdown processing and content parsing  
//! - `ast_parser`: AST-based parsing for TOC and footnotes
//! - `assets`: Asset copying and file management
//! - `templates`: HTML template rendering and page generation
//! - `feeds`: Atom feed generation
//! - `sitemap`: XML sitemap generation
//! - `robots`: robots.txt generation
//! - `pdf`: PDF generation using pandoc and typst

pub mod core;
pub mod markdown;
pub mod ast_parser;
pub mod assets;
pub mod templates;
pub mod feeds;
pub mod sitemap;
pub mod robots;
pub mod pdf;
pub mod pipeline;

// Re-export the main SiteGenerator for backwards compatibility
pub use core::SiteGenerator;

