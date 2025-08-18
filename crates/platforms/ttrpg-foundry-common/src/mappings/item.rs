use crate::generated::item::{FoundryPf2eItem, FoundryPf2eItemType};
use ttrpg_types::{TtrpgItem, ItemType, Currency, ArmorClass};
use serde_json::{json, Value};
use std::collections::HashMap;

impl From<FoundryPf2eItem> for TtrpgItem {
    fn from(item: FoundryPf2eItem) -> Self {
        let mut system_data = HashMap::new();

        // Extract basic information
        let id = item.id.to_string();
        let name = item.name.to_string();
        let image = item.img;

        // Convert item type
        let item_type = foundry_item_type_to_ttrpg(match item.type_ {
            FoundryPf2eItemType::Weapon => "weapon",
            FoundryPf2eItemType::Armor => "armor",
            FoundryPf2eItemType::Equipment => "equipment",
            FoundryPf2eItemType::Consumable => "consumable",
            FoundryPf2eItemType::Spell => "spell",
            FoundryPf2eItemType::Feat => "equipment",
            FoundryPf2eItemType::Action => "equipment",
            FoundryPf2eItemType::Backpack => "equipment",
            FoundryPf2eItemType::Treasure => "equipment",
            FoundryPf2eItemType::Kit => "equipment",
            FoundryPf2eItemType::Ancestry => "equipment",
            FoundryPf2eItemType::Background => "equipment",
            FoundryPf2eItemType::Class => "equipment",
            FoundryPf2eItemType::Lore => "equipment",
        });

        // Extract nested system data
        let _system = &item.system;

        // Extract description from nested path
        let description = None; // TODO: Extract from system struct

        // Extract basic item properties
        let quantity = 1; // TODO: Extract from system struct
        let weight: Option<f64> = None; // TODO: Extract from system struct
        let cost: Option<Currency> = None; // TODO: Extract from system struct

        // Extract rarity
        let rarity: Option<String> = None; // TODO: Extract from system struct

        // Extract armor class for armor items
        let armor_class: Option<ArmorClass> = None; // TODO: Extract from system struct

        // Extract attunement requirement
        let requires_attunement = None; // TODO: Extract from system struct

        // Store Foundry-specific data in system_data
        // TODO: Extract additional properties from system struct when schema is properly mapped

        // Store ownership and flags
        system_data.insert("ownership".to_string(), json!(item.ownership));
        system_data.insert("flags".to_string(), json!(item.flags));

        TtrpgItem {
            id,
            name,
            description,
            item_type,
            quantity,
            weight,
            cost,
            rarity,
            image,
            damage: None, // Damage info stored in system_data
            armor_class,
            requires_attunement,
            system_data,
        }
    }
}

pub fn foundry_item_type_to_ttrpg(foundry_type: &str) -> ItemType {
    match foundry_type {
        "weapon" => ItemType::Weapon,
        "armor" => ItemType::Armor,
        "shield" => ItemType::Armor, // Shields are armor in TTRPG types
        "consumable" => ItemType::Consumable,
        "tool" => ItemType::Tool,
        "loot" => ItemType::Treasure,
        "backpack" => ItemType::Equipment, // Containers are equipment
        "class" | "feat" | "spell" => ItemType::Equipment,
        _ => ItemType::Equipment, // Default fallback
    }
}

fn extract_nested_string(data: &Value, path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;
    
    for part in parts {
        current = current.get(part)?;
    }
    
    current.as_str().map(|s| s.to_string())
}

fn extract_nested_number<T>(data: &Value, path: &str) -> Option<T> 
where 
    T: serde::de::DeserializeOwned,
{
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;
    
    for part in parts {
        current = current.get(part)?;
    }
    
    serde_json::from_value(current.clone()).ok()
}
