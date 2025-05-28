# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tombi (鳶) is a feature-rich TOML toolkit written in Rust that provides:
- **Formatter** - Code formatting for TOML files
- **Linter** - Code quality checking for TOML files
- **Language Server** - IDE support with completion, diagnostics, hover, goto definition

## Common Development Commands

### Rust Development
```sh
# Build the entire workspace
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Debug the CLI
cargo tombi

# Run TOML compliance tests
cargo xtask toml-test

# Generate code from grammar definitions
cargo xtask codegen
```

### JavaScript/TypeScript Development
```sh
# Format JS/TS code
pnpm format

# Lint JS/TS code
pnpm lint

# VSCode extension development
pnpm vscode
```

### Testing Specific Functionality
```sh
# Test a specific crate
cargo test -p tombi-parser

# Run tests with output
cargo test -- --nocapture

# Test with specific features
cargo test --features "feature_name"
```

## High-Level Architecture

The codebase follows a layered architecture:

### Core Foundation
- **tombi-text**: Text utilities (positions, ranges, spans)
- **tombi-rg-tree**: Red/Green tree implementation for syntax trees
- **tombi-syntax**: Generated syntax definitions

### Parsing Pipeline
1. **tombi-lexer**: Tokenizes TOML text
2. **tombi-parser**: Event-based parser producing syntax trees
3. **tombi-ast**: Abstract Syntax Tree layer
4. **tombi-document-tree**: High-level semantic TOML representation

### Feature Layers
- **tombi-formatter**: Implements TOML formatting
- **tombi-linter**: Linting rules and diagnostics
- **tombi-validator**: JSON Schema validation
- **tombi-ast-editor**: AST manipulation capabilities

### Language Server
- **tombi-lsp**: Full LSP implementation integrating all features
  - Completions with schema support
  - Hover information
  - Goto definition/declaration
  - Formatting and linting

### Extensions
- **tombi-extension**: Extension framework
- **tombi-extension-cargo**: Cargo.toml specific features
- **tombi-extension-uv**: uv/Python project support

## Development Guidelines

### Code Style
- Write all comments in English
- Don't use mod.rs files - use Rust 2018 style module structure (e.g., `foo.rs` instead of `foo/mod.rs`)

### Testing
- Each crate has its own tests in `tests/` or `src/` directories
- Integration tests for LSP features are in `crates/tombi-lsp/tests/`
- Use `tombi-test-lib` for test utilities

### Debugging VS Code Extension
1. Open VS Code's Run and Debug sidebar
2. Select "Run Extension (Debug Build)"
3. Press the play button

## Key Concepts

### Syntax Trees
The project uses a Red/Green tree architecture where:
- Green nodes are immutable and shared
- Red nodes provide a mutable API over green nodes
- This allows efficient incremental parsing and tree manipulation

### Schema Integration
- JSON schemas are loaded from `tombi-schema-store`
- Schemas drive completion, validation, and hover information
- Custom schemas can be associated with TOML files

### Extension System
Extensions can provide:
- Custom goto definition handlers
- Document link providers
- Schema associations