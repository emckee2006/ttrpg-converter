//! Error handling for the TTRPGConverter
//!
//! This module provides comprehensive error types following Rust best practices
//! with thiserror for clean error handling across the entire application.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for the TTRPGConverter application
///
/// This enum covers all possible error conditions that can occur during
/// campaign conversion, following the error handling patterns outlined in
/// our architecture decisions.
///
/// # Examples
///
/// ```rust
/// use ttrpg_core::error::ConversionError;
///
/// let error = ConversionError::FileNotFound {
///     path: "/path/to/missing/file.zip".into(),
///     source: None,
///     context: "Roll20 campaign export".to_string(),
/// };
///
/// println!("Error: {}", error);
/// ```
#[derive(Error, Debug)]
pub enum ConversionError {
    /// File system related errors
    #[error("File not found: {path}")]
    FileNotFound {
        path: PathBuf,
        #[source]
        source: Option<std::io::Error>,
        context: String,
    },

    /// File access permission errors
    #[error("Permission denied accessing file: {path}")]
    PermissionDenied {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// JSON parsing and serialization errors
    #[error("JSON parsing failed in {context}")]
    JsonError {
        #[source]
        source: serde_json::Error,
        context: String,
        line: Option<usize>,
    },

    /// Campaign data validation errors
    #[error("Validation failed for {entity_type}: {message}")]
    ValidationError { entity_type: String, message: String, field: Option<String> },

    /// Plugin system errors
    #[error("Plugin error: {message}")]
    PluginError {
        message: String,
        plugin_name: Option<String>,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String, field: Option<String>, expected_type: Option<String> },

    #[error("Unsupported input format: {0}")]
    UnsupportedInputFormat(PathBuf),

    #[error("Unsupported output format: {0}")]
    UnsupportedOutputFormat(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Asset processing errors
    #[error("Asset processing failed: {asset_path}")]
    AssetError {
        asset_path: String,
        #[source]
        source: AssetError,
        context: String,
    },

    /// Network/download errors for external assets
    #[error("Network error downloading asset: {url}")]
    NetworkError {
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// Configuration errors
    #[error("Configuration error in {section}: {message}")]
    ConfigError { section: String, message: String, config_path: Option<PathBuf> },

    /// Format conversion errors
    #[error("Format conversion failed: {from_format} -> {to_format}")]
    ConversionFormatError { from_format: String, to_format: String, reason: String },

    /// ZIP archive handling errors
    #[error("ZIP archive error: {path}")]
    ZipError {
        path: PathBuf,
        #[source]
        source: zip::result::ZipError,
        operation: String,
    },

    /// Generic I/O errors with context
    #[error("I/O error during {operation}")]
    IoError {
        #[source]
        source: std::io::Error,
        operation: String,
        path: Option<PathBuf>,
    },
}

/// Asset-specific error types
///
/// These errors are specific to asset processing operations like
/// downloading, resizing, format conversion, etc.
#[derive(Error, Debug)]
pub enum AssetError {
    /// Image processing errors
    #[error("Image processing failed: {operation}")]
    ImageError {
        #[source]
        source: image::ImageError,
        operation: String,
        image_path: String,
    },

    /// Asset download errors
    #[error("Download failed for asset: {url}")]
    DownloadError { url: String, status_code: Option<u16>, reason: String },

    /// Asset validation errors
    #[error("Asset validation failed: {asset_path}")]
    ValidationError { asset_path: String, reason: String, expected_type: Option<String> },

    /// File size limit errors
    #[error("Asset too large: {size_mb}MB (limit: {limit_mb}MB)")]
    SizeLimitExceeded { asset_path: String, size_mb: f64, limit_mb: f64 },
}

/// Result type alias for conversion operations
///
/// This is used throughout the application for consistent error handling.
/// All major functions return this type for uniform error propagation.
pub type ConversionResult<T> = Result<T, Box<ConversionError>>;

/// Result type alias for asset operations
pub type AssetResult<T> = Result<T, AssetError>;

impl ConversionError {
    /// Create a file not found error with context
    pub fn file_not_found<P: Into<PathBuf>>(path: P, context: impl Into<String>) -> Self {
        Self::FileNotFound { path: path.into(), source: None, context: context.into() }
    }

    /// Create a validation error
    pub fn validation(entity_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ValidationError {
            entity_type: entity_type.into(),
            message: message.into(),
            field: None,
        }
    }

    /// Create a validation error with specific field
    pub fn validation_field(
        entity_type: impl Into<String>,
        message: impl Into<String>,
        field: impl Into<String>,
    ) -> Self {
        Self::ValidationError {
            entity_type: entity_type.into(),
            message: message.into(),
            field: Some(field.into()),
        }
    }

    /// Add context to an I/O error
    pub fn from_io(source: std::io::Error, operation: impl Into<String>) -> Self {
        Self::IoError { source, operation: operation.into(), path: None }
    }

    /// Add context and path to an I/O error
    pub fn from_io_with_path(
        source: std::io::Error,
        operation: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self::IoError { source, operation: operation.into(), path: Some(path.into()) }
    }

    /// Create a JSON parsing error with context
    pub fn from_json(
        source: serde_json::Error,
        context: impl Into<String>,
        line: Option<usize>,
    ) -> Self {
        Self::JsonError { source, context: context.into(), line }
    }
}

impl AssetError {
    /// Create a download error
    pub fn download_failed(url: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::DownloadError { url: url.into(), status_code: None, reason: reason.into() }
    }

    /// Create a download error with HTTP status code
    pub fn download_failed_with_status(
        url: impl Into<String>,
        status_code: u16,
        reason: impl Into<String>,
    ) -> Self {
        Self::DownloadError {
            url: url.into(),
            status_code: Some(status_code),
            reason: reason.into(),
        }
    }
}

/// Extension trait for adding context to any Result type
///
/// This trait provides convenient methods for adding contextual information
/// to both success and error cases.
#[allow(clippy::result_large_err)]
pub trait ErrorExt<T> {
    /// Add context to an error
    fn with_context(self, context: &str) -> ConversionResult<T>;

    /// Add context with file path
    fn with_file_context<P: Into<PathBuf>>(self, path: P) -> ConversionResult<T>;
}

impl<T> ErrorExt<T> for Result<T, std::io::Error> {
    fn with_context(self, context: &str) -> ConversionResult<T> {
        self.map_err(|e| Box::new(ConversionError::from_io(e, context)))
    }

    fn with_file_context<P: Into<PathBuf>>(self, path: P) -> ConversionResult<T> {
        let path = path.into();
        self.map_err(|e| Box::new(ConversionError::from_io_with_path(e, "file operation", path)))
    }
}

impl<T> ErrorExt<T> for Result<T, serde_json::Error> {
    fn with_context(self, context: &str) -> ConversionResult<T> {
        self.map_err(|e| {
            Box::new(ConversionError::JsonError {
                source: e,
                context: context.to_string(),
                line: None,
            })
        })
    }

    fn with_file_context<P: Into<PathBuf>>(self, path: P) -> ConversionResult<T> {
        self.with_context(&format!("parsing JSON file: {}", path.into().display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ConversionError::file_not_found("/test/path", "test context");
        assert!(matches!(error, ConversionError::FileNotFound { .. }));
    }

    #[test]
    fn test_error_display() {
        let error = ConversionError::validation("Character", "Missing required field");
        let error_string = error.to_string();
        assert!(error_string.contains("Validation failed"));
        assert!(error_string.contains("Character"));
    }

    #[test]
    fn test_error_extension() {
        let result: Result<(), std::io::Error> =
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));

        let converted = result.with_context("test operation");
        assert!(
            matches!(converted, Err(ref boxed) if matches!(**boxed, ConversionError::IoError { .. }))
        );
    }
}
