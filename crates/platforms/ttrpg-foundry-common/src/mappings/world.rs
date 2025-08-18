use serde_json::json;
use std::collections::HashMap;
use ttrpg_types::TtrpgItem;
use crate::generated::world::FoundryVttWorld;

impl From<FoundryVttWorld> for TtrpgItem {
    fn from(world: FoundryVttWorld) -> Self {
        let mut system_data = HashMap::new();

        // Extract basic information
        let id = world.id.to_string();
        let name = format!("{:?}", world.title);
        let description = world.description;

        // Store Foundry world-specific data in system_data
        system_data.insert("game_system".to_string(), json!(world.system));
        system_data.insert("core_version".to_string(), json!(world.core_version));
        if let Some(system_version) = world.system_version {
            system_data.insert("system_version".to_string(), json!(system_version));
        }
        if let Some(last_played) = world.last_played {
            system_data.insert("last_played".to_string(), json!(last_played));
        }
        if let Some(playtime) = world.playtime {
            system_data.insert("playtime".to_string(), json!(playtime));
        }
        if let Some(version) = world.version {
            system_data.insert("world_version".to_string(), json!(version));
        }
        if let Some(compatibility) = world.compatibility {
            system_data.insert("compatibility".to_string(), json!(compatibility));
        }
        // TODO: Add world-specific fields when they exist in schema

        // Treat world as a document item
        ttrpg_types::TtrpgItem {
            id,
            name,
            description,
            item_type: ttrpg_types::ItemType::Equipment, // World as document
            quantity: 1,
            weight: None,
            cost: None,
            rarity: None,
            image: None,
            damage: None,
            armor_class: None,
            requires_attunement: None,
            system_data,
        }
    }
}
