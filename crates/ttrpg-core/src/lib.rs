//! TTRPGConverter Core Library
//!
//! This crate provides the fundamental data structures, error handling, and domain logic
//! for the TTRPGConverter application. It serves as the foundation for all other crates
//! in the workspace.
//!
//! # Architecture
//!
//! The core library is organized around several key concepts:
//!
//! - **Types**: Core data structures representing TTRPG campaigns and entities
//! - **Errors**: Comprehensive error handling with contextual information
//! - **Services**: Abstract service definitions for dependency injection
//! - **Validation**: Data validation and integrity checking
//! - **Logging**: Logging utilities for tracking application events
//!
//! # Usage
//!
//! Most users should import from the [`prelude`] module which re-exports the most
//! commonly used types and functions:
//!
//! ```rust
//! use ttrpg_core::prelude::*;
//!
//! let campaign = Campaign::new("My Campaign".to_string(), SourceFormat::Roll20);
//! ```

// Public modules
pub mod error;
pub mod logging;
pub mod services;
pub mod types;
pub mod validation;

// Re-exports for convenience
pub use error::{AssetError, AssetResult, ConversionError, ConversionResult, ErrorExt};
pub use logging::{LoggingConfig, RustLogger};
pub use services::{
    AssetService, IssueSeverity, LogLevel, LoggingService, ServiceManager, ValidationIssue,
    ValidationResult, ValidationService,
};
pub use types::{
    Actor, Campaign, EntityPermissions, Item, JournalEntry, PermissionLevel, Scene, SourceFormat,
    TargetFormat,
};
pub use validation::{ValidationContext, ValidationRules, Validator};

/// Common imports for this crate and dependent crates
///
/// This module provides a convenient way to import the most commonly used
/// types and traits from this crate. It follows the standard Rust pattern
/// of providing a prelude module for easy imports.
///
/// # Usage
///
/// Import everything you need with:
/// ```rust
/// use ttrpg_core::prelude::*;
/// ```
pub mod prelude {

    // Error handling
    pub use crate::error::{AssetError, AssetResult, ConversionError, ConversionResult, ErrorExt};

    // Core types
    pub use crate::types::{
        Actor, ActorImages, ActorType, AssetCollection, AssetReference, AssetStats, AssetType,
        AssetUsage, AttributeValue, AudioTrack, Campaign, CampaignMetadata, CampaignStats,
        EntityPermissions, Feature, FeatureType, FeatureUsage, Folder, GridConfig, GridType, Item,
        ItemProperties, ItemType, JournalEntry, Macro, MacroType, PermissionLevel, Playlist,
        Position, RechargeType, RollableTable, Scene, SceneDimensions, SourceFormat, Spell,
        SpellComponents, TableEntry, TargetFormat, Token, TokenSize, Wall, WallType,
    };

    // Service abstractions
    pub use crate::services::{AssetService, LoggingService, ServiceManager, ValidationService};

    // Validation
    pub use crate::validation::{
        ValidationContext, ValidationError, ValidationReport, ValidationRules, ValidationWarning,
        Validator,
    };

    // External dependencies commonly used
    pub use serde::{Deserialize, Serialize};
    pub use std::collections::HashMap;
    pub use std::path::PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;
    use prelude::*;

    #[test]
    fn test_campaign_creation() {
        let campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);
        assert_eq!(campaign.metadata.name, "Test Campaign");
        assert_eq!(campaign.metadata.source_format, SourceFormat::Roll20);
        assert!(campaign.actors.is_empty());
        assert!(campaign.scenes.is_empty());
    }

    #[test]
    fn test_campaign_stats_update() {
        let mut campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);

        // Add some test data
        campaign.actors.push(Actor {
            id: "test_actor".to_string(),
            name: "Test Actor".to_string(),
            actor_type: ActorType::Pc,
            images: ActorImages::default(),
            attributes: HashMap::new(),
            items: Vec::new(),
            spells: Vec::new(),
            features: Vec::new(),
            biography: String::new(),
            notes: String::new(),
            permissions: EntityPermissions::default(),
            source_data: HashMap::new(),
        });

        campaign.update_stats();

        assert_eq!(campaign.metadata.stats.total_entities, 1);
        assert_eq!(campaign.metadata.stats.entity_counts.get("actors"), Some(&1));
    }

    #[test]
    fn test_error_handling() {
        let error = ConversionError::file_not_found("/test/path", "test context");
        assert!(error.to_string().contains("File not found"));

        let validation_error = ConversionError::validation("Actor", "Missing name");
        assert!(validation_error.to_string().contains("Validation failed"));
    }

    #[test]
    fn test_format_display() {
        assert_eq!(SourceFormat::Roll20.to_string(), "Roll20");
        assert_eq!(TargetFormat::FoundryV12.to_string(), "Foundry VTT v12");
    }
}
