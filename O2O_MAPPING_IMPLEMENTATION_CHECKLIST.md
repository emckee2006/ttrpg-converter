# O2O Mapping Implementation Checklist

**Purpose**: Systematic implementation of Pathbuilder PF2e schema-to-internal type mappings using o2o derive macros
**Status**: Updated with correct schema types and access patterns
**Coverage**: 12 Pathbuilder schema types → TtrpgTypes with comprehensive field mappings
**Schema Location**: `crates/platforms/ttrpg-pathbuilder-pf2e/schemas/pathbuilder/*.json`
**Generated Types**: Available as tuple struct wrappers (e.g., `CustomAncestryId(String)`)

## Schema Type Access Patterns

### **Generated Schema Type Structure**
All generated schema types follow this pattern:
- **Tuple Structs**: `CustomAncestryId(String)`, `CustomAncestryName(String)`, etc.
- **Optional Fields**: Most schema struct fields are `Option<TupleStruct>`
- **Access Pattern**: `field.map_or_else(String::new, |x| x.0)` for required String fields
- **Access Pattern**: `field.map(|x| x.0)` for optional String fields

## Implementation Priority

### **Phase 1: Pathbuilder Custom Content Mappings (High Priority)**

#### 1. Custom Ancestry → TtrpgCharacter

**Schema Type**: `CustomAncestry` (pathbuilder/custom-ancestry.json)
**Target Type**: `TtrpgCharacter` (ttrpg-types/src/character.rs)

**✅ TYPE VALIDATION AGAINST TTRPG-TYPES CRATE**:
- All target fields confirmed to exist in `TtrpgCharacter` struct
- Field types match expected: `Option<String>`, `Option<u32>`, etc.
- `actor_type` field confirmed as `Option<ActorType>` enum
- `system_data: HashMap<String, serde_json::Value>` confirmed

**Schema Import**: 
```rust
use crate::schemas::custom_ancestry::{CustomAncestry, CustomAncestryId, CustomAncestryName, CustomAncestryDescription, CustomAncestryHp, CustomAncestrySize, CustomAncestrySpeed, CustomAncestryTraits, CustomAncestryCreatedAt, CustomAncestryUpdatedAt};
```
- `CustomAncestryTimestamp(::std::string::String)`

- [x] **Field Mappings**:
  - `id: Option<CustomAncestryId>` → `id: String` via `.map_or_else(String::new, |x| x.0)`
  - `name: Option<CustomAncestryName>` → `name: String` via `.map_or_else(String::new, |x| x.0)`
  - `description: Option<CustomAncestryDescription>` → `biography: Option<String>` via `.map(|x| x.0)`
  - `size: Option<CustomAncestrySize>` → `system_data["size"]` **Note**: `CustomAncestrySize(i64)` not `i32`
  - `timestamp: Option<CustomAncestryTimestamp>` → `system_data["timestamp"]` via `.map(|x| x.0)`
  - Other fields from JSON schema: `hp`, `speed`, `traits`, `boost_ref_1/2`, `flaw_ref`, `languages`, `senses`
  - Set defaults: `actor_type: Some(ActorType::Character)`, `level: Some(1)`

#### 1.2 Custom Background → TtrpgCharacter  
**Schema**: `custom-background.json` → **Generated**: `CustomBackground`
**Target Type**: `TtrpgCharacter` (ttrpg-types/src/character.rs)

**✅ TYPE VALIDATION AGAINST TTRPG-TYPES CRATE**:
- All target fields confirmed to exist in `TtrpgCharacter` struct
- Field types match expected: `Option<String>`, `Option<u32>`, etc.
- `actor_type` field confirmed as `Option<ActorType>` enum
- `system_data: HashMap<String, serde_json::Value>` confirmed

**Import**: `use crate::schemas::custom_background::*;`
**Validated Types from index.html**:
- `CustomBackgroundId(::std::string::String)`
- `CustomBackgroundName(::std::string::String)`
- `CustomBackgroundDescription(::std::string::String)`
- `CustomBackgroundTimestamp(::std::string::String)`

- [x] **Field Mappings**:
  - `id: Option<CustomBackgroundId>` → `id: String` via `.map_or_else(String::new, |x| x.0)`
  - `name: Option<CustomBackgroundName>` → `name: String` via `.map_or_else(String::new, |x| x.0)`
  - `description: Option<CustomBackgroundDescription>` → `biography: Option<String>` via `.map(|x| x.0)`
  - `timestamp: Option<CustomBackgroundTimestamp>` → `system_data["timestamp"]` via `.map(|x| x.0)`
  - Other fields from JSON schema: background-specific fields → `system_data` HashMap
  - Set defaults: `actor_type: Some(ActorType::Character)`, `level: Some(1)`

