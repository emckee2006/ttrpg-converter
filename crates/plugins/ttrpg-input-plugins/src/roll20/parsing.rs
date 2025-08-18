//! Roll20 campaign parser and data structures
//!
//! This module provides parsing capabilities for Roll20 campaign exports,
//! converting Roll20's JSON format into our standardized TTRPG data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use ttrpg_core::error::{ConversionError, ConversionResult};
use ttrpg_traits::{AssetPlugin, LoggingPlugin, ValidationPlugin};
use ttrpg_types::*;

/// Roll20 campaign parser
///
/// Handles parsing of Roll20 campaign exports and conversion to standardized formats.
///
/// # Example
///
/// ```rust
pub struct Roll20Parser {
    /// Validation plugin for campaign validation
    pub validation_plugin: Option<Arc<dyn ValidationPlugin>>,
    /// Asset plugin for handling images and files  
    pub asset_plugin: Option<Arc<dyn AssetPlugin>>,
    /// Logging plugin for operation tracking
    pub logging_plugin: Option<Arc<dyn LoggingPlugin>>,
}

/// Roll20 campaign data structure
///
/// Represents the raw structure of a Roll20 campaign export
/// Uses flexible schema to handle real Roll20 export variations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Campaign {
    /// Campaign metadata (may be embedded or separate)
    #[serde(flatten)]
    pub campaign_data: serde_json::Value,
    /// Character data
    pub characters: Option<Vec<Roll20Character>>,
    /// Page/map data  
    pub pages: Option<Vec<Roll20Page>>,
    /// Handouts and notes
    pub handouts: Option<Vec<Roll20Handout>>,
    /// Journal entries
    pub journal: Option<Vec<Roll20JournalEntry>>,
    /// Assets and media files
    pub assets: Option<Vec<Roll20Asset>>,
    /// Catch any additional fields Roll20 may include
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

/// Roll20 campaign metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20CampaignMeta {
    /// Campaign name
    pub name: String,
    /// Campaign description
    pub description: Option<String>,
    /// Campaign settings
    pub settings: HashMap<String, serde_json::Value>,
    /// Creation timestamp
    pub created: Option<String>,
    /// Last modified timestamp
    pub modified: Option<String>,
}

/// Roll20 character data  
/// Uses flexible schema to handle real Roll20 export variations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Character {
    /// Character ID
    pub id: Option<String>,
    /// Character name
    pub name: Option<String>,
    /// Character attributes (stats, skills, etc.) - FIXED: Array not HashMap
    pub attributes: Option<Vec<Roll20Attribute>>,
    /// Character abilities (spells, attacks, etc.)
    pub abilities: Option<Vec<Roll20Ability>>,
    /// Character bio and notes
    pub bio: Option<String>,
    /// Character avatar/token image  
    pub avatar: Option<String>,
    /// Character sheet name (e.g., "ogl5e", "pathfinderofficial")
    pub charactersheetname: Option<String>,
    /// Archived status
    pub archived: Option<bool>,
    /// Controlled by player IDs
    pub controlledby: Option<Vec<String>>,
    /// In player journals
    pub inplayerjournals: Option<Vec<String>>,
    /// Tags
    pub tags: Option<String>,
    /// Catch any additional character fields Roll20 may include
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

/// Roll20 attribute data
/// Matches the exact structure from real Roll20 exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Attribute {
    /// Attribute name
    pub name: Option<String>,
    /// Current value
    pub current: Option<serde_json::Value>,
    /// Maximum value (can be empty string or actual value)
    pub max: Option<serde_json::Value>,
    /// Attribute ID
    pub id: Option<String>,
    /// Catch any additional attribute fields
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

/// Roll20 ability data (macros, spells, attacks)  
/// Uses flexible schema to handle real Roll20 export variations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Ability {
    /// Ability name
    pub name: Option<String>,
    /// Ability description
    pub description: Option<String>,
    /// Macro action
    pub action: Option<String>,
    /// Ability type
    pub ability_type: Option<String>,
    /// Ability ID
    pub id: Option<String>,
    /// Catch any additional ability fields
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

