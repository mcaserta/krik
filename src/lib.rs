//! # Krik - Fast Static Site Generator
//!
//! Krik is a fast static site generator written in Rust that transforms Markdown files
//! into beautiful, responsive websites with internationalization support and modern theming.
//!
//! ## Features
//!
//! - **Full Markdown Support**: GitHub Flavored Markdown with tables, footnotes, and code blocks
//! - **Internationalization**: Multi-language support with filename-based detection
//! - **Theme System**: Automatic light/dark mode with responsive design
//! - **Advanced Navigation**: Table of contents, footnote links, and scroll-to-top
//! - **Atom Feeds**: RFC 4287 compliant feed generation
//! - **Fast Performance**: Built with Rust for optimal speed
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use krik::generator::SiteGenerator;
//! use std::path::PathBuf;
//!
//! // Create a new site generator
//! let mut generator = SiteGenerator::new(
//!     PathBuf::from("content"),
//!     PathBuf::from("_site"),
//!     None::<PathBuf>
//! )?;
//!
//! // Scan for markdown files
//! generator.scan_files()?;
//!
//! // Generate the site
//! generator.generate_site()?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Content Organization
//!
//! - `content/posts/` - Blog posts (uses post template)
//! - `content/pages/` - Static pages (uses page template)
//! - `content/site.toml` - Site configuration
//! - `content/images/` - Images and assets (copied as-is)

pub mod parser;
pub mod theme;
pub mod i18n;
pub mod generator;
pub mod site;
pub mod server;

pub use parser::*;
pub use theme::*;
pub use i18n::*;
pub use generator::*;
pub use site::*;