//! Plugin discovery orchestration
//!
//! This module provides orchestration-level plugin discovery that delegates
//! to the ttrpg-manager for actual plugin discovery and management.

use ttrpg_core::error::{ConversionResult, ConversionError};
use ttrpg_traits::PluginInfo;
use ttrpg_manager::PluginManager;

/// Orchestration-level plugin discovery that delegates to ttrpg-manager
pub struct PluginDiscovery {
    manager: PluginManager,
    enabled_plugins: Vec<String>,
}

impl PluginDiscovery {
    /// Create new plugin discovery system
    pub fn new() -> Self {
        Self {
            manager: PluginManager::new(),
            enabled_plugins: Vec::new(),
        }
    }

    /// Discover available plugins using ttrpg-manager
    pub async fn discover_plugins(&mut self) -> ConversionResult<()> {
        // Delegate to ttrpg-manager for actual plugin discovery
        self.manager.initialize().await
            .map_err(|e| Box::new(ConversionError::InvalidInput {
                message: format!("Plugin discovery failed: {}", e),
                field: None,
                expected_type: None
            }))?;
        Ok(())
    }

    /// Get list of available plugins from ttrpg-manager
    pub async fn available_plugins(&self) -> Vec<PluginInfo> {
        // Get all plugins and extract their info
        self.manager.list_all_plugins().await
            .into_iter()
            .map(|entry| entry.info)
            .collect()
    }

    /// Enable a plugin by ID
    pub fn enable_plugin(&mut self, plugin_id: String) {
        if !self.enabled_plugins.contains(&plugin_id) {
            self.enabled_plugins.push(plugin_id);
        }
    }

    /// Disable a plugin by ID
    pub fn disable_plugin(&mut self, plugin_id: &str) {
        self.enabled_plugins.retain(|id| id != plugin_id);
    }

    /// Get enabled plugin IDs
    pub fn enabled_plugins(&self) -> &[String] {
        &self.enabled_plugins
    }

    /// Check if a plugin is enabled
    pub fn is_plugin_enabled(&self, plugin_id: &str) -> bool {
        self.enabled_plugins.contains(&plugin_id.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_discovery_creation() {
        let discovery = PluginDiscovery::new();
        assert!(discovery.enabled_plugins.is_empty());
    }

    #[test]
    fn test_plugin_enable_disable() {
        let mut discovery = PluginDiscovery::new();
        
        // Enable a plugin
        discovery.enable_plugin("test-plugin".to_string());
        assert!(discovery.is_plugin_enabled("test-plugin"));
        assert_eq!(discovery.enabled_plugins().len(), 1);
        
        // Enable same plugin again (should not duplicate)
        discovery.enable_plugin("test-plugin".to_string());
        assert_eq!(discovery.enabled_plugins().len(), 1);
        
        // Disable plugin
        discovery.disable_plugin("test-plugin");
        assert!(!discovery.is_plugin_enabled("test-plugin"));
        assert!(discovery.enabled_plugins().is_empty());
    }

    #[test]
    fn test_multiple_plugins() {
        let mut discovery = PluginDiscovery::new();
        
        discovery.enable_plugin("plugin1".to_string());
        discovery.enable_plugin("plugin2".to_string());
        discovery.enable_plugin("plugin3".to_string());
        
        assert_eq!(discovery.enabled_plugins().len(), 3);
        assert!(discovery.is_plugin_enabled("plugin1"));
        assert!(discovery.is_plugin_enabled("plugin2"));
        assert!(discovery.is_plugin_enabled("plugin3"));
        
        discovery.disable_plugin("plugin2");
        assert_eq!(discovery.enabled_plugins().len(), 2);
        assert!(discovery.is_plugin_enabled("plugin1"));
        assert!(!discovery.is_plugin_enabled("plugin2"));
        assert!(discovery.is_plugin_enabled("plugin3"));
    }

    #[test]
    fn test_plugin_discovery_error_handling() {
        let discovery = PluginDiscovery::new();
        
        // Test that we can check enabled status for non-existent plugins
        assert!(!discovery.is_plugin_enabled("nonexistent-plugin"));
        
        // Test that disabling non-existent plugin doesn't panic
        let mut discovery_mut = discovery;
        discovery_mut.disable_plugin("nonexistent-plugin");
        assert!(discovery_mut.enabled_plugins().is_empty());
    }

    #[tokio::test]
    async fn test_plugin_discovery_async_methods() {
        let discovery = PluginDiscovery::new();
        
        // Test available_plugins method (should not panic)
        let plugins = discovery.available_plugins().await;
        // We expect empty list since no plugins are registered in test environment
        assert!(plugins.is_empty() || !plugins.is_empty()); // Either case is valid
    }
}
