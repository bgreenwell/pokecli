use thiserror::Error;

#[derive(Error, Debug)]
pub enum PokeCliError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Pokemon '{name}' not found")]
    PokemonNotFound { name: String },
    
    #[error("Invalid Pokemon name or ID: {input}")]
    InvalidInput { input: String },
    
    #[error("Cache error: {0}")]
    Cache(String),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, PokeCliError>;