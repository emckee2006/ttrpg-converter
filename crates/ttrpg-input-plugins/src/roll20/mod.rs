//! Roll20 Input Plugin
//!
//! Unified Roll20 input plugin that consolidates all Roll20-related functionality
//! including parsing, asset processing, validation, and data conversion.

use async_trait::async_trait;
use ttrpg_core::plugin_framework::InputPlugin;
use ttrpg_core::types::*;

pub mod parsing;
// Relocated modules during crate reorganization:
// - asset_processing → ttrpg-processing-plugins/asset/roll20/processing.rs
// - validation → ttrpg-processing-plugins/validation/roll20/validation.rs
// - asset_integration → ttrpg-processing-plugins/asset/roll20/integration.rs
// - validation_integration → ttrpg-processing-plugins/validation/roll20/integration.rs

// Re-export main structures
pub use parsing::{Roll20Campaign, Roll20Character, Roll20Page, Roll20Parser};

/// Roll20 Input Plugin
///
/// Consolidated plugin that handles all Roll20 campaign parsing and conversion
pub struct Roll20InputPlugin {
    parser: Roll20Parser,
}

impl Roll20InputPlugin {
    pub fn new() -> Self {
        Self { parser: Roll20Parser::new() }
    }
}

impl Default for Roll20InputPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InputPlugin for Roll20InputPlugin {
    async fn initialize(&mut self) -> ttrpg_core::error::ConversionResult<()> {
        // Plugin initialization logic
        Ok(())
    }

    async fn cleanup(&mut self) -> ttrpg_core::error::ConversionResult<()> {
        // Plugin cleanup logic
        Ok(())
    }

    async fn parse_campaign(
        &mut self,
        input_data: &[u8],
    ) -> ttrpg_core::error::ConversionResult<Campaign> {
        // Delegate to internal parser
        // This will be implemented after consolidating all Roll20 logic
        todo!("Implement after consolidating Roll20 parsing logic")
    }

    fn metadata(&self) -> ttrpg_core::plugin_framework::types::PluginMetadata {
        ttrpg_core::plugin_framework::types::PluginMetadata {
            name: "Roll20 Input Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Unified Roll20 campaign parsing and conversion plugin".to_string(),
            author: "TTRPG Converter Team".to_string(),
        }
    }
}
