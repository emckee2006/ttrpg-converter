//! Input plugins for various TTRPG formats
//!
//! This crate provides input plugin implementations for parsing and converting
//! campaign data from various TTRPG virtual tabletop platforms and tools.

pub mod roll20;
pub mod foundry;
pub mod fantasy_grounds;

// Re-export main plugin implementations
pub use roll20::{Roll20InputPlugin, Roll20InputHandler};
pub use foundry::FoundryInputHandler;
pub use fantasy_grounds::FantasyGroundsInputHandler;
