//! Asset plugin trait for handling game assets

use async_trait::async_trait;
use std::path::{Path, PathBuf};
use ttrpg_core::prelude::*;
use super::{PluginInfo, PluginConfig};

/// Asset processing options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetProcessingOptions {
    /// Resize dimensions (width, height)
    pub resize: Option<(u32, u32)>,
    /// Quality setting (0-100)
    pub quality: Option<u8>,
    /// Convert to specific format
    pub format_conversion: Option<String>,
    /// Optimize file size
    pub optimize: bool,
    /// Preserve metadata
    pub preserve_metadata: bool,
}

/// Processed asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedAsset {
    /// Original file path
    pub original_path: PathBuf,
    /// Processed file path
    pub processed_path: PathBuf,
    /// Asset metadata
    pub metadata: AssetMetadata,
    /// Processing statistics
    pub processing_stats: AssetProcessingStats,
}

/// Asset metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// File size in bytes
    pub size: Option<u64>,
    /// Image dimensions (width, height)
    pub dimensions: Option<(u32, u32)>,
    /// File format
    pub format: Option<String>,
    /// Content type
    pub content_type: Option<String>,
    /// Content hash
    pub hash: Option<String>,
}

/// Asset processing statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetProcessingStats {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Size reduction percentage
    pub size_reduction: Option<f64>,
    /// Quality retained percentage
    pub quality_retained: Option<f64>,
}

/// Asset validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetValidationResult {
    /// Whether the asset is valid
    pub is_valid: bool,
    /// Validation issues found
    pub issues: Vec<String>,
    /// Asset metadata
    pub metadata: AssetMetadata,
}

/// Asset cache statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetCacheStats {
    /// Number of cached assets
    pub cached_assets: usize,
    /// Cache size in bytes
    pub cache_size_bytes: u64,
    /// Cache hit rate
    pub hit_rate: f64,
    /// Last cleanup time
    pub last_cleanup: Option<std::time::SystemTime>,
}

/// Asset plugin trait
#[async_trait]
pub trait AssetPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Discover assets from campaign data
    async fn discover_assets(
        &self,
        campaign: &ttrpg_types::Campaign,
    ) -> ConversionResult<Vec<super::input::AssetInfo>>;

    /// Download and cache asset from URL
    async fn download_asset(&self, asset_ref: &str) -> AssetResult<PathBuf>;

    /// Process and optimize asset
    async fn process_asset(
        &self,
        input_path: &Path,
        options: &AssetProcessingOptions,
    ) -> AssetResult<ProcessedAsset>;

    /// Validate asset integrity and format
    async fn validate_asset(&self, asset_path: &Path) -> AssetResult<AssetValidationResult>;

    /// Get asset cache statistics
    fn get_cache_stats(&self) -> AssetCacheStats;

    /// Clear asset cache
    async fn clear_cache(&self) -> AssetResult<()>;

    /// Get supported asset types and formats
    fn get_supported_formats(&self) -> Vec<String>;
}