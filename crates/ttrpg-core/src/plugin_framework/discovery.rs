//! Plugin Auto-Discovery with Inventory
//!
//! Automatic plugin discovery and registration system using the inventory crate
//! for compile-time plugin registration, runtime discovery, and dynamic loading.

use inventory::collect;
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::{debug, info, warn};

// Plugin interfaces are imported for potential future use in discovery system
// use super::interfaces::{AssetPlugin, ExportPlugin, InputPlugin, LoggingPlugin, ValidationPlugin};
use super::{PluginInfo, StaticPluginInfo};
use crate::{ConversionError, ConversionResult};

/// Plugin discovery metadata for inventory registration (runtime version)
#[derive(Debug, Clone)]
pub struct PluginDiscoveryInfo {
    /// Plugin metadata
    pub info: PluginInfo,
    /// Plugin category for organization
    pub category: PluginCategory,
    /// Plugin tags for filtering and search
    pub tags: Vec<String>,
    /// Plugin load priority (lower = higher priority)
    pub priority: u32,
    /// Whether this plugin should be loaded by default
    pub auto_load: bool,
    /// Plugin configuration schema (JSON schema string)
    pub config_schema: Option<String>,
    /// Plugin documentation URL
    pub documentation_url: Option<String>,
}

/// Const-compatible plugin discovery info for inventory registration
#[derive(Debug, Clone, Copy)]
pub struct StaticPluginDiscoveryInfo {
    /// Plugin metadata
    pub info: StaticPluginInfo,
    /// Plugin category for organization
    pub category: StaticPluginCategory,
    /// Plugin tags for filtering and search
    pub tags: &'static [&'static str],
    /// Plugin load priority (lower = higher priority)
    pub priority: u32,
    /// Whether this plugin should be loaded by default
    pub auto_load: bool,
    /// Plugin configuration schema (JSON schema string)
    pub config_schema: Option<&'static str>,
    /// Plugin documentation URL
    pub documentation_url: Option<&'static str>,
}

impl From<StaticPluginDiscoveryInfo> for PluginDiscoveryInfo {
    fn from(static_info: StaticPluginDiscoveryInfo) -> Self {
        Self {
            info: static_info.info.into(),
            category: static_info.category.into(),
            tags: static_info.tags.iter().map(|s| s.to_string()).collect(),
            priority: static_info.priority,
            auto_load: static_info.auto_load,
            config_schema: static_info.config_schema.map(|s| s.to_string()),
            documentation_url: static_info.documentation_url.map(|s| s.to_string()),
        }
    }
}

/// Plugin category enumeration for discovery organization (runtime version)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginCategory {
    Input(String),      // Input format type (e.g., "roll20", "foundry")
    Output(String),     // Output format type (e.g., "foundry", "pdf")
    Validation(String), // Validation type (e.g., "schema", "rules")
    Asset(String),      // Asset processing type (e.g., "images", "audio")
    Export(String),     // Export type (e.g., "zip", "directory")
    Logging(String),    // Logging type (e.g., "file", "console")
    Utility(String),    // Utility plugin type (e.g., "converter", "helper")
}

