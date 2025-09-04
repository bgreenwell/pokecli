# PokeCLI: Rust PokeAPI Command Line Tool

## Project Overview

**pokecli** is a command line interface tool written in Rust that provides easy access to Pokemon data through the PokeAPI (https://pokeapi.co/). The tool allows users to query Pokemon information, moves, items, and other game data directly from the terminal.

## Core Features

### MVP (Minimum Viable Product)
- Query Pokemon by name or ID
- Display basic Pokemon information (name, types, stats, abilities)
- Error handling for invalid Pokemon names/IDs
- Formatted console output

### Extended Features
- Query moves, items, berries, and other PokeAPI resources
- Caching for improved performance
- Configuration file support
- Multiple output formats (JSON, YAML, table)
- Fuzzy search for Pokemon names
- Batch operations
- Offline mode with cached data

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        pokecli                              │
├─────────────────────────────────────────────────────────────┤
│  CLI Interface (clap)                                       │
├─────────────────────────────────────────────────────────────┤
│  Command Handlers                                           │
├─────────────────────────────────────────────────────────────┤
│  API Client (reqwest) │  Cache Layer    │  Output Formatter │
├─────────────────────────────────────────────────────────────┤
│  Data Models (serde)                                        │
├─────────────────────────────────────────────────────────────┤
│  Error Handling                                             │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
pokecli/
├── Cargo.toml
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs              # Entry point and CLI setup
│   ├── lib.rs               # Library exports
│   ├── cli/
│   │   ├── mod.rs           # CLI module exports
│   │   ├── args.rs          # Argument parsing structures
│   │   └── commands.rs      # Command implementations
│   ├── api/
│   │   ├── mod.rs           # API module exports
│   │   ├── client.rs        # HTTP client implementation
│   │   ├── endpoints.rs     # API endpoint definitions
│   │   └── models.rs        # Data models and deserialization
│   ├── cache/
│   │   ├── mod.rs           # Cache module exports
│   │   ├── memory.rs        # In-memory cache
│   │   └── disk.rs          # Disk-based cache (optional)
│   ├── output/
│   │   ├── mod.rs           # Output module exports
│   │   ├── formatters.rs    # Various output formatters
│   │   └── themes.rs        # Color and styling themes
│   ├── config/
│   │   ├── mod.rs           # Configuration module
│   │   └── settings.rs      # Configuration structures
│   ├── error.rs             # Custom error types
│   └── utils.rs             # Utility functions
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   └── api_tests.rs
│   └── fixtures/
│       └── sample_responses.json
├── docs/
│   ├── usage.md
│   └── api_reference.md
└── examples/
    ├── basic_usage.rs
    └── batch_operations.rs
```

## Dependencies (Cargo.toml)

```toml
[package]
name = "pokecli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <email@example.com>"]
description = "A command line tool for querying Pokemon data via PokeAPI"
repository = "https://github.com/username/pokecli"
license = "MIT"
keywords = ["pokemon", "api", "cli", "pokeapi"]
categories = ["command-line-utilities"]

[[bin]]
name = "pokecli"
path = "src/main.rs"

[dependencies]
# CLI framework
clap = { version = "4.0", features = ["derive"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Async runtime
tokio = { version = "1.0", features = ["full"] }

# JSON serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Output formatting
colored = "2.0"
tabled = "0.14"

# Configuration
config = "0.13"
dirs = "5.0"

# Caching (optional)
sled = { version = "0.34", optional = true }

# Fuzzy search
fuzzy-matcher = { version = "0.3", optional = true }

[dev-dependencies]
wiremock = "0.5"
tempfile = "3.0"
assert_cmd = "2.0"
predicates = "3.0"

[features]
default = ["cache", "fuzzy"]
cache = ["sled"]
fuzzy = ["fuzzy-matcher"]
```

## Core Components

### 1. CLI Interface (src/cli/)

**args.rs** - Command line argument definitions:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pokecli")]
#[command(about = "A CLI tool for querying Pokemon data")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, global = true)]
    pub output: Option<OutputFormat>,
    
    #[arg(long, global = true)]
    pub no_cache: bool,
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

#[derive(Clone, Debug)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}
```

### 2. API Client (src/api/)

**client.rs** - HTTP client implementation:
```rust
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::time::Duration;

pub struct PokeApiClient {
    client: Client,
    base_url: String,
    cache: Option<Box<dyn Cache>>,
}

impl PokeApiClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("pokecli/0.1.0")
            .build()?;
            
        Ok(Self {
            client,
            base_url: "https://pokeapi.co/api/v2".to_string(),
            cache: None,
        })
    }
    
    pub async fn get_pokemon(&self, name_or_id: &str) -> Result<Pokemon> {
        self.get(&format!("/pokemon/{}", name_or_id)).await
    }
    
    pub async fn get_move(&self, name_or_id: &str) -> Result<Move> {
        self.get(&format!("/move/{}", name_or_id)).await
    }
    
    async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        // Implementation with caching logic
    }
}
```

**models.rs** - Data structures for API responses:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub base_experience: Option<u32>,
    pub types: Vec<PokemonType>,
    pub abilities: Vec<PokemonAbility>,
    pub stats: Vec<PokemonStat>,
    pub sprites: PokemonSprites,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub type_info: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub effort: u32,
    pub stat: NamedApiResource,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

// Additional models for moves, items, etc.
```

### 3. Output Formatting (src/output/)

**formatters.rs** - Different output format implementations:
```rust
use crate::api::models::Pokemon;
use colored::*;
use tabled::{Table, Tabled};

pub trait Formatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> String;
    fn format_error(&self, error: &str) -> String;
}

pub struct TableFormatter {
    colored: bool,
}

impl Formatter for TableFormatter {
    fn format_pokemon(&self, pokemon: &Pokemon) -> String {
        let mut output = String::new();
        
        // Header
        output.push_str(&format!(
            "{} (#{}) - {}\n", 
            pokemon.name.bright_cyan(),
            pokemon.id.to_string().yellow(),
            pokemon.types.iter()
                .map(|t| t.type_info.name.clone())
                .collect::<Vec<_>>()
                .join("/").green()
        ));
        
        // Stats table
        let stats_data: Vec<StatRow> = pokemon.stats.iter()
            .map(|s| StatRow {
                name: s.stat.name.clone(),
                value: s.base_stat,
            })
            .collect();
            
        output.push_str(&Table::new(stats_data).to_string());
        output
    }
}

#[derive(Tabled)]
struct StatRow {
    name: String,
    value: u32,
}
```

### 4. Error Handling (src/error.rs)

```rust
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
}

