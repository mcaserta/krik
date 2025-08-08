use clap::ArgMatches;
use crate::generator::SiteGenerator;
use crate::server::DevServer;
use crate::init::init_site;
use crate::content::{create_post, create_page};
use crate::lint::lint_content;
use crate::error::{KrikResult, KrikError, ServerError, ServerErrorKind, GenerationError, GenerationErrorKind};
use crate::logging;
use std::path::PathBuf;
use tracing::{info, warn, error, debug};
use super::validate::{validate_directory, ensure_directory, normalize_path, validate_theme_dir, parse_port};

/// Handle the server subcommand
pub async fn handle_server(server_matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("server");
    let _enter = _span.enter();
    
    let input_dir = validate_directory(
        server_matches.get_one::<String>("input").map(|s| s.as_str()).unwrap_or("content"),
        "Validating --input directory for server",
    )?;
    let output_dir = ensure_directory(
        server_matches.get_one::<String>("output").map(|s| s.as_str()).unwrap_or("_site"),
        "Ensuring --output directory for server",
    )?;
    let theme_dir = validate_theme_dir(
        server_matches.get_one::<String>("theme").map(|s| s.as_str()),
        "Validating --theme directory for server",
    )?.or_else(|| Some(PathBuf::from("themes/default")));
    let port = parse_port(
        server_matches.get_one::<String>("port").map(|s| s.as_str()).unwrap_or("3000"),
        "Parsing --port value for server",
    )?;
    let no_live_reload = server_matches.get_flag("no-live-reload");

    info!("Starting development server on port {}", port);
    debug!("Input directory: {}", input_dir.display());
    debug!("Output directory: {}", output_dir.display());
    debug!("Theme directory: {:?}", theme_dir.as_ref().map(|p| p.display()));
    debug!("Live reload: {}", !no_live_reload);

    let server = DevServer::new(input_dir, output_dir, theme_dir, port, !no_live_reload);
    server.start().await
        .map_err(|e| match e.downcast::<std::io::Error>() {
            Ok(io_err) => KrikError::Server(ServerError {
                kind: ServerErrorKind::BindError { port, source: *io_err },
                context: format!("Starting development server on port {port}"),
            }),
            Err(other_err) => KrikError::Server(ServerError {
                kind: ServerErrorKind::WebSocketError(other_err.to_string()),
                context: "Starting development server".to_string(),
            }),
        })?;
    Ok(())
}

/// Handle the init subcommand
pub fn handle_init(init_matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("init");
    let _enter = _span.enter();
    
    let directory = normalize_path(
        init_matches.get_one::<String>("directory").map(|s| s.as_str()).unwrap_or("."),
        false,
        "Normalizing target directory for init",
    )?;
    let force = init_matches.get_flag("force");
    
    info!("Initializing new Krik site in: {}", directory.display());
    debug!("Force overwrite: {}", force);
    
    init_site(&directory, force)
}

/// Handle the post subcommand
pub fn handle_post(post_matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("post");
    let _enter = _span.enter();
    
    let title = post_matches
        .get_one::<String>("title")
        .map(|s| s.as_str())
        .unwrap_or("New post");
    let filename = post_matches.get_one::<String>("filename");
    let content_dir = ensure_directory(
        post_matches.get_one::<String>("content-dir").map(|s| s.as_str()).unwrap_or("content"),
        "Ensuring content directory for post",
    )?;
    
    info!("Creating new post: {}", title);
    debug!("Content directory: {}", content_dir.display());
    debug!("Custom filename: {:?}", filename);
    
    create_post(&content_dir, title, filename)
}

/// Handle the page subcommand
pub fn handle_page(page_matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("page");
    let _enter = _span.enter();
    
    let title = page_matches
        .get_one::<String>("title")
        .map(|s| s.as_str())
        .unwrap_or("New page");
    let filename = page_matches.get_one::<String>("filename");
    let content_dir = ensure_directory(
        page_matches.get_one::<String>("content-dir").map(|s| s.as_str()).unwrap_or("content"),
        "Ensuring content directory for page",
    )?;
    
    info!("Creating new page: {}", title);
    debug!("Content directory: {}", content_dir.display());
    debug!("Custom filename: {:?}", filename);
    
    create_page(&content_dir, title, filename)
}