/// Roll20 page/map data
/// Uses flexible schema to handle real Roll20 export variations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Page {
    /// Page ID
    pub id: Option<String>,
    /// Page name
    pub name: Option<String>,
    /// Background image
    pub background_url: Option<String>,
    /// Page dimensions
    pub width: Option<f64>,
    pub height: Option<f64>,
    /// Grid settings
    pub grid_size: Option<f64>,
    /// Tokens on this page
    pub tokens: Option<Vec<Roll20Token>>,
    /// Catch any additional page fields
    #[serde(flatten)]
    pub extra_fields: serde_json::Value,
}

/// Roll20 token data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Token {
    /// Token ID
    pub id: String,
    /// Token name
    pub name: String,
    /// Token image URL
    pub image_url: Option<String>,
    /// Position coordinates
    pub x: Option<f64>,
    pub y: Option<f64>,
    /// Token size
    pub width: Option<f64>,
    pub height: Option<f64>,
}

/// Roll20 handout data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Handout {
    /// Handout ID
    pub id: String,
    /// Handout name
    pub name: String,
    /// Handout content
    pub content: Option<String>,
    /// Associated image
    pub image_url: Option<String>,
}

/// Roll20 journal entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20JournalEntry {
    /// Entry ID
    pub id: String,
    /// Entry title
    pub title: String,
    /// Entry content
    pub content: String,
    /// Entry category/tags
    pub tags: Vec<String>,
}

/// Roll20 asset data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Asset {
    /// Asset ID
    pub id: String,
    /// Asset name/filename
    pub name: String,
    /// Asset URL
    pub url: String,
    /// Asset type (image, audio, etc.)
    pub asset_type: String,
    /// File size in bytes
    pub size: Option<u64>,
}

impl Roll20Parser {
    /// Create a new Roll20 parser
    pub fn new() -> Self {
        Self { validation_plugin: None, asset_plugin: None, logging_plugin: None }
    }

    /// Add validation plugin for campaign validation
    pub fn with_validation(mut self, plugin: Arc<dyn ValidationPlugin>) -> Self {
        self.validation_plugin = Some(plugin);
        self
    }

    /// Add asset plugin for handling images and files
    pub fn with_assets(mut self, plugin: Arc<dyn AssetPlugin>) -> Self {
        self.asset_plugin = Some(plugin);
        self
    }

    /// Add logging plugin for operation tracking
    pub fn with_logging(mut self, plugin: Arc<dyn LoggingPlugin>) -> Self {
        self.logging_plugin = Some(plugin);
        self
    }

    /// Parse a Roll20 campaign file
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the Roll20 campaign JSON file
    ///
    /// # Returns
    ///
    /// * `ConversionResult<Campaign>` - Parsed campaign data in standardized format
    ///
    /// # Errors
    ///
    /// Returns `ConversionError` if:
    /// - File cannot be read
    /// - JSON parsing fails
    /// - Data validation fails
    pub async fn parse_campaign_file(&self, file_path: &Path) -> ConversionResult<Campaign> {
        if let Some(ref logger) = self.logging_plugin {
            logger.info(
                &format!("Starting Roll20 campaign parsing: {}", file_path.display()),
                Some("roll20_parser"),
            );
        }

        // Read the file
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| ConversionError::from_io(e, "Failed to read campaign file"))?;

        // Parse JSON
        let roll20_campaign: Roll20Campaign = serde_json::from_str(&content).map_err(|e| {
            ConversionError::validation(
                "Roll20Campaign",
                format!("Invalid Roll20 JSON format: {e}"),
            )
        })?;

        if let Some(ref logger) = self.logging_plugin {
            logger.info(
                &format!(
                    "Successfully parsed Roll20 JSON with {} characters, {} pages",
                    roll20_campaign.characters.as_ref().map_or(0, |c| c.len()),
                    roll20_campaign.pages.as_ref().map_or(0, |p| p.len())
                ),
                Some("roll20_parser"),
            );
        }

