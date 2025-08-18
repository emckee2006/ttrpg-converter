# M3: Platform Format Plugins - Junior Developer Implementation Guide

## 🎯 **MILESTONE 3 OVERVIEW**
**Duration**: 2.5 weeks | **Total Points**: 20 | **Priority**: 🚨 CRITICAL
**Dependencies**: M2 Processing Plugins DLL Complete

Unified platform format plugins as DLLs - Roll20.dll, Foundry.dll, Pathbuilder.dll each handling import+export+mappings for their respective platforms.

### 🔌 **PLATFORM PLUGIN FEATURES**
- **Roll20.dll**: JSON import/export with character sheet mappings
- **Foundry.dll**: World folder import/export with LevelDB integration  
- **Pathbuilder.dll**: Character import with PF2e system mappings
- **Unified Architecture**: Each plugin handles complete platform lifecycle

---

## **T3.1: Roll20 Unified Plugin DLL**
**Duration**: 6 days | **Points**: 7 | **Priority**: 🚨 CRITICAL

### **Implementation Steps**

**Step 1: Create Roll20 Plugin Crate**
```bash
cd crates/plugins
cargo new ttrpg-roll20-plugin --lib
```

Update `ttrpg-roll20-plugin/Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
ttrpg-roll20 = { path = "../../platforms/ttrpg-roll20" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
async-trait = "0.1"
uuid = { version = "1.6", features = ["v4"] }
```

