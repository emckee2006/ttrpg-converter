//! Asset processing plugin implementations
//!
//! This module provides comprehensive asset processing capabilities including
//! downloading, caching, validation, and format-specific asset handling.

// Core asset processing components
pub mod core;
pub mod http_client;
pub mod memory_cache;

// Format-specific asset processors
pub mod roll20;

// Re-export main asset plugin implementation
pub use core::UnifiedAssetPlugin;

// Re-export Roll20-specific functionality
pub use roll20::{Roll20AssetIntegration, Roll20AssetProcessor};
