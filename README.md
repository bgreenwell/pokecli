<div align="center">

# PokeCLI

<img src="https://upload.wikimedia.org/wikipedia/commons/9/98/International_Pok%C3%A9mon_logo.svg" alt="Pokemon Logo" width="300"/>

**A blazing fast command line Pokemon data explorer** ⚡

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

*Built with Rust for performance and reliability*

</div>

---

> **Note:** This package was developed as part of a live demonstration for course IS 4010 - Application Development with Artificial Intelligence.

## 🚀 Overview

PokeCLI is a high-performance command line interface for querying Pokemon data via the PokeAPI. Whether you're a developer, Pokemon enthusiast, or just curious about Pokemon stats, PokeCLI provides lightning-fast access to comprehensive Pokemon information with multiple output formats and intelligent caching.

## ✨ Features

- 🔍 **Pokemon Queries**: Search Pokemon by name or ID
- 📊 **Multiple Output Formats**: Table (default), JSON, and YAML output
- ⚡ **Smart Caching**: Built-in memory caching for improved performance  
- 🔧 **Extensible**: Designed to support moves, items, and other PokeAPI resources
- 🚀 **Async**: Non-blocking HTTP requests with proper error handling
- 💻 **Cross-platform**: Works on Linux, macOS, and Windows

## 🛠️ Installation

### Prerequisites
- [Rust 1.70+](https://rustup.rs) - Install via rustup

### Quick Start

```bash
# Clone the repository
git clone https://github.com/bgreenwell/pokecli.git
cd pokecli

# Build in release mode for optimal performance
cargo build --release

# Run pokecli
./target/release/pokecli --help
```

### Alternative Installation Methods

```bash
# Install from local source
cargo install --path .

# Then use globally
pokecli --help
```

## 📖 Usage

### Basic Pokemon Queries
```bash
# Query by name
pokecli pokemon pikachu

# Query by ID  
pokecli pokemon 25

# Get detailed information
pokecli pokemon charizard --detailed

# Enable verbose output for debugging
pokecli pokemon bulbasaur --verbose
```

### Output Formats
```bash
# JSON output (perfect for APIs and scripts)
pokecli pokemon squirtle --output json

# YAML output (human-readable structured data)
pokecli pokemon charmander --output yaml

# Table output (default, great for terminal viewing)
pokecli pokemon eevee --output table
```

### Advanced Features
```bash
# Disable caching for fresh data
pokecli pokemon --no-cache pikachu

# Clear local cache
pokecli clear-cache

# Combine options
pokecli pokemon 150 --detailed --output json --verbose
```

### Planned Features
```bash
# Move information (coming soon)
pokecli move thunderbolt

# Item information (coming soon) 
pokecli item pokeball
```

## 📋 Command Reference

### Global Options
| Option | Short | Description |
|--------|--------|-------------|
| `--output` | `-o` | Output format: `table`, `json`, `yaml` |
| `--no-cache` | | Disable caching for fresh API data |
| `--verbose` | `-v` | Enable verbose logging and debugging |
| `--help` | `-h` | Show help information |

### Subcommands
| Command | Description | Status |
|---------|-------------|--------|
| `pokemon <name_or_id>` | Get Pokemon information | ✅ Available |
| `move <name_or_id>` | Get move information | 🚧 Planned |
| `item <name_or_id>` | Get item information | 🚧 Planned |
| `clear-cache` | Clear the local cache | ✅ Available |

#### Pokemon Command Options
| Option | Short | Description |
|--------|--------|-------------|
| `--detailed` | `-d` | Show comprehensive Pokemon details |

## 🏗️ Architecture

PokeCLI follows a clean, modular architecture designed for maintainability and extensibility:

```
src/
├── api/          # 🌐 PokeAPI client and data models
├── cache/        # 💾 Caching implementations  
├── cli/          # ⌨️  Command line interface
├── config/       # ⚙️  Configuration management
├── output/       # 📊 Output formatters and themes
├── error.rs      # ❌ Centralized error handling
├── utils.rs      # 🔧 Shared utility functions
└── main.rs       # 🚀 Application entry point
```

### Key Design Principles
- **Performance First**: Async operations with intelligent caching
- **Type Safety**: Leverages Rust's type system for reliability  
- **Extensible**: Plugin-ready architecture for future features
- **User-Friendly**: Intuitive CLI with helpful error messages

## 🤝 Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help is appreciated.

### Getting Started
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests if applicable
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Setup
```bash
# Clone your fork
git clone https://github.com/yourusername/pokecli.git
cd pokecli

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- pokemon pikachu
```

## 🐛 Issues & Support

Found a bug or have a feature request? Please [open an issue](https://github.com/bgreenwell/pokecli/issues) on GitHub.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Resources

- [PokeAPI Documentation](https://pokeapi.co) - The comprehensive Pokemon data source
- [Rust Programming Language](https://www.rust-lang.org) - Learn Rust
- [GitHub Repository](https://github.com/bgreenwell/pokecli) - Star us on GitHub!

---

<div align="center">

**Made with ❤️ and Rust** 

*Gotta query 'em all!*

</div>
