use krik::cli::KrikCli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = KrikCli::new();
    cli.run().await
}
