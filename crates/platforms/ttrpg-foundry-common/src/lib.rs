//! Foundry VTT common schemas and mappings
//! 
//! This crate provides shared Foundry VTT types including:
//! - Common schemas used across all Foundry systems
//! - Core Foundry VTT document types
//! - Bidirectional mappings to internal TTRPG types

#![cfg_attr(feature = "schemas", allow(dead_code))]

#[cfg(feature = "schemas")]
pub mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated/mod.rs"));
}

#[cfg(feature = "mappings")]
pub mod mappings;

// Only export mappings publicly to avoid ambiguous glob re-exports
#[cfg(feature = "mappings")]  
pub use mappings::*;
