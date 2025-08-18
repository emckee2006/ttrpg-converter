# Foundry Common to TTRPG Core O2O Mapping Guide

## Overview
This document defines the one-to-one mappings between Foundry VTT generated schema types and TTRPG core types, following the modular mapping approach established in the platform crates.

## Actor Mapping

### FoundryActor → TtrpgCharacter

| **Foundry Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|-------------------|----------|-----------------|---------------|-----------|
| `_id` | `String` | `id` | Direct mapping | Foundry document ID |
| `name` | `String` | `name` | Direct mapping | Actor name |
| `img` | `Option<String>` | `image` | Direct mapping | Actor portrait |
| `type` | `String` | `actor_type` | String to enum | "character", "npc", etc. |
| `system.details.biography.value` | `Option<String>` | `biography` | Extract nested | Character backstory |
| `system.attributes.hp.value` | `i32` | `hit_points.current` | Direct mapping | Current HP |
| `system.attributes.hp.max` | `i32` | `hit_points.max` | Direct mapping | Maximum HP |
| `system.attributes.hp.temp` | `Option<i32>` | `hit_points.temp` | Direct mapping | Temporary HP |
| `system.abilities.str.value` | `u8` | `abilities[0].score` | Direct mapping | Strength score |
| `system.abilities.dex.value` | `u8` | `abilities[1].score` | Direct mapping | Dexterity score |
| `system.abilities.con.value` | `u8` | `abilities[2].score` | Direct mapping | Constitution score |
| `system.abilities.int.value` | `u8` | `abilities[3].score` | Direct mapping | Intelligence score |
| `system.abilities.wis.value` | `u8` | `abilities[4].score` | Direct mapping | Wisdom score |
| `system.abilities.cha.value` | `u8` | `abilities[5].score` | Direct mapping | Charisma score |
| `system.details.level` | `Option<u32>` | `level` | Direct mapping | Character level |
| `system.details.xp.value` | `Option<u32>` | `experience_points` | Direct mapping | Experience points |
| `system.attributes.ac.value` | `Option<u8>` | `armor_class` | Direct mapping | Armor class |
| `ownership` | `FoundryDocumentOwnership` | `permissions` | Convert ownership | See ownership mapping |
| `flags` | `HashMap` | `system_data["flags"]` | JSON value | Foundry flags |
| `effects` | `Vec` | `system_data["effects"]` | JSON array | Active effects |

**Default Values:**
- `actor_type`: Parse from `type` field or default to `ActorType::Character`
- `inspiration`: `None` (system-specific)
- `items`: `Vec::new()` (items stored separately)
- `spells`: `Vec::new()` (spells stored separately)
- `skills`: Parse from `system.skills` object

### Foundry Ownership Mapping
```rust
fn convert_foundry_ownership(ownership: FoundryDocumentOwnership) -> Option<Ownership> {
    // Convert Foundry permission levels (0-3) to TTRPG ownership structure
    // 0=None, 1=Limited, 2=Observer, 3=Owner
}
```

## Scene Mapping

### FoundryScene → TtrpgScene

| **Foundry Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|-------------------|----------|-----------------|---------------|-----------|
| `_id` | `String` | `id` | Direct mapping | Scene document ID |
| `name` | `String` | `name` | Direct mapping | Scene name |
| `img` | `Option<String>` | `background` | Direct mapping | Background image |
| `width` | `u32` | `width` | Direct mapping | Scene width in pixels |
| `height` | `u32` | `height` | Direct mapping | Scene height in pixels |
| `padding` | `f32` | `system_data["padding"]` | JSON value | Scene padding |
| `initial.x` | `f32` | `system_data["initial_view_x"]` | JSON value | Initial view X |
| `initial.y` | `f32` | `system_data["initial_view_y"]` | JSON value | Initial view Y |
| `initial.scale` | `f32` | `system_data["initial_scale"]` | JSON value | Initial zoom |
| `backgroundColor` | `Option<String>` | `system_data["background_color"]` | JSON value | Canvas color |
| `grid.type` | `i32` | `grid.grid_type` | Integer to enum | Grid type (0=Square, 1=Hex) |
| `grid.size` | `f32` | `grid.size` | Direct mapping | Grid cell size |
| `grid.distance` | `f32` | `system_data["grid_distance"]` | JSON value | Distance per grid |
| `grid.units` | `String` | `system_data["grid_units"]` | JSON value | Distance units |
| `grid.alpha` | `f32` | `grid.opacity` | Direct mapping | Grid opacity |
| `grid.color` | `String` | `system_data["grid_color"]` | JSON value | Grid color |
| `globalLight` | `bool` | `lighting.global_illumination` | Direct mapping | Global light |
| `darkness` | `f32` | `system_data["darkness_level"]` | JSON value | Darkness level |
| `tokenVision` | `bool` | `lighting.enforce_line_of_sight` | Direct mapping | Token vision |
| `fogExploration` | `bool` | `system_data["fog_exploration"]` | JSON value | Fog of war |
| `active` | `bool` | `system_data["active"]` | JSON value | Active scene |
| `navigation` | `bool` | `system_data["navigation"]` | JSON value | Show in nav |
| `ownership` | `FoundryDocumentOwnership` | `system_data["ownership"]` | JSON value | Scene permissions |

