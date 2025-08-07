//! Service abstractions and dependency injection
//! 
//! This module defines abstract service interfaces that can be implemented by different
//! crates in the workspace. It supports the service-oriented architecture outlined
//! in our architectural decisions.

use crate::error::{ConversionResult, AssetResult};
use crate::types::{Campaign, AssetReference};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Logging service abstraction
/// 
/// Provides structured logging capabilities throughout the application.
/// Can be implemented by different logging backends (console, file, etc.)
pub trait LoggingService: Send + Sync {
    /// Log an error message
    fn error(&self, message: &str, context: Option<&str>);
    
    /// Log a warning message  
    fn warn(&self, message: &str, context: Option<&str>);
    
    /// Log an info message
    fn info(&self, message: &str, context: Option<&str>);
    
    /// Log a debug message
    fn debug(&self, message: &str, context: Option<&str>);
    
    /// Log with structured data
    fn log_with_data(&self, level: LogLevel, message: &str, data: &serde_json::Value);
    
    /// Set the minimum log level
    fn set_level(&mut self, level: LogLevel);
    
    /// Check if a log level is enabled
    fn is_enabled(&self, level: LogLevel) -> bool;
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

/// Validation service abstraction
/// 
/// Provides data validation and integrity checking capabilities.
/// Can validate campaigns, entities, file paths, and data formats.
#[allow(clippy::result_large_err)]
pub trait ValidationService: Send + Sync {
    /// Validate a complete campaign
    fn validate_campaign(&self, campaign: &Campaign) -> ConversionResult<ValidationResult>;
    
    /// Validate required fields for an entity
    fn validate_required_fields(
        &self, 
        entity_type: &str, 
        data: &serde_json::Value
    ) -> ConversionResult<ValidationResult>;
    
    /// Validate data types
    fn validate_data_types(
        &self,
        entity_type: &str,
        data: &serde_json::Value
    ) -> ConversionResult<ValidationResult>;
    
    /// Validate entity-specific business rules
    fn validate_entity_data(
        &self,
        entity_type: &str,
        data: &serde_json::Value
    ) -> ConversionResult<ValidationResult>;
    
    /// Validate file paths and asset references
    fn validate_file_path(&self, path: &str) -> ConversionResult<ValidationResult>;
    
    /// Validate JSON structure
    fn validate_json_data(&self, data: &serde_json::Value) -> ConversionResult<ValidationResult>;
    
    /// Get validation statistics
    fn get_validation_stats(&self) -> ValidationStats;
    
    /// Clear validation cache
    fn clear_cache(&self);
}

/// Asset processing service abstraction
/// 
/// Handles asset downloading, caching, and processing operations.
pub trait AssetService: Send + Sync {
    /// Download an asset from a URL
    fn download_asset(&self, url: &str, output_path: &Path) -> AssetResult<AssetInfo>;
    
    /// Process and optimize an asset
    fn process_asset(&self, asset_path: &Path) -> AssetResult<AssetInfo>;
    
    /// Cache an asset locally
    fn cache_asset(&self, asset_ref: &AssetReference) -> AssetResult<PathBuf>;
    
    /// Get cached asset path
    fn get_cached_asset(&self, url: &str) -> Option<PathBuf>;
    
    /// Validate asset integrity
    fn validate_asset(&self, asset_path: &Path) -> AssetResult<bool>;
    
    /// Get asset metadata
    fn get_asset_info(&self, asset_path: &Path) -> AssetResult<AssetInfo>;
    
    /// Clear asset cache
    fn clear_cache(&self);
    
    /// Get asset processing statistics
    fn get_stats(&self) -> AssetStats;
}

/// Service manager for coordinating all services
/// 
/// Provides centralized service management with dependency injection.
/// All services should be accessed through this manager.
#[allow(clippy::result_large_err)]
pub trait ServiceManager: Send + Sync {
    /// Get logging service
    fn logging(&self) -> Arc<dyn LoggingService>;
    
    /// Get validation service
    fn validation(&self) -> Arc<dyn ValidationService>;
    
    /// Get asset service  
    fn assets(&self) -> Arc<dyn AssetService>;
    
    /// Register a logging service implementation
    fn register_logging(&mut self, service: Arc<dyn LoggingService>);
    
    /// Register a validation service implementation
    fn register_validation(&mut self, service: Arc<dyn ValidationService>);
    
    /// Register an asset service implementation
    fn register_assets(&mut self, service: Arc<dyn AssetService>);
    
    /// Initialize all services with default implementations
    fn init_defaults(&mut self) -> ConversionResult<()>;
    
    /// Shutdown all services gracefully
    fn shutdown(&mut self) -> ConversionResult<()>;
}

/// Validation result information
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    
    /// Validation errors (critical issues)
    pub errors: Vec<ValidationIssue>,
    
