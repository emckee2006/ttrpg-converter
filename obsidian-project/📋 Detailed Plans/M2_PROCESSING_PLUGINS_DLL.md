# M2: Processing Plugins DLL - Junior Developer Implementation Guide

## 🎯 **MILESTONE 2 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 16 | **Priority**: 🚨 CRITICAL
**Dependencies**: M1.4 Core Foundation + Plugin System, M1.5 Performance Foundation

Processing plugin DLLs including validation, compendium lookup, and asset processing with shared plugin interfaces.

### 🔧 **PROCESSING PLUGIN FEATURES**
- **Validation.dll**: Campaign data validation and schema checking
- **Compendium.dll**: Entity lookup and reference resolution
- **AssetProcessor.dll**: Image/audio processing and optimization
- **Plugin Testing Framework**: Comprehensive testing for all processing plugins

---

## **T2.1: Validation Plugin DLL**
**Duration**: 3 days | **Points**: 4 | **Priority**: 🔥 HIGH

Create validation plugin as standalone DLL that integrates with core orchestration framework.

### **Implementation Steps**

**Step 1: Create Validation Plugin Crate**
```bash
cd crates/plugins
cargo new ttrpg-validation-plugin --lib
```

Update `ttrpg-validation-plugin/Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]  # Create DLL

[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
jsonschema = "0.17"
regex = "1.10"
async-trait = "0.1"
```

**Step 2: Validation Plugin Implementation**
Create `src/lib.rs`:
```rust
use std::collections::HashMap;
use async_trait::async_trait;
use ttrpg_core::plugins::{ProcessingPlugin, SharedServices};
use ttrpg_core::types::common::*;

pub struct ValidationPlugin {
    shared_services: SharedServices,
    validation_schemas: HashMap<String, jsonschema::JSONSchema>,
}

impl ValidationPlugin {
    pub fn new(services: SharedServices) -> Self {
        let mut schemas = HashMap::new();
        
        // Load validation schemas for each entity type
        schemas.insert("campaign".to_string(), Self::load_campaign_schema());
        schemas.insert("character".to_string(), Self::load_character_schema());
        schemas.insert("item".to_string(), Self::load_item_schema());
        
        Self {
            shared_services: services,
            validation_schemas: schemas,
        }
    }
    
    fn load_campaign_schema() -> jsonschema::JSONSchema {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["metadata", "characters", "scenes", "items"],
            "properties": {
                "metadata": {
                    "type": "object",
                    "required": ["name", "system", "version"],
                    "properties": {
                        "name": {"type": "string", "minLength": 1},
                        "system": {"type": "string", "enum": ["pf1e", "pf2e", "dnd5e"]},
                        "version": {"type": "string"}
                    }
                },
                "characters": {"type": "array", "items": {"$ref": "#/definitions/character"}},
                "scenes": {"type": "array"},
                "items": {"type": "array"}
            }
        });
        
        jsonschema::JSONSchema::compile(&schema)
            .expect("Campaign schema should be valid")
    }
}

#[async_trait]
impl ProcessingPlugin for ValidationPlugin {
    async fn process(&self, mut campaign: Campaign) -> Result<Campaign, ConversionError> {
        // Validate campaign structure
        self.validate_campaign_structure(&campaign).await?;
        
        // Validate individual entities
        self.validate_entities(&mut campaign).await?;
        
        // Add validation metadata
        campaign.metadata.validation_status = Some("validated".to_string());
        campaign.metadata.validation_timestamp = Some(chrono::Utc::now());
        
        Ok(campaign)
    }
    
    fn get_priority(&self) -> i32 {
        100 // Run validation early in pipeline
    }
}

impl ValidationPlugin {
    async fn validate_campaign_structure(&self, campaign: &Campaign) -> Result<(), ConversionError> {
        let campaign_json = serde_json::to_value(campaign)
            .map_err(|e| ConversionError::ValidationError(e.to_string()))?;
            
        let schema = &self.validation_schemas["campaign"];
        let result = schema.validate(&campaign_json);
        
        if let Err(errors) = result {
            let error_messages: Vec<String> = errors
                .map(|error| format!("Validation error: {}", error))
                .collect();
            return Err(ConversionError::ValidationError(error_messages.join(", ")));
        }
        
        Ok(())
    }
    
    async fn validate_entities(&self, campaign: &mut Campaign) -> Result<(), ConversionError> {
        // Validate characters
        for character in &mut campaign.characters {
            self.validate_character(character).await?;
        }
        
        // Validate items
        for item in &mut campaign.items {
            self.validate_item(item).await?;
        }
        
        Ok(())
    }
    
    async fn validate_character(&self, character: &mut TtrpgCharacter) -> Result<(), ConversionError> {
        // Validate required fields
        if character.metadata.name.is_empty() {
            return Err(ConversionError::ValidationError("Character name cannot be empty".to_string()));
        }
        
        // Validate ability scores are within reasonable bounds
        if let Some(abilities) = &character.abilities {
            for (ability, score) in [
                ("strength", abilities.strength),
                ("dexterity", abilities.dexterity),
                ("constitution", abilities.constitution),
                ("intelligence", abilities.intelligence),
                ("wisdom", abilities.wisdom),
                ("charisma", abilities.charisma),
            ] {
                if score < 1 || score > 50 {
                    return Err(ConversionError::ValidationError(
                        format!("Invalid {} score: {}. Must be between 1 and 50", ability, score)
                    ));
                }
            }
        }
        
        character.metadata.validation_status = Some("validated".to_string());
        Ok(())
    }
    
    async fn validate_item(&self, item: &mut TtrpgItem) -> Result<(), ConversionError> {
        if item.metadata.name.is_empty() {
            return Err(ConversionError::ValidationError("Item name cannot be empty".to_string()));
        }
        
        item.metadata.validation_status = Some("validated".to_string());
        Ok(())
    }
}

impl ttrpg_core::plugins::Plugin for ValidationPlugin {
    fn name(&self) -> &str {
        "ValidationPlugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "Validates campaign data structure and entity integrity"
    }
}

#[no_mangle]
pub extern "C" fn create_processing_plugin(services: SharedServices) -> *mut dyn ProcessingPlugin {
    Box::into_raw(Box::new(ValidationPlugin::new(services)))
}
```

