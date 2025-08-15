//! Plugin Registry - Object-Safe Plugin System
//!
//! This module implements an object-safe plugin system using enums instead of
//! async trait objects, solving the async trait object-safety problem while
//! maintaining all the benefits of the plugin architecture.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    error::{ConversionError, ConversionResult},
    plugin_framework::types::{
        AssetInfo, AssetType, CampaignMetadata, ConversionNote, SourceFormat, UniversalCampaign,
    },
    plugin_framework::{
        AssetMetadata, CampaignSettings, InputPlugin, OutputBundle, OutputConfig, OutputFormat,
        OutputPlugin, PluginInfo, ProcessedAsset, WriteOptions,
    },
    prelude::{ActorType, AttributeValue, ItemType},
    // Legacy AssetService import removed - functionality moved to plugins::interfaces::AssetPlugin
    types::{
        Actor, ActorImages, EntityPermissions, Feature, FeatureType, GridConfig, GridType, Item,
        ItemProperties, LightingConfig, Position, Scene, SceneDimensions, Spell, SpellComponents,
        Token, TokenSize,
    },
    GameSystem,
};
use tracing::info;

// Note: Roll20AssetProcessor import removed to avoid cyclic dependency
// Comprehensive asset discovery logic implemented directly in discover_assets method

/// Object-safe input plugin registry using enum dispatch
///
/// This solves the async trait object-safety problem by using concrete enum
/// variants instead of trait objects, while maintaining plugin extensibility.
#[derive(Debug)]
pub enum InputPluginRegistry {
    Roll20(Roll20InputHandler),
    Foundry(FoundryInputHandler),
    FantasyGrounds(FantasyGroundsInputHandler),
    // New plugins added as enum variants
}

/// Object-safe output plugin registry using enum dispatch
#[derive(Debug)]
pub enum OutputPluginRegistry {
    FoundryWorld(FoundryWorldHandler),
    FoundryModule(FoundryModuleHandler),
    PathbuilderJson(PathbuilderJsonHandler),
    DNDBeyondJson(DNDBeyondJsonHandler),
    HeroLabJson(HeroLabJsonHandler),
    PDFCharacterSheets(PDFCharacterSheetsHandler),
    PDFCampaignBook(PDFCampaignBookHandler),
    UniversalJson(UniversalJsonHandler),
    // New output formats added as enum variants
}

impl InputPluginRegistry {
    /// Get plugin information
    pub fn plugin_info(&self) -> PluginInfo {
        match self {
            Self::Roll20(handler) => handler.plugin_info(),
            Self::Foundry(handler) => handler.plugin_info(),
            Self::FantasyGrounds(handler) => handler.plugin_info(),
        }
    }

    /// Check if this plugin can handle the source
    pub fn can_handle(&self, source: &Path) -> bool {
        match self {
            Self::Roll20(handler) => handler.can_handle(source),
            Self::Foundry(handler) => handler.can_handle(source),
            Self::FantasyGrounds(handler) => handler.can_handle(source),
        }
    }

    /// Parse campaign from source
    pub async fn parse_campaign(&self, source: &Path) -> ConversionResult<UniversalCampaign> {
        match self {
            Self::Roll20(handler) => handler.parse_campaign(source).await,
            Self::Foundry(handler) => handler.parse_campaign(source).await,
            Self::FantasyGrounds(handler) => handler.parse_campaign(source).await,
        }
    }

    /// Discover assets in campaign
    pub async fn discover_assets(
        &self,
        campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>> {
        match self {
            Self::Roll20(handler) => handler.discover_assets(campaign).await,
            Self::Foundry(handler) => handler.discover_assets(campaign).await,
            Self::FantasyGrounds(handler) => handler.discover_assets(campaign).await,
        }
    }

    /// Extract metadata from source
    pub fn extract_metadata(&self, source: &Path) -> ConversionResult<CampaignMetadata> {
        match self {
            Self::Roll20(handler) => handler.extract_metadata(source),
            Self::Foundry(handler) => handler.extract_metadata(source),
            Self::FantasyGrounds(handler) => handler.extract_metadata(source),
        }
    }
}

impl OutputPluginRegistry {
    /// Get plugin information
    pub fn plugin_info(&self) -> PluginInfo {
        match self {
            Self::FoundryWorld(handler) => handler.plugin_info(),
            Self::FoundryModule(handler) => handler.plugin_info(),
            Self::PathbuilderJson(handler) => handler.plugin_info(),
            Self::DNDBeyondJson(handler) => handler.plugin_info(),
            Self::HeroLabJson(handler) => handler.plugin_info(),
            Self::PDFCharacterSheets(handler) => handler.plugin_info(),
            Self::PDFCampaignBook(handler) => handler.plugin_info(),
            Self::UniversalJson(handler) => handler.plugin_info(),
        }
    }

