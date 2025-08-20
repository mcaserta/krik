use crate::error::KrikResult;
use futures_util::stream::{self, StreamExt};
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info, warn};
use url::Url;
use walkdir::WalkDir;

/// Information about a broken link
#[derive(Debug, Clone)]
pub struct BrokenLink {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub url: String,
    pub error: String,
}

/// Information about a link to be checked
#[derive(Debug, Clone)]
struct LinkToCheck {
    file_path: PathBuf,
    line_number: usize,
    url: String,
}

/// Check all links in markdown files within a directory
pub async fn check_links_in_directory(content_dir: &Path) -> KrikResult<Vec<BrokenLink>> {
    debug!(
        "Starting parallel link scanning in directory: {}",
        content_dir.display()
    );

    // First, collect all links from all files
    let links_to_check = collect_links_from_files(content_dir)?;

    if links_to_check.is_empty() {
        info!("No HTTP(S) links found to check");
        return Ok(Vec::new());
    }

    let total_links = links_to_check.len();
    info!("Found {} links to check across all files", total_links);
    info!("Starting parallel link validation (max 10 concurrent requests)...");

    // Show the links being checked for better user experience
    for link in &links_to_check {
        debug!(
            "Will check: {} from {}:{}",
            link.url,
            link.file_path.display(),
            link.line_number
        );
    }

    // Create a shared HTTP client for all requests
    let client = Arc::new(Client::new());

    // Process links in parallel with a concurrency limit
    let broken_links = stream::iter(links_to_check)
        .map(|link| {
            let client = Arc::clone(&client);
            async move { check_single_link_with_logging(client, link).await }
        })
        .buffer_unordered(10) // Process up to 10 links concurrently
        .filter_map(|result| async move { result })
        .collect::<Vec<_>>()
        .await;

    let working_links = total_links - broken_links.len();
    info!(
        "Link checking completed. {} working, {} broken, {} total",
        working_links,
        broken_links.len(),
        total_links
    );
    Ok(broken_links)
}

/// Collect all links from markdown files in a directory
fn collect_links_from_files(content_dir: &Path) -> KrikResult<Vec<LinkToCheck>> {
    let mut links_to_check = Vec::new();

    // Precompiled regex for extracting links
    static LINK_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[([^\]]*)\]\(([^\s)]+)(?:\s[^)]+)?\)").unwrap());

    debug!("Scanning files for links in: {}", content_dir.display());
    let mut files_scanned = 0;

    for entry in WalkDir::new(content_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Only check markdown files
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
            continue;
        }

        // Skip site config
        if path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
            continue;
        }

        files_scanned += 1;
        debug!("Scanning file for links: {}", path.display());

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to read file {}: {}", path.display(), e);
                continue;
            }
        };

        let mut file_link_count = 0;

        // Extract links from each line
        for (line_num, line) in content.lines().enumerate() {
            for cap in LINK_REGEX.captures_iter(line) {
                if let Some(url_match) = cap.get(2) {
                    let url_str = url_match.as_str();

                    // Skip relative links, anchor links, and email links
                    if url_str.starts_with('#')
                        || url_str.starts_with("mailto:")
                        || (!url_str.starts_with("http://") && !url_str.starts_with("https://"))
                    {
                        debug!("Skipping non-HTTP link: {}", url_str);
                        continue;
                    }

                    file_link_count += 1;
                    links_to_check.push(LinkToCheck {
                        file_path: path.to_path_buf(),
                        line_number: line_num + 1, // 1-indexed
                        url: url_str.to_string(),
                    });
                }
            }
        }

        if file_link_count > 0 {
            debug!(
                "Found {} HTTP(S) links in {}",
                file_link_count,
                path.display()
            );
        }
    }

    debug!(
        "Scanned {} files, found {} total HTTP(S) links",
        files_scanned,
        links_to_check.len()
    );
    Ok(links_to_check)
}

/// Check a single link with comprehensive logging
async fn check_single_link_with_logging(
    client: Arc<Client>,
    link: LinkToCheck,
) -> Option<BrokenLink> {
    debug!(
        "🔗 Checking: {} from {}:{}",
        link.url,
        link.file_path.display(),
        link.line_number
    );

    match check_link(&client, &link.url).await {
        Ok(()) => {
            debug!("✅ OK: {}", link.url);
            None
        }
        Err(error) => {
            warn!(
                "❌ BROKEN: {} from {}:{} - {}",
                link.url,
                link.file_path.display(),
                link.line_number,
                error
            );
            Some(BrokenLink {
                file_path: link.file_path,
                line_number: link.line_number,
                url: link.url,
                error,
            })
        }
    }
}

/// Check if a single link is valid
async fn check_link(client: &Client, url_str: &str) -> Result<(), String> {
    // Parse URL
    let url = match Url::parse(url_str) {
        Ok(u) => u,
        Err(e) => return Err(format!("Invalid URL: {}", e)),
    };

    // Make HTTP request with realistic browser headers to avoid bot detection
    let response = match client
        .get(url.as_str())
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("DNT", "1")
        .header("Connection", "keep-alive")
        .header("Upgrade-Insecure-Requests", "1")
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Request failed: {}", e)),
    };

    // Check status code - accept 2xx success codes and 3xx redirects
    let status = response.status();
    if status.is_success() || status.is_redirection() {
        Ok(())
    } else {
        Err(format!(
            "HTTP {}: {}",
            status.as_u16(),
            status.canonical_reason().unwrap_or("Unknown")
        ))
    }
}
