use clap::ArgMatches;
use crate::generator::SiteGenerator;
use crate::server::DevServer;
use crate::init::init_site;
use crate::content::{create_post, create_page};
use crate::error::{KrikResult, KrikError, ServerError, ServerErrorKind, GenerationError, GenerationErrorKind};
use std::path::PathBuf;

/// Handle the server subcommand
pub async fn handle_server(server_matches: &ArgMatches) -> KrikResult<()> {
    let input_dir = PathBuf::from(server_matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(server_matches.get_one::<String>("output").unwrap());
    let theme_dir = server_matches.get_one::<String>("theme")
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from("themes/default")));
    let port: u16 = server_matches.get_one::<String>("port").unwrap().parse()
        .map_err(|_| KrikError::Server(ServerError {
            kind: ServerErrorKind::BindError { 
                port: 0, 
                source: std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid port number") 
            },
            context: "Parsing port number from command line".to_string(),
        }))?;
    let no_live_reload = server_matches.get_flag("no-live-reload");

    let server = DevServer::new(input_dir, output_dir, theme_dir, port, !no_live_reload);
    server.start().await
        .map_err(|e| match e.downcast::<std::io::Error>() {
            Ok(io_err) => KrikError::Server(ServerError {
                kind: ServerErrorKind::BindError { port, source: *io_err },
                context: format!("Starting development server on port {}", port),
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
    let directory = PathBuf::from(init_matches.get_one::<String>("directory").unwrap());
    let force = init_matches.get_flag("force");
    
    init_site(&directory, force)
}

/// Handle the post subcommand
pub fn handle_post(post_matches: &ArgMatches) -> KrikResult<()> {
    let title = post_matches.get_one::<String>("title").unwrap();
    let filename = post_matches.get_one::<String>("filename");
    let content_dir = PathBuf::from(post_matches.get_one::<String>("content-dir").unwrap());
    
    create_post(&content_dir, title, filename)
}

/// Handle the page subcommand
pub fn handle_page(page_matches: &ArgMatches) -> KrikResult<()> {
    let title = page_matches.get_one::<String>("title").unwrap();
    let filename = page_matches.get_one::<String>("filename");
    let content_dir = PathBuf::from(page_matches.get_one::<String>("content-dir").unwrap());
    
    create_page(&content_dir, title, filename)
}

/// Handle the default generate command
pub fn handle_generate(matches: &ArgMatches) -> KrikResult<()> {
    let input_dir = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let theme_dir = matches.get_one::<String>("theme").map(PathBuf::from);

    println!("Scanning files in: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    let mut generator = SiteGenerator::new(&input_dir, &output_dir, theme_dir.as_ref())
        .map_err(|e| match &e {
            KrikError::Theme(theme_err) => {
                eprintln!("Theme Error: {theme_err}");
                eprintln!("Suggestion: Check that the theme directory exists and contains required templates");
                e
            }
            _ => e,
        })?;
    
    generator.scan_files().map_err(|e| {
        eprintln!("Scan Error: {e}");
        match &e {
            KrikError::Io(_) => {
                eprintln!("Suggestion: Check that the content directory exists and is readable");
            }
            KrikError::Markdown(_) => {
                eprintln!("Suggestion: Fix the markdown or front matter syntax error");
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
    
    println!("Found {} documents", generator.documents.len());

    generator.generate_site()?;
    println!("Site generated successfully!");

    Ok(())
}