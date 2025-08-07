# TTRPGConverter Development Tools Installation Script
# This script installs all essential Rust development tools for optimal productivity

param(
    [switch]$SkipOptional,  # Skip optional/experimental tools
    [switch]$Verbose        # Show detailed output
)

Write-Host "🛠️  Installing TTRPGConverter Development Tools" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# Check if Rust is installed
try {
    $rustVersion = cargo --version
    Write-Host "✅ Rust found: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust not found. Please install Rust first: https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Essential tools (required for development)
$essentialTools = @(
    @{name="cargo-watch"; desc="Live reloading during development"},
    @{name="cargo-edit"; desc="Add/remove dependencies from command line"},
    @{name="just"; desc="Modern command runner (replaces make)"},
    @{name="cargo-audit"; desc="Security vulnerability scanning"},
    @{name="cargo-outdated"; desc="Check for outdated dependencies"}
)

# Quality tools (recommended for code quality)
$qualityTools = @(
    @{name="cargo-llvm-cov"; desc="Cross-platform code coverage"},
    @{name="cargo-criterion"; desc="Performance benchmarking"},
    @{name="cargo-expand"; desc="Macro expansion for debugging"},
    @{name="cargo-tree"; desc="Dependency tree visualization"},
    @{name="bacon"; desc="Background code checking"}
)

# Optional tools (nice to have but not critical)
$optionalTools = @(
    @{name="cargo-bloat"; desc="Binary size analysis"},
    @{name="cargo-udeps"; desc="Find unused dependencies"},
    @{name="cargo-mutants"; desc="Mutation testing"},
    @{name="mdbook"; desc="Documentation generation"},
    @{name="sccache"; desc="Compilation caching for faster builds"},
    @{name="cargo-nextest"; desc="Next-generation test runner"},
    @{name="cargo-machete"; desc="Remove unused dependencies"},
    @{name="cargo-deny"; desc="Cargo plugin for linting dependencies"}
)

function Install-Tool($toolInfo) {
    $name = $toolInfo.name
    $desc = $toolInfo.desc
    
    Write-Host "Installing $name..." -ForegroundColor Yellow -NoNewline
    
    if ($Verbose) {
        Write-Host ""
        Write-Host "  Description: $desc" -ForegroundColor Gray
    }
    
    try {
        if ($Verbose) {
            cargo install $name
        } else {
            cargo install $name 2>$null
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host " ✅" -ForegroundColor Green
        } else {
            Write-Host " ⚠️  (may already be installed)" -ForegroundColor Yellow
        }
    } catch {
        Write-Host " ❌ Failed" -ForegroundColor Red
        if ($Verbose) {
            Write-Host "  Error: $_" -ForegroundColor Red
        }
    }
}

function Install-ToolCategory($tools, $categoryName) {
    Write-Host ""
    Write-Host "📦 Installing $categoryName..." -ForegroundColor Magenta
    Write-Host ("-" * 40) -ForegroundColor Gray
    
    foreach ($tool in $tools) {
        Install-Tool $tool
    }
}

# Install essential tools
Install-ToolCategory $essentialTools "Essential Tools"

# Install quality tools  
Install-ToolCategory $qualityTools "Code Quality Tools"

# Install optional tools (unless skipped)
if (-not $SkipOptional) {
    Install-ToolCategory $optionalTools "Optional Tools"
} else {
    Write-Host ""
    Write-Host "⏭️  Skipping optional tools (use -SkipOptional:$false to include)" -ForegroundColor Yellow
}

# Install Rust components if not already present
Write-Host ""
Write-Host "🦀 Installing Rust Components..." -ForegroundColor Magenta
Write-Host ("-" * 40) -ForegroundColor Gray

$components = @("clippy", "rustfmt", "rust-analyzer")
foreach ($component in $components) {
    Write-Host "Installing $component..." -ForegroundColor Yellow -NoNewline
    try {
        rustup component add $component 2>$null
        Write-Host " ✅" -ForegroundColor Green
    } catch {
        Write-Host " ⚠️  (may already be installed)" -ForegroundColor Yellow
    }
}

# Verify installations
Write-Host ""
Write-Host "🔍 Verifying Tool Installation..." -ForegroundColor Magenta
Write-Host ("-" * 40) -ForegroundColor Gray

function Test-Command($command) {
    try {
        & $command --version 2>$null | Out-Null
        return $true
    } catch {
        return $false
    }
}

$verificationTools = @(
    "cargo", "rustc", "rustfmt", "clippy-driver", 
    "just", "cargo-watch", "cargo-audit", "bacon"
)

foreach ($tool in $verificationTools) {
    Write-Host "Checking $tool..." -ForegroundColor Yellow -NoNewline
    if (Test-Command $tool) {
        Write-Host " ✅" -ForegroundColor Green
    } else {
        Write-Host " ❌ Not found" -ForegroundColor Red
    }
}

# Setup pre-commit hooks (if git repository)
if (Test-Path ".git") {
    Write-Host ""
    Write-Host "🪝 Setting up Git hooks..." -ForegroundColor Magenta
    Write-Host ("-" * 40) -ForegroundColor Gray
    
    # Create pre-commit hook
    $preCommitHook = @'
#!/bin/sh
# Pre-commit hook for TTRPGConverter
echo "Running pre-commit checks..."

# Run formatting check
if ! cargo fmt --check; then
    echo "❌ Code formatting failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
if ! cargo clippy --all-features --all-targets -- -D warnings; then
    echo "❌ Clippy checks failed. Fix the warnings above."
    exit 1
fi

# Run tests
if ! cargo test --all-features; then
    echo "❌ Tests failed. Fix the failing tests."
    exit 1
fi

echo "✅ Pre-commit checks passed!"
'@
    
    $hookPath = ".git/hooks/pre-commit"
    $preCommitHook | Out-File -FilePath $hookPath -Encoding UTF8
    
    # Make executable (Windows doesn't need chmod but we'll note it)
    Write-Host "Pre-commit hook installed at $hookPath" -ForegroundColor Green
    Write-Host "  Note: Hook will run before each commit" -ForegroundColor Gray
}

# Display summary
Write-Host ""
Write-Host "🎉 Installation Complete!" -ForegroundColor Green
Write-Host "========================" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Run 'just --list' to see available commands" -ForegroundColor White
Write-Host "2. Run 'just dev' to start development mode" -ForegroundColor White  
Write-Host "3. Run 'just help' for development workflow guide" -ForegroundColor White
Write-Host "4. Choose your IDE and follow DEVELOPMENT_ENVIRONMENT_SETUP.md" -ForegroundColor White
Write-Host ""
Write-Host "Useful commands:" -ForegroundColor Cyan
Write-Host "  just dev       - Start live development mode" -ForegroundColor White
Write-Host "  just test      - Run all tests" -ForegroundColor White
Write-Host "  just lint      - Check code quality" -ForegroundColor White
Write-Host "  just fix       - Fix formatting and linting" -ForegroundColor White
Write-Host "  just pre-commit - Full pre-commit validation" -ForegroundColor White
Write-Host ""

if ($SkipOptional) {
    Write-Host "💡 Tip: Run again without -SkipOptional to install additional tools" -ForegroundColor Yellow
}

Write-Host "Happy coding! 🚀" -ForegroundColor Green