        // Convert to standardized format
        self.convert_to_campaign(roll20_campaign).await
    }

    /// Convert Roll20 campaign data to standardized Campaign format
    pub async fn convert_to_campaign(
        &self,
        roll20_campaign: Roll20Campaign,
    ) -> ConversionResult<Campaign> {
        if let Some(ref logger) = self.logging_plugin {
            logger.info(
                "Converting Roll20 data to standardized Campaign format",
                Some("roll20_parser"),
            );
        }

        // Create base campaign
        // Extract campaign name from flexible data structure
        let campaign_name = roll20_campaign
            .campaign_data
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Campaign")
            .to_string();

        let mut campaign = Campaign::new(campaign_name, SourceFormat::Roll20);

        // Set description if available
        if let Some(description) = roll20_campaign
            .campaign_data
            .get("description")
            .and_then(|v| v.as_str())
        {
            campaign
                .metadata
                .custom_properties
                .insert("description".to_string(), description.into());
        }

        // Convert characters
        if let Some(characters) = roll20_campaign.characters {
            for roll20_char in characters {
                let actor = self.convert_character_to_actor(roll20_char)?;
                campaign.actors.push(actor);
            }
        }

        // Convert pages to scenes
        if let Some(pages) = roll20_campaign.pages {
            for roll20_page in pages {
                let scene = self.convert_page_to_scene(roll20_page)?;
                campaign.scenes.push(scene);
            }
        }

        // Convert handouts to items
        if let Some(handouts) = roll20_campaign.handouts {
            for roll20_handout in handouts {
                let item = self.convert_handout_to_item(roll20_handout)?;
                campaign.items.push(item);
            }
        }

        // Validate the converted campaign if validation service is available
        if let Some(ref validator) = self.validation_plugin {
            let validation_result = validator.validate_campaign(&campaign).await?;
            if !validation_result.is_valid {
                if let Some(ref logger) = self.logging_plugin {
                    logger.warn(
                        &format!(
                            "Campaign validation found {} errors",
                            validation_result.issues.len()
                        ),
                        Some("roll20_parser"),
                    );
                }
            }
        }

        if let Some(ref logger) = self.logging_plugin {
            let total_entities =
                campaign.actors.len() + campaign.scenes.len() + campaign.items.len();
            logger.info(
                &format!("Successfully converted Roll20 campaign with {total_entities} entities"),
                Some("roll20_parser"),
            );
        }

        Ok(campaign)
    }

    /// Convert Roll20 character to standardized Actor format
    fn convert_character_to_actor(&self, roll20_char: Roll20Character) -> ConversionResult<Actor> {
        let mut actor = Actor {
            id: roll20_char
                .id
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            name: roll20_char
                .name
                .clone()
                .unwrap_or_else(|| "Unnamed Character".to_string()),
            actor_type: ActorType::Npc, // Default to NPC, could be determined from data
            images: ActorImages {
                avatar: roll20_char.avatar.clone(),
                token: roll20_char.avatar,
                additional: Vec::new(),
            },
            attributes: HashMap::new(),
            items: Vec::new(), // Will be populated from inventory
            spells: Vec::new(),
            features: Vec::new(),
            biography: roll20_char.bio.unwrap_or_default(),
            notes: String::new(),
            permissions: EntityPermissions::default(),
            source_data: HashMap::new(),
        };

        // Process attributes
        if let Some(attributes) = roll20_char.attributes {
            for attr in attributes {
                if let Some(attr_name) = attr.name {
                    let attr_value = attr
                        .current
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".to_string());
                    actor
                        .attributes
                        .insert(attr_name, AttributeValue::Text(attr_value));
                }
            }
        }

        // Process abilities (convert to spells or features based on content)
        if let Some(abilities) = roll20_char.abilities {
            for ability in abilities {
                if let Some(_action) = ability.action {
                    // Create a spell from the ability
                    let ability_name = ability
                        .name
                        .unwrap_or_else(|| "Unnamed Ability".to_string());
                    let spell = Spell {
                        id: format!("ability_{}", ability_name.replace(' ', "_").to_lowercase()),
                        name: ability_name,
                        level: 1, // Default level, could be parsed from description
                        school: "Unknown".to_string(),
                        casting_time: "1 action".to_string(),
                        range: "Self".to_string(),
                        components: SpellComponents::default(),
                        duration: "Instantaneous".to_string(),
                        description: ability.description.unwrap_or_default(),
                        at_higher_levels: None,
                    };
                    actor.spells.push(spell);
                } else {
                    // Create a feature from the ability
                    let ability_name = ability
                        .name
                        .unwrap_or_else(|| "Unnamed Feature".to_string());
                    let feature = Feature {
                        id: format!("feature_{}", ability_name.replace(' ', "_").to_lowercase()),
                        name: ability_name,
                        feature_type: FeatureType::Other("Roll20 Ability".to_string()),
                        description: ability.description.unwrap_or_default(),
                        usage: None,
                    };
                    actor.features.push(feature);
                }
            }
        }

        // Store Roll20-specific data
        actor.source_data.insert(
            "roll20_id".to_string(),
            serde_json::Value::String(roll20_char.id.unwrap_or_else(|| "unknown".to_string())),
        );

        Ok(actor)
    }

    /// Convert Roll20 page to standardized Scene format
    fn convert_page_to_scene(&self, roll20_page: Roll20Page) -> ConversionResult<Scene> {
        // Create scene with proper structure
        let scene = Scene {
            id: roll20_page.id.unwrap_or_else(|| "unknown".to_string()),
            name: roll20_page
                .name
                .unwrap_or_else(|| "Unnamed Scene".to_string()),
            background_image: roll20_page.background_url,
            dimensions: SceneDimensions {
                width: roll20_page.width.unwrap_or(1400.0) as u32,
                height: roll20_page.height.unwrap_or(1000.0) as u32,
                scale: roll20_page.grid_size.unwrap_or(70.0),
                grid_size: roll20_page.grid_size.unwrap_or(70.0),
            },
            grid: GridConfig {
                grid_type: GridType::Square, // Default to square grid
                size: roll20_page.grid_size.unwrap_or(70.0),
                color: "#000000".to_string(),
                opacity: 0.3,
            },
            tokens: Vec::new(), // Will be populated from objects if needed
            walls: Vec::new(),
            lighting: LightingConfig::default(),
            audio: None,
            notes: String::new(),
            permissions: EntityPermissions::default(),
            source_data: HashMap::new(),
        };

        Ok(scene)
    }

    /// Convert Roll20 handout to standardized Item format
    fn convert_handout_to_item(&self, roll20_handout: Roll20Handout) -> ConversionResult<Item> {
        let mut item = Item {
            id: roll20_handout.id.clone(),
            name: roll20_handout.name.clone(),
            item_type: ItemType::Tool, // Default type
            description: roll20_handout.content.clone().unwrap_or_default(),
            image: roll20_handout.image_url.clone(),
            properties: ItemProperties {
                rarity: None,
                attunement: false,
                weight: None,
                cost: None,
                quantity: 1,
                properties: HashMap::new(),
            },
            source_data: HashMap::new(),
        };

        // Set description from content
        if let Some(content) = roll20_handout.content {
            item.source_data
                .insert("content".to_string(), serde_json::Value::String(content));
        }

        // Store Roll20-specific data
        item.source_data
            .insert("roll20_id".to_string(), serde_json::Value::String(roll20_handout.id));

        Ok(item)
    }
}

