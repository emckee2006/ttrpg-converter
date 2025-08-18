//! Character mappings for Pathbuilder PF2e

use crate::schemas::custom_ancestry::CustomAncestry;
use crate::schemas::custom_background::CustomBackground;
use ttrpg_types::character::TtrpgCharacter;
use ttrpg_types::enums::ActorType;
use std::collections::HashMap;
use serde_json;

/// Convert Pathbuilder CustomAncestry to unified TtrpgCharacter
impl From<CustomAncestry> for TtrpgCharacter {
    fn from(ancestry: CustomAncestry) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("size".to_string(), serde_json::json!(format!("{:?}", ancestry.size)));
        system_data.insert("timestamp".to_string(), serde_json::json!(format!("{:?}", ancestry.timestamp)));
        
        Self {
            id: String::from(ancestry.id),
            name: String::from(ancestry.name),
            biography: Some(String::from(ancestry.description)),
            level: Some(1),
            actor_type: Some(ActorType::Character),
            image: None,
            permissions: None,
            experience_points: None,
            inspiration: None,
            notes: None,
            hit_points: None,
            abilities: Vec::new(),
            skills: Vec::new(),
            items: Vec::new(),
            spells: Vec::new(),
            armor_class: None,
            system_data,
        }
    }
}

/// Convert Pathbuilder CustomBackground to unified TtrpgCharacter  
impl From<CustomBackground> for TtrpgCharacter {
    fn from(background: CustomBackground) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("timestamp".to_string(), serde_json::json!(format!("{:?}", background.timestamp)));
        
        Self {
            id: String::from(background.id),
            name: String::from(background.name),
            biography: Some(String::from(background.description)),
            level: Some(1),
            actor_type: Some(ActorType::Character),
            image: None,
            permissions: None,
            experience_points: None,
            inspiration: None,
            notes: None,
            hit_points: None,
            abilities: Vec::new(),
            skills: Vec::new(),
            items: Vec::new(),
            spells: Vec::new(),
            armor_class: None,
            system_data,
        }
    }
}
