# Contributing to Lion

First of all, thank you for considering contributing to Lion! It's people like you who make Lion a great tool for developers.

## Getting Started

Before you begin:
- Make sure you have a [GitHub account](https://github.com/signup)
- Familiarize yourself with [Rust](https://www.rust-lang.org/) if you haven't already
- Check out the [existing issues](https://github.com/TeenCoder159/lion/issues) to see if the issue you want to work on already exists

## How to Contribute

### Reporting Bugs

If you've found a bug, please create an issue on the [GitHub repository](https://github.com/TeenCoder159/lion/issues) with the following information:

1. A clear title and description
2. Steps to reproduce the bug
3. Expected behavior
4. Actual behavior
5. Your environment (OS, Rust version, etc.)

### Suggesting Enhancements

We love hearing ideas for new features! To suggest an enhancement:

1. Check if the enhancement has already been suggested or implemented
2. Create an issue on GitHub with a clear title and detailed description
3. Explain why this enhancement would be useful to Lion users

### Pull Requests

1. Fork the repository on GitHub
2. Clone your fork locally
   ```bash
   git clone https://github.com/your-username/lion.git
   cd lion
   ```
3. Create a new branch for your changes
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. Make your changes
5. Run the tests and make sure everything passes
   ```bash
   cargo test
   ```
6. Commit your changes with a descriptive message
   ```bash
   git commit -m "Add feature: your feature description"
   ```
7. Push to your fork
   ```bash
   git push origin feature/your-feature-name
   ```
8. Create a pull request from your branch to the main repository

## Development Process

### Setting Up Your Development Environment

1. Make sure you have Rust and Cargo installed. If not, follow the [official instructions](https://www.rust-lang.org/tools/install)
2. Clone the repository and navigate to the project directory
3. Run `cargo build` to ensure everything compiles correctly

### Code Style

- Follow the standard Rust formatting guidelines
- Run `cargo fmt` before committing to ensure consistent style
- Use `cargo clippy` to catch common mistakes

### Adding New Language Support

If you want to add support for a new programming language:

1. Create a new module under `src/languages/`
2. Implement at least the basic functions: `new`, `run`, and optionally `dep` and `proj`
3. Update the file type detection in `main.rs`
4. Add your language to the supported languages list in the README

### Testing

Please ensure all your code changes have appropriate tests:

1. Run existing tests with `cargo test`
2. Add new tests for new functionality
3. Ensure your changes don't break existing functionality

## Project Structure

- `src/main.rs` - Entry point of the application
- `src/controller.rs` - Contains the main logic for processing commands
- `src/languages/` - Contains modules for each supported language
- `src/utils.rs` - Utility functions used across the project

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/TeenCoder159/lion/tags).

## Code of Conduct

- Be respectful and inclusive
- Focus on the issue, not the person
- Consider different perspectives
- Accept constructive criticism

## Questions?

If you have any questions about contributing, feel free to open an issue labeled "question" in the GitHub repository.

Thank you for contributing to Lion!