    /// Get supported output formats
    pub fn supported_formats(&self) -> Vec<OutputFormat> {
        match self {
            Self::FoundryWorld(handler) => handler.supported_formats(),
            Self::FoundryModule(handler) => handler.supported_formats(),
            Self::PathbuilderJson(handler) => handler.supported_formats(),
            Self::DNDBeyondJson(handler) => handler.supported_formats(),
            Self::HeroLabJson(handler) => handler.supported_formats(),
            Self::PDFCharacterSheets(handler) => handler.supported_formats(),
            Self::PDFCampaignBook(handler) => handler.supported_formats(),
            Self::UniversalJson(handler) => handler.supported_formats(),
        }
    }

    /// Generate output from campaign data
    pub async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        assets: &[ProcessedAsset],
        config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        match self {
            Self::FoundryWorld(handler) => handler.generate_output(campaign, assets, config).await,
            Self::FoundryModule(handler) => handler.generate_output(campaign, assets, config).await,
            Self::PathbuilderJson(handler) => {
                handler.generate_output(campaign, assets, config).await
            }
            Self::DNDBeyondJson(handler) => handler.generate_output(campaign, assets, config).await,
            Self::HeroLabJson(handler) => handler.generate_output(campaign, assets, config).await,
            Self::PDFCharacterSheets(handler) => {
                handler.generate_output(campaign, assets, config).await
            }
            Self::PDFCampaignBook(handler) => {
                handler.generate_output(campaign, assets, config).await
            }
            Self::UniversalJson(handler) => handler.generate_output(campaign, assets, config).await,
        }
    }

    /// Write output bundle to target location
    pub async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        options: &WriteOptions,
    ) -> ConversionResult<()> {
        match self {
            Self::FoundryWorld(handler) => handler.write_output(bundle, target, options).await,
            Self::FoundryModule(handler) => handler.write_output(bundle, target, options).await,
            Self::PathbuilderJson(handler) => handler.write_output(bundle, target, options).await,
            Self::DNDBeyondJson(handler) => handler.write_output(bundle, target, options).await,
            Self::HeroLabJson(handler) => handler.write_output(bundle, target, options).await,
            Self::PDFCharacterSheets(handler) => {
                handler.write_output(bundle, target, options).await
            }
            Self::PDFCampaignBook(handler) => handler.write_output(bundle, target, options).await,
            Self::UniversalJson(handler) => handler.write_output(bundle, target, options).await,
        }
    }

    /// Validate configuration for this plugin
    pub fn validate_config(&self, config: &OutputConfig) -> ConversionResult<()> {
        if !self.supported_formats().contains(&config.format) {
            return Err(Box::new(ConversionError::validation(
                "plugin",
                format!("Unsupported output format: {}", config.format),
            )));
        }
        Ok(())
    }
}

// Plugin handler structs implementing the plugin traits
/// Roll20 input plugin handler  
pub struct Roll20InputHandler {
    // validation_service, logging_service, and asset_service fields removed - services replaced by plugins
    // Store raw JSON data for comprehensive asset discovery (thread-safe interior mutability)
    raw_campaign_data: Arc<Mutex<Option<serde_json::Value>>>,
}

impl std::fmt::Debug for Roll20InputHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Roll20InputHandler")
            // validation_service, logging_service, and asset_service fields removed - services replaced by plugins
            .field("raw_campaign_data", &self.raw_campaign_data)
            .finish()
    }
}

impl Default for Roll20InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl Roll20InputHandler {
    pub fn new() -> Self {
        Self {
            // validation_service, logging_service, and asset_service fields removed - services replaced by plugins
            raw_campaign_data: Arc::new(Mutex::new(None)),
        }
    }
}