pub type Result<T> = std::result::Result<T, PokeCliError>;
```

### 5. Caching System (src/cache/)

```rust
use serde::{Serialize, de::DeserializeOwned};
use std::time::{Duration, SystemTime};

pub trait Cache: Send + Sync {
    fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
    fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()>;
    fn clear(&self) -> Result<()>;
}

pub struct MemoryCache {
    // Implementation using HashMap with expiration times
}

#[cfg(feature = "cache")]
pub struct DiskCache {
    // Implementation using sled database
}
```

## Command Examples

```bash
# Basic Pokemon query
pokecli pokemon pikachu
pokecli pokemon 25

# Detailed information
pokecli pokemon charizard --detailed

# Move information
pokecli move thunderbolt

# Item information
pokecli item pokeball

# Different output formats
pokecli pokemon bulbasaur --output json
pokecli pokemon squirtle --output yaml

# Clear cache
pokecli clear-cache
```

## Configuration

**~/.config/pokecli/config.toml**:
```toml
[api]
base_url = "https://pokeapi.co/api/v2"
timeout_seconds = 30
rate_limit_per_second = 10

[cache]
enabled = true
ttl_hours = 24
max_size_mb = 100

[output]
default_format = "table"
colored = true
show_sprites = false

[fuzzy]
enabled = true
threshold = 0.6
```

## Testing Strategy

### Unit Tests
- Model deserialization tests
- Formatter output tests
- Cache functionality tests
- Error handling tests

### Integration Tests
- Full command execution tests using `assert_cmd`
- Mock API responses using `wiremock`
- Configuration loading tests

### Example Integration Test:
```rust
#[tokio::test]
async fn test_pokemon_command() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("GET"))
        .and(path("/api/v2/pokemon/pikachu"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(&sample_pokemon_response()))
        .mount(&mock_server)
        .await;
    
    let mut cmd = Command::cargo_bin("pokecli").unwrap();
    cmd.arg("pokemon")
       .arg("pikachu")
       .env("POKEAPI_BASE_URL", mock_server.uri());
       
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Pikachu"));
}
```

## Performance Considerations

1. **Caching**: Implement both memory and disk caching to reduce API calls
2. **Rate Limiting**: Respect PokeAPI rate limits (though they're generous)
3. **Async**: Use tokio for non-blocking HTTP requests
4. **Batch Operations**: Allow querying multiple Pokemon at once
5. **Connection Pooling**: Reuse HTTP connections via reqwest

## Error Handling Strategy

1. **Graceful Degradation**: Show partial information when possible
2. **User-Friendly Messages**: Convert technical errors to readable messages
3. **Suggestions**: Provide suggestions for typos (fuzzy matching)
4. **Retry Logic**: Implement exponential backoff for transient failures

## Distribution

### Release Strategy
- GitHub releases with pre-built binaries for major platforms
- Cargo crates.io publication
- Package managers (brew, scoop, etc.)

### Cross-compilation Targets
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin
- x86_64-pc-windows-msvc

## Future Enhancements

1. **Interactive Mode**: TUI interface using `ratatui`
2. **Pokemon Team Builder**: Save and manage Pokemon teams
3. **Battle Simulator**: Simple battle calculations
4. **Sprite Display**: Show Pokemon sprites in terminal
5. **Plugin System**: Allow custom commands and formatters
6. **Shell Completions**: Generate completions for bash, zsh, fish
7. **Localization**: Support multiple languages
8. **Export Features**: Export data to CSV, PDF, etc.

## Development Workflow

1. **Setup**: Clone repo and run `cargo build`
2. **Testing**: `cargo test` for unit tests, `cargo test --test integration` for integration tests
3. **Formatting**: Use `cargo fmt` and `cargo clippy`
4. **Documentation**: `cargo doc --open`
5. **Release**: Use `cargo-release` for version management

This architecture provides a solid foundation for a professional-grade Pokemon CLI tool that's extensible, well-tested, and user-friendly.