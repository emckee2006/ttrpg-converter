#!/usr/bin/env pwsh
# Comprehensive GitHub Planning Setup Script
# Converts detailed milestone planning to GitHub Issues and Projects

param(
    [string]$RepoOwner = "emckee2006",
    [string]$RepoName = "ttrpg-converter"
)

$ErrorActionPreference = "Stop"

Write-Host "🎯 Setting up comprehensive GitHub planning for TTRPGConverter..." -ForegroundColor Green
Write-Host "Repository: $RepoOwner/$RepoName" -ForegroundColor Cyan

# Verify GitHub CLI
try {
    $authStatus = gh auth status 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ GitHub CLI not authenticated. Run: gh auth login" -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ GitHub CLI authenticated" -ForegroundColor Green
} catch {
    Write-Host "❌ GitHub CLI not found. Install GitHub CLI first." -ForegroundColor Red
    exit 1
}

# Step 1: Create Strategic Milestones
Write-Host "📊 Creating Strategic Milestones..." -ForegroundColor Cyan

$milestones = @(
    @{
        title = "M1: Foundation (Week 1-2)"
        description = @"
🎯 **FOUNDATION SETUP** - Project workspace, error handling, core types

**Duration**: 2 weeks | **Points**: 25 | **Priority**: 🚨 CRITICAL PATH

**Key Deliverables**:
- ✅ Clean Rust workspace with 5 crates
- ✅ Comprehensive error handling with thiserror  
- ✅ Core type system and traits
- ✅ Cross-platform build system
- ✅ CLI argument parsing foundation

**Success Criteria**:
- All crates build successfully
- Zero clippy warnings
- Documentation generates  
- Cross-platform Git configuration
- Ready for core development
"@
        due_date = (Get-Date).AddDays(14).ToString("yyyy-MM-dd")
    },
    @{
        title = "M2: Core Engine (Week 3-5)"  
        description = @"
🔧 **CONVERSION ENGINE** - Roll20 parser, entity processing, Foundry output

**Duration**: 3 weeks | **Points**: 35 | **Priority**: 🔥 HIGH

**Key Deliverables**:
- 📄 Roll20 JSON parser with complete entity support
- ⚡ Parallel entity conversion pipeline with rayon
- 🏗️ Foundry VTT output generation (v10, v11, v12)
- 🗂️ Asset processing and download system
- ✅ Comprehensive validation framework

**Performance Targets**:
- Parse 50MB campaign in <2 seconds
- 5-10x faster than Python version
- Memory-efficient streaming for large files
"@
        due_date = (Get-Date).AddDays(35).ToString("yyyy-MM-dd")
    },
    @{
        title = "M3: CLI Interface (Week 6-7)"
        description = @"
⚡ **COMMAND-LINE INTERFACE** - Full-featured CLI with modern UX

**Duration**: 2 weeks | **Points**: 20 | **Priority**: 🔥 HIGH

**Key Deliverables**:
- 🖥️ Complete CLI with convert, validate, info commands
- 📊 Progress bars and colored output
- 🔍 Interactive prompts and confirmations  
- ⚙️ TOML configuration system
- 📋 Comprehensive help and examples

**CLI Features**:
- Drag-and-drop file handling
- Real-time progress tracking
- Detailed error reporting
- Batch processing support
"@
        due_date = (Get-Date).AddDays(49).ToString("yyyy-MM-dd")
    },
    @{
        title = "M4: Native GUI (Week 8-10)"
        description = @"
🎨 **NATIVE GUI** - Modern egui interface replacing Python GUI

**Duration**: 3 weeks | **Points**: 25 | **Priority**: 🟡 MEDIUM

**Key Deliverables**:
- 🖼️ Native egui GUI with modern UX
- 🌙 Dark/light theme support
- 📁 Drag-and-drop file handling
- 🔄 Real-time conversion progress
- ⚙️ Settings and configuration panel

**GUI Features**:
- Campaign preview with statistics
- Asset management interface
- Error handling with suggestions
- Responsive layout system
"@
        due_date = (Get-Date).AddDays(70).ToString("yyyy-MM-dd")
    },
    @{
        title = "M5: Advanced Features (Week 11-14)"
        description = @"
🚀 **ADVANCED FEATURES** - Multi-format output and performance optimization

**Duration**: 4 weeks | **Points**: 30 | **Priority**: 🟢 LOW

**Key Deliverables**:
- 📄 PDF export for campaign books
- 🌐 HTML/web export format
- 🔄 Batch processing system
- ⚡ Performance optimizations
- 🧪 Property-based testing suite

**Output Formats**:
- PDF campaign books with layouts
- Standalone HTML campaign browser
- JSON export for other tools
- Print-friendly formats
"@
        due_date = (Get-Date).AddDays(98).ToString("yyyy-MM-dd")
    }
)

foreach ($milestone in $milestones) {
    Write-Host "  Creating: $($milestone.title)" -ForegroundColor Yellow
    try {
        gh api repos/$RepoOwner/$RepoName/milestones -f title="$($milestone.title)" -f description="$($milestone.description)" -f due_on="$($milestone.due_date)T23:59:59Z" | Out-Null
        Write-Host "  ✅ Created: $($milestone.title)" -ForegroundColor Green
    } catch {
        Write-Host "  ⚠️  Milestone may exist: $($milestone.title)" -ForegroundColor Yellow
    }
}

# Step 2: Create Comprehensive M1 Foundation Issues
Write-Host "📝 Creating M1 Foundation Issues..." -ForegroundColor Cyan

$m1Issues = @(
    @{
        title = "[M1.T1] Project Workspace Setup - Junior Dev Ready"
        body = @"
## 🎯 **Objective**
Create clean Rust workspace with 5 crates and cross-platform configuration.

**Duration**: 1 day | **Points**: 3 | **Priority**: 🚨 CRITICAL (BLOCKS EVERYTHING)

## ✅ **Acceptance Criteria**
- [ ] Workspace builds successfully: `cargo check --workspace`
- [ ] All 5 crates have proper structure and basic tests
- [ ] Clippy passes with zero warnings: `cargo clippy --workspace -- -D warnings`
- [ ] Documentation generates: `cargo doc --workspace --no-deps`
- [ ] Cross-platform Git configuration (.gitignore, .gitattributes)
- [ ] README.md with project overview

## 📋 **Detailed Implementation Steps**

### Step 1: Create Project Structure
``````bash
cd C:\Users\alithanna.mckee\Documents
mkdir ttrpg-converter-dev && cd ttrpg-converter-dev
git init && git branch -m main
``````

### Step 2: Create Workspace Cargo.toml
Create workspace with exact dependencies:
- **Core**: serde, tokio, anyhow, thiserror, tracing
- **CLI**: clap, indicatif, colored, dialoguer  
- **Performance**: rayon, dashmap
- **Assets**: image, zip, reqwest

### Step 3: Create 5 Crate Structure
``````bash
# Create all crates with proper Cargo.toml and lib.rs
# - ttrpg-core: Core types and error handling
# - ttrpg-cli: Command line interface
# - ttrpg-formats: File format parsers
# - ttrpg-assets: Asset processing
# - ttrpg-gui: Native GUI (Phase 2)
``````

### Step 4: Cross-Platform Configuration
- `.gitignore`: Rust artifacts, IDE files, OS files
- `.gitattributes`: LF line endings for all text files
- README.md: Project overview and quick start

## 🧪 **Testing Commands**
``````bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo doc --workspace --no-deps
``````

## 🔗 **Dependencies**: None (UNBLOCKS EVERYTHING ELSE)
## 🚧 **Blocks**: All other M1 tasks
"@
        labels = @("M1: Foundation", "critical", "workspace", "junior-dev-ready")
        milestone = "M1: Foundation (Week 1-2)"
    },
    @{
        title = "[M1.T2] Core Error Handling System"  
        body = @"
## 🎯 **Objective**
Implement comprehensive error handling with thiserror derive macros and proper error chains.

**Duration**: 2 days | **Points**: 5 | **Priority**: 🚨 CRITICAL

## ✅ **Acceptance Criteria** 
- [ ] `ConversionError` enum with all error types (IO, JSON, Validation, Asset, etc.)
- [ ] `ConversionResult<T>` type alias for convenience
- [ ] `ErrorContext` trait for adding context to errors
- [ ] All error types have proper Display and Debug implementations
- [ ] Comprehensive tests for error handling and context chaining
- [ ] Documentation with examples for all error types

## 📋 **Detailed Implementation Steps**

### Step 1: Update ttrpg-core Dependencies
Add to `crates/ttrpg-core/Cargo.toml`:
``````toml
[dependencies]
serde = { workspace = true }
thiserror = { workspace = true }
anyhow = { workspace = true }
``````

### Step 2: Create src/error.rs
``````rust
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("IO error while {operation}: {message}")]
    Io { operation: String, message: String, #[source] source: std::io::Error },
    
    #[error("JSON parsing failed for {context}: {message}")]
    Json { context: String, message: String, #[source] source: serde_json::Error },
    
    #[error("Validation failed for {entity_type} '{entity_id}': {reason}")]
    Validation { entity_type: String, entity_id: String, reason: String },
    
    // ... other error types
}

pub type ConversionResult<T> = Result<T, ConversionError>;
``````

### Step 3: Implement ErrorContext Trait
``````rust
pub trait ErrorContext<T> {
    fn with_context<F>(self, f: F) -> ConversionResult<T> where F: FnOnce() -> String;
    fn with_entity_context(self, entity_type: &str, entity_id: &str) -> ConversionResult<T>;
    fn with_file_context(self, path: &Path) -> ConversionResult<T>;
}
``````

### Step 4: Add Error Conversions
Implement `From` traits for common error types (io::Error, serde_json::Error)

### Step 5: Comprehensive Testing
Test all error types, Display formatting, error chains, and context helpers

## 🧪 **Testing Commands**
``````bash
cargo test --package ttrpg-core
cargo doc --package ttrpg-core --open
``````

## 🔗 **Dependencies**: M1.T1 Complete
## 🚧 **Blocks**: All other development (core errors needed everywhere)
"@
        labels = @("M1: Foundation", "critical", "error-handling", "junior-dev-ready")
        milestone = "M1: Foundation (Week 1-2)"
    },
    @{
        title = "[M1.T3] Core Type System & Entity Traits"
        body = @"
## 🎯 **Objective**
Implement core type system with Entity traits, permission levels, and conversion contexts.

**Duration**: 2 days | **Points**: 6 | **Priority**: 🔥 HIGH

## ✅ **Acceptance Criteria**
- [ ] `EntityType` enum with all TTRPG entity types  
- [ ] `Entity` trait with id(), validate(), convert() methods
- [ ] `PermissionLevel` enum with proper ordering
- [ ] `ConversionContext` and `ConversionOptions` structs
- [ ] `EntityId` type using UUID for uniqueness
- [ ] All types have proper Serialize/Deserialize
- [ ] Comprehensive tests including property tests

## 📋 **Detailed Implementation Steps**

### Step 1: Create src/types.rs
Define core types:
``````rust
pub type EntityId = Uuid;  // UUID for uniqueness
pub type Attributes = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]  
pub enum EntityType {
    Actor, Item, Scene, Journal, Table, Playlist, Folder, Macro, ChatMessage,
}
``````

### Step 2: Implement Entity Trait
``````rust
pub trait Entity {
    fn id(&self) -> EntityId;
    fn entity_type(&self) -> EntityType;
    fn name(&self) -> &str;
    fn validate(&self) -> ConversionResult<()>;
    fn attributes(&self) -> &Attributes;
}
``````

### Step 3: Permission System  
``````rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PermissionLevel {
    None = 0, Limited = 1, Observer = 2, Owner = 3,
}
``````

### Step 4: Conversion Context
``````rust
pub struct ConversionContext {
    pub source_format: String,
    pub target_format: String, 
    pub options: ConversionOptions,
}

pub struct ConversionOptions {
    pub download_assets: bool,
    pub max_asset_size: u64,
    pub optimize_images: bool,
    pub thread_count: Option<usize>,
}
``````

### Step 5: Helper Functions
- `new_entity_id()` - Generate UUID
- Display names and plural forms for EntityType
- Permission checking methods

## 🧪 **Testing Commands**
``````bash
cargo test --package ttrpg-core -- types
cargo check --package ttrpg-core --all-features
``````

## 🔗 **Dependencies**: M1.T2 Complete (needs error types)
## 🚧 **Blocks**: M2 Core Engine tasks (needs entity framework)
"@
        labels = @("M1: Foundation", "high-priority", "core-types", "junior-dev-ready")
        milestone = "M1: Foundation (Week 1-2)"
    }
)

# Create M1 issues
foreach ($issue in $m1Issues) {
    Write-Host "  Creating: $($issue.title)" -ForegroundColor Yellow
    $labelsParam = $issue.labels -join ","
    try {
        $cmd = "gh issue create --repo $RepoOwner/$RepoName --title `"$($issue.title)`" --body `"$($issue.body)`" --label `"$labelsParam`""
        if ($issue.milestone) { $cmd += " --milestone `"$($issue.milestone)`"" }
        Invoke-Expression $cmd | Out-Null
        Write-Host "  ✅ Created: $($issue.title)" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Failed: $($issue.title) - $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "📋 Creating issue templates..." -ForegroundColor Cyan
$templateDir = ".github/ISSUE_TEMPLATE"
New-Item -ItemType Directory -Path $templateDir -Force | Out-Null

# Task template for junior developers
@"
---
name: 🛠️ Implementation Task
about: Detailed implementation task with step-by-step instructions
title: '[MX.TX] Task Name - Junior Dev Ready'
labels: ['task', 'junior-dev-ready']
assignees: []
---

## 🎯 **Objective**
Clear description of what needs to be implemented.

**Duration**: X days | **Points**: X | **Priority**: 🚨/🔥/🟡/🟢

## ✅ **Acceptance Criteria**
- [ ] Specific, testable criteria
- [ ] Each criterion has clear pass/fail conditions
- [ ] Include performance targets where applicable

## 📋 **Detailed Implementation Steps**

### Step 1: Setup/Preparation
Exact commands to run, files to create, dependencies to add.

### Step 2: Core Implementation  
``````rust
// Exact code patterns to implement
// Include specific struct definitions, method signatures
``````

### Step 3: Testing
Required tests to write and commands to verify functionality.

## 🧪 **Testing Commands**
``````bash
# Exact commands to verify the task is complete
cargo test --package crate-name
cargo clippy --package crate-name -- -D warnings
``````

## 🔗 **Dependencies**: List of tasks that must be complete first
## 🚧 **Blocks**: List of tasks that depend on this one

## 📚 **Reference Materials**
- Link to relevant documentation
- Examples from existing codebase
- External resources if needed
"@ | Out-File -FilePath "$templateDir/task.md" -Encoding UTF8

Write-Host ""
Write-Host "🎉 Comprehensive GitHub Planning Setup Complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📊 **Created:**" -ForegroundColor Green
Write-Host "  • 5 Strategic Milestones with detailed descriptions" -ForegroundColor White
Write-Host "  • $($m1Issues.Count) M1 Foundation Tasks with junior-dev instructions" -ForegroundColor White
Write-Host "  • Issue templates for consistent task creation" -ForegroundColor White
Write-Host ""
Write-Host "🔗 **Next Steps:**" -ForegroundColor Yellow
Write-Host "1. Visit: https://github.com/$RepoOwner/$RepoName/issues" -ForegroundColor Cyan
Write-Host "2. Create GitHub Project boards for visual management" -ForegroundColor White  
Write-Host "3. Start with [M1.T1] Project Workspace Setup" -ForegroundColor White
Write-Host "4. Follow step-by-step instructions in each issue" -ForegroundColor White
Write-Host ""
Write-Host "💡 **Usage:**" -ForegroundColor Blue
Write-Host "  Each task has detailed implementation steps" -ForegroundColor White
Write-Host "  Use 'Closes #123' in commit messages for auto-linking" -ForegroundColor White
Write-Host "  All tasks include exact testing commands" -ForegroundColor White
Write-Host ""
