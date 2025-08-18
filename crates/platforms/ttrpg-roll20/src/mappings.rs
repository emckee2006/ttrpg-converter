//! Roll20 to TTRPG Core Type Mappings
//! 
//! This module implements one-to-one mappings between Roll20 generated schema types
//! and internal TTRPG types, following the modular mapping approach.

// Re-export generated types that are used in mappings
// use crate::generated::{
//     character::{Roll20CharacterSheet, Roll20CharacterAttribute, Roll20CharacterAbility},
//     page::Page,
//     handout::Handout,
// };
// Core types are used by individual mapping modules as needed
// No shared imports required at this level

pub mod character;
pub mod scene;
pub mod item;
// Campaign mapping not needed - Roll20 exports contain collections of entities directly

#[cfg(test)]
mod tests;
