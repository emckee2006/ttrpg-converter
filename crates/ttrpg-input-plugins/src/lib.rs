//! Input plugins for various TTRPG formats
//!
//! This crate provides input plugin implementations for parsing and converting
//! campaign data from various TTRPG virtual tabletop platforms and tools.

pub mod roll20;

// Re-export main plugin implementations
pub use roll20::Roll20InputPlugin;