/// Const-compatible plugin category for inventory registration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaticPluginCategory {
    Input(&'static str),      // Input format type (e.g., "roll20", "foundry")
    Output(&'static str),     // Output format type (e.g., "foundry", "pdf")
    Validation(&'static str), // Validation type (e.g., "schema", "rules")
    Asset(&'static str),      // Asset processing type (e.g., "images", "audio")
    Export(&'static str),     // Export type (e.g., "zip", "directory")
    Logging(&'static str),    // Logging type (e.g., "file", "console")
    Utility(&'static str),    // Utility plugin type (e.g., "converter", "helper")
}

impl From<StaticPluginCategory> for PluginCategory {
    fn from(static_category: StaticPluginCategory) -> Self {
        match static_category {
            StaticPluginCategory::Input(s) => PluginCategory::Input(s.to_string()),
            StaticPluginCategory::Output(s) => PluginCategory::Output(s.to_string()),
            StaticPluginCategory::Validation(s) => PluginCategory::Validation(s.to_string()),
            StaticPluginCategory::Asset(s) => PluginCategory::Asset(s.to_string()),
            StaticPluginCategory::Export(s) => PluginCategory::Export(s.to_string()),
            StaticPluginCategory::Logging(s) => PluginCategory::Logging(s.to_string()),
            StaticPluginCategory::Utility(s) => PluginCategory::Utility(s.to_string()),
        }
    }
}

/// Plugin factory function type for dynamic instantiation
pub type PluginFactory = fn() -> Box<dyn std::any::Any + Send + Sync>;

/// Complete plugin registration combining discovery info and factory (runtime version)
#[derive(Clone)]
pub struct PluginRegistration {
    /// Discovery metadata
    pub discovery_info: PluginDiscoveryInfo,
    /// Factory function for creating plugin instances
    pub factory: PluginFactory,
}

/// Const-compatible plugin registration for inventory
#[derive(Clone, Copy)]
pub struct StaticPluginRegistration {
    /// Discovery metadata
    pub discovery_info: StaticPluginDiscoveryInfo,
    /// Factory function for creating plugin instances
    pub factory: PluginFactory,
}

impl From<StaticPluginRegistration> for PluginRegistration {
    fn from(static_reg: StaticPluginRegistration) -> Self {
        Self { discovery_info: static_reg.discovery_info.into(), factory: static_reg.factory }
    }
}

// Inventory collection for plugin registrations (using const-compatible version)
collect!(StaticPluginRegistration);

/// Plugin discovery and registry system
///
/// Manages automatic plugin discovery, registration, and instantiation using
/// the inventory crate for compile-time registration and runtime discovery.
pub struct PluginDiscovery {
    /// Registry of discovered plugins organized by category
    registry: RwLock<HashMap<PluginCategory, Vec<PluginRegistration>>>,
    /// Plugin loading configuration
    config: DiscoveryConfig,
    /// Discovery statistics
    stats: RwLock<DiscoveryStats>,
}

/// Plugin discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Enable automatic loading of plugins marked with auto_load
    pub auto_load_enabled: bool,
    /// Filter plugins by tags (empty = no filter)
    pub tag_filter: Vec<String>,
    /// Minimum priority level for plugin loading
    pub min_priority: u32,
    /// Maximum number of plugins to load per category
    pub max_plugins_per_category: Option<usize>,
    /// Enable plugin validation during discovery
    pub validate_plugins: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            auto_load_enabled: true,
            tag_filter: Vec::new(),
            min_priority: 0,
            max_plugins_per_category: None,
            validate_plugins: true,
        }
    }
}

/// Plugin discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub total_discovered: usize,
    pub loaded_plugins: usize,
    pub failed_loads: usize,
    pub categories_found: usize,
    pub auto_loaded: usize,
    pub filtered_out: usize,
}

impl Default for DiscoveryStats {
    fn default() -> Self {
        Self {
            total_discovered: 0,
            loaded_plugins: 0,
            failed_loads: 0,
            categories_found: 0,
            auto_loaded: 0,
            filtered_out: 0,
        }
    }
}

impl PluginDiscovery {
    /// Create a new plugin discovery system
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            registry: RwLock::new(HashMap::new()),
            config,
            stats: RwLock::new(DiscoveryStats::default()),
        }
    }

    /// Discover all registered plugins using inventory
    pub fn discover_all(&self) -> ConversionResult<()> {
        info!("Starting plugin discovery process");

        let mut registry = self.registry.write().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        let mut stats = self.stats.write().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Stats lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        registry.clear();
        *stats = DiscoveryStats::default();

        // Iterate through all registered plugins using inventory (using static version)
        for static_registration in inventory::iter::<StaticPluginRegistration> {
            // Convert to runtime version for processing
            let registration: PluginRegistration = (*static_registration).into();
            stats.total_discovered += 1;

            // Apply filters
            if self.should_filter_plugin(&registration.discovery_info) {
                stats.filtered_out += 1;
                debug!(
                    plugin_name = %registration.discovery_info.info.name,
                    "Plugin filtered out by discovery configuration"
                );
                continue;
            }

            // Add to appropriate category
            let category = registration.discovery_info.category.clone();
            registry
                .entry(category.clone())
                .or_insert_with(Vec::new)
                .push(registration.clone());

            debug!(
                plugin_name = %registration.discovery_info.info.name,
                category = ?category,
                priority = registration.discovery_info.priority,
                auto_load = registration.discovery_info.auto_load,
                "Discovered plugin"
            );
        }

        stats.categories_found = registry.keys().len();

        // Sort plugins within each category by priority
        for plugins in registry.values_mut() {
            plugins.sort_by_key(|p| p.discovery_info.priority);
        }

        info!(
            total_discovered = stats.total_discovered,
            categories_found = stats.categories_found,
            filtered_out = stats.filtered_out,
            "Plugin discovery completed"
        );

        Ok(())
    }

    /// Get all plugins in a specific category
    pub fn get_plugins_by_category(
        &self,
        category: &PluginCategory,
    ) -> ConversionResult<Vec<PluginRegistration>> {
        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        Ok(registry.get(category).cloned().unwrap_or_default())
    }

    /// Get all plugins with specific tags
    pub fn get_plugins_by_tags(
        &self,
        tags: &[String],
    ) -> ConversionResult<Vec<PluginRegistration>> {
        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        let mut matched_plugins = Vec::new();

        for plugins in registry.values() {
            for plugin in plugins {
                if plugin
                    .discovery_info
                    .tags
                    .iter()
                    .any(|tag| tags.contains(tag))
                {
                    matched_plugins.push(plugin.clone());
                }
            }
        }

        // Sort by priority
        matched_plugins.sort_by_key(|p| p.discovery_info.priority);

        Ok(matched_plugins)
    }

    /// Find a specific plugin by name
    pub fn find_plugin(&self, name: &str) -> ConversionResult<Option<PluginRegistration>> {
        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        for plugins in registry.values() {
            for plugin in plugins {
                if plugin.discovery_info.info.name == name {
                    return Ok(Some(plugin.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Get all available plugin categories
    pub fn get_categories(&self) -> ConversionResult<Vec<PluginCategory>> {
        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        Ok(registry.keys().cloned().collect())
    }

    /// Auto-load plugins marked for automatic loading
    pub fn auto_load_plugins(&self) -> ConversionResult<Vec<PluginRegistration>> {
        if !self.config.auto_load_enabled {
            return Ok(Vec::new());
        }

        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        let mut auto_loaded = Vec::new();

        for plugins in registry.values() {
            for plugin in plugins {
                if plugin.discovery_info.auto_load {
                    auto_loaded.push(plugin.clone());
                }
            }
        }

        // Sort by priority
        auto_loaded.sort_by_key(|p| p.discovery_info.priority);

        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.auto_loaded = auto_loaded.len();
        }

        info!(auto_loaded_count = auto_loaded.len(), "Auto-loaded plugins");

        Ok(auto_loaded)
    }

    /// Validate a plugin registration
    pub fn validate_plugin(&self, registration: &PluginRegistration) -> ConversionResult<()> {
        let info = &registration.discovery_info.info;

        // Basic validation checks
        if info.name.is_empty() {
            return Err(Box::new(ConversionError::InvalidInput {
                message: "Plugin name cannot be empty".to_string(),
                field: Some("name".to_string()),
                expected_type: Some("non_empty_string".to_string()),
            }));
        }

        if info.version.is_empty() {
            return Err(Box::new(ConversionError::InvalidInput {
                message: "Plugin version cannot be empty".to_string(),
                field: Some("version".to_string()),
                expected_type: Some("non_empty_string".to_string()),
            }));
        }

        // Validate dependencies exist if specified
        for dependency in &info.dependencies {
            if self.find_plugin(dependency)?.is_none() {
                warn!(
                    plugin_name = %info.name,
                    missing_dependency = %dependency,
                    "Plugin has unresolved dependency"
                );
            }
        }

        debug!(
            plugin_name = %info.name,
            "Plugin validation successful"
        );

        Ok(())
    }

    /// Get discovery statistics
    pub fn get_stats(&self) -> ConversionResult<DiscoveryStats> {
        let stats = self.stats.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Stats lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        Ok(stats.clone())
    }

    /// Configure the discovery system
    pub fn configure(&mut self, config: DiscoveryConfig) {
        self.config = config;
        info!("Plugin discovery configuration updated");
    }

    /// Check if a plugin should be filtered based on configuration
    fn should_filter_plugin(&self, info: &PluginDiscoveryInfo) -> bool {
        // Priority filter
        if info.priority < self.config.min_priority {
            return true;
        }

        // Tag filter (if configured)
        if !self.config.tag_filter.is_empty() {
            let has_matching_tag = info
                .tags
                .iter()
                .any(|tag| self.config.tag_filter.contains(tag));
            if !has_matching_tag {
                return true;
            }
        }

        false
    }

    /// List all discovered plugins with metadata
    pub fn list_all_plugins(&self) -> ConversionResult<Vec<PluginDiscoveryInfo>> {
        let registry = self.registry.read().map_err(|e| {
            Box::new(ConversionError::PluginError {
                message: format!("Registry lock error: {}", e),
                plugin_name: None,
                source: None,
            })
        })?;

        let mut all_plugins = Vec::new();

        for plugins in registry.values() {
            for plugin in plugins {
                all_plugins.push(plugin.discovery_info.clone());
            }
        }

        // Sort by category and then by priority
        all_plugins.sort_by(|a, b| {
            a.category
                .partial_cmp(&b.category)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.priority.cmp(&b.priority))
        });

        Ok(all_plugins)
    }
}

/// Macro for registering plugins with the discovery system
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty, $discovery_info:expr, $factory:expr) => {
        inventory::submit! {
            PluginRegistration {
                discovery_info: $discovery_info,
                factory: $factory,
            }
        }
    };
}

/// Helper function to create plugin discovery info
pub fn create_discovery_info(
    info: PluginInfo,
    category: PluginCategory,
    tags: Vec<String>,
    priority: u32,
    auto_load: bool,
) -> PluginDiscoveryInfo {
    PluginDiscoveryInfo {
        info,
        category,
        tags,
        priority,
        auto_load,
        config_schema: None,
        documentation_url: None,
    }
}

impl std::fmt::Display for PluginCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginCategory::Input(format) => write!(f, "Input({})", format),
            PluginCategory::Output(format) => write!(f, "Output({})", format),
            PluginCategory::Validation(rule) => write!(f, "Validation({})", rule),
            PluginCategory::Asset(processor) => write!(f, "Asset({})", processor),
            PluginCategory::Export(exporter) => write!(f, "Export({})", exporter),
            PluginCategory::Logging(strategy) => write!(f, "Logging({})", strategy),
            PluginCategory::Utility(utility) => write!(f, "Utility({})", utility),
        }
    }
}

