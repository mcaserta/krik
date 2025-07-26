use clap::{Arg, Command};
use krik::generator::SiteGenerator;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("kk")
        .version("0.1.2")
        .author("Krik Static Site Generator")
        .about("A fast static site generator with markdown support")
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
