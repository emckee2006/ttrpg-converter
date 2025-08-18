//! Roll20-specific asset processing implementations
//!
//! This module contains Roll20-specific asset processing logic that was relocated
//! during the crate reorganization for proper architectural separation.

// Roll20 asset processing components
pub mod integration;
pub mod processing;

// Re-export main implementations
// TODO: Fix compilation errors before uncommenting
// pub use integration:://! Roll20 asset processing implementations

// Re-export Roll20 asset processors
// TODO: Fix compilation errors before uncommenting
// pub use integration::Roll20AssetIntegration;
// pub use processing::Roll20AssetProcessor;
