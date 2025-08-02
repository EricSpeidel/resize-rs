# Development and Quality Assurance Commands
# Run with: just <command>
# Install just: cargo install just

# Use PowerShell as the shell
set shell := ["powershell.exe", "-c"]

# Default recipe
default:
    @just --list

# Install development dependencies
install-tools:
    cargo install cargo-audit
    rustup component add clippy rustfmt

# Run all quality checks
check-all: format lint test audit

# Format code
format:
    cargo fmt --all

# Check formatting without making changes
check-format:
    cargo fmt --all -- --check

# Run clippy lints
lint:
    cargo clippy --all-targets --all-features

# Run clippy with pedantic lints
lint-pedantic:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic -W clippy::nursery -D warnings

# Run tests
test:
    cargo test --all-features

# Run tests with coverage (requires cargo-tarpaulin)
test-coverage:
    cargo tarpaulin --all-features --out html

# Check for security vulnerabilities
audit:
    cargo audit

# Check for outdated dependencies
outdated:
    cargo outdated

# Build in release mode
build-release:
    cargo build --release

# Run the application
run:
    cargo run

# Clean build artifacts
clean:
    cargo clean

# Check documentation
doc:
    cargo doc --all-features --no-deps

# Open documentation in browser
doc-open:
    cargo doc --all-features --no-deps --open

# Run benchmarks (if any)
bench:
    cargo bench

# Update dependencies
update:
    cargo update

# Fix automatically fixable issues
fix:
    cargo fix --allow-dirty --allow-staged
    cargo clippy --fix --allow-dirty --allow-staged

# Pre-commit checks (run before committing)
pre-commit: check-format lint test
