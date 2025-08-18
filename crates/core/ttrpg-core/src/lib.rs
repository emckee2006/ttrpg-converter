//! TTRPGConverter Core Library
//!
//! This crate provides foundational error handling and basic types for the modular architecture.
//! Complex types and plugin systems are in separate crates to avoid circular dependencies.
//!
//! # Architecture
//! - `ttrpg-core`: Error types and basic format enums (this crate)
//! - `ttrpg-schema-types`: Generated external format types
//! - `ttrpg-plugin-framework`: Plugin traits and internal types
//! - `ttrpg-*-plugins`: Plugin implementations
//! - `ttrpg-orchestration`: Pipeline management
//!
//! # Usage
//!
//! ```rust
//! use ttrpg_core::prelude::*;
//! 
//! // Use error types and format enums
//! let result: ConversionResult<()> = Ok(());
//! let source = SourceFormat::Roll20;
//! ```

// Public modules
pub mod error;
pub mod types;
pub mod utils;

// Re-exports for convenience
pub use error::{AssetError, AssetResult, ConversionError, ConversionResult, ErrorExt};
pub use types::{SourceFormat, TargetFormat};

/// Common imports for this crate and dependent crates
pub mod prelude {
    // Error handling
    pub use crate::error::{AssetError, AssetResult, ConversionError, ConversionResult, ErrorExt};
    
    // Basic format types
    pub use crate::types::{SourceFormat, TargetFormat};

    // External dependencies commonly used
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;
    pub use std::path::PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_error_creation() {
        let error = ConversionError::UnsupportedInputFormat(PathBuf::from("test.unknown"));
        assert!(error.to_string().contains("Unsupported input format"));
        
        let error2 = ConversionError::UnsupportedOutputFormat("Unknown".to_string());
        assert!(error2.to_string().contains("Unsupported output format"));
        
        let error3 = ConversionError::FileNotFound {
            path: PathBuf::from("missing.json"),
            source: None,
            context: "Test context".to_string(),
        };
        assert!(error3.to_string().contains("File not found"));
    }

    #[test]
    fn test_source_format_display() {
        assert_eq!(SourceFormat::Roll20.to_string(), "Roll20");
        assert_eq!(SourceFormat::FoundryV9.to_string(), "Foundry VTT v9");
        assert_eq!(SourceFormat::FoundryV12.to_string(), "Foundry VTT v12");
        assert_eq!(SourceFormat::PathbuilderJson.to_string(), "Pathbuilder JSON");
    }

    #[test]
    fn test_target_format_display() {
        assert_eq!(TargetFormat::FoundryV12.to_string(), "Foundry VTT v12");
        assert_eq!(TargetFormat::Roll20.to_string(), "Roll20");
        assert_eq!(TargetFormat::Json.to_string(), "JSON");
    }

    #[test]
    fn test_conversion_result() {
        let success: ConversionResult<String> = Ok("Success".to_string());
        assert!(success.is_ok());
        assert_eq!(success.unwrap(), "Success");

        let failure: ConversionResult<String> = Err(Box::new(ConversionError::UnsupportedInputFormat(
            PathBuf::from("test.bad")
        )));
        assert!(failure.is_err());
    }

    #[test]
    fn test_asset_result() {
        let asset_success: AssetResult<String> = Ok("Asset loaded".to_string());
        assert!(asset_success.is_ok());

        let asset_failure: AssetResult<String> = Err(AssetError::DownloadError {
            url: "missing.png".to_string(),
            status_code: Some(404),
            reason: "Not found".to_string(),
        });
        assert!(asset_failure.is_err());
    }

    #[test]
    fn test_error_ext_trait() {
        let result: Result<(), std::io::Error> = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        let contextualized = result.with_context("Failed to load campaign");
        
        // ErrorExt trait should be available for adding context to Results
        assert!(contextualized.is_err());
    }
}
