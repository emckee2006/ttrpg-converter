# Roll20 to TTRPG Core O2O Mapping Guide

## Overview
This document defines the one-to-one mappings between Roll20 generated schema types and TTRPG core types, following the modular mapping approach established in the Pathbuilder PF2e crate.

## Character Mapping

### Roll20CharacterSheet → TtrpgCharacter

| **Roll20 Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|------------------|----------|-----------------|---------------|-----------|
| `id` | `Roll20CharacterSheetId` | `id` | `.to_string()` | Extract string from wrapper |
| `name` | `Roll20CharacterSheetName` | `name` | `.to_string()` | Extract string from wrapper |
| `bio` | `Option<String>` | `biography` | Direct mapping | Character description |
| `avatar` | `Option<String>` | `image` | Direct mapping | Character portrait |
| `controlledby` | `Option<String>` | `permissions.owner_id` | Parse CSV string | First player ID |
| `archived` | `bool` | `system_data["archived"]` | JSON value | Roll20 specific |
| `gmnotes` | `Option<String>` | `notes` | Direct mapping | GM notes |
| `abilities` | `Vec<Roll20CharacterAbility>` | `system_data["abilities"]` | JSON array | Roll20 macros |
| `attribs` | `Vec<Roll20CharacterAttribute>` | Parse to character fields | Custom logic | See attribute mapping |

**Default Values:**
- `actor_type`: `Some(ActorType::Character)`
- `level`: Extract from attributes or default to `Some(1)`
- `experience_points`: Extract from attributes or `None`
- `inspiration`: `None`
- `hit_points`: Parse from HP attributes
- `abilities`: Parse from Roll20 attributes
- `skills`: Parse from Roll20 attributes  
- `items`: `Vec::new()` (items stored separately)
- `spells`: `Vec::new()` (spells stored separately)
- `armor_class`: Extract from AC attribute or `None`

### Roll20CharacterAttribute Processing

Roll20 stores all character data as string attributes. Common attribute patterns:

| **Attribute Name** | **TTRPG Mapping** | **Transform** |
|-------------------|-------------------|---------------|
| `"strength"`, `"str"` | `abilities[0].score` | Parse string to u8 |
| `"dexterity"`, `"dex"` | `abilities[1].score` | Parse string to u8 |
| `"constitution"`, `"con"` | `abilities[2].score` | Parse string to u8 |
| `"intelligence"`, `"int"` | `abilities[3].score` | Parse string to u8 |
| `"wisdom"`, `"wis"` | `abilities[4].score` | Parse string to u8 |
| `"charisma"`, `"cha"` | `abilities[5].score` | Parse string to u8 |
| `"hp"`, `"hit_points"` | `hit_points.max` | Parse string to i32 |
| `"hp_current"` | `hit_points.current` | Parse string to i32 |
| `"ac"`, `"armor_class"` | `armor_class` | Parse string to u8 |
| `"level"` | `level` | Parse string to u32 |
| `"experience"`, `"xp"` | `experience_points` | Parse string to u32 |

## Scene Mapping

### Page → TtrpgScene

| **Roll20 Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|------------------|----------|-----------------|---------------|-----------|
| `id` | `PageId` | `id` | `.to_string()` | Extract string from wrapper |
| `name` | `PageName` | `name` | `.to_string()` | Extract string from wrapper |
| `archived` | `Option<bool>` | `system_data["archived"]` | JSON value | Roll20 specific |
| `background_color` | `Option<String>` | `background.color` | Direct mapping | Scene background |
| `width` | `Option<f64>` | `dimensions.width` | Direct mapping | Scene width |
| `height` | `Option<f64>` | `dimensions.height` | Direct mapping | Scene height |
| `grid_size` | `Option<f64>` | `grid.size` | Direct mapping | Grid cell size |
| `grid_opacity` | `Option<f64>` | `grid.opacity` | Direct mapping | Grid transparency |
| `snapping_increment` | `Option<f64>` | `grid.snap` | Direct mapping | Grid snapping |
| `fog_opacity` | `Option<f64>` | `system_data["fog_opacity"]` | JSON value | Roll20 fog |
| `lightenforcelos` | `Option<bool>` | `lighting.enforce_los` | Direct mapping | Line of sight |
| `lightglobalillum` | `Option<bool>` | `lighting.global_illumination` | Direct mapping | Global light |
| `lightrestrictmovement` | `Option<bool>` | `system_data["light_restrict_movement"]` | JSON value | Roll20 specific |
| `lightupdatedrop` | `Option<bool>` | `system_data["light_update_drop"]` | JSON value | Roll20 specific |

**Default Values:**
- `description`: `None`
- `tokens`: `Vec::new()` (tokens stored separately)
- `lights`: `Vec::new()` (lights stored separately)
- `system_data`: Populate with Roll20-specific fields

## Handout Mapping

### Handout → TtrpgItem (for documents/notes)

| **Roll20 Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|------------------|----------|-----------------|---------------|-----------|
| `id` | `HandoutId` | `id` | `.to_string()` | Extract string from wrapper |
| `name` | `HandoutName` | `name` | `.to_string()` | Extract string from wrapper |
| `notes` | `Option<String>` | `description` | Direct mapping | Handout content |
| `archived` | `Option<bool>` | `system_data["archived"]` | JSON value | Roll20 specific |
| `controlledby` | `Option<String>` | `system_data["controlled_by"]` | JSON value | Access control |
| `inplayerjournals` | `Option<String>` | `system_data["in_player_journals"]` | JSON value | Visibility |

**Default Values:**
- `item_type`: `ItemType::Equipment` (or create `ItemType::Document`)
- `quantity`: `1`
- `weight`: `None`
- `cost`: `None`
- `rarity`: `None`
- `image`: `None`
- `damage`: `None`
- `armor_class`: `None`
- `requires_attunement`: `Some(false)`

## Campaign Mapping

### Campaign → TtrpgCampaign

| **Roll20 Field** | **Type** | **TTRPG Field** | **Transform** | **Notes** |
|------------------|----------|-----------------|---------------|-----------|
| `id` | `CampaignId` | `id` | `.to_string()` | Extract string from wrapper |
| `name` | `CampaignName` | `name` | `.to_string()` | Extract string from wrapper |
| `description` | `Option<String>` | `description` | Direct mapping | Campaign summary |
| `created_date` | `Option<String>` | `system_data["created_date"]` | JSON value | Creation timestamp |
| `last_played` | `Option<String>` | `system_data["last_played"]` | JSON value | Last session |

## Implementation Structure

```rust
// src/mappings/character.rs
impl From<Roll20CharacterSheet> for TtrpgCharacter {
    fn from(character: Roll20CharacterSheet) -> Self {
        // Implementation with attribute parsing logic
    }
}

// src/mappings/scene.rs  
impl From<Page> for TtrpgScene {
    fn from(page: Page) -> Self {
        // Implementation with Roll20 scene specifics
    }
}

// src/mappings/item.rs
impl From<Handout> for TtrpgItem {
    fn from(handout: Handout) -> Self {
        // Implementation treating handouts as document items
    }
}

// src/mappings/campaign.rs
impl From<Campaign> for TtrpgCampaign {
    fn from(campaign: Campaign) -> Self {
        // Implementation with Roll20 campaign data
    }
}
```

## Helper Functions

### Attribute Parsing
```rust
fn parse_attributes_to_abilities(attribs: &[Roll20CharacterAttribute]) -> Vec<Ability> {
    // Parse Roll20 string attributes into ability scores
}

fn parse_hit_points_from_attributes(attribs: &[Roll20CharacterAttribute]) -> Option<HitPoints> {
    // Find HP-related attributes and create HitPoints struct
}

fn find_attribute_value(attribs: &[Roll20CharacterAttribute], name: &str) -> Option<String> {
    // Helper to find attribute by name
}
```

### String Parsing
```rust
fn parse_string_to_u8(value: &str) -> Option<u8> {
    // Parse Roll20 string values to numbers with error handling
}

fn parse_csv_to_vec(value: &str) -> Vec<String> {
    // Parse comma-separated values (controlledby, inplayerjournals)
}
```

## Error Handling

Roll20 data can be inconsistent. Implement defensive parsing:

1. **String Conversion**: All Roll20 wrapper types need `.to_string()`
2. **Attribute Parsing**: String attributes may be empty or invalid numbers
3. **Missing Data**: Many optional fields may be `None`
4. **CSV Fields**: `controlledby` and `inplayerjournals` are CSV strings that need splitting
5. **Default Fallbacks**: Provide sensible defaults when data is missing

## System Data Strategy

Store Roll20-specific fields that don't map to core TTRPG types:
- `archived` status
- Light settings (enforcement, global illumination)
- Access control strings
- Roll20 macro/ability definitions
- Original attribute structure for debugging

This preserves Roll20-specific functionality while maintaining clean core type compatibility.
