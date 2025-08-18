//! Roll20 Input Plugin
//!
//! Unified Roll20 input plugin that consolidates all Roll20-related functionality
//! including parsing, asset processing, validation, and data conversion.

pub mod parsing;
pub mod handler;

// Relocated modules during crate reorganization:
// - asset_processing → ttrpg-processing-plugins/asset/roll20/processing.rs
// - validation → ttrpg-processing-plugins/validation/roll20/validation.rs
// - asset_integration → ttrpg-processing-plugins/asset/roll20/integration.rs
// - validation_integration → ttrpg-processing-plugins/validation/roll20/integration.rs

// Re-export main structures and handler
pub use parsing::{Roll20Campaign, Roll20Character, Roll20Page, Roll20Parser};
pub use handler::Roll20InputHandler;

/// Roll20 Input Plugin
///
/// Consolidated plugin that handles all Roll20 campaign parsing and conversion
pub struct Roll20InputPlugin {
    #[allow(dead_code)]
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

use async_trait::async_trait;
use ttrpg_traits::InputPlugin;

#[async_trait]
impl InputPlugin for Roll20InputPlugin {
    fn plugin_info(&self) -> ttrpg_traits::PluginInfo {
        ttrpg_traits::PluginInfo {
            name: "Roll20 Input Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Unified Roll20 campaign parsing and conversion plugin".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec!["roll20".to_string(), "json".to_string()],
            dependencies: vec![],
        }
    }

    fn can_handle(&self, source: &std::path::Path) -> bool {
        // Check if it's a Roll20 file (*.json or campaign.json)
        source.extension().is_some_and(|ext| ext == "json")
            || source
                .file_name()
                .is_some_and(|name| name == "campaign.json")
    }

    async fn parse_campaign(
        &self,
        _source: &std::path::Path,
    ) -> ttrpg_core::error::ConversionResult<ttrpg_types::Campaign>
    {
        // Delegate to internal parser
        // This will be implemented after consolidating all Roll20 logic
        todo!("Implement after consolidating Roll20 parsing logic")
    }

    async fn discover_assets(
        &self,
        _campaign: &ttrpg_types::Campaign,
    ) -> ttrpg_core::error::ConversionResult<Vec<ttrpg_traits::AssetInfo>>
    {
        // Asset discovery implementation
        todo!("Implement asset discovery")
    }

    fn extract_metadata(
        &self,
        _source: &std::path::Path,
    ) -> ttrpg_core::error::ConversionResult<ttrpg_types::CampaignMetadata>
    {
        // Metadata extraction implementation
        todo!("Implement metadata extraction")
    }
}
