//! Pure Plugin Ecosystem - Unified Plugin Manager
//!
//! Revolutionary plugin management system that replaces service injection with
//! a unified plugin registry supporting all plugin types: Input, Output, Asset,
//! Validation, Logging, and Export plugins with dependency resolution.

use std::collections::HashMap;
use std::path::Path;

use tracing::{error, info, warn};

use super::interfaces::{AssetPlugin, ExportPlugin, LoggingPlugin, PluginConfig, ValidationPlugin};
use super::registry::{InputPluginRegistry, OutputPluginRegistry};
use super::{OutputFormat, PluginInfo};
use crate::{ConversionError, ConversionResult};

/// Pure Plugin Ecosystem - Unified Plugin Management System
///
/// The PluginManager coordinates ALL plugin types in the pure plugin ecosystem:
/// - Input/Output plugins for format conversion
/// - Asset plugins for comprehensive asset discovery and processing
/// - Validation plugins for data integrity and rule checking
/// - Logging plugins for flexible logging strategies
/// - Export plugins for advanced export capabilities
/// - Plugin dependency resolution and lifecycle management
pub struct PluginManager {
    /// Registered input format plugins using object-safe enum registry
    input_plugins: HashMap<String, InputPluginRegistry>,

    /// Registered output format plugins using object-safe enum registry
    output_plugins: HashMap<String, OutputPluginRegistry>,

    /// Registered asset processing plugins
    asset_plugins: HashMap<String, Box<dyn AssetPlugin>>,

    /// Registered validation plugins
    validation_plugins: HashMap<String, Box<dyn ValidationPlugin>>,

    /// Registered logging plugins
    logging_plugins: HashMap<String, Box<dyn LoggingPlugin>>,

    /// Registered export plugins
    export_plugins: HashMap<String, Box<dyn ExportPlugin>>,

    /// Active plugin configuration
    active_plugins: ActivePluginConfig,

    /// Plugin manager configuration
    config: PluginManagerConfig,
}

/// Active plugin configuration - tracks which plugins are currently selected
///
/// Allows dynamic plugin switching and dependency management:
/// - Plugin selection per operation type
/// - Plugin dependency resolution
/// - Plugin lifecycle coordination
#[derive(Debug, Clone, Default)]
pub struct ActivePluginConfig {
    /// Currently selected asset plugin
    pub asset_plugin: Option<String>,

    /// Currently selected validation plugin
    pub validation_plugin: Option<String>,

    /// Currently selected logging plugin
    pub logging_plugin: Option<String>,

    /// Currently selected export plugin
    pub export_plugin: Option<String>,
}

/// Plugin manager configuration options
#[derive(Debug, Clone)]
pub struct PluginManagerConfig {
    /// Enable automatic plugin discovery
    pub auto_discovery: bool,

    /// Plugin loading timeout in seconds
    pub plugin_timeout: u64,

    /// Maximum concurrent plugin operations
    pub max_concurrent_ops: usize,

    /// Enable plugin validation on registration
    pub validate_plugins: bool,
}

impl Default for PluginManagerConfig {
    fn default() -> Self {
        Self {
            auto_discovery: true,
            plugin_timeout: 30,
            max_concurrent_ops: 4,
            validate_plugins: true,
        }
    }
}

impl PluginManager {
    /// Create a new plugin manager with default configuration
    pub fn new() -> Self {
        Self::with_config(PluginManagerConfig::default())
    }

    /// Create a new plugin manager with custom configuration
    pub fn with_config(config: PluginManagerConfig) -> Self {
        Self {
            input_plugins: HashMap::new(),
            output_plugins: HashMap::new(),
            asset_plugins: HashMap::new(),
            validation_plugins: HashMap::new(),
            logging_plugins: HashMap::new(),
            export_plugins: HashMap::new(),
            active_plugins: ActivePluginConfig::default(),
            config,
        }
    }

    /// Register an asset plugin
    pub async fn register_asset_plugin(
        &mut self,
        name: String,
        mut plugin: Box<dyn AssetPlugin>,
    ) -> ConversionResult<()> {
        plugin.initialize(PluginConfig::default()).await?;

        if self.config.validate_plugins {
            // Basic plugin info validation
            let info = plugin.plugin_info();
            if info.name.is_empty() {
                return Err(Box::new(ConversionError::validation(
                    "plugin",
                    "Asset plugin name cannot be empty",
                )));
            }
        }

        info!("Registering asset plugin: {}", name);
        self.asset_plugins.insert(name, plugin);
        Ok(())
    }

    /// Register a validation plugin
    pub async fn register_validation_plugin(
        &mut self,
        name: String,
        mut plugin: Box<dyn ValidationPlugin>,
    ) -> ConversionResult<()> {
        plugin.initialize(PluginConfig::default()).await?;

        if self.config.validate_plugins {
            let info = plugin.plugin_info();
            if info.name.is_empty() {
                return Err(Box::new(ConversionError::validation(
                    "plugin",
                    "Validation plugin name cannot be empty",
                )));
            }
        }

        info!("Registering validation plugin: {}", name);
        self.validation_plugins.insert(name, plugin);
        Ok(())
    }

