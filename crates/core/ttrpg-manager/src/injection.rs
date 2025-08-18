//! Plugin Dependency Injection - Simplified for Manager Crate
//!
//! Plugin lifecycle management and dependency injection without the complexity 
//! of Shaku until we get core compilation stable.

use async_trait::async_trait;
use std::sync::Arc;
use ttrpg_core::error::ConversionResult;
use ttrpg_traits::PluginInfo;

/// Plugin lifecycle management trait
#[async_trait]
pub trait PluginLifecycle: Send + Sync {
    /// Initialize the plugin
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

/// Plugin service registration types
#[derive(Debug, Clone, PartialEq)]
pub enum PluginServiceType {
    Input,
    Output,
    Asset,
    Validation,
    Logging,
    Export,
}

/// Plugin registration information
#[derive(Debug, Clone)]
pub struct PluginRegistration {
    pub info: PluginInfo,
    pub service_type: PluginServiceType,
    pub priority: i32,
}

/// Plugin registration builder
pub struct PluginRegistrationBuilder {
    info: Option<PluginInfo>,
    service_type: Option<PluginServiceType>,
    priority: i32,
}

impl PluginRegistrationBuilder {
    pub fn new() -> Self {
        Self {
            info: None,
            service_type: None,
            priority: 0,
        }
    }

    pub fn with_info(mut self, info: PluginInfo) -> Self {
        self.info = Some(info);
        self
    }

    pub fn with_service_type(mut self, service_type: PluginServiceType) -> Self {
        self.service_type = Some(service_type);
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn build(self) -> ConversionResult<PluginRegistration> {
        let info = self.info.ok_or_else(|| {
            ttrpg_core::error::ConversionError::validation("PluginRegistrationBuilder", "Plugin info is required")
        })?;

        let service_type = self.service_type.ok_or_else(|| {
            ttrpg_core::error::ConversionError::validation("PluginRegistrationBuilder", "Service type is required")
        })?;

        Ok(PluginRegistration {
            info,
            service_type,
            priority: self.priority,
        })
    }
}

impl Default for PluginRegistrationBuilder {
    fn default() -> Self {
        Self::new()
    }
}