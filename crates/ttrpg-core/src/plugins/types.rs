//! Plugin System Types
//!
//! Core data structures used throughout the plugin system for campaign conversion,
//! asset processing, and format generation.

use crate::types::{Actor, Item, JournalEntry, Scene};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Universal campaign representation that all plugins work with
///
/// This is the common data structure that input plugins populate
/// and output plugins consume. It provides a system-agnostic
/// representation of TTRPG campaign data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalCampaign {
    /// Campaign metadata
    pub metadata: CampaignMetadata,

    /// Detected or specified game system
    pub game_system: GameSystem,

    /// All actors (PCs, NPCs, monsters)
    pub actors: Vec<Actor>,

    /// All scenes/maps
    pub scenes: Vec<Scene>,

    /// All items/equipment
    pub items: Vec<Item>,

    /// All journal entries/handouts
    pub journal_entries: Vec<JournalEntry>,

    /// Macros and automation
    pub macros: Vec<Macro>,

    /// Playlists and audio
    pub playlists: Vec<Playlist>,

    /// Combat encounters
    pub encounters: Vec<Encounter>,

    /// Campaign-specific settings and preferences
    pub settings: CampaignSettings,

    /// Conversion tracking and metadata
    pub conversion_notes: Vec<ConversionNote>,
}

impl UniversalCampaign {
    /// Create a new empty universal campaign
    pub fn new() -> Self {
        Self {
            metadata: CampaignMetadata::default(),
            game_system: GameSystem::Unknown,
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal_entries: Vec::new(),
            macros: Vec::new(),
            playlists: Vec::new(),
            encounters: Vec::new(),
            settings: CampaignSettings::default(),
            conversion_notes: Vec::new(),
        }
    }

    /// Get campaign statistics
    pub fn stats(&self) -> CampaignStats {
        CampaignStats {
            total_actors: self.actors.len(),
            total_scenes: self.scenes.len(),
            total_items: self.items.len(),
            total_journal_entries: self.journal_entries.len(),
            total_macros: self.macros.len(),
            total_encounters: self.encounters.len(),
            game_system: self.game_system.clone(),
        }
    }
}

impl Default for UniversalCampaign {
    fn default() -> Self {
        Self::new()
    }
}

/// Campaign metadata and system detection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetadata {
    /// Campaign title
    pub title: String,

    /// Campaign description
    pub description: Option<String>,

    /// Original source format
    pub source_format: SourceFormat,

    /// Detected game system
    pub detected_system: Option<GameSystem>,

    /// System detection confidence (0.0 to 1.0)
    pub system_confidence: f64,

    /// Source file/directory path
    pub source_path: Option<PathBuf>,

    /// Creation timestamp
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Last modified timestamp  
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Source application version
    pub source_version: Option<String>,
}

impl Default for CampaignMetadata {
    fn default() -> Self {
        Self {
            title: "Untitled Campaign".to_string(),
            description: None,
            source_format: SourceFormat::Unknown,
            detected_system: None,
            system_confidence: 0.0,
            source_path: None,
            created_at: None,
            modified_at: None,
            source_version: None,
        }
    }
}

/// Supported game systems for conversion
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameSystem {
    /// Dungeons & Dragons 5th Edition
    DnD5e,
    /// Pathfinder 2nd Edition
    Pathfinder2e,
    /// Pathfinder 1st Edition
    Pathfinder1e,
    /// Call of Cthulhu 7th Edition
    CallOfCthulhu7e,
    /// Savage Worlds Adventure Edition
    SavageWorlds,
    /// GURPS 4th Edition
    GURPS4e,
    /// Fate Core/Accelerated
    Fate,
    /// Generic/Unknown system
    Unknown,
    /// Custom system with name
    Custom(String),
}

impl std::fmt::Display for GameSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameSystem::DnD5e => write!(f, "D&D 5e"),
            GameSystem::Pathfinder2e => write!(f, "Pathfinder 2e"),
            GameSystem::Pathfinder1e => write!(f, "Pathfinder 1e"),
            GameSystem::CallOfCthulhu7e => write!(f, "Call of Cthulhu 7e"),
            GameSystem::SavageWorlds => write!(f, "Savage Worlds"),
            GameSystem::GURPS4e => write!(f, "GURPS 4e"),
            GameSystem::Fate => write!(f, "Fate"),
            GameSystem::Unknown => write!(f, "Unknown"),
            GameSystem::Custom(name) => write!(f, "Custom: {name}"),
        }
    }
}

/// Source format identification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceFormat {
    /// Roll20 campaign export (.zip or extracted)
    Roll20,
    /// Foundry VTT world or module
    FoundryVTT,
    /// Fantasy Grounds campaign
    FantasyGrounds,
    /// D&D Beyond campaign export
    DNDBeyond,
    /// Generic JSON format
    GenericJson,
    /// Unknown source format
    Unknown,
}

/// Output format specification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Foundry VTT World
    FoundryWorld,
    /// Foundry VTT Module  
    FoundryModule,
    /// Pathbuilder character JSON
    PathbuilderJson,
    /// D&D Beyond compatible JSON
    DNDBeyondJson,
    /// HeroLab portfolio
    HeroLabJson,
    /// Fantasy Grounds campaign
    FantasyGroundsXML,
    /// Professional PDF character sheets
    PDFCharacterSheets,
    /// Campaign compendium PDF
    PDFCampaignBook,
    /// Universal JSON format
    UniversalJson,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::FoundryWorld => write!(f, "Foundry World"),
            OutputFormat::FoundryModule => write!(f, "Foundry Module"),
            OutputFormat::PathbuilderJson => write!(f, "Pathbuilder JSON"),
            OutputFormat::DNDBeyondJson => write!(f, "D&D Beyond JSON"),
            OutputFormat::HeroLabJson => write!(f, "HeroLab JSON"),
            OutputFormat::FantasyGroundsXML => write!(f, "Fantasy Grounds XML"),
            OutputFormat::PDFCharacterSheets => write!(f, "PDF Character Sheets"),
            OutputFormat::PDFCampaignBook => write!(f, "PDF Campaign Book"),
            OutputFormat::UniversalJson => write!(f, "Universal JSON"),
        }
    }
}

/// Asset information for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInfo {
    /// Original asset URL or path
    pub source: String,

    /// Asset type (image, audio, etc.)
    pub asset_type: AssetType,

    /// Local file path if downloaded
    pub local_path: Option<PathBuf>,

    /// Asset metadata
    pub metadata: AssetMetadata,
}

/// Types of campaign assets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    /// Character portraits and tokens
    CharacterArt,
    /// Scene/map backgrounds  
    MapBackground,
    /// Token images
    TokenImage,
    /// Journal handout images
    HandoutImage,
    /// Audio files
    Audio,
    /// Video files
    Video,
    /// Generic file attachment
    Attachment,
}

/// Asset metadata for processing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetMetadata {
    /// File size in bytes
    pub file_size: Option<u64>,

    /// File format/extension
    pub format: Option<String>,

    /// Image dimensions if applicable
    pub dimensions: Option<(u32, u32)>,

    /// Processing options
    pub processing_hints: HashMap<String, String>,
}

/// Processed asset ready for output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedAsset {
    /// Original asset info
    pub original: AssetInfo,

    /// Processed file path
    pub processed_path: PathBuf,

    /// Processing applied
    pub processing_applied: Vec<String>,

    /// Target mappings for different formats
    pub target_mappings: HashMap<OutputFormat, String>,
}

impl From<AssetInfo> for ProcessedAsset {
    fn from(asset: AssetInfo) -> Self {
        Self {
            processed_path: asset
                .local_path
                .clone()
                .unwrap_or_else(|| PathBuf::from(&asset.source)),
            original: asset,
            processing_applied: Vec::new(),
            target_mappings: HashMap::new(),
        }
    }
}

/// Output generation bundle
#[derive(Debug, Clone)]
pub struct OutputBundle {
    /// Generated files (path -> content)
    pub files: HashMap<String, String>,

    /// Database files (path -> bytes)
    pub databases: HashMap<String, Vec<u8>>,

    /// Asset files (target path -> source path)
    pub assets: HashMap<String, PathBuf>,

    /// Bundle metadata
    pub metadata: OutputMetadata,
}

impl OutputBundle {
    /// Create new empty output bundle
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            databases: HashMap::new(),
            assets: HashMap::new(),
            metadata: OutputMetadata::default(),
        }
    }

    /// Add a text file to the bundle
    pub fn add_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
        self.files.insert(path.into(), content.into());
    }

    /// Add a database file to the bundle
    pub fn add_database(&mut self, path: impl Into<String>, data: Vec<u8>) {
        self.databases.insert(path.into(), data);
    }

    /// Add an asset file to the bundle
    pub fn add_asset(&mut self, target_path: impl Into<String>, source_path: PathBuf) {
        self.assets.insert(target_path.into(), source_path);
    }
}

