//! Structured logging service implementation
//!
//! The RustLogger provides structured logging with tracing, supporting both console
//! and file output with configurable levels and formatting. It offers
//! excellent performance and flexibility.

// Allow clippy lints temporarily while we stabilize the core implementation
#![allow(clippy::result_large_err, clippy::uninlined_format_args)]

use crate::error::{ConversionError, ConversionResult};
use crate::services::{LogLevel, LoggingService};
use std::path::Path;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

/// Configuration for the logging service
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log level filter (trace, debug, info, warn, error)
    pub level: String,
    /// Enable console output
    pub console: bool,
    /// Optional file output path
    pub file: Option<String>,
    /// Enable JSON formatting
    pub json: bool,
    /// Enable colored output (console only)
    pub colored: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self { level: "info".to_string(), console: true, file: None, json: false, colored: true }
    }
}

/// High-performance structured logging service
///
/// Provides structured logging with multiple output formats, thread-safe operation,
/// and excellent performance characteristics. Built on the tracing ecosystem.
pub struct RustLogger {
    config: LoggingConfig,
    _guard: Option<tracing_appender::non_blocking::WorkerGuard>,
}

impl RustLogger {
    /// Create a new RustLogger instance
    ///
    /// # Arguments
    /// * `config` - Logging configuration
    ///
    /// # Returns
    /// * `ConversionResult<Self>` - The configured logger instance
    ///
    /// # Errors
    /// Returns `ConversionError::ConfigError` if logging setup fails
    pub fn new(config: LoggingConfig) -> ConversionResult<Self> {
        let mut logger = Self { config: config.clone(), _guard: None };

        logger.initialize_logging()?;

        Ok(logger)
    }

    /// Initialize the logging system with configured outputs
    fn initialize_logging(&mut self) -> ConversionResult<()> {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(&self.config.level));

        let registry = Registry::default().with(env_filter);

        match (self.config.console, self.config.file.as_ref()) {
            (true, Some(file_path)) => {
                let file_path = Path::new(file_path);
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ConversionError::ValidationError {
                            entity_type: "logging".to_string(),
                            message: format!("Failed to create log directory: {}", e),
                            field: Some("log_directory".to_string()),
                        }
                    })?;
                }

                let log_dir = file_path.parent().unwrap_or(Path::new("."));
                let file_appender = tracing_appender::rolling::daily(
                    log_dir,
                    file_path
                        .file_name()
                        .unwrap_or(std::ffi::OsStr::new("ttrpg-converter.log")),
                );

                let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
                self._guard = Some(guard);

                let console_layer = if self.config.json {
                    fmt::layer()
                        .json()
                        .with_target(false)
                        .with_current_span(false)
                        .boxed()
                } else {
                    let mut layer = fmt::layer().with_target(false);

                    if self.config.colored {
                        layer = layer.with_ansi(true);
                    }

                    layer.boxed()
                };

                let file_layer = if self.config.json {
                    fmt::layer()
                        .json()
                        .with_writer(non_blocking)
                        .with_target(true)
                        .boxed()
                } else {
                    fmt::layer()
                        .with_writer(non_blocking)
                        .with_target(true)
                        .with_ansi(false)
                        .boxed()
                };

                registry.with(console_layer).with(file_layer).init();
            }

            (true, None) => {
                let console_layer = if self.config.json {
                    fmt::layer().json().with_target(false).boxed()
                } else {
                    let mut layer = fmt::layer().with_target(false);

                    if self.config.colored {
                        layer = layer.with_ansi(true);
                    }

                    layer.boxed()
                };

                registry.with(console_layer).init();
            }

            (false, Some(file_path)) => {
                let file_path = Path::new(file_path);
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        ConversionError::ValidationError {
                            entity_type: "logging".to_string(),
                            message: format!("Failed to create log directory: {}", e),
                            field: Some("log_directory".to_string()),
                        }
                    })?;
                }

                let log_dir = file_path.parent().unwrap_or(Path::new("."));
                let file_appender = tracing_appender::rolling::daily(
                    log_dir,
                    file_path
                        .file_name()
                        .unwrap_or(std::ffi::OsStr::new("ttrpg-converter.log")),
                );

                let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
                self._guard = Some(guard);

                let file_layer = if self.config.json {
                    fmt::layer()
                        .json()
                        .with_writer(non_blocking)
                        .with_target(true)
                        .boxed()
                } else {
                    fmt::layer()
                        .with_writer(non_blocking)
                        .with_target(true)
                        .with_ansi(false)
                        .boxed()
                };

                registry.with(file_layer).init();
            }

            (false, None) => {
                // No logging configured - just initialize with basic registry
                registry.init();
            }
        }

        info!("RustLogger initialized with config: {:?}", self.config);
        Ok(())
    }

    /// Create a new logger with default configuration
    pub fn with_defaults() -> ConversionResult<Self> {
        Self::new(LoggingConfig::default())
    }

    /// Create a new logger with console output only
    pub fn console_only() -> ConversionResult<Self> {
        let config = LoggingConfig { console: true, file: None, ..Default::default() };
        Self::new(config)
    }

    /// Create a new logger with file output
    pub fn with_file<P: AsRef<Path>>(file_path: P) -> ConversionResult<Self> {
        let config = LoggingConfig {
            file: Some(file_path.as_ref().to_string_lossy().to_string()),
            ..Default::default()
        };
        Self::new(config)
    }
}

impl LoggingService for RustLogger {
    fn error(&self, message: &str, context: Option<&str>) {
        if let Some(ctx) = context {
            error!(context = ctx, "{}", message);
        } else {
            error!("{}", message);
        }
    }

    fn warn(&self, message: &str, context: Option<&str>) {
        if let Some(ctx) = context {
            warn!(context = ctx, "{}", message);
        } else {
            warn!("{}", message);
        }
    }

    fn info(&self, message: &str, context: Option<&str>) {
        if let Some(ctx) = context {
            info!(context = ctx, "{}", message);
        } else {
            info!("{}", message);
        }
    }

    fn debug(&self, message: &str, context: Option<&str>) {
        if let Some(ctx) = context {
            debug!(context = ctx, "{}", message);
        } else {
            debug!("{}", message);
        }
    }

    fn log_with_data(&self, level: LogLevel, message: &str, data: &serde_json::Value) {
        match level {
            LogLevel::Error => error!(data = ?data, "{}", message),
            LogLevel::Warn => warn!(data = ?data, "{}", message),
            LogLevel::Info => info!(data = ?data, "{}", message),
            LogLevel::Debug => debug!(data = ?data, "{}", message),
            LogLevel::Trace => trace!(data = ?data, "{}", message),
        }
    }

    fn set_level(&mut self, level: LogLevel) {
        // Note: tracing level changes require reinitializing the subscriber
        // For now, we'll log this as a request that would require restart
        info!("Log level change requested: {:?} (requires restart)", level);
    }

    fn is_enabled(&self, level: LogLevel) -> bool {
        match level {
            LogLevel::Error => tracing::enabled!(tracing::Level::ERROR),
            LogLevel::Warn => tracing::enabled!(tracing::Level::WARN),
            LogLevel::Info => tracing::enabled!(tracing::Level::INFO),
            LogLevel::Debug => tracing::enabled!(tracing::Level::DEBUG),
            LogLevel::Trace => tracing::enabled!(tracing::Level::TRACE),
        }
    }
}

// ============================================================================
// THREAD SAFETY
// ============================================================================

// Thread-safety: RustLogger is Send + Sync
unsafe impl Send for RustLogger {}
unsafe impl Sync for RustLogger {}

// ============================================================================
// COMPREHENSIVE UNIT TESTS
// ============================================================================

