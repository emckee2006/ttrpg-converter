//! Roll20-specific asset processing implementations
//!
//! This module contains Roll20-specific asset processing logic that was relocated
//! during the crate reorganization for proper architectural separation.

// Roll20 asset processing components
pub mod integration;
pub mod processing;

// Re-export main implementations
pub use integration::Roll20AssetIntegration;
pub use processing::Roll20AssetProcessor;
