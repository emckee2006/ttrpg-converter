//! Common utility types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Journal entry data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtrpgJournal {
    /// Journal unique identifier
    pub id: String,
    /// Journal name
    pub name: String,
    /// Journal content
    pub content: Option<String>,
    /// Show to players
    pub show_to_players: bool,
    /// Journal pages
    pub pages: Option<Vec<JournalPage>>,
    /// System-specific additional data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Journal page types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalPage {
    /// Page ID
    pub id: String,
    /// Page name
    pub name: String,
    /// Page type
    pub page_type: JournalPageType,
    /// Page content
    pub content: String,
}

/// Journal page types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JournalPageType {
    Text,
    Image,
    PDF,
    Video,
}

/// Asset reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetReference {
    /// Asset path or URL
    pub path: String,
    /// Asset type
    pub asset_type: AssetType,
    /// Asset size in bytes
    pub size: Option<u64>,
}

/// Asset types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Audio,
    Video,
    PDF,
    Text,
}

/// Ownership/permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ownership {
    /// Owner ID
    pub owner: String,
    /// Permission levels for users
    pub permissions: HashMap<String, PermissionLevel>,
}

/// Permission levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionLevel {
    None,
    Limited,
    Observer,
    Owner,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ttrpg_journal_creation() {
        let journal = TtrpgJournal {
            id: "journal_001".to_string(),
            name: "Session Notes".to_string(),
            content: Some("The party entered the ancient ruins...".to_string()),
            show_to_players: true,
            pages: Some(vec![
                JournalPage {
                    id: "page_001".to_string(),
                    name: "Chapter 1: The Beginning".to_string(),
                    page_type: JournalPageType::Text,
                    content: "Our heroes meet in a tavern...".to_string(),
                }
            ]),
            system_data: HashMap::new(),
        };

        assert_eq!(journal.name, "Session Notes");
        assert!(journal.show_to_players);
        assert!(journal.pages.is_some());
        assert_eq!(journal.pages.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_journal_page_types() {
        let text_page = JournalPage {
            id: "text_page".to_string(),
            name: "Rules".to_string(),
            page_type: JournalPageType::Text,
            content: "House rules document".to_string(),
        };

        let image_page = JournalPage {
            id: "map_page".to_string(),
            name: "World Map".to_string(),
            page_type: JournalPageType::Image,
            content: "world_map.png".to_string(),
        };

        matches!(text_page.page_type, JournalPageType::Text);
        matches!(image_page.page_type, JournalPageType::Image);
        assert_eq!(text_page.name, "Rules");
        assert_eq!(image_page.name, "World Map");
    }

    #[test]
    fn test_asset_reference_creation() {
        let asset = AssetReference {
            path: "images/creatures/dragon_red.png".to_string(),
            asset_type: AssetType::Image,
            size: Some(2048576), // 2MB
        };

        assert!(asset.path.contains("dragon_red.png"));
        matches!(asset.asset_type, AssetType::Image);
        assert_eq!(asset.size, Some(2048576));
    }

    #[test]
    fn test_asset_type_variants() {
        let types = vec![
            AssetType::Image,
            AssetType::Audio,
            AssetType::Video,
            AssetType::PDF,
            AssetType::Text,
        ];

        assert_eq!(types.len(), 5);
        matches!(types[0], AssetType::Image);
        matches!(types[1], AssetType::Audio);
        matches!(types[3], AssetType::PDF);
    }

    #[test]
    fn test_ownership_permissions() {
        let mut permissions = HashMap::new();
        permissions.insert("player_alice".to_string(), PermissionLevel::Observer);
        permissions.insert("player_bob".to_string(), PermissionLevel::Limited);
        permissions.insert("gm_carol".to_string(), PermissionLevel::Owner);

        let ownership = Ownership {
            owner: "master_gm".to_string(),
            permissions,
        };

        assert_eq!(ownership.owner, "master_gm");
        assert_eq!(ownership.permissions.len(), 3);
        
        let alice_perm = ownership.permissions.get("player_alice").unwrap();
        matches!(alice_perm, PermissionLevel::Observer);
    }

    #[test]
    fn test_permission_level_variants() {
        let levels = vec![
            PermissionLevel::None,
            PermissionLevel::Limited,
            PermissionLevel::Observer,
            PermissionLevel::Owner,
        ];

        assert_eq!(levels.len(), 4);
        matches!(levels[0], PermissionLevel::None);
        matches!(levels[3], PermissionLevel::Owner);
    }

    #[test]
    fn test_journal_serialization() {
        let journal = TtrpgJournal {
            id: "serialize_test".to_string(),
            name: "Test Journal".to_string(),
            content: Some("Test content".to_string()),
            show_to_players: false,
            pages: None,
            system_data: HashMap::new(),
        };

        let serialized = serde_json::to_string(&journal).unwrap();
        let deserialized: TtrpgJournal = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(journal.id, deserialized.id);
        assert_eq!(journal.name, deserialized.name);
        assert_eq!(journal.show_to_players, deserialized.show_to_players);
    }
}