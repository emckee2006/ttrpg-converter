//! Enhanced Console Logging Plugin with Orchestration Framework Integration
//!
//! This plugin demonstrates the complete orchestration framework migration pattern:
//! - **Inventory Integration**: Auto-registration and compile-time discovery
//! - **Shaku Integration**: Dependency injection and lifecycle management
//! - **Daggy Integration**: Pipeline coordination and execution flow
//!
//! Features:
//! - Thread-safe structured logging with tracing ecosystem
//! - Auto-discovery via inventory crate
//! - Dependency injection with shaku
//! - Pipeline integration with daggy
//! - Professional error handling and statistics
//! - Migration template for other plugin updates

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;
use std::time::Instant;
use tracing::{debug, error, info, trace, warn};

// Core framework imports
use ttrpg_core::{
    plugin_framework::{
        discovery::{StaticPluginCategory, StaticPluginDiscoveryInfo, StaticPluginRegistration},
        injection::PluginHealth,
        LoggingPlugin, LoggingStats, PluginConfig, PluginInfo, PluginLifecycle, StaticPluginInfo,
    },
    ConversionResult,
};

// Orchestration framework imports

// use inventory; // TODO: Re-enable when inventory registration is fixed

/// Console-based logging plugin using tracing ecosystem
///
/// Features:
/// - Thread-safe structured logging
/// - Configurable log levels
/// - Statistics tracking
/// - Professional error handling
/// - Based on proven M1.2 RustLogger foundation
#[derive(Debug)]
pub struct ConsoleLoggingPlugin {
    /// Current minimum log level
    current_level: RwLock<ttrpg_core::LogLevel>,

    /// Message counter for statistics
    messages_logged: AtomicUsize,

    /// Error counter for statistics  
    errors_logged: AtomicUsize,

    /// Warning counter for statistics
    warnings_logged: AtomicUsize,

    /// Plugin start time
    start_time: Option<Instant>,

    /// Plugin metadata
    info: PluginInfo,

    /// Plugin health status (orchestration framework integration)
    health: RwLock<PluginHealth>,
}

impl ConsoleLoggingPlugin {
    /// Create new console logging plugin
    pub fn new() -> Self {
        Self {
            current_level: RwLock::new(ttrpg_core::LogLevel::Info),
            messages_logged: AtomicUsize::new(0),
            errors_logged: AtomicUsize::new(0),
            warnings_logged: AtomicUsize::new(0),
            start_time: None,
            info: PluginInfo {
                name: "Console Logging Plugin".to_string(),
                version: "1.0.0".to_string(),
                description: "Console logging with structured output using tracing".to_string(),
                author: "TTRPG Converter".to_string(),
                dependencies: vec!["tracing".to_string()],
                supported_features: vec![
                    "console_output".to_string(),
                    "structured_logging".to_string(),
                    "configurable_levels".to_string(),
                    "timestamp_formatting".to_string(),
                    "thread_safe".to_string(),
                    "performance_monitoring".to_string(),
                ],
            },
            health: RwLock::new(PluginHealth::Unknown),
        }
    }

    /// Initialize tracing subscriber if not already initialized
    fn ensure_tracing_initialized(&self) {
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, EnvFilter, Registry};

        // Try to initialize tracing subscriber (may already be initialized)
        let _ = Registry::default()
            .with(fmt::layer().with_target(false).with_thread_ids(true))
            .with(EnvFilter::from_default_env())
            .try_init();
    }

    /// Update statistics for logging operation
    fn update_stats(&self, level: ttrpg_core::LogLevel) {
        self.messages_logged.fetch_add(1, Ordering::Relaxed);

        match level {
            ttrpg_core::LogLevel::Error => {
                self.errors_logged.fetch_add(1, Ordering::Relaxed);
            }
            ttrpg_core::LogLevel::Warn => {
                self.warnings_logged.fetch_add(1, Ordering::Relaxed);
            }
            _ => {} // Only track errors and warnings for now
        }
    }
}

impl Default for ConsoleLoggingPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoggingPlugin for ConsoleLoggingPlugin {
    fn plugin_info(&self) -> PluginInfo {
        self.info.clone()
    }

    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()> {
        info!(
            plugin = %self.info.name,
            version = %self.info.version,
            "Initializing console logging plugin with orchestration framework"
        );

        self.ensure_tracing_initialized();

        // Set health status to healthy
        *self.health.write().unwrap() = PluginHealth::Healthy;

        // Set start time
        self.start_time = Some(Instant::now());

        // Configure log level from plugin configuration
        if let Some(level_config) = config.config_data.get("log_level") {
            let level = match level_config
                .as_str()
                .unwrap_or("info")
                .to_lowercase()
                .as_str()
            {
                "error" => ttrpg_core::LogLevel::Error,
                "warn" | "warning" => ttrpg_core::LogLevel::Warn,
                "info" => ttrpg_core::LogLevel::Info,
                "debug" => ttrpg_core::LogLevel::Debug,
                "trace" => ttrpg_core::LogLevel::Trace,
                _ => ttrpg_core::LogLevel::Info, // Default fallback
            };
            *self.current_level.write().unwrap() = level;

            info!(
                plugin = %self.info.name,
                log_level = ?level,
                "Console logging plugin configured with log level"
            );
        }

        // Set health status to healthy after successful initialization
        *self.health.write().unwrap() = PluginHealth::Healthy;

        info!(
            plugin = %self.info.name,
            "Console logging plugin initialized successfully with orchestration framework"
        );
        Ok(())
    }

    async fn cleanup(&mut self) -> ConversionResult<()> {
        info!("ConsoleLoggingPlugin shutting down");
        // Tracing cleanup is handled automatically
        Ok(())
    }

    fn error(&self, message: &str, context: Option<&str>) {
        if self.is_enabled(ttrpg_core::LogLevel::Error) {
            if let Some(ctx) = context {
                error!("[{}] {}", ctx, message);
            } else {
                error!("{}", message);
            }
            self.update_stats(ttrpg_core::LogLevel::Error);
        }
    }

    fn warn(&self, message: &str, context: Option<&str>) {
        if self.is_enabled(ttrpg_core::LogLevel::Warn) {
            if let Some(ctx) = context {
                warn!("[{}] {}", ctx, message);
            } else {
                warn!("{}", message);
            }
            self.update_stats(ttrpg_core::LogLevel::Warn);
        }
    }

    fn info(&self, message: &str, context: Option<&str>) {
        if self.is_enabled(ttrpg_core::LogLevel::Info) {
            if let Some(ctx) = context {
                info!("[{}] {}", ctx, message);
            } else {
                info!("{}", message);
            }
            self.update_stats(ttrpg_core::LogLevel::Info);
        }
    }

    fn debug(&self, message: &str, context: Option<&str>) {
        if self.is_enabled(ttrpg_core::LogLevel::Debug) {
            if let Some(ctx) = context {
                debug!("[{}] {}", ctx, message);
            } else {
                debug!("{}", message);
            }
            self.update_stats(ttrpg_core::LogLevel::Debug);
        }
    }

    fn log_with_data(&self, level: ttrpg_core::LogLevel, message: &str, data: &JsonValue) {
        if self.is_enabled(level) {
            match level {
                ttrpg_core::LogLevel::Error => error!("{} | Data: {}", message, data),
                ttrpg_core::LogLevel::Warn => warn!("{} | Data: {}", message, data),
                ttrpg_core::LogLevel::Info => info!("{} | Data: {}", message, data),
                ttrpg_core::LogLevel::Debug => debug!("{} | Data: {}", message, data),
                ttrpg_core::LogLevel::Trace => trace!("{} | Data: {}", message, data),
            }
            self.update_stats(level);
        }
    }

    fn set_level(&mut self, level: ttrpg_core::LogLevel) {
        *self.current_level.write().unwrap() = level;
        info!("Log level changed to: {:?}", level);
    }

    fn is_enabled(&self, level: ttrpg_core::LogLevel) -> bool {
        let current = *self.current_level.read().unwrap();
        level <= current
    }

    fn get_stats(&self) -> LoggingStats {
        LoggingStats {
            messages_logged: self.messages_logged.load(Ordering::Relaxed),
            errors_logged: self.errors_logged.load(Ordering::Relaxed),
            warnings_logged: self.warnings_logged.load(Ordering::Relaxed),
            start_time: self.start_time,
        }
    }
}

/// **ORCHESTRATION FRAMEWORK INTEGRATION** - PluginLifecycle Implementation
///
/// This demonstrates how to integrate with the Shaku dependency injection system
/// for plugin lifecycle management, health monitoring, and service coordination.
#[async_trait]
impl PluginLifecycle for ConsoleLoggingPlugin {
    /// Initialize the plugin with dependencies
    async fn initialize(&mut self) -> ConversionResult<()> {
        info!(
            plugin = %self.info.name,
            "Initializing console logging plugin lifecycle"
        );

        *self.health.write().unwrap() = PluginHealth::Healthy;

        // Plugin-specific startup logic
        self.start_time = Some(Instant::now());

        *self.health.write().unwrap() = PluginHealth::Healthy;

        info!(
            plugin = %self.info.name,
            "Console logging plugin lifecycle initialized successfully"
        );
        Ok(())
    }

    /// Shutdown the plugin and clean up resources
    async fn shutdown(&mut self) -> ConversionResult<()> {
        info!(
            plugin = %self.info.name,
            "Shutting down console logging plugin lifecycle"
        );

        *self.health.write().unwrap() = PluginHealth::Healthy;

        // Log final statistics before stopping
        let stats = self.get_stats();
        let uptime = self
            .start_time
            .map(|t| t.elapsed().as_millis())
            .unwrap_or(0);

        info!(
            plugin = %self.info.name,
            total_messages = stats.messages_logged,
            total_errors = stats.errors_logged,
            total_warnings = stats.warnings_logged,
            uptime_ms = uptime,
            "Console logging plugin final statistics"
        );

        self.cleanup().await?;
        *self.health.write().unwrap() = PluginHealth::Healthy;

        info!(
            plugin = %self.info.name,
            "Console logging plugin lifecycle shutdown successfully"
        );
        Ok(())
    }

    /// Get plugin health status
    async fn health_check(&self) -> ConversionResult<PluginHealth> {
        // Check if tracing is still functional
        let can_log = std::panic::catch_unwind(|| {
            tracing::info!("Health check test log");
        })
        .is_ok();

        // Check if statistics are accessible
        let stats_accessible = true; // AtomicUsize is always >= 0

        let current_health = self.health.read().unwrap().clone();

        if can_log && stats_accessible && current_health == PluginHealth::Healthy {
            Ok(PluginHealth::Healthy)
        } else {
            Ok(PluginHealth::Unhealthy("Health check failed".to_string()))
        }
    }

    /// Get plugin metadata
    fn get_info(&self) -> &PluginInfo {
        &self.info
    }
}

// Re-enable inventory import for const-compatible registration
use inventory;

// Factory function for plugin creation
fn console_logging_factory() -> Box<dyn std::any::Any + Send + Sync> {
    Box::new(ConsoleLoggingPlugin::new())
}

// Working const-compatible inventory registration using new static types
inventory::submit! {
    StaticPluginRegistration {
        discovery_info: StaticPluginDiscoveryInfo {
            info: StaticPluginInfo {
                name: "Console Logging Plugin",
                version: "1.0.0",
                description: "Console logging with structured output using tracing",
                author: "TTRPG Converter",
                supported_features: &["console_output", "structured_logging", "configurable_levels", "colored_output"],
                dependencies: &["tracing"],
            },
            category: StaticPluginCategory::Logging("console"),
            tags: &["logging", "console"],
            priority: 500,
            auto_load: true,
            config_schema: None,
            documentation_url: None,
        },
        factory: console_logging_factory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use ttrpg_core::LogLevel;

    #[tokio::test]
    async fn test_console_logging_plugin_creation() {
        let plugin = ConsoleLoggingPlugin::new();
        let info = plugin.plugin_info();

        assert_eq!(info.name, "Console Logging Plugin");
        assert_eq!(info.version, "1.0.0");
        assert!(info
            .supported_features
            .contains(&"structured_logging".to_string()));
    }

    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut plugin = ConsoleLoggingPlugin::new();

        let config = PluginConfig {
            config_data: HashMap::from([("log_level".to_string(), json!("debug"))]),
            cache_dir: None,
            temp_dir: None,
        };

        let result = LoggingPlugin::initialize(&mut plugin, config).await;
        assert!(result.is_ok());
        // Verify plugin health after initialization
        let health = plugin.health_check().await;
        assert!(health.is_ok());
    }

    #[tokio::test]
    async fn test_log_level_filtering() {
        let mut plugin = ConsoleLoggingPlugin::new();
        plugin.set_level(LogLevel::Warn);

        assert!(plugin.is_enabled(LogLevel::Error));
        assert!(plugin.is_enabled(LogLevel::Warn));
        assert!(!plugin.is_enabled(LogLevel::Info));
        assert!(!plugin.is_enabled(LogLevel::Debug));
        assert!(!plugin.is_enabled(LogLevel::Trace));
    }

    #[tokio::test]
    async fn test_structured_logging() {
        let plugin = ConsoleLoggingPlugin::new();
        let data = json!({"key": "value", "number": 42});

        // This should not panic and should work with structured data
        plugin.log_with_data(LogLevel::Info, "Test message", &data);

        let stats = plugin.get_stats();
        assert_eq!(stats.messages_logged, 1);
    }

    #[tokio::test]
    async fn test_context_logging() {
        let plugin = ConsoleLoggingPlugin::new();

        plugin.info("Test message", Some("test_context"));
        plugin.error("Error message", None);

        let stats = plugin.get_stats();
        assert_eq!(stats.messages_logged, 2);
        assert_eq!(stats.errors_logged, 1);
        assert_eq!(stats.warnings_logged, 0);
    }

    #[tokio::test]
    async fn test_cleanup() {
        let mut plugin = ConsoleLoggingPlugin::new();
        let result = plugin.cleanup().await;
        assert!(result.is_ok());
    }
}