#[cfg(test)]
mod logging_tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::tempdir;

    fn init_test_logger() -> RustLogger {
        // Always create a minimal logger for testing without attempting to set global subscriber
        // This avoids the "global default trace dispatcher has already been set" error
        RustLogger {
            config: LoggingConfig {
                level: "info".to_string(),
                console: true,
                file: None,
                json: false,
                colored: false,
            },
            _guard: None,
        }
    }

    /// Test logger configuration creation
    #[test]
    fn test_logger_config_creation() {
        let config = LoggingConfig {
            level: "info".to_string(),
            console: true,
            file: None,
            json: false,
            colored: false,
        };

        assert_eq!(config.level, "info");
        assert!(config.console);
        assert!(config.file.is_none());
        assert!(!config.json);
        assert!(!config.colored);
    }

    /// Test logger configuration with file output
    #[test]
    fn test_file_config_creation() {
        let temp_dir = tempdir().unwrap();
        let log_file = temp_dir.path().join("test.log");

        let config = LoggingConfig {
            level: "debug".to_string(),
            console: false,
            file: Some(log_file.to_string_lossy().to_string()),
            json: false,
            colored: false,
        };

        assert_eq!(config.level, "debug");
        assert!(!config.console);
        assert!(config.file.is_some());
        assert!(!config.json);
        assert!(!config.colored);
    }

    /// Test logger configuration with all options enabled
    #[test]
    fn test_full_config_creation() {
        let temp_dir = tempdir().unwrap();
        let log_file = temp_dir.path().join("test.log");

        let config = LoggingConfig {
            level: "warn".to_string(),
            console: true,
            file: Some(log_file.to_string_lossy().to_string()),
            json: true,
            colored: true,
        };

        assert_eq!(config.level, "warn");
        assert!(config.console);
        assert!(config.file.is_some());
        assert!(config.json);
        assert!(config.colored);
    }

    /// Test logging service trait methods (no global subscriber required)
    #[test]
    fn test_logging_service_methods() {
        let logger = init_test_logger();

        // Test basic logging methods (should not panic)
        logger.error("Test error message", None);
        logger.warn("Test warning message", Some("test_context"));
        logger.info("Test info message", None);
        logger.debug("Test debug message", Some("debug_context"));
    }

    /// Test log_with_data method
    #[test]
    fn test_log_with_data() {
        let logger = init_test_logger();
        let test_data = serde_json::json!({
            "test_key": "test_value",
            "number": 42
        });

        // Test all log levels with structured data
        logger.log_with_data(LogLevel::Error, "Error with data", &test_data);
        logger.log_with_data(LogLevel::Warn, "Warning with data", &test_data);
        logger.log_with_data(LogLevel::Info, "Info with data", &test_data);
        logger.log_with_data(LogLevel::Debug, "Debug with data", &test_data);
        logger.log_with_data(LogLevel::Trace, "Trace with data", &test_data);
    }

    /// Test set_level method
    #[test]
    fn test_set_level() {
        let mut logger = init_test_logger();

        // Test setting different log levels (should not panic)
        logger.set_level(LogLevel::Debug);
        logger.set_level(LogLevel::Error);
        logger.set_level(LogLevel::Trace);
    }

    /// Test is_enabled method for different log levels
    #[test]
    fn test_is_enabled() {
        let logger = init_test_logger();

        // Test that the method returns boolean values without panicking
        let _error_enabled = logger.is_enabled(LogLevel::Error);
        let _warn_enabled = logger.is_enabled(LogLevel::Warn);
        let _info_enabled = logger.is_enabled(LogLevel::Info);
        let _debug_enabled = logger.is_enabled(LogLevel::Debug);
        let _trace_enabled = logger.is_enabled(LogLevel::Trace);
    }

    /// Test LogLevel enum ordering and comparison
    #[test]
    fn test_log_level_comparison() {
        // Error = 0, Warn = 1, Info = 2, Debug = 3, Trace = 4
        // Lower numbers mean higher priority, so Error < Warn < Info < Debug < Trace
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);

        assert!(LogLevel::Trace > LogLevel::Debug);
        assert!(LogLevel::Debug > LogLevel::Info);
        assert!(LogLevel::Info > LogLevel::Warn);
        assert!(LogLevel::Warn > LogLevel::Error);
    }

    /// Test LogLevel string conversion
    #[test]
    fn test_log_level_string_conversion() {
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Trace.to_string(), "TRACE");
    }

    /// Test concurrent logging (basic thread safety)
    #[test]
    fn test_concurrent_logging() {
        use std::thread;

        let logger = Arc::new(init_test_logger());
        let mut handles = vec![];

        // Spawn multiple threads to test concurrent logging
        for i in 0..3 {
            let logger_clone = Arc::clone(&logger);
            let handle = thread::spawn(move || {
                logger_clone.info(&format!("Thread {} logging", i), None);
                logger_clone.error(&format!("Thread {} error", i), Some("concurrent_test"));
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    }

    /// Test configuration validation patterns
    #[test]
    fn test_configuration_patterns() {
        // Test console-only configuration
        let console_config = LoggingConfig {
            level: "info".to_string(),
            console: true,
            file: None,
            json: false,
            colored: false,
        };
        assert!(console_config.console);
        assert!(console_config.file.is_none());

        // Test file-only configuration
        let file_config = LoggingConfig {
            level: "debug".to_string(),
            console: false,
            file: Some("test.log".to_string()),
            json: true,
            colored: false,
        };
        assert!(!file_config.console);
        assert!(file_config.file.is_some());
        assert!(file_config.json);

        // Test mixed configuration
        let mixed_config = LoggingConfig {
            level: "warn".to_string(),
            console: true,
            file: Some("test.log".to_string()),
            json: false,
            colored: true,
        };
        assert!(mixed_config.console);
        assert!(mixed_config.file.is_some());
        assert!(mixed_config.colored);
    }

    /// Test configuration edge cases
    #[test]
    fn test_configuration_edge_cases() {
        // Test configuration with no output (edge case)
        let no_output_config = LoggingConfig {
            level: "info".to_string(),
            console: false,
            file: None,
            json: false,
            colored: false,
        };
        assert!(!no_output_config.console);
        assert!(no_output_config.file.is_none());

        // Test configuration with all features enabled
        let full_config = LoggingConfig {
            level: "trace".to_string(),
            console: true,
            file: Some("full.log".to_string()),
            json: true,
            colored: true,
        };
        assert!(full_config.console);
        assert!(full_config.file.is_some());
        assert!(full_config.json);
        assert!(full_config.colored);
        assert_eq!(full_config.level, "trace");
    }
}
