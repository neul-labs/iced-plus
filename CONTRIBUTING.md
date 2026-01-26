# Contributing to iced-plus

Thank you for your interest in contributing to iced-plus! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- Git

### Setting Up the Development Environment

1. Clone the repository:
   ```bash
   git clone https://github.com/neul-labs/iced-plus.git
   cd iced-plus
   ```

2. Build the project:
   ```bash
   cargo build --workspace
   ```

3. Run tests:
   ```bash
   cargo test --workspace
   ```

4. Run the kitchen sink example:
   ```bash
   cargo run -p kitchen_sink
   ```

## Development Workflow

### Code Style

We follow Rust's standard style guidelines:

- Run `cargo fmt` before committing
- Run `cargo clippy --workspace --all-features` and address warnings
- Follow the existing code patterns in the codebase

### Linting

The project has strict linting enabled:

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-features -- -D warnings
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests with all features
cargo test --workspace --all-features

# Run a specific test
cargo test -p iced_plus_components test_name
```

### Documentation

- Add doc comments to public items
- Run `cargo doc --workspace --no-deps` to verify documentation builds
- Update user-facing docs in `documentation/` when adding features

## Pull Request Process

1. **Fork and branch**: Create a feature branch from `main`
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes**: Implement your feature or fix

3. **Test**: Ensure all tests pass and add new tests for new functionality

4. **Lint**: Run formatting and clippy checks

5. **Commit**: Write clear, concise commit messages
   ```
   Add Button loading state

   - Add loading prop to Button component
   - Show spinner when loading is true
   - Disable button during loading
   ```

6. **Push and PR**: Push your branch and open a pull request

### PR Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Include screenshots for UI changes
- Keep PRs focused - one feature/fix per PR

## Project Structure

```
iced-plus/
├── crates/
│   ├── iced_plus/           # Umbrella crate
│   ├── iced_plus_tokens/    # Design tokens
│   ├── iced_plus_theme/     # Theme bridge
│   ├── iced_plus_layouts/   # Layout primitives
│   ├── iced_plus_components/# UI components
│   └── iced_plus_platform/  # Platform APIs
├── examples/
│   └── kitchen_sink/        # Demo application
├── docs/                    # Design documents
└── documentation/           # User-facing mkdocs
```

## Adding a New Component

1. Create the component file in `crates/iced_plus_components/src/`
2. Add the module to `lib.rs`
3. Re-export main types in `lib.rs`
4. Add documentation with examples
5. Add to the kitchen sink demo
6. Update user documentation in `documentation/docs/components/`

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system and version
- Steps to reproduce
- Expected vs actual behavior
- Relevant code snippets or error messages

## Questions?

Feel free to open an issue for questions or discussions about potential contributions.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
