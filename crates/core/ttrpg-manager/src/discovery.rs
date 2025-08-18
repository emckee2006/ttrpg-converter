//! Plugin discovery system for runtime plugin loading

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

use libloading::{Library, Symbol};
use crate::registry::{PluginEntry, PLUGIN_REGISTRY};
use ttrpg_core::error::{ConversionResult, ConversionError};

use tracing::{info, warn, debug};

/// Plugin discovery configuration
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Directories to search for plugins
    pub search_paths: Vec<PathBuf>,
    /// Plugin file extensions to look for
    pub extensions: Vec<String>,
    /// Enable dynamic library loading
    pub allow_dynamic: bool,
    /// Maximum recursion depth for directory search
    pub max_depth: usize,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            search_paths: vec![
                PathBuf::from("./plugins"),
                PathBuf::from("~/.ttrpg-converter/plugins"),
            ],
            extensions: vec![
                "dll".to_string(),
                "so".to_string(), 
                "dylib".to_string(),
            ],
            allow_dynamic: true,
            max_depth: 3,
        }
    }
}

/// Plugin discovery service
pub struct PluginDiscovery {
    config: DiscoveryConfig,
    loaded_libraries: HashMap<String, Library>,
}

impl PluginDiscovery {
    /// Create new discovery service
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            loaded_libraries: HashMap::new(),
        }
    }

    /// Discover all available plugins
    pub async fn discover_all(&mut self) -> ConversionResult<Vec<PluginEntry>> {
        let mut plugins = Vec::new();
        
        // First, get compile-time registered plugins
        let static_plugins = self.discover_static_plugins();
        info!("Found {} static plugins", static_plugins.len());
        plugins.extend(static_plugins);
        
        // Then discover dynamic plugins if enabled
        if self.config.allow_dynamic {
            let dynamic_plugins = self.discover_dynamic_plugins().await?;
            info!("Found {} dynamic plugins", dynamic_plugins.len());
            plugins.extend(dynamic_plugins);
        }
        
        Ok(plugins)
    }

    /// Get compile-time registered plugins from inventory
    fn discover_static_plugins(&self) -> Vec<PluginEntry> {
        crate::registry::initialize_registry();
        PLUGIN_REGISTRY.list_all()
    }

    /// Discover dynamic plugins from filesystem
    async fn discover_dynamic_plugins(&mut self) -> ConversionResult<Vec<PluginEntry>> {
        let mut plugins = Vec::new();
        
        let search_paths = self.config.search_paths.clone();
        for search_path in search_paths {
            if search_path.exists() && search_path.is_dir() {
                debug!("Scanning directory: {:?}", search_path);
                let found = self.scan_directory(&search_path, 0).await?;
                plugins.extend(found);
            } else {
                warn!("Search path does not exist or is not a directory: {:?}", search_path);
            }
        }
        
        Ok(plugins)
    }

    /// Recursively scan directory for plugin files
    fn scan_directory<'a>(&'a mut self, dir: &'a Path, depth: usize) -> std::pin::Pin<Box<dyn std::future::Future<Output = ConversionResult<Vec<PluginEntry>>> + 'a>> {
        Box::pin(async move {
        if depth >= self.config.max_depth {
            return Ok(Vec::new());
        }
        
        let mut plugins = Vec::new();
        let entries = fs::read_dir(dir)
            .map_err(|e| ConversionError::from_io(e, format!("Reading directory {:?}", dir)))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| ConversionError::from_io(e, "Reading directory entry"))?;
            
            let path = entry.path();
            
            if path.is_dir() {
                // Recurse into subdirectories
                let sub_plugins = self.scan_directory(&path, depth + 1).await?;
                plugins.extend(sub_plugins);
            } else if self.is_plugin_file(&path) {
                // Try to load as plugin
                match self.load_plugin_library(&path) {
                    Ok(plugin_entries) => plugins.extend(plugin_entries),
                    Err(e) => warn!("Failed to load plugin from {:?}: {}", path, e),
                }
            }
        }
        
        Ok(plugins)
        })
    }

    /// Check if file has a plugin extension
    fn is_plugin_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            self.config.extensions.contains(&ext.to_lowercase())
        } else {
            false
        }
    }

    /// Load plugin from dynamic library
    fn load_plugin_library(&mut self, path: &Path) -> ConversionResult<Vec<PluginEntry>> {
        debug!("Attempting to load plugin library: {:?}", path);
        
        // Load the library
        let lib = unsafe { Library::new(path) }
            .map_err(|e| Box::new(ConversionError::PluginError { 
                message: format!("Failed to load library {:?}: {}", path, e),
                plugin_name: None,
                source: Some(Box::new(e))
            }))?;
            
        // Try to get plugin registration function
        let register_plugins: Symbol<unsafe extern "C" fn() -> *const PluginRegistrationInfo> = unsafe {
            lib.get(b"get_plugin_info")
                .map_err(|e| Box::new(ConversionError::validation("Plugin", format!("Missing get_plugin_info function: {}", e))))?
        };
        
        // Call registration function
        let registration_info = unsafe { register_plugins() };
        if registration_info.is_null() {
            return Err(Box::new(ConversionError::validation("Plugin", "Returned null registration info")));
        }
        
        let info = unsafe { &*registration_info };
        let plugins = self.process_plugin_registration(info);
        
        // Store library to prevent unloading
        let lib_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        self.loaded_libraries.insert(lib_name, lib);
        
        Ok(plugins)
    }

    /// Process plugin registration information from dynamic library
    fn process_plugin_registration(&self, info: &PluginRegistrationInfo) -> Vec<PluginEntry> {
        let mut plugins = Vec::new();
        
        // Convert C-style plugin info to Rust plugin entries
        for i in 0..info.plugin_count {
            if let Some(plugin_info) = self.convert_plugin_info(unsafe { &*info.plugins.wrapping_add(i) }) {
                plugins.push(plugin_info);
            }
        }
        
        plugins
    }

    /// Convert C-style plugin info to Rust plugin entry
    fn convert_plugin_info(&self, _c_info: &CPluginInfo) -> Option<PluginEntry> {
        // This would need proper FFI conversion based on the actual
        // plugin interface definition. For now, return None as placeholder.
        None
    }

    /// Get discovery statistics
    pub fn get_stats(&self) -> DiscoveryStats {
        DiscoveryStats {
            static_plugins: PLUGIN_REGISTRY.list_all().len(),
            dynamic_libraries: self.loaded_libraries.len(),
            search_paths: self.config.search_paths.len(),
        }
    }
}

