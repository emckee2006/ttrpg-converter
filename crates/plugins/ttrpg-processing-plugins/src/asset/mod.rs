//! Asset processing plugin implementations
//!
//! This module provides comprehensive asset processing capabilities including
//! downloading, caching, validation, and format-specific asset handling.

// Asset processing plugin modules
pub mod retrieval;
pub mod resolution;
pub mod conversion;
pub mod scene;
pub mod reference;

// Format-specific asset processors
pub mod roll20;

// Re-export asset plugins
pub use retrieval::AssetRetrievalPlugin;
pub use resolution::AssetResolutionPlugin;
pub use conversion::AssetConversionPlugin;
pub use scene::SceneProcessingPlugin;
pub use reference::ReferenceTrackingPlugin;

// Re-export Roll20-specific plugins
// TODO: Fix compilation errors before uncommenting
// pub use roll20::{Roll20AssetIntegration, Roll20AssetProcessor};