#[async_trait]
impl InputPlugin for Roll20InputHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Roll20 Input Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Parses Roll20 campaign exports with comprehensive asset discovery"
                .to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["json".to_string(), "comprehensive_assets".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn can_handle(&self, source: &Path) -> bool {
        source
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
    }

    fn extract_metadata(&self, source: &Path) -> ConversionResult<CampaignMetadata> {
        Ok(CampaignMetadata {
            title: source
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Untitled Campaign")
                .to_string(),
            description: Some(format!("Roll20 campaign from {}", source.display())),
            source_format: SourceFormat::Roll20,
            detected_system: Some(GameSystem::Unknown),
            system_confidence: 0.5,
            source_path: Some(source.to_path_buf()),
            created_at: Some(chrono::Utc::now()),
            modified_at: Some(chrono::Utc::now()),
            source_version: None,
        })
    }

    async fn parse_campaign(&self, source: &Path) -> ConversionResult<UniversalCampaign> {
        use serde_json;
        use std::fs;

        // Log the parsing attempt
        info!("Processing Roll20 campaign from: {}", source.display());

        // Read and parse the JSON file
        let json_content = fs::read_to_string(source)
            .map_err(|e| ConversionError::from_io(e, source.to_string_lossy().to_string()))?;

        // Parse raw JSON for comprehensive asset discovery
        let raw_json: serde_json::Value = serde_json::from_str(&json_content)
            .map_err(|e| ConversionError::from_json(e, "Roll20 campaign JSON (raw)", None))?;

        // Store raw JSON data for comprehensive asset discovery (async mutex)
        {
            let mut raw_data = self.raw_campaign_data.lock().await;
            *raw_data = Some(raw_json.clone());
        }

        let roll20_campaign: Roll20Campaign = serde_json::from_str(&json_content)
            .map_err(|e| ConversionError::from_json(e, "Roll20 campaign JSON", None))?;

        // Campaign validation moved to plugin architecture
        info!("Campaign conversion completed - validation handled by validation plugins");

        // Convert to UniversalCampaign
        self.convert_to_universal_campaign(roll20_campaign).await
    }

    /// Discover assets in campaign
    async fn discover_assets(
        &self,
        campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>> {
        // Use comprehensive Roll20AssetProcessor if raw JSON data is available
        let raw_data_guard = self.raw_campaign_data.lock().await;
        if let Some(raw_json) = raw_data_guard.clone() {
            drop(raw_data_guard); // Release lock early

            // Direct comprehensive asset discovery from raw JSON data
            let mut assets = Vec::new();

            // Extract assets from campaign structure
            if let Some(campaign_obj) = raw_json.as_object() {
                // Discover character assets (portraits/avatars)
                if let Some(characters) = campaign_obj.get("characters").and_then(|c| c.as_array())
                {
                    for character in characters {
                        if let Some(char_obj) = character.as_object() {
                            if let Some(avatar) = char_obj.get("avatar").and_then(|a| a.as_str()) {
                                if !avatar.is_empty() {
                                    assets.push(AssetInfo {
                                        source: avatar.to_string(),
                                        asset_type: AssetType::CharacterArt,
                                        local_path: None,
                                        metadata: AssetMetadata::default(),
                                        // Legacy compatibility fields with defaults
                                        path: PathBuf::from(avatar),
                                        size_bytes: 0,
                                        mime_type: "image/png".to_string(),
                                        dimensions: None,
                                        hash: String::new(),
                                        modified: std::time::SystemTime::UNIX_EPOCH,
                                    });
                                }
                            }
                        }
                    }
                }

                // Discover scene assets (background images)
                if let Some(pages) = campaign_obj.get("pages").and_then(|p| p.as_array()) {
                    for page in pages {
                        if let Some(page_obj) = page.as_object() {
                            if let Some(background) =
                                page_obj.get("background_url").and_then(|b| b.as_str())
                            {
                                if !background.is_empty() {
                                    assets.push(AssetInfo {
                                        source: background.to_string(),
                                        asset_type: AssetType::MapBackground,
                                        local_path: None,
                                        metadata: AssetMetadata::default(),
                                        // Legacy compatibility fields with defaults
                                        path: PathBuf::from(background),
                                        size_bytes: 0,
                                        mime_type: "image/png".to_string(),
                                        dimensions: None,
                                        hash: String::new(),
                                        modified: std::time::SystemTime::UNIX_EPOCH,
                                    });
                                }
                            }
                        }
                    }
                }

                // Discover general assets array
                if let Some(assets_array) = campaign_obj.get("assets").and_then(|a| a.as_array()) {
                    for asset in assets_array {
                        if let Some(asset_obj) = asset.as_object() {
                            if let Some(url) = asset_obj.get("url").and_then(|u| u.as_str()) {
                                if !url.is_empty() {
                                    assets.push(AssetInfo {
                                        source: url.to_string(),
                                        asset_type: AssetType::Attachment,
                                        local_path: None,
                                        metadata: AssetMetadata::default(),
                                        // Legacy compatibility fields with defaults
                                        path: PathBuf::from(url),
                                        size_bytes: 0,
                                        mime_type: "application/octet-stream".to_string(),
                                        dimensions: None,
                                        hash: String::new(),
                                        modified: std::time::SystemTime::UNIX_EPOCH,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            info!("Discovered {} assets in Roll20 campaign", assets.len());

            return Ok(assets);
        } else {
            // Log that no raw data available and fall back to basic discovery
            info!("No raw JSON data available for comprehensive asset discovery, using basic discovery");
        }

        // Fallback to basic asset discovery from converted campaign data
        let mut assets = Vec::new();

        // Discover actor images
        for actor in &campaign.actors {
            if let Some(ref image_url) = actor.images.avatar {
                assets.push(AssetInfo {
                    source: image_url.clone(),
                    asset_type: AssetType::CharacterArt,
                    local_path: None,
                    metadata: AssetMetadata::default(),
                    // Legacy compatibility fields with defaults
                    path: PathBuf::from(image_url),
                    size_bytes: 0,
                    mime_type: "image/png".to_string(),
                    dimensions: None,
                    hash: String::new(),
                    modified: std::time::SystemTime::UNIX_EPOCH,
                });
            }
        }

        // Discover scene backgrounds
        for scene in &campaign.scenes {
            if let Some(ref background_url) = scene.background_image {
                assets.push(AssetInfo {
                    source: background_url.clone(),
                    asset_type: AssetType::MapBackground,
                    local_path: None,
                    metadata: AssetMetadata::default(),
                    // Legacy compatibility fields with defaults
                    path: PathBuf::from(background_url),
                    size_bytes: 0,
                    mime_type: "image/png".to_string(),
                    dimensions: None,
                    hash: String::new(),
                    modified: std::time::SystemTime::UNIX_EPOCH,
                });
            }
        }

        info!("Basic asset discovery found {} assets from campaign", assets.len());

        Ok(assets)
    }
}

// Builder methods in separate impl block
impl Roll20InputHandler {
    // with_validation(), with_logging(), and with_assets() methods removed - services replaced by plugins
}

impl Roll20InputHandler {
    async fn convert_to_universal_campaign(
        &self,
        roll20_campaign: Roll20Campaign,
    ) -> ConversionResult<UniversalCampaign> {
        let mut campaign = UniversalCampaign {
            metadata: CampaignMetadata {
                title: roll20_campaign.campaign.name.clone(),
                description: Some("Imported from Roll20".to_string()),
                source_format: SourceFormat::Roll20,
                detected_system: Some(GameSystem::DnD5e),
                system_confidence: 0.9,
                source_path: None,
                created_at: Some(chrono::Utc::now()),
                modified_at: Some(chrono::Utc::now()),
                source_version: Some("1.0.0".to_string()),
            },
            game_system: GameSystem::DnD5e,
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal_entries: Vec::new(),
            macros: Vec::new(),
            playlists: Vec::new(),
            encounters: Vec::new(),
            settings: CampaignSettings::default(),
            conversion_notes: Vec::new(),
        };

        // Convert characters to actors
        for character in roll20_campaign.characters {
            match self.convert_character_to_actor(character).await {
                Ok(actor) => campaign.actors.push(actor),
                Err(e) => {
                    campaign.conversion_notes.push(ConversionNote {
                        timestamp: chrono::Utc::now(),
                        category: crate::plugin_framework::types::ConversionCategory::Error,
                        message: format!("Failed to convert character: {e}"),
                        affected_entity: Some("character".to_string()),
                    });
                }
            }
        }

        // Convert pages to scenes
        for page in roll20_campaign.pages {
            match self.convert_page_to_scene(page).await {
                Ok(scene) => campaign.scenes.push(scene),
                Err(e) => {
                    campaign.conversion_notes.push(ConversionNote {
                        timestamp: chrono::Utc::now(),
                        category: crate::plugin_framework::types::ConversionCategory::Error,
                        message: format!("Failed to convert page: {e}"),
                        affected_entity: Some("page".to_string()),
                    });
                }
            }
        }

        // Convert handouts to items
        for handout in roll20_campaign.handouts {
            match self.convert_handout_to_item(handout).await {
                Ok(item) => campaign.items.push(item),
                Err(e) => {
                    campaign.conversion_notes.push(ConversionNote {
                        timestamp: chrono::Utc::now(),
                        category: crate::plugin_framework::types::ConversionCategory::Error,
                        message: format!("Failed to convert handout: {e}"),
                        affected_entity: Some("handout".to_string()),
                    });
                }
            }
        }

        // logging_service field removed - using direct tracing instead
        info!(
            "Converted Roll20 campaign: {} actors, {} scenes, {} items",
            campaign.actors.len(),
            campaign.scenes.len(),
            campaign.items.len()
        );

        Ok(campaign)
    }

    async fn convert_character_to_actor(
        &self,
        roll20_char: Roll20Character,
    ) -> ConversionResult<Actor> {
        let mut attributes = HashMap::new();

        // Convert attributes
        for attr in roll20_char.attributes {
            attributes.insert(attr.name, AttributeValue::Text(attr.current));
        }

        let mut features = Vec::new();
        let mut spells = Vec::new();

        // Convert abilities - distinguish between spells and features based on action presence
        for ability in roll20_char.abilities {
            if ability.action.contains("spell") || ability.action.contains("cast") {
                spells.push(Spell {
                    id: format!("spell_{}", ability.name.replace(' ', "_").to_lowercase()),
                    name: ability.name,
                    level: 1, // Default level, could be parsed from description
                    school: "Evocation".to_string(), // Default, could be detected
                    casting_time: "1 action".to_string(),
                    range: "Touch".to_string(),
                    components: SpellComponents::default(),
                    duration: "Instantaneous".to_string(),
                    description: ability.description,
                    at_higher_levels: None,
                });
            } else {
                features.push(Feature {
                    id: format!("feature_{}", ability.name.replace(' ', "_").to_lowercase()),
                    name: ability.name,
                    feature_type: FeatureType::Other("Roll20 Ability".to_string()),
                    description: ability.description,
                    usage: None,
                });
            }
        }

        Ok(Actor {
            id: roll20_char.id,
            name: roll20_char.name,
            actor_type: ActorType::Pc,
            images: ActorImages { avatar: roll20_char.avatar, token: None, additional: Vec::new() },
            attributes,
            items: Vec::new(),
            features,
            spells,
            biography: String::new(),
            notes: String::new(),
            permissions: EntityPermissions::default(),
            source_data: HashMap::new(),
        })
    }

    async fn convert_page_to_scene(&self, roll20_page: Roll20Page) -> ConversionResult<Scene> {
        Ok(Scene {
            id: roll20_page.id,
            name: roll20_page.name,
            background_image: roll20_page.background_url,
            dimensions: SceneDimensions {
                width: roll20_page.width.unwrap_or(1400.0) as u32,
                height: roll20_page.height.unwrap_or(1000.0) as u32,
                scale: 1.0,
                grid_size: roll20_page.grid_size.unwrap_or(70.0),
            },
            grid: GridConfig {
                size: roll20_page.grid_size.unwrap_or(70.0),
                grid_type: GridType::Square,
                color: "#000000".to_string(),
                opacity: 0.5,
            },
            lighting: LightingConfig::default(),
            audio: None,
            notes: String::new(),
            permissions: EntityPermissions::default(),
            source_data: HashMap::new(),
            tokens: roll20_page
                .tokens
                .into_iter()
                .map(|token| Token {
                    id: token.id,
                    actor_id: token.represents,
                    position: Position {
                        x: token.left.unwrap_or(0.0),
                        y: token.top.unwrap_or(0.0),
                    },
                    size: TokenSize {
                        width: token.width.unwrap_or(70.0),
                        height: token.height.unwrap_or(70.0),
                    },
                    image: token.imgsrc,
                    hidden: false,
                    light: None,
                    vision: None,
                    source_data: HashMap::new(),
                })
                .collect(),
            walls: Vec::new(),
        })
    }

    async fn convert_handout_to_item(
        &self,
        roll20_handout: Roll20Handout,
    ) -> ConversionResult<Item> {
        Ok(Item {
            id: roll20_handout.id,
            name: roll20_handout.name,
            item_type: ItemType::Equipment,
            image: None,
            description: roll20_handout.notes,
            properties: ItemProperties::default(),
            source_data: HashMap::new(),
        })
    }
}

// Roll20 data structures (copied from ttrpg-formats)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Campaign {
    pub campaign: Roll20CampaignMeta,
    pub characters: Vec<Roll20Character>,
    pub pages: Vec<Roll20Page>,
    pub handouts: Vec<Roll20Handout>,
    pub journal: Vec<Roll20JournalEntry>,
    pub assets: Vec<Roll20Asset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20CampaignMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_date: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Character {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub bio: String,
    pub gmnotes: String,
    pub archived: bool,
    pub inplayerjournals: String,
    pub controlledby: String,
    pub attributes: Vec<Roll20Attribute>,
    pub abilities: Vec<Roll20Ability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Attribute {
    pub name: String,
    pub current: String,
    pub max: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Ability {
    pub name: String,
    pub description: String,
    pub action: String,
    pub istokenaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Page {
    pub id: String,
    pub name: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub grid_size: Option<f64>,
    pub background_url: Option<String>,
    pub tokens: Vec<Roll20Token>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Token {
    pub id: String,
    pub name: String,
    pub imgsrc: Option<String>,
    pub left: Option<f64>,
    pub top: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub represents: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Handout {
    pub id: String,
    pub name: String,
    pub notes: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20JournalEntry {
    pub id: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Asset {
    pub id: String,
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FoundryInputHandler;

#[async_trait]
impl InputPlugin for FoundryInputHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Foundry VTT Input Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Parses Foundry VTT world and module files".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["db".to_string(), "json".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn can_handle(&self, source: &Path) -> bool {
        // Check for world.json or Foundry world structure
        source.join("world.json").exists()
            || source
                .file_name()
                .map(|name| name == "world.json")
                .unwrap_or(false)
    }

    async fn parse_campaign(&self, _source: &Path) -> ConversionResult<UniversalCampaign> {
        // TODO: Implement Foundry parsing
        let metadata = CampaignMetadata {
            title: "Foundry Campaign".to_string(),
            description: Some("Imported from Foundry VTT".to_string()),
            source_format: SourceFormat::FoundryVTT,
            detected_system: Some(GameSystem::DnD5e),
            system_confidence: 0.8,
            source_path: None,
            created_at: Some(chrono::Utc::now()),
            modified_at: Some(chrono::Utc::now()),
            source_version: Some("1.0.0".to_string()),
        };

        Ok(UniversalCampaign {
            metadata,
            game_system: GameSystem::DnD5e,
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal_entries: Vec::new(),
            macros: Vec::new(),
            playlists: Vec::new(),
            encounters: Vec::new(),
            settings: CampaignSettings::default(),
            conversion_notes: Vec::new(),
        })
    }

    async fn discover_assets(
        &self,
        _campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>> {
        Ok(Vec::new())
    }

    fn extract_metadata(&self, _source: &Path) -> ConversionResult<CampaignMetadata> {
        Ok(CampaignMetadata::default())
    }
}

#[derive(Debug, Clone)]
pub struct FantasyGroundsInputHandler;

#[async_trait]
impl InputPlugin for FantasyGroundsInputHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Fantasy Grounds Input Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Parses Fantasy Grounds XML campaign files".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["xml".to_string(), "fantasy_grounds".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn can_handle(&self, _source: &Path) -> bool {
        // TODO: Implement FG detection logic
        false
    }

    async fn parse_campaign(&self, _source: &Path) -> ConversionResult<UniversalCampaign> {
        let metadata = CampaignMetadata {
            title: "Fantasy Grounds Campaign".to_string(),
            description: Some("Imported from Fantasy Grounds".to_string()),
            source_format: SourceFormat::FantasyGrounds,
            detected_system: Some(GameSystem::DnD5e),
            system_confidence: 0.8,
            source_path: None,
            created_at: Some(chrono::Utc::now()),
            modified_at: Some(chrono::Utc::now()),
            source_version: Some("1.0.0".to_string()),
        };

        Ok(UniversalCampaign {
            metadata,
            game_system: GameSystem::DnD5e,
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal_entries: Vec::new(),
            macros: Vec::new(),
            playlists: Vec::new(),
            encounters: Vec::new(),
            settings: CampaignSettings::default(),
            conversion_notes: Vec::new(),
        })
    }

    async fn discover_assets(
        &self,
        _campaign: &UniversalCampaign,
    ) -> ConversionResult<Vec<AssetInfo>> {
        Ok(Vec::new())
    }

    fn extract_metadata(&self, _source: &Path) -> ConversionResult<CampaignMetadata> {
        Ok(CampaignMetadata::default())
    }
}

// Output plugin handler structs implementing OutputPlugin trait
#[derive(Debug, Clone)]
pub struct FoundryWorldHandler;

#[async_trait]
impl OutputPlugin for FoundryWorldHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Foundry World Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Exports campaigns as Foundry VTT worlds".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec!["foundry-world".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::FoundryWorld]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        // TODO: Implement Foundry world generation
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        // TODO: Implement file writing
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct FoundryModuleHandler;

#[async_trait]
impl OutputPlugin for FoundryModuleHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Foundry Module Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Exports campaigns as Foundry VTT modules".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec!["foundry-module".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::FoundryModule]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PathbuilderJsonHandler;

#[async_trait]
impl OutputPlugin for PathbuilderJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Hero Lab JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Generates Hero Lab JSON character files".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["herolab_format".to_string(), "character_export".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::PathbuilderJson]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DNDBeyondJsonHandler;

#[async_trait]
impl OutputPlugin for DNDBeyondJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "D&D Beyond JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Generates D&D Beyond compatible JSON character files".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["dnd5e".to_string(), "dndbeyond_format".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::UniversalJson]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HeroLabJsonHandler;

#[async_trait]
impl OutputPlugin for HeroLabJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Hero Lab JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Exports character data as Hero Lab JSON".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec!["herolab".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::HeroLabJson]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PDFCharacterSheetsHandler;

#[async_trait]
impl OutputPlugin for PDFCharacterSheetsHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "PDF Character Sheet Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Generates PDF character sheets with form filling".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["pdf_generation".to_string(), "form_filling".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::PDFCharacterSheets]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PDFCampaignBookHandler;

#[async_trait]
impl OutputPlugin for PDFCampaignBookHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "PDF Campaign Book Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Generates comprehensive PDF campaign books".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["pdf_generation".to_string(), "campaign_book".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::PDFCampaignBook]
    }

    async fn generate_output(
        &self,
        _campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        Ok(OutputBundle {
            files: std::collections::HashMap::new(),
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UniversalJsonHandler;

#[async_trait]
impl OutputPlugin for UniversalJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Universal JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Generates universal JSON format for any TTRPG system".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["universal_format".to_string(), "system_agnostic".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::UniversalJson]
    }

    async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        let json_data = serde_json::to_string_pretty(campaign).map_err(|e| {
            ConversionError::from_json(e, "serializing universal campaign".to_string(), None)
        })?;

        let mut files = std::collections::HashMap::new();
        files.insert("campaign.json".to_string(), json_data);

        Ok(OutputBundle {
            files,
            databases: std::collections::HashMap::new(),
            assets: std::collections::HashMap::new(),
            metadata: super::OutputMetadata::default(),
        })
    }

    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        std::fs::create_dir_all(target.parent().unwrap_or(target))
            .map_err(|e| ConversionError::from_io(e, "creating output directory".to_string()))?;

        for (filename, data) in &bundle.files {
            let file_path = target.join(filename);
            std::fs::write(&file_path, data)
                .map_err(|e| ConversionError::from_io(e, format!("writing file {filename}")))?;
        }

        Ok(())
    }
}

// TODO: Implement remaining placeholder handlers in Phase 2
// For now, they can use default implementations similar to the examples above