/// Handle the lint subcommand
pub fn handle_lint(lint_matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("lint");
    let _enter = _span.enter();
    
    let input_dir = validate_directory(
        lint_matches.get_one::<String>("input").map(|s| s.as_str()).unwrap_or("content"),
        "Validating --input directory for lint",
    )?;
    let strict = lint_matches.get_flag("strict");
    let _verbose = lint_matches.get_flag("verbose");

    info!("🔎 Linting content in: {}", input_dir.display());
    debug!("Strict mode: {}", strict);
    debug!("Starting content validation...");
    debug!("Verbose logging enabled");

    let report = lint_content(&input_dir)?;

    info!("Scanned {} file(s)", report.files_scanned);
    debug!("Validation completed successfully");

    if !report.warnings.is_empty() {
        warn!("Found {} warning(s):", report.warnings.len());
        for w in &report.warnings {
            warn!("  - {}", w);
        }
    }

    if !report.errors.is_empty() || (strict && !report.warnings.is_empty()) {
        error!("Found {} error(s):", report.errors.len());
        for e in &report.errors {
            error!("  - {}", e);
        }
        if strict && !report.warnings.is_empty() {
            error!("Strict mode: treating {} warning(s) as error(s)", report.warnings.len());
        }
        // Return a content validation error
        return Err(KrikError::Content(crate::error::ContentError {
            kind: crate::error::ContentErrorKind::ValidationFailed({
                let mut msgs = report.errors.clone();
                if strict { msgs.extend(report.warnings.clone()); }
                msgs
            }),
            path: None,
            context: "Content lint failed".to_string(),
        }));
    }

    info!("✅ No lint errors found");
    Ok(())
}

/// Handle the default generate command
pub fn handle_generate(matches: &ArgMatches) -> KrikResult<()> {
    let _span = logging::get_logger("generate");
    let _enter = _span.enter();
    
    let input_dir = validate_directory(
        matches.get_one::<String>("input").map(|s| s.as_str()).unwrap_or("content"),
        "Validating --input directory for generate",
    )?;
    let output_dir = ensure_directory(
        matches.get_one::<String>("output").map(|s| s.as_str()).unwrap_or("_site"),
        "Ensuring --output directory for generate",
    )?;
    let theme_dir = super::validate::validate_theme_dir(
        matches.get_one::<String>("theme").map(|s| s.as_str()),
        "Validating --theme directory for generate",
    )?;

    info!("Scanning files in: {}", input_dir.display());
    info!("Output directory: {}", output_dir.display());
    debug!("Theme directory: {:?}", theme_dir.as_ref().map(|p| p.display()));

    let mut generator = SiteGenerator::new(&input_dir, &output_dir, theme_dir.as_ref())
        .map_err(|e| match &e {
            KrikError::Theme(theme_err) => {
                error!("Theme Error: {theme_err}");
                error!("Suggestion: Check that the theme directory exists and contains required templates");
                e
            }
            _ => e,
        })?;
    
    generator.scan_files().map_err(|e| {
        error!("Scan Error: {e}");
        match &e {
            KrikError::Io(_) => {
                error!("Suggestion: Check that the content directory exists and is readable");
            }
            KrikError::Markdown(_) => {
                error!("Suggestion: Fix the markdown or front matter syntax error");
            }
            _ => {}
        }
        e
    })?;
    
    if generator.documents.is_empty() {
        return Err(KrikError::Generation(GenerationError {
            kind: GenerationErrorKind::NoContent,
            context: format!("No markdown files found in {}", input_dir.display()),
        }));
    }
    
    info!("Found {} documents", generator.documents.len());

    generator.generate_site()?;
    info!("Site generated successfully!");

    Ok(())
}