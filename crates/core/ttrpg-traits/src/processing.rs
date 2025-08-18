//! Processing plugin trait for transforming campaign data

use async_trait::async_trait;
use ttrpg_core::prelude::*;
use ttrpg_types::Campaign;
use super::{PluginInfo, PluginConfig};

/// Processing pipeline stages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessingStage {
    /// Parse input format
    Parse,
    /// Validate parsed data
    Validate,
    /// Transform/normalize data
    Transform,
    /// Optimize data structure
    Optimize,
    /// Generate output format
    Generate,
    /// Post-process output
    PostProcess,
}

/// Processing context and options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingContext {
    /// Current processing stage
    pub stage: ProcessingStage,
    /// Source format information
    pub source_format: String,
    /// Target format information
    pub target_format: Option<String>,
    /// Processing options
    pub options: ProcessingOptions,
}

/// Processing configuration options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessingOptions {
    /// Custom processing parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Enable verbose logging
    pub verbose: bool,
    /// Maximum processing time in seconds
    pub timeout_secs: Option<u64>,
    /// Parallel processing options
    pub parallel: bool,
}

/// Processing result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    /// Processed campaign data
    pub campaign: Campaign,
    /// Processing statistics
    pub stats: ProcessingStats,
    /// Processing messages/warnings
    pub messages: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Processing performance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessingStats {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Number of entities processed
    pub entities_processed: usize,
    /// Number of transformations applied
    pub transformations_applied: usize,
    /// Memory usage in bytes
    pub peak_memory_usage: Option<u64>,
}

/// Processing plugin trait
#[async_trait]
pub trait ProcessingPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Get supported processing stages
    fn get_supported_stages(&self) -> Vec<ProcessingStage>;

    /// Check if plugin can process this campaign in given context
    async fn can_process(
        &self,
        campaign: &Campaign,
        context: &ProcessingContext,
    ) -> ConversionResult<bool>;

    /// Process campaign data through transformation pipeline
    async fn process_campaign(
        &self,
        campaign: Campaign,
        context: ProcessingContext,
    ) -> ConversionResult<ProcessingResult>;

    /// Validate processing context and options
    fn validate_context(&self, context: &ProcessingContext) -> ConversionResult<()>;

    /// Get processing statistics
    fn get_stats(&self) -> ProcessingStats;

    /// Get default processing options for a stage
    fn get_default_options(&self, stage: ProcessingStage) -> ProcessingOptions;
}