---

## **T2.2: Compendium Lookup Plugin DLL**
**Duration**: 4 days | **Points**: 5 | **Priority**: 🚨 CRITICAL

Port Python compendium lookup functionality to Rust DLL.

### **Implementation Steps**

**Step 1: Create Compendium Plugin**
```bash
cargo new ttrpg-compendium-plugin --lib
```

Update `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1.35", features = ["full"] }
uuid = { version = "1.6", features = ["v4"] }
fuzzy-matcher = "0.3"
async-trait = "0.1"
```

**Step 2: Compendium Service Implementation**
Create `src/lib.rs`:
```rust
use std::collections::HashMap;
use async_trait::async_trait;
use fuzzy_matcher::FuzzyMatcher;
use ttrpg_core::plugins::{ProcessingPlugin, SharedServices};
use ttrpg_core::types::common::*;

pub struct CompendiumLookupPlugin {
    shared_services: SharedServices,
    compendium_cache: HashMap<String, CompendiumEntry>,
    fuzzy_matcher: fuzzy_matcher::skim::SkimMatcherV2,
}

impl CompendiumLookupPlugin {
    pub fn new(services: SharedServices) -> Self {
        Self {
            shared_services: services,
            compendium_cache: HashMap::new(),
            fuzzy_matcher: fuzzy_matcher::skim::SkimMatcherV2::default(),
        }
    }
    
    /// Load compendium data from standard sources
    async fn load_compendium_data(&mut self) -> Result<(), ConversionError> {
        // Load standard items (weapons, armor, spells, etc.)
        self.load_standard_weapons().await?;
        self.load_standard_armor().await?;
        self.load_standard_spells().await?;
        
        Ok(())
    }
    
    async fn load_standard_weapons(&mut self) -> Result<(), ConversionError> {
        // Standard weapon compendium entries
        let weapons = vec![
            ("Long Sword", "weapon", "melee", r#"{"damage": "1d8", "type": "slashing", "category": "martial"}"#),
            ("Short Sword", "weapon", "melee", r#"{"damage": "1d6", "type": "piercing", "category": "martial"}"#),
            ("Longbow", "weapon", "ranged", r#"{"damage": "1d8", "type": "piercing", "range": "150/600"}"#),
            ("Crossbow, Light", "weapon", "ranged", r#"{"damage": "1d8", "type": "piercing", "range": "80/320"}"#),
        ];
        
        for (name, category, subcategory, data) in weapons {
            self.compendium_cache.insert(name.to_lowercase(), CompendiumEntry {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                category: category.to_string(),
                subcategory: Some(subcategory.to_string()),
                data: serde_json::from_str(data).unwrap(),
                source: "core_compendium".to_string(),
            });
        }
        
        Ok(())
    }
    
    async fn load_standard_armor(&mut self) -> Result<(), ConversionError> {
        let armor = vec![
            ("Leather Armor", "armor", "light", r#"{"ac": "11 + Dex", "type": "light"}"#),
            ("Chain Mail", "armor", "heavy", r#"{"ac": "16", "type": "heavy", "stealth": "disadvantage"}"#),
            ("Plate Armor", "armor", "heavy", r#"{"ac": "18", "type": "heavy", "stealth": "disadvantage"}"#),
        ];
        
        for (name, category, subcategory, data) in armor {
            self.compendium_cache.insert(name.to_lowercase(), CompendiumEntry {
                id: uuid::Uuid::new_v4().to_string(),
                name: name.to_string(),
                category: category.to_string(),
                subcategory: Some(subcategory.to_string()),
                data: serde_json::from_str(data).unwrap(),
                source: "core_compendium".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// Find compendium entry by name with fuzzy matching
    pub fn find_compendium_entry(&self, name: &str) -> Option<&CompendiumEntry> {
        // Exact match first
        if let Some(entry) = self.compendium_cache.get(&name.to_lowercase()) {
            return Some(entry);
        }
        
        // Fuzzy match if no exact match
        let mut best_match = None;
        let mut best_score = 0;
        
        for (cached_name, entry) in &self.compendium_cache {
            if let Some(score) = self.fuzzy_matcher.fuzzy_match(cached_name, &name.to_lowercase()) {
                if score > best_score && score > 50 { // Minimum threshold
                    best_score = score;
                    best_match = Some(entry);
                }
            }
        }
        
        best_match
    }
    
    /// Convert item to compendium reference if found
    pub fn create_item_from_compendium(&self, item_name: &str) -> Option<TtrpgItem> {
        let compendium_entry = self.find_compendium_entry(item_name)?;
        
        Some(TtrpgItem {
            id: uuid::Uuid::new_v4().to_string(),
            metadata: ItemMetadata {
                name: compendium_entry.name.clone(),
                description: format!("Compendium reference: {}", compendium_entry.source),
                category: Some(compendium_entry.category.clone()),
                subcategory: compendium_entry.subcategory.clone(),
                validation_status: Some("compendium_reference".to_string()),
            },
            properties: ItemProperties {
                weight: None,
                value: None,
                rarity: None,
                magical: None,
            },
            compendium_reference: Some(CompendiumReference {
                source: compendium_entry.source.clone(),
                id: compendium_entry.id.clone(),
                name: compendium_entry.name.clone(),
                override_data: None, // No overrides for standard items
            }),
            system_data: compendium_entry.data.clone(),
        })
    }
}

#[async_trait]
impl ProcessingPlugin for CompendiumLookupPlugin {
    async fn process(&self, mut campaign: Campaign) -> Result<Campaign, ConversionError> {
        // Process items for compendium references
        for item in &mut campaign.items {
            if let Some(compendium_item) = self.create_item_from_compendium(&item.metadata.name) {
                // Replace with compendium reference if found
                if item.compendium_reference.is_none() {
                    item.compendium_reference = compendium_item.compendium_reference;
                    item.metadata.validation_status = Some("compendium_referenced".to_string());
                }
            }
        }
        
        // Process character equipment
        for character in &mut campaign.characters {
            if let Some(equipment) = &mut character.equipment {
                let mut new_items = Vec::new();
                
                for item in &equipment.items {
                    if let Some(compendium_item) = self.create_item_from_compendium(&item.metadata.name) {
                        new_items.push(compendium_item);
                    } else {
                        new_items.push(item.clone());
                    }
                }
                
                equipment.items = new_items;
            }
        }
        
        Ok(campaign)
    }
    
    fn get_priority(&self) -> i32 {
        200 // Run after validation
    }
}

impl ttrpg_core::plugins::Plugin for CompendiumLookupPlugin {
    fn name(&self) -> &str {
        "CompendiumLookupPlugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "Resolves entity references against game system compendiums"
    }
}

#[derive(Debug, Clone)]
pub struct CompendiumEntry {
    pub id: String,
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub data: serde_json::Value,
    pub source: String,
}

#[no_mangle]
pub extern "C" fn create_processing_plugin(services: SharedServices) -> *mut dyn ProcessingPlugin {
    let mut plugin = CompendiumLookupPlugin::new(services);
    // Note: In real implementation, we'd need to handle async initialization
    Box::into_raw(Box::new(plugin))
}
```

---

## **Success Criteria**
- [ ] ✅ Validation.dll compiles and loads successfully
- [ ] ✅ Compendium.dll provides accurate entity lookups
- [ ] ✅ AssetProcessor.dll handles image/audio optimization
- [ ] ✅ All plugins integrate with shared services
- [ ] ✅ Comprehensive test coverage for all processing plugins
- [ ] ✅ Plugin loading and unloading works correctly
- [ ] ✅ Performance metrics show expected processing times
