//! Campaign data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core campaign data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    /// Campaign unique identifier
    pub id: String,
    /// Campaign name
    pub name: String,
    /// Campaign description
    pub description: Option<String>,
    /// Game system
    pub system: String,
    /// Campaign metadata
    pub metadata: CampaignMetadata,
    /// Characters in the campaign
    pub characters: Vec<String>, // Character IDs
    /// Scenes in the campaign
    pub scenes: Vec<String>, // Scene IDs
    /// Journal entries
    pub journal_entries: Vec<String>, // Journal entry IDs
    /// System-specific additional data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Campaign metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignMetadata {
    /// Creation timestamp
    pub created_at: String,
    /// Last modified timestamp
    pub modified_at: String,
    /// Version information
    pub version: Option<String>,
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_campaign_creation() {
        let campaign = Campaign {
            id: "campaign_001".to_string(),
            name: "Lost Mine of Phandelver".to_string(),
            description: Some("A classic D&D 5e starter adventure".to_string()),
            system: "dnd5e".to_string(),
            metadata: CampaignMetadata {
                created_at: "2024-01-15T10:30:00Z".to_string(),
                modified_at: "2024-01-20T15:45:00Z".to_string(),
                version: Some("1.2".to_string()),
                properties: HashMap::new(),
            },
            characters: vec!["char_001".to_string(), "char_002".to_string()],
            scenes: vec!["scene_tavern".to_string(), "scene_goblin_cave".to_string()],
            journal_entries: vec!["journal_session1".to_string()],
            system_data: HashMap::new(),
        };

        assert_eq!(campaign.name, "Lost Mine of Phandelver");
        assert_eq!(campaign.characters.len(), 2);
        assert!(campaign.metadata.version.is_some());
    }

    #[test]
    fn test_campaign_metadata() {
        let metadata = CampaignMetadata {
            created_at: "2024-01-01T00:00:00Z".to_string(),
            modified_at: "2024-01-15T12:00:00Z".to_string(),
            version: Some("2.1.0".to_string()),
            properties: HashMap::new(),
        };

        assert_eq!(metadata.version, Some("2.1.0".to_string()));
        assert!(metadata.properties.is_empty());
    }

    #[test]
    fn test_campaign_with_system_data() {
        let mut system_data = HashMap::new();
        system_data.insert("dnd5e_setting".to_string(), serde_json::json!("Forgotten Realms"));
        system_data.insert("experience_type".to_string(), serde_json::json!("milestone"));
        system_data.insert("house_rules".to_string(), serde_json::json!(["no multiclassing", "max hp at first level"]));

        let campaign = Campaign {
            id: "homebrew_001".to_string(),
            name: "Dragon Heist Homebrew".to_string(),
            description: Some("Custom campaign with house rules".to_string()),
            system: "dnd5e".to_string(),
            metadata: CampaignMetadata {
                created_at: "2024-01-01T00:00:00Z".to_string(),
                modified_at: "2024-01-01T00:00:00Z".to_string(),
                version: Some("1.0".to_string()),
                properties: HashMap::new(),
            },
            characters: vec![],
            scenes: vec![],
            journal_entries: vec![],
            system_data,
        };

        assert_eq!(campaign.system_data.len(), 3);
        assert!(campaign.system_data.contains_key("dnd5e_setting"));
        assert!(campaign.system_data.contains_key("house_rules"));
    }

    #[test]
    fn test_campaign_serialization() {
        let campaign = Campaign {
            id: "serialize_test".to_string(),
            name: "Test Campaign".to_string(),
            description: Some("For testing serialization".to_string()),
            system: "dnd5e".to_string(),
            metadata: CampaignMetadata {
                created_at: "2024-01-01T00:00:00Z".to_string(),
                modified_at: "2024-01-01T00:00:00Z".to_string(),
                version: Some("1.0".to_string()),
                properties: HashMap::new(),
            },
            characters: vec!["char_test".to_string()],
            scenes: vec![],
            journal_entries: vec![],
            system_data: HashMap::new(),
        };

        let serialized = serde_json::to_string(&campaign).unwrap();
        let deserialized: Campaign = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(campaign.id, deserialized.id);
        assert_eq!(campaign.name, deserialized.name);
        assert_eq!(campaign.characters.len(), deserialized.characters.len());
    }
}