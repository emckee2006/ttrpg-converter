//! Output plugin trait for generating campaign formats

use async_trait::async_trait;
use std::path::Path;
use ttrpg_core::prelude::*;
use ttrpg_types::Campaign;
use super::PluginInfo;

/// Output bundle containing generated data and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputBundle {
    /// Output format
    pub format: TargetFormat,
    /// Generated files (filename -> content)
    pub files: HashMap<String, Vec<u8>>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Output configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Target format
    pub format: TargetFormat,
    /// Platform-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Asset handling options
    pub copy_assets: bool,
    /// Asset directory path
    pub asset_dir: Option<String>,
}

/// Write options for output operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOptions {
    /// Overwrite existing files
    pub overwrite: bool,
    /// Create backup of existing files
    pub backup: bool,
    /// Validate output after writing
    pub validate: bool,
}

/// Output plugin trait for generating campaign formats
#[async_trait]
pub trait OutputPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;
    
    /// List of output formats this plugin supports
    fn supported_formats(&self) -> Vec<TargetFormat>;
    
    /// Generate output bundle from campaign data
    async fn generate_output(
        &self,
        campaign: &Campaign,
        assets: &[super::input::AssetInfo],
        config: &OutputConfig,
    ) -> ConversionResult<OutputBundle>;
    
    /// Write output bundle to target location
    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        options: &WriteOptions,
    ) -> ConversionResult<()>;
    
    /// Validate output configuration
    fn validate_config(&self, config: &OutputConfig) -> ConversionResult<()> {
        if !self.supported_formats().contains(&config.format) {
            return Err(Box::new(ConversionError::UnsupportedOutputFormat(config.format.to_string())));
        }
        Ok(())
    }
}