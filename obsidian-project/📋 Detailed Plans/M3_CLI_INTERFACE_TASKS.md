# M3: CLI Interface - Junior Developer Implementation Guide

## 🎯 **MILESTONE 3 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 20 | **Priority**: 🔥 HIGH
**Dependencies**: M2 Processing Plugin Architecture Foundation

Professional CLI interface with automatic plugin discovery and interactive processing pipeline management.

### 🔧 **CLI CORE FEATURES**
Streamlined CLI leveraging Processing Plugin Architecture:
- **Plugin Discovery Commands**: List and inspect available processing plugins
- **Interactive Pipeline Builder**: Step-through plugin configuration with `dialoguer`
- **Template Management**: Save/load common processing workflows
- **Progress Visualization**: Real-time processing with `indicatif` progress bars
- **Error Handling**: Clear error messages with recovery suggestions

### 🎆 **USER EXPERIENCE FOCUS**
Professional CLI experience:
- **Colored Output**: `console` crate for beautiful terminal output
- **Interactive Prompts**: Multi-select plugin configuration
- **Smart Validation**: Real-time pipeline validation before execution
- **Quick Commands**: Common workflows accessible via shortcuts

### 📐 **PROFESSIONAL CLI FRAMEWORKS**
Eliminate reinvented wheels with professional libraries:
- `clap` (v4) - Modern command-line argument parsing with derive macros
- `dialoguer` - Interactive prompts and confirmation dialogs
- `indicatif` - Professional progress bars with ETA and throughput
- `console` - Terminal styling and cross-platform compatibility
- `inventory` - Plugin discovery and metadata access

---

## **T3.1: Interactive Plugin Selection CLI** 🆕 **MOVED FROM M4**
**Duration**: 3 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: M2.0 Plugin Orchestration Foundation

### **Implementation Steps for Junior Developer**

**Step 1: Enhanced CLI Dependencies with Plugin Discovery**
```bash
cd crates\ttrpg-cli
```

Update `Cargo.toml` with plugin orchestration support:
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

# 🔍 PLUGIN DISCOVERY SUPPORT
inventory = "0.3"        # Access to plugin registry
serde = { workspace = true }  # Pipeline configuration serialization
toml = "0.8"             # Template file format
```

**Step 2: Plugin Discovery CLI Commands**
Create `ttrpg-cli/src/commands/plugins.rs`:
```rust
use clap::Subcommand;
use dialoguer::{Select, MultiSelect, Input, Confirm};
use inventory;
use ttrpg_core::plugins::{PluginInfo, PluginType};
use ttrpg_processing_plugins::shared::{AssetExecutionContext, ValidationExecutionContext};
use ttrpg_processing_plugins::asset::coordinator::AssetProcessingCoordinator;
use std::collections::HashMap;

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// List all available plugins
    List {
        #[arg(short, long)]
        plugin_type: Option<PluginType>,
        #[arg(short, long)]
        verbose: bool,
    },
    /// Interactive plugin selection for pipeline building
    Select {
        #[arg(short, long)]
        input_format: String,
        #[arg(short, long)]
        output_format: String,
        #[arg(long)]
        save_template: Option<String>,
    },
    /// Show detailed plugin information
    Info {
        plugin_id: String,
    },
    /// Validate plugin compatibility
    Validate {
        plugin_ids: Vec<String>,
    },
}

pub async fn handle_plugin_commands(cmd: PluginCommands) -> Result<(), CliError> {
    match cmd {
        PluginCommands::List { plugin_type, verbose } => {
            list_plugins(plugin_type, verbose).await
        }
        PluginCommands::Select { input_format, output_format, save_template } => {
            interactive_plugin_selection(input_format, output_format, save_template).await
        }
        PluginCommands::Info { plugin_id } => {
            show_plugin_info(plugin_id).await
        }
        PluginCommands::Validate { plugin_ids } => {
            validate_plugin_compatibility(plugin_ids).await
        }
    }
}

/// List all available plugins with auto-discovery
async fn list_plugins(plugin_type: Option<PluginType>, verbose: bool) -> Result<(), CliError> {
    println!("🔍 Discovering available plugins...");
    
    let mut plugins_by_type: HashMap<PluginType, Vec<&PluginInfo>> = HashMap::new();
    
    // Auto-discover plugins using inventory
    for plugin_info in inventory::iter::<PluginInfo> {
        if let Some(filter_type) = &plugin_type {
            if &plugin_info.plugin_type != filter_type {
                continue;
            }
        }
        
        plugins_by_type
            .entry(plugin_info.plugin_type.clone())
            .or_insert_with(Vec::new)
            .push(plugin_info);
    }
    
    // Display plugins organized by type
    for (ptype, plugins) in plugins_by_type {
        println!("\n🔧 {} Plugins:", format!("{:?}", ptype).to_uppercase());
        
        for plugin in plugins {
            if verbose {
                println!("  ✅ {} ({})", plugin.name.bold(), plugin.id.cyan());
                println!("      Version: {}", plugin.version);
                println!("      Dependencies: {:?}", plugin.dependencies);
            } else {
                println!("  ✅ {} ({})", plugin.name, plugin.id.cyan());
            }
        }
    }
    
    Ok(())
}

