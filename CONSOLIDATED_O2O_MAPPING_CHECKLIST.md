# Consolidated O2O Mapping Implementation Checklist

## Overview
This checklist consolidates mappings from both `COMPREHENSIVE_TYPE_MAPPINGS.md` and `O2O_MAPPING_IMPLEMENTATION_CHECKLIST.md` into an actionable implementation guide for all 63 schema files → 78 internal types.

**Implementation Location**: `crates/platforms/*/src/mappings.rs`

---

## Phase 1: Core Entity Mappings (High Priority)

### ✅ Character Mappings
- **Foundry**: FoundryActor → TtrpgCharacter
  - [x] Basic fields (id, name, biography, level)
  - [x] Hit points (system.attributes.hp)
  - [x] Armor class (system.attributes.ac.value)
  - [ ] **MISSING**: Abilities mapping from system.abilities
  - [ ] **MISSING**: Skills mapping from system.skills
  - [ ] **MISSING**: New fields (actor_type, image, permissions, experience_points, inspiration, notes)

- **Roll20**: Roll20CharacterSheet → TtrpgCharacter  
  - [x] Basic fields (id, name, biography)
  - [x] Abilities from attribs array
  - [x] Hit points from attribs (name="hp")
  - [x] Level from attribs (name="level")
  - [ ] **MISSING**: Skills mapping
  - [ ] **MISSING**: New fields (actor_type, image, permissions, experience_points, inspiration, notes)

- **Pathbuilder**: PathbuilderCharacter → TtrpgCharacter
  - [x] Basic fields (id, name, notes→biography, level)
  - [x] Hit points (hit_points object)
  - [x] Armor class (ac_total.ac_total)
  - [ ] **MISSING**: Abilities mapping (commented out - schema needed)
  - [ ] **MISSING**: Skills mapping (commented out - schema needed)
  - [ ] **MISSING**: New fields (actor_type, image, permissions, experience_points, inspiration, notes)

### ✅ Scene Mappings
- **Foundry**: FoundryScene → TtrpgScene
  - [x] Basic fields (id, name, dimensions, background)
  - [ ] **MISSING**: Grid configuration mapping
  - [ ] **MISSING**: Light sources mapping
  - [ ] **MISSING**: Token mapping
  - [ ] **MISSING**: New fields (weather, grid_opacity)

- **Roll20**: Roll20Page → TtrpgScene
  - [x] Basic fields (id, name, dimensions, background)
  - [ ] **MISSING**: Grid configuration (scale_number, grid settings)
  - [ ] **MISSING**: Lighting settings mapping
  - [ ] **MISSING**: Token mapping
  - [ ] **MISSING**: New fields (weather, grid_opacity)

### ✅ Campaign Mappings  
- **Foundry**: FoundryWorld → TtrpgCampaign
  - [x] Basic fields (id→title, description, version, game_system→system)
  - [x] Empty collections setup
  - [ ] **MISSING**: system_data field (recently added to Campaign)

- **Roll20**: Roll20Campaign → TtrpgCampaign
  - [x] Basic fields (id, name, description)
  - [x] Character/scene/journal collections
  - [ ] **MISSING**: system_data field (recently added to Campaign)

- **Pathbuilder**: PathbuilderCustomContentPack → TtrpgCampaign  
  - [x] Basic fields (custom_pack_id→id, custom_pack_name→name)
  - [x] Custom content in system_data
  - [ ] **MISSING**: system_data field (recently added to Campaign)

### ✅ Item Mappings
- **Foundry**: FoundryItem → TtrpgItem
  - [x] Basic fields (id, name, description, type mapping)
  - [x] Quantity, weight, system_data
  - [ ] **MISSING**: New fields (image, damage, armor_class, requires_attunement)

- **Pathbuilder**: CustomItem → TtrpgItem
  - [x] Basic fields (id, name, description, type mapping)
  - [x] Quantity, weight→bulk, cost→price, rarity
  - [ ] **MISSING**: New fields (image, damage, armor_class, requires_attunement)

### ✅ Journal Mappings
- **Foundry**: FoundryJournalEntry → TtrpgJournal
  - [x] Basic fields (id, name)
  - [ ] **MISSING**: Content mapping
  - [ ] **MISSING**: Pages mapping
  - [ ] **MISSING**: Ownership→show_to_players mapping

- **Roll20**: Roll20Handout → TtrpgJournal
  - [x] Basic fields (id, name, content→notes)
  - [x] Visibility (showplayers→show_to_players)
  - [x] Single document structure (pages: None)

---

## Phase 2: Component Type Mappings (Medium Priority)

### Helper Functions Needed
```rust
// Required helper functions for complex mappings
fn abilities_from_foundry_dnd5e(abilities: &FoundryAbilities) -> Vec<Ability>
fn skills_from_foundry_dnd5e(skills: &FoundrySkills) -> Vec<Skill>  
fn grid_config_from_foundry(scene: &FoundryScene) -> Option<GridConfig>
fn light_sources_from_foundry(lights: &[FoundryLight]) -> Vec<LightSource>
fn tokens_from_foundry(tokens: &[FoundryToken]) -> Vec<Token>

fn abilities_from_roll20_attribs(attribs: &[Roll20Attribute]) -> Vec<Ability>
fn skills_from_roll20_attribs(attribs: &[Roll20Attribute]) -> Vec<Skill>
fn grid_config_from_roll20(page: &Roll20Page) -> Option<GridConfig>

fn abilities_from_pathbuilder(character: &PathbuilderCharacter) -> Vec<Ability>
fn skills_from_pathbuilder(character: &PathbuilderCharacter) -> Vec<Skill>
```

### ✅ Spell Mappings
- **Pathbuilder**: CustomSpell → TtrpgSpell
  - [x] Basic fields (id, name, description, level, school)
  - [x] Casting details (cast→casting_time, range, duration)
  - [x] Components mapping (component_* flags)
  - [x] System-specific data (traditions, traits, rarity)

---

## Phase 3: Missing Core Types (High Priority)

### Core Types Not Yet Implemented
- [ ] **RollableTable + TableResult**: Roll20 has rollabletables in system_data
- [ ] **Macro**: Roll20 has macros in system_data  
- [ ] **Card/CardDeck**: Roll20 has decks in system_data
- [ ] **Audio (Playlist/AudioTrack)**: Foundry has playlist support

### Pathbuilder-Specific Types Needed
- [ ] **Ancestry**: For PF2e character creation
- [ ] **Background**: For PF2e character creation
- [ ] **Heritage**: For PF2e character creation  
- [ ] **Feat**: For PF2e character abilities

---

## Phase 4: Field Audit - Recently Added Fields

### TtrpgCharacter New Fields (All platforms need these)
- [ ] **actor_type**: `Option<ActorType>` - Map from platform type fields
- [ ] **image**: `Option<String>` - Map from avatar/img fields
- [ ] **permissions**: `Option<Ownership>` - Map from controlledby/ownership
- [ ] **experience_points**: `Option<u32>` - Map from xp/experience attributes
- [ ] **inspiration**: `Option<bool>` - Map from inspiration attributes (D&D 5e)
- [ ] **notes**: `Option<String>` - Map from GM notes/private notes

### TtrpgItem New Fields (All platforms need these) 
- [ ] **image**: `Option<String>` - Map from img/icon fields
- [ ] **damage**: `Option<DamageInfo>` - Map from damage dice/type
- [ ] **armor_class**: `Option<ArmorClass>` - Map from AC values for armor
- [ ] **requires_attunement**: `Option<bool>` - Map from attunement requirements

### TtrpgScene New Fields
- [ ] **weather**: `Option<String>` - Map from weather conditions
- [ ] **grid_opacity**: `Option<f32>` - Map from grid visibility settings

### Campaign New Field
- [x] **system_data**: `HashMap<String, serde_json::Value>` - Added to Campaign type
- [ ] **Update all platform mappings** to include system_data initialization

---

## Implementation Status Summary

### ✅ Completed Mappings
- **Foundry**: 5/5 core entities (partial field coverage)
- **Roll20**: 4/4 core entities (partial field coverage)  
- **Pathbuilder**: 3/3 implemented entities (partial field coverage)

### 🔄 In Progress
- Field completeness for all platforms
- Helper function implementations
- Missing component type creation

### ❌ Not Started
- Audio/Playlist mappings
- RollableTable/Macro/Card mappings
- Pathbuilder-specific types (Ancestry, Background, Heritage, Feat)

---

## Testing Strategy

### Unit Tests Required
- [x] Basic mapping tests exist for all platforms
- [ ] **Missing field tests** for newly added fields
- [ ] **Helper function tests** for complex mappings
- [ ] **Round-trip tests** for bidirectional mapping accuracy

### Integration Tests Needed
- [ ] **Full campaign conversion** tests
- [ ] **Cross-platform compatibility** tests
- [ ] **Performance benchmarks** for large datasets

---

## Next Implementation Steps

1. **Add missing fields** to existing platform mappings
2. **Implement helper functions** for complex nested mappings
3. **Create missing core types** (RollableTable, Macro, Card, Audio)
4. **Add Pathbuilder-specific types** (Ancestry, Background, Heritage, Feat)
5. **Update tests** for new fields and complete coverage
6. **Implement bidirectional mappings** (TtrpgType → PlatformType)

---

## Code Organization

```
crates/platforms/
├── ttrpg-foundry-common/src/
│   ├── mappings.rs           # ✅ Foundry → Internal mappings
│   └── reverse_mappings.rs   # ❌ Internal → Foundry mappings (TODO)
├── ttrpg-roll20/src/
│   ├── mappings.rs           # ✅ Roll20 → Internal mappings  
│   └── reverse_mappings.rs   # ❌ Internal → Roll20 mappings (TODO)
└── ttrpg-pathbuilder-pf2e/src/
    ├── mappings.rs           # ✅ Pathbuilder → Internal mappings
    └── reverse_mappings.rs   # ❌ Internal → Pathbuilder mappings (TODO)
```
