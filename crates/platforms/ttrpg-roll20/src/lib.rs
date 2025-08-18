//! Roll20 schemas and mappings
//! 
//! This crate provides self-contained Roll20 support including:
//! - Generated schema types from Roll20 JSON schemas
//! - Bidirectional mappings between Roll20 types and internal TTRPG types
//! - Platform-specific utilities and helpers

#![cfg_attr(feature = "schemas", allow(dead_code))]

#[cfg(feature = "schemas")]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

#[cfg(feature = "mappings")]
pub mod mappings;

#[cfg(feature = "mappings")]
// Generated types are internal to this crate only
// Re-export mappings for external use
pub use mappings::*;
