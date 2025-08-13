//! Pure Plugin Ecosystem Foundation
//!
//! Revolutionary plugin infrastructure that replaces the entire service layer with
//! a unified plugin system. Every functionality is provided through plugins:
//! Input, Output, Asset, Validation, Logging, and Export.
//!
//! # Architecture
//!
//! The pure plugin ecosystem consists of:
//! - **InputPlugin**: Handles parsing different campaign formats (Roll20, Foundry, etc.)
//! - **OutputPlugin**: Generates output in various formats (Foundry, Pathbuilder, PDF, etc.)
//! - **AssetPlugin**: Comprehensive asset discovery and processing
//! - **ValidationPlugin**: Data validation and integrity checking  
//! - **LoggingPlugin**: Flexible logging strategies
//! - **ExportPlugin**: Advanced export capabilities
//! - **PluginManager**: Unified plugin registration, discovery, and coordination
//! - **UniversalCampaign**: Common data structure that all plugins work with
//!
//! # Revolutionary Benefits
//!
//! - **Ultimate Extensibility**: Everything is swappable and extensible
//! - **Linear Complexity**: Adding new formats requires N+M work, not N×M
//! - **Plugin Ecosystem**: Complete plugin marketplace potential
//! - **Dynamic CLI**: Runtime plugin discovery and switching

use async_trait::async_trait;
// HashMap import moved to specific functions that need it
use crate::{ConversionError, ConversionResult};
use std::path::Path;

pub mod interfaces;
pub mod manager;
pub mod registry;
pub mod types;

// Export plugin interface traits (new plugin system)
pub use interfaces::{
    AssetCacheStats, AssetPlugin, AssetProcessingOptions, AssetValidationResult, ExportPlugin,
    LogLevel, LoggingPlugin, LoggingStats, PluginConfig, ValidationPlugin,
};

// Export core plugin types (existing system, keep for compatibility)
pub use types::{
    AssetInfo, AssetMetadata, AssetType, CampaignSettings, GameSystem, OutputBundle, OutputConfig,
    OutputFormat, OutputMetadata, ProcessedAsset, SourceFormat, WriteOptions,
};

// Export plugin management
pub use manager::PluginManager;
pub use registry::{InputPluginRegistry, OutputPluginRegistry};

// Re-export specific types that need to be public
pub use self::types::{CampaignMetadata, UniversalCampaign};

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

/// Core trait for output format plugins that generate campaign data
///
/// Output plugins are responsible for:
/// - Converting UniversalCampaign to target format
/// - Handling format-specific configuration options
/// - Writing output files and databases
/// - Managing asset processing and mapping
#[async_trait]
pub trait OutputPlugin: Send + Sync {
    /// Plugin identification and capabilities
    fn plugin_info(&self) -> PluginInfo;

    /// List of output formats this plugin supports
    fn supported_formats(&self) -> Vec<OutputFormat>;

    /// Generate output bundle from universal campaign data
    ///
    /// # Arguments
    /// * `campaign` - Universal campaign data to convert
    /// * `assets` - Processed assets ready for output
    /// * `config` - Output-specific configuration
    ///
    /// # Returns
    /// * `OutputBundle` containing all generated files and data
    ///
    /// # Errors
    /// * Returns `ConversionError` if output generation fails
    async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        assets: &[ProcessedAsset],
        config: &OutputConfig,
    ) -> ConversionResult<OutputBundle>;

    /// Write output bundle to target location
    ///
    /// # Arguments
    /// * `bundle` - Generated output bundle
    /// * `target` - Target directory path
    /// * `options` - Write options and preferences
    ///
    /// # Returns
    /// * `()` on successful write
    ///
    /// # Errors
    /// * Returns `ConversionError` if writing fails  
    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        options: &WriteOptions,
    ) -> ConversionResult<()>;

    /// Validate that output configuration is valid for this plugin
    ///
    /// # Arguments
    /// * `config` - Output configuration to validate
    ///
    /// # Returns
    /// * `()` if configuration is valid
    ///
    /// # Errors
    /// * Returns `ConversionError` if configuration is invalid
    fn validate_config(&self, config: &OutputConfig) -> ConversionResult<()> {
        if !self.supported_formats().contains(&config.format) {
            return Err(Box::new(ConversionError::UnsupportedOutputFormat(config.format.clone())));
        }
        Ok(())
    }
}

/// Plugin identification and metadata
#[derive(Debug, Clone, PartialEq)]
pub struct PluginInfo {
    /// Plugin name (e.g., "Roll20 Input", "Foundry VTT Output")
    pub name: String,
    /// Plugin version (semantic versioning)
    pub version: String,
    /// Human-readable description
    pub description: String,
    /// Plugin author/maintainer
    pub author: String,
    /// List of supported features/capabilities
    pub supported_features: Vec<String>,
}

impl PluginInfo {
    /// Create new plugin info
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        author: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: author.into(),
            supported_features: Vec::new(),
        }
    }

    /// Add a supported feature to the plugin info
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.supported_features.push(feature.into());
        self
    }

    /// Add multiple supported features
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.supported_features.extend(features);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_info_creation() {
        let plugin_info = PluginInfo::new("Test Plugin", "1.0.0", "A test plugin", "Test Author")
            .with_feature("test_feature")
            .with_features(vec!["feature1".to_string(), "feature2".to_string()]);

        assert_eq!(plugin_info.name, "Test Plugin");
        assert_eq!(plugin_info.version, "1.0.0");
        assert_eq!(plugin_info.description, "A test plugin");
        assert_eq!(plugin_info.author, "Test Author");
        assert_eq!(plugin_info.supported_features.len(), 3);
        assert!(plugin_info
            .supported_features
            .contains(&"test_feature".to_string()));
    }
}