impl PartialOrd for PluginCategory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_string().cmp(&other.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discovery_creation() {
        let config = DiscoveryConfig::default();
        let discovery = PluginDiscovery::new(config);

        let stats = discovery.get_stats().unwrap();
        assert_eq!(stats.total_discovered, 0);
    }

    #[test]
    fn test_plugin_category_display() {
        let category = PluginCategory::Input("roll20".to_string());
        assert_eq!(category.to_string(), "Input(roll20)");

        let category = PluginCategory::Output("foundry".to_string());
        assert_eq!(category.to_string(), "Output(foundry)");
    }

    #[test]
    fn test_discovery_info_creation() {
        let info = PluginInfo::new("Test Plugin", "1.0", "Test description", "Test Author");
        let category = PluginCategory::Input("test".to_string());
        let tags = vec!["test".to_string(), "example".to_string()];

        let discovery_info = create_discovery_info(info, category, tags, 10, true);

        assert_eq!(discovery_info.info.name, "Test Plugin");
        assert_eq!(discovery_info.priority, 10);
        assert!(discovery_info.auto_load);
        assert_eq!(discovery_info.tags.len(), 2);
    }

    #[tokio::test]
    async fn test_plugin_discovery_workflow() {
        let config = DiscoveryConfig::default();
        let discovery = PluginDiscovery::new(config);

        // Test discovery process
        let result = discovery.discover_all();
        assert!(result.is_ok());

        // Test category retrieval
        let categories = discovery.get_categories().unwrap();
        // No plugins registered in test environment
        assert!(categories.is_empty());

        // Test stats
        let stats = discovery.get_stats().unwrap();
        assert_eq!(stats.total_discovered, 0);
    }

    #[test]
    fn test_plugin_filtering() {
        let mut config = DiscoveryConfig::default();
        config.min_priority = 5;
        config.tag_filter = vec!["production".to_string()];

        let discovery = PluginDiscovery::new(config);

        // Test filtering logic through should_filter_plugin
        let info1 = PluginDiscoveryInfo {
            info: PluginInfo::new("Plugin1", "1.0", "Test", "Author"),
            category: PluginCategory::Input("test".to_string()),
            tags: vec!["development".to_string()],
            priority: 1, // Below min_priority
            auto_load: true,
            config_schema: None,
            documentation_url: None,
        };

        let info2 = PluginDiscoveryInfo {
            info: PluginInfo::new("Plugin2", "1.0", "Test", "Author"),
            category: PluginCategory::Input("test".to_string()),
            tags: vec!["production".to_string()],
            priority: 10, // Above min_priority and has matching tag
            auto_load: true,
            config_schema: None,
            documentation_url: None,
        };

        assert!(discovery.should_filter_plugin(&info1)); // Should filter due to priority and tag
        assert!(!discovery.should_filter_plugin(&info2)); // Should not filter
    }
}
