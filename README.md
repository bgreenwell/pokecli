# PokeCLI

> **Note:** This package was developed as part of a live demonstration for course IS 4010 - Application Development with Artificial Intelligence.

A fast and user-friendly command line tool for querying Pokemon data via the PokeAPI. Built with Rust for performance and reliability.

## Features

- **Pokemon Queries**: Search Pokemon by name or ID
- **Multiple Output Formats**: Table (default), JSON, and YAML output
- **Caching**: Built-in memory caching for improved performance  
- **Extensible**: Designed to support moves, items, and other PokeAPI resources
- **Async**: Non-blocking HTTP requests with proper error handling

## Installation

### Prerequisites
- Rust 1.70+ (https://rustup.rs)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/bgreenwell/pokecli.git
cd pokecli

# Build the project
cargo build --release

# Run pokecli
./target/release/pokecli --help
```

## Usage

### Basic Pokemon Queries
```bash
# Query by name
pokecli pokemon pikachu

# Query by ID  
pokecli pokemon 25

# Detailed information
pokecli pokemon charizard --detailed

# Verbose output
pokecli pokemon bulbasaur --verbose
```

### Output Formats
```bash
# JSON output
pokecli pokemon squirtle --output json

# YAML output
pokecli pokemon charmander --output yaml

# Table output (default)
pokecli pokemon eevee --output table
```

### Other Commands
```bash
# Move information (not yet implemented)
pokecli move thunderbolt

# Item information (not yet implemented) 
pokecli item pokeball

# Clear cache
pokecli clear-cache
```

## Command Reference

### Global Options
- `--output, -o`: Output format (table, json, yaml)
- `--no-cache`: Disable caching
- `--verbose, -v`: Enable verbose output
- `--help, -h`: Show help information

### Subcommands
- `pokemon <name_or_id>`: Get Pokemon information
  - `--detailed, -d`: Show detailed information
- `move <name_or_id>`: Get move information (planned)
- `item <name_or_id>`: Get item information (planned)
- `clear-cache`: Clear the local cache

## Architecture

The project follows a modular architecture:

```
src/
├── api/          # PokeAPI client and data models
├── cache/        # Caching implementations
├── cli/          # Command line interface
├── config/       # Configuration management
├── output/       # Output formatters and themes
├── error.rs      # Error handling
├── utils.rs      # Utility functions
└── main.rs       # Entry point
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Links

- [PokeAPI](https://pokeapi.co) - The Pokemon data source
- [Rust](https://www.rust-lang.org) - Programming language
- [GitHub Repository](https://github.com/bgreenwell/pokecli)
