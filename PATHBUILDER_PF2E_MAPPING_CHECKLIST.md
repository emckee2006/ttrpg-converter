# Pathbuilder PF2e Mapping Implementation Checklist

**Purpose**: Systematic implementation of Pathbuilder PF2e schema-to-internal type mappings
**Platform**: `ttrpg-pathbuilder-pf2e` crate only
**Status**: Based on actual generated schema structure from index.html and ttrpg-types validation
**Schema Location**: `crates/platforms/ttrpg-pathbuilder-pf2e/schemas/pathbuilder/*.json`

## Actual Generated Schema Structure (from index.html)

### **Tuple Struct Types**
```rust
// From content_pack module
CustomAncestryId(::std::string::String);
CustomAncestryName(::std::string::String);
CustomAncestryDescription(::std::string::String);
CustomAncestrySize(i64);
CustomAncestryTimestamp(::std::string::String);

CustomBackgroundId(::std::string::String);
CustomBackgroundName(::std::string::String);
CustomBackgroundDescription(::std::string::String);
CustomBackgroundTimestamp(::std::string::String);

CustomItemId(::std::string::String);
CustomItemName(::std::string::String);
CustomItemDescription(::std::string::String);
CustomItemTimestamp(::std::string::String);

CustomSpellId(::std::string::String);
CustomSpellName(::std::string::String);
CustomSpellDescription(::std::string::String);
CustomSpellTimestamp(::std::string::String);

CustomFeatId(::std::string::String);
CustomFeatName(::std::string::String);
CustomFeatDescription(::std::string::String);
CustomFeatTimestamp(::std::string::String);

CustomHeritageId(::std::string::String);
CustomHeritageName(::std::string::String);
CustomHeritageDescription(::std::string::String);
CustomHeritageTimestamp(::std::string::String);
```

### **Field Access Patterns**
- **String Conversion**: `String::from(schema_field)` to convert tuple structs to strings
- **Deref Access**: Schema types implement `Deref<Target = String>` for string access
- **No Option Wrapping**: Schema fields are direct values, not `Option<T>`
- **String Fields**: Use `String::from(field)` for conversion
- **Integer Fields**: Direct access (e.g., `spell.level` as `i64`)

## Target Types Validation (ttrpg-types crate)

### **✅ TtrpgCharacter (character.rs)**
```rust
pub struct TtrpgCharacter {
    pub id: String,                          // REQUIRED
    pub name: String,                        // REQUIRED  
    pub biography: Option<String>,
    pub level: Option<u32>,
    pub actor_type: Option<ActorType>,
    pub image: Option<String>,
    pub permissions: Option<Ownership>,
    pub experience_points: Option<u32>,
    pub inspiration: Option<bool>,
    pub notes: Option<String>,
    pub system_data: HashMap<String, serde_json::Value>,
    // ... other fields
}
```

### **✅ TtrpgItem (item.rs)**
```rust
pub struct TtrpgItem {
    pub id: String,                          // REQUIRED
    pub name: String,                        // REQUIRED
    pub description: Option<String>,
    pub item_type: ItemType,                 // REQUIRED enum
    pub quantity: u32,                       // REQUIRED
    pub weight: Option<f64>,
    pub cost: Option<Currency>,              // Currency { value: f64, denomination: String }
    pub damage: Option<DamageInfo>,          // From crate::damage
    pub armor_class: Option<ArmorClass>,     // From crate::damage
    pub requires_attunement: Option<bool>,
    pub system_data: HashMap<String, serde_json::Value>,
    // ... other fields
}
```

### **✅ TtrpgSpell (spell.rs)**
```rust
pub struct TtrpgSpell {
    pub id: String,                          // REQUIRED
    pub name: String,                        // REQUIRED
    pub description: String,                 // REQUIRED (not Option!)
    pub level: u8,                          // REQUIRED (not Option!)
    pub school: SpellSchool,                // REQUIRED enum
    pub casting_time: Option<String>,
    pub range: Option<String>,
    pub components: Option<SpellComponents>, // SpellComponents { verbal: bool, somatic: bool, material: bool, material_description: Option<String> }
    pub duration: Option<String>,
    pub ritual: Option<bool>,
    pub concentration: Option<bool>,
    pub system_data: HashMap<String, serde_json::Value>,
    // ... other fields
}
```

## Implementation Mappings

### **1. CustomAncestry → TtrpgCharacter**

**Schema Import**: `use crate::schemas::content_pack::CustomAncestry;`

**Field Mappings**:
```rust
impl From<CustomAncestry> for TtrpgCharacter {
    fn from(ancestry: CustomAncestry) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("size".to_string(), serde_json::json!(ancestry.size)); // i64
        system_data.insert("timestamp".to_string(), serde_json::json!(ancestry.timestamp));
        
        Self {
            id: String::from(ancestry.id),        // Use From trait
            name: String::from(ancestry.name),    // Use From trait  
            biography: Some(String::from(ancestry.description)), // Use From trait
            level: Some(1),                       // Default level for ancestry
            actor_type: Some(ActorType::Character),
            // Set defaults for remaining fields
            image: None,
            permissions: None,
            experience_points: None,
            inspiration: None,
            notes: None,
            // ... other fields with defaults
        }
    }
}
```

### **2. CustomBackground → TtrpgCharacter**

**Schema Import**: `use crate::schemas::content_pack::CustomBackground;`

**Field Mappings**:
```rust
impl From<CustomBackground> for TtrpgCharacter {
    fn from(background: CustomBackground) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("timestamp".to_string(), serde_json::json!(background.timestamp));
        
        Self {
            id: String::from(background.id),      // Use From trait
            name: String::from(background.name),  // Use From trait
            biography: Some(String::from(background.description)), // Use From trait
            level: Some(1),                       // Default level
            actor_type: Some(ActorType::Character),
            system_data,
            // Set defaults for remaining fields
            // ... other fields with defaults
        }
    }
}
```

### **3. CustomItem → TtrpgItem**

**Schema Import**: `use crate::schemas::content_pack::CustomItem;`

**Field Mappings**:
```rust
impl From<CustomItem> for TtrpgItem {
    fn from(item: CustomItem) -> Self {
        Self {
            id: item.id.0,                        // CustomItemId.0 → String
            name: item.name.0,                    // CustomItemName.0 → String
            description: Some(item.description.0), // CustomItemDescription.0 → Option<String>
            item_type: ItemType::Equipment,       // Default - parse from category field
            quantity: 1,                          // Default quantity
            weight: None,                         // Parse from bulk field
            cost: None,                           // Parse from price field
            damage: None,                         // Parse from damage field
            armor_class: None,                    // Parse from ac field
            requires_attunement: Some(false),     // Default
            system_data: {
                let mut data = HashMap::new();
                data.insert("timestamp".to_string(), serde_json::json!(item.timestamp.0));
                // Add item-specific fields from JSON schema
                data
            },
            // ... other fields with defaults
        }
    }
}
```

### **4. CustomSpell → TtrpgSpell**

**Schema Import**: `use crate::schemas::content_pack::CustomSpell;`

**Critical**: `description` and `level` are REQUIRED fields in TtrpgSpell!

**Field Mappings**:
```rust
impl From<CustomSpell> for TtrpgSpell {
    fn from(spell: CustomSpell) -> Self {
        Self {
            id: spell.id.0,                      // CustomSpellId.0 → String
            name: spell.name.0,                  // CustomSpellName.0 → String
            description: spell.description.0,   // CustomSpellDescription.0 → String (REQUIRED!)
            level: spell.level as u8,            // i64 → u8 cast (REQUIRED!)
            school: SpellSchool::Evocation,      // Default - parse from traits field
            casting_time: None,                  // Parse from cast field
            range: None,                         // Parse from range field  
            components: None,                    // Parse from components field
            duration: None,                      // Parse from duration field
            ritual: Some(false),                 // PF2e doesn't use ritual spells
            concentration: Some(false),          // PF2e uses sustained spells instead
            system_data: {
                let mut data = HashMap::new();
                data.insert("timestamp".to_string(), serde_json::json!(spell.timestamp.0));
                // Add spell-specific fields from JSON schema
                data
            },
            // ... other fields with defaults
        }
    }
}
```

## Helper Functions Needed

### **String Parsing Helpers**
- `parse_item_category(category: &str) -> ItemType`
- `parse_spell_school(traits: &str) -> SpellSchool`
- `parse_spell_components(components: &str) -> SpellComponents`
- `parse_damage_info(damage: &str, damage_type: &str) -> Option<DamageInfo>`
- `parse_armor_class(ac: i32, dex_cap: Option<i32>) -> Option<ArmorClass>`
- `parse_currency(price: &str) -> Option<Currency>`
- `bulk_to_weight(bulk: &str) -> Option<f64>`

### **Enum Conversion Helpers**
- `damage_type_from_string(damage_type: &str) -> DamageType`
- `ability_from_string(ability: &str) -> AbilityType`

## Implementation Priority

1. **High Priority**: CustomAncestry, CustomBackground, CustomItem, CustomSpell → Core internal types
2. **Medium Priority**: CustomFeat, CustomHeritage → Extended character features
3. **Low Priority**: Standard ancestry/background/item/spell → Official content

## Notes

- All schema fields should be accessed via `.0` tuple extraction
- Required target fields need explicit defaults when schema doesn't provide values
- Use `system_data` HashMap for platform-specific fields that don't map to internal types
- Handle type mismatches (i64 → u8, String parsing) carefully
- All timestamp fields should be stored in `system_data` for audit trails
