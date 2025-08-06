use krik::cli::KrikCli;
use krik::error::KrikError;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = KrikCli::new();
    
    if let Err(e) = cli.run().await {
        // Print user-friendly error message
        eprintln!("Error: {}", e);
        
        // Print additional context for debugging if available
        if let Some(source) = e.source() {
            eprintln!("Caused by: {}", source);
        }
        
        // Set appropriate exit code based on error type
        let exit_code = match &e {
            KrikError::Config(_) => 2,
            KrikError::Io(_) => 3,
            KrikError::Markdown(_) => 4,
            KrikError::Template(_) => 5,
            KrikError::Theme(_) => 6,
            KrikError::Server(_) => 7,
            KrikError::Content(_) => 8,
            KrikError::Generation(_) => 9,
        };
        
        std::process::exit(exit_code);
    }
    
    Ok(())
}
