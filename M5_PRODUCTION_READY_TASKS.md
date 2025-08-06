# M5: Production Ready Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 5 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 18 | **Priority**: 🟢 LOW

Production deployment, testing, documentation, and performance optimization for professional release.

---

## **T5.1: Comprehensive Testing Suite**
**Duration**: 4 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: M4 Complete

### **Implementation Steps for Junior Developer**

**Step 1: Unit Test Infrastructure**
Create `crates/ttrpg-core/tests/mod.rs`:
```rust
use ttrpg_core::prelude::*;

mod test_utils {
    use super::*;
    
    /// Create a sample Roll20 campaign for testing
    pub fn create_test_campaign() -> Roll20Campaign {
        Roll20Campaign {
            name: "Test Campaign".to_string(),
            schema_version: Some("1.0".to_string()),
            characters: vec![create_test_character()],
            pages: vec![create_test_scene()],
            handouts: vec![create_test_handout()],
            ..Default::default()
        }
    }
    
    pub fn create_test_character() -> Roll20Character {
        Roll20Character {
            id: "test_char_1".to_string(),
            name: "Test Fighter".to_string(),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            attributes: test_attributes(),
            ..Default::default()
        }
    }
    
    /// Create temporary test files
    pub fn create_test_zip() -> tempfile::NamedTempFile {
        let file = tempfile::NamedTempFile::new().unwrap();
        // Create actual ZIP content for testing
        let mut zip = zip::ZipWriter::new(std::fs::File::create(file.path()).unwrap());
        
        // Add campaign.json
        zip.start_file("campaign.json", zip::write::FileOptions::default()).unwrap();
        let campaign = create_test_campaign();
        let json = serde_json::to_string(&campaign).unwrap();
        zip.write_all(json.as_bytes()).unwrap();
        
        // Add test image
        zip.start_file("images/test.jpg", zip::write::FileOptions::default()).unwrap();
        zip.write_all(b"fake image data").unwrap();
        
        zip.finish().unwrap();
        file
    }
}
```

**Step 2: Core Functionality Tests**
Create `crates/ttrpg-core/tests/conversion_tests.rs`:
```rust
use ttrpg_core::prelude::*;
use crate::test_utils::*;

#[test]
fn test_roll20_campaign_parsing() {
    let test_zip = create_test_zip();
    let parser = Roll20Parser::new();
    
    let result = parser.parse_campaign(test_zip.path());
    
    assert!(result.is_ok());
    let campaign = result.unwrap();
    assert_eq!(campaign.name, "Test Campaign");
    assert_eq!(campaign.characters.len(), 1);
    assert_eq!(campaign.pages.len(), 1);
}

#[test]
fn test_character_conversion() {
    let roll20_char = create_test_character();
    let converter = CharacterConverter::new();
    
    let result = converter.convert(&roll20_char);
    
    assert!(result.is_ok());
    let foundry_actor = result.unwrap();
    assert_eq!(foundry_actor.name, "Test Fighter");
    assert!(foundry_actor.data.attributes.contains_key("hp"));
}

#[test]
fn test_asset_mapping() {
    let campaign = create_test_campaign();
    let asset_mapper = AssetMapper::new();
    
    let assets = asset_mapper.extract_asset_references(&campaign);
    
    assert!(!assets.is_empty());
    assert!(assets.iter().any(|a| a.url.contains("avatar.jpg")));
}

#[test]
fn test_validation_pipeline() {
    let campaign = create_test_campaign();
    let validator = CampaignValidator::new();
    
    let result = validator.validate(&campaign);
    
    assert!(result.is_ok());
    let validation = result.unwrap();
    assert!(validation.errors.is_empty());
}
```

**Step 3: Property-Based Testing**
Add to `Cargo.toml`:
```toml
[dev-dependencies]
proptest = "1.0"
```

