use crate::error::{IoError, IoErrorKind, KrikError, KrikResult};
use crate::lint::report_generator::LintReport;
use crate::parser::{extract_language_from_filename, parse_markdown_with_frontmatter_for_file};
use chrono::Utc;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::debug;
use walkdir::WalkDir;

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

    // Precompiled regex
    static MD_LINK_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\[[^\]]+\]\(([^)\s]+\.md)(?:#[^)]+)?\)").unwrap());

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
                report
                    .errors
                    .push(format!("{}: failed to read file: {}", path.display(), e));
                continue;
            }
        };

        match parse_markdown_with_frontmatter_for_file(&content, path) {
            Ok((front, _markdown)) => {
                process_file_frontmatter(path, &front, &mut report, content_dir)?;
                track_duplicates(path, &front, content_dir, &mut seen_slugs, &mut seen_titles)?;
            }
            Err(e) => {
                report.errors.push(format!("{e}"));
            }
        }

        // Check for unresolved .md links in markdown body (naive pattern)
        // This is a lightweight check to catch links that likely should be .html
        // Patterns considered: [text](path.md) or [text](../dir/file.md)
        for cap in MD_LINK_REGEX.captures_iter(&content) {
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

    // Check for duplicates
    check_duplicates(&mut report, seen_slugs, seen_titles);

    Ok(report)
}

/// Process frontmatter validation for a single file
fn process_file_frontmatter(
    path: &Path,
    front: &crate::parser::FrontMatter,
    report: &mut LintReport,
    _content_dir: &Path,
) -> KrikResult<()> {
    // filename without extension
    let stem = match path.file_stem() {
        Some(s) => s.to_string_lossy().to_string(),
        None => {
            report.errors.push(format!(
                "{}: invalid filename (missing stem)",
                path.display()
            ));
            return Ok(());
        }
    };

    // Determine base_name and language from filename
    let (base_name, language) = match extract_language_from_filename(&stem) {
        Ok(pair) => pair,
        Err(e) => {
            report.errors.push(format!("{e}"));
            return Ok(());
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
    static SLUG_REGEX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap());
    if !SLUG_REGEX.is_match(&base_name) {
        report.errors.push(format!(
            "{}: invalid slug '{}' (use lowercase letters, numbers, and hyphens)",
            path.display(),
            base_name
        ));
    }

    // Validate layout if present
    if let Some(layout) = front.extra.get("layout").and_then(|v| v.as_str()) {
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
            report
                .warnings
                .push(format!("{}: 'toc' should be a boolean", path.display()));
        }
    }

    // Validate date presence/validity for posts (recommended)
    let is_post = path.to_string_lossy().contains(&format!(
        "{}posts{}",
        std::path::MAIN_SEPARATOR,
        std::path::MAIN_SEPARATOR
    )) || front.extra.get("layout").and_then(|v| v.as_str()) == Some("post");
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
            report
                .errors
                .push(format!("{}: empty 'title' in front matter", path.display()));
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
        "layout",
        "toc",
        "description", // extras commonly used
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
    } else if let Some(layout) = front.extra.get("layout").and_then(|v| v.as_str()) {
        if layout == "post"
            && path.to_string_lossy().contains(&format!(
                "{}pages{}",
                std::path::MAIN_SEPARATOR,
                std::path::MAIN_SEPARATOR
            ))
        {
            report.warnings.push(format!(
                "{}: file appears under pages but layout is 'post'",
                path.display()
            ));
        }
    }

    Ok(())
}

/// Track duplicates for slugs and titles
fn track_duplicates(
    path: &Path,
    front: &crate::parser::FrontMatter,
    content_dir: &Path,
    seen_slugs: &mut HashMap<(String, String, String), Vec<PathBuf>>,
    seen_titles: &mut HashMap<(String, String, String), Vec<PathBuf>>,
) -> KrikResult<()> {
    let stem = path
        .file_stem()
        .ok_or_else(|| {
            KrikError::Io(IoError {
                kind: IoErrorKind::InvalidPath,
                path: path.to_path_buf(),
                context: "Invalid filename (missing stem)".to_string(),
            })
        })?
        .to_string_lossy()
        .to_string();

    let (base_name, language) = extract_language_from_filename(&stem)?;

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
        let title_key = (rel_parent.clone(), norm_title, language.clone());
        seen_titles
            .entry(title_key)
            .or_default()
            .push(path.to_path_buf());
    }

    Ok(())
}

/// Check for duplicate slugs and titles
fn check_duplicates(
    report: &mut LintReport,
    seen_slugs: HashMap<(String, String, String), Vec<PathBuf>>,
    seen_titles: HashMap<(String, String, String), Vec<PathBuf>>,
) {
    // Duplicate detection
    for ((rel_parent, base, lang), paths) in seen_slugs.into_iter() {
        if paths.len() > 1 {
            let list = paths
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ");
            report.errors.push(format!(
                "Duplicate slug '{base}' (lang '{lang}') under '{rel_parent}' in files: {list}"
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
                "Duplicate title '{title}' (lang '{lang}') under '{rel_parent}' in files: {list}"
            ));
        }
    }
}
