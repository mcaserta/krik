use clap::ArgMatches;
use crate::generator::SiteGenerator;
use crate::server::DevServer;
use crate::init::init_site;
use crate::content::{create_post, create_page};
use std::path::PathBuf;

/// Handle the server subcommand
pub async fn handle_server(server_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = PathBuf::from(server_matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(server_matches.get_one::<String>("output").unwrap());
    let theme_dir = server_matches.get_one::<String>("theme")
        .map(PathBuf::from)
        .or_else(|| Some(PathBuf::from("themes/default")));
    let port: u16 = server_matches.get_one::<String>("port").unwrap().parse()
        .map_err(|_| "Invalid port number")?;
    let no_live_reload = server_matches.get_flag("no-live-reload");

    let server = DevServer::new(input_dir, output_dir, theme_dir, port, !no_live_reload);
    server.start().await?;
    Ok(())
}

/// Handle the init subcommand
pub fn handle_init(init_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let directory = PathBuf::from(init_matches.get_one::<String>("directory").unwrap());
    let force = init_matches.get_flag("force");
    
    init_site(&directory, force)?;
    Ok(())
}

/// Handle the post subcommand
pub fn handle_post(post_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let title = post_matches.get_one::<String>("title").unwrap();
    let filename = post_matches.get_one::<String>("filename");
    let content_dir = PathBuf::from(post_matches.get_one::<String>("content-dir").unwrap());
    
    create_post(&content_dir, title, filename)?;
    Ok(())
}

/// Handle the page subcommand
pub fn handle_page(page_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let title = page_matches.get_one::<String>("title").unwrap();
    let filename = page_matches.get_one::<String>("filename");
    let content_dir = PathBuf::from(page_matches.get_one::<String>("content-dir").unwrap());
    
    create_page(&content_dir, title, filename)?;
    Ok(())
}

/// Handle the default generate command
pub fn handle_generate(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = PathBuf::from(matches.get_one::<String>("input").unwrap());
    let output_dir = PathBuf::from(matches.get_one::<String>("output").unwrap());
    let theme_dir = matches.get_one::<String>("theme").map(PathBuf::from);

    println!("Scanning files in: {}", input_dir.display());
    println!("Output directory: {}", output_dir.display());

    let mut generator = SiteGenerator::new(&input_dir, &output_dir, theme_dir.as_ref())?;
    
    generator.scan_files()?;
    println!("Found {} documents", generator.documents.len());

    generator.generate_site()?;
    println!("Site generated successfully!");

    Ok(())
}