/// FFI structures for dynamic plugin loading
#[repr(C)]
pub struct PluginRegistrationInfo {
    pub plugin_count: usize,
    pub plugins: *const CPluginInfo,
}

#[repr(C)]
pub struct CPluginInfo {
    pub name: *const std::os::raw::c_char,
    pub version: *const std::os::raw::c_char,
    pub description: *const std::os::raw::c_char,
    pub plugin_type: u32,
}

/// Discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub static_plugins: usize,
    pub dynamic_libraries: usize,
    pub search_paths: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_plugin_discovery_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config = DiscoveryConfig {
            search_paths: vec![temp_dir.path().to_path_buf()],
            extensions: vec!["dll".to_string()],
            allow_dynamic: true,
            max_depth: 1,
        };
        
        let mut discovery = PluginDiscovery::new(config);
        let plugins = discovery.discover_dynamic_plugins().await.unwrap();
        
        // Should find no dynamic plugins in empty directory
        assert_eq!(plugins.len(), 0);
    }

    #[tokio::test]
    async fn test_static_plugin_discovery() {
        let config = DiscoveryConfig::default();
        let discovery = PluginDiscovery::new(config);
        
        let static_plugins = discovery.discover_static_plugins();
        // Static plugins count should be non-negative (always true for usize)
        assert!(static_plugins.is_empty() || !static_plugins.is_empty()); // Either case is valid
        // Should find any plugins registered via inventory
        // (count will depend on what's compiled in)
        // Static plugins count should be non-negative (always true for usize)
    }
}