**Step 2: Roll20 Plugin Implementation**
Create `src/lib.rs`:
```rust
use std::path::Path;
use async_trait::async_trait;
use ttrpg_core::plugins::{PlatformFormatPlugin, SharedServices, PlatformOutput, PlatformMetadata, OutputFile};
use ttrpg_core::types::common::*;
use ttrpg_roll20::generated::*;

pub struct Roll20FormatPlugin {
    shared_services: SharedServices,
}

impl Roll20FormatPlugin {
    pub fn new(services: SharedServices) -> Self {
        Self {
            shared_services: services,
        }
    }
    
    /// Parse Roll20 JSON export file
    async fn parse_roll20_json(&self, path: &Path) -> Result<Campaign, ConversionError> {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| ConversionError::ReadError(e.to_string()))?;
            
        let roll20_data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ConversionError::ParseError(e.to_string()))?;
            
        let mut campaign = Campaign {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: CampaignMetadata {
                name: roll20_data.get("campaign_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Imported Campaign")
                    .to_string(),
                system: "dnd5e".to_string(), // Default, can be detected
                version: "1.0.0".to_string(),
                created_at: Some(chrono::Utc::now()),
                validation_status: None,
                validation_timestamp: None,
            },
            characters: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journals: Vec::new(),
        };
        
        // Parse characters
        if let Some(characters) = roll20_data.get("characters").and_then(|v| v.as_array()) {
            for char_data in characters {
                if let Ok(character) = self.convert_roll20_character(char_data).await {
                    campaign.characters.push(character);
                }
            }
        }
        
        // Parse handouts as journal entries
        if let Some(handouts) = roll20_data.get("handouts").and_then(|v| v.as_array()) {
            for handout_data in handouts {
                if let Ok(journal) = self.convert_roll20_handout(handout_data).await {
                    campaign.journals.push(journal);
                }
            }
        }
        
        // Parse pages as scenes
        if let Some(pages) = roll20_data.get("pages").and_then(|v| v.as_array()) {
            for page_data in pages {
                if let Ok(scene) = self.convert_roll20_page(page_data).await {
                    campaign.scenes.push(scene);
                }
            }
        }
        
        Ok(campaign)
    }
    
    async fn convert_roll20_character(&self, char_data: &serde_json::Value) -> Result<TtrpgCharacter, ConversionError> {
        let name = char_data.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unnamed Character")
            .to_string();
            
        let attributes = char_data.get("attribs").and_then(|v| v.as_array());
        let mut ability_scores = AbilityScores::default();
        
        if let Some(attrs) = attributes {
            for attr in attrs {
                if let (Some(attr_name), Some(current_val)) = (
                    attr.get("name").and_then(|v| v.as_str()),
                    attr.get("current").and_then(|v| v.as_str())
                ) {
                    if let Ok(value) = current_val.parse::<i32>() {
                        match attr_name {
                            "strength" => ability_scores.strength = value,
                            "dexterity" => ability_scores.dexterity = value,
                            "constitution" => ability_scores.constitution = value,
                            "intelligence" => ability_scores.intelligence = value,
                            "wisdom" => ability_scores.wisdom = value,
                            "charisma" => ability_scores.charisma = value,
                            _ => {}
                        }
                    }
                }
            }
        }
        
        Ok(TtrpgCharacter {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: CharacterMetadata {
                name,
                player_name: char_data.get("controlledby")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                character_level: Some(1), // Default
                character_class: None,
                race: None,
                validation_status: None,
            },
            abilities: Some(ability_scores),
            skills: None,
            equipment: None,
            spells: None,
            system_data: char_data.clone(),
        })
    }
    
    async fn convert_roll20_handout(&self, handout_data: &serde_json::Value) -> Result<TtrpgJournal, ConversionError> {
        Ok(TtrpgJournal {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: JournalMetadata {
                name: handout_data.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled Handout")
                    .to_string(),
                category: Some("handout".to_string()),
                tags: Vec::new(),
                validation_status: None,
            },
            content: handout_data.get("notes")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            system_data: handout_data.clone(),
        })
    }
    
    async fn convert_roll20_page(&self, page_data: &serde_json::Value) -> Result<TtrpgScene, ConversionError> {
        Ok(TtrpgScene {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: SceneMetadata {
                name: page_data.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled Scene")
                    .to_string(),
                description: None,
                tags: Vec::new(),
                validation_status: None,
            },
            dimensions: SceneDimensions {
                width: page_data.get("width")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000) as u32,
                height: page_data.get("height")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000) as u32,
                grid_size: page_data.get("snapping_increment")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(70) as u32,
            },
            background: page_data.get("background_url")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            tokens: Vec::new(), // Would need to parse graphics array
            lighting: None,
            system_data: page_data.clone(),
        })
    }
    
    /// Generate Roll20 JSON export
    async fn generate_roll20_json(&self, campaign: &Campaign) -> Result<PlatformOutput, ConversionError> {
        let mut roll20_export = serde_json::Map::new();
        
        roll20_export.insert("campaign_name".to_string(), 
            serde_json::Value::String(campaign.metadata.name.clone()));
            
        // Convert characters to Roll20 format
        let mut characters = Vec::new();
        for character in &campaign.characters {
            let roll20_char = self.convert_character_to_roll20(character).await?;
            characters.push(roll20_char);
        }
        roll20_export.insert("characters".to_string(), serde_json::Value::Array(characters));
        
        // Convert journal entries to handouts
        let mut handouts = Vec::new();
        for journal in &campaign.journals {
            let roll20_handout = self.convert_journal_to_roll20_handout(journal).await?;
            handouts.push(roll20_handout);
        }
        roll20_export.insert("handouts".to_string(), serde_json::Value::Array(handouts));
        
        // Convert scenes to pages
        let mut pages = Vec::new();
        for scene in &campaign.scenes {
            let roll20_page = self.convert_scene_to_roll20_page(scene).await?;
            pages.push(roll20_page);
        }
        roll20_export.insert("pages".to_string(), serde_json::Value::Array(pages));
        
        let json_content = serde_json::to_string_pretty(&roll20_export)
            .map_err(|e| ConversionError::SerializationError(e.to_string()))?;
            
        Ok(PlatformOutput {
            files: vec![OutputFile {
                path: std::path::PathBuf::from("campaign_export.json"),
                content: json_content.into_bytes(),
                mime_type: "application/json".to_string(),
            }],
            metadata: self.get_platform_metadata(),
        })
    }
    
    async fn convert_character_to_roll20(&self, character: &TtrpgCharacter) -> Result<serde_json::Value, ConversionError> {
        let mut roll20_char = serde_json::Map::new();
        
        roll20_char.insert("name".to_string(), 
            serde_json::Value::String(character.metadata.name.clone()));
            
        if let Some(player) = &character.metadata.player_name {
            roll20_char.insert("controlledby".to_string(),
                serde_json::Value::String(player.clone()));
        }
        
        // Convert ability scores to Roll20 attributes
        if let Some(abilities) = &character.abilities {
            let mut attributes = Vec::new();
            
            let ability_attrs = [
                ("strength", abilities.strength),
                ("dexterity", abilities.dexterity),
                ("constitution", abilities.constitution),
                ("intelligence", abilities.intelligence),
                ("wisdom", abilities.wisdom),
                ("charisma", abilities.charisma),
            ];
            
            for (name, value) in &ability_attrs {
                attributes.push(serde_json::json!({
                    "name": name,
                    "current": value.to_string(),
                    "max": ""
                }));
            }
            
            roll20_char.insert("attribs".to_string(), serde_json::Value::Array(attributes));
        }
        
        Ok(serde_json::Value::Object(roll20_char))
    }
    
    async fn convert_journal_to_roll20_handout(&self, journal: &TtrpgJournal) -> Result<serde_json::Value, ConversionError> {
        Ok(serde_json::json!({
            "name": journal.metadata.name,
            "notes": journal.content,
            "gmnotes": "",
            "inplayerjournals": "",
            "controlledby": ""
        }))
    }
    
    async fn convert_scene_to_roll20_page(&self, scene: &TtrpgScene) -> Result<serde_json::Value, ConversionError> {
        Ok(serde_json::json!({
            "name": scene.metadata.name,
            "width": scene.dimensions.width,
            "height": scene.dimensions.height,
            "snapping_increment": scene.dimensions.grid_size,
            "background_url": scene.background.as_deref().unwrap_or(""),
            "scale_number": 70,
            "scale_units": "ft"
        }))
    }
}

#[async_trait]
impl PlatformFormatPlugin for Roll20FormatPlugin {
    fn can_handle_input(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            extension.to_lowercase() == "json"
        } else {
            false
        }
    }
    
    fn get_supported_extensions(&self) -> Vec<&str> {
        vec!["json"]
    }
    
    async fn parse_campaign(&self, path: &Path) -> Result<Campaign, ConversionError> {
        self.parse_roll20_json(path).await
    }
    
    async fn generate_campaign(&self, campaign: &Campaign) -> Result<PlatformOutput, ConversionError> {
        self.generate_roll20_json(campaign).await
    }
    
    fn get_platform_metadata(&self) -> PlatformMetadata {
        PlatformMetadata {
            platform_type: PlatformType::Roll20,
            version: "1.0.0".to_string(),
            supported_systems: vec!["dnd5e".to_string(), "pf1e".to_string()],
        }
    }
}

impl ttrpg_core::plugins::Plugin for Roll20FormatPlugin {
    fn name(&self) -> &str {
        "Roll20FormatPlugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "Unified Roll20 import/export plugin with character sheet mappings"
    }
}

#[no_mangle]
pub extern "C" fn create_format_plugin(services: SharedServices) -> *mut dyn PlatformFormatPlugin {
    Box::into_raw(Box::new(Roll20FormatPlugin::new(services)))
}
```

