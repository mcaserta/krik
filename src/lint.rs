use crate::error::{IoError, IoErrorKind, KrikError, KrikResult};
use crate::parser::{extract_language_from_filename, parse_markdown_with_frontmatter_for_file};
use chrono::Utc;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use tracing::debug;

/// Report produced by the content linter
#[derive(Debug, Default)]
pub struct LintReport {
    pub files_scanned: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl LintReport {
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Lint markdown content in a directory. Returns a report with errors and warnings.
pub fn lint_content(content_dir: &Path) -> KrikResult<LintReport> {
    debug!("Starting content linting in: {}", content_dir.display());
    
    if !content_dir.exists() {
        return Err(KrikError::Io(IoError {
            kind: IoErrorKind::NotFound,
            path: content_dir.to_path_buf(),
            context: "Content directory not found".to_string(),
        }));
    }

    let mut report = LintReport::default();

    // slug pattern: lowercase letters/numbers separated by single hyphens
    let slug_regex: Regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();

    // Track duplicates: (relative_parent_dir, base_name, language) -> Vec<paths>
    let mut seen_slugs: HashMap<(String, String, String), Vec<PathBuf>> = HashMap::new();
    // Track duplicate titles: (relative_parent_dir, normalized_title, language) -> Vec<paths>
    let mut seen_titles: HashMap<(String, String, String), Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(content_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Only lint markdown files
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
            continue;
        }

        // Skip site config if placed under content
        if path.file_name() == Some(std::ffi::OsStr::new("site.toml")) {
            continue;
        }

        debug!("Linting file: {}", path.display());
        report.files_scanned += 1;

        // Read and parse
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                report.errors.push(format!(
                    "{}: failed to read file: {}",
                    path.display(),
                    e
                ));
                continue;
            }
        };

        match parse_markdown_with_frontmatter_for_file(&content, path) {
            Ok((front, _markdown)) => {
                // filename without extension
                let stem = match path.file_stem() {
                    Some(s) => s.to_string_lossy().to_string(),
                    None => {
                        report.errors.push(format!(
                            "{}: invalid filename (missing stem)",
                            path.display()
                        ));
                        continue;
                    }
                };

                // Determine base_name and language from filename
                let (base_name, language) = match extract_language_from_filename(&stem) {
                    Ok(pair) => pair,
                    Err(e) => {
                        report.errors.push(format!("{}", e));
                        continue;
                    }
                };

                // Validate optional frontmatter lang: must match filename language if present
                if let Some(lang_in_front) = front.lang.as_deref() {
                    if lang_in_front != language {
                        report.warnings.push(format!(
                            "{}: front matter lang '{}' does not match filename language '{}'",
                            path.display(),
                            lang_in_front,
                            language
                        ));
                    }
                }

                // Validate slug format
                if !slug_regex.is_match(&base_name) {
                    report.errors.push(format!(
                        "{}: invalid slug '{}' (use lowercase letters, numbers, and hyphens)",
                        path.display(),
                        base_name
                    ));
                }

                // Validate layout if present
                if let Some(layout) = front
                    .extra
                    .get("layout")
                    .and_then(|v| v.as_str())
                {
                    if layout != "post" && layout != "page" {
                        report.warnings.push(format!(
                            "{}: unrecognized layout '{}' (expected 'post' or 'page')",
                            path.display(),
                            layout
                        ));
                    }
                }

                // Validate toc type if present
                if let Some(toc_val) = front.extra.get("toc") {
                    if !toc_val.is_bool() {
                        report.warnings.push(format!(
                            "{}: 'toc' should be a boolean",
                            path.display()
                        ));
                    }
                }

                // Validate date presence/validity for posts (recommended)
                let is_post = path
                    .to_string_lossy()
                    .contains(&format!("{}posts{}", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR))
                    || front
                        .extra
                        .get("layout")
                        .and_then(|v| v.as_str())
                        == Some("post");
                if is_post && front.date.is_none() {
                    report.warnings.push(format!(
                        "{}: missing 'date' in front matter for a post (recommended)",
                        path.display()
                    ));
                }

                // Warn on far-future dates (> 365 days from now)
                if let Some(date) = front.date {
                    let now = Utc::now();
                    if date > now + chrono::Duration::days(365) {
                        report.warnings.push(format!(
                            "{}: 'date' is more than 1 year in the future ({})",
                            path.display(),
                            date
                        ));
                    }
                }

                // Validate title presence
                if let Some(title) = front.title.as_deref() {
                    if title.trim().is_empty() {
                        report.errors.push(format!(
                            "{}: empty 'title' in front matter",
                            path.display()
                        ));
                    }
                } else {
                    report.errors.push(format!(
                        "{}: missing 'title' in front matter",
                        path.display()
                    ));
                }

                // Validate tags (array of non-empty strings)
                if let Some(tags) = &front.tags {
                    for tag in tags {
                        if tag.trim().is_empty() {
                            report.warnings.push(format!(
                                "{}: contains an empty tag in 'tags'",
                                path.display()
                            ));
                        }
                    }
                }

                // Unknown front matter keys (flat extras) — warn if not in known set
                let known_keys = [
                    "layout", "toc", "description", // extras commonly used
                ];
                for key in front.extra.keys() {
                    if !known_keys.contains(&key.as_str()) {
                        // Allow custom keys but warn to document them in theme/README
                        report.warnings.push(format!(
                            "{}: unknown front matter key '{}' (ensure your theme supports it)",
                            path.display(),
                            key
                        ));
                    }
                }

                // Directory vs layout consistency (warn only)
                if is_post {
                    if let Some(layout) = front.extra.get("layout").and_then(|v| v.as_str()) {
                        if layout == "page" {
                            report.warnings.push(format!(
                                "{}: file appears under posts but layout is 'page'",
                                path.display()
                            ));
                        }
                    }
                } else {
                    if let Some(layout) = front.extra.get("layout").and_then(|v| v.as_str()) {
                        if layout == "post" && path.to_string_lossy().contains(&format!("{}pages{}", std::path::MAIN_SEPARATOR, std::path::MAIN_SEPARATOR)) {
                            report.warnings.push(format!(
                                "{}: file appears under pages but layout is 'post'",
                                path.display()
                            ));
                        }
                    }
                }

                // Track duplicates per relative parent dir + base + lang
                let rel_parent = path
                    .strip_prefix(content_dir)
                    .ok()
                    .and_then(|p| p.parent())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "".to_string());
                let key = (rel_parent.clone(), base_name.clone(), language.clone());
                seen_slugs.entry(key).or_default().push(path.to_path_buf());

                // Track titles for duplicates
                if let Some(title) = front.title.as_deref() {
                    let norm_title = title.trim().to_lowercase();
                    let title_key = (
                        rel_parent.clone(),
                        norm_title,
                        language.clone(),
                    );
                    seen_titles
                        .entry(title_key)
                        .or_default()
                        .push(path.to_path_buf());
                }
            }
            Err(e) => {
                report.errors.push(format!("{}", e));
            }
        }

        // Check for unresolved .md links in markdown body (naive pattern)
        // This is a lightweight check to catch links that likely should be .html
        // Patterns considered: [text](path.md) or [text](../dir/file.md)
        let md_link_re = Regex::new(r"\[[^\]]+\]\(([^)\s]+\.md)(?:#[^)]+)?\)").unwrap();
        for cap in md_link_re.captures_iter(&content) {
            let target = &cap[1];
            // Skip absolute URLs
            if target.starts_with("http://") || target.starts_with("https://") {
                continue;
            }
            report.warnings.push(format!(
                "{}: link to markdown file '{}' detected; consider using .html in links",
                path.display(),
                target
            ));
        }
    }

    // Duplicate detection
    for ((rel_parent, base, lang), paths) in seen_slugs.into_iter() {
        if paths.len() > 1 {
            let list = paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            report.errors.push(format!(
                "Duplicate slug '{}' (lang '{}') under '{}' in files: {}",
                base, lang, rel_parent, list
            ));
        }
    }

    // Duplicate title detection (warn)
    for ((rel_parent, title, lang), paths) in seen_titles.into_iter() {
        if paths.len() > 1 {
            let list = paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            report.warnings.push(format!(
                "Duplicate title '{}' (lang '{}') under '{}' in files: {}",
                title, lang, rel_parent, list
            ));
        }
    }

    Ok(report)
}


