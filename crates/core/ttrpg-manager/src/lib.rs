//! TTRPG Plugin management system for TTRPG converter
//! 
//! This crate provides plugin discovery, lifecycle management, and execution coordination.
//! Plugins are registered at compile-time using the inventory crate and can be discovered
//! and managed at runtime.

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use ttrpg_core::error::{ConversionResult, ConversionError};
use tracing::info;

pub mod registry;
pub mod discovery;
pub mod lifecycle;

// Re-export key types
pub use registry::{PluginRegistry, PluginType, PluginEntry, PLUGIN_REGISTRY};
pub use discovery::{PluginDiscovery, DiscoveryConfig, DiscoveryStats};
pub use lifecycle::{LifecycleManager, LifecycleState, PluginInstance, LifecycleStats};

/// Main plugin manager with comprehensive lifecycle and discovery support
pub struct PluginManager {
    /// Plugin discovery service
    discovery: Arc<PluginDiscovery>,
    /// Plugin lifecycle manager
    lifecycle: Arc<LifecycleManager>,
    /// Plugin registry access
    registry: Arc<RwLock<Vec<PluginEntry>>>,
}

impl PluginManager {
    /// Create a new plugin manager with default configuration
    pub fn new() -> Self {
        Self::with_config(DiscoveryConfig::default())
    }

    /// Create plugin manager with custom discovery configuration
    pub fn with_config(config: DiscoveryConfig) -> Self {
        Self {
            discovery: Arc::new(PluginDiscovery::new(config)),
            lifecycle: Arc::new(LifecycleManager::new()),
            registry: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize the plugin manager
    pub async fn initialize(&self) -> ConversionResult<()> {
        info!("Initializing comprehensive plugin manager");
        
        // Initialize lifecycle manager
        self.lifecycle.initialize().await?;
        
        // Discover all plugins
        let plugins = {
            let mut discovery = Arc::try_unwrap(Arc::clone(&self.discovery))
                .map_err(|_| Box::new(ConversionError::validation("PluginManager", "Failed to get discovery service")))?;
            discovery.discover_all().await?
        };
        
        // Store discovered plugins
        let mut registry = self.registry.write().await;
        *registry = plugins;
        
        info!("Plugin manager initialized with {} plugins", registry.len());
        Ok(())
    }

    /// Get all discovered plugins
    pub async fn list_all_plugins(&self) -> Vec<PluginEntry> {
        let registry = self.registry.read().await;
        registry.clone()
    }

    /// Get plugins by type
    pub async fn list_plugins_by_type(&self, plugin_type: PluginType) -> Vec<PluginEntry> {
        let registry = self.registry.read().await;
        registry.iter()
            .filter(|entry| entry.plugin_type == plugin_type)
            .cloned()
            .collect()
    }

    /// Load a plugin by ID
    pub async fn load_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        self.lifecycle.load_plugin(plugin_id).await
    }

    /// Initialize a loaded plugin
    pub async fn initialize_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        self.lifecycle.initialize_plugin(plugin_id).await
    }

    /// Start a plugin
    pub async fn start_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        self.lifecycle.start_plugin(plugin_id).await
    }

    /// Stop a plugin
    pub async fn stop_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        self.lifecycle.stop_plugin(plugin_id).await
    }

    /// Get plugin state
    pub async fn get_plugin_state(&self, plugin_id: &str) -> Option<LifecycleState> {
        self.lifecycle.get_plugin_state(plugin_id).await
    }

    /// Get all plugin states
    pub async fn get_all_plugin_states(&self) -> HashMap<String, LifecycleState> {
        self.lifecycle.get_all_states().await
    }

    /// Perform health check on all plugins
    pub async fn health_check_all(&self) -> ConversionResult<HashMap<String, bool>> {
        self.lifecycle.health_check_all().await
    }

    /// Get comprehensive plugin manager statistics
    pub async fn get_stats(&self) -> PluginManagerStats {
        let lifecycle_stats = self.lifecycle.get_stats().await;
        let discovery_stats = {
            // Need to access discovery stats differently since we moved discovery
            // For now, create basic stats
            DiscoveryStats {
                static_plugins: PLUGIN_REGISTRY.list_all().len(),
                dynamic_libraries: 0, // Would need proper tracking
                search_paths: 0,
            }
        };
        
        PluginManagerStats {
            discovery: discovery_stats,
            lifecycle: lifecycle_stats,
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive plugin manager statistics
#[derive(Debug, Clone)]
pub struct PluginManagerStats {
    pub discovery: DiscoveryStats,
    pub lifecycle: LifecycleStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        let stats = manager.get_stats().await;
        
        // Should have basic stats structure
        assert_eq!(stats.lifecycle.total_plugins, 0);
    }

    #[tokio::test]
    async fn test_plugin_manager_with_config() {
        let config = DiscoveryConfig {
            search_paths: vec![std::path::PathBuf::from("./test-plugins")],
            extensions: vec!["dll".to_string()],
            allow_dynamic: false,
            max_depth: 1,
        };
        
        let manager = PluginManager::with_config(config);
        let stats = manager.get_stats().await;
        
        assert_eq!(stats.lifecycle.total_plugins, 0);
    }
}