    /// Register a logging plugin
    pub async fn register_logging_plugin(
        &mut self,
        name: String,
        mut plugin: Box<dyn LoggingPlugin>,
    ) -> ConversionResult<()> {
        plugin.initialize(PluginConfig::default()).await?;

        if self.config.validate_plugins {
            let info = plugin.plugin_info();
            if info.name.is_empty() {
                return Err(Box::new(ConversionError::validation(
                    "plugin",
                    "Logging plugin name cannot be empty",
                )));
            }
        }

        info!("Registering logging plugin: {}", name);
        self.logging_plugins.insert(name, plugin);
        Ok(())
    }

    /// Register an export plugin
    pub async fn register_export_plugin(
        &mut self,
        name: String,
        mut plugin: Box<dyn ExportPlugin>,
    ) -> ConversionResult<()> {
        plugin.initialize(PluginConfig::default()).await?;

        if self.config.validate_plugins {
            let info = plugin.plugin_info();
            if info.name.is_empty() {
                return Err(Box::new(ConversionError::validation(
                    "plugin",
                    "Export plugin name cannot be empty",
                )));
            }
        }

        info!("Registering export plugin: {}", name);
        self.export_plugins.insert(name, plugin);
        Ok(())
    }

    /// Register an input plugin
    ///
    /// # Arguments
    /// * `plugin` - Input plugin to register
    ///
    /// # Returns
    /// * `Ok(())` if registration successful
    ///
    /// # Errors
    /// * Returns `ConversionError` if plugin validation fails
    pub fn register_input_plugin(&mut self, plugin: InputPluginRegistry) -> ConversionResult<()> {
        let info = plugin.plugin_info();

        if self.config.validate_plugins {
            self.validate_input_plugin(&plugin)?;
        }

        info!("Registering input plugin: {} v{}", info.name, info.version);
        self.input_plugins.insert(info.name.clone(), plugin);
        Ok(())
    }

    /// Register an output plugin
    ///
    /// # Arguments
    /// * `plugin` - Output plugin to register
    ///
    /// # Returns
    /// * `Ok(())` if registration successful
    ///
    /// # Errors
    /// * Returns `ConversionError` if plugin validation fails
    pub fn register_output_plugin(&mut self, plugin: OutputPluginRegistry) -> ConversionResult<()> {
        let info = plugin.plugin_info();

        if self.config.validate_plugins {
            self.validate_output_plugin(&plugin)?;
        }

        info!("Registering output plugin: {} v{}", info.name, info.version);
        self.output_plugins.insert(info.name.clone(), plugin);
        Ok(())
    }

    /// Auto-detect and return appropriate input plugin for source
    ///
    /// # Arguments
    /// * `source` - Path to campaign source file or directory
    ///
    /// # Returns
    /// * Reference to input plugin that can handle the source
    ///
    /// # Errors
    /// * Returns `ConversionError::UnsupportedInputFormat` if no plugin can handle source
    pub fn detect_input_plugin(&self, source: &Path) -> ConversionResult<&InputPluginRegistry> {
        info!("Auto-detecting input plugin for: {}", source.display());

        for (name, plugin) in &self.input_plugins {
            if plugin.can_handle(source) {
                info!("Selected input plugin: {}", name);
                return Ok(plugin);
            }
        }

        error!("No input plugin found for source: {}", source.display());
        Err(Box::new(ConversionError::UnsupportedInputFormat(source.to_path_buf())))
    }

    /// Get output plugin by format
    ///
    /// # Arguments
    /// * `format` - Desired output format
    ///
    /// # Returns
    /// * Reference to output plugin that supports the format
    ///
    /// # Errors
    /// * Returns `ConversionError::UnsupportedOutputFormat` if no plugin supports format
    pub fn get_output_plugin(
        &self,
        format: OutputFormat,
    ) -> ConversionResult<&OutputPluginRegistry> {
        info!("Finding output plugin for format: {}", format);

        for (name, plugin) in &self.output_plugins {
            if plugin.supported_formats().contains(&format) {
                info!("Selected output plugin: {}", name);
                return Ok(plugin);
            }
        }

        error!("No output plugin found for format: {}", format);
        Err(Box::new(ConversionError::UnsupportedOutputFormat(format)))
    }

    /// Register all built-in plugins
    ///
    /// This method registers the core plugins that ship with the converter.
    /// Additional plugins can be registered separately.
    ///
    /// # Returns
    /// * `Ok(())` if all plugins registered successfully
    ///
    /// # Errors
    /// * Returns `ConversionError` if any plugin registration fails
    pub fn register_builtin_plugins(&mut self) -> ConversionResult<()> {
        use super::registry::*;

        info!("Registering built-in plugins");

        // Register built-in input plugins
        self.register_input_plugin(InputPluginRegistry::Roll20(Roll20InputHandler::new()))?;

        // Register built-in output plugins
        self.register_output_plugin(OutputPluginRegistry::FoundryWorld(FoundryWorldHandler))?;
        self.register_output_plugin(OutputPluginRegistry::UniversalJson(UniversalJsonHandler))?;

        info!("Built-in plugin registration complete");
        Ok(())
    }

