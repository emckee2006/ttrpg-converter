//! Character data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::enums::ActorType;
use crate::common::Ownership;

/// Core character data structure - unified representation across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtrpgCharacter {
    /// Character unique identifier
    pub id: String,
    /// Character name
    pub name: String,
    /// Character biography/description
    pub biography: Option<String>,
    /// Character level
    pub level: Option<u32>,
    /// Character type (PC, NPC, etc.)
    pub actor_type: Option<ActorType>,
    /// Character image/avatar
    pub image: Option<String>,
    /// Character permissions/ownership
    pub permissions: Option<Ownership>,
    /// Experience points
    pub experience_points: Option<u32>,
    /// Inspiration (D&D5e)
    pub inspiration: Option<bool>,
    /// GM notes
    pub notes: Option<String>,
    /// Hit points configuration
    pub hit_points: Option<HitPoints>,
    /// Ability scores (Strength, Dexterity, etc.)
    pub abilities: Vec<Ability>,
    /// Character skills
    pub skills: Vec<Skill>,
    /// Character equipment/items
    pub items: Vec<String>, // Item IDs
    /// Character spells
    pub spells: Vec<String>, // Spell IDs
    /// Armor class
    pub armor_class: Option<u8>,
    /// System-specific data containers
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Hit points tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitPoints {
    /// Current hit points
    pub current: i32,
    /// Maximum hit points
    pub max: i32,
    /// Temporary hit points
    pub temp: Option<i32>,
}

/// Ability score representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    /// Ability type (Strength, Dexterity, etc.)
    pub ability_type: AbilityType,
    /// Base ability score
    pub score: u8,
    /// Calculated modifier
    pub modifier: i8,
    /// Proficiency bonus applied
    pub proficiency: Option<f32>,
}

/// Character skill representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    /// Skill name
    pub name: String,
    /// Base ability for this skill
    pub ability: AbilityType,
    /// Proficiency level
    pub proficiency: Option<f32>,
    /// Total skill bonus
    pub bonus: Option<i8>,
}

/// Ability types across all game systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

/// Character save throws
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveThrows {
    /// Fortitude/Constitution saves
    pub fortitude: Option<i8>,
    /// Reflex/Dexterity saves
    pub reflex: Option<i8>,
    /// Will/Wisdom saves
    pub will: Option<i8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ttrpg_character_creation() {
        let character = TtrpgCharacter {
            id: "char_001".to_string(),
            actor_type: Some(ActorType::Character),
            image: Some("character.png".to_string()),
            permissions: None,
            experience_points: Some(6500),
            inspiration: Some(false),
            notes: Some("Test notes".to_string()),
            name: "Gandalf".to_string(),
            biography: Some("A powerful wizard".to_string()),
            level: Some(20),
            hit_points: Some(HitPoints {
                current: 100,
                max: 120,
                temp: Some(10),
            }),
            abilities: vec![
                Ability {
                    ability_type: AbilityType::Wisdom,
                    score: 18,
                    modifier: 4,
                    proficiency: Some(2.0),
                }
            ],
            skills: vec![
                Skill {
                    name: "Arcana".to_string(),
                    ability: AbilityType::Intelligence,
                    proficiency: Some(2.0),
                    bonus: Some(12),
                }
            ],
            items: vec!["staff_of_power".to_string()],
            spells: vec!["fireball".to_string(), "teleport".to_string()],
            armor_class: Some(15),
            system_data: HashMap::new(),
        };

        assert_eq!(character.id, "char_001");
        assert_eq!(character.name, "Gandalf");
        assert_eq!(character.level, Some(20));
        assert_eq!(character.hit_points.as_ref().unwrap().max, 120);
        assert_eq!(character.abilities.len(), 1);
        assert_eq!(character.skills.len(), 1);
        assert_eq!(character.spells.len(), 2);
    }

    #[test]
    fn test_hit_points_functionality() {
        let mut hp = HitPoints {
            current: 50,
            max: 100,
            temp: Some(5),
        };

        assert_eq!(hp.current, 50);
        assert_eq!(hp.max, 100);
        assert_eq!(hp.temp, Some(5));

        // Test damage application
        hp.current -= 20;
        assert_eq!(hp.current, 30);

        // Test healing
        hp.current = hp.max.min(hp.current + 30);
        assert_eq!(hp.current, 60);
    }

    #[test]
    fn test_ability_score_calculation() {
        let ability = Ability {
            ability_type: AbilityType::Strength,
            score: 16,
            modifier: 3, // (16-10)/2 = 3
            proficiency: Some(2.0),
        };

        assert_eq!(ability.score, 16);
        assert_eq!(ability.modifier, 3);
        assert_eq!(ability.proficiency, Some(2.0));
        
        // Test different ability types
        let wisdom = Ability {
            ability_type: AbilityType::Wisdom,
            score: 14,
            modifier: 2,
            proficiency: None,
        };
        
        matches!(wisdom.ability_type, AbilityType::Wisdom);
        assert_eq!(wisdom.modifier, 2);
    }

    #[test]
    fn test_skill_proficiency() {
        let skill = Skill {
            name: "Stealth".to_string(),
            ability: AbilityType::Dexterity,
            proficiency: Some(2.0),
            bonus: Some(8), // Dex modifier + proficiency
        };

        assert_eq!(skill.name, "Stealth");
        matches!(skill.ability, AbilityType::Dexterity);
        assert_eq!(skill.proficiency, Some(2.0));
        assert_eq!(skill.bonus, Some(8));
    }

    #[test]
    fn test_ability_type_variants() {
        let abilities = vec![
            AbilityType::Strength,
            AbilityType::Dexterity,
            AbilityType::Constitution,
            AbilityType::Intelligence,
            AbilityType::Wisdom,
            AbilityType::Charisma,
        ];

        assert_eq!(abilities.len(), 6);
        
        // Test each ability type serializes correctly
        for ability_type in abilities {
            let ability = Ability {
                ability_type: ability_type.clone(),
                score: 10,
                modifier: 0,
                proficiency: None,
            };
            let _serialized = serde_json::to_string(&ability).unwrap();
        }
    }

    #[test]
    fn test_save_throws() {
        let saves = SaveThrows {
            fortitude: Some(5),
            reflex: Some(3),
            will: Some(7),
        };

        assert_eq!(saves.fortitude, Some(5));
        assert_eq!(saves.reflex, Some(3));
        assert_eq!(saves.will, Some(7));

        // Test partial saves
        let partial_saves = SaveThrows {
            fortitude: Some(2),
            reflex: None,
            will: Some(4),
        };

        assert_eq!(partial_saves.reflex, None);
    }

    #[test]
    fn test_character_serialization() {
        let character = TtrpgCharacter {
            id: "char_002".to_string(),
            name: "Fighter".to_string(),
            biography: None,
            level: Some(1),
            actor_type: Some(ActorType::Character),
            image: None,
            permissions: None,
            experience_points: Some(0),
            inspiration: Some(true),
            notes: None,
            hit_points: Some(HitPoints {
                current: 8,
                max: 8,
                temp: None,
            }),
            abilities: vec![],
            skills: vec![],
            items: vec![],
            spells: vec![],
            armor_class: Some(10),
            system_data: HashMap::new(),
        };

        let serialized = serde_json::to_string(&character).unwrap();
        let deserialized: TtrpgCharacter = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(character.id, deserialized.id);
        assert_eq!(character.name, deserialized.name);
        assert_eq!(character.level, deserialized.level);
        assert_eq!(character.armor_class, deserialized.armor_class);
    }

    #[test]
    fn test_character_with_system_data() {
        let mut system_data = HashMap::new();
        system_data.insert("dnd5e_class".to_string(), serde_json::json!("Fighter"));
        system_data.insert("dnd5e_subclass".to_string(), serde_json::json!("Champion"));
        system_data.insert("pathfinder_favored_class".to_string(), serde_json::json!("Paladin"));

        let character = TtrpgCharacter {
            id: "multi_system_char".to_string(),
            name: "Multi System Character".to_string(),
            biography: Some("Works across multiple systems".to_string()),
            level: Some(5),
            actor_type: Some(ActorType::Character),
            image: None,
            permissions: None,
            experience_points: Some(6500),
            inspiration: None,
            notes: None,
            hit_points: None,
            abilities: vec![],
            skills: vec![],
            items: vec![],
            spells: vec![],
            armor_class: None,
            system_data,
        };

        assert_eq!(character.system_data.len(), 3);
        assert_eq!(character.system_data.get("dnd5e_class"), Some(&serde_json::json!("Fighter")));
    }
}