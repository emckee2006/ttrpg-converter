//! Plugin lifecycle management with dependency injection

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
// Removed shaku dependency for now - will add back once compilation is stable
use tokio::sync::RwLock;
use ttrpg_core::error::{ConversionResult, ConversionError};
use ttrpg_traits::PluginInfo;
use crate::registry::{PluginEntry, PluginType, PLUGIN_REGISTRY};
use tracing::{info, warn, debug, error};

/// Plugin lifecycle states
#[derive(Debug, Clone, PartialEq)]
pub enum LifecycleState {
    /// Plugin discovered but not loaded
    Discovered,
    /// Plugin loaded but not initialized
    Loaded,
    /// Plugin initialized and ready
    Ready,
    /// Plugin running
    Active,
    /// Plugin paused/suspended
    Suspended,
    /// Plugin stopped
    Stopped,
    /// Plugin failed
    Failed(String),
}

/// Plugin lifecycle manager with dependency injection
pub struct LifecycleManager {
    /// Placeholder for DI container
    _container_placeholder: Arc<RwLock<Option<()>>>,
    /// Plugin states
    plugin_states: Arc<RwLock<HashMap<String, LifecycleState>>>,
    /// Plugin instances
    plugin_instances: Arc<RwLock<HashMap<String, Box<dyn PluginInstance>>>>,
}

/// Generic plugin instance wrapper
#[async_trait]
pub trait PluginInstance: Send + Sync {
    /// Get plugin information
    async fn info(&self) -> ConversionResult<PluginInfo>;
    /// Initialize the plugin
    async fn initialize(&mut self) -> ConversionResult<()>;
    /// Start the plugin
    async fn start(&mut self) -> ConversionResult<()>;
    /// Stop the plugin
    async fn stop(&mut self) -> ConversionResult<()>;
    /// Get current state
    fn state(&self) -> LifecycleState;
    /// Check if plugin is healthy
    async fn health_check(&self) -> ConversionResult<bool>;
}

impl LifecycleManager {
    /// Create new lifecycle manager
    pub fn new() -> Self {
        Self {
            _container_placeholder: Arc::new(RwLock::new(None)),
            plugin_states: Arc::new(RwLock::new(HashMap::new())),
            plugin_instances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize the lifecycle manager with DI container
    pub async fn initialize(&self) -> ConversionResult<()> {
        info!("Initializing plugin lifecycle manager");
        
        // Create DI container
        // Placeholder for DI initialization
        let mut container_guard = self._container_placeholder.write().await;
        *container_guard = Some(());
        
        info!("Plugin lifecycle manager initialized");
        Ok(())
    }

    /// Load a plugin by ID
    pub async fn load_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        debug!("Loading plugin: {}", plugin_id);
        
        // Get plugin entry from registry
        // Find plugin entry by ID
        let registry_entries = PLUGIN_REGISTRY.list_all();
        let plugin_entry = registry_entries.iter()
            .find(|entry| entry.info.name == plugin_id)
            .ok_or_else(|| Box::new(ConversionError::validation("PluginManager", format!("Plugin not found: {}", plugin_id))))?;
        
        // Create plugin instance based on type
        let instance = self.create_plugin_instance(plugin_entry).await?;
        
        // Store instance and update state
        let mut instances = self.plugin_instances.write().await;
        instances.insert(plugin_id.to_string(), instance);
        
        let mut states = self.plugin_states.write().await;
        states.insert(plugin_id.to_string(), LifecycleState::Loaded);
        
        info!("Plugin loaded: {}", plugin_id);
        Ok(())
    }

    /// Initialize a loaded plugin
    pub async fn initialize_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        debug!("Initializing plugin: {}", plugin_id);
        
        let mut instances = self.plugin_instances.write().await;
        if let Some(instance) = instances.get_mut(plugin_id) {
            instance.initialize().await?;
            
            let mut states = self.plugin_states.write().await;
            states.insert(plugin_id.to_string(), LifecycleState::Ready);
            
            info!("Plugin initialized: {}", plugin_id);
            Ok(())
        } else {
            Err(Box::new(ConversionError::validation("PluginManager", format!("Plugin not loaded: {}", plugin_id))))
        }
    }

    /// Start a plugin
    pub async fn start_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        debug!("Starting plugin: {}", plugin_id);
        
        let mut instances = self.plugin_instances.write().await;
        if let Some(instance) = instances.get_mut(plugin_id) {
            instance.start().await?;
            
            let mut states = self.plugin_states.write().await;
            states.insert(plugin_id.to_string(), LifecycleState::Active);
            
            info!("Plugin started: {}", plugin_id);
            Ok(())
        } else {
            Err(Box::new(ConversionError::validation("PluginManager", format!("Plugin not loaded: {}", plugin_id))))
        }
    }

    /// Stop a plugin
    pub async fn stop_plugin(&self, plugin_id: &str) -> ConversionResult<()> {
        debug!("Stopping plugin: {}", plugin_id);
        
        let mut instances = self.plugin_instances.write().await;
        if let Some(instance) = instances.get_mut(plugin_id) {
            instance.stop().await?;
            
            let mut states = self.plugin_states.write().await;
            states.insert(plugin_id.to_string(), LifecycleState::Stopped);
            
            info!("Plugin stopped: {}", plugin_id);
            Ok(())
        } else {
            Err(Box::new(ConversionError::validation("PluginManager", format!("Plugin not loaded: {}", plugin_id))))
        }
    }

    /// Get plugin state
    pub async fn get_plugin_state(&self, plugin_id: &str) -> Option<LifecycleState> {
        let states = self.plugin_states.read().await;
        states.get(plugin_id).cloned()
    }

    /// Get all plugin states
    pub async fn get_all_states(&self) -> HashMap<String, LifecycleState> {
        let states = self.plugin_states.read().await;
        states.clone()
    }

    /// Perform health check on all active plugins
    pub async fn health_check_all(&self) -> ConversionResult<HashMap<String, bool>> {
        let mut results = HashMap::new();
        let instances = self.plugin_instances.read().await;
        
        for (plugin_id, instance) in instances.iter() {
            match instance.health_check().await {
                Ok(healthy) => {
                    results.insert(plugin_id.clone(), healthy);
                    if !healthy {
                        warn!("Plugin {} failed health check", plugin_id);
                    }
                },
                Err(e) => {
                    error!("Health check error for plugin {}: {}", plugin_id, e);
                    results.insert(plugin_id.clone(), false);
                }
            }
        }
        
        Ok(results)
    }

    /// Create plugin instance based on plugin entry
    async fn create_plugin_instance(&self, entry: &PluginEntry) -> ConversionResult<Box<dyn PluginInstance>> {
        match entry.plugin_type {
            PluginType::Input => {
                // For now, create a generic wrapper
                // In a real implementation, this would instantiate the actual plugin
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
            PluginType::Output => {
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
            PluginType::Asset => {
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
            PluginType::Validation => {
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
            PluginType::Processing => {
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
            PluginType::Export => {
                Ok(Box::new(GenericPluginInstance::new(entry.clone())))
            },
        }
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic plugin instance implementation
struct GenericPluginInstance {
    entry: PluginEntry,
    state: LifecycleState,
}

impl GenericPluginInstance {
    fn new(entry: PluginEntry) -> Self {
        Self {
            entry,
            state: LifecycleState::Discovered,
        }
    }
}

#[async_trait]
impl PluginInstance for GenericPluginInstance {
    async fn info(&self) -> ConversionResult<PluginInfo> {
        Ok(self.entry.info.clone())
    }

    async fn initialize(&mut self) -> ConversionResult<()> {
        debug!("Initializing plugin: {}", self.entry.info.name);
        self.state = LifecycleState::Ready;
        Ok(())
    }

    async fn start(&mut self) -> ConversionResult<()> {
        debug!("Starting plugin: {}", self.entry.info.name);
        self.state = LifecycleState::Active;
        Ok(())
    }

    async fn stop(&mut self) -> ConversionResult<()> {
        debug!("Stopping plugin: {}", self.entry.info.name);
        self.state = LifecycleState::Stopped;
        Ok(())
    }

    fn state(&self) -> LifecycleState {
        self.state.clone()
    }

    async fn health_check(&self) -> ConversionResult<bool> {
        // Generic health check - always healthy unless failed
        Ok(!matches!(self.state, LifecycleState::Failed(_)))
    }
}

// Removed DI module for now - will add back when shaku is properly configured

/// Lifecycle statistics
#[derive(Debug, Clone)]
pub struct LifecycleStats {
    pub total_plugins: usize,
    pub loaded_plugins: usize,
    pub active_plugins: usize,
    pub failed_plugins: usize,
    pub by_state: HashMap<String, usize>,
}

impl LifecycleManager {
    /// Get lifecycle statistics
    pub async fn get_stats(&self) -> LifecycleStats {
        let states = self.plugin_states.read().await;
        let mut by_state = HashMap::new();
        let mut loaded = 0;
        let mut active = 0;
        let mut failed = 0;
        
        for state in states.values() {
            let state_name = match state {
                LifecycleState::Discovered => "discovered",
                LifecycleState::Loaded => { loaded += 1; "loaded" },
                LifecycleState::Ready => "ready",
                LifecycleState::Active => { active += 1; "active" },
                LifecycleState::Suspended => "suspended",
                LifecycleState::Stopped => "stopped",
                LifecycleState::Failed(_) => { failed += 1; "failed" },
            };
            
            *by_state.entry(state_name.to_string()).or_insert(0) += 1;
        }
        
        LifecycleStats {
            total_plugins: states.len(),
            loaded_plugins: loaded,
            active_plugins: active,
            failed_plugins: failed,
            by_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lifecycle_manager_creation() {
        let manager = LifecycleManager::new();
        let stats = manager.get_stats().await;
        
        assert_eq!(stats.total_plugins, 0);
        assert_eq!(stats.active_plugins, 0);
    }

    #[tokio::test]
    async fn test_plugin_state_tracking() {
        let manager = LifecycleManager::new();
        
        // Initially no states
        let states = manager.get_all_states().await;
        assert!(states.is_empty());
        
        // Would need actual plugin for full test
        // This is a placeholder for proper integration testing
    }
}