    /// List all registered input plugins
    pub fn list_input_plugins(&self) -> Vec<PluginInfo> {
        self.input_plugins
            .values()
            .map(|plugin| plugin.plugin_info())
            .collect()
    }

    /// List all registered output plugins
    pub fn list_output_plugins(&self) -> Vec<PluginInfo> {
        self.output_plugins
            .values()
            .map(|plugin| plugin.plugin_info())
            .collect()
    }

    /// Get available output formats from all registered plugins
    pub fn available_output_formats(&self) -> Vec<OutputFormat> {
        let mut formats = Vec::new();
        for plugin in self.output_plugins.values() {
            formats.extend(plugin.supported_formats());
        }
        formats.sort_by_key(|a| a.to_string());
        formats.dedup();
        formats
    }

    /// Get active plugin configuration
    pub fn active_plugins(&self) -> &ActivePluginConfig {
        &self.active_plugins
    }

    /// Validate input plugin during registration
    fn validate_input_plugin(&self, plugin: &InputPluginRegistry) -> ConversionResult<()> {
        let info = plugin.plugin_info();

        // Basic validation checks
        if info.name.is_empty() {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                "Input plugin name cannot be empty",
            )));
        }

        if info.version.is_empty() {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                "Input plugin version cannot be empty",
            )));
        }

        // Check for duplicate names
        if self.input_plugins.contains_key(&info.name) {
            warn!("Input plugin '{}' is already registered, will be replaced", info.name);
        }

        Ok(())
    }

    /// Validate output plugin during registration
    fn validate_output_plugin(&self, plugin: &OutputPluginRegistry) -> ConversionResult<()> {
        let info = plugin.plugin_info();

        // Basic validation checks
        if info.name.is_empty() {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                "Output plugin name cannot be empty",
            )));
        }

        if info.version.is_empty() {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                "Output plugin version cannot be empty",
            )));
        }

        if plugin.supported_formats().is_empty() {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                format!("Output plugin '{}' must support at least one output format", info.name),
            )));
        }

        // Check for duplicate names
        if self.output_plugins.contains_key(&info.name) {
            warn!("Output plugin '{}' is already registered, will be replaced", info.name);
        }

        Ok(())
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::registry::{
        FoundryWorldHandler, InputPluginRegistry, OutputPluginRegistry, Roll20InputHandler,
    };
    use crate::plugins::OutputFormat;
    use std::path::PathBuf;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.input_plugins.len(), 0);
        assert_eq!(manager.output_plugins.len(), 0);
    }

    #[test]
    fn test_input_plugin_registration() {
        let mut manager = PluginManager::new();
        let plugin = InputPluginRegistry::Roll20(Roll20InputHandler {
            validation_service: None,
            asset_service: None,
            logging_service: None,
        });

        let result = manager.register_input_plugin(plugin);
        assert!(result.is_ok());
        assert_eq!(manager.input_plugins.len(), 1);
    }

    #[test]
    fn test_output_plugin_registration() {
        let mut manager = PluginManager::new();
        let plugin = OutputPluginRegistry::FoundryWorld(FoundryWorldHandler);

        let result = manager.register_output_plugin(plugin);
        assert!(result.is_ok());
        assert_eq!(manager.output_plugins.len(), 1);
    }

    #[test]
    fn test_plugin_detection() {
        let mut manager = PluginManager::new();
        let plugin = InputPluginRegistry::Roll20(Roll20InputHandler::new());
        manager.register_input_plugin(plugin).unwrap();

        // Use a JSON file that Roll20InputHandler can recognize
        let test_path = PathBuf::from("campaign.json");
        let result = manager.detect_input_plugin(&test_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_output_plugin_lookup() {
        let mut manager = PluginManager::new();
        let plugin = OutputPluginRegistry::FoundryWorld(FoundryWorldHandler);
        manager.register_output_plugin(plugin).unwrap();

        let result = manager.get_output_plugin(OutputFormat::FoundryWorld);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), OutputPluginRegistry::FoundryWorld(_)));
    }

    #[test]
    fn test_available_formats() {
        let mut manager = PluginManager::new();
        let plugin1 = OutputPluginRegistry::FoundryWorld(FoundryWorldHandler);
        let plugin2 =
            OutputPluginRegistry::FoundryModule(crate::plugins::registry::FoundryModuleHandler);
        manager.register_output_plugin(plugin1).unwrap();
        manager.register_output_plugin(plugin2).unwrap();

        let formats = manager.available_output_formats();
        assert_eq!(formats.len(), 2);
        assert!(formats.contains(&OutputFormat::FoundryWorld));
        assert!(formats.contains(&OutputFormat::FoundryModule));
    }
}
