# Development Environment Setup Guide

This guide provides comprehensive IDE configurations for **VS Code**, **RustRover**, and **Eclipse** to ensure a consistent, productive Rust development experience across all platforms.

## 🎯 Quick Setup

Choose your preferred IDE and follow the corresponding section:

- [VS Code Setup](#vs-code-setup) - Recommended for beginners
- [RustRover Setup](#rustrover-setup) - Best for professional Rust development  
- [Eclipse Setup](#eclipse-setup) - Familiar for Java developers

## 📁 Project Structure

```
ttrpg-converter/
├── ide-templates/          # Template configurations (copy to your IDE)
│   ├── vscode/            # VS Code templates
│   ├── rustrover/         # RustRover templates
│   └── eclipse/           # Eclipse templates
├── .vscode/               # Your personal VS Code config (gitignored)
├── .idea/                 # Your personal RustRover config (gitignored)
├── docs/                  # Project documentation
└── crates/               # Rust workspace crates
```

---

## VS Code Setup

### Prerequisites
```bash
# Install VS Code from https://code.visualstudio.com/
# Install Rust: https://rustup.rs/
```

### Essential Extensions
Install these extensions for optimal Rust development:

```bash
# Required
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates

# Recommended  
code --install-extension tamasfe.even-better-toml
code --install-extension usernamehw.errorlens
code --install-extension streetsidesoftware.code-spell-checker
code --install-extension ms-vscode.vscode-json
code --install-extension redhat.vscode-yaml
```

### Configuration Setup
Copy template configurations:

```bash
# Copy VS Code templates to your workspace
cp ide-templates/vscode/settings.json .vscode/
cp ide-templates/vscode/launch.json .vscode/
cp ide-templates/vscode/tasks.json .vscode/
cp ide-templates/vscode/extensions.json .vscode/
```

---

## RustRover Setup

### Prerequisites
```bash
# Install RustRover from https://www.jetbrains.com/rust/
# Install Rust toolchain: https://rustup.rs/
```

### Initial Configuration

1. **Open Project**: File → Open → Select `ttrpg-converter` directory
2. **Import Settings**: File → Import Settings → Select `ide-templates/rustrover/settings.zip`
3. **Apply Run Configurations**: Copy from `ide-templates/rustrover/runConfigurations/`

### Essential Plugins

Go to **File → Settings → Plugins** and install:

- **Rust** (bundled) - Core Rust language support
- **TOML** (bundled) - Configuration file support  
- **Markdown** - Documentation editing
- **GitToolBox** - Enhanced Git integration
- **String Manipulation** - Text processing utilities
- **Rainbow Brackets** - Code readability
- **Key Promoter X** - Learn shortcuts faster

### Rust-Specific Settings

**Languages & Frameworks → Rust**:
- ✅ Use cargo check instead of cargo build  
- ✅ Run clippy instead of cargo check
- ✅ Show compiler progress in tool window
- ✅ Enable offline mode when network unavailable
- ✅ Show type hints
- ✅ Show parameter hints

**Editor → Code Style → Rust**:
- Tab size: 4
- Indent: 4  
- Continuation indent: 8
- ✅ Use tab character: false

---

## Eclipse Setup

### Prerequisites  
```bash
# Install Eclipse IDE for Java Developers
# Install Corrosion plugin for Rust support
# Install Rust toolchain: https://rustup.rs/
```

### Installing Corrosion Plugin

1. **Help → Eclipse Marketplace**
2. Search for "**Corrosion**"
3. Install **Corrosion: Rust edition in Eclipse IDE**
4. Restart Eclipse

### Project Import

1. **File → Import → General → Existing Projects into Workspace**
2. Select `ttrpg-converter` directory
3. Import as Rust project

### Essential Configuration

**Window → Preferences → Rust**:
- Rust installation path: `~/.cargo/bin` (or your rustup path)
- ✅ Enable cargo check on save
- ✅ Show inlay hints  
- ✅ Enable clippy linting

**Project Properties → Rust Build**:
- Build command: `cargo build`
- Clean command: `cargo clean`  
- Test command: `cargo test`

---

## Development Tools Setup

### Install Essential Rust Tools

Run this script to install all development tools:

```powershell
# Save as install_dev_tools.ps1
Write-Host "Installing Rust development tools..." -ForegroundColor Green

$tools = @(
    "cargo-watch",      # Live reloading during development
    "cargo-edit",       # Add/remove dependencies easily  
    "cargo-audit",      # Security vulnerability scanning
    "cargo-outdated",   # Check for outdated dependencies
    "cargo-tarpaulin",  # Code coverage (Linux/macOS)
    "cargo-llvm-cov",   # Code coverage (cross-platform)
    "cargo-criterion",  # Performance benchmarking
    "cargo-mutants",    # Mutation testing
    "just",             # Modern command runner
    "mdbook",           # Documentation generation
    "cargo-expand",     # Macro expansion debugging
    "cargo-tree",       # Dependency visualization
    "cargo-bloat",      # Binary size analysis
    "cargo-udeps",      # Find unused dependencies
    "bacon",            # Background code checking
    "sccache"           # Compilation caching
)

foreach ($tool in $tools) {
    Write-Host "Installing $tool..." -ForegroundColor Yellow
    cargo install $tool
}

Write-Host "All tools installed successfully!" -ForegroundColor Green
```

### Project-Level Configuration

The following files work with **all IDEs**:

#### Code Quality (`clippy.toml`)
```toml
# Clippy configuration based on Rust best practices
avoid-breaking-exported-api = true
msrv = "1.70"

# Lint levels (following user rules for Rust)
correctness = "deny"
suspicious = "deny"
perf = "warn"  
complexity = "warn"
style = "warn"
pedantic = "warn"
nursery = "warn"
```

#### Code Formatting (`.rustfmt.toml`)
```toml
# Rust formatting configuration
edition = "2021"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Max"
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_imports = true
reorder_modules = true
remove_nested_parens = true
```

#### Command Runner (`justfile`)
```just
# Development commands that work with any IDE

# Start development mode with live reloading
dev:
    cargo watch -x "check" -x "test" -x "run --bin ttrpg-cli"

# Run all tests
test:
    cargo test --all-features --all-targets
    cargo test --doc

# Lint and format check
lint:
    cargo clippy --all-features --all-targets -- -D warnings
    cargo fmt --check

# Fix formatting and linting issues
fix:
    cargo clippy --all-features --all-targets --fix
    cargo fmt

# Generate documentation
doc:
    cargo doc --all-features --no-deps --open

# Run benchmarks
bench:
    cargo criterion

# Check code coverage
coverage:
    @echo "Running code coverage..."
    cargo llvm-cov --html --output-dir target/coverage
    @echo "Coverage report: target/coverage/index.html"

# Security audit
audit:
    cargo audit
    cargo outdated
    
# Clean all build artifacts
clean:
    cargo clean
    rm -rf target/coverage target/criterion

# Build release binary
release:
    cargo build --release --all-features

# Quick development cycle
check:
    bacon
```

---

## IDE Comparison

| Feature | VS Code | RustRover | Eclipse |
|---------|---------|-----------|---------|
| **Ease of Setup** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Rust Integration** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| **Debugging** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Refactoring** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Free/Open Source** | ✅ | ❌ | ✅ |
| **Cross Platform** | ✅ | ✅ | ✅ |

### Recommendations

- **New to Rust**: Start with **VS Code** - easiest setup and great community
- **Professional Development**: Use **RustRover** - best-in-class Rust tooling
- **Java Background**: Try **Eclipse** - familiar interface with Corrosion plugin
- **Team Environment**: Provide templates for all three to support developer preferences

---

## Getting Started

1. **Choose your IDE** from the options above
2. **Install prerequisites** and extensions/plugins
3. **Copy template configurations** to your IDE directory  
4. **Run setup script**: `./install_dev_tools.ps1`
5. **Open the project** and start developing!

The development environment is designed to work consistently across all IDEs, so team members can use their preferred tools while maintaining code quality standards.

---

## Troubleshooting

### Common Issues

**Rust Analyzer not working**:
- Ensure `rust-analyzer` is installed: `rustup component add rust-analyzer`
- Check Rust toolchain: `rustc --version`
- Restart your IDE

**Clippy not running**:
- Install clippy: `rustup component add clippy`
- Check clippy configuration in `clippy.toml`

**Debugging not working**:
- Install LLDB/GDB debugger for your platform
- Check debug configuration in your IDE
- Ensure debug symbols: `cargo build` (not `--release`)

**Performance issues**:
- Enable `sccache` for faster compilation
- Increase IDE memory allocation
- Use `cargo check` instead of `cargo build` during development

Need help? Open an issue with your IDE setup and error details.
