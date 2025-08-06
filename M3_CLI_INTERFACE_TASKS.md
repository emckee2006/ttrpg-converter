# M3: CLI Interface Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 3 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 20 | **Priority**: 🔥 HIGH

Complete command-line interface with modern UX, progress tracking, and comprehensive functionality.

---

## **T3.1: CLI Command Structure & Argument Parsing**
**Duration**: 2 days | **Points**: 6 | **Priority**: 🔥 HIGH
**Dependencies**: M1 Complete

### **Implementation Steps for Junior Developer**

**Step 1: Update ttrpg-cli Dependencies**
```bash
cd crates\ttrpg-cli
```

Update `Cargo.toml`:
```toml
[dependencies]
clap = { workspace = true }
indicatif = { workspace = true }
colored = { workspace = true }
dialoguer = { workspace = true }
ttrpg-core = { path = "../ttrpg-core" }
ttrpg-formats = { path = "../ttrpg-formats" }
console = "0.15"
directories = "5.0"
```

**Step 2: Design CLI Structure**
Create `src/cli.rs`:
```rust
#[derive(Parser, Debug)]
#[command(name = "ttrpg-cli", version, about = "Convert TTRPG campaigns between formats")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Convert {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, value_enum)]
        source: SourceFormat,
        #[arg(short, long, value_enum)]
        target: TargetFormat,
        #[arg(long)]
        no_assets: bool,
        #[arg(long)]
        force: bool,
    },
    Validate {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long, value_enum)]
        source: SourceFormat,
        #[arg(long)]
        detailed: bool,
    },
    Info {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long, value_enum)]
        source: SourceFormat,
        #[arg(long)]
        stats: bool,
    },
    Config {
        #[arg(short, long, default_value = "ttrpg-config.toml")]
        output: PathBuf,
        #[arg(long)]
        force: bool,
    },
}

#[derive(ValueEnum, Clone, Debug)]
pub enum SourceFormat {
    Roll20,
    FoundryVtt,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TargetFormat {
    FoundryV10,
    FoundryV11, 
    FoundryV12,
    JsonExport,
}
```

**Step 3: Implement Main Entry Point**
Create `src/main.rs`:
```rust
fn main() {
    let cli = Cli::parse();
    setup_logging(&cli);
    
    if let Err(e) = run_command(&cli) {
        error!("Command failed: {}", e);
        process::exit(1);
    }
}

fn run_command(cli: &Cli) -> ConversionResult<()> {
    match &cli.command {
        Commands::Convert { input, output, source, target, no_assets, force } => {
            commands::convert::run_convert(input, output, source, target, *no_assets, *force)
        }
        // ... other commands
    }
}
```

**Acceptance Criteria**:
- [ ] Complete CLI argument parsing with clap derive API
- [ ] All subcommands: convert, validate, info, config
- [ ] Proper help text and examples
- [ ] Verbosity control (-v, -vv, -vvv, --quiet)
- [ ] Input validation and error messages

---

## **T3.2: Convert Command Implementation**
**Duration**: 3 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: T3.1 + M2 Complete

### **Implementation Steps**

**Step 1: Create Command Module Structure**
```bash
mkdir src\commands
```

Create `src/commands/convert.rs`:
```rust
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use ttrpg_core::prelude::*;
use ttrpg_formats::roll20::Roll20Parser;

pub fn run_convert(
    input: &Path,
    output: &Path,
    source: &SourceFormat,
    target: &TargetFormat,
    no_assets: bool,
    force: bool,
) -> ConversionResult<()> {
    // Step 1: Validate inputs
    validate_convert_inputs(input, output, force)?;
    
    // Step 2: Create progress tracking
    let pb = create_progress_bar("Converting campaign...");
    
    // Step 3: Parse source file
    pb.set_message("Parsing source file...");
    let campaign = parse_source_file(input, source)?;
    pb.inc(25);
    
    // Step 4: Convert entities  
    pb.set_message("Converting entities...");
    let converted = convert_campaign(campaign, target)?;
    pb.inc(50);
    
    // Step 5: Download assets (if enabled)
    if !no_assets {
        pb.set_message("Processing assets...");
        download_assets(&converted)?;
        pb.inc(15);
    }
    
    // Step 6: Generate output
    pb.set_message("Generating output...");
    write_output(&converted, output, target)?;
    pb.finish_with_message("Conversion complete!".green().to_string());
    
    print_conversion_summary(&converted);
    Ok(())
}
```

**Step 2: Implement Progress Tracking**
```rust
fn create_progress_bar(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}"
        ).unwrap()
    );
    pb.set_message(msg.to_string());
    pb
}

fn print_conversion_summary(campaign: &FoundryCampaign) {
    println!();
    println!("{}", "Conversion Summary:".bold().green());
    println!("  Characters: {}", campaign.actors.len().to_string().cyan());
    println!("  Scenes: {}", campaign.scenes.len().to_string().cyan());
    println!("  Items: {}", campaign.items.len().to_string().cyan());
    println!("  Journal Entries: {}", campaign.journal.len().to_string().cyan());
}
```

**Step 3: Add Interactive Confirmations**
```rust
use dialoguer::{Confirm, Select};

fn confirm_overwrite(output: &Path) -> ConversionResult<bool> {
    let confirm = Confirm::new()
        .with_prompt(format!("Output directory '{}' exists. Overwrite?", output.display()))
        .default(false)
        .interact()?;
    Ok(confirm)
}
```

**Acceptance Criteria**:
- [ ] Complete convert command with all options
- [ ] Real-time progress bars with ETA
- [ ] Colored output and formatting
- [ ] Interactive confirmations for overwrites
- [ ] Detailed conversion summary
- [ ] Error handling with actionable messages

---

## **T3.3: Validate & Info Commands**
**Duration**: 2 days | **Points**: 4 | **Priority**: 🟡 MEDIUM
**Dependencies**: T3.1 Complete

### **Implementation Steps**

**Step 1: Implement Validate Command**
Create `src/commands/validate.rs`:
```rust
pub fn run_validate(
    input: &Path, 
    source: &SourceFormat, 
    detailed: bool
) -> ConversionResult<()> {
    let pb = create_progress_bar("Validating campaign...");
    
    // Parse and validate
    let campaign = parse_source_file(input, source)?;
    let validation_result = campaign.validate()?;
    
    if detailed {
        print_detailed_validation(&validation_result);
    } else {
        print_validation_summary(&validation_result);
    }
    
    pb.finish_with_message("Validation complete!".green().to_string());
    Ok(())
}

fn print_detailed_validation(result: &ValidationResult) {
    println!("{}", "Detailed Validation Report:".bold().green());
    
    if result.errors.is_empty() {
        println!("  {} No errors found", "✓".green());
    } else {
        println!("  {} Errors found:", "✗".red());
        for error in &result.errors {
            println!("    - {}: {}", error.entity_type.red(), error.message);
        }
    }
    
    if !result.warnings.is_empty() {
        println!("  {} Warnings:", "⚠".yellow());
        for warning in &result.warnings {
            println!("    - {}: {}", warning.entity_type.yellow(), warning.message);
        }
    }
}
```

**Step 2: Implement Info Command**
Create `src/commands/info.rs`:
```rust
pub fn run_info(
    input: &Path, 
    source: &SourceFormat, 
    stats: bool
) -> ConversionResult<()> {
    let campaign = parse_source_file(input, source)?;
    
    print_basic_info(&campaign);
    
    if stats {
        print_detailed_stats(&campaign);
    }
    
    Ok(())
}

fn print_basic_info(campaign: &Roll20Campaign) {
    println!("{}", "Campaign Information:".bold().green());
    println!("  Name: {}", campaign.name.cyan());
    println!("  Schema Version: {}", campaign.schema_version.as_deref().unwrap_or("Unknown").cyan());
    println!("  Total Entities: {}", campaign.stats().total_entities().to_string().cyan());
}

fn print_detailed_stats(campaign: &Roll20Campaign) {
    let stats = campaign.stats();
    println!();
    println!("{}", "Detailed Statistics:".bold().green());
    
    // Create table-like output
    println!("  {:<15} {:>8}", "Entity Type", "Count");
    println!("  {}", "─".repeat(25));
    println!("  {:<15} {:>8}", "Characters", stats.characters);
    println!("  {:<15} {:>8}", "Scenes", stats.pages);
    println!("  {:<15} {:>8}", "Handouts", stats.handouts);
    println!("  {:<15} {:>8}", "Tokens", stats.graphics);
    println!("  {:<15} {:>8}", "Walls", stats.paths);
    println!("  {:<15} {:>8}", "Tables", stats.rollable_tables);
    println!("  {:<15} {:>8}", "Macros", stats.macros);
    
    // Asset statistics
    let assets = campaign.asset_references();
    println!();
    println!("  {:<15} {:>8}", "Total Assets", assets.len());
    
    let image_count = assets.iter().filter(|a| a.asset_type == AssetType::Image).count();
    let audio_count = assets.iter().filter(|a| a.asset_type == AssetType::Audio).count();
    
    println!("  {:<15} {:>8}", "Images", image_count);
    println!("  {:<15} {:>8}", "Audio Files", audio_count);
}
```

**Acceptance Criteria**:
- [ ] Validate command with detailed/summary modes
- [ ] Info command with basic/stats modes  
- [ ] Formatted table output for statistics
- [ ] Color-coded validation results
- [ ] Progress tracking for large files

---

## **T3.4: Configuration System**
**Duration**: 2 days | **Points**: 4 | **Priority**: 🟡 MEDIUM
**Dependencies**: T3.1 Complete

### **Implementation Steps**

**Step 1: Create Config Structures**
Create `src/config.rs`:
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTRPGConfig {
    pub conversion: ConversionConfig,
    pub output: OutputConfig,
    pub logging: LoggingConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    pub download_assets: bool,
    pub max_asset_size: String, // "50MB", "100MB", etc.
    pub optimize_images: bool,
    pub skip_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub foundry_version: String, // "v10", "v11", "v12"
    pub compress_assets: bool,
    pub preserve_folder_structure: bool,
    pub backup_existing: bool,
}

impl Default for TTRPGConfig {
    fn default() -> Self {
        Self {
            conversion: ConversionConfig {
                download_assets: true,
                max_asset_size: "50MB".to_string(),
                optimize_images: true,
                skip_validation: false,
            },
            output: OutputConfig {
                foundry_version: "v12".to_string(),
                compress_assets: true,
                preserve_folder_structure: true,
                backup_existing: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "compact".to_string(),
            },
            performance: PerformanceConfig {
                thread_count: None, // Auto-detect
                memory_limit: "1GB".to_string(),
            },
        }
    }
}
```

**Step 2: Implement Config Command**
Create `src/commands/config.rs`:
```rust
pub fn run_config(output: &Path, force: bool) -> ConversionResult<()> {
    if output.exists() && !force {
        let overwrite = dialoguer::Confirm::new()
            .with_prompt(format!("Config file '{}' exists. Overwrite?", output.display()))
            .default(false)
            .interact()?;
            
        if !overwrite {
            println!("Config generation cancelled.");
            return Ok(());
        }
    }
    
    let config = TTRPGConfig::default();
    let toml_content = toml::to_string_pretty(&config)?;
    
    std::fs::write(output, toml_content)?;
    
    println!("{}", "Configuration file created successfully!".green());
    println!("  Location: {}", output.display().to_string().cyan());
    println!("  Edit the file to customize conversion settings.");
    
    Ok(())
}
```

**Step 3: Config Loading and Validation**
```rust
pub fn load_config<P: AsRef<Path>>(path: P) -> ConversionResult<TTRPGConfig> {
    let content = std::fs::read_to_string(path.as_ref())
        .with_file_context(path.as_ref())?;
    
    let config: TTRPGConfig = toml::from_str(&content)
        .map_err(|e| ConversionError::Config {
            section: "TOML parsing".to_string(),
            message: e.to_string(),
            config_path: Some(path.as_ref().to_path_buf()),
        })?;
    
    validate_config(&config)?;
    Ok(config)
}

fn validate_config(config: &TTRPGConfig) -> ConversionResult<()> {
    // Validate max_asset_size format
    parse_size(&config.conversion.max_asset_size)?;
    
    // Validate foundry_version
    match config.output.foundry_version.as_str() {
        "v10" | "v11" | "v12" => {}
        _ => return Err(ConversionError::Config {
            section: "output.foundry_version".to_string(),
            message: "Must be 'v10', 'v11', or 'v12'".to_string(),
            config_path: None,
        }),
    }
    
    Ok(())
}
```

**Acceptance Criteria**:
- [ ] TOML configuration file generation
- [ ] Config validation with helpful error messages
- [ ] Default values for all settings
- [ ] Interactive config creation
- [ ] Environment variable support
- [ ] Config file discovery (current dir, home dir, etc.)

---

This completes M3 with the same comprehensive detail level as your original Kanban planning!