---

## **T3.2: Foundry Unified Plugin DLL**
**Duration**: 6 days | **Points**: 7 | **Priority**: 🚨 CRITICAL

### **Implementation Steps**

**Step 1: Create Foundry Plugin**
```bash
cargo new ttrpg-foundry-plugin --lib
```

Update `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
ttrpg-foundry-common = { path = "../../platforms/ttrpg-foundry-common" }
leveldb = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.35", features = ["full"] }
walkdir = "2.4"
```

**Step 2: Foundry Plugin Implementation**
Create `src/lib.rs`:
```rust
use std::path::Path;
use async_trait::async_trait;
use ttrpg_core::plugins::{PlatformFormatPlugin, SharedServices, PlatformOutput};
use ttrpg_foundry_common::generated::*;

pub struct FoundryFormatPlugin {
    shared_services: SharedServices,
}

impl FoundryFormatPlugin {
    pub fn new(services: SharedServices) -> Self {
        Self {
            shared_services: services,
        }
    }
    
    /// Parse Foundry world directory
    async fn parse_foundry_world(&self, world_path: &Path) -> Result<Campaign, ConversionError> {
        let world_json_path = world_path.join("world.json");
        if !world_json_path.exists() {
            return Err(ConversionError::InvalidFormat("Missing world.json".to_string()));
        }
        
        let world_data: serde_json::Value = {
            let content = tokio::fs::read_to_string(&world_json_path).await
                .map_err(|e| ConversionError::ReadError(e.to_string()))?;
            serde_json::from_str(&content)
                .map_err(|e| ConversionError::ParseError(e.to_string()))?
        };
        
        let mut campaign = Campaign {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: CampaignMetadata {
                name: world_data.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Foundry Campaign")
                    .to_string(),
                system: world_data.get("system")
                    .and_then(|v| v.as_str())
                    .unwrap_or("dnd5e")
                    .to_string(),
                version: world_data.get("coreVersion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("11")
                    .to_string(),
                created_at: Some(chrono::Utc::now()),
                validation_status: None,
                validation_timestamp: None,
            },
            characters: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journals: Vec::new(),
        };
        
        // Use database service to read LevelDB data
        let data_path = world_path.join("data");
        if data_path.exists() {
            let mut db_service = self.shared_services.database_service.lock().await;
            let world_name = campaign.metadata.name.clone();
            
            // Read actors (characters)
            if let Ok(actor_ids) = db_service.list_entities(&world_name, "actors") {
                for actor_id in actor_ids {
                    if let Ok(Some(foundry_actor)) = db_service.get_entity::<FoundryPf2eActor>(&world_name, "actors", &actor_id) {
                        let character = self.convert_foundry_actor_to_character(&foundry_actor).await?;
                        campaign.characters.push(character);
                    }
                }
            }
            
            // Read scenes
            if let Ok(scene_ids) = db_service.list_entities(&world_name, "scenes") {
                for scene_id in scene_ids {
                    if let Ok(Some(foundry_scene)) = db_service.get_entity::<FoundryPf2eScene>(&world_name, "scenes", &scene_id) {
                        let scene = self.convert_foundry_scene_to_scene(&foundry_scene).await?;
                        campaign.scenes.push(scene);
                    }
                }
            }
        }
        
        Ok(campaign)
    }
    
    /// Generate Foundry world folder
    async fn generate_foundry_world(&self, campaign: &Campaign) -> Result<PlatformOutput, ConversionError> {
        let mut files = Vec::new();
        
        // Create world.json
        let world_json = serde_json::json!({
            "title": campaign.metadata.name,
            "system": campaign.metadata.system,
            "coreVersion": "11.315",
            "systemVersion": "5.3.2",
            "description": "Generated by TTRPGConverter",
            "version": "1.0.0",
            "compatibility": {
                "minimum": "11",
                "verified": "11.315"
            }
        });
        
        files.push(OutputFile {
            path: std::path::PathBuf::from("world.json"),
            content: serde_json::to_string_pretty(&world_json)
                .map_err(|e| ConversionError::SerializationError(e.to_string()))?
                .into_bytes(),
            mime_type: "application/json".to_string(),
        });
        
        // Use database service to create LevelDB files
        let mut db_service = self.shared_services.database_service.lock().await;
        let world_name = &campaign.metadata.name;
        
        // Store actors
        for character in &campaign.characters {
            let foundry_actor = self.convert_character_to_foundry_actor(character).await?;
            db_service.store_entity(world_name, "actors", &character.id, &foundry_actor)?;
        }
        
        // Store scenes  
        for scene in &campaign.scenes {
            let foundry_scene = self.convert_scene_to_foundry_scene(scene).await?;
            db_service.store_entity(world_name, "scenes", &scene.id, &foundry_scene)?;
        }
        
        Ok(PlatformOutput {
            files,
            metadata: self.get_platform_metadata(),
        })
    }
}

#[async_trait]
impl PlatformFormatPlugin for FoundryFormatPlugin {
    fn can_handle_input(&self, path: &Path) -> bool {
        path.is_dir() && path.join("world.json").exists()
    }
    
    fn get_supported_extensions(&self) -> Vec<&str> {
        vec![] // Directories, not files
    }
    
    async fn parse_campaign(&self, path: &Path) -> Result<Campaign, ConversionError> {
        self.parse_foundry_world(path).await
    }
    
    async fn generate_campaign(&self, campaign: &Campaign) -> Result<PlatformOutput, ConversionError> {
        self.generate_foundry_world(campaign).await
    }
    
    fn get_platform_metadata(&self) -> PlatformMetadata {
        PlatformMetadata {
            platform_type: PlatformType::FoundryVTT,
            version: "1.0.0".to_string(),
            supported_systems: vec!["pf2e".to_string(), "dnd5e".to_string()],
        }
    }
}

#[no_mangle]
pub extern "C" fn create_format_plugin(services: SharedServices) -> *mut dyn PlatformFormatPlugin {
    Box::into_raw(Box::new(FoundryFormatPlugin::new(services)))
}
```

---

## **T3.3: Pathbuilder Plugin DLL**
**Duration**: 5 days | **Points**: 6 | **Priority**: 🔥 HIGH

Create unified Pathbuilder plugin for PF2e character imports.

### **Success Criteria**
- [ ] ✅ Roll20.dll handles JSON import/export with full character data
- [ ] ✅ Foundry.dll integrates with LevelDB for world folder operations
- [ ] ✅ Pathbuilder.dll imports PF2e characters with complete data
- [ ] ✅ All plugins use shared services (database, validation, compendium)
- [ ] ✅ Plugin loading/unloading works reliably
- [ ] ✅ Cross-platform conversion (Roll20 ↔ Foundry) functions correctly
