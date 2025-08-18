# M6: CLI Tools and Automation - Junior Developer Implementation Guide

## 🎯 **MILESTONE 6 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 16 | **Priority**: 🔥 HIGH
**Dependencies**: M5 Advanced Processing Complete

Advanced CLI tooling, batch processing capabilities, workflow automation, and developer utilities for efficient TTRPG content management.

### 🔧 **CLI FEATURES**
- **Advanced CLI Interface**: Rich terminal UI with progress tracking
- **Batch Processing**: Multi-file and directory processing
- **Workflow Automation**: Pipeline scripting and job scheduling
- **Developer Tools**: Plugin development utilities and debugging

---

## **T6.1: Advanced CLI Interface**
**Duration**: 3 days | **Points**: 5 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: Enhanced CLI Dependencies**
Update `crates/apps/ttrpg-cli/Cargo.toml`:
```toml
[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
ttrpg-manager = { path = "../../core/ttrpg-manager" }
clap = { version = "4.4", features = ["derive", "color"] }
indicatif = { version = "0.17", features = ["rayon"] }
console = "0.15"
dialoguer = { version = "0.11", features = ["fuzzy-select"] }
crossterm = "0.27"
ratatui = "0.24"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
rayon = "1.8"
anyhow = "1.0"
colored = "2.1"
tabled = "0.15"
```