#### 1.3 Custom Item → TtrpgItem
**Schema**: `custom-item.json` → **Generated**: `CustomItem`
**Target Type**: `TtrpgItem` (ttrpg-types/src/item.rs)

**✅ TYPE VALIDATION AGAINST TTRPG-TYPES CRATE**:
- All target fields confirmed to exist in `TtrpgItem` struct
- `item_type: ItemType` enum variants confirmed: Weapon, Armor, Equipment, Consumable, Tool, Treasure, Spell, Feat
- `cost: Option<Currency>` struct confirmed with `value: f64` and `denomination: String`
- `damage: Option<DamageInfo>` confirmed (uses crate::damage::DamageInfo)
- `armor_class: Option<ArmorClass>` confirmed (uses crate::damage::ArmorClass)
- `system_data: HashMap<String, serde_json::Value>` confirmed

**Import**: `use crate::schemas::custom_item::*;`
**Validated Types from index.html**:
- `CustomItemId(::std::string::String)`
- `CustomItemName(::std::string::String)`
- `CustomItemDescription(::std::string::String)` 
- `CustomItemTimestamp(::std::string::String)`

- [x] **Field Mappings**:
  - `id: Option<CustomItemId>` → `id: String` via `.map_or_else(String::new, |x| x.0)`
  - `name: Option<CustomItemName>` → `name: String` via `.map_or_else(String::new, |x| x.0)`
  - `description: Option<CustomItemDescription>` → `description: Option<String>` via `.map(|x| x.0)`
  - `timestamp: Option<CustomItemTimestamp>` → `system_data["timestamp"]` via `.map(|x| x.0)`
  - Other fields from JSON schema: `level`, `category`, `traits`, `price`, `bulk`, `damage`, etc.
  - `category: Option<String>` → `item_type: ItemType` (enum conversion: weapon, armor, consumable, treasure, tool)
  - `traits: Option<String>` → Parse comma-separated to `rarity` and other properties
  - `price: Option<String>` → `cost: Option<Currency>` (parse "1 gp" format)
  - `bulk: Option<String>` → `weight: Option<f64>` (convert PF2e bulk to weight)
  - `damage: Option<String>` → `damage: Option<DamageInfo>` (parse dice notation)
  - `damageType: Option<String>` → Map to `DamageType` enum
  - `ac: Option<i32>` → `armor_class: Option<ArmorClass>`
  - `dexCap: Option<i32>` → Map to `armor_class.max_dex`
  - `hands: Option<String>` → Parse to two-handed flag
  - Set defaults: `quantity: 1`, `requires_attunement: Some(false)`

#### 1.4 Custom Spell → TtrpgSpell
**Schema**: `custom-spell.json` → **Generated**: `CustomSpell`
**Target Type**: `TtrpgSpell` (ttrpg-types/src/spell.rs)

**✅ TYPE VALIDATION AGAINST TTRPG-TYPES CRATE**:
- All target fields confirmed to exist in `TtrpgSpell` struct
- `school: SpellSchool` enum variants confirmed: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, Cantrip, Focus
- `components: Option<SpellComponents>` struct confirmed with `verbal: bool`, `somatic: bool`, `material: bool`, `material_description: Option<String>`
- `level: u8` confirmed (not Option - required field)
- `description: String` confirmed (not Option - required field)
- `system_data: HashMap<String, serde_json::Value>` confirmed

**Import**: `use crate::schemas::custom_spell::*;`
**Validated Types from index.html**:
- `CustomSpellId(::std::string::String)`
- `CustomSpellName(::std::string::String)`
- `CustomSpellDescription(::std::string::String)`
- `CustomSpellTimestamp(::std::string::String)`

- [x] **Field Mappings**:
  - `id: Option<CustomSpellId>` → `id: String` via `.map_or_else(String::new, |x| x.0)`
  - `name: Option<CustomSpellName>` → `name: String` via `.map_or_else(String::new, |x| x.0)`
  - `description: Option<CustomSpellDescription>` → `description: String` via `.map_or_else(String::new, |x| x.0)`
  - `timestamp: Option<CustomSpellTimestamp>` → `system_data["timestamp"]` via `.map(|x| x.0)`
  - Other fields from JSON schema: `level`, `traits`, `traditions`, `cast`, `components`, etc.
  - `level: Option<i32>` → `level: u8` (**CRITICAL**: Target field is REQUIRED, use `.map_or(0, |x| x.0 as u8)`)
  - `description: Option<CustomSpellDescription>` → `description: String` (**CRITICAL**: Target field is REQUIRED, use `.map_or_else(String::new, |x| x.0)`)
  - `traits: Option<String>` → Parse to `school: SpellSchool` (search for school keywords)
  - `traditions: Option<Vec<String>>` → Map to `system_data["traditions"]`
  - `cast: Option<String>` → `casting_time: Option<String>`
  - `components: Option<String>` → `components: Option<SpellComponents>` (parse "somatic,verbal,material")
  - `range: Option<String>` → `range: Option<String>`
  - `area: Option<String>` → Map to `system_data["area"]`
  - `targets: Option<String>` → Map to `system_data["targets"]`
  - `duration: Option<String>` → `duration: Option<String>`
  - `savingThrow: Option<String>` → `save_type: Option<String>`
  - `heightened: Option<String>` → Map to `system_data["heightened"]`
  - Set defaults: `ritual: Some(false)`, `concentration: Some(false)` (PF2e uses sustained spells)

### **Phase 2: Standard Pathbuilder Content (Medium Priority)**

#### 2.1 Standard Ancestry → TtrpgCharacter (Future)
**Schema**: `ancestry.json` → **Generated**: `Ancestry`
- [ ] Standard PF2e ancestries with official stats
- [ ] Map to same TtrpgCharacter structure as custom ancestry

#### 2.2 Standard Background → TtrpgCharacter (Future)  
**Schema**: `background.json` → **Generated**: `Background`
- [ ] Standard PF2e backgrounds with official stats
- [ ] Map to same TtrpgCharacter structure as custom background

#### 2.3 Standard Item → TtrpgItem (Future)
**Schema**: `item.json` → **Generated**: `Item`  
- [ ] Standard PF2e items with official stats
- [ ] Map to same TtrpgItem structure as custom item

#### 2.4 Standard Spell → TtrpgSpell (Future)
**Schema**: `spell.json` → **Generated**: `Spell`
- [ ] Standard PF2e spells with official stats  
- [ ] Map to same TtrpgSpell structure as custom spell

#### 2.5 Feat Content → TtrpgFeat (Future)
**Schema**: `feat.json`, `custom-feat.json` → **Generated**: `Feat`, `CustomFeat`
- [ ] Create TtrpgFeat type in ttrpg-types
- [ ] Map feat prerequisites, actions, descriptions

#### 2.6 Heritage Content → TtrpgHeritage (Future) 
**Schema**: `custom-heritage.json` → **Generated**: `CustomHeritage`
- [ ] Create TtrpgHeritage type in ttrpg-types
- [ ] Map heritage bonuses and features

### **Phase 3: Content Pack Integration (Lower Priority)**

#### 3.1 Content Pack Management
**Schema**: `content_pack.json` → **Generated**: `PathbuilderCustomContentPack`
**Import**: `use crate::schemas::content_pack::*;`
**Validated Types from index.html**:
- `PathbuilderCustomContentPackCustomPackId(::std::string::String)`
- `PathbuilderCustomContentPackCustomPackName(::std::string::String)`
- Contains all custom types: `CustomAncestry`, `CustomBackground`, `CustomItem`, `CustomSpell`, `CustomFeat`, `CustomHeritage`

- [ ] **Field Mappings**:
  - `customPackId: Option<PathbuilderCustomContentPackCustomPackId>` → `id: String` via `.map_or_else(String::new, |x| x.0)`
  - `customPackName: Option<PathbuilderCustomContentPackCustomPackName>` → `name: String` via `.map_or_else(String::new, |x| x.0)`
  - `custom_ancestries: Option<Vec<CustomAncestry>>` → Map to collection
  - `custom_backgrounds: Option<Vec<CustomBackground>>` → Map to collection
  - `custom_items: Option<Vec<CustomItem>>` → Map to collection
  - `custom_spells: Option<Vec<CustomSpell>>` → Map to collection
  - `custom_feats: Option<Vec<CustomFeat>>` → Map to collection
  - `custom_heritages: Option<Vec<CustomHeritage>>` → Map to collection

## Helper Functions & Utilities

### **Conversion Utilities**
```rust
// String extraction from tuple structs
fn extract_string_field<T>(field: Option<T>) -> String 
where T: Into<String> { field.map_or_else(String::new, Into::into) }

// Optional string extraction  
fn extract_optional_string<T>(field: Option<T>) -> Option<String>
where T: Into<String> { field.map(Into::into) }

// Parse PF2e ability references ("0"=STR, "1"=DEX, etc.)
fn parse_ability_ref(ref_str: &str) -> Option<AbilityType>

// Parse PF2e size enum (1=Small, 2=Medium, 3=Large, 4=Huge)
fn parse_size_category(size: i32) -> String

// Parse comma-separated traits to Vec<String>
fn parse_traits(traits_str: &str) -> Vec<String>

// Parse spell components string to SpellComponents struct
fn parse_spell_components(components_str: &str) -> SpellComponents

// Parse item category to ItemType enum
fn parse_item_category(category: &str) -> ItemType

// Parse damage type string to DamageType enum  
fn parse_damage_type(damage_type: &str) -> DamageType

// Parse PF2e bulk to weight conversion
fn bulk_to_weight(bulk_str: &str) -> Option<f64>

// Parse PF2e price format ("1 gp", "5 sp", etc.)
fn parse_price_to_currency(price_str: &str) -> Option<Currency>
```

## Implementation Strategy

### Step 1: Implement Pathbuilder Platform Mappings
```rust
// crates/platforms/ttrpg-pathbuilder-pf2e/src/mappings/
├── character.rs     // CustomAncestry/CustomBackground → TtrpgCharacter
├── item.rs          // CustomItem → TtrpgItem  
├── spell.rs         // CustomSpell → TtrpgSpell
├── helpers/
│   ├── conversion.rs // String extraction, parsing utilities
│   ├── validation.rs // Data validation helpers
│   └── platform_specific.rs // PF2e-specific conversions
└── mod.rs
```

### Step 2: Access Pattern Implementation
```rust
// Correct tuple struct access patterns (VALIDATED against index.html):
use crate::schemas::custom_ancestry::*;

impl From<CustomAncestry> for TtrpgCharacter {
    fn from(ancestry: CustomAncestry) -> Self {
        Self {
            // ✅ CustomAncestryId(::std::string::String)
            id: ancestry.id.map_or_else(String::new, |x| x.0),
            // ✅ CustomAncestryName(::std::string::String)
            name: ancestry.name.map_or_else(String::new, |x| x.0),
            // ✅ CustomAncestryDescription(::std::string::String)
            biography: ancestry.description.map(|x| x.0),
            // ⚠️ CustomAncestrySize(i64) - Note: i64 not i32
            // Add timestamp handling: CustomAncestryTimestamp(::std::string::String)
            // ... more field mappings
        }
    }
}
```

### Step 3: Comprehensive Testing  
- Test each From<Schema> → TtrpgType conversion
- Verify correct tuple struct field access (.0 extraction)
- Test with real Pathbuilder JSON data samples
- Validate enum conversions (ItemType, DamageType, SpellSchool)
- Test helper parsing functions independently

### Step 4: Integration & Validation
- Run workspace clippy check for zero warnings
- Compile and test all platform mappings
- Verify no missing required fields in target types
- Performance test with large content packs

## Success Criteria
- [x] Correct schema type access patterns identified and **validated against index.html**
- [x] **Type validation completed**: All tuple struct types match generated documentation
- [ ] All 4 core Pathbuilder custom types mapped to TtrpgTypes
- [ ] Helper functions implemented and tested
- [ ] Zero compilation errors/warnings
- [ ] 85%+ test coverage for mapping modules
- [ ] Documentation with mapping examples

## ⚠️ Key Validation Findings
1. **CustomAncestrySize is `i64`** not `i32` - update mapping logic
2. **All schema types have `Timestamp` fields** - need to handle in `system_data`
3. **Import paths confirmed**: `use crate::schemas::{custom_ancestry, custom_background, custom_item, custom_spell}::*;`
4. **Tuple struct pattern confirmed**: All generated types are `TypeName(::std::string::String)` or `TypeName(i64)`

## Current Status & Next Steps
1. ✅ **Schema structure analyzed** - Tuple struct access patterns identified
2. ✅ **Target types verified** - TtrpgCharacter, TtrpgItem, TtrpgSpell have required fields
3. ✅ **Mappings validated against index.html** - All types and imports confirmed
4. ⏭️ **Next**: Implement clean mappings using validated patterns
5. ⏭️ **Then**: Add comprehensive tests and helper functions
6. ⏭️ **Finally**: Integration testing and cleanup
