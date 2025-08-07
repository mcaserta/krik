use clap::{Arg, ArgMatches, Command};
use crate::error::KrikResult;
use crate::logging;

mod commands;

/// CLI configuration and command handling for Krik
pub struct KrikCli {
    matches: ArgMatches,
}

impl KrikCli {
    /// Create a new CLI instance with parsed arguments
    pub fn new() -> Self {
        let matches = Self::build_cli().get_matches();
        Self { matches }
    }

    /// Build the CLI command structure
    fn build_cli() -> Command {
        Command::new("kk")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Krik Static Site Generator")
            .about("A fast static site generator with markdown support")
            .subcommand(Self::build_server_command())
            .subcommand(Self::build_init_command())
            .subcommand(Self::build_post_command())
            .subcommand(Self::build_page_command())
            .subcommand(Self::build_lint_command())
            .arg(Self::input_arg())
            .arg(Self::output_arg())
            .arg(Self::theme_arg())
            .arg(Self::verbose_arg())
    }

    /// Build the server subcommand
    fn build_server_command() -> Command {
        Command::new("server")
            .about("Start development server with live reload")
            .arg(Self::input_arg())
            .arg(Self::output_arg())
            .arg(Self::theme_arg())
            .arg(Self::verbose_arg())
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
            )
    }

    /// Build the init subcommand
    fn build_init_command() -> Command {
        Command::new("init")
            .about("Initialize a new Krik site with default content and theme")
            .arg(Self::verbose_arg())
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
            )
    }

    /// Build the post subcommand
    fn build_post_command() -> Command {
        Command::new("post")
            .about("Create a new blog post")
            .arg(Self::verbose_arg())
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
            )
    }

    /// Build the page subcommand
    fn build_page_command() -> Command {
        Command::new("page")
            .about("Create a new page")
            .arg(Self::verbose_arg())
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
            )
    }

    /// Build the lint subcommand
    fn build_lint_command() -> Command {
        Command::new("lint")
            .about("Validate content front matter, dates, slugs, and language codes")
            .arg(Self::input_arg())
            .arg(Self::verbose_arg())
            .arg(
                Arg::new("strict")
                    .long("strict")
                    .help("Treat warnings as errors (non-zero exit on warnings)")
                    .action(clap::ArgAction::SetTrue),
            )
    }

    /// Create the input directory argument
    fn input_arg() -> Arg {
        Self::create_dir_arg("input", 'i', "Input directory containing markdown files", Some("content"))
    }

    /// Create the output directory argument
    fn output_arg() -> Arg {
        Self::create_dir_arg("output", 'o', "Output directory for generated HTML files", Some("_site"))
    }

    /// Create the theme directory argument
    fn theme_arg() -> Arg {
        Self::create_dir_arg("theme", 't', "Theme directory path", None)
    }

    /// Create the verbose argument
    fn verbose_arg() -> Arg {
        Arg::new("verbose")
            .short('v')
            .long("verbose")
            .help("Enable verbose logging output")
            .action(clap::ArgAction::SetTrue)
    }

    /// Helper method to create directory arguments with consistent structure
    fn create_dir_arg(name: &'static str, short: char, help: &'static str, default: Option<&'static str>) -> Arg {
        let mut arg = Arg::new(name)
            .short(short)
            .long(name)
            .value_name("DIR")
            .help(help);
        
        if let Some(default_value) = default {
            arg = arg.default_value(default_value);
        }
        
        arg
    }

    /// Run the CLI application
    pub async fn run(self) -> KrikResult<()> {
        // Initialize logging based on verbose flag
        let verbose = self.matches.get_flag("verbose");
        logging::init_logging(verbose);

        match self.matches.subcommand() {
            Some(("server", server_matches)) => commands::handle_server(server_matches).await,
            Some(("init", init_matches)) => commands::handle_init(init_matches),
            Some(("post", post_matches)) => commands::handle_post(post_matches),
            Some(("page", page_matches)) => commands::handle_page(page_matches),
            Some(("lint", lint_matches)) => commands::handle_lint(lint_matches),
            _ => commands::handle_generate(&self.matches),
        }
    }
}

impl Default for KrikCli {
    fn default() -> Self {
        Self::new()
    }
}