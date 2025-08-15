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

// Plugin orchestration modules (M2.0B Foundation)
pub mod discovery;
pub mod injection;
pub mod pipeline;

// Plugin framework core modules
pub mod interfaces;
pub mod manager;
pub mod registry;
pub mod types;

// Export plugin interface traits (new plugin system)
pub use interfaces::{
    AssetCacheStats, AssetPlugin, AssetProcessingOptions, AssetValidationResult, ExportPlugin,
    InputPlugin, LogLevel, LoggingPlugin, LoggingStats, PluginConfig, ValidationPlugin,
};

// Export core plugin types (existing system, keep for compatibility)
pub use types::{
    AssetInfo, AssetMetadata, AssetType, CampaignSettings, GameSystem, OutputBundle, OutputConfig,
    OutputFormat, OutputMetadata, ProcessedAsset, SourceFormat, WriteOptions,
};

// Export plugin management
pub use discovery::{
    create_discovery_info, DiscoveryConfig, DiscoveryStats, PluginCategory, PluginDiscovery,
    PluginDiscoveryInfo, StaticPluginCategory, StaticPluginDiscoveryInfo, StaticPluginRegistration,
};
pub use injection::{
    DIContainerConfig, DIContainerStats, PluginDIContainer, PluginHealth, PluginLifecycle,
    PluginRegistration, PluginRegistrationBuilder, PluginServiceType,
};
pub use manager::PluginManager;
pub use pipeline::{
    PipelineBuilder, PipelineConfig, PipelineContext, PipelineNode, PipelinePluginType,
    PipelineStats, PluginPipeline,
};
pub use registry::{InputPluginRegistry, OutputPluginRegistry};

// Re-export specific types that need to be public
pub use self::types::{CampaignMetadata, UniversalCampaign};

// InputPlugin trait moved to interfaces.rs for proper architectural separation

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

// PluginInfo moved to interfaces.rs for proper architectural separation
// Re-export from interfaces for backward compatibility
pub use interfaces::{PluginInfo, StaticPluginInfo};

// Core service plugin implementations moved to appropriate plugin crates:
// - ConsoleLoggingPlugin → ttrpg-output-plugins/logging/console.rs
// - DefaultValidationPlugin → ttrpg-processing-plugins/validation/core.rs

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
