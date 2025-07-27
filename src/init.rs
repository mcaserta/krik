use std::fs;
use std::path::Path;
use include_dir::{include_dir, Dir};

// Embed the content and themes directories at compile time
static CONTENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/content");
static THEMES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/themes");

pub fn init_site(target_dir: &Path, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Initializing new Krik site in: {}", target_dir.display());
    
    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
        println!("📁 Created directory: {}", target_dir.display());
    }
    
    // Check if directory is empty (unless force is specified)
    if !force && is_directory_not_empty(target_dir)? {
        return Err(format!(
            "Directory '{}' is not empty. Use --force to overwrite existing files.",
            target_dir.display()
        ).into());
    }
    
    // Extract content directory
    let content_target = target_dir.join("content");
    extract_embedded_dir(&CONTENT_DIR, &content_target, force)?;
    println!("📝 Created content directory with sample posts and pages");
    
    // Extract themes directory  
    let themes_target = target_dir.join("themes");
    extract_embedded_dir(&THEMES_DIR, &themes_target, force)?;
    println!("🎨 Created themes directory with default theme");
    
    println!("\n✅ Site initialized successfully!");
    println!("\n🔧 Next steps:");
    println!("   cd {}", target_dir.display());
    println!("   kk server          # Start development server");
    println!("   kk                 # Generate static site");
    
    Ok(())
}

fn is_directory_not_empty(dir: &Path) -> Result<bool, Box<dyn std::error::Error>> {
    if !dir.exists() {
        return Ok(false);
    }
    
    let entries = fs::read_dir(dir)?;
    Ok(entries.count() > 0)
}

fn extract_embedded_dir(embedded_dir: &Dir, target_path: &Path, force: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Create target directory
    fs::create_dir_all(target_path)?;
    
    // Extract all files in this directory level
    for file in embedded_dir.files() {
        let file_path = target_path.join(file.path().file_name().unwrap());
        
        // Check if file exists and force is not specified
        if file_path.exists() && !force {
            println!("⚠️  Skipping existing file: {}", file_path.display());
            continue;
        }
        
        // Write file contents
        fs::write(&file_path, file.contents())?;
        println!("📄 Created: {}", file_path.strip_prefix(target_path.parent().unwrap_or(target_path)).unwrap_or(&file_path).display());
    }
    
    // Recursively extract subdirectories
    for subdir in embedded_dir.dirs() {
        let subdir_name = subdir.path().file_name().unwrap();
        let subdir_path = target_path.join(subdir_name);
        extract_embedded_dir(subdir, &subdir_path, force)?;
    }
    
    Ok(())
}