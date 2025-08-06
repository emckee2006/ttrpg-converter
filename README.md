# TTRPGConverter

A modern, high-performance TTRPG campaign converter built in pure Rust.

## Features

- 🚀 **Pure Rust** - Single binary, no external dependencies
- 🎯 **Multi-VTT Support** - Roll20, Foundry VTT, and more
- 🎨 **Native GUI** - Modern interface built with egui  
- ⚡ **High Performance** - Parallel processing with Rayon
- 📱 **Cross-Platform** - Windows, macOS, Linux support
- 🛡️ **Type Safety** - Rust's memory safety and error handling

## Quick Start

\\\ash
# Build all crates
cargo build --workspace

# Run CLI tool
cargo run --bin ttrpg-cli

# Run GUI application
cargo run --bin ttrpg-gui

# Run tests
cargo test --workspace
\\\

## Architecture

- **ttrpg-core**: Domain logic and data structures
- **ttrpg-formats**: File format parsers (JSON, XML, etc.)
- **ttrpg-assets**: Asset processing (images, audio, etc.)
- **ttrpg-gui**: Native GUI using egui
- **ttrpg-cli**: Command-line interface

## Development

This project follows Rust best practices:
- Comprehensive error handling with Result<T>
- Async/await for I/O operations  
- Property-based testing with proptest
- Documentation with cargo doc

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
