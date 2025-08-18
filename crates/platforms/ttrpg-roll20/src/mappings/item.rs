//! Roll20 Handout to TTRPG Item mappings

use crate::generated::handout::Handout;
use ttrpg_types::{TtrpgItem, ItemType, Ownership, PermissionLevel};
use serde_json::json;
use std::collections::HashMap;

impl From<Handout> for TtrpgItem {
    fn from(handout: Handout) -> Self {
        let mut system_data = HashMap::new();
        
        // Extract basic information
        let id = handout.id.to_string();
        let name = handout.name.to_string();
        let description = handout.notes;
        
        // Store Roll20-specific data
        if let Some(archived) = handout.archived {
            system_data.insert("archived".to_string(), json!(archived));
        }
        // Roll20 handouts don't have controlledby field, store as system data
        // system_data.insert("controlled_by".to_string(), json!("gm")); // Default owner
        // Handle permissions/ownership 
        let _ownership = handout.inplayerjournals.as_ref().map(|viewers| {
            let mut permissions = HashMap::new();
            for viewer in viewers.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
                permissions.insert(viewer, PermissionLevel::Observer);
            }
            let owner = "gm".to_string(); // Default since handouts don't have controlledby
            permissions.insert(owner.clone(), PermissionLevel::Owner);
            
            Ownership {
                owner,
                permissions,
            }
        });

        TtrpgItem {
            id,
            name,
            description,
            item_type: ItemType::Equipment, // Treat handouts as equipment/documents
            quantity: 1, // Handouts are singular
            weight: None, // Handouts have no weight
            cost: None, // Handouts have no cost
            rarity: None, // Handouts have no rarity
            image: None, // Roll20 handouts don't have avatar field
            damage: None, // Handouts don't deal damage
            armor_class: None, // Handouts don't provide AC
            requires_attunement: Some(false), // Handouts don't require attunement
            system_data,
        }
    }
}
