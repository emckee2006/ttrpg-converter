# Development Commands - Works with all IDEs (VS Code, RustRover, Eclipse)
# Run with: just <command>

# Set shell to PowerShell on Windows
set shell := ["powershell.exe", "-c"]

# Default recipe shows available commands
default:
    @just --list

# 🚀 Development workflow commands

# Start development mode with live reloading
dev:
    @echo "Starting development mode with live reloading..."
    cargo watch -x "check" -x "test --lib" -x "run --bin ttrpg-cli -- --help"

# Quick check (fastest feedback loop)
check:
    @echo "Running quick check..."
    cargo check --all-features --all-targets

# Run all tests
test:
    @echo "Running all tests..."
    cargo test --all-features --all-targets
    cargo test --doc

# Run tests with coverage (requires cargo-llvm-cov)
test-coverage:
    @echo "Running tests with coverage..."
    cargo llvm-cov --html --output-dir target/coverage
    @echo "Coverage report: target/coverage/index.html"

# 🔍 Code quality commands

# Lint code (clippy + fmt check) - Pre-commit friendly (ignores generated code)
lint:
    @echo "Running clippy on source code..."
    cargo clippy --all-features --lib --bins -- -D warnings
    @echo "Checking formatting..."
    cargo fmt --check

# Lint all targets including tests (may show generated code warnings)
lint-all:
    @echo "Running clippy on all targets..."
    cargo clippy --all-features --all-targets -- -D warnings
    @echo "Checking formatting..."
    cargo fmt --check

# Fix linting and formatting issues
fix:
    @echo "Fixing clippy issues..."
    cargo clippy --all-features --all-targets --fix
    @echo "Formatting code..."
    cargo fmt

# Full code quality check
quality: lint test
    @echo "✅ All quality checks passed!"

# 📚 Documentation commands

# Generate and open documentation
doc:
    @echo "Generating documentation..."
    cargo doc --all-features --no-deps --open

# Build documentation for offline viewing
doc-build:
    @echo "Building documentation..."
    cargo doc --all-features --no-deps
    @echo "Documentation built: target/doc/index.html"

# Check documentation warnings
doc-check:
    @echo "Checking documentation..."
    $env:RUSTDOCFLAGS = "-D warnings"
    cargo doc --all-features --no-deps

# 📊 Performance and analysis commands

# Run benchmarks (requires cargo-criterion)
bench:
    @echo "Running benchmarks..."
    cargo criterion

# Analyze binary size (requires cargo-bloat)
bloat:
    @echo "Analyzing binary size..."
    cargo bloat --release --crates

# Check for unused dependencies (requires cargo-udeps)
unused-deps:
    @echo "Checking for unused dependencies..."
    cargo +nightly udeps

# 🔒 Security and maintenance commands

# Security audit (requires cargo-audit)
audit:
    @echo "Running security audit..."
    cargo audit
    
# Check for outdated dependencies (requires cargo-outdated)
outdated:
    @echo "Checking for outdated dependencies..."
    cargo outdated

# Update dependencies
update:
    @echo "Updating dependencies..."
    cargo update
    @echo "Consider running 'just outdated' to check for major version updates"

# 🧹 Cleanup commands

# Clean all build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue target/coverage
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue target/criterion

# Deep clean including caches
clean-all: clean
    @echo "Cleaning caches..."
    Remove-Item -Recurse -Force -ErrorAction SilentlyContinue ~/.cargo/registry/cache

# 🚢 Build and release commands

# Build release binary
build-release:
    @echo "Building release binary..."
    cargo build --release --all-features

# Build all targets (CLI and GUI)
build-all:
    @echo "Building all targets..."
    cargo build --all-features --bins

# Create release packages for all platforms
package: build-release
    @echo "Creating release packages..."
    .\scripts\package.ps1

# 🔧 Development tools setup

# Install all required development tools
setup-tools:
    @echo "Installing Rust development tools..."
    .\install_dev_tools.ps1

# Verify tool installation
verify-tools:
    @echo "Verifying development tools..."
    @cargo --version
    @rustc --version
    @clippy-driver --version
    @rustfmt --version
    @just --version || echo "❌ just not installed"
    @cargo watch --version || echo "❌ cargo-watch not installed" 
    @cargo audit --version || echo "❌ cargo-audit not installed"

# 🧪 Testing commands

# Run unit tests only
test-unit:
    cargo test --lib --all-features

# Run integration tests only  
test-integration:
    cargo test --test '*' --all-features

# Run tests for specific crate
test-crate CRATE:
    cargo test -p {{CRATE}} --all-features

# Run property-based tests (requires proptest)
test-property:
    @echo "Running property-based tests..."
    cargo test --features proptest

# 🎯 Specific development scenarios

# Test CLI commands
test-cli:
    @echo "Testing CLI functionality..."
    cargo run --bin ttrpg-cli -- --help
    cargo run --bin ttrpg-cli -- --version

# Test GUI (when implemented)
test-gui:
    @echo "Testing GUI functionality..."
    cargo run --bin ttrpg-gui

# Run converter on sample data
test-convert:
    @echo "Testing conversion with sample data..."
    cargo run --bin ttrpg-cli -- convert --input test_data/sample.zip --output temp_output --source roll20 --target foundry-v12

# 📋 Pre-commit checks (run before committing)
pre-commit: lint test doc-check
    @echo "✅ Pre-commit checks completed successfully!"

# Full validation (comprehensive check)
validate: clean quality bench audit
    @echo "🎉 Full validation completed - project is ready!"

# 🆘 Help commands

# Show development workflow
workflow:
    @echo "📋 Typical Development Workflow:"
    @echo ""
    @echo "1. Start development: just dev"
    @echo "2. Make changes in your IDE"
    @echo "3. Quick check: just check"
    @echo "4. Run tests: just test"
    @echo "5. Fix issues: just fix"
    @echo "6. Before commit: just pre-commit"
    @echo "7. Build release: just build-release"
    @echo ""
    @echo "For help: just help"

# Show all available commands with descriptions
help:
    @echo "🛠️  TTRPGConverter Development Commands"
    @echo ""
    @echo "🚀 Development:"
    @echo "   dev           - Live reloading development mode"
    @echo "   check         - Quick compile check"
    @echo "   test          - Run all tests"
    @echo "   quality       - Run lint + test"
    @echo ""
    @echo "🔍 Code Quality:"
    @echo "   lint          - Run clippy + format check"
    @echo "   fix           - Fix lint + formatting issues"
    @echo "   pre-commit    - Full pre-commit validation"
    @echo ""
    @echo "📚 Documentation:"
    @echo "   doc           - Generate and open docs"
    @echo "   doc-check     - Check doc warnings"
    @echo ""
    @echo "🚢 Build & Release:"
    @echo "   build-release - Build optimized binary"
    @echo "   package       - Create release packages"
    @echo ""
    @echo "🔧 Setup:"
    @echo "   setup-tools   - Install development tools"
    @echo "   verify-tools  - Check tool installation"
    @echo ""
    @echo "Use 'just workflow' for typical development flow"
