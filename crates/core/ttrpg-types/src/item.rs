//! Item data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::damage::{DamageInfo, ArmorClass};

/// Core item data structure - unified representation across all systems  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtrpgItem {
    /// Item unique identifier
    pub id: String,
    /// Item name
    pub name: String,
    /// Item description
    pub description: Option<String>,
    /// Item type
    pub item_type: ItemType,
    /// Item quantity
    pub quantity: u32,
    /// Item weight
    pub weight: Option<f64>,
    /// Item cost
    pub cost: Option<Currency>,
    /// Item rarity
    pub rarity: Option<String>,
    /// Item image
    pub image: Option<String>,
    /// Damage information (for weapons)
    pub damage: Option<DamageInfo>,
    /// Armor class (for armor/shields)
    pub armor_class: Option<ArmorClass>,
    /// Requires attunement
    pub requires_attunement: Option<bool>,
    /// System-specific data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Item types across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Equipment,
    Consumable,
    Tool,
    Treasure,
    Spell,
    Feat,
}

/// Currency representation
#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct Currency {
    /// Currency value
    pub value: f64,
    /// Currency denomination (gp, sp, cp, etc.)
    pub denomination: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ttrpg_item_creation() {
        let item = TtrpgItem {
            id: "sword_001".to_string(),
            name: "Longsword".to_string(),
            description: Some("A finely crafted longsword".to_string()),
            item_type: ItemType::Weapon,
            quantity: 1,
            weight: Some(3.0),
            cost: Some(Currency {
                value: 15.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Common".to_string()),
            image: Some("longsword.png".to_string()),
            damage: Some(DamageInfo {
                dice: "1d8".to_string(),
                damage_type: crate::enums::DamageType::Slashing,
                bonus: None,
                versatile: Some("1d10".to_string()),
            }),
            armor_class: None,
            requires_attunement: Some(false),
            system_data: HashMap::new(),
        };

        assert_eq!(item.id, "sword_001");
        assert_eq!(item.name, "Longsword");
        assert_eq!(item.quantity, 1);
        assert_eq!(item.weight, Some(3.0));
        matches!(item.item_type, ItemType::Weapon);
        assert_eq!(item.cost.as_ref().unwrap().value, 15.0);
        assert_eq!(item.rarity, Some("Common".to_string()));
    }

    #[test]
    fn test_item_type_variants() {
        let items = vec![
            (ItemType::Weapon, "Weapon"),
            (ItemType::Armor, "Armor"),
            (ItemType::Equipment, "Equipment"),
            (ItemType::Consumable, "Consumable"),
            (ItemType::Tool, "Tool"),
            (ItemType::Treasure, "Treasure"),
            (ItemType::Spell, "Spell"),
            (ItemType::Feat, "Feat"),
        ];

        for (item_type, expected_name) in items {
            let item = TtrpgItem {
                id: "test_item".to_string(),
                name: expected_name.to_string(),
                description: None,
                item_type,
                quantity: 1,
                weight: None,
                cost: None,
                rarity: None,
                image: None,
                damage: None,
                armor_class: None,
                requires_attunement: None,
                system_data: HashMap::new(),
            };

            assert_eq!(item.name, expected_name);
        }
    }

    #[test]
    fn test_currency_functionality() {
        let gold = Currency {
            value: 100.0,
            denomination: "gp".to_string(),
        };
        
        assert_eq!(gold.value, 100.0);
        assert_eq!(gold.denomination, "gp");

        let silver = Currency {
            value: 50.0,
            denomination: "sp".to_string(),
        };
        
        assert_eq!(silver.value, 50.0);
        assert_eq!(silver.denomination, "sp");

        // Test fractional values
        let copper = Currency {
            value: 0.01,
            denomination: "cp".to_string(),
        };
        
        assert!(copper.value > 0.0);
    }

    #[test]
    fn test_item_serialization() {
        let item = TtrpgItem {
            id: "test_item".to_string(),
            name: "Test Item".to_string(),
            description: Some("A test item".to_string()),
            item_type: ItemType::Equipment,
            quantity: 5,
            weight: Some(2.5),
            cost: Some(Currency {
                value: 25.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Uncommon".to_string()),
            image: None,
            damage: None,
            armor_class: None,
            requires_attunement: None,
            system_data: HashMap::new(),
        };

        let serialized = serde_json::to_string(&item).unwrap();
        let deserialized: TtrpgItem = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(item.id, deserialized.id);
        assert_eq!(item.name, deserialized.name);
        assert_eq!(item.quantity, deserialized.quantity);
        assert_eq!(item.weight, deserialized.weight);
        assert_eq!(item.cost.as_ref().unwrap().value, deserialized.cost.as_ref().unwrap().value);
    }

    #[test]
    fn test_consumable_items() {
        let potion = TtrpgItem {
            id: "potion_healing".to_string(),
            name: "Potion of Healing".to_string(),
            description: Some("Restores 2d4+2 hit points".to_string()),
            item_type: ItemType::Consumable,
            quantity: 3,
            weight: Some(0.5),
            cost: Some(Currency {
                value: 50.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Common".to_string()),
            image: None,
            damage: None,
            armor_class: None,
            requires_attunement: Some(false),
            system_data: HashMap::new(),
        };

        matches!(potion.item_type, ItemType::Consumable);
        assert_eq!(potion.quantity, 3);
        assert_eq!(potion.weight, Some(0.5));
    }

    #[test]
    fn test_item_with_system_data() {
        let mut system_data = HashMap::new();
        system_data.insert("dnd5e_damage_dice".to_string(), serde_json::json!("1d8"));
        system_data.insert("dnd5e_damage_type".to_string(), serde_json::json!("slashing"));
        system_data.insert("pathfinder_hardness".to_string(), serde_json::json!(10));

        let weapon = TtrpgItem {
            id: "magic_sword".to_string(),
            name: "Magic Sword +1".to_string(),
            description: Some("A magically enhanced sword".to_string()),
            item_type: ItemType::Weapon,
            quantity: 1,
            weight: Some(3.0),
            cost: Some(Currency {
                value: 1000.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Rare".to_string()),
            image: Some("magic_sword.png".to_string()),
            damage: Some(DamageInfo {
                dice: "1d8+1".to_string(),
                damage_type: crate::enums::DamageType::Slashing,
                bonus: Some(1),
                versatile: None,
            }),
            armor_class: None,
            requires_attunement: Some(true),
            system_data,
        };

        assert_eq!(weapon.system_data.len(), 3);
        assert_eq!(weapon.system_data.get("dnd5e_damage_dice"), Some(&serde_json::json!("1d8")));
        assert_eq!(weapon.system_data.get("pathfinder_hardness"), Some(&serde_json::json!(10)));
    }

    #[test]
    fn test_treasure_items() {
        let gem = TtrpgItem {
            id: "ruby_001".to_string(),
            name: "Ruby".to_string(),
            description: Some("A brilliant red ruby".to_string()),
            item_type: ItemType::Treasure,
            quantity: 1,
            weight: Some(0.1),
            cost: Some(Currency {
                value: 500.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Very Rare".to_string()),
            image: Some("ruby.png".to_string()),
            damage: None,
            armor_class: None,
            requires_attunement: Some(false),
            system_data: HashMap::new(),
        };

        matches!(gem.item_type, ItemType::Treasure);
        assert_eq!(gem.cost.as_ref().unwrap().value, 500.0);
        assert_eq!(gem.weight, Some(0.1));
    }

    #[test]
    fn test_armor_items() {
        let armor = TtrpgItem {
            id: "chainmail_001".to_string(),
            name: "Chain Mail".to_string(),
            description: Some("Made of interlocking metal rings".to_string()),
            item_type: ItemType::Armor,
            quantity: 1,
            weight: Some(55.0),
            cost: Some(Currency {
                value: 75.0,
                denomination: "gp".to_string(),
            }),
            rarity: Some("Common".to_string()),
            image: None,
            damage: None,
            armor_class: Some(ArmorClass {
                base: 16,
                dex_bonus: None,
                max_dex: Some(0),
                formula: Some("16".to_string()),
            }),
            requires_attunement: Some(false),
            system_data: HashMap::new(),
        };

        matches!(armor.item_type, ItemType::Armor);
        assert_eq!(armor.weight, Some(55.0));
        assert_eq!(armor.cost.as_ref().unwrap().value, 75.0);
    }
}