//! Pure Plugin Ecosystem - Core Plugin Interfaces
//!
//! Revolutionary plugin architecture that replaces the service layer with a unified
//! plugin system. Every functionality is provided through plugins with a clean,
//! extensible interface.

use crate::error::{AssetResult, ConversionResult};
use crate::plugin_framework::types::{CampaignMetadata, UniversalCampaign};
use crate::types::{AssetReference, Campaign, TargetFormat};
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Core plugin information and metadata (runtime version)
#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub supported_features: Vec<String>,
    pub dependencies: Vec<String>,
}

/// Const-compatible plugin information for inventory registration
#[derive(Debug, Clone, Copy)]
pub struct StaticPluginInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub supported_features: &'static [&'static str],
    pub dependencies: &'static [&'static str],
}

impl From<StaticPluginInfo> for PluginInfo {
    fn from(static_info: StaticPluginInfo) -> Self {
        Self {
            name: static_info.name.to_string(),
            version: static_info.version.to_string(),
            description: static_info.description.to_string(),
            author: static_info.author.to_string(),
            supported_features: static_info
                .supported_features
                .iter()
                .map(|s| s.to_string())
                .collect(),
            dependencies: static_info
                .dependencies
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

impl PluginInfo {
    /// Create a new PluginInfo with basic metadata
    pub fn new(name: &str, version: &str, description: &str, author: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            supported_features: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Add a single feature to the supported features list (builder pattern)
    pub fn with_feature(mut self, feature: &str) -> Self {
        self.supported_features.push(feature.to_string());
        self
    }

    /// Add multiple features to the supported features list (builder pattern)
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.supported_features.extend(features);
        self
    }

    /// Add a single dependency (builder pattern)
    pub fn with_dependency(mut self, dependency: &str) -> Self {
        self.dependencies.push(dependency.to_string());
        self
    }

    /// Add multiple dependencies (builder pattern)
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies.extend(dependencies);
        self
    }
}

/// Plugin initialization configuration
#[derive(Debug, Clone, Default)]
pub struct PluginConfig {
    pub config_data: HashMap<String, JsonValue>,
    pub cache_dir: Option<PathBuf>,
    pub temp_dir: Option<PathBuf>,
}

/// Logging plugin interface - replaces LoggingService
#[async_trait]
pub trait LoggingPlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Log an error message
    fn error(&self, message: &str, context: Option<&str>);

    /// Log a warning message  
    fn warn(&self, message: &str, context: Option<&str>);

    /// Log an info message
    fn info(&self, message: &str, context: Option<&str>);

    /// Log a debug message
    fn debug(&self, message: &str, context: Option<&str>);

    /// Log with structured data
    fn log_with_data(&self, level: LogLevel, message: &str, data: &JsonValue);

    /// Set the minimum log level
    fn set_level(&mut self, level: LogLevel);

    /// Check if a log level is enabled
    fn is_enabled(&self, level: LogLevel) -> bool;

    /// Get logging statistics
    fn get_stats(&self) -> LoggingStats;
}

/// Log levels (copied from services.rs for compatibility)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

/// Logging plugin statistics
#[derive(Debug, Clone, Default)]
pub struct LoggingStats {
    pub messages_logged: usize,
    pub errors_logged: usize,
    pub warnings_logged: usize,
    pub start_time: Option<std::time::Instant>,
}

/// Validation plugin interface - replaces ValidationService
#[async_trait]
pub trait ValidationPlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Validate a complete campaign
    async fn validate_campaign(&self, campaign: &Campaign) -> ConversionResult<ValidationResult>;

    /// Validate campaign file path and format
    async fn validate_file_path(&self, path: &Path) -> ConversionResult<ValidationResult>;

    /// Validate specific data format compatibility  
    async fn validate_format(
        &self,
        format: &str,
        data: &JsonValue,
    ) -> ConversionResult<ValidationResult>;

    /// Get validation capabilities and supported formats
    fn get_supported_formats(&self) -> Vec<String>;

    /// Get validation statistics
    fn get_stats(&self) -> ValidationStats;
}

