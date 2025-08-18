//! Modular platform mappings for Pathbuilder PF2e
//! 
//! This module provides organized, maintainable mappings between 
//! Pathbuilder PF2e schema types and internal TTRPG types.

pub mod character;
pub mod item;
pub mod spell;

#[cfg(test)]
mod tests;
pub mod helpers;

// Re-export helper functionality only to avoid name conflicts with schemas
pub use helpers::*;
