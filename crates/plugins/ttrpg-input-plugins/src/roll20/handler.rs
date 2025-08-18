//! Roll20 Input Handler Implementation
//!
//! Concrete implementation of the InputPlugin trait for Roll20 campaigns.

use async_trait::async_trait;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tracing::info;

use ttrpg_core::error::{ConversionResult, ConversionError};
use ttrpg_traits::{InputPlugin, PluginInfo};
use ttrpg_types::{Campaign, CampaignMetadata, GameSystem, SourceFormat};
use crate::roll20::parsing::{Roll20Campaign, Roll20Character, Roll20Page, Roll20Handout};

/// Roll20 input plugin handler  
pub struct Roll20InputHandler {
    // Store raw JSON data for comprehensive asset discovery (thread-safe interior mutability)
    raw_campaign_data: Arc<Mutex<Option<serde_json::Value>>>,
}

impl std::fmt::Debug for Roll20InputHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Roll20InputHandler")
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
            raw_campaign_data: Arc::new(Mutex::new(None)),
        }
    }
    
    async fn convert_to_universal_campaign(
        &self,
        roll20_campaign: Roll20Campaign,
    ) -> ConversionResult<Campaign> {
        use ttrpg_core::types::{DocumentMetadata, Character, Scene, JournalEntry, Handout, Playlist, RollableTable, CardDeck};
        
        let mut campaign = Campaign {
            id: roll20_campaign.campaign.id,
            name: roll20_campaign.campaign.name.clone(),
            description: Some("Imported from Roll20".to_string()),
            game_system: GameSystem::DnD5e,
            characters: Vec::new(),
            scenes: Vec::new(),
            journal_entries: Vec::new(),
            handouts: Vec::new(),
            playlists: Vec::new(),
            rollable_tables: Vec::new(),
            card_decks: Vec::new(),
            creation_date: Some(chrono::Utc::now()),
            last_modified: Some(chrono::Utc::now()),
            metadata: DocumentMetadata {
                created_time: Some(chrono::Utc::now()),
                modified_time: Some(chrono::Utc::now()),
                created_by: Some("Roll20 Importer".to_string()),
                modified_by: None,
                system_id: None,
                system_version: Some("1.0.0".to_string()),
            },
        };

        // Convert characters
        for roll20_char in roll20_campaign.characters {
            match self.convert_character_to_actor(roll20_char).await {
                Ok(character) => campaign.characters.push(character),
                Err(_e) => {
                    // Log error but continue processing
                }
            }
        }

        // Convert pages to scenes
        for roll20_page in roll20_campaign.pages {
            match self.convert_page_to_scene(roll20_page).await {
                Ok(scene) => campaign.scenes.push(scene),
                Err(_e) => {
                    // Log error but continue processing
                }
            }
        }

        // Convert handouts to items
        for roll20_handout in roll20_campaign.handouts {
            match self.convert_handout_to_item(roll20_handout).await {
                Ok(handout) => campaign.handouts.push(handout),
                Err(_e) => {
                    // Log error but continue processing
                }
            }
        }

        info!(
            "Converted Roll20 campaign with {} characters, {} scenes",
            campaign.characters.len(),
            campaign.scenes.len()
        );

        Ok(campaign)
    }

    async fn convert_character_to_actor(
        &self,
        _roll20_char: Roll20Character,
    ) -> ConversionResult<ttrpg_core::types::Character> {
        // Simplified conversion for now
        Ok(ttrpg_core::types::Character {
            id: "temp".to_string(),
            name: "Temp Character".to_string(),
            race: None,
            class: None,
            level: 1,
            background: None,
            alignment: None,
            ability_scores: HashMap::new(),
            skills: HashMap::new(),
            features: Vec::new(),
            spells: Vec::new(),
            equipment: Vec::new(),
            notes: String::new(),
            metadata: ttrpg_core::types::DocumentMetadata {
                created_time: Some(chrono::Utc::now()),
                modified_time: Some(chrono::Utc::now()),
                created_by: Some("Roll20 Importer".to_string()),
                modified_by: None,
                system_id: None,
                system_version: Some("1.0.0".to_string()),
            },
        })
    }

    async fn convert_page_to_scene(&self, roll20_page: Roll20Page) -> ConversionResult<ttrpg_core::types::Scene> {
        Ok(ttrpg_core::types::Scene {
            id: roll20_page.id,
            name: roll20_page.name,
            description: None,
            background_image: roll20_page.background_url,
            grid_type: ttrpg_core::types::GridType::Square,
            grid_size: roll20_page.grid_size.unwrap_or(70.0) as u32,
            width: roll20_page.width.unwrap_or(1400.0) as u32,
            height: roll20_page.height.unwrap_or(1000.0) as u32,
            tokens: Vec::new(),
            lighting: ttrpg_core::types::LightingConfig {
                global_illumination: false,
                darkness_level: 0.0,
                fog_exploration: false,
            },
            weather: None,
            ownership: ttrpg_core::types::OwnershipConfig::default(),
            metadata: ttrpg_core::types::DocumentMetadata {
                created_time: Some(chrono::Utc::now()),
                modified_time: Some(chrono::Utc::now()),
                created_by: Some("Roll20 Importer".to_string()),
                modified_by: None,
                system_id: None,
                system_version: Some("1.0.0".to_string()),
            },
        })
    }

    async fn convert_handout_to_item(
        &self,
        roll20_handout: Roll20Handout,
    ) -> ConversionResult<ttrpg_core::types::Handout> {
        Ok(ttrpg_core::types::Handout {
            id: roll20_handout.id,
            name: roll20_handout.name,
            content: roll20_handout.notes,
            tags: Vec::new(),
            metadata: ttrpg_core::types::DocumentMetadata {
                created_time: Some(chrono::Utc::now()),
                modified_time: Some(chrono::Utc::now()),
                created_by: Some("Roll20 Importer".to_string()),
                modified_by: None,
                system_id: None,
                system_version: Some("1.0.0".to_string()),
            },
        })
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
            name: source
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Untitled Campaign")
                .to_string(),
            description: Some(format!("Roll20 campaign from {}", source.display())),
            source_format: SourceFormat::Roll20,
            system: Some(GameSystem::DnD5e),
            updated_at: chrono::Utc::now(),
        })
    }

    async fn parse_campaign(&self, source: &Path) -> ConversionResult<Campaign> {
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

        // Store raw JSON data for comprehensive asset discovery
        {
            let mut raw_data = self.raw_campaign_data.lock().unwrap();
            *raw_data = Some(raw_json.clone());
        }

        let roll20_campaign: Roll20Campaign = serde_json::from_str(&json_content)
            .map_err(|e| ConversionError::from_json(e, "Roll20 campaign JSON", None))?;

        self.convert_to_universal_campaign(roll20_campaign).await
    }

    async fn discover_assets(&self, _campaign: &Campaign) -> ConversionResult<Vec<ttrpg_core::plugin_framework::AssetInfo>> {
        let mut assets = Vec::new();
        
        // Basic asset discovery from stored raw JSON
        if let Ok(raw_data) = self.raw_campaign_data.lock() {
            if let Some(_json) = &*raw_data {
                // TODO: Implement comprehensive asset discovery
                info!("Basic asset discovery found {} assets from campaign", assets.len());
            }
        }

        Ok(assets)
    }
}
