//! Site generation functionality for Krik
//!
//! This module provides focused components for better maintainability:
//! 
//! - `core`: Main SiteGenerator struct and orchestration
//! - `markdown`: Markdown processing and content parsing  
//! - `assets`: Asset copying and file management
//! - `templates`: HTML template rendering and page generation
//! - `feeds`: Atom feed generation

pub mod core;
pub mod markdown;
pub mod assets;
pub mod templates;
pub mod feeds;

// Re-export the main SiteGenerator for backwards compatibility
pub use core::SiteGenerator;