**Default Values:**
- `description`: `None`
- `tokens`: `Vec::new()` (tokens stored separately)
- `lights`: `Vec::new()` (lights stored separately)
- `lighting`: Construct from individual fields

### Grid Type Conversion
```rust
fn foundry_grid_to_ttrpg(grid_type: i32) -> GridType {
    match grid_type {
        0 => GridType::Square,
        1 => GridType::Hexagonal,
        _ => GridType::Square, // Default fallback
    }
}
```

## Item Mapping

### FoundryItem → TtrpgItem

| **Foundry Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|-------------------|----------|-----------------|---------------|-----------|
| `_id` | `String` | `id` | Direct mapping | Item document ID |
| `name` | `String` | `name` | Direct mapping | Item name |
| `img` | `Option<String>` | `image` | Direct mapping | Item image |
| `type` | `String` | `item_type` | String to enum | "weapon", "armor", "consumable", etc. |
| `system.description.value` | `Option<String>` | `description` | Extract nested | Item description |
| `system.quantity` | `u32` | `quantity` | Direct mapping | Item quantity |
| `system.weight` | `Option<f32>` | `weight` | Direct mapping | Item weight |
| `system.price.value` | `Option<f32>` | `cost` | Extract nested | Item cost |
| `system.rarity` | `Option<String>` | `rarity` | String to enum | Item rarity |
| `system.attuned` | `Option<bool>` | `requires_attunement` | Direct mapping | Attunement required |
| `system.damage.parts` | `Vec` | `damage` | Parse damage | Weapon damage |
| `system.armor.value` | `Option<u8>` | `armor_class` | Direct mapping | Armor AC |
| `system.armor.type` | `Option<String>` | `system_data["armor_type"]` | JSON value | Armor category |
| `system.actionType` | `Option<String>` | `system_data["action_type"]` | JSON value | Action type |
| `system.activation` | `Object` | `system_data["activation"]` | JSON value | Activation requirements |
| `system.range` | `Object` | `system_data["range"]` | JSON value | Item range |
| `system.uses` | `Object` | `system_data["uses"]` | JSON value | Item uses |
| `ownership` | `FoundryDocumentOwnership` | `system_data["ownership"]` | JSON value | Item permissions |
| `flags` | `HashMap` | `system_data["flags"]` | JSON value | Module flags |

### Item Type Conversion
```rust
fn foundry_item_type_to_ttrpg(foundry_type: &str) -> ItemType {
    match foundry_type {
        "weapon" => ItemType::Weapon,
        "armor" => ItemType::Armor,
        "shield" => ItemType::Shield,
        "consumable" => ItemType::Consumable,
        "tool" => ItemType::Tool,
        "loot" => ItemType::Treasure,
        "class" | "feat" | "spell" => ItemType::Equipment,
        _ => ItemType::Equipment, // Default fallback
    }
}
```

## Journal Mapping

### FoundryJournal → TtrpgItem (for documents)

| **Foundry Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|-------------------|----------|-----------------|---------------|-----------|
| `_id` | `String` | `id` | Direct mapping | Journal document ID |
| `name` | `String` | `name` | Direct mapping | Journal name |
| `pages` | `Vec<JournalPage>` | `description` | Concatenate pages | All page content |
| `ownership` | `FoundryDocumentOwnership` | `system_data["ownership"]` | JSON value | Journal permissions |
| `flags` | `HashMap` | `system_data["flags"]` | JSON value | Module flags |

**Default Values:**
- `item_type`: `ItemType::Equipment` (treat as document)
- `quantity`: `1`
- `weight`: `None`
- `cost`: `None`
- `rarity`: `None`
- `image`: First page image or `None`
- `damage`: `None`
- `armor_class`: `None`
- `requires_attunement`: `Some(false)`

## World Mapping

### FoundryWorld → TtrpgCampaign

| **Foundry Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|-------------------|----------|-----------------|---------------|-----------|
| `id` | `String` | `id` | Direct mapping | World ID |
| `title` | `String` | `name` | Direct mapping | World title |
| `description` | `Option<String>` | `description` | Direct mapping | World description |
| `system` | `String` | `system_data["game_system"]` | JSON value | Game system ID |
| `coreVersion` | `String` | `system_data["core_version"]` | JSON value | Foundry version |
| `systemVersion` | `String` | `system_data["system_version"]` | JSON value | System version |
| `lastPlayed` | `Option<String>` | `system_data["last_played"]` | JSON value | Last session |
| `playtime` | `Option<u64>` | `system_data["playtime"]` | JSON value | Total playtime |
| `version` | `String` | `system_data["world_version"]` | JSON value | World version |
| `compatibility` | `Object` | `system_data["compatibility"]` | JSON value | Compatibility info |

**Default Values:**
- `game_master`: `None` (extract from user data if available)
- `players`: `Vec::new()` (extract from user data)
- `sessions`: `Vec::new()` (not stored in world data)

## Implementation Structure

```rust
// src/mappings/actor.rs
impl From<FoundryActor> for TtrpgCharacter {
    fn from(actor: FoundryActor) -> Self {
        // Implementation with nested system data extraction
    }
}

// src/mappings/scene.rs
impl From<FoundryScene> for TtrpgScene {
    fn from(scene: FoundryScene) -> Self {
        // Implementation with grid and lighting mapping
    }
}

// src/mappings/item.rs
impl From<FoundryItem> for TtrpgItem {
    fn from(item: FoundryItem) -> Self {
        // Implementation with system-specific data handling
    }
}

// src/mappings/journal.rs
impl From<FoundryJournal> for TtrpgItem {
    fn from(journal: FoundryJournal) -> Self {
        // Implementation treating journals as document items
    }
}

// src/mappings/world.rs
impl From<FoundryWorld> for TtrpgCampaign {
    fn from(world: FoundryWorld) -> Self {
        // Implementation with world metadata
    }
}
```

## Helper Functions

### Nested Data Extraction
```rust
fn extract_nested_string(data: &serde_json::Value, path: &str) -> Option<String> {
    // Navigate nested JSON paths like "system.details.biography.value"
}

fn extract_nested_number<T>(data: &serde_json::Value, path: &str) -> Option<T> 
where T: serde::de::DeserializeOwned {
    // Extract numeric values from nested paths
}
```

### Permission Conversion  
```rust
fn convert_foundry_permissions(ownership: &FoundryDocumentOwnership) -> Option<Ownership> {
    // Convert Foundry's numeric permission system to TTRPG ownership
}

fn get_owner_from_permissions(ownership: &FoundryDocumentOwnership) -> Option<String> {
    // Find the user with highest permissions (level 3)
}
```

### Ability Score Processing
```rust
fn extract_ability_scores(system_data: &serde_json::Value) -> Vec<Ability> {
    // Parse Foundry's ability score structure to TTRPG abilities
}

fn calculate_modifier_from_score(score: u8) -> i8 {
    // Standard D&D ability modifier calculation
}
```

## Error Handling

Foundry data is highly structured but can have system-specific variations:

1. **Nested Access**: Use safe JSON navigation for deeply nested system data
2. **Type Variants**: Different game systems store data in different structures
3. **Missing Fields**: Many system fields are optional and need defaults
4. **Permission Levels**: Foundry uses numeric permission levels (0-3)
5. **Module Data**: Flags object can contain arbitrary module-specific data
6. **Document References**: IDs reference other documents that may not be loaded

## System Data Strategy

Store Foundry-specific fields that don't map to core TTRPG types:
- Document flags and module data
- Foundry-specific UI settings (navigation, active status)
- Advanced lighting and vision settings
- Grid distance units and measurements
- Document creation/modification timestamps
- Compatibility and version information

This preserves Foundry's rich feature set while maintaining clean core type compatibility.
