//! Pathbuilder Pathfinder 2e schemas and mappings
//! 
//! This crate provides self-contained Pathbuilder PF2e support including:
//! - Generated schema types from Pathbuilder PF2e JSON schemas
//! - Bidirectional mappings between Pathbuilder PF2e types and internal TTRPG types
//! - Platform-specific utilities and helpers

#![cfg_attr(feature = "schemas", allow(dead_code))]

#[cfg(feature = "schemas")]
pub mod schemas {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

#[cfg(feature = "mappings")]
pub mod mappings;

#[cfg(feature = "schemas")]
#[allow(unused_imports, ambiguous_glob_reexports)]
pub use mappings::*;

#[cfg(feature = "mappings")]
#[allow(unused_imports)] // Will be used once mappings are implemented
pub use mappings::*;