/// Interactive plugin selection with smart defaults
async fn interactive_plugin_selection(
    input_format: String,
    output_format: String,
    save_template: Option<String>,
) -> Result<(), CliError> {
    println!("🎯 Building pipeline for {} → {}", input_format.cyan(), output_format.cyan());
    
    let mut coordinator = AssetProcessingCoordinator::new()?;
    
    // Get smart processing plugin recommendations
    let recommended_pipeline = coordinator.recommend_processing_pipeline(&input_format, &output_format)?;
    
    println!("\n🤖 Recommended pipeline:");
    for (i, plugin) in recommended_pipeline.plugins.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, plugin.name.green(), plugin.id);
    }
    
    let use_recommended = Confirm::new()
        .with_prompt("Use recommended pipeline?")
        .default(true)
        .interact()?;
    
    let selected_plugins = if use_recommended {
        recommended_pipeline.plugins
    } else {
        // Manual plugin selection
        interactive_manual_selection(&input_format, &output_format)?
    };
    
    // Build and validate processing pipeline
    let pipeline_id = coordinator.build_processing_pipeline_from_plugins(selected_plugins)?;
    
    println!("✅ Pipeline built successfully: {}", pipeline_id.to_string().green());
    
    // Save as processing template if requested
    if let Some(template_name) = save_template {
        coordinator.save_processing_template(pipeline_id, &template_name)?;
        println!("💾 Processing template saved: {}", template_name.cyan());
    }
    
    Ok(())
}
```

---

## **T3.2: Pipeline Template Management CLI** 🆕 **MOVED FROM M4**
**Duration**: 2 days | **Points**: 4 | **Priority**: 🟡 MEDIUM
**Dependencies**: T3.1 Interactive Plugin Selection CLI

### **Implementation Steps for Junior Developer**

**Step 1: Template Management Commands**
Create `ttrpg-cli/src/commands/templates.rs`:
```rust
use clap::Subcommand;
use ttrpg_processing_plugins::asset::coordinator::{AssetProcessingCoordinator, ProcessingTemplate};
use std::path::PathBuf;
use std::fs;

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// List available pipeline templates
    List,
    /// Save current pipeline as template
    Save {
        pipeline_id: String,
        template_name: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Load pipeline from template
    Load {
        template_name: String,
        #[arg(short, long)]
        override_input: Option<String>,
        #[arg(short, long)]
        override_output: Option<String>,
    },
    /// Delete pipeline template
    Delete {
        template_name: String,
        #[arg(short, long)]
        confirm: bool,
    },
    /// Show template details
    Info {
        template_name: String,
    },
}

pub async fn handle_template_commands(cmd: TemplateCommands) -> Result<(), CliError> {
    let coordinator = AssetProcessingCoordinator::new()?;
    
    match cmd {
        TemplateCommands::List => list_templates(&coordinator).await,
        TemplateCommands::Save { pipeline_id, template_name, description } => {
            save_template(&orchestrator, pipeline_id, template_name, description).await
        }
        TemplateCommands::Load { template_name, override_input, override_output } => {
            load_template(&orchestrator, template_name, override_input, override_output).await
        }
        TemplateCommands::Delete { template_name, confirm } => {
            delete_template(&orchestrator, template_name, confirm).await
        }
        TemplateCommands::Info { template_name } => {
            show_template_info(&orchestrator, template_name).await
        }
    }
}

/// List all available processing templates
async fn list_templates(coordinator: &AssetProcessingCoordinator) -> Result<(), CliError> {
    println!("📋 Available Pipeline Templates:");
    
    let templates = coordinator.list_processing_templates()?;
    
    if templates.is_empty() {
        println!("  No templates found. Create one with 'ttrpg-cli plugins select --save-template <name>'");
        return Ok(());
    }
    
    for template in templates {
        println!("\n  📋 {} ({})", template.name.bold(), template.id.cyan());
        println!("      {} → {}", template.input_format.green(), template.output_format.green());
        
        if let Some(description) = &template.description {
            println!("      {}", description.dimmed());
        }
        
        println!("      Plugins: {}", template.plugin_count);
        println!("      Created: {}", template.created_at.format("%Y-%m-%d %H:%M"));
    }
    
    Ok(())
}
```

**Step 2: Built-in Template Gallery**
Create `ttrpg-cli/src/templates/gallery.rs`:
```rust
use ttrpg_core::orchestration::{PipelineTemplate, PluginConfig};
use serde::{Deserialize, Serialize};

/// Built-in template gallery with common processing workflows
pub struct ProcessingTemplateGallery;

impl ProcessingTemplateGallery {
    /// Get all built-in processing templates
    pub fn get_builtin_templates() -> Vec<ProcessingTemplate> {
        vec![
            Self::roll20_to_foundry_basic(),
            Self::roll20_to_foundry_with_assets(),
            Self::foundry_to_roll20(),
            Self::multi_format_validation(),
            Self::asset_optimization_only(),
        ]
    }
    
    /// Basic Roll20 to Foundry processing pipeline
    fn roll20_to_foundry_basic() -> ProcessingTemplate {
        PipelineTemplate {
            id: "roll20-foundry-basic".to_string(),
            name: "Roll20 → Foundry (Basic)".to_string(),
            description: Some("Basic Roll20 to Foundry VTT conversion without asset processing".to_string()),
            input_format: "roll20".to_string(),
            output_format: "foundry".to_string(),
            plugins: vec![
                PluginConfig {
                    plugin_id: "roll20_input_plugin".to_string(),
                    config: serde_json::json!({
                        "flexible_schema": true,
                        "skip_malformed": false,
                    }),
                },
                PluginConfig {
                    plugin_id: "validation_plugin".to_string(),
                    config: serde_json::json!({
                        "use_jsonschema": true,
                        "parallel_validation": true,
                        "shared_cpu_pool": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "foundry_output_plugin".to_string(),
                    config: serde_json::json!({
                        "foundry_version": "v12",
                        "create_module": false,
                    }),
                },
            ],
            plugin_count: 3,
            created_at: chrono::Utc::now(),
        }
    }
    
    /// Roll20 to Foundry with comprehensive asset processing
    fn roll20_to_foundry_with_assets() -> PipelineTemplate {
        PipelineTemplate {
            id: "roll20-foundry-assets".to_string(),
            name: "Roll20 → Foundry (With Assets)".to_string(),
            description: Some("Complete Roll20 to Foundry conversion with asset optimization".to_string()),
            input_format: "roll20".to_string(),
            output_format: "foundry".to_string(),
            plugins: vec![
                PluginConfig {
                    plugin_id: "roll20_input_plugin".to_string(),
                    config: serde_json::json!({
                        "flexible_schema": true,
                        "extract_assets": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "asset_processing_coordinator".to_string(),
                    config: serde_json::json!({
                        "use_blake3_hashing": true,
                        "shared_http_client": true,
                        "concurrent_semaphore_limit": 50,
                        "shared_cpu_pool": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "asset_resolution_plugin".to_string(),
                    config: serde_json::json!({
                        "blake3_deduplication": true,
                        "shared_execution_context": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "asset_conversion_plugin".to_string(),
                    config: serde_json::json!({
                        "imageproc_optimization": true,
                        "parallel_processing": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "scene_processing_plugin".to_string(),
                    config: serde_json::json!({
                        "geo_wall_extraction": true,
                        "parallel_processing": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "reference_tracking_plugin".to_string(),
                    config: serde_json::json!({
                        "campaign_coordination": true,
                        "dashmap_references": true,
                    }),
                },
                PluginConfig {
                    plugin_id: "foundry_output_plugin".to_string(),
                    config: serde_json::json!({
                        "foundry_version": "v12",
                        "create_module": true,
                        "include_assets": true,
                    }),
                },
            ],
            plugin_count: 7,
            created_at: chrono::Utc::now(),
        }
    }
}
```

### **Success Criteria for Junior Developer**
- [ ] ✅ Template save/load functionality working with `daggy` serialization
- [ ] ✅ Built-in template gallery with common workflows
- [ ] ✅ Template validation and compatibility checking
- [ ] ✅ CLI commands for template management
- [ ] ✅ Template metadata and versioning support

---

## **T3.3: Enhanced CLI Command Structure**
**Duration**: 2 days | **Points**: 6 | **Priority**: 🔥 HIGH
**Dependencies**: M2.0 Plugin Orchestration Foundation

### **Implementation Steps for Junior Developer**

**Step 1: Main CLI Structure with Plugin Support**
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
    /// 🔌 Plugin management and discovery commands
    #[command(subcommand)]
    Plugins(PluginCommands),
    
    /// 📋 Pipeline template management
    #[command(subcommand)]  
    Templates(TemplateCommands),
    
    /// 🔄 Convert campaign between formats (smart pipeline selection)
    Convert {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        input_format: Option<String>,
        #[arg(short = 'f', long)]
        output_format: Option<String>,
        #[arg(long)]
        template: Option<String>,
        #[arg(long)]
        interactive: bool,
        #[arg(long)]
        no_assets: bool,
        #[arg(long)]
        force: bool,
    },
    
    /// ✅ Validate campaign data
    Validate {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        format: Option<String>,
        #[arg(long)]
        detailed: bool,
    },
    
    /// ℹ️ Show information about files or plugins
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
}
```

**Step 3: Advanced CLI Options Structure (30+ Options from Previous R20Converter)**
Expand the Convert command with comprehensive options:
```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    Convert {
        // Basic I/O options
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, value_enum)]
        source: SourceFormat,
        #[arg(short, long, value_enum)]
        target: TargetFormat,
        
        // Campaign Customization Options
        #[arg(long, help = "Custom campaign title")]
        campaign_title: Option<String>,
        #[arg(long, help = "Campaign description")]
        description: Option<String>,
        #[arg(long, help = "Set campaign password")]
        password: Option<String>,
        #[arg(long, help = "Interactive setup mode")]
        interactive: bool,
        
        // Wall Processing Options (from M2 Advanced Wall Processing)
        #[arg(long, help = "Walls restrict movement")]
        restrict_movement: bool,
        #[arg(long, help = "Add walls around map boundaries")]
        add_walls_around_map: bool,
        #[arg(long, help = "Clean up scenes (optimize cave walls)")]
        cleanup_scenes: bool,
        #[arg(long, default_value = "35", help = "Minimum wall length in pixels")]
        minimum_wall_length: f64,
        #[arg(long, default_value = "30", help = "Maximum wall angle for merging (degrees)")]
        maximum_wall_angle: f64,
        
        // Door Detection Options
        #[arg(long, help = "Auto-detect doors from wall colors")]
        auto_doors: bool,
        #[arg(long, default_value = "#ff0000", help = "Door color (hex)")]
        door_color: String,
        #[arg(long, default_value = "#00ff00", help = "Secret door color (hex)")]
        secret_door_color: String,
        
        // Asset Handling Options
        #[arg(long, help = "Use original image URLs (no local download)")]
        use_original_image_urls: bool,
        #[arg(long, help = "Convert all images to drawings")]
        images_as_drawings: bool,
        #[arg(long, help = "Treat all backgrounds as tiles")]
        all_backgrounds_as_tiles: bool,
        #[arg(long, default_value = "260", help = "Maximum asset path length")]
        max_path: usize,
        #[arg(long, default_value = "50", help = "Maximum asset size in MB")]
        max_asset_size: u64,
        
        // Map Features
        #[arg(long, help = "Enable fog of war")]
        enable_fog: bool,
        #[arg(long, help = "Disable fog of war")]
        disable_fog: bool,
        #[arg(long, help = "Force HP display on token bar 1")]
        force_hp_for_token_bar1: bool,
        #[arg(long, help = "Force HP display on token bar 2")]
        force_hp_for_token_bar2: bool,
        #[arg(long, help = "Grid size in pixels")]
        grid_size: Option<f64>,
        #[arg(long, help = "Grid units per square")]
        grid_units: Option<f64>,
        
        // Module Export Options
        #[arg(long, help = "Export as Foundry module instead of world")]
        export_as_module: bool,
        #[arg(long, help = "Disable module actors")]
        disable_module_actors: bool,
        #[arg(long, help = "Disable module items")]
        disable_module_items: bool,
        #[arg(long, help = "Disable module scenes")]
        disable_module_scenes: bool,
        #[arg(long, help = "Disable module journals")]
        disable_module_journals: bool,
        
        // Data Integration Options
        #[arg(long, help = "FoundryVTT data path for integration")]
        fvtt_data_path: Option<PathBuf>,
        #[arg(long, help = "NPC data source (compendium path)")]
        npc_source: Option<PathBuf>,
        #[arg(long, help = "Don't overwrite existing compendium data")]
        no_compendium_overwrite: bool,
        
        // Batch Processing Options
        #[arg(long, help = "Don't convert archived campaigns")]
        disable_archived: bool,
        #[arg(long, help = "Don't convert chat messages")]
        dont_convert_chat: bool,
        #[arg(long, help = "Process multiple campaigns in directory")]
        batch_mode: bool,
        
        // Advanced Options
        #[arg(long, help = "Generate debug page with conversion statistics")]
        debug_page: bool,
        #[arg(long, help = "Parallel processing threads (default: CPU cores)")]
        threads: Option<usize>,
        #[arg(long, help = "Verbose progress reporting")]
        verbose_progress: bool,
        
        // Legacy compatibility
        #[arg(long, help = "Skip asset processing entirely")]
        no_assets: bool,
        #[arg(long, help = "Force overwrite existing output")]
        force: bool,
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
- [ ] Environment variable support
- [ ] Config file discovery (current dir, home dir, etc.)

---

{{ ... }}