/// Asset plugin interface - replaces AssetService
#[async_trait]
pub trait AssetPlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Discover assets from campaign data
    async fn discover_assets(
        &self,
        campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>>;

    /// Download and cache asset from URL
    async fn download_asset(&self, asset_ref: &AssetReference) -> AssetResult<PathBuf>;

    /// Process and optimize asset (resize, convert format, etc.)
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

/// Export plugin interface - replaces ExportService
#[async_trait]
pub trait ExportPlugin: Send + Sync {
    /// Plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Export campaign to target format
    async fn export_campaign(
        &self,
        campaign: &UniversalCampaign,
        target: &Path,
        options: &ExportOptions,
    ) -> ConversionResult<ExportResult>;

    /// Export with template system support
    async fn export_with_template(
        &self,
        campaign: &UniversalCampaign,
        template_path: &Path,
        target: &Path,
        options: &ExportOptions,
    ) -> ConversionResult<ExportResult>;

    /// Preview export (dry run)
    async fn preview_export(
        &self,
        campaign: &UniversalCampaign,
        options: &ExportOptions,
    ) -> ConversionResult<ExportPreview>;

    /// Get supported export formats
    fn get_supported_formats(&self) -> Vec<TargetFormat>;

    /// Get export statistics
    fn get_stats(&self) -> ExportStats;
}

/// Re-export types from plugins::types for pure plugin ecosystem
pub use super::types::{
    AssetInfo, AssetResolution, AssetStats, ExportOptions, ExportPreview, ExportResult,
    ExportStats, IssueSeverity, ValidationIssue, ValidationResult, ValidationStats,
};

/// Asset processing configuration
#[derive(Debug, Clone, Default)]
pub struct AssetProcessingOptions {
    pub resize: Option<(u32, u32)>,
    pub quality: Option<u8>,
    pub format_conversion: Option<String>,
    pub optimize: bool,
    pub preserve_metadata: bool,
}

/// Core trait for input format plugins that parse campaign data
///
/// Input plugins are responsible for:
/// - Detecting if they can handle a given source
/// - Parsing campaign data into UniversalCampaign format  
/// - Discovering and cataloging assets
/// - Extracting metadata for system detection
#[async_trait]
pub trait InputPlugin: Send + Sync {
    /// Plugin identification and capabilities
    fn plugin_info(&self) -> PluginInfo;

    /// Determine if this plugin can handle the given source
    ///
    /// # Arguments
    /// * `source` - Path to campaign file or directory
    ///
    /// # Returns
    /// * `true` if this plugin can parse the source format
    fn can_handle(&self, source: &Path) -> bool;

    /// Parse campaign data from source into universal format
    ///
    /// # Arguments  
    /// * `source` - Path to campaign file or directory
    ///
    /// # Returns
    /// * `UniversalCampaign` with all parsed data
    ///
    /// # Errors
    /// * Returns `ConversionError` if parsing fails
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<UniversalCampaign>;

    /// Discover all assets referenced in the campaign
    ///
    /// # Arguments
    /// * `campaign` - Parsed campaign data
    ///
    /// # Returns  
    /// * Vector of `AssetInfo` describing all discovered assets
    ///
    /// # Errors
    /// * Returns `ConversionError` if asset discovery fails
    async fn discover_assets(
        &self,
        campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>>;

    /// Extract metadata for system detection and conversion planning
    ///
    /// # Arguments
    /// * `source` - Path to campaign file or directory
    ///
    /// # Returns
    /// * `CampaignMetadata` with system detection and other metadata
    ///
    /// # Errors  
    /// * Returns `ConversionError` if metadata extraction fails
    fn extract_metadata(&self, source: &Path) -> ConversionResult<CampaignMetadata>;
}

/// Processed asset information
#[derive(Debug, Clone)]
pub struct ProcessedAsset {
    pub original_path: PathBuf,
    pub processed_path: PathBuf,
    pub metadata: AssetMetadata,
    pub processing_stats: ProcessingStats,
}

/// Asset metadata
#[derive(Debug, Clone, Default)]
pub struct AssetMetadata {
    pub size: Option<u64>,
    pub dimensions: Option<(u32, u32)>,
    pub format: Option<String>,
    pub content_type: Option<String>,
    pub hash: Option<String>,
}

/// Asset processing statistics
#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    pub processing_time_ms: u64,
    pub size_reduction: Option<f64>,
    pub quality_retained: Option<f64>,
}

/// Asset validation result
#[derive(Debug, Clone)]
pub struct AssetValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub metadata: AssetMetadata,
}

/// Asset cache statistics
#[derive(Debug, Clone, Default)]
pub struct AssetCacheStats {
    pub cached_assets: usize,
    pub cache_size_bytes: u64,
    pub hit_rate: f64,
    pub last_cleanup: Option<std::time::SystemTime>,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_config_default() {
        let config = PluginConfig::default();
        assert!(config.config_data.is_empty());
        assert!(config.cache_dir.is_none());
        assert!(config.temp_dir.is_none());
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);
    }
}
