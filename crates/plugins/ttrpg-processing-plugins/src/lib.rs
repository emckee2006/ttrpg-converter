//! Processing plugins for TTRPG Converter
//! 
//! This crate provides comprehensive processing plugins including:
//! - Asset retrieval, resolution, conversion, and scene processing
//! - Validation plugins with JSON schema and struct validation
//! - Plugin orchestration with dependency injection and pipeline management

// Core plugin modules
// TODO: Re-enable asset plugins after fixing validation and orchestration
// pub mod asset;
pub mod validation;
pub mod shared;
pub mod orchestration;

// Re-export main types
// TODO: Re-enable asset re-exports after fixing core issues
// pub use asset::*;
pub use validation::*;
pub use orchestration::*;
