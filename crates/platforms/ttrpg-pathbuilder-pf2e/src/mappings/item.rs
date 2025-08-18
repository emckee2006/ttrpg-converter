//! Item mappings for Pathbuilder PF2e

use crate::schemas::custom_item::CustomItem;
use ttrpg_types::item::{TtrpgItem, ItemType};
use std::collections::HashMap;
use serde_json;

/// Convert Pathbuilder CustomItem to unified TtrpgItem
impl From<CustomItem> for TtrpgItem {
    fn from(item: CustomItem) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("timestamp".to_string(), serde_json::json!(format!("{:?}", item.timestamp)));
        
        Self {
            id: String::from(item.id),
            name: String::from(item.name),
            description: Some(String::from(item.description)),
            item_type: ItemType::Equipment, // Default - TODO: parse from category field
            quantity: 1,
            weight: None, // TODO: Parse from bulk field
            cost: None,   // TODO: Parse from price field
            rarity: None,
            image: None,
            damage: None, // TODO: Parse from damage field
            armor_class: None, // TODO: Parse from ac field
            requires_attunement: Some(false),
            system_data,
        }
    }
}
