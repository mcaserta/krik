//! Content linting and validation module
//!
//! This module has been refactored into separate modules for better maintainability:
//!
//! - `lint::core` - Core linting functionality
//! - `lint::link_checker` - HTTP link validation
//! - `lint::report_generator` - HTML report generation
//!
//! All functionality is re-exported through this module for backward compatibility.

pub mod core;
pub mod link_checker;
pub mod report_generator;

// Re-export everything for backward compatibility
pub use core::lint_content;
pub use link_checker::{check_links_in_directory, BrokenLink};
pub use report_generator::{generate_html_report, LintReport};

use crate::error::KrikResult;
use std::path::Path;
use tracing::debug;

/// Lint markdown content and check for broken links
pub async fn lint_content_with_links(content_dir: &Path) -> KrikResult<LintReport> {
    debug!("Starting content linting with link checking in: {}", content_dir.display());
    
    // First, run the regular linting
    let mut report = lint_content(content_dir)?;
    
    // Then check links
    let broken_links = check_links_in_directory(content_dir).await?;
    report.broken_links = broken_links;
    
    Ok(report)
}