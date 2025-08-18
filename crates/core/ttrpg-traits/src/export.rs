//! Export plugin trait for specialized data exports

use async_trait::async_trait;
use std::path::Path;
use ttrpg_core::prelude::*;
use ttrpg_types::Campaign;
use super::{PluginInfo, PluginConfig};

/// Export format specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFormat {
    /// Format identifier
    pub format_id: String,
    /// Human-readable name
    pub display_name: String,
    /// File extensions supported
    pub extensions: Vec<String>,
    /// MIME type
    pub mime_type: String,
    /// Format description
    pub description: String,
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Target format
    pub format: ExportFormat,
    /// Include asset files
    pub include_assets: bool,
    /// Asset handling options
    pub asset_handling: AssetHandling,
    /// Compression settings
    pub compression: Option<CompressionConfig>,
    /// Custom parameters
    pub custom_params: HashMap<String, serde_json::Value>,
}

/// Asset handling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetHandling {
    /// Embed assets in export
    Embed,
    /// Reference assets by path
    Reference,
    /// Copy assets to export directory
    Copy,
    /// Exclude assets entirely
    Exclude,
}

/// Compression configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    /// Compression algorithm
    pub algorithm: String,
    /// Compression level (0-9)
    pub level: u8,
}

/// Export result bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportBundle {
    /// Primary export file
    pub main_file: ExportFile,
    /// Additional files (assets, manifests, etc.)
    pub additional_files: Vec<ExportFile>,
    /// Export metadata
    pub metadata: ExportMetadata,
}

/// Individual export file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportFile {
    /// Relative file path
    pub path: String,
    /// File content
    pub content: Vec<u8>,
    /// Content type
    pub content_type: String,
}

/// Export operation metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportMetadata {
    /// Export timestamp
    pub exported_at: String,
    /// Source campaign name
    pub campaign_name: String,
    /// Export format used
    pub format_info: ExportFormat,
    /// Total file count
    pub file_count: usize,
    /// Total export size in bytes
    pub total_size: u64,
}

/// Export plugin trait
#[async_trait]
pub trait ExportPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Get list of supported export formats
    fn get_supported_formats(&self) -> Vec<ExportFormat>;

    /// Validate export configuration
    fn validate_config(&self, config: &ExportConfig) -> ConversionResult<()>;

    /// Check if plugin can export this campaign
    async fn can_export_campaign(&self, campaign: &Campaign) -> ConversionResult<bool>;

    /// Export campaign to specified format
    async fn export_campaign(
        &self,
        campaign: &Campaign,
        assets: &[super::input::AssetInfo],
        config: &ExportConfig,
    ) -> ConversionResult<ExportBundle>;

    /// Write export bundle to target directory
    async fn write_export(
        &self,
        bundle: ExportBundle,
        target_dir: &Path,
    ) -> ConversionResult<()>;

    /// Get default export configuration for a format
    fn get_default_config(&self, format: &ExportFormat) -> ConversionResult<ExportConfig>;
}