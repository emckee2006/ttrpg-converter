//! Core data types and structures for TTRPGConverter
//!
//! This module defines the fundamental data structures used throughout the application
//! for representing TTRPG campaigns, entities, and conversion metadata.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a TTRPG campaign in its abstract form
///
/// This is the core data structure that holds all campaign information
/// after parsing from source formats but before conversion to target formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    /// Campaign metadata
    pub metadata: CampaignMetadata,

    /// All characters/actors in the campaign
    pub actors: Vec<Actor>,

    /// All scenes/maps in the campaign
    pub scenes: Vec<Scene>,

    /// All items in the campaign
    pub items: Vec<Item>,

    /// All journal entries/handouts
    pub journal: Vec<JournalEntry>,

    /// All rollable tables
    pub tables: Vec<RollableTable>,

    /// All playlists/audio content
    pub playlists: Vec<Playlist>,

    /// All macros
    pub macros: Vec<Macro>,

    /// Folder organization structure
    pub folders: Vec<Folder>,

    /// Asset references and mappings
    pub assets: AssetCollection,
}

/// Campaign metadata and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetadata {
    /// Campaign name
    pub name: String,

    /// Original format (Roll20, Foundry, etc.)
    pub source_format: SourceFormat,

    /// Schema version of the source format
    pub schema_version: Option<String>,

    /// Creation/export timestamp
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// File size and statistics
    pub stats: CampaignStats,

    /// Custom properties from source format
    pub custom_properties: HashMap<String, serde_json::Value>,
}

/// Statistical information about the campaign
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignStats {
    /// Total number of entities
    pub total_entities: usize,

    /// Counts by entity type
    pub entity_counts: HashMap<String, usize>,

    /// Total assets
    pub total_assets: usize,

    /// Asset size information
    pub total_asset_size_mb: f64,

    /// Processing statistics
    pub processing_stats: ProcessingStats,
}

/// Statistics about the conversion process
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessingStats {
    /// How many entities were successfully converted
    pub converted_entities: usize,

    /// How many had warnings
    pub entities_with_warnings: usize,

    /// How many failed conversion
    pub failed_entities: usize,

    /// Asset processing stats
    pub assets_downloaded: usize,
    pub assets_cached: usize,
    pub assets_failed: usize,
}

/// Supported source formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceFormat {
    /// Roll20 campaign export
    Roll20,
    /// Foundry VTT world
    FoundryVtt,
    /// Generic JSON format
    Json,
}

/// Supported target formats  
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetFormat {
    /// Foundry VTT v10
    FoundryV10,
    /// Foundry VTT v11
    FoundryV11,
    /// Foundry VTT v12 (latest)
    FoundryV12,
    /// JSON export for analysis
    JsonExport,
}

/// Generic actor/character representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    /// Unique identifier
    pub id: String,

    /// Display name
    pub name: String,

    /// Actor type (PC, NPC, etc.)
    pub actor_type: ActorType,

    /// Image URLs/paths
    pub images: ActorImages,

    /// Character attributes and stats
    pub attributes: HashMap<String, AttributeValue>,

    /// Inventory items
    pub items: Vec<String>, // Item IDs

    /// Spells and abilities
    pub spells: Vec<Spell>,
    pub features: Vec<Feature>,

    /// Biography and notes
    pub biography: String,
    pub notes: String,

    /// Permissions and visibility
    pub permissions: EntityPermissions,

    /// Source-specific data
    pub source_data: HashMap<String, serde_json::Value>,
}

/// Types of actors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActorType {
    /// Player character
    Pc,
    /// Non-player character
    Npc,
    /// Vehicle
    Vehicle,
    /// Other/custom type
    Other(String),
}

/// Actor images (avatar, token, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActorImages {
    /// Main avatar image
    pub avatar: Option<String>,
    /// Token image for maps
    pub token: Option<String>,
    /// Additional character art
    pub additional: Vec<String>,
}

/// Flexible attribute value
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttributeValue {
    /// Numeric value
    Number(f64),
    /// Text value
    Text(String),
    /// Boolean value
    Boolean(bool),
    /// Complex structured value
    Complex(serde_json::Value),
}

/// Scene/map representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    /// Unique identifier
    pub id: String,

    /// Scene name
    pub name: String,

    /// Background image
    pub background_image: Option<String>,

    /// Scene dimensions
    pub dimensions: SceneDimensions,

    /// Grid configuration
    pub grid: GridConfig,

    /// Tokens placed on the scene
    pub tokens: Vec<Token>,

    /// Walls and barriers (if supported)
    pub walls: Vec<Wall>,

    /// Lighting configuration
    pub lighting: LightingConfig,

    /// Audio settings
    pub audio: Option<String>, // Playlist ID

    /// Notes and description
    pub notes: String,

    /// Permissions
    pub permissions: EntityPermissions,

    /// Source-specific data
    pub source_data: HashMap<String, serde_json::Value>,
}

/// Scene dimensions and scaling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneDimensions {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Pixels per grid unit
    pub scale: f64,
    /// Physical grid size
    pub grid_size: f64,
}

/// Grid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Grid type (square, hex, etc.)
    pub grid_type: GridType,
    /// Grid size in pixels
    pub size: f64,
    /// Grid color
    pub color: String,
    /// Grid opacity (0.0-1.0)
    pub opacity: f64,
}

/// Grid types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridType {
    Square,
    HexEven,
    HexOdd,
    None,
}

/// Token on a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Token identifier
    pub id: String,

    /// Linked actor ID
    pub actor_id: Option<String>,

    /// Position on scene
    pub position: Position,

    /// Token size (grid units)
    pub size: TokenSize,

    /// Token image
    pub image: Option<String>,

    /// Visibility settings
    pub hidden: bool,

    /// Light emission
    pub light: Option<LightConfig>,

    /// Vision settings
    pub vision: Option<VisionConfig>,

    /// Source-specific data
    pub source_data: HashMap<String, serde_json::Value>,
}

/// 2D position
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Token size
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TokenSize {
    pub width: f64,
    pub height: f64,
}

/// Wall/barrier on a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wall {
    /// Wall identifier
    pub id: String,

    /// Start and end coordinates
    pub coordinates: [Position; 2],

    /// Wall type
    pub wall_type: WallType,

    /// Movement restriction
    pub blocks_movement: bool,

    /// Sight restriction
    pub blocks_sight: bool,

    /// Light restriction
    pub blocks_light: bool,
}

/// Types of walls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WallType {
    Wall,
    Door,
    SecretDoor,
    Window,
    Terrain,
    Invisible,
}

/// Lighting configuration for scenes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LightingConfig {
    /// Global illumination level
    pub global_light: f64,

    /// Darkness level
    pub darkness: f64,

    /// Enable fog of war
    pub fog_of_war: bool,
}

/// Light emission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightConfig {
    /// Light radius in grid units
    pub radius: f64,

    /// Dim light radius
    pub dim_radius: f64,

    /// Light color (hex)
    pub color: String,

    /// Light intensity (0.0-1.0)
    pub intensity: f64,

    /// Light angle (degrees, 0 = omnidirectional)
    pub angle: f64,
}

/// Vision configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionConfig {
    /// Vision range
    pub range: f64,

    /// Darkvision range
    pub darkvision: f64,

    /// Can see through walls
    pub see_invisible: bool,
}

/// Item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique identifier
    pub id: String,

    /// Item name
    pub name: String,

    /// Item type
    pub item_type: ItemType,

    /// Item image
    pub image: Option<String>,

    /// Description
    pub description: String,

    /// Item properties
    pub properties: ItemProperties,

    /// Source-specific data
    pub source_data: HashMap<String, serde_json::Value>,
}

/// Types of items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Equipment,
    Consumable,
    Tool,
    Loot,
    Spell,
    Feature,
    Other(String),
}

/// Item properties
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemProperties {
    /// Item rarity
    pub rarity: Option<String>,

    /// Requires attunement
    pub attunement: bool,

    /// Item weight
    pub weight: Option<f64>,

    /// Item cost
    pub cost: Option<ItemCost>,

    /// Quantity
    pub quantity: u32,

    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
}

/// Item cost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCost {
    /// Cost amount
    pub amount: f64,
    /// Currency type
    pub currency: String,
}

/// Spell representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    /// Spell identifier
    pub id: String,

    /// Spell name
    pub name: String,

    /// Spell level (0 for cantrips)
    pub level: u8,

    /// Spell school
    pub school: String,

    /// Casting time
    pub casting_time: String,

    /// Range
    pub range: String,

    /// Components (V, S, M)
    pub components: SpellComponents,

    /// Duration
    pub duration: String,

    /// Spell description
    pub description: String,

    /// Higher level effects
    pub at_higher_levels: Option<String>,
}

/// Spell components
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpellComponents {
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
    pub materials_description: Option<String>,
}

/// Character feature/ability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    /// Feature identifier
    pub id: String,

    /// Feature name
    pub name: String,

    /// Feature type
    pub feature_type: FeatureType,

    /// Description
    pub description: String,

    /// Usage information
    pub usage: Option<FeatureUsage>,
}

/// Types of features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureType {
    RacialTrait,
    ClassFeature,
    Feat,
    Background,
    Other(String),
}

/// Feature usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureUsage {
    /// Uses per rest/day
    pub uses: u32,
    /// Rest type required
    pub recharge: RechargeType,
}

/// Recharge types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RechargeType {
    ShortRest,
    LongRest,
    Dawn,
    None,
}

/// Journal entry representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    /// Entry identifier
    pub id: String,

    /// Entry title
    pub title: String,

    /// Entry content (HTML/Markdown)
    pub content: String,

    /// Entry image
    pub image: Option<String>,

    /// Permissions
    pub permissions: EntityPermissions,

    /// Source-specific data
    pub source_data: HashMap<String, serde_json::Value>,
}

/// Rollable table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollableTable {
    /// Table identifier
    pub id: String,

    /// Table name
    pub name: String,

    /// Table entries
    pub entries: Vec<TableEntry>,

    /// Dice formula
    pub formula: String,
}

/// Table entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableEntry {
    /// Range (e.g., 1-5)
    pub range: (u32, u32),

    /// Result text
    pub text: String,

    /// Result image
    pub image: Option<String>,
}

/// Playlist representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Playlist identifier
    pub id: String,

    /// Playlist name
    pub name: String,

    /// Audio tracks
    pub tracks: Vec<AudioTrack>,
}

/// Audio track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    /// Track identifier
    pub id: String,

    /// Track name
    pub name: String,

    /// Audio file path/URL
    pub path: String,

    /// Loop the track
    pub loop_track: bool,

    /// Volume (0.0-1.0)
    pub volume: f64,
}

/// Macro representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    /// Macro identifier
    pub id: String,

    /// Macro name
    pub name: String,

    /// Macro command/script
    pub command: String,

    /// Macro type
    pub macro_type: MacroType,
}

/// Macro types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MacroType {
    Chat,
    Script,
    Other(String),
}

/// Folder for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// Folder identifier
    pub id: String,

    /// Folder name
    pub name: String,

    /// Folder type (what it contains)
    pub folder_type: String,

    /// Parent folder ID
    pub parent: Option<String>,

    /// Sorting order
    pub sort: i32,
}

/// Entity permissions and visibility
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityPermissions {
    /// Entity owner
    pub owner: Option<String>,

    /// Default permission level
    pub default_permission: PermissionLevel,

    /// User-specific permissions
    pub user_permissions: HashMap<String, PermissionLevel>,
}

/// Permission levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    None,
    Limited,
    Observer,
    Owner,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        Self::None
    }
}

/// Asset collection and management
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetCollection {
    /// All asset references found in the campaign
    pub references: Vec<AssetReference>,

    /// Successfully downloaded/cached assets
    pub cached_assets: HashMap<String, PathBuf>,

    /// Failed asset downloads
    pub failed_assets: Vec<String>,

    /// Asset statistics
    pub stats: AssetStats,
}

/// Individual asset reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    /// Original URL or path
    pub original_path: String,

    /// Local file path (if downloaded)
    pub local_path: Option<PathBuf>,

    /// Asset type
    pub asset_type: AssetType,

    /// File size in bytes
    pub size_bytes: Option<u64>,

    /// MIME type
    pub mime_type: Option<String>,

    /// Where this asset is used
    pub used_by: Vec<AssetUsage>,
}

/// Types of assets
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AssetType {
    Image,
    Audio,
    Video,
    Document,
    Other,
}

/// How an asset is used
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUsage {
    /// Entity type using this asset
    pub entity_type: String,

    /// Entity ID
    pub entity_id: String,

    /// Field where asset is used
    pub field: String,
}

/// Asset processing statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetStats {
    /// Total assets found
    pub total_assets: usize,

    /// Successfully processed
    pub processed_assets: usize,

    /// Failed to process
    pub failed_assets: usize,

    /// Total size of processed assets
    pub total_size_mb: f64,

    /// Asset type breakdown
    pub by_type: HashMap<AssetType, usize>,
}

// Display implementations for better error messages and debugging
impl std::fmt::Display for SourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFormat::Roll20 => write!(f, "Roll20"),
            SourceFormat::FoundryVtt => write!(f, "Foundry VTT"),
            SourceFormat::Json => write!(f, "JSON"),
        }
    }
}

impl std::fmt::Display for TargetFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetFormat::FoundryV10 => write!(f, "Foundry VTT v10"),
            TargetFormat::FoundryV11 => write!(f, "Foundry VTT v11"),
            TargetFormat::FoundryV12 => write!(f, "Foundry VTT v12"),
            TargetFormat::JsonExport => write!(f, "JSON Export"),
        }
    }
}

impl Campaign {
    /// Create a new empty campaign
    pub fn new(name: String, source_format: SourceFormat) -> Self {
        Self {
            metadata: CampaignMetadata {
                name,
                source_format,
                schema_version: None,
                created_at: Some(chrono::Utc::now()),
                stats: CampaignStats::default(),
                custom_properties: HashMap::new(),
            },
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal: Vec::new(),
            tables: Vec::new(),
            playlists: Vec::new(),
            macros: Vec::new(),
            folders: Vec::new(),
            assets: AssetCollection::default(),
        }
    }

    /// Update campaign statistics
    pub fn update_stats(&mut self) {
        let total_entities = self.actors.len()
            + self.scenes.len()
            + self.items.len()
            + self.journal.len()
            + self.tables.len()
            + self.playlists.len()
            + self.macros.len();

        let mut stats = CampaignStats { total_entities, ..Default::default() };

        stats
            .entity_counts
            .insert("actors".to_string(), self.actors.len());
        stats
            .entity_counts
            .insert("scenes".to_string(), self.scenes.len());
        stats
            .entity_counts
            .insert("items".to_string(), self.items.len());
        stats
            .entity_counts
            .insert("journal".to_string(), self.journal.len());
        stats
            .entity_counts
            .insert("tables".to_string(), self.tables.len());
        stats
            .entity_counts
            .insert("playlists".to_string(), self.playlists.len());
        stats
            .entity_counts
            .insert("macros".to_string(), self.macros.len());

        stats.total_assets = self.assets.references.len();
        stats.total_asset_size_mb = self.assets.stats.total_size_mb;

        self.metadata.stats = stats;
    }
}
