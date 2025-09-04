use clap::Parser;
use pokecli::{
    cli::{handle_command, Cli},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let output_format = cli.output.unwrap_or_default();
    
    if let Err(e) = handle_command(cli.command, output_format, cli.no_cache, cli.verbose).await {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
    
    Ok(())
}