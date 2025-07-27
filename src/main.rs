use clap::{Arg, Command};
use krik::generator::SiteGenerator;
use krik::server::DevServer;
use krik::init::init_site;
use krik::content::{create_post, create_page};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("kk")
        .version("0.1.6")
        .author("Krik Static Site Generator")
        .about("A fast static site generator with markdown support")
        .subcommand(
            Command::new("server")
                .about("Start development server with live reload")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("DIR")
                        .help("Input directory containing markdown files")
                        .default_value("content"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("DIR")
                        .help("Output directory for generated HTML files")
                        .default_value("_site"),
                )
                .arg(
                    Arg::new("theme")
                        .short('t')
                        .long("theme")
                        .value_name("DIR")
                        .help("Theme directory path"),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("PORT")
                        .help("Port to bind the server to")
                        .default_value("3000"),
                )
                .arg(
                    Arg::new("no-live-reload")
                        .long("no-live-reload")
                        .help("Disable live reload functionality")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new Krik site with default content and theme")
                .arg(
                    Arg::new("directory")
                        .help("Directory to initialize (default: current directory)")
                        .value_name("DIR")
                        .default_value("."),
                )
                .arg(
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .help("Overwrite existing files")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("post")
                .about("Create a new blog post")
                .arg(
                    Arg::new("title")
                        .help("Post title")
                        .value_name("TITLE")
                        .default_value("New post"),
                )
                .arg(
                    Arg::new("filename")
                        .long("filename")
                        .short('f')
                        .help("Custom filename (without .md extension)")
                        .value_name("NAME"),
                )
                .arg(
                    Arg::new("content-dir")
                        .long("content-dir")
                        .help("Content directory path")
                        .value_name("DIR")
                        .default_value("content"),
                ),
        )
        .subcommand(
            Command::new("page")
                .about("Create a new page")
                .arg(
                    Arg::new("title")
                        .help("Page title")
                        .value_name("TITLE")
                        .default_value("New page"),
                )
                .arg(
                    Arg::new("filename")
                        .long("filename")
                        .short('f')
                        .help("Custom filename (without .md extension)")
                        .value_name("NAME"),
                )
                .arg(
                    Arg::new("content-dir")
                        .long("content-dir")
                        .help("Content directory path")
                        .value_name("DIR")
                        .default_value("content"),
                ),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("DIR")
                .help("Input directory containing markdown files")
                .default_value("content"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("DIR")
                .help("Output directory for generated HTML files")
                .default_value("_site"),
        )
        .arg(
            Arg::new("theme")
                .short('t')
                .long("theme")
                .value_name("DIR")
                .help("Theme directory path"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("server", server_matches)) => {
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
        }
        Some(("init", init_matches)) => {
            let directory = PathBuf::from(init_matches.get_one::<String>("directory").unwrap());
            let force = init_matches.get_flag("force");
            
            init_site(&directory, force)?;
        }
        Some(("post", post_matches)) => {
            let title = post_matches.get_one::<String>("title").unwrap();
            let filename = post_matches.get_one::<String>("filename");
            let content_dir = PathBuf::from(post_matches.get_one::<String>("content-dir").unwrap());
            
            create_post(&content_dir, title, filename)?;
        }
        Some(("page", page_matches)) => {
            let title = page_matches.get_one::<String>("title").unwrap();
            let filename = page_matches.get_one::<String>("filename");
            let content_dir = PathBuf::from(page_matches.get_one::<String>("content-dir").unwrap());
            
            create_page(&content_dir, title, filename)?;
        }
        _ => {
            // Default behavior: generate site once
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
        }
    }

    Ok(())
}
