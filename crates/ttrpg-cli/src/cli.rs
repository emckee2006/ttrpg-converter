//! Enhanced CLI interface with plugin orchestration capabilities
//!
//! This module provides sophisticated command-line interface with:
//! - Interactive plugin selection and discovery
//! - Pipeline template management  
//! - Smart conversion workflows
//! - Professional progress visualization

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ttrpg-cli", version, about = "Convert TTRPG campaigns between formats with plugin orchestration")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
    
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(long)]
    pub config: Option<PathBuf>,
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
        #[arg(short, long, help = "Input campaign file or directory")]
        input: PathBuf,
        
        #[arg(short, long, help = "Output directory or file")]
        output: PathBuf,
        
        #[arg(long, help = "Input format (Roll20, Foundry, FantasyGrounds)")]
        input_format: Option<String>,
        
        #[arg(short = 'f', long, help = "Output format")]
        output_format: Option<String>,
        
        #[arg(long, help = "Use pipeline template")]
        template: Option<String>,
        
        #[arg(long, help = "Interactive pipeline selection")]
        interactive: bool,
        
        #[arg(long, help = "Skip asset processing")]
        no_assets: bool,
        
        #[arg(long, help = "Force overwrite existing files")]
        force: bool,
    },
    
    /// ✅ Validate campaign data
    Validate {
        #[arg(short, long, help = "Input campaign file or directory")]
        input: PathBuf,
        
        #[arg(short, long, help = "Format to validate as")]
        format: Option<String>,
        
        #[arg(long, help = "Show detailed validation report")]
        detailed: bool,
        
        #[arg(long, help = "Fix common validation issues")]
        fix: bool,
    },
    
    /// ℹ️ Show information about files or plugins
    Info {
        #[arg(help = "Path to file or plugin ID")]
        target: String,
        
        #[arg(long, help = "Show technical details")]
        verbose: bool,
    },
    
    /// 🧪 Test pipeline configuration
    Test {
        #[arg(short, long, help = "Template or pipeline configuration")]
        template: Option<String>,
        
        #[arg(short, long, help = "Dry run without actual processing")]
        dry_run: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// List all available plugins
    List {
        #[arg(short, long, help = "Filter by plugin type")]
        plugin_type: Option<String>,
        
        #[arg(short, long, help = "Show detailed plugin information")]
        verbose: bool,
    },
    
    /// Interactive plugin selection for pipeline building
    Select {
        #[arg(short, long, help = "Input format")]
        input_format: String,
        
        #[arg(short, long, help = "Output format")]
        output_format: String,
        
        #[arg(long, help = "Save pipeline as template")]
        save_template: Option<String>,
    },
    
    /// Show detailed plugin information
    Info {
        #[arg(help = "Plugin ID to show information for")]
        plugin_id: String,
    },
    
    /// Validate plugin compatibility
    Validate {
        #[arg(help = "Plugin IDs to validate together")]
        plugin_ids: Vec<String>,
    },
    
    /// Discover and register new plugins
    Discover {
        #[arg(short, long, help = "Search directory for plugins")]
        path: Option<PathBuf>,
        
        #[arg(long, help = "Force re-discovery of plugins")]
        force_refresh: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// List available pipeline templates
    List {
        #[arg(short, long, help = "Show only built-in templates")]
        builtin: bool,
        
        #[arg(short, long, help = "Show detailed template information")]
        verbose: bool,
    },
    
    /// Save current pipeline as template
    Save {
        #[arg(help = "Pipeline ID to save")]
        pipeline_id: String,
        
        #[arg(help = "Template name")]
        template_name: String,
        
        #[arg(short, long, help = "Template description")]
        description: Option<String>,
    },
    
    /// Load pipeline from template
    Load {
        #[arg(help = "Template name to load")]
        template_name: String,
        
        #[arg(short, long, help = "Override input format")]
        override_input: Option<String>,
        
        #[arg(short, long, help = "Override output format")]
        override_output: Option<String>,
    },
    
    /// Delete pipeline template
    Delete {
        #[arg(help = "Template name to delete")]
        template_name: String,
        
        #[arg(short, long, help = "Skip confirmation prompt")]
        confirm: bool,
    },
    
    /// Show template details
    Info {
        #[arg(help = "Template name to show information for")]
        template_name: String,
    },
    
    /// Export template to file
    Export {
        #[arg(help = "Template name to export")]
        template_name: String,
        
        #[arg(short, long, help = "Output file path")]
        output: PathBuf,
    },
    
    /// Import template from file
    Import {
        #[arg(help = "Template file to import")]
        file: PathBuf,
        
        #[arg(short, long, help = "Override template name")]
        name: Option<String>,
    },
}

impl Cli {
    /// Get the logging level based on verbosity flags
    pub fn log_level(&self) -> tracing::Level {
        if self.quiet {
            tracing::Level::ERROR
        } else {
            match self.verbose {
                0 => tracing::Level::INFO,
                1 => tracing::Level::DEBUG,
                _ => tracing::Level::TRACE,
            }
        }
    }
}