Create `crates/ttrpg-core/tests/property_tests.rs`:
```rust
use proptest::prelude::*;
use ttrpg_core::prelude::*;

proptest! {
    #[test]
    fn test_character_name_conversion(name in "[a-zA-Z0-9 ]{1,50}") {
        let mut character = Roll20Character::default();
        character.name = name.clone();
        
        let converter = CharacterConverter::new();
        let result = converter.convert(&character);
        
        prop_assert!(result.is_ok());
        let foundry_actor = result.unwrap();
        prop_assert_eq!(foundry_actor.name, name);
    }
    
    #[test]
    fn test_asset_url_validation(url in r"https?://[a-zA-Z0-9./\-_]{1,200}") {
        let asset_ref = AssetReference {
            url: url.clone(),
            asset_type: AssetType::Image,
        };
        
        let result = asset_ref.validate();
        
        // Should not panic and should return some result
        prop_assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_attribute_value_conversion(value in -1000i32..1000i32) {
        let attr = Roll20Attribute {
            name: "strength".to_string(),
            current: Some(value.to_string()),
            max: Some((value + 10).to_string()),
        };
        
        let converter = AttributeConverter::new();
        let result = converter.convert(&attr);
        
        prop_assert!(result.is_ok());
        let foundry_attr = result.unwrap();
        prop_assert!(foundry_attr.value >= -1000 && foundry_attr.value <= 1000);
    }
}
```

**Step 4: Integration Tests**
Create `crates/ttrpg-cli/tests/cli_integration.rs`:
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_convert_command_success() {
    let temp_dir = TempDir::new().unwrap();
    let test_zip = create_test_campaign_zip();
    let output_path = temp_dir.path().join("output");
    
    let mut cmd = Command::cargo_bin("ttrpg-cli").unwrap();
    cmd.arg("convert")
       .arg("--input").arg(test_zip.path())
       .arg("--output").arg(&output_path)
       .arg("--source").arg("roll20")
       .arg("--target").arg("foundry-v12");
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Conversion complete"));
    
    // Verify output files were created
    assert!(output_path.join("world.json").exists());
    assert!(output_path.join("data").exists());
}

#[test]
fn test_validate_command() {
    let test_zip = create_test_campaign_zip();
    
    let mut cmd = Command::cargo_bin("ttrpg-cli").unwrap();
    cmd.arg("validate")
       .arg("--input").arg(test_zip.path())
       .arg("--source").arg("roll20");
    
    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Validation complete"));
}

#[test]
fn test_invalid_input_file() {
    let mut cmd = Command::cargo_bin("ttrpg-cli").unwrap();
    cmd.arg("convert")
       .arg("--input").arg("/nonexistent/file.zip")
       .arg("--output").arg("/tmp/output")
       .arg("--source").arg("roll20")
       .arg("--target").arg("foundry-v12");
    
    cmd.assert()
       .failure()
       .stderr(predicate::str::contains("File not found"));
}
```

**Acceptance Criteria**:
- [ ] Unit tests for all core conversion functions (>90% coverage)
- [ ] Property-based tests for edge cases
- [ ] Integration tests for CLI and GUI
- [ ] Performance benchmarks for large campaigns
- [ ] Test data fixtures and utilities
- [ ] Continuous integration setup

---

## **T5.2: Performance Optimization**
**Duration**: 3 days | **Points**: 6 | **Priority**: 🟡 MEDIUM  
**Dependencies**: T5.1 Complete

### **Implementation Steps**

**Step 1: Benchmarking Infrastructure**
Add to workspace `Cargo.toml`:
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "conversion_benchmarks" 
harness = false
```

Create `benches/conversion_benchmarks.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use ttrpg_core::prelude::*;

fn benchmark_character_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("character_conversion");
    
    // Test with different character complexities
    for size in [1, 10, 50, 100].iter() {
        let characters = create_test_characters(*size);
        
        group.bench_with_input(
            BenchmarkId::new("convert_characters", size),
            &characters,
            |b, chars| {
                b.iter(|| {
                    let converter = CharacterConverter::new();
                    for char in chars {
                        black_box(converter.convert(char).unwrap());
                    }
                });
            },
        );
    }
    group.finish();
}

fn benchmark_asset_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("asset_processing");
    
    // Test asset mapping performance
    let campaign = create_large_test_campaign();
    
    group.bench_function("asset_extraction", |b| {
        b.iter(|| {
            let mapper = AssetMapper::new();
            black_box(mapper.extract_asset_references(&campaign));
        });
    });
    
    group.bench_function("asset_download", |b| {
        b.iter(|| {
            let downloader = AssetDownloader::new();
            let assets = vec![create_test_asset_ref()];
            black_box(downloader.download_batch(&assets));
        });
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_character_conversion, benchmark_asset_processing);
criterion_main!(benches);
```

**Step 2: Memory Usage Optimization**
```rust
// Implement streaming for large files
pub struct StreamingParser<R: Read> {
    reader: R,
    buffer_size: usize,
}

impl<R: Read> StreamingParser<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer_size: 8192, // 8KB buffer
        }
    }
    
    pub fn parse_campaign(&mut self) -> ConversionResult<Roll20Campaign> {
        // Parse JSON in chunks to avoid loading entire file into memory
        let mut de = serde_json::Deserializer::from_reader(&mut self.reader);
        let campaign = Roll20Campaign::deserialize(&mut de)?;
        Ok(campaign)
    }
}

// Implement parallel processing for independent operations
pub fn convert_characters_parallel(characters: Vec<Roll20Character>) -> ConversionResult<Vec<FoundryActor>> {
    use rayon::prelude::*;
    
    characters
        .par_iter()
        .map(|char| {
            let converter = CharacterConverter::new();
            converter.convert(char)
        })
        .collect()
}
```

**Step 3: Caching System**
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ConversionCache {
    character_cache: Arc<Mutex<HashMap<String, FoundryActor>>>,
    asset_cache: Arc<Mutex<HashMap<String, PathBuf>>>,
    validation_cache: Arc<Mutex<HashMap<String, ValidationResult>>>,
}

impl ConversionCache {
    pub fn new() -> Self {
        Self {
            character_cache: Arc::new(Mutex::new(HashMap::new())),
            asset_cache: Arc::new(Mutex::new(HashMap::new())),
            validation_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn get_character(&self, id: &str) -> Option<FoundryActor> {
        self.character_cache.lock().unwrap().get(id).cloned()
    }
    
    pub fn cache_character(&self, id: String, actor: FoundryActor) {
        self.character_cache.lock().unwrap().insert(id, actor);
    }
    
    pub fn clear(&self) {
        self.character_cache.lock().unwrap().clear();
        self.asset_cache.lock().unwrap().clear();
        self.validation_cache.lock().unwrap().clear();
    }
}
```

**Acceptance Criteria**:
- [ ] Comprehensive benchmarks for all major operations
- [ ] Memory usage profiling and optimization
- [ ] Parallel processing for independent operations
- [ ] Caching system for repeated operations
- [ ] Performance regression tests
- [ ] Target: <5 seconds for typical campaign conversion

---

## **T5.3: Documentation and User Guides**
**Duration**: 3 days | **Points**: 4 | **Priority**: 🟡 MEDIUM
**Dependencies**: T5.2 Complete

### **Implementation Steps**

**Step 1: API Documentation**
Create comprehensive rustdoc comments:
```rust
//! TTRPGConverter - Convert TTRPG campaigns between formats
//! 
//! This crate provides functionality to convert Roll20 campaigns to Foundry VTT
//! and other supported formats. It handles character sheets, scenes, assets,
//! and all associated data with high fidelity.
//! 
//! # Quick Start
//! 
//! ```rust
//! use ttrpg_core::prelude::*;
//! 
//! // Parse a Roll20 campaign
//! let parser = Roll20Parser::new();
//! let campaign = parser.parse_file("campaign.zip")?;
//! 
//! // Convert to Foundry VTT
//! let converter = FoundryConverter::new();
//! let foundry_world = converter.convert(campaign)?;
//! 
//! // Write output
//! foundry_world.write_to_directory("./output")?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//! 
//! # Architecture
//! 
//! The converter is built around several key components:
//! 
//! - [`Roll20Parser`] - Parses Roll20 campaign exports
//! - [`FoundryConverter`] - Converts to Foundry VTT format
//! - [`AssetMapper`] - Handles asset extraction and mapping
//! - [`Validator`] - Validates campaign data integrity

/// Converts Roll20 characters to Foundry VTT actors
/// 
/// This converter handles the complex mapping between Roll20's character sheet
/// format and Foundry VTT's actor system. It preserves all attributes, inventory,
/// spells, and other character data.
/// 
/// # Examples
/// 
/// ```rust
/// let converter = CharacterConverter::new();
/// let foundry_actor = converter.convert(&roll20_character)?;
/// assert_eq!(foundry_actor.name, roll20_character.name);
/// ```
/// 
/// # Errors
/// 
/// Returns [`ConversionError`] if:
/// - Character data is malformed
/// - Required attributes are missing
/// - Asset references cannot be resolved
/// 
/// # Panics
/// 
/// This function will panic if the internal conversion state becomes corrupted,
/// which should never happen under normal usage.
pub struct CharacterConverter {
    // ... implementation
}
```

**Step 2: User Guide**
Create `docs/USER_GUIDE.md`:
```markdown
# TTRPGConverter User Guide

## Table of Contents
1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Command Line Interface](#command-line-interface)
4. [Graphical Interface](#graphical-interface)
5. [Configuration](#configuration)
6. [Troubleshooting](#troubleshooting)

## Installation

### From Releases (Recommended)
Download the latest release from GitHub:
- Windows: `ttrpg-converter-windows.exe`
- macOS: `ttrpg-converter-macos`
- Linux: `ttrpg-converter-linux`

### From Source
```bash
git clone https://github.com/yourusername/ttrpg-converter
cd ttrpg-converter
cargo build --release
```

## Quick Start

### Converting a Roll20 Campaign

1. **Export your Roll20 campaign**:
   - Go to Roll20 campaign settings
   - Click "Export Campaign"
   - Download the ZIP file

2. **Convert using CLI**:
   ```bash
   ttrpg-cli convert --input campaign.zip --output ./foundry-world --source roll20 --target foundry-v12
   ```

3. **Import into Foundry VTT**:
   - Copy output folder to Foundry data directory
   - Restart Foundry VTT
   - Create new world using converted data

### Using the GUI

1. Launch `ttrpg-gui`
2. Click "Browse..." to select your campaign ZIP file
3. Choose output directory
4. Select source and target formats
5. Click "Start Conversion"
6. Wait for completion and check results

## Command Line Interface

### Convert Command
```bash
ttrpg-cli convert [OPTIONS]

Options:
    -i, --input <FILE>        Input campaign file
    -o, --output <DIR>        Output directory
    -s, --source <FORMAT>     Source format (roll20, foundry-vtt)
    -t, --target <FORMAT>     Target format (foundry-v10, foundry-v11, foundry-v12)
        --no-assets           Skip asset download
        --force               Overwrite existing files
    -v, --verbose             Verbose output
```

### Validate Command
```bash
ttrpg-cli validate [OPTIONS]

Options:
    -i, --input <FILE>        Input campaign file
    -s, --source <FORMAT>     Source format
        --detailed            Show detailed validation report
```

### Examples
```bash
# Basic conversion
ttrpg-cli convert -i campaign.zip -o ./output -s roll20 -t foundry-v12

# Conversion without assets
ttrpg-cli convert -i campaign.zip -o ./output -s roll20 -t foundry-v12 --no-assets

# Validate before converting
ttrpg-cli validate -i campaign.zip -s roll20 --detailed
```

## Configuration

Create `ttrpg-config.toml`:
```toml
[conversion]
download_assets = true
max_asset_size = "50MB"
optimize_images = true
skip_validation = false

[output]
foundry_version = "v12"
compress_assets = true
preserve_folder_structure = true
backup_existing = true

[logging]
level = "info"
format = "compact"
```

## Troubleshooting

### Common Issues

**"File not found" error**:
- Ensure the input file path is correct
- Check file permissions

**"Conversion failed" error**:
- Try validating the campaign first
- Check the detailed error message
- Ensure you have enough disk space

**Missing assets in output**:
- Check if assets were external URLs
- Try running with `--verbose` to see download attempts
- Some Roll20 assets may require authentication

**Foundry VTT import issues**:
- Ensure you're using the correct Foundry version
- Check Foundry logs for import errors
- Verify output directory structure

### Getting Help

1. Check this guide and FAQ
2. Run with `--verbose` for detailed logs
3. Open an issue on GitHub with:
   - Command used
   - Error message
   - Input file information (no sensitive data)

## Advanced Usage

### Batch Processing
```bash
# Process multiple campaigns
for file in *.zip; do
    ttrpg-cli convert -i "$file" -o "./converted/${file%.zip}" -s roll20 -t foundry-v12
done
```

### Custom Configuration
```bash
# Use custom config file
ttrpg-cli --config ./my-config.toml convert -i campaign.zip -o output
```
```

**Acceptance Criteria**:
- [ ] Complete API documentation with rustdoc
- [ ] Comprehensive user guide with examples
- [ ] Installation instructions for all platforms
- [ ] Troubleshooting section with common issues
- [ ] Developer documentation for contributors
- [ ] Generated docs published to GitHub Pages

---

## **T5.4: Distribution and Packaging**
**Duration**: 2 days | **Points**: 0 | **Priority**: 🟢 LOW
**Dependencies**: T5.3 Complete

### **Implementation Steps**

**Step 1: GitHub Actions CI/CD**
Create `.github/workflows/release.yml`:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target/
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Build release
      run: cargo build --release
      
    - name: Run tests
      run: cargo test --release
      
    - name: Package artifacts
      shell: bash
      run: |
        if [ "${{ matrix.os }}" = "windows-latest" ]; then
          cp target/release/ttrpg-cli.exe ttrpg-cli-windows.exe
          cp target/release/ttrpg-gui.exe ttrpg-gui-windows.exe
        elif [ "${{ matrix.os }}" = "macos-latest" ]; then
          cp target/release/ttrpg-cli ttrpg-cli-macos
          cp target/release/ttrpg-gui ttrpg-gui-macos
        else
          cp target/release/ttrpg-cli ttrpg-cli-linux
          cp target/release/ttrpg-gui ttrpg-gui-linux
        fi
        
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: binaries-${{ matrix.os }}
        path: ttrpg-*
        
  release:
    needs: build
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/')
    
    steps:
    - name: Download artifacts
      uses: actions/download-artifact@v3
      
    - name: Create release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          binaries-*/*
        generate_release_notes: true
```

**Step 2: Cross-Platform Packaging**
Create `scripts/package.sh`:
```bash
#!/bin/bash

VERSION=${1:-"dev"}
PLATFORMS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin"

for PLATFORM in $PLATFORMS; do
    echo "Building for $PLATFORM..."
    
    cargo build --release --target $PLATFORM
    
    case $PLATFORM in
        *windows*)
            ARCHIVE="ttrpg-converter-${VERSION}-windows.zip"
            zip -j $ARCHIVE target/$PLATFORM/release/ttrpg-cli.exe target/$PLATFORM/release/ttrpg-gui.exe
            ;;
        *darwin*)
            ARCHIVE="ttrpg-converter-${VERSION}-macos.tar.gz"
            tar -czf $ARCHIVE -C target/$PLATFORM/release ttrpg-cli ttrpg-gui
            ;;
        *linux*)
            ARCHIVE="ttrpg-converter-${VERSION}-linux.tar.gz"
            tar -czf $ARCHIVE -C target/$PLATFORM/release ttrpg-cli ttrpg-gui
            ;;
    esac
    
    echo "Created $ARCHIVE"
done
```

**Step 3: Installation Scripts**
Create `install.sh` (Unix):
```bash
#!/bin/bash

set -e

REPO="yourusername/ttrpg-converter"
INSTALL_DIR="$HOME/.local/bin"

# Detect platform
case "$(uname -s)" in
    Darwin) PLATFORM="macos" ;;
    Linux)  PLATFORM="linux" ;;
    *)      echo "Unsupported platform"; exit 1 ;;
esac

# Get latest release
RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
DOWNLOAD_URL=$(curl -s $RELEASE_URL | grep "browser_download_url.*$PLATFORM" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Could not find download URL for $PLATFORM"
    exit 1
fi

echo "Downloading TTRPGConverter..."
TEMP_FILE=$(mktemp)
curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL"

echo "Installing to $INSTALL_DIR..."
mkdir -p "$INSTALL_DIR"
tar -xzf "$TEMP_FILE" -C "$INSTALL_DIR"

chmod +x "$INSTALL_DIR/ttrpg-cli" "$INSTALL_DIR/ttrpg-gui"

echo "Installation complete!"
echo "Add $INSTALL_DIR to your PATH to use ttrpg-cli and ttrpg-gui from anywhere."
```

**Acceptance Criteria**:
- [ ] Automated builds for Windows, macOS, and Linux
- [ ] GitHub Releases with binary artifacts
- [ ] Installation scripts for easy setup
- [ ] Checksums for security verification
- [ ] Package management integration (brew, chocolatey, etc.)
- [ ] Docker images for containerized usage

---

This completes the comprehensive M5 production-ready milestone with the same detailed junior-developer-ready format as the previous milestones!
