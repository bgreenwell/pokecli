# pokecli 🐾

A fast and user-friendly command line tool for searching, retrieving, and displaying detailed Pokémon data from the PokeAPI. Built for trainers, developers, and enthusiasts who want Pokémon info at their fingertips! ⚡️

---

## Features ✨
- Search for Pokémon by name, type, or ID
- View detailed stats, abilities, and evolutions
- Filter and format output for easy reading
- Fast queries powered by PokeAPI
- Built with Rust for performance and reliability 🦀
- Easy to use CLI with helpful flags and options

---

## Installation 🚀

### Prerequisites
- Rust (https://rustup.rs)

### Build from Source
```sh
# Clone the repo
git clone https://github.com/username/pokecli.git
cd pokecli

# Build the binary
cargo build --release

# Run pokecli
./target/release/pokecli --help
```

---

## Usage 🕹️

```sh
pokecli pikachu
pokecli --type fire
pokecli --id 25
pokecli --help
```

### Example Output
```
Name: Pikachu ⚡️
Type: Electric
Abilities: Static, Lightning Rod
Stats: HP: 35, Attack: 55, Defense: 40, ...
```

---

## CLI Options 🧰

| Option         | Description                       |
| -------------- | --------------------------------- |
| `--name`       | Search Pokémon by name            |
| `--type`       | Filter Pokémon by type            |
| `--id`         | Search Pokémon by ID              |
| `--format`     | Output format (json, table, etc.) |
| `--help`       | Show help message                 |

---

## Contributing 🤝

Pull requests, issues, and suggestions are welcome! Please read our [contributing guidelines](CONTRIBUTING.md) first.

---

## License 📄

MIT License. See [LICENSE](LICENSE) for details.

---

## Links 🔗
- [PokeAPI](https://pokeapi.co)
- [Rust](https://www.rust-lang.org)
- [GitHub Repo](https://github.com/username/pokecli)

---

Made with ❤️ by bgreenwell