    /// Validation warnings (non-critical issues)
    pub warnings: Vec<ValidationIssue>,
    
    /// Processing statistics
    pub stats: ValidationStats,
}

/// Individual validation issue
#[derive(Debug, Clone)]
pub struct ValidationIssue {
    /// Issue severity
    pub severity: IssueSeverity,
    
    /// Entity type being validated
    pub entity_type: String,
    
    /// Entity ID (if applicable)
    pub entity_id: Option<String>,
    
    /// Field name (if applicable)
    pub field: Option<String>,
    
    /// Issue description
    pub message: String,
    
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
}

/// Issue severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueSeverity {
    /// Critical error that prevents conversion
    Error,
    /// Warning that may cause issues
    Warning,
    /// Informational notice
    Info,
}

/// Validation processing statistics
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    /// Total entities validated
    pub entities_validated: usize,
    
    /// Entities with errors
    pub entities_with_errors: usize,
    
    /// Entities with warnings
    pub entities_with_warnings: usize,
    
    /// Total validation time (milliseconds)
    pub validation_time_ms: u64,
    
    /// Validation operations cached
    pub cache_hits: usize,
    
    /// Validation operations computed fresh
    pub cache_misses: usize,
}

/// Asset information
#[derive(Debug, Clone)]
pub struct AssetInfo {
    /// Asset file path
    pub path: PathBuf,
    
    /// File size in bytes
    pub size_bytes: u64,
    
    /// MIME type
    pub mime_type: String,
    
    /// Image dimensions (if applicable)
    pub dimensions: Option<(u32, u32)>,
    
    /// Asset hash for integrity checking
    pub hash: String,
    
    /// Last modified timestamp
    pub modified: std::time::SystemTime,
}

/// Asset processing statistics
#[derive(Debug, Clone, Default)]
pub struct AssetStats {
    /// Total assets processed
    pub assets_processed: usize,
    
    /// Assets successfully downloaded
    pub downloads_successful: usize,
    
    /// Failed downloads
    pub downloads_failed: usize,
    
    /// Assets served from cache
    pub cache_hits: usize,
    
    /// Total download size (bytes)
    pub total_download_bytes: u64,
    
    /// Total processing time (milliseconds)
    pub processing_time_ms: u64,
}

impl ValidationResult {
    /// Create a new successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Create a new failed validation result with errors
    pub fn with_errors(errors: Vec<ValidationIssue>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Add an error to the validation result
    pub fn add_error(&mut self, issue: ValidationIssue) {
        self.errors.push(issue);
        self.is_valid = false;
    }
    
    /// Add a warning to the validation result
    pub fn add_warning(&mut self, issue: ValidationIssue) {
        self.warnings.push(issue);
    }
    
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    
    /// Get total number of issues
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }
}

impl ValidationIssue {
    /// Create a new error issue
    pub fn error(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Error,
            entity_type: entity_type.into(),
            entity_id: None,
            field: None,
            message: message.into(),
            suggestion: None,
        }
    }
    
    /// Create a new warning issue
    pub fn warning(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Warning,
            entity_type: entity_type.into(),
            entity_id: None,
            field: None,
            message: message.into(),
            suggestion: None,
        }
    }
    
    /// Add entity ID context
    pub fn with_entity_id(mut self, id: impl Into<String>) -> Self {
        self.entity_id = Some(id.into());
        self
    }
    
    /// Add field context
    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
    
    /// Add a suggested fix
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Trace => write!(f, "TRACE"),
        }
    }
}

impl std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueSeverity::Error => write!(f, "ERROR"),
            IssueSeverity::Warning => write!(f, "WARNING"),
            IssueSeverity::Info => write!(f, "INFO"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_creation() {
        let success = ValidationResult::success();
        assert!(success.is_valid);
        assert!(success.errors.is_empty());
        assert!(success.warnings.is_empty());
        
        let error_issue = ValidationIssue::error("Actor", "Missing name");
        let failed = ValidationResult::with_errors(vec![error_issue]);
        assert!(!failed.is_valid);
        assert!(!failed.errors.is_empty());
    }

    #[test]
    fn test_validation_issue_builder() {
        let issue = ValidationIssue::error("Character", "Missing required field")
            .with_entity_id("char_123")
            .with_field("name")
            .with_suggestion("Add a character name");
            
        assert_eq!(issue.severity, IssueSeverity::Error);
        assert_eq!(issue.entity_type, "Character");
        assert_eq!(issue.entity_id, Some("char_123".to_string()));
        assert_eq!(issue.field, Some("name".to_string()));
        assert!(issue.suggestion.is_some());
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Trace);
    }
}