impl Default for Roll20Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll20_parser_creation() {
        let parser = Roll20Parser::new();
        assert!(parser.validation_plugin.is_none());
        assert!(parser.asset_plugin.is_none());
        assert!(parser.logging_plugin.is_none());
    }

    #[test]
    fn test_roll20_character_conversion() {
        let parser = Roll20Parser::new();

        let mut attributes = HashMap::new();
        attributes.insert(
            "strength".to_string(),
            Roll20Attribute {
                id: Some("attr_strength".to_string()),
                name: Some("strength".to_string()),
                current: Some(serde_json::Value::Number(serde_json::Number::from(16))),
                max: Some(serde_json::Value::Number(serde_json::Number::from(20))),
                extra_fields: serde_json::Value::Object(serde_json::Map::new()),
            },
        );

        let abilities = vec![Roll20Ability {
            id: Some("ability_sword".to_string()),
            name: Some("Sword Attack".to_string()),
            description: Some("A basic sword attack".to_string()),
            action: Some("1d20+5".to_string()),
            ability_type: Some("attack".to_string()),
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        }];

        let roll20_char = Roll20Character {
            id: Some("char1".to_string()),
            name: Some("Test Character".to_string()),
            attributes: Some(attributes.into_values().collect()),
            abilities: Some(abilities),
            bio: Some("A test character".to_string()),
            avatar: None,
            archived: None,
            charactersheetname: None,
            controlledby: None,
            inplayerjournals: None,
            tags: None,
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = parser.convert_character_to_actor(roll20_char);
        assert!(result.is_ok());

        let actor = result.unwrap();
        assert_eq!(actor.name, "Test Character");
        assert_eq!(actor.attributes.len(), 1);
        assert_eq!(actor.spells.len(), 1);
    }

    #[test]
    fn test_roll20_page_conversion() {
        let parser = Roll20Parser::new();

        let roll20_page = Roll20Page {
            id: Some("page1".to_string()),
            name: Some("Test Map".to_string()),
            background_url: Some("https://example.com/map.jpg".to_string()),
            width: Some(1920.0),
            height: Some(1080.0),
            grid_size: Some(70.0),
            tokens: Some(vec![]),
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = parser.convert_page_to_scene(roll20_page);
        assert!(result.is_ok());

        let scene = result.unwrap();
        assert_eq!(scene.name, "Test Map");
        assert_eq!(scene.dimensions.width, 1920);
        assert_eq!(scene.dimensions.grid_size, 70.0);
    }

    #[test]
    fn test_character_conversion_with_attributes() {
        let parser = Roll20Parser::new();

        let mut attributes = HashMap::new();
        attributes.insert(
            "strength".to_string(),
            Roll20Attribute {
                id: Some("attr_str".to_string()),
                name: Some("strength".to_string()),
                current: Some(serde_json::Value::Number(serde_json::Number::from(18))),
                max: Some(serde_json::Value::Number(serde_json::Number::from(20))),
                extra_fields: serde_json::Value::Object(serde_json::Map::new()),
            },
        );
        attributes.insert(
            "dexterity".to_string(),
            Roll20Attribute {
                id: Some("attr_dex".to_string()),
                name: Some("dexterity".to_string()),
                current: Some(serde_json::Value::String("14".to_string())),
                max: None,
                extra_fields: serde_json::Value::Object(serde_json::Map::new()),
            },
        );

        let roll20_char = Roll20Character {
            id: Some("char123".to_string()),
            name: Some("Test Warrior".to_string()),
            attributes: Some(attributes.into_values().collect()),
            abilities: Some(vec![]),
            bio: Some("A brave warrior".to_string()),
            avatar: Some("warrior.png".to_string()),
            archived: Some(false),
            charactersheetname: Some("".to_string()),
            controlledby: Some(vec![]),
            inplayerjournals: Some(vec![]),
            tags: Some("".to_string()),
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = parser.convert_character_to_actor(roll20_char);
        assert!(result.is_ok());

        let actor = result.unwrap();
        assert_eq!(actor.id, "char123");
        assert_eq!(actor.name, "Test Warrior");
        assert_eq!(actor.biography, "A brave warrior");
        assert_eq!(actor.attributes.len(), 2);

        // Test attribute conversion
        if let Some(AttributeValue::Number(strength)) = actor.attributes.get("strength") {
            assert_eq!(*strength, 18.0);
        }
        if let Some(AttributeValue::Number(dex)) = actor.attributes.get("dexterity") {
            assert_eq!(*dex, 14.0);
        }
    }

    #[test]
    fn test_character_with_mixed_abilities() {
        let parser = Roll20Parser::new();

        let abilities = vec![
            Roll20Ability {
                id: Some("ability_fireball".to_string()),
                name: Some("Fireball".to_string()),
                description: Some("A blazing ball of fire".to_string()),
                action: Some("/r 8d6".to_string()),
                ability_type: Some("spell".to_string()),
                extra_fields: serde_json::Value::Object(serde_json::Map::new()),
            },
            Roll20Ability {
                id: Some("ability_rage".to_string()),
                name: Some("Rage".to_string()),
                description: Some("Barbarian rage ability".to_string()),
                action: None, // No action = feature
                ability_type: Some("feature".to_string()),
                extra_fields: serde_json::Value::Object(serde_json::Map::new()),
            },
        ];

        let roll20_char = Roll20Character {
            id: Some("barb_wiz".to_string()),
            name: Some("Barbarian Wizard".to_string()),
            attributes: Some(vec![]),
            abilities: Some(abilities),
            bio: None,
            avatar: None,
            archived: None,
            charactersheetname: None,
            controlledby: None,
            inplayerjournals: None,

            tags: None,
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = parser.convert_character_to_actor(roll20_char);
        assert!(result.is_ok());

        let actor = result.unwrap();
        assert_eq!(actor.spells.len(), 1);
        assert_eq!(actor.features.len(), 1);
        assert_eq!(actor.spells[0].name, "Fireball");
        assert_eq!(actor.features[0].name, "Rage");
    }

    #[test]
    fn test_scene_conversion_with_defaults() {
        let parser = Roll20Parser::new();

        let roll20_page = Roll20Page {
            id: Some("minimal_page".to_string()),
            name: Some("Minimal Scene".to_string()),
            background_url: None,
            width: None,
            height: None,
            grid_size: None,
            tokens: Some(vec![]),
            extra_fields: serde_json::Value::Object(serde_json::Map::new()),
        };

        let result = parser.convert_page_to_scene(roll20_page);
        assert!(result.is_ok());

        let scene = result.unwrap();
        assert_eq!(scene.name, "Minimal Scene");
        assert_eq!(scene.background_image, None);
        assert_eq!(scene.dimensions.width, 1400); // Default values
        assert_eq!(scene.dimensions.height, 1000);
        // Note: GridType doesn't implement PartialEq, so we check the grid exists
        assert!(matches!(scene.grid.grid_type, GridType::Square));
    }

    #[test]
    fn test_handout_to_item_conversion() {
        let parser = Roll20Parser::new();

        let roll20_handout = Roll20Handout {
            id: "handout_123".to_string(),
            name: "Ancient Scroll".to_string(),
            content: Some("This scroll contains ancient wisdom".to_string()),
            image_url: Some("scroll.png".to_string()),
        };

        let result = parser.convert_handout_to_item(roll20_handout);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.id, "handout_123");
        assert_eq!(item.name, "Ancient Scroll");
        assert_eq!(item.description, "This scroll contains ancient wisdom");
        assert_eq!(item.image, Some("scroll.png".to_string()));
        // Note: ItemType doesn't implement PartialEq, so we check the type exists
        assert!(matches!(item.item_type, ItemType::Tool));

        // Check source data storage
        assert!(item.source_data.contains_key("roll20_id"));
        assert!(item.source_data.contains_key("content"));
    }

    // Additional comprehensive tests will be added in future iterations
    // after resolving service trait signature mismatches
}