impl Default for OutputBundle {
    fn default() -> Self {
        Self::new()
    }
}

/// Output bundle metadata
#[derive(Debug, Clone, Default)]
pub struct OutputMetadata {
    /// Generation timestamp
    pub generated_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Generator info
    pub generator: Option<String>,

    /// Target format
    pub format: Option<OutputFormat>,

    /// Generation statistics
    pub stats: Option<GenerationStats>,
}

/// Output generation statistics
#[derive(Debug, Clone, Default)]
pub struct GenerationStats {
    /// Number of files generated
    pub files_generated: usize,

    /// Number of assets processed
    pub assets_processed: usize,

    /// Total processing time
    pub processing_time: Option<std::time::Duration>,
}

/// Campaign statistics for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignStats {
    pub total_actors: usize,
    pub total_scenes: usize,
    pub total_items: usize,
    pub total_journal_entries: usize,
    pub total_macros: usize,
    pub total_encounters: usize,
    pub game_system: GameSystem,
}

/// Output configuration for plugins
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Target output format
    pub format: OutputFormat,

    /// Output subdirectory
    pub subdirectory: String,

    /// Format-specific options
    pub foundry_options: Option<FoundryConfig>,
    pub pdf_options: Option<PDFConfig>,

    /// Write options
    pub write_options: WriteOptions,
}

/// Foundry-specific configuration options
#[derive(Debug, Clone)]
pub struct FoundryConfig {
    pub database_type: FoundryDatabaseType,
    pub output_type: FoundryOutputType,
    pub version: FoundryVersion,
    pub world_settings: Option<FoundryWorldSettings>,
    pub module_settings: Option<FoundryModuleSettings>,
}

/// Foundry database type
#[derive(Debug, Clone, PartialEq)]
pub enum FoundryDatabaseType {
    LevelDB, // v10+
    NeDB,    // v9 and below
}

/// Foundry output type
#[derive(Debug, Clone, PartialEq)]
pub enum FoundryOutputType {
    World,  // Complete world
    Module, // Compendium module
}

/// Foundry version for schema compatibility
#[derive(Debug, Clone, PartialEq)]
pub enum FoundryVersion {
    V9,
    V10,
    V11,
    V12,
}

/// Foundry world settings
#[derive(Debug, Clone, Default)]
pub struct FoundryWorldSettings {
    pub world_title: Option<String>,
    pub world_description: Option<String>,
    pub system: Option<String>,
}

/// Foundry module settings
#[derive(Debug, Clone, Default)]
pub struct FoundryModuleSettings {
    pub module_id: Option<String>,
    pub module_title: Option<String>,
    pub module_description: Option<String>,
}

/// PDF generation options
#[derive(Debug, Clone, Default)]
pub struct PDFConfig {
    pub include_character_sheets: bool,
    pub include_campaign_notes: bool,
    pub page_format: PDFPageFormat,
    pub font_family: Option<String>,
}

/// PDF page format options
#[derive(Debug, Clone, Default)]
pub enum PDFPageFormat {
    #[default]
    Letter,
    A4,
    Legal,
    Tabloid,
}

/// File writing options
#[derive(Debug, Clone, Default)]
pub struct WriteOptions {
    pub overwrite_existing: bool,
    pub create_directories: bool,
    pub preserve_permissions: bool,
}

/// Conversion tracking note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionNote {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub category: ConversionCategory,
    pub message: String,
    pub affected_entity: Option<String>,
}

/// Categories of conversion notes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversionCategory {
    Info,
    Warning,
    Error,
    SystemConversion,
    AssetProcessing,
    FormatLimitation,
}

// Placeholder types for entities not yet fully defined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub name: String,
    pub command: String,
    pub visible_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<AudioTrack>,
    pub shuffle: bool,
    pub repeat: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    pub name: String,
    pub source: String,
    pub volume: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Encounter {
    pub name: String,
    pub description: Option<String>,
    pub participants: Vec<String>, // Actor IDs
    pub initiative_order: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignSettings {
    pub default_token_vision: bool,
    pub grid_type: GridType,
    pub grid_size: u32,
    pub default_permissions: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum GridType {
    #[default]
    Square,
    Hex,
    Gridless,
}
