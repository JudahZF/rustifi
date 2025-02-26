# CLAUDE.md - Rustifi Development Guidelines

## Build Commands
- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo test test_ip` - Run a specific test
- `cargo clippy` - Run linter checks
- `cargo fmt` - Format code

## Style Guidelines
- **Formatting**: Follow standard Rust 2021 edition style
- **Imports**: Group by external crates first, then internal modules
- **Naming**: Use snake_case for variables/functions, CamelCase for types/structs
- **Error Handling**: Return `Result<T, Box<dyn std::error::Error>>` for external APIs
- **Types**: Use strong typing with structs; avoid primitive obsession
- **Module Structure**: Follow Rust's module system with mod.rs files
- **Documentation**: Document public API with proper doc comments
- **Testing**: Write unit tests for all public functionality

## Project Specifics
- This library wraps the Unifi Controller API
- Respect the existing hierarchy of device/site/controller modules
- Model structs should derive: Debug, Clone, PartialEq when appropriate