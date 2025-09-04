use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "pokecli")]
#[command(about = "A CLI tool for querying Pokemon data")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, global = true)]
    pub output: Option<OutputFormat>,
    
    #[arg(long, global = true)]
    pub no_cache: bool,
    
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get information about a Pokemon
    Pokemon {
        /// Pokemon name or ID
        name_or_id: String,
        
        #[arg(short, long)]
        detailed: bool,
    },
    /// Get information about a move
    Move {
        /// Move name or ID
        name_or_id: String,
    },
    /// Get information about an item
    Item {
        /// Item name or ID
        name_or_id: String,
    },
    /// Clear the local cache
    ClearCache,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Table
    }
}