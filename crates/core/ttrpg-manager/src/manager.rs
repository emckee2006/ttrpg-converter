//! Plugin Manager - Core Plugin Management System
//!
//! Coordinates all plugin types in the TTRPG converter ecosystem:
//! - Plugin registration and discovery
//! - Plugin lifecycle management 
//! - Plugin dependency resolution
//! - Plugin configuration and validation

use std::collections::HashMap;
use std::path::Path;
use ttrpg_core::prelude::*;
use ttrpg_traits::*;
use super::{discovery::*, injection::*, lifecycle::*};

/// Core plugin management system
#[derive(Debug)]
pub struct PluginManager {
    /// Plugin discovery system
    discovery: PluginDiscovery,
    /// Plugin lifecycle manager
    lifecycle: LifecycleManager,
    /// Registered plugin instances
    plugins: HashMap<String, PluginEntry>,
    /// Manager configuration
    config: PluginManagerConfig,
}

/// Plugin entry with metadata
#[derive(Debug, Clone)]
pub struct PluginEntry {
    /// Plugin metadata
    pub info: PluginInfo,
    /// Plugin state
    pub state: PluginState,
    /// Plugin configuration
    pub config: PluginConfig,
    /// Registration timestamp
    pub registered_at: std::time::Instant,
}

/// Plugin manager configuration
#[derive(Debug, Clone)]
pub struct PluginManagerConfig {
    /// Plugin discovery configuration
    pub discovery: DiscoveryConfig,
    /// Enable plugin validation on registration
    pub validate_on_register: bool,
    /// Maximum plugin initialization time
    pub init_timeout_secs: u64,
    /// Plugin cache directory
    pub cache_dir: Option<std::path::PathBuf>,
}

impl Default for PluginManagerConfig {
    fn default() -> Self {
        Self {
            discovery: DiscoveryConfig::default(),
            validate_on_register: true,
            init_timeout_secs: 30,
            cache_dir: None,
        }
    }
}

/// Plugin manager statistics
#[derive(Debug, Clone, Default)]
pub struct PluginManagerStats {
    /// Number of registered plugins
    pub registered_plugins: usize,
    /// Number of active plugins
    pub active_plugins: usize,
    /// Number of failed plugins
    pub failed_plugins: usize,
    /// Plugin discovery time
    pub discovery_time_ms: u64,
    /// Last discovery timestamp
    pub last_discovery: Option<std::time::Instant>,
}

impl PluginManager {
    /// Create new plugin manager with default configuration
    pub fn new() -> Self {
        Self::with_config(PluginManagerConfig::default())
    }

    /// Create plugin manager with custom configuration
    pub fn with_config(config: PluginManagerConfig) -> Self {
        Self {
            discovery: PluginDiscovery::new(config.discovery.clone()),
            lifecycle: LifecycleManager::new(),
            plugins: HashMap::new(),
            config,
        }
    }

    /// Initialize the plugin manager and discover plugins
    pub async fn initialize(&mut self) -> ConversionResult<()> {
        tracing::info!("Initializing plugin manager");
        
        // Initialize lifecycle manager
        self.lifecycle.initialize().await?;
        
        // Discover available plugins
        self.discover_plugins().await?;
        
        tracing::info!("Plugin manager initialization complete");
        Ok(())
    }

    /// Discover and register available plugins
    pub async fn discover_plugins(&mut self) -> ConversionResult<()> {
        let start_time = std::time::Instant::now();
        tracing::info!("Starting plugin discovery");

        // Discover static plugins
        let static_plugins = self.discovery.discover_static_plugins().await?;
        for plugin_info in static_plugins {
            self.register_static_plugin(plugin_info).await?;
        }

        // Discover dynamic plugins
        let dynamic_plugins = self.discovery.discover_dynamic_plugins().await?;
        for (path, plugin_info) in dynamic_plugins {
            self.register_dynamic_plugin(path, plugin_info).await?;
        }

        let discovery_time = start_time.elapsed();
        tracing::info!(
            "Plugin discovery complete in {}ms, {} plugins found",
            discovery_time.as_millis(),
            self.plugins.len()
        );

        Ok(())
    }

