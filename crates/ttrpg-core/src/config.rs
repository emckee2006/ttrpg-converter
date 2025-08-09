//! Professional configuration management using `figment`
//!
//! This module provides hierarchical configuration support with automatic
//! loading from TOML files, environment variables, and sensible defaults.
//!
//! # Configuration Hierarchy (highest to lowest priority)
//!
//! 1. **Environment Variables** - `TTRPG_*` prefixed variables
//! 2. **User Config File** - `~/.config/ttrpg-converter/config.toml`
//! 3. **Project Config File** - `./ttrpg-converter.toml`
//! 4. **Built-in Defaults** - Sensible defaults for all settings
//!
//! # Usage
//!
//! ```rust
//! use ttrpg_core::config::TTRPGConfig;
//!
//! // Load configuration with full hierarchy support
//! let config = TTRPGConfig::load()?;
//!
//! // Access configuration values
//! println!("Log level: {}", config.logging.level);
//! println!("Cache size: {} MB", config.cache.max_size_mb);
//! ```
//!
//! # Example Configuration File (`ttrpg-converter.toml`)
//!
//! ```toml
//! [logging]
//! level = "info"
//! format = "pretty"
//! file_enabled = true
//! console_enabled = true
//!
//! [validation]
//! strict_mode = false
//! max_file_size_mb = 100
//! timeout_seconds = 30
//!
//! [cache]
//! enabled = true
//! max_size_mb = 512
//! ttl_seconds = 3600
//!
//! [processing]
//! parallel_downloads = 8
//! retry_attempts = 3
//! timeout_seconds = 60
//!
//! [output]
//! format = "foundry"
//! compress_assets = true
//! preserve_structure = true
//! ```

use crate::error::{ConversionResult, ConversionError};
use figment::{Figment, providers::{Format, Toml, Env}};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// MAIN CONFIGURATION STRUCTURE
// ============================================================================

/// Comprehensive TTRPGConverter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTRPGConfig {
    /// Logging configuration
    pub logging: LoggingConfig,
    
    /// Validation configuration
    pub validation: ValidationConfig,
    
    /// Cache configuration
    pub cache: CacheConfig,
    
    /// Asset processing configuration
    pub processing: ProcessingConfig,
    
    /// Output generation configuration
    pub output: OutputConfig,
    
    /// Network/HTTP configuration
    pub network: NetworkConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level: trace, debug, info, warn, error
    pub level: String,
    
    /// Log format: pretty, json, compact
    pub format: String,
    
    /// Enable file logging
    pub file_enabled: bool,
    
    /// Enable console logging
    pub console_enabled: bool,
    
    /// Log file directory
    pub file_directory: PathBuf,
    
    /// Log file name pattern
    pub file_name_pattern: String,
    
    /// Maximum log file size in MB
    pub max_file_size_mb: u64,
    
    /// Number of archived log files to keep
    pub max_archived_files: u32,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable strict validation mode
    pub strict_mode: bool,
    
    /// Maximum file size for validation in MB
    pub max_file_size_mb: u64,
    
    /// Validation timeout in seconds
    pub timeout_seconds: u64,
    
    /// Enable business rules validation
    pub business_rules_enabled: bool,
    
    /// Enable asset integrity checking
    pub integrity_checking_enabled: bool,
    
    /// Custom validation rules file path
    pub custom_rules_file: Option<PathBuf>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Maximum cache size in MB
    pub max_size_mb: u64,
    
    /// Cache entry time-to-live in seconds
    pub ttl_seconds: u64,
    
    /// Cache directory path
    pub directory: PathBuf,
    
    /// Enable persistent disk cache
    pub persistent_enabled: bool,
    
    /// Cache cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
}

/// Asset processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
    /// Number of parallel downloads
    pub parallel_downloads: usize,
    
    /// Number of retry attempts for failed operations
    pub retry_attempts: u32,
    
    /// Operation timeout in seconds
    pub timeout_seconds: u64,
    
    /// Enable asset optimization
    pub optimize_assets: bool,
    
    /// Enable image resizing
    pub resize_images: bool,
    
    /// Maximum image dimensions
    pub max_image_width: u32,
    pub max_image_height: u32,
    
    /// Enable WebP conversion
    pub convert_to_webp: bool,
    
    /// WebP quality (1-100)
    pub webp_quality: u8,
}

/// Output configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    /// Output format: foundry, roll20, universal
    pub format: String,
    
    /// Enable asset compression
    pub compress_assets: bool,
    
    /// Preserve original directory structure
    pub preserve_structure: bool,
    
    /// Output directory path
    pub directory: PathBuf,
    
    /// Enable incremental updates
    pub incremental_enabled: bool,
    
    /// Generate backup before overwriting
    pub create_backup: bool,
    
    /// Backup directory path
    pub backup_directory: PathBuf,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// HTTP timeout in seconds
    pub timeout_seconds: u64,
    
    /// User agent string
    pub user_agent: String,
    
    /// Maximum concurrent connections
    pub max_connections: usize,
    
    /// Enable HTTP/2
    pub http2_enabled: bool,
    
    /// Enable connection pooling
    pub connection_pooling: bool,
    
    /// Proxy URL if needed
    pub proxy_url: Option<String>,
    
    /// Enable certificate verification
    pub verify_ssl: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable content hash verification
    pub verify_content_hashes: bool,
    
    /// Hash algorithm: sha256, blake3
    pub hash_algorithm: String,
    
    /// Enable URL whitelist filtering
    pub url_whitelist_enabled: bool,
    
    /// Allowed URL patterns
    pub allowed_url_patterns: Vec<String>,
    
    /// Maximum download size in MB
    pub max_download_size_mb: u64,
    
    /// Enable sandbox mode (restricted operations)
    pub sandbox_mode: bool,
}

// ============================================================================
// CONFIGURATION LOADING AND MANAGEMENT
// ============================================================================

impl TTRPGConfig {
    /// Load configuration with full hierarchy support
    ///
    /// Configuration is loaded in this priority order:
    /// 1. Environment variables (TTRPG_* prefixed)
    /// 2. User config file (~/.config/ttrpg-converter/config.toml)
    /// 3. Project config file (./ttrpg-converter.toml)
    /// 4. Built-in defaults
    pub fn load() -> ConversionResult<Self> {
        let figment = Figment::new()
            // Start with built-in defaults
            .merge(Self::default_figment()?)
            // Add project-level config file
            .merge(Toml::file("ttrpg-converter.toml"))
            // Add user-level config file
            .merge(Self::user_config_file()?)
            // Override with environment variables
            .merge(Env::prefixed("TTRPG_").split("_"));

        let config: TTRPGConfig = figment.extract().map_err(|e| {
            ConversionError::Other(format!("Failed to load configuration: {}", e))
        })?;

        // Validate configuration
        config.validate()?;

        Ok(config)
    }
    
    /// Load configuration from a specific file
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> ConversionResult<Self> {
        let figment = Figment::new()
            .merge(Self::default_figment()?)
            .merge(Toml::file(path));

        let config: TTRPGConfig = figment.extract().map_err(|e| {
            ConversionError::Other(format!("Failed to load configuration from file: {}", e))
        })?;

        config.validate()?;
        Ok(config)
    }
    
    /// Create default configuration
    pub fn default() -> Self {
        Self {
            logging: LoggingConfig::default(),
            validation: ValidationConfig::default(),
            cache: CacheConfig::default(),
            processing: ProcessingConfig::default(),
            output: OutputConfig::default(),
            network: NetworkConfig::default(),
            security: SecurityConfig::default(),
        }
    }
    
    /// Create figment with default values
    fn default_figment() -> ConversionResult<Figment> {
        let default_config = Self::default();
        let toml_string = toml::to_string(&default_config).map_err(|e| {
            ConversionError::Other(format!("Failed to serialize default config: {}", e))
        })?;
        
        Ok(Figment::new().merge(Toml::string(&toml_string)))
    }
    
    /// Get user config file path and merge if it exists
    fn user_config_file() -> ConversionResult<Figment> {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        });
        
        config_dir.push("ttrpg-converter");
        config_dir.push("config.toml");
        
        Ok(Figment::new().merge(Toml::file(config_dir)))
    }
    
    /// Validate configuration values
    fn validate(&self) -> ConversionResult<()> {
        // Validate logging configuration
        match self.logging.level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {}
            _ => return Err(ConversionError::Other(
                "Invalid log level. Must be one of: trace, debug, info, warn, error".to_string()
            )),
        }
        
        match self.logging.format.as_str() {
            "pretty" | "json" | "compact" => {}
            _ => return Err(ConversionError::Other(
                "Invalid log format. Must be one of: pretty, json, compact".to_string()
            )),
        }
        
        // Validate output format
        match self.output.format.as_str() {
            "foundry" | "roll20" | "universal" => {}
            _ => return Err(ConversionError::Other(
                "Invalid output format. Must be one of: foundry, roll20, universal".to_string()
            )),
        }
        
        // Validate security settings
        match self.security.hash_algorithm.as_str() {
            "sha256" | "blake3" => {}
            _ => return Err(ConversionError::Other(
                "Invalid hash algorithm. Must be one of: sha256, blake3".to_string()
            )),
        }
        
        // Validate numeric ranges
        if self.processing.webp_quality == 0 || self.processing.webp_quality > 100 {
            return Err(ConversionError::Other(
                "WebP quality must be between 1 and 100".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Generate example configuration file
    pub fn generate_example_config() -> ConversionResult<String> {
        let config = Self::default();
        toml::to_string_pretty(&config).map_err(|e| {
            ConversionError::Other(format!("Failed to generate example config: {}", e))
        })
    }
    
    /// Save configuration to file
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> ConversionResult<()> {
        let toml_content = toml::to_string_pretty(self).map_err(|e| {
            ConversionError::Other(format!("Failed to serialize configuration: {}", e))
        })?;
        
        std::fs::write(path, toml_content).map_err(|e| {
            ConversionError::Other(format!("Failed to write configuration file: {}", e))
        })?;
        
        Ok(())
    }
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS
// ============================================================================

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            file_enabled: true,
            console_enabled: true,
            file_directory: PathBuf::from("logs"),
            file_name_pattern: "ttrpg-converter.%d.log".to_string(),
            max_file_size_mb: 10,
            max_archived_files: 7,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            max_file_size_mb: 100,
            timeout_seconds: 30,
            business_rules_enabled: true,
            integrity_checking_enabled: true,
            custom_rules_file: None,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size_mb: 512,
            ttl_seconds: 3600, // 1 hour
            directory: PathBuf::from("cache"),
            persistent_enabled: true,
            cleanup_interval_seconds: 300, // 5 minutes
        }
    }
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            parallel_downloads: 8,
            retry_attempts: 3,
            timeout_seconds: 60,
            optimize_assets: true,
            resize_images: true,
            max_image_width: 2048,
            max_image_height: 2048,
            convert_to_webp: true,
            webp_quality: 80,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: "foundry".to_string(),
            compress_assets: true,
            preserve_structure: true,
            directory: PathBuf::from("output"),
            incremental_enabled: true,
            create_backup: true,
            backup_directory: PathBuf::from("backups"),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            user_agent: "TTRPGConverter/1.0".to_string(),
            max_connections: 10,
            http2_enabled: true,
            connection_pooling: true,
            proxy_url: None,
            verify_ssl: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            verify_content_hashes: true,
            hash_algorithm: "sha256".to_string(),
            url_whitelist_enabled: false,
            allowed_url_patterns: vec![
                "https://app.roll20.net/*".to_string(),
                "https://marketplace-*.roll20.net/*".to_string(),
                "https://*.amazonaws.com/*".to_string(),
            ],
            max_download_size_mb: 100,
            sandbox_mode: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = TTRPGConfig::default();
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.output.format, "foundry");
        assert!(config.cache.enabled);
        assert_eq!(config.processing.parallel_downloads, 8);
    }

    #[test]
    fn test_config_validation() {
        let mut config = TTRPGConfig::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Invalid log level should fail
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_toml() {
        let toml_content = r#"
[logging]
level = "debug"
format = "json"

[cache]
enabled = false
max_size_mb = 256

[processing]
parallel_downloads = 4
"#;
        
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        
        let config = TTRPGConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.logging.level, "debug");
        assert_eq!(config.logging.format, "json");
        assert!(!config.cache.enabled);
        assert_eq!(config.cache.max_size_mb, 256);
        assert_eq!(config.processing.parallel_downloads, 4);
    }

    #[test]
    fn test_generate_example_config() {
        let example = TTRPGConfig::generate_example_config().unwrap();
        assert!(example.contains("[logging]"));
        assert!(example.contains("[validation]"));
        assert!(example.contains("[cache]"));
        assert!(example.contains("[processing]"));
        assert!(example.contains("[output]"));
    }

    #[test]
    fn test_save_and_load_config() {
        let mut config = TTRPGConfig::default();
        config.logging.level = "debug".to_string();
        config.processing.parallel_downloads = 16;
        
        let temp_file = NamedTempFile::new().unwrap();
        config.save_to_file(temp_file.path()).unwrap();
        
        let loaded_config = TTRPGConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(loaded_config.logging.level, "debug");
        assert_eq!(loaded_config.processing.parallel_downloads, 16);
    }
}
