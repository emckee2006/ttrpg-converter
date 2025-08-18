//! Plugin registry for compile-time registration and runtime discovery

use std::sync::Arc;
use ttrpg_traits::*;
use dashmap::DashMap;
use once_cell::sync::Lazy;

/// Plugin registry entry
#[derive(Clone)]
pub struct PluginEntry {
    pub info: PluginInfo,
    pub plugin_type: PluginType,
    pub factory: PluginFactory,
}

/// Plugin type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginType {
    Input,
    Output,
    Processing,
    Asset,
    Validation,
    Export,
}

/// Plugin factory function type
pub type PluginFactory = Arc<dyn Fn() -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>;

/// Global plugin registry using compile-time registration
pub static PLUGIN_REGISTRY: Lazy<PluginRegistry> = Lazy::new(|| PluginRegistry::new());

/// Thread-safe plugin registry
pub struct PluginRegistry {
    plugins: DashMap<String, PluginEntry>,
    type_index: DashMap<PluginType, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: DashMap::new(),
            type_index: DashMap::new(),
        }
    }

    /// Register a plugin at compile time
    pub fn register<T>(&self, info: PluginInfo, plugin_type: PluginType, factory: fn() -> T) 
    where
        T: Send + Sync + 'static,
    {
        let factory_fn: PluginFactory = Arc::new(move || Box::new(factory()));
        
        let entry = PluginEntry {
            info: info.clone(),
            plugin_type: plugin_type.clone(),
            factory: factory_fn,
        };
        
        // Register in main registry
        self.plugins.insert(info.name.clone(), entry);
        
        // Update type index
        self.type_index.entry(plugin_type).or_insert_with(Vec::new).push(info.name.clone());
    }

    /// Get all plugins of a specific type
    pub fn get_plugins_by_type(&self, plugin_type: &PluginType) -> Vec<PluginEntry> {
        if let Some(names) = self.type_index.get(plugin_type) {
            names.iter()
                .filter_map(|name| self.plugins.get(name).map(|entry| entry.clone()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<PluginEntry> {
        self.plugins.get(name).map(|entry| entry.clone())
    }

    /// List all registered plugins
    pub fn list_all(&self) -> Vec<PluginEntry> {
        self.plugins.iter().map(|entry| entry.clone()).collect()
    }

    /// Get plugin count by type
    pub fn count_by_type(&self, plugin_type: &PluginType) -> usize {
        self.type_index.get(plugin_type).map(|names| names.len()).unwrap_or(0)
    }

    /// Check if plugin is registered
    pub fn is_registered(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }
}

/// Macro for registering plugins at compile time
#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:expr, $factory:expr, $info:expr) => {
        inventory::submit! {
            $crate::registry::PluginRegistration {
                plugin_type: $plugin_type,
                factory: $factory,
                info: $info,
            }
        }
    };
}

/// Plugin registration for inventory collection
pub struct PluginRegistration {
    pub plugin_type: PluginType,
    pub factory: fn() -> Box<dyn std::any::Any + Send + Sync>,
    pub info: PluginInfo,
}

inventory::collect!(PluginRegistration);

/// Initialize all registered plugins from inventory
pub fn initialize_registry() {
    for registration in inventory::iter::<PluginRegistration> {
        let factory: PluginFactory = Arc::new(registration.factory);
        
        let entry = PluginEntry {
            info: registration.info.clone(),
            plugin_type: registration.plugin_type.clone(),
            factory,
        };
        
        PLUGIN_REGISTRY.plugins.insert(registration.info.name.clone(), entry);
        PLUGIN_REGISTRY.type_index
            .entry(registration.plugin_type.clone())
            .or_insert_with(Vec::new)
            .push(registration.info.name.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registry_basic() {
        let registry = PluginRegistry::new();
        
        let info = PluginInfo {
            name: "test-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test plugin".to_string(),
            author: "Test Author".to_string(),
            supported_features: vec![],
            dependencies: vec![],
        };
        
        registry.register(info.clone(), PluginType::Input, || "test");
        
        assert!(registry.is_registered("test-plugin"));
        assert_eq!(registry.count_by_type(&PluginType::Input), 1);
        
        let plugins = registry.get_plugins_by_type(&PluginType::Input);
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].info.name, "test-plugin");
    }

    #[test]
    fn test_plugin_registry_multiple_types() {
        let registry = PluginRegistry::new();
        
        let input_info = PluginInfo {
            name: "input-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Input plugin".to_string(),
            author: "Test Author".to_string(),
            supported_features: vec!["input".to_string()],
            dependencies: vec![],
        };
        
        let output_info = PluginInfo {
            name: "output-plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Output plugin".to_string(),
            author: "Test Author".to_string(),
            supported_features: vec!["output".to_string()],
            dependencies: vec![],
        };
        
        registry.register(input_info, PluginType::Input, || "input");
        registry.register(output_info, PluginType::Output, || "output");
        
        assert_eq!(registry.count_by_type(&PluginType::Input), 1);
        assert_eq!(registry.count_by_type(&PluginType::Output), 1);
        assert_eq!(registry.count_by_type(&PluginType::Processing), 0);
    }
}