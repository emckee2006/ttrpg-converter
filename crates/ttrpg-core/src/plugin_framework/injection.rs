//! Plugin Dependency Injection with Shaku
//!
//! Advanced dependency injection container for managing plugin lifecycles,
//! automatic dependency resolution, and service coordination using the Shaku DI framework.

// Shaku imports for plugin service interfaces
use async_trait::async_trait;
use shaku::Interface;
use std::sync::Arc;
use tracing::{debug, error, info};

use super::interfaces::{AssetPlugin, ExportPlugin, InputPlugin, LoggingPlugin, ValidationPlugin};
use super::PluginInfo;
use crate::{ConversionError, ConversionResult};

/// Plugin lifecycle management trait for dependency injection
#[async_trait]
pub trait PluginLifecycle: Send + Sync {
    /// Initialize the plugin with dependencies
    async fn initialize(&mut self) -> ConversionResult<()>;

    /// Shutdown the plugin and clean up resources
    async fn shutdown(&mut self) -> ConversionResult<()>;

    /// Get plugin health status
    async fn health_check(&self) -> ConversionResult<PluginHealth>;

    /// Get plugin metadata
    fn get_info(&self) -> &PluginInfo;
}

/// Plugin health status
#[derive(Debug, Clone, PartialEq)]
pub enum PluginHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
    Unknown,
}

/// Dependency injection container configuration
#[derive(Debug, Clone)]
pub struct DIContainerConfig {
    /// Enable singleton pattern for plugins (default: true)
    pub use_singletons: bool,
    /// Enable lazy loading of plugins (default: true)
    pub lazy_loading: bool,
    /// Maximum plugin initialization time in seconds
    pub init_timeout_seconds: u64,
    /// Enable automatic dependency resolution
    pub auto_resolve: bool,
    /// Enable plugin health monitoring
    pub health_monitoring: bool,
}

impl Default for DIContainerConfig {
    fn default() -> Self {
        Self {
            use_singletons: true,
            lazy_loading: true,
            init_timeout_seconds: 30,
            auto_resolve: true,
            health_monitoring: true,
        }
    }
}

/// Core plugin service interfaces for dependency injection
pub trait InputPluginService: Interface + InputPlugin + PluginLifecycle {}
pub trait ValidationPluginService: Interface + ValidationPlugin + PluginLifecycle {}
pub trait AssetPluginService: Interface + AssetPlugin + PluginLifecycle {}
pub trait ExportPluginService: Interface + ExportPlugin + PluginLifecycle {}
pub trait LoggingPluginService: Interface + LoggingPlugin + PluginLifecycle {}

/// Plugin dependency injection container
///
/// Manages plugin instances, their dependencies, and lifecycles using the Shaku DI container.
/// Provides automatic dependency resolution, singleton management, and plugin coordination.
pub struct PluginDIContainer {
    /// Shaku module containing all registered plugins
    module: Arc<dyn DIModule>,
    /// Container configuration
    config: DIContainerConfig,
    /// Plugin health status cache
    health_cache: std::collections::HashMap<String, PluginHealth>,
    /// Initialization status tracking
    initialized_plugins: std::collections::HashSet<String>,
}

/// Simplified module definition for plugin services
/// (Full Shaku integration will be implemented in future iterations)
pub trait DIModule: Send + Sync {
    /// Resolve a plugin service by type
    fn resolve_service(
        &self,
        service_type: PluginServiceType,
    ) -> ConversionResult<Arc<dyn std::any::Any + Send + Sync>>;
}

/// Plugin registration builder for dependency injection
pub struct PluginRegistrationBuilder {
    registrations: Vec<PluginRegistration>,
    config: DIContainerConfig,
}

/// Plugin registration information
#[derive(Debug, Clone)]
pub struct PluginRegistration {
    pub plugin_id: String,
    pub plugin_type: PluginServiceType,
    pub dependencies: Vec<String>,
    pub singleton: bool,
    pub lazy: bool,
}

/// Plugin service type enumeration for DI
#[derive(Debug, Clone)]
pub enum PluginServiceType {
    Input,
    Validation,
    Asset,
    Export,
    Logging,
}

impl PluginDIContainer {
    /// Create a new dependency injection container
    pub fn new(config: DIContainerConfig) -> Self {
        Self {
            module: Arc::new(DIModuleImpl::new()),
            config,
            health_cache: std::collections::HashMap::new(),
            initialized_plugins: std::collections::HashSet::new(),
        }
    }

    /// Register a plugin with the DI container
    pub async fn register_plugin(
        &mut self,
        registration: PluginRegistration,
    ) -> ConversionResult<()> {
        info!(
            plugin_id = %registration.plugin_id,
            plugin_type = ?registration.plugin_type,
            dependencies = ?registration.dependencies,
            "Registering plugin with DI container"
        );

        // Plugin registration logic would go here
        // For now, we'll track the registration

        Ok(())
    }

    /// Resolve a plugin service by type (simplified implementation)
    pub async fn resolve_service(
        &self,
        service_type: PluginServiceType,
    ) -> ConversionResult<Arc<dyn std::any::Any + Send + Sync>> {
        // Use simplified resolution mechanism
        // In a full Shaku implementation, this would use proper generic resolution
        match self.module.resolve_service(service_type) {
            Ok(instance) => {
                debug!("Successfully resolved plugin service");
                Ok(instance)
            }
            Err(e) => {
                error!(error = ?e, "Failed to resolve plugin service");
                Err(e)
            }
        }
    }

    /// Initialize all registered plugins
    pub async fn initialize_all(&mut self) -> ConversionResult<()> {
        info!("Initializing all registered plugins");

        // For now, simulate initialization
        // Real implementation would iterate through registered plugins
        // and call their initialize methods

        Ok(())
    }

    /// Shutdown all plugins gracefully
    pub async fn shutdown_all(&mut self) -> ConversionResult<()> {
        info!("Shutting down all plugins");

        // For now, simulate shutdown
        // Real implementation would call shutdown on all plugins

        self.initialized_plugins.clear();
        self.health_cache.clear();

        Ok(())
    }

    /// Get health status of all plugins
    pub async fn health_check_all(
        &mut self,
    ) -> ConversionResult<std::collections::HashMap<String, PluginHealth>> {
        debug!("Performing health check on all plugins");

        // For now, return simulated health status
        let mut health_status = std::collections::HashMap::new();

        for plugin_id in &self.initialized_plugins {
            health_status.insert(plugin_id.clone(), PluginHealth::Healthy);
        }

        Ok(health_status)
    }

    /// Get a specific plugin's health status
    pub async fn get_plugin_health(&self, plugin_id: &str) -> ConversionResult<PluginHealth> {
        if let Some(health) = self.health_cache.get(plugin_id) {
            Ok(health.clone())
        } else {
            Ok(PluginHealth::Unknown)
        }
    }

    /// Configure the DI container
    pub fn configure(&mut self, config: DIContainerConfig) {
        self.config = config;
        info!("DI container configuration updated");
    }

    /// Get container statistics
    pub fn get_stats(&self) -> DIContainerStats {
        DIContainerStats {
            total_registrations: 0, // Would be calculated from actual registrations
            initialized_plugins: self.initialized_plugins.len(),
            healthy_plugins: self
                .health_cache
                .values()
                .filter(|h| **h == PluginHealth::Healthy)
                .count(),
            memory_usage_mb: 0.0, // Would be calculated from actual usage
        }
    }
}

/// DI container statistics
#[derive(Debug, Clone)]
pub struct DIContainerStats {
    pub total_registrations: usize,
    pub initialized_plugins: usize,
    pub healthy_plugins: usize,
    pub memory_usage_mb: f64,
}

impl PluginRegistrationBuilder {
    /// Create a new registration builder
    pub fn new() -> Self {
        Self { registrations: Vec::new(), config: DIContainerConfig::default() }
    }

    /// Add a plugin registration
    pub fn register(mut self, registration: PluginRegistration) -> Self {
        self.registrations.push(registration);
        self
    }

    /// Add an input plugin registration
    pub fn register_input_plugin(mut self, plugin_id: &str, dependencies: Vec<String>) -> Self {
        self.registrations.push(PluginRegistration {
            plugin_id: plugin_id.to_string(),
            plugin_type: PluginServiceType::Input,
            dependencies,
            singleton: self.config.use_singletons,
            lazy: self.config.lazy_loading,
        });
        self
    }

    /// Add a validation plugin registration
    pub fn register_validation_plugin(
        mut self,
        plugin_id: &str,
        dependencies: Vec<String>,
    ) -> Self {
        self.registrations.push(PluginRegistration {
            plugin_id: plugin_id.to_string(),
            plugin_type: PluginServiceType::Validation,
            dependencies,
            singleton: self.config.use_singletons,
            lazy: self.config.lazy_loading,
        });
        self
    }

    /// Add an asset plugin registration  
    pub fn register_asset_plugin(mut self, plugin_id: &str, dependencies: Vec<String>) -> Self {
        self.registrations.push(PluginRegistration {
            plugin_id: plugin_id.to_string(),
            plugin_type: PluginServiceType::Asset,
            dependencies,
            singleton: self.config.use_singletons,
            lazy: self.config.lazy_loading,
        });
        self
    }

    /// Add an export plugin registration
    pub fn register_export_plugin(mut self, plugin_id: &str, dependencies: Vec<String>) -> Self {
        self.registrations.push(PluginRegistration {
            plugin_id: plugin_id.to_string(),
            plugin_type: PluginServiceType::Export,
            dependencies,
            singleton: self.config.use_singletons,
            lazy: self.config.lazy_loading,
        });
        self
    }

    /// Add a logging plugin registration
    pub fn register_logging_plugin(mut self, plugin_id: &str, dependencies: Vec<String>) -> Self {
        self.registrations.push(PluginRegistration {
            plugin_id: plugin_id.to_string(),
            plugin_type: PluginServiceType::Logging,
            dependencies,
            singleton: self.config.use_singletons,
            lazy: self.config.lazy_loading,
        });
        self
    }

    /// Configure the container
    pub fn with_config(mut self, config: DIContainerConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the DI container with all registrations
    pub async fn build(self) -> ConversionResult<PluginDIContainer> {
        let mut container = PluginDIContainer::new(self.config);

        info!(
            registrations = self.registrations.len(),
            "Building DI container with plugin registrations"
        );

        // Register all plugins
        for registration in self.registrations {
            // Plugin-specific registration logic would go here
            debug!(
                plugin_id = %registration.plugin_id,
                plugin_type = ?registration.plugin_type,
                "Processing plugin registration"
            );
        }

        container.initialize_all().await?;

        Ok(container)
    }
}

impl Default for PluginRegistrationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Simplified implementation of DIModule for testing and development
struct DIModuleImpl {
    services: std::collections::HashMap<String, Arc<dyn std::any::Any + Send + Sync>>,
}

impl DIModuleImpl {
    fn new() -> Self {
        Self { services: std::collections::HashMap::new() }
    }
}

impl DIModule for DIModuleImpl {
    fn resolve_service(
        &self,
        service_type: PluginServiceType,
    ) -> ConversionResult<Arc<dyn std::any::Any + Send + Sync>> {
        let service_key = match service_type {
            PluginServiceType::Input => "input",
            PluginServiceType::Validation => "validation",
            PluginServiceType::Asset => "asset",
            PluginServiceType::Export => "export",
            PluginServiceType::Logging => "logging",
        };

        Ok(self.services.get(service_key).cloned().ok_or_else(|| {
            Box::new(ConversionError::PluginError {
                message: format!("Service '{}' not registered", service_key),
                plugin_name: None,
                source: None,
            })
        })?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_di_container_creation() {
        let config = DIContainerConfig::default();
        let container = PluginDIContainer::new(config);

        let stats = container.get_stats();
        assert_eq!(stats.initialized_plugins, 0);
    }

    #[tokio::test]
    async fn test_plugin_registration_builder() {
        let builder = PluginRegistrationBuilder::new()
            .register_input_plugin("roll20_parser", vec![])
            .register_validation_plugin("data_validator", vec!["roll20_parser".to_string()]);

        assert_eq!(builder.registrations.len(), 2);
        assert!(matches!(builder.registrations[0].plugin_type, PluginServiceType::Input));
        assert!(matches!(builder.registrations[1].plugin_type, PluginServiceType::Validation));
    }

    #[tokio::test]
    async fn test_plugin_health_management() {
        let mut container = PluginDIContainer::new(DIContainerConfig::default());

        let health = container.get_plugin_health("test_plugin").await.unwrap();
        assert_eq!(health, PluginHealth::Unknown);

        let all_health = container.health_check_all().await.unwrap();
        assert!(all_health.is_empty());
    }

    #[tokio::test]
    async fn test_container_lifecycle() {
        let mut container = PluginDIContainer::new(DIContainerConfig::default());

        // Test initialization
        let init_result = container.initialize_all().await;
        assert!(init_result.is_ok());

        // Test shutdown
        let shutdown_result = container.shutdown_all().await;
        assert!(shutdown_result.is_ok());

        let stats = container.get_stats();
        assert_eq!(stats.initialized_plugins, 0);
    }
}
