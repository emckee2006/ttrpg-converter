//! Input plugin trait for parsing campaign formats

use async_trait::async_trait;
use std::path::Path;
use ttrpg_core::prelude::*;
use ttrpg_types::Campaign;
use super::PluginInfo;

/// Asset information discovered during campaign parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInfo {
    pub path: String,
    pub url: Option<String>,
    pub asset_type: String,
    pub size: Option<u64>,
    pub hash: Option<String>,
}

/// Campaign metadata extracted during parsing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetadata {
    pub name: String,
    pub game_system: Option<String>,
    pub created_date: Option<String>,
    pub last_modified: Option<String>,
    pub player_count: Option<u32>,
    pub asset_count: Option<u32>,
}

/// Input plugin trait for parsing campaign formats
#[async_trait]
pub trait InputPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Check if this plugin can handle the given input
    fn can_handle(&self, source: &Path) -> bool;

    /// Parse campaign data from source
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<Campaign>;

    /// Discover assets referenced in the campaign
    async fn discover_assets(&self, campaign: &Campaign) -> ConversionResult<Vec<AssetInfo>>;

    /// Extract metadata for system detection
    fn extract_metadata(&self, source: &Path) -> ConversionResult<CampaignMetadata>;
}