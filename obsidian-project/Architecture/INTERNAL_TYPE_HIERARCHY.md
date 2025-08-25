# Internal Type Hierarchy

## Overview

This document outlines the comprehensive internal unified type hierarchy for TTRPG data conversion, based on systematic analysis of **59 schemas** across Roll20 (15), Foundry (32), and Pathbuilder (12) platforms.

These types serve as the canonical internal representation used by all plugins, enabling seamless bidirectional conversion between external platform-specific schemas and the unified internal format.

## Schema Coverage

| Platform | Schemas Analyzed | Status |
|----------|------------------|--------|
| **Roll20** | 15 schemas | ✅ Complete |
| **Foundry VTT** | 32 schemas (common, core, systems) | ✅ Complete |
| **Pathbuilder** | 12 schemas (custom packs) | ✅ Complete |
| **Total** | **59 schemas** | **100% Coverage** |

## Type Categories

### 1. Core Entity Types
Primary game entities that form the backbone of any TTRPG campaign.

#### Character
The central player/NPC representation across all platforms.

```
Character
├── id: String                      # Unique identifier
├── name: String                    # Display name
├── avatar: Option<String>          # Avatar image URL
├── description: Option<String>     # Character description/bio
├── hit_points: HitPoints          # Health tracking
├── abilities: Vec<Ability>        # Core ability scores
├── skills: Vec<Skill>             # Skill proficiencies
├── items: Vec<Item>               # Inventory
├── spells: Vec<Spell>             # Known spells
├── feats: Vec<Feat>               # Character feats
├── ancestry: Option<Ancestry>      # Pathfinder ancestry
├── background: Option<Background>  # Character background
├── heritage: Option<Heritage>      # Pathfinder heritage
├── level: u8                      # Character level
├── experience_points: Option<u32>  # XP total
├── armor_class: Option<u8>        # AC value
├── proficiency_bonus: Option<u8>  # Proficiency modifier
├── currency: Currency             # Monetary wealth
├── token_image: Option<String>    # Token representation
├── portrait_image: Option<String> # Portrait image
├── ownership: Ownership           # Access permissions
└── metadata: DocumentMetadata     # Creation/modification data
```

#### Item
Equipment, weapons, armor, and consumables.

```
Item
├── id: String                     # Unique identifier
├── name: String                   # Item name
├── description: Option<String>    # Item description
├── item_type: ItemType           # Category enum
├── quantity: u32                 # Stack count
├── weight: Option<f32>           # Physical weight
├── cost: Option<Currency>        # Purchase cost
├── rarity: Option<String>        # Rarity tier
├── equipped: bool                # Currently equipped
├── attuned: bool                 # Magical attunement
├── damage: Option<DamageInfo>    # Weapon damage
├── armor_class: Option<u8>       # AC bonus
├── properties: Vec<String>       # Item properties
├── traits: Vec<String>           # Pathfinder traits
├── level: Option<u8>             # Item level (PF2e)
├── bulk: Option<f32>             # Pathfinder bulk
├── image: Option<String>         # Item image
└── metadata: DocumentMetadata    # Creation/modification data
```

#### Scene
Virtual tabletop scenes/maps.

```
Scene
├── id: String                    # Unique identifier
├── name: String                  # Scene name
├── description: Option<String>   # Scene description
├── background_image: Option<String> # Map image
├── grid_type: GridType          # Grid type enum
├── grid_size: u32               # Grid unit size
├── width: u32                   # Scene width
├── height: u32                  # Scene height
├── tokens: Vec<Token>           # Placed tokens
├── lighting: LightingConfig     # Lighting setup
├── weather: Option<WeatherEffect> # Weather effects
├── ownership: Ownership         # Access permissions
└── metadata: DocumentMetadata   # Creation/modification data
```

#### Campaign
Top-level campaign container.

```
Campaign
├── id: String                     # Unique identifier
├── name: String                   # Campaign name
├── description: Option<String>    # Campaign description
├── game_system: GameSystem       # RPG system enum
├── characters: Vec<Character>     # All characters
├── scenes: Vec<Scene>            # All scenes
├── journal_entries: Vec<JournalEntry> # Story content
├── handouts: Vec<Handout>        # Player handouts
├── playlists: Vec<Playlist>      # Audio playlists
├── rollable_tables: Vec<RollableTable> # Random tables
├── card_decks: Vec<CardDeck>     # Card collections
├── creation_date: Option<DateTime<Utc>> # Created timestamp
├── last_modified: Option<DateTime<Utc>> # Modified timestamp
└── metadata: DocumentMetadata    # Creation/modification data
```