**Step 2: Rich CLI Interface Implementation**
Update `src/main.rs`:
```rust
use clap::{Parser, Subcommand};
use console::{style, Term};
use dialoguer::{MultiSelect, Select, Input, Confirm};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use tabled::{Table, Tabled};
use std::path::PathBuf;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
#[command(name = "ttrpg-cli")]
#[command(about = "Advanced TTRPG Content Converter CLI", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, default_value = "ttrpg-config.json")]
    config: PathBuf,
    
    /// Enable interactive mode
    #[arg(short, long)]
    interactive: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert single or multiple files
    Convert {
        /// Input file or directory
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Target format
        #[arg(short, long)]
        format: Option<String>,
        
        /// Enable batch processing
        #[arg(short, long)]
        batch: bool,
        
        /// Parallel processing threads
        #[arg(short, long, default_value = "4")]
        threads: usize,
    },
    
    /// Interactive conversion wizard
    Wizard,
    
    /// Batch processing operations
    Batch {
        /// Batch configuration file
        #[arg(short, long)]
        config: PathBuf,
        
        /// Dry run (don't execute)
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// Plugin management
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
    
    /// Validation and debugging
    Debug {
        /// Input file to debug
        #[arg(short, long)]
        input: PathBuf,
        
        /// Validation level
        #[arg(short, long, default_value = "standard")]
        level: String,
    },
}

#[derive(Subcommand)]
enum PluginCommands {
    /// List available plugins
    List,
    /// Install a plugin
    Install { path: PathBuf },
    /// Remove a plugin
    Remove { name: String },
    /// Test plugin functionality
    Test { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize CLI theme
    let term = Term::stdout();
    print_banner(&term)?;
    
    // Load configuration
    let config = load_or_create_config(&cli.config).await?;
    
    match cli.command {
        Some(Commands::Convert { input, output, format, batch, threads }) => {
            if cli.interactive {
                interactive_convert_wizard(input, output, format, batch, threads).await?;
            } else {
                standard_convert(input, output, format, batch, threads).await?;
            }
        }
        Some(Commands::Wizard) => {
            conversion_wizard().await?;
        }
        Some(Commands::Batch { config, dry_run }) => {
            batch_process(config, dry_run).await?;
        }
        Some(Commands::Plugin { action }) => {
            handle_plugin_command(action).await?;
        }
        Some(Commands::Debug { input, level }) => {
            debug_file(input, level).await?;
        }
        None => {
            // Interactive mode when no subcommand provided
            main_menu().await?;
        }
    }
    
    Ok(())
}

fn print_banner(term: &Term) -> anyhow::Result<()> {
    term.clear_screen()?;
    println!("{}", style("
╔══════════════════════════════════════════════════════════╗
║                 TTRPG CONTENT CONVERTER                  ║
║              Advanced CLI Interface v1.0                 ║
╚══════════════════════════════════════════════════════════╝
    ").cyan().bold());
    Ok(())
}

async fn interactive_convert_wizard(
    input: PathBuf,
    output: Option<PathBuf>,
    format: Option<String>,
    batch: bool,
    threads: usize
) -> anyhow::Result<()> {
    println!("{}", style("🧙 Interactive Conversion Wizard").green().bold());
    
    // File selection if not provided
    let input_path = if input.as_os_str().is_empty() {
        let input_str: String = Input::new()
            .with_prompt("Enter input file or directory path")
            .interact_text()?;
        PathBuf::from(input_str)
    } else {
        input
    };
    
    // Format selection
    let available_formats = vec!["foundry", "roll20", "pathbuilder", "pdf", "web-export"];
    let format_selection = Select::new()
        .with_prompt("Select target format")
        .items(&available_formats)
        .default(0)
        .interact()?;
    
    let target_format = available_formats[format_selection];
    
    // Processing options
    let use_batch = Confirm::new()
        .with_prompt("Enable batch processing for multiple files?")
        .default(batch)
        .interact()?;
    
    let thread_count = if use_batch {
        let threads_str: String = Input::new()
            .with_prompt("Number of parallel threads")
            .default(threads.to_string())
            .interact_text()?;
        threads_str.parse().unwrap_or(threads)
    } else {
        1
    };
    
    // Confirm and execute
    println!("\n{}", style("Conversion Configuration:").yellow().bold());
    println!("Input: {}", style(input_path.display()).cyan());
    println!("Format: {}", style(target_format).cyan());
    println!("Batch: {}", style(use_batch).cyan());
    println!("Threads: {}", style(thread_count).cyan());
    
    if Confirm::new()
        .with_prompt("\nProceed with conversion?")
        .default(true)
        .interact()? 
    {
        execute_conversion(input_path, output, Some(target_format.to_string()), use_batch, thread_count).await?;
    }
    
    Ok(())
}

async fn execute_conversion(
    input: PathBuf,
    output: Option<PathBuf>,
    format: Option<String>,
    batch: bool,
    threads: usize
) -> anyhow::Result<()> {
    let multi_progress = MultiProgress::new();
    
    if batch && input.is_dir() {
        // Batch processing
        let files = discover_convertible_files(&input).await?;
        println!("Found {} files for conversion", files.len());
        
        let overall_pb = multi_progress.add(ProgressBar::new(files.len() as u64));
        overall_pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files ({eta})")?
            .progress_chars("#>-"));
        
        // Process files in parallel
        use rayon::prelude::*;
        files.par_iter()
            .map(|file| {
                // Individual file processing with progress tracking
                let file_pb = multi_progress.add(ProgressBar::new(100));
                file_pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.blue} {wide_msg}")?
                    .progress_chars("#>-"));
                
                file_pb.set_message(format!("Processing {}", file.display()));
                
                // Simulate conversion process
                for i in 0..100 {
                    std::thread::sleep(Duration::from_millis(10));
                    file_pb.set_position(i);
                }
                
                file_pb.finish_with_message(format!("✅ Converted {}", file.display()));
                overall_pb.inc(1);
                
                Ok(())
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        
        overall_pb.finish_with_message("🎉 Batch conversion completed!");
        
    } else {
        // Single file processing
        let pb = ProgressBar::new(100);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}% {wide_msg}")?
            .progress_chars("#>-"));
        
        pb.set_message("Initializing conversion...");
        
        // Simulate conversion steps
        let steps = vec![
            "Loading file...",
            "Detecting format...",
            "Loading plugins...",
            "Processing content...",
            "Applying transformations...",
            "Generating output...",
            "Saving results...",
        ];
        
        for (i, step) in steps.iter().enumerate() {
            pb.set_message(step.to_string());
            pb.set_position((i * 100 / steps.len()) as u64);
            sleep(Duration::from_millis(500)).await;
        }
        
        pb.finish_with_message("✅ Conversion completed successfully!");
    }
    
    Ok(())
}

async fn main_menu() -> anyhow::Result<()> {
    loop {
        let options = vec![
            "🔄 Convert Files",
            "🧙 Conversion Wizard", 
            "📦 Batch Processing",
            "🔌 Manage Plugins",
            "🐛 Debug Tools",
            "⚙️  Settings",
            "❌ Exit",
        ];
        
        let selection = Select::new()
            .with_prompt("Select an option")
            .items(&options)
            .default(0)
            .interact()?;
        
        match selection {
            0 => { /* Convert Files */ }
            1 => { conversion_wizard().await?; }
            2 => { /* Batch Processing */ }
            3 => { plugin_management_menu().await?; }
            4 => { /* Debug Tools */ }
            5 => { /* Settings */ }
            6 => break,
            _ => {}
        }
    }
    
    Ok(())
}

async fn plugin_management_menu() -> anyhow::Result<()> {
    let plugins = discover_plugins().await?;
    
    #[derive(Tabled)]
    struct PluginInfo {
        name: String,
        version: String,
        status: String,
        description: String,
    }
    
    let plugin_data: Vec<PluginInfo> = plugins.iter().map(|p| PluginInfo {
        name: p.name.clone(),
        version: p.version.clone(),
        status: if p.enabled { "✅ Enabled".to_string() } else { "❌ Disabled".to_string() },
        description: p.description.clone(),
    }).collect();
    
    let table = Table::new(plugin_data);
    println!("\n{}", table);
    
    let actions = vec![
        "Install new plugin",
        "Enable/Disable plugin", 
        "Remove plugin",
        "Test plugin",
        "Back to main menu",
    ];
    
    let selection = Select::new()
        .with_prompt("Plugin management action")
        .items(&actions)
        .interact()?;
    
    match selection {
        0 => { install_plugin().await?; }
        1 => { toggle_plugin().await?; }
        2 => { remove_plugin().await?; }
        3 => { test_plugin().await?; }
        4 => { return Ok(()); }
        _ => {}
    }
    
    Ok(())
}

#[derive(Debug)]
struct PluginMetadata {
    name: String,
    version: String,
    enabled: bool,
    description: String,
}

async fn discover_plugins() -> anyhow::Result<Vec<PluginMetadata>> {
    // Mock plugin discovery
    Ok(vec![
        PluginMetadata {
            name: "Roll20.dll".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            description: "Roll20 platform format plugin".to_string(),
        },
        PluginMetadata {
            name: "Foundry.dll".to_string(),
            version: "1.2.0".to_string(),
            enabled: true,
            description: "FoundryVTT platform format plugin".to_string(),
        },
        PluginMetadata {
            name: "PDF.dll".to_string(),
            version: "0.9.0".to_string(),
            enabled: false,
            description: "PDF export plugin".to_string(),
        },
    ])
}

async fn discover_convertible_files(dir: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    // Mock file discovery
    Ok(vec![
        dir.join("character1.json"),
        dir.join("character2.json"),
        dir.join("campaign.db"),
    ])
}
```

---

## **T6.2: Batch Processing Engine**
**Duration**: 4 days | **Points**: 6 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: Batch Configuration System**
Create `src/batch/mod.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchConfiguration {
    pub name: String,
    pub description: Option<String>,
    pub jobs: Vec<BatchJob>,
    pub global_settings: GlobalSettings,
    pub notifications: NotificationSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchJob {
    pub name: String,
    pub input_pattern: String,
    pub output_directory: PathBuf,
    pub target_format: String,
    pub processing_options: ProcessingOptions,
    pub filters: Vec<FileFilter>,
    pub retry_policy: RetryPolicy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingOptions {
    pub parallel_threads: usize,
    pub memory_limit_mb: usize,
    pub timeout_seconds: u64,
    pub validation_level: String,
    pub plugins: Vec<String>,
    pub custom_settings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileFilter {
    pub filter_type: FilterType,
    pub pattern: String,
    pub action: FilterAction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FilterType {
    Extension,
    FileName,
    FileSize,
    ModifiedDate,
    ContentPattern,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FilterAction {
    Include,
    Exclude,
    Transform(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub delay_seconds: u64,
    pub exponential_backoff: bool,
}

impl Default for BatchConfiguration {
    fn default() -> Self {
        Self {
            name: "Default Batch Job".to_string(),
            description: None,
            jobs: vec![],
            global_settings: GlobalSettings::default(),
            notifications: NotificationSettings::default(),
        }
    }
}

impl BatchConfiguration {
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: BatchConfiguration = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &PathBuf) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Create a sample batch configuration for new users
    pub fn create_sample() -> Self {
        Self {
            name: "Sample TTRPG Batch Conversion".to_string(),
            description: Some("Convert all Roll20 character files to FoundryVTT format".to_string()),
            jobs: vec![
                BatchJob {
                    name: "Character Conversion".to_string(),
                    input_pattern: "characters/**/*.json".to_string(),
                    output_directory: PathBuf::from("output/foundry/characters"),
                    target_format: "foundry".to_string(),
                    processing_options: ProcessingOptions {
                        parallel_threads: 4,
                        memory_limit_mb: 2048,
                        timeout_seconds: 300,
                        validation_level: "standard".to_string(),
                        plugins: vec!["validation".to_string(), "roll20".to_string(), "foundry".to_string()],
                        custom_settings: HashMap::new(),
                    },
                    filters: vec![
                        FileFilter {
                            filter_type: FilterType::Extension,
                            pattern: "json".to_string(),
                            action: FilterAction::Include,
                        },
                        FileFilter {
                            filter_type: FilterType::FileSize,
                            pattern: "100MB".to_string(),
                            action: FilterAction::Exclude,
                        }
                    ],
                    retry_policy: RetryPolicy {
                        max_attempts: 3,
                        delay_seconds: 5,
                        exponential_backoff: true,
                    },
                }
            ],
            global_settings: GlobalSettings::default(),
            notifications: NotificationSettings::default(),
        }
    }
}

pub struct BatchProcessor {
    config: BatchConfiguration,
    progress_reporter: BatchProgressReporter,
}

impl BatchProcessor {
    pub fn new(config: BatchConfiguration) -> Self {
        Self {
            config,
            progress_reporter: BatchProgressReporter::new(),
        }
    }
    
    pub async fn execute(&mut self, dry_run: bool) -> anyhow::Result<BatchResult> {
        let mut results = BatchResult::new();
        
        println!("🚀 Starting batch processing: {}", self.config.name);
        
        for (job_idx, job) in self.config.jobs.iter().enumerate() {
            println!("\n📋 Processing job {} of {}: {}", 
                job_idx + 1, self.config.jobs.len(), job.name);
            
            let job_result = self.execute_job(job, dry_run).await?;
            results.job_results.push(job_result);
        }
        
        self.generate_report(&results).await?;
        
        Ok(results)
    }
    
    async fn execute_job(&mut self, job: &BatchJob, dry_run: bool) -> anyhow::Result<JobResult> {
        let mut job_result = JobResult::new(job.name.clone());
        
        // Discover input files
        let input_files = self.discover_files(&job.input_pattern, &job.filters).await?;
        println!("Found {} files matching pattern", input_files.len());
        
        if dry_run {
            println!("🔍 DRY RUN - Would process {} files", input_files.len());
            for file in &input_files {
                println!("  📄 {}", file.display());
            }
            job_result.files_processed = input_files.len();
            return Ok(job_result);
        }
        
        // Process files with progress tracking
        let progress_bar = indicatif::ProgressBar::new(input_files.len() as u64);
        progress_bar.set_style(indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files")?);
        
        // Parallel processing with thread pool
        use rayon::prelude::*;
        let processing_results: Vec<FileProcessingResult> = input_files
            .par_iter()
            .map(|file| {
                let result = self.process_single_file(file, job);
                progress_bar.inc(1);
                result
            })
            .collect();
        
        progress_bar.finish_with_message("✅ Job completed");
        
        // Aggregate results
        for result in processing_results {
            match result {
                Ok(_) => job_result.files_processed += 1,
                Err(e) => {
                    job_result.errors.push(e.to_string());
                    job_result.files_failed += 1;
                }
            }
        }
        
        Ok(job_result)
    }
}

#[derive(Debug)]
pub struct BatchResult {
    pub job_results: Vec<JobResult>,
    pub total_files_processed: usize,
    pub total_files_failed: usize,
    pub execution_time: std::time::Duration,
}

impl BatchResult {
    pub fn new() -> Self {
        Self {
            job_results: vec![],
            total_files_processed: 0,
            total_files_failed: 0,
            execution_time: std::time::Duration::from_secs(0),
        }
    }
}
```

---

## **T6.3: Workflow Automation System**
**Duration**: 4 days | **Points**: 3 | **Priority**: 🔥 HIGH

Pipeline scripting and job scheduling for automated workflows.

---

## **T6.4: Developer Utilities**
**Duration**: 3 days | **Points**: 2 | **Priority**: 🔥 HIGH

Plugin development tools, debugging utilities, and validation helpers.

### **Implementation Steps**

**Step 1: Plugin Development Kit**
Create `src/dev_tools/plugin_dev.rs`:
```rust
use std::path::PathBuf;
use serde_json;

pub struct PluginDevKit;

impl PluginDevKit {
    /// Generate plugin template with boilerplate code
    pub async fn create_plugin_template(
        &self,
        name: &str,
        plugin_type: PluginType,
        output_dir: &PathBuf
    ) -> anyhow::Result<()> {
        let template_dir = output_dir.join(format!("ttrpg-{}-plugin", name.to_lowercase()));
        std::fs::create_dir_all(&template_dir)?;
        
        // Generate Cargo.toml
        let cargo_toml = self.generate_cargo_toml(name, plugin_type)?;
        std::fs::write(template_dir.join("Cargo.toml"), cargo_toml)?;
        
        // Generate lib.rs template
        let lib_rs = self.generate_lib_template(name, plugin_type)?;
        let src_dir = template_dir.join("src");
        std::fs::create_dir_all(&src_dir)?;
        std::fs::write(src_dir.join("lib.rs"), lib_rs)?;
        
        // Generate example tests
        let test_code = self.generate_test_template(name)?;
        std::fs::write(src_dir.join("lib.rs"), test_code)?;
        
        println!("✅ Plugin template created at: {}", template_dir.display());
        
        Ok(())
    }
    
    fn generate_cargo_toml(&self, name: &str, plugin_type: PluginType) -> anyhow::Result<String> {
        let template = match plugin_type {
            PluginType::PlatformFormat => include_str!("templates/platform_plugin_cargo.toml"),
            PluginType::Processing => include_str!("templates/processing_plugin_cargo.toml"),
            PluginType::Output => include_str!("templates/output_plugin_cargo.toml"),
        };
        
        Ok(template.replace("{{PLUGIN_NAME}}", name))
    }
    
    /// Test plugin functionality
    pub async fn test_plugin(&self, plugin_path: &PathBuf) -> anyhow::Result<TestResult> {
        println!("🧪 Testing plugin: {}", plugin_path.display());
        
        // Load plugin
        let plugin = self.load_plugin_for_testing(plugin_path).await?;
        
        // Run plugin tests
        let mut test_result = TestResult::new();
        
        // Test 1: Plugin loading
        test_result.add_test("Plugin Loading", self.test_plugin_loading(&plugin).await);
        
        // Test 2: Interface compliance
        test_result.add_test("Interface Compliance", self.test_interface_compliance(&plugin).await);
        
        // Test 3: Basic functionality
        test_result.add_test("Basic Functionality", self.test_basic_functionality(&plugin).await);
        
        // Test 4: Error handling
        test_result.add_test("Error Handling", self.test_error_handling(&plugin).await);
        
        test_result.print_summary();
        
        Ok(test_result)
    }
}

#[derive(Debug)]
pub enum PluginType {
    PlatformFormat,
    Processing,
    Output,
}

#[derive(Debug)]
pub struct TestResult {
    tests: Vec<(String, TestOutcome)>,
}

#[derive(Debug)]
pub enum TestOutcome {
    Passed,
    Failed(String),
    Skipped(String),
}

impl TestResult {
    pub fn new() -> Self {
        Self { tests: vec![] }
    }
    
    pub fn add_test(&mut self, name: &str, outcome: TestOutcome) {
        self.tests.push((name.to_string(), outcome));
    }
    
    pub fn print_summary(&self) {
        println!("\n📊 Test Summary");
        println!("═══════════════");
        
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;
        
        for (name, outcome) in &self.tests {
            match outcome {
                TestOutcome::Passed => {
                    println!("✅ {}", name);
                    passed += 1;
                }
                TestOutcome::Failed(reason) => {
                    println!("❌ {} - {}", name, reason);
                    failed += 1;
                }
                TestOutcome::Skipped(reason) => {
                    println!("⏭️  {} - {}", name, reason);
                    skipped += 1;
                }
            }
        }
        
        println!("\nResults: {} passed, {} failed, {} skipped", passed, failed, skipped);
    }
}
```

### **Success Criteria**
- [ ] ✅ Rich CLI interface with progress bars, interactive menus, and colored output
- [ ] ✅ Batch processing engine handles multiple files with parallel processing
- [ ] ✅ Workflow automation supports pipeline scripting and job scheduling
- [ ] ✅ Plugin development kit generates templates and testing utilities
- [ ] ✅ Developer tools include debugging, validation, and profiling capabilities
- [ ] ✅ CLI tools integrate seamlessly with existing plugin architecture