    /// Register a static plugin (compile-time registered)
    async fn register_static_plugin(&mut self, info: PluginInfo) -> ConversionResult<()> {
        let entry = PluginEntry {
            info: info.clone(),
            state: PluginState::Registered,
            config: PluginConfig::default(),
            registered_at: std::time::Instant::now(),
        };

        self.plugins.insert(info.name.clone(), entry);
        tracing::debug!("Registered static plugin: {}", info.name);
        Ok(())
    }

    /// Register a dynamic plugin (runtime loaded from library)
    async fn register_dynamic_plugin(
        &mut self,
        _path: std::path::PathBuf,
        info: PluginInfo,
    ) -> ConversionResult<()> {
        let entry = PluginEntry {
            info: info.clone(),
            state: PluginState::Registered,
            config: PluginConfig::default(),
            registered_at: std::time::Instant::now(),
        };

        self.plugins.insert(info.name.clone(), entry);
        tracing::debug!("Registered dynamic plugin: {}", info.name);
        Ok(())
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<&PluginEntry> {
        self.plugins.get(name)
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<&PluginEntry> {
        self.plugins.values().collect()
    }

    /// Get plugins by type/capability
    pub fn get_plugins_by_type(&self, plugin_type: &str) -> Vec<&PluginEntry> {
        self.plugins
            .values()
            .filter(|entry| entry.info.supported_features.contains(&plugin_type.to_string()))
            .collect()
    }

    /// Load and initialize a specific plugin
    pub async fn load_plugin(&mut self, name: &str) -> ConversionResult<()> {
        let entry = self.plugins.get_mut(name)
            .ok_or_else(|| ConversionError::plugin_error(format!("Plugin not found: {}", name), Some(name.to_string())))?;

        if entry.state != PluginState::Registered {
            return Err(ConversionError::plugin_error(
                format!("Plugin {} is not in registered state", name),
                Some(name.to_string()),
            ));
        }

        // Use lifecycle manager to load the plugin
        self.lifecycle.load_plugin(name, &entry.config).await?;
        entry.state = PluginState::Loaded;

        tracing::info!("Loaded plugin: {}", name);
        Ok(())
    }

    /// Start a loaded plugin
    pub async fn start_plugin(&mut self, name: &str) -> ConversionResult<()> {
        let entry = self.plugins.get_mut(name)
            .ok_or_else(|| ConversionError::plugin_error(format!("Plugin not found: {}", name), Some(name.to_string())))?;

        if entry.state != PluginState::Loaded {
            return Err(ConversionError::plugin_error(
                format!("Plugin {} must be loaded before starting", name),
                Some(name.to_string()),
            ));
        }

        self.lifecycle.start_plugin(name).await?;
        entry.state = PluginState::Running;

        tracing::info!("Started plugin: {}", name);
        Ok(())
    }

    /// Stop a running plugin
    pub async fn stop_plugin(&mut self, name: &str) -> ConversionResult<()> {
        let entry = self.plugins.get_mut(name)
            .ok_or_else(|| ConversionError::plugin_error(format!("Plugin not found: {}", name), Some(name.to_string())))?;

        if entry.state != PluginState::Running {
            return Err(ConversionError::plugin_error(
                format!("Plugin {} is not running", name),
                Some(name.to_string()),
            ));
        }

        self.lifecycle.stop_plugin(name).await?;
        entry.state = PluginState::Loaded;

        tracing::info!("Stopped plugin: {}", name);
        Ok(())
    }

    /// Get plugin manager statistics
    pub fn get_stats(&self) -> PluginManagerStats {
        let active_plugins = self.plugins
            .values()
            .filter(|entry| matches!(entry.state, PluginState::Running))
            .count();
            
        let failed_plugins = self.plugins
            .values()
            .filter(|entry| matches!(entry.state, PluginState::Failed(_)))
            .count();

        PluginManagerStats {
            registered_plugins: self.plugins.len(),
            active_plugins,
            failed_plugins,
            discovery_time_ms: 0, // TODO: Track this
            last_discovery: None, // TODO: Track this
        }
    }

    /// Get plugin discovery system
    pub fn discovery(&self) -> &PluginDiscovery {
        &self.discovery
    }

    /// Get plugin lifecycle manager
    pub fn lifecycle(&self) -> &LifecycleManager {
        &self.lifecycle
    }

    /// Get manager configuration
    pub fn config(&self) -> &PluginManagerConfig {
        &self.config
    }

    /// Shutdown the plugin manager
    pub async fn shutdown(&mut self) -> ConversionResult<()> {
        tracing::info!("Shutting down plugin manager");
        
        // Stop all running plugins
        let running_plugins: Vec<String> = self.plugins
            .iter()
            .filter(|(_, entry)| matches!(entry.state, PluginState::Running))
            .map(|(name, _)| name.clone())
            .collect();
            
        for plugin_name in running_plugins {
            if let Err(e) = self.stop_plugin(&plugin_name).await {
                tracing::warn!("Failed to stop plugin {}: {}", plugin_name, e);
            }
        }
        
        // Cleanup lifecycle manager
        self.lifecycle.cleanup().await?;
        
        tracing::info!("Plugin manager shutdown complete");
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

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert_eq!(manager.plugins.len(), 0);
    }

    #[tokio::test]
    async fn test_plugin_manager_initialization() {
        let mut manager = PluginManager::new();
        let result = manager.initialize().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_plugin_manager_config() {
        let config = PluginManagerConfig::default();
        let manager = PluginManager::with_config(config);
        assert!(manager.config.validate_on_register);
        assert_eq!(manager.config.init_timeout_secs, 30);
    }

    #[test]
    fn test_plugin_manager_stats() {
        let manager = PluginManager::new();
        let stats = manager.get_stats();
        assert_eq!(stats.registered_plugins, 0);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 0);
    }

    #[tokio::test]
    async fn test_manager_concurrent_operations() {
        let manager = PluginManager::new();
        
        // Test concurrent plugin loading
        let plugin_id = "concurrent-test-plugin";
        let results = tokio::join!(
            manager.load_plugin(plugin_id),
            manager.load_plugin(plugin_id),
            manager.load_plugin(plugin_id)
        );
        
        // All should either succeed or fail consistently
        let success_count = results.0.is_ok() as i32 + results.1.is_ok() as i32 + results.2.is_ok() as i32;
        assert!(success_count == 0 || success_count == 3); // Either all fail or all succeed
    }

    #[tokio::test]
    async fn test_manager_plugin_state_transitions() {
        let manager = PluginManager::new();
        let plugin_id = "state-transition-test";
        
        // Initial state should be None (not loaded)
        assert!(manager.get_plugin_state(plugin_id).await.is_none());
        
        // After attempting to load non-existent plugin, state should still be None
        let _ = manager.load_plugin(plugin_id).await; // Expected to fail
        assert!(manager.get_plugin_state(plugin_id).await.is_none());
    }

    #[tokio::test]
    async fn test_manager_health_check() {
        let manager = PluginManager::new();
        
        // Health check should not panic even with no plugins
        let health_results = manager.health_check_all().await;
        assert!(health_results.is_ok());
        
        let health_map = health_results.unwrap();
        assert!(health_map.is_empty()); // No plugins registered
    }

    #[tokio::test]
    async fn test_manager_error_recovery() {
        let manager = PluginManager::new();
        
        // Test that manager can handle invalid plugin operations gracefully
        let invalid_plugin_id = "definitely-does-not-exist";
        
        // These should all fail gracefully without panicking
        let load_result = manager.load_plugin(invalid_plugin_id).await;
        let start_result = manager.start_plugin(invalid_plugin_id).await;
        let stop_result = manager.stop_plugin(invalid_plugin_id).await;
        
        assert!(load_result.is_err());
        assert!(start_result.is_err());
        assert!(stop_result.is_err());
    }
}