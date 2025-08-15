//! Processing plugins for asset management and validation
//!
//! This crate provides plugins for processing campaign data, including asset
//! management, validation, and data transformation operations.

pub mod asset;
pub mod validation;

// Re-export main plugin implementations
pub use asset::UnifiedAssetPlugin;
pub use validation::UnifiedValidationPlugin;