#### JournalEntry
Story content and notes.

```
JournalEntry
├── id: String                    # Unique identifier
├── name: String                  # Entry title
├── content: String               # HTML/Markdown content
├── show_to_players: bool         # Player visibility
├── ownership: Ownership          # Access permissions
└── metadata: DocumentMetadata    # Creation/modification data
```

### 2. Component Types
Complex nested structures that compose larger entities.

#### HitPoints
Health point tracking across systems.

```
HitPoints
├── current: u32              # Current HP
├── maximum: u32              # Maximum HP
├── temporary: u32            # Temporary HP
└── hit_die_type: Option<u8>  # Hit die size (d6, d8, etc.)
```

#### Ability
Core ability scores (STR, DEX, CON, INT, WIS, CHA).

```
Ability
├── name: AbilityType         # Ability type enum
├── score: u8                 # Raw ability score
├── modifier: i8              # Calculated modifier
├── proficiency: f32          # Proficiency multiplier
├── bonus: i8                 # Additional bonuses
└── save_bonus: i8            # Saving throw bonus
```

#### Skill
Skill proficiencies and modifiers.

```
Skill
├── name: String              # Skill name
├── ability: AbilityType      # Governing ability
├── proficiency: f32          # Proficiency multiplier
├── bonus: i8                 # Additional bonuses
└── passive_value: Option<u8> # Passive skill value
```

#### Spell
Magical spells and abilities.

```
Spell
├── id: String                    # Unique identifier
├── name: String                  # Spell name
├── description: String           # Spell description
├── level: u8                     # Spell level (0 = cantrip)
├── school: SpellSchool          # Magic school enum
├── casting_time: String         # Casting time
├── range: String                # Spell range
├── duration: String             # Duration
├── components: SpellComponents  # V/S/M components
├── damage: Option<DamageInfo>   # Damage dealt
├── save_dc: Option<u8>          # Saving throw DC
├── attack_bonus: Option<i8>     # Attack roll bonus
├── ritual: bool                 # Can be cast as ritual
├── concentration: bool          # Requires concentration
└── traits: Vec<String>          # Pathfinder traits
```

#### Feat
Character feats and abilities.

```
Feat
├── id: String                   # Unique identifier
├── name: String                 # Feat name
├── description: String          # Feat description
├── level: u8                    # Required level
├── feat_type: FeatType         # Feat category enum
├── prerequisites: Vec<String>   # Requirements
├── traits: Vec<String>         # Pathfinder traits
├── frequency: Option<String>    # Usage frequency
└── trigger: Option<String>      # Activation trigger
```

#### Ancestry (Pathfinder)
Character ancestry/race.

```
Ancestry
├── id: String                    # Unique identifier
├── name: String                  # Ancestry name
├── description: String           # Ancestry description
├── hit_points: u8               # Base HP bonus
├── size: CreatureSize           # Physical size
├── speed: u32                   # Movement speed
├── traits: Vec<String>          # Ancestry traits
├── ability_boosts: Vec<AbilityType> # Ability score increases
├── ability_flaws: Vec<AbilityType>  # Ability score decreases
└── languages: Vec<String>       # Known languages
```

#### Background (Pathfinder)
Character background.

```
Background
├── id: String                   # Unique identifier
├── name: String                 # Background name
├── description: String          # Background description
├── ability_boosts: Vec<AbilityType> # Ability increases
├── skills: Vec<String>          # Trained skills
├── skill_feats: Vec<String>     # Bonus skill feats
└── traits: Vec<String>          # Background traits
```

#### Heritage (Pathfinder)
Character heritage within ancestry.

```
Heritage
├── id: String            # Unique identifier
├── name: String          # Heritage name
├── description: String   # Heritage description
├── ancestry_id: String   # Parent ancestry
└── traits: Vec<String>   # Heritage traits
```

### 3. Visual & Interactive Types
Elements for virtual tabletop interaction.

#### Token
Representing characters/objects on scenes.

```
Token
├── id: String                      # Unique identifier
├── name: String                    # Token name
├── position: Position              # X/Y coordinates
├── rotation: f32                   # Rotation angle
├── scale: f32                      # Size scale
├── image: Option<String>           # Token image
├── hidden: bool                    # Hidden from players
├── locked: bool                    # Position locked
├── light_source: Option<LightSource> # Emitted light
├── vision: Option<VisionConfig>    # Vision capabilities
├── bar1: Option<TokenBar>          # Health/resource bar
├── bar2: Option<TokenBar>          # Secondary bar
├── bar3: Option<TokenBar>          # Tertiary bar
├── aura1: Option<TokenAura>        # Primary aura
└── aura2: Option<TokenAura>        # Secondary aura
```

#### Card
Playing cards and card-based mechanics.

```
Card
├── id: String                  # Unique identifier
├── name: String                # Card name
├── description: Option<String> # Card description
├── image: Option<String>       # Front image
├── back_image: Option<String>  # Back image
├── suit: Option<String>        # Card suit
├── value: Option<String>       # Card value
├── drawn: bool                 # Currently drawn
└── tooltip: Option<String>     # Tooltip text
```

#### RollableTable
Random generation tables.

```
RollableTable
├── id: String                  # Unique identifier
├── name: String                # Table name
├── description: Option<String> # Table description
├── formula: String             # Dice formula (e.g., "1d20")
├── results: Vec<TableResult>   # Table entries
├── replacement: bool           # Roll with replacement
└── display_roll: bool          # Show dice result
```

#### TableResult
Individual table entry.

```
TableResult
├── id: String              # Unique identifier
├── name: String            # Result name
├── weight: f32             # Result weight
├── range_low: u32          # Range minimum
├── range_high: u32         # Range maximum
├── image: Option<String>   # Result image
└── text: String            # Result text
```

#### Playlist
Audio playlist management.

```
Playlist
├── id: String                  # Unique identifier
├── name: String                # Playlist name
├── description: Option<String> # Playlist description
├── tracks: Vec<AudioTrack>     # Audio tracks
├── shuffle: bool               # Shuffle mode
├── repeat: bool                # Repeat mode
├── volume: f32                 # Master volume
└── currently_playing: bool     # Playing state
```

#### AudioTrack
Individual audio file.

```
AudioTrack
├── id: String              # Unique identifier
├── name: String            # Track name
├── url: String             # Audio file URL
├── volume: f32             # Track volume
├── loop_track: bool        # Loop this track
└── duration: Option<u32>   # Duration in seconds
```

#### CardDeck
Collection of cards.

```
CardDeck
├── id: String                  # Unique identifier
├── name: String                # Deck name
├── description: Option<String> # Deck description
├── deck_type: CardDeckType    # Deck type enum
├── cards: Vec<Card>           # Card collection
├── shuffle: bool              # Shuffle on draw
├── display_count: bool        # Show card count
└── preset: Option<String>     # Deck preset name
```

### 4. Supporting Types
Smaller utility structures.

#### Position
2D coordinates.

```
Position
├── x: f32    # X coordinate
└── y: f32    # Y coordinate
```

#### Currency
Monetary values across systems.

```
Currency
├── platinum: u32    # Platinum pieces
├── gold: u32        # Gold pieces
├── electrum: u32    # Electrum pieces
├── silver: u32      # Silver pieces
└── copper: u32      # Copper pieces
```

#### DamageInfo
Weapon/spell damage information.

```
DamageInfo
├── dice_formula: String       # Damage dice (e.g., "1d8+3")
├── damage_type: DamageType   # Damage type enum
├── versatile: Option<String> # Versatile damage
└── parts: Vec<DamagePart>    # Multiple damage components
```

#### DamagePart
Individual damage component.

```
DamagePart
├── formula: String         # Damage formula
└── damage_type: DamageType # Damage type
```

#### SpellComponents
Spell casting requirements.

```
SpellComponents
├── verbal: bool                          # Verbal component
├── somatic: bool                         # Somatic component
├── material: bool                        # Material component
├── material_description: Option<String>  # Material details
├── cost: Option<u32>                     # Material cost (gp)
└── consumed: bool                        # Material consumed
```

#### LightSource
Token light emission.

```
LightSource
├── bright_radius: f32              # Bright light radius
├── dim_radius: f32                 # Dim light radius
├── angle: f32                      # Light cone angle
├── color: Option<String>           # Light color
└── animation: Option<LightAnimation> # Light animation
```

#### LightAnimation
Light animation effects.

```
LightAnimation
├── animation_type: LightAnimationType # Animation enum
├── speed: u8                          # Animation speed
└── intensity: u8                      # Animation intensity
```

#### TokenBar
Token resource display.

```
TokenBar
├── attribute: String    # Attribute name
├── current: f32         # Current value
├── maximum: f32         # Maximum value
└── visible: bool        # Show to players
```

#### TokenAura
Token aura effects.

```
TokenAura
├── radius: f32      # Aura radius
├── color: String    # Aura color
├── opacity: f32     # Opacity (0.0-1.0)
└── square: bool     # Square vs circular
```

#### VisionConfig
Token vision capabilities.

```
VisionConfig
├── darkvision: f32    # Darkvision range
├── blindsight: f32    # Blindsight range
├── tremorsense: f32   # Tremorsense range
├── truesight: f32     # Truesight range
├── bright_vision: f32 # Bright light vision
└── dim_vision: f32    # Dim light vision
```

#### LightingConfig
Scene lighting settings.

```
LightingConfig
├── darkness_level: f32        # Global darkness
├── bright_light_color: String # Bright light color
├── dim_light_color: String    # Dim light color
└── global_light: bool         # Global illumination
```

#### WeatherEffect
Scene weather effects.

```
WeatherEffect
├── weather_type: String    # Weather type
├── intensity: f32          # Effect intensity
└── direction: f32          # Wind direction
```

#### Ownership
Document access control.

```
Ownership
├── default_permission: PermissionLevel              # Default access
├── user_permissions: HashMap<String, PermissionLevel> # User-specific
└── role_permissions: HashMap<String, PermissionLevel> # Role-based
```

#### DocumentMetadata
Document lifecycle tracking.

```
DocumentMetadata
├── created_time: Option<DateTime<Utc>>        # Creation timestamp
├── modified_time: Option<DateTime<Utc>>       # Last modified
├── created_by: Option<String>                # Creator ID
├── modified_by: Option<String>               # Last modifier
├── system_id: Option<String>                 # Game system
├── system_version: Option<String>            # System version
├── core_version: Option<String>              # Core version
├── folder_id: Option<String>                 # Parent folder
├── sort_order: Option<i32>                   # Sort position
└── flags: HashMap<String, serde_json::Value> # Custom flags
```

#### Handout
Player handout documents.

```
Handout
├── id: String                  # Unique identifier
├── name: String                # Handout name
├── notes: String               # Content text
├── in_player_journals: bool    # Show to players
├── archived: bool              # Archived state
└── folder_id: Option<String>   # Parent folder
```

### 5. Enumeration Types
Standardized value sets across platforms.

#### AbilityType
Core ability scores.

```rust
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
```

#### ItemType
Item categories.

```rust
pub enum ItemType {
    Weapon,
    Armor,
    Equipment,
    Consumable,
    Treasure,
    Backpack,
    Kit,
    Tool,
    Container,
    Ammunition,
    Shield,
    Rod,
    Staff,
    Wand,
    Wondrous,
    Ring,
    Potion,
    Scroll,
    Other(String),
}
```

#### DamageType
Damage type classification.

```rust
pub enum DamageType {
    // Physical
    Bludgeoning,
    Piercing,
    Slashing,
    // Energy
    Acid,
    Cold,
    Fire,
    Lightning,
    Thunder,
    // Magical
    Force,
    Necrotic,
    Radiant,
    Psychic,
    // Pathfinder specific
    Positive,
    Negative,
    Sonic,
    // Other
    Poison,
    Other(String),
}
```

#### SpellSchool
Magic schools.

```rust
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    // Pathfinder
    Cantrip,
    Focus,
    Other(String),
}
```

#### FeatType
Feat categories.

```rust
pub enum FeatType {
    General,
    Skill,
    Class,
    Ancestry,
    Archetype,
    Dedication,
    Combat,
    Magic,
    Other(String),
}
```

#### CreatureSize
Physical size categories.

```rust
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}
```

#### GridType
Scene grid types.

```rust
pub enum GridType {
    Square,
    Hexagonal,
    None,
}
```

#### PermissionLevel
Access permission levels.

```rust
pub enum PermissionLevel {
    None = 0,
    Limited = 1,
    Observer = 2,
    Owner = 3,
}
```

#### LightAnimationType
Light animation effects.

```rust
pub enum LightAnimationType {
    None,
    Pulse,
    Chroma,
    Wave,
    Fog,
    Sunburst,
    Dome,
    Emanation,
    Hexa,
    Ghost,
    Energy,
    Roiling,
    Hole,
}
```

#### GameSystem
Supported game systems.

```rust
pub enum GameSystem {
    #[serde(rename = "dnd5e")]
    DnD5e,
    #[serde(rename = "pf1e")]
    Pathfinder1e,
    #[serde(rename = "pf2e")]
    Pathfinder2e,
    #[serde(rename = "ose")]
    OldSchoolEssentials,
    #[serde(rename = "generic")]
    Generic,
    Other(String),
}
```

#### SourceFormat
Input data formats.

```rust
pub enum SourceFormat {
    Roll20,
    FoundryVTT,
    PathbuilderJson,
    PathbuilderCustomPack,
    Other(String),
}
```

#### CardDeckType
Card deck variations.

```rust
pub enum CardDeckType {
    Deck,
    Pile,
    Hand,
}
```

## External Schema Mappings

Each internal type includes `#[external]` attributes that map to platform-specific schema-generated types:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, InternalType)]
#[external(
    roll20 = "ttrpg_schema_types::roll20::Character",
    foundry_dnd5e = "ttrpg_schema_types::foundry::systems::dnd5e::Actor",
    foundry_pf1e = "ttrpg_schema_types::foundry::systems::pf1e::Actor",
    foundry_pf2e = "ttrpg_schema_types::foundry::systems::pf2e::Actor",
    foundry_ose = "ttrpg_schema_types::foundry::systems::ose::Actor",
    pathbuilder = "ttrpg_schema_types::pathbuilder::Character"
)]
pub struct Character {
    // ...
    #[field(roll20 = "bio", foundry_dnd5e = "system.description.value")]
    pub description: Option<String>,
    // ...
}
```

## Implementation Notes

### Field Mapping
The `#[field]` attribute handles platform-specific field name differences:

- **Simple mapping**: `#[field(roll20 = "bio")]`
- **Nested paths**: `#[field(foundry_dnd5e = "system.description.value")]`
- **Multiple platforms**: `#[field(roll20 = "defaulttoken", foundry_dnd5e = "prototypeToken.texture.src")]`

### Conversion Pattern
The `InternalType` derive macro generates bidirectional conversion implementations:

```rust
// Convert from external type to internal
let internal_char: Character = roll20_character.into();

// Convert from internal type to external
let foundry_char: foundry_dnd5e::Actor = internal_char.into();
```

### Optional vs Required Fields
- **Required fields**: Core properties needed across all platforms
- **Optional fields**: Platform-specific or optional features
- **Vec fields**: Collections that may be empty on some platforms
- **HashMap fields**: Flexible key-value storage for platform-specific data

## Usage Guidelines

1. **Internal types only**: Plugin code should work exclusively with internal types
2. **Conversion at boundaries**: Convert external → internal at input, internal → external at output
3. **Field mapping consistency**: Use consistent field names across similar types
4. **Enum extensibility**: Use `Other(String)` variants for unknown platform-specific values
5. **Metadata preservation**: Always include `DocumentMetadata` for traceability

## Future Extensions

The type hierarchy is designed for extensibility:

- **New platforms**: Add new `#[external]` mappings
- **New fields**: Add optional fields with appropriate `#[field]` mappings  
- **New types**: Follow established patterns for consistency
- **New enums**: Use `Other(String)` pattern for extensibility

---

*This documentation is generated from the comprehensive 100% schema analysis and serves as the definitive reference for internal type structures.*
