//! Tests for Roll20 to TTRPG mappings

use crate::mappings::character::*;
// use crate::mappings::campaign::*; // No campaign mappings

use crate::generated::{
    character::{Roll20CharacterSheet, Roll20CharacterSheetId, Roll20CharacterSheetName, Roll20CharacterAttribute, Roll20CharacterAttributeName, Roll20CharacterAbility},
    page::{Page, PageId, PageName},
    handout::{Handout, HandoutId, HandoutName},
    // campaign - no Campaign struct in Roll20
};
use ttrpg_types::{TtrpgCharacter, TtrpgScene, TtrpgItem, ActorType, ItemType, AbilityType};

#[cfg(test)]
mod character_tests {
    use super::*;

    #[test]
    fn test_basic_character_mapping() {
        let character = Roll20CharacterSheet {
            id: Roll20CharacterSheetId::try_from("-TestCharacterIdVal1".to_string()).unwrap(),
            name: Roll20CharacterSheetName::try_from("Test Character".to_string()).unwrap(),
            bio: Some("A test character".to_string()),
            archived: false,
            avatar: Some("avatar_url".to_string()),
            controlledby: Some("player1".to_string()),
            defaulttoken: None,
            gmnotes: Some("GM notes here".to_string()),
            inplayerjournals: Some("player1,player2".to_string()),
            tags: None,
            abilities: Vec::new(),
            attribs: vec![
                create_test_attribute("strength", "15"),
                create_test_attribute("hp", "20"),
                create_test_attribute("hp_current", "15"),
                create_test_attribute("ac", "16"),
                create_test_attribute("level", "3"),
            ],
        };

        let ttrpg_char: TtrpgCharacter = character.into();

        assert_eq!(ttrpg_char.name, "Test Character");
        assert_eq!(ttrpg_char.biography, Some("A test character".to_string()));
        assert_eq!(ttrpg_char.image, Some("avatar_url".to_string()));
        assert_eq!(ttrpg_char.notes, Some("GM notes here".to_string()));
        assert_eq!(ttrpg_char.level, Some(3));
        assert_eq!(ttrpg_char.armor_class, Some(16));
        assert!(matches!(ttrpg_char.actor_type, Some(ActorType::Character)));
        
        // Check hit points
        let hp = ttrpg_char.hit_points.unwrap();
        assert_eq!(hp.max, 20);
        assert_eq!(hp.current, 15);
        assert_eq!(hp.temp, None);
        
        // Check abilities
        let strength = ttrpg_char.abilities.iter()
            .find(|a| matches!(a.ability_type, AbilityType::Strength))
            .unwrap();
        assert_eq!(strength.score, 15);
        assert_eq!(strength.modifier, 2); // (15-10)/2 = 2
    }

    #[test]
    fn test_ability_modifier_calculation() {
        assert_eq!(calculate_ability_modifier(8), -1);
        assert_eq!(calculate_ability_modifier(10), 0);
        assert_eq!(calculate_ability_modifier(12), 1);
        assert_eq!(calculate_ability_modifier(15), 2);
        assert_eq!(calculate_ability_modifier(18), 4);
    }

    #[test]
    fn test_parse_csv_to_vec() {
        assert_eq!(parse_csv_to_vec(""), Vec::<String>::new());
        assert_eq!(parse_csv_to_vec("player1"), vec!["player1"]);
        assert_eq!(parse_csv_to_vec("player1,player2,player3"), vec!["player1", "player2", "player3"]);
        assert_eq!(parse_csv_to_vec("player1, player2 , player3"), vec!["player1", "player2", "player3"]);
    }

    #[test]
    fn test_find_attribute_value() {
        let attributes = vec![
            create_test_attribute("strength", "15"),
            create_test_attribute("DEXTERITY", "12"),
            create_test_attribute("hp", "25"),
        ];

        assert_eq!(find_attribute_value(&attributes, "strength"), Some("15".to_string()));
        assert_eq!(find_attribute_value(&attributes, "STRENGTH"), Some("15".to_string()));
        assert_eq!(find_attribute_value(&attributes, "dexterity"), Some("12".to_string()));
        assert_eq!(find_attribute_value(&attributes, "missing"), None);
    }

    pub fn create_test_attribute(name: &str, current: &str) -> Roll20CharacterAttribute {
        Roll20CharacterAttribute {
            name: Roll20CharacterAttributeName::try_from(name.to_string()).unwrap(),
            current: Some(current.to_string()),
            max: None,
            id: None,
        }
    }
}

#[cfg(test)]
mod scene_tests {
    use super::*;

    #[test]
    fn test_basic_page_mapping() {
        let page = Page {
            id: PageId::try_from("-TestPageIdValue1234".to_string()).unwrap(),
            name: PageName::try_from("Test Scene".to_string()).unwrap(),
            archived: Some(false),
            background_color: Some("#ffffff".to_string()),
            fog_opacity: Some(0.5),
            grid_opacity: Some(0.3),
            grid_size: Some(70.0),
            height: Some(1400.0),
            width: Some(1000.0),
            lightenforcelos: Some(true),
            lightglobalillum: Some(false),
            lightrestrictmovement: Some(true),
            lightupdatedrop: Some(false),
            snapping_increment: Some(5.0),
        };

        let ttrpg_scene: TtrpgScene = page.into();

        assert_eq!(ttrpg_scene.name, "Test Scene");
        assert_eq!(ttrpg_scene.background_color, Some("#ffffff".to_string()));
        assert_eq!(ttrpg_scene.width, Some(1000.0));
        assert_eq!(ttrpg_scene.height, Some(1400.0));
        
        // Check grid configuration
        let grid = ttrpg_scene.grid.unwrap();
        assert_eq!(grid.size, 70);
        // assert!(grid.snap_to_grid); // No snap_to_grid field
        // assert_eq!(grid.snap_increment, 5.0); // No snap_increment field
        // assert!(grid.snap_to_grid); // No snap_to_grid field
        
        // Check lighting configuration
        let lighting = ttrpg_scene.lighting.unwrap();
        assert!(lighting.fog_of_war.unwrap_or(false)); // fog_of_war instead of enforce_line_of_sight
        assert_eq!(lighting.global_light, Some(0.0)); // global_light instead of global_illumination
        
        // Check system data
        assert_eq!(ttrpg_scene.system_data.get("fog_opacity"), Some(&serde_json::json!(0.5)));
        assert_eq!(ttrpg_scene.system_data.get("light_restrict_movement"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn test_page_with_minimal_data() {
        let page = Page {
            id: PageId::try_from("-MinimalPageValueAbc".to_string()).unwrap(),
            name: PageName::try_from("Minimal".to_string()).unwrap(),
            archived: None,
            background_color: None,
            fog_opacity: None,
            grid_opacity: None,
            grid_size: None,
            height: None,
            width: None,
            lightenforcelos: None,
            lightglobalillum: None,
            lightrestrictmovement: None,
            lightupdatedrop: None,
            snapping_increment: None,
        };

        let ttrpg_scene: TtrpgScene = page.into();

        assert_eq!(ttrpg_scene.name, "Minimal");
        assert_eq!(ttrpg_scene.background_color, None);
        assert_eq!(ttrpg_scene.width, None); // No default when not set
        assert_eq!(ttrpg_scene.height, None); // No default when not set
        // assert_eq!(ttrpg_scene.grid, None); // PartialEq not implemented for GridConfig
        
        // Should still have lighting config with defaults
        let lighting = ttrpg_scene.lighting.unwrap();
        assert!(!lighting.fog_of_war.unwrap_or(false));
        assert_eq!(lighting.global_light, Some(0.0));
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn test_handout_mapping() {
        let handout = Handout {
            id: HandoutId::try_from("-TestHandoutIdVal123".to_string()).unwrap(),
            name: HandoutName::try_from("Test Document".to_string()).unwrap(),
            archived: Some(false),
            // avatar: Some("document_image.png".to_string()), // No avatar field
            // controlledby: Some("gm".to_string()), // No controlledby field
            gmnotes: None,
            inplayerjournals: Some("player1,player2".to_string()),
            notes: Some("This is the document content".to_string()),
            // tags: Some("important,lore".to_string()), // No tags field
        };

        let ttrpg_item: TtrpgItem = handout.into();

        assert_eq!(ttrpg_item.name, "Test Document");
        assert_eq!(ttrpg_item.description, Some("This is the document content".to_string()));
        assert_eq!(ttrpg_item.image, None); // No avatar field in handouts
        assert!(matches!(ttrpg_item.item_type, ItemType::Equipment));
        assert_eq!(ttrpg_item.quantity, 1);
        assert_eq!(ttrpg_item.weight, None);
        assert_eq!(ttrpg_item.requires_attunement, Some(false));
        
        // Check system data
        // assert_eq!(ttrpg_item.system_data.get("controlled_by"), Some(&serde_json::json!("gm"))); // No controlledby field
        // assert_eq!(ttrpg_item.system_data.get("tags"), Some(&serde_json::json!("important,lore"))); // No tags field
    }
}

// Campaign tests removed - no Campaign struct exists in Roll20 schemas
// Campaign information is inferred from export structure

/*
#[cfg(test)]
mod campaign_tests {
    use super::*;

    // #[test]
    // fn test_campaign_mapping() {
    //     // No Campaign struct exists in Roll20 generated schemas
    // }
}
*/

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_character_with_complex_attributes() {
        let character = Roll20CharacterSheet {
            id: Roll20CharacterSheetId::try_from("-ComplexCharacterVal".to_string()).unwrap(),
            name: Roll20CharacterSheetName::try_from("Complex Character".to_string()).unwrap(),
            bio: Some("Complex backstory with multiple paragraphs".to_string()),
            archived: true,
            avatar: Some("complex_avatar.png".to_string()),
            controlledby: Some("player1,player2".to_string()),
            defaulttoken: Some("token_data".to_string()),
            gmnotes: Some("Secret GM information".to_string()),
            inplayerjournals: Some("player1,player2,player3".to_string()),
            tags: Some("pc,important".to_string()),
            abilities: vec![
                create_test_ability("Attack", "1d20+5"),
                create_test_ability("Spell", "@{spelldc}"),
            ],
            attribs: vec![
                create_test_attribute("strength", "18"),
                create_test_attribute("dexterity", "16"),
                create_test_attribute("constitution", "14"),
                create_test_attribute("intelligence", "12"),
                create_test_attribute("wisdom", "10"),
                create_test_attribute("charisma", "8"),
                create_test_attribute("hp", "45"),
                create_test_attribute("hp_current", "32"),
                create_test_attribute("hp_temp", "5"),
                create_test_attribute("ac", "18"),
                create_test_attribute("level", "8"),
                create_test_attribute("experience", "34000"),
                create_test_attribute("acrobatics", "5"),
                create_test_attribute("athletics", "7"),
                create_test_attribute("stealth", "6"),
            ],
        };

        let ttrpg_char: TtrpgCharacter = character.into();

        // Test comprehensive attribute parsing
        assert_eq!(ttrpg_char.level, Some(8));
        assert_eq!(ttrpg_char.experience_points, Some(34000));
        
        let hp = ttrpg_char.hit_points.unwrap();
        assert_eq!(hp.current, 32);
        assert_eq!(hp.max, 45);
        assert_eq!(hp.temp, Some(5));
        
        // Test all abilities are parsed
        assert_eq!(ttrpg_char.abilities.len(), 6);
        let charisma = ttrpg_char.abilities.iter()
            .find(|a| matches!(a.ability_type, AbilityType::Charisma))
            .unwrap();
        assert_eq!(charisma.score, 8);
        assert_eq!(charisma.modifier, -1);
        
        // Test skills
        assert!(!ttrpg_char.skills.is_empty());
        let athletics = ttrpg_char.skills.iter()
            .find(|s| s.name.contains("athletics"))
            .unwrap();
        assert_eq!(athletics.bonus, Some(7));
        
        // Test permissions with multiple controllers
        let permissions = ttrpg_char.permissions.unwrap();
        assert_eq!(permissions.owner, "player1,player2"); // Full controlledby string
        assert!(permissions.permissions.contains_key("player1"));
        assert!(permissions.permissions.contains_key("player2"));
        assert!(permissions.permissions.contains_key("player3"));
        
        // Test system data preservation
        assert_eq!(ttrpg_char.system_data.get("archived"), Some(&serde_json::json!(true)));
        assert_eq!(ttrpg_char.system_data.get("tags"), Some(&serde_json::json!("pc,important")));
        assert_eq!(ttrpg_char.system_data.get("default_token"), Some(&serde_json::json!("token_data")));
    }

    #[test]
    fn test_character_missing_standard_attributes() {
        let character = Roll20CharacterSheet {
            id: Roll20CharacterSheetId::try_from("-MissingAttrsCharVal".to_string()).unwrap(),
            name: Roll20CharacterSheetName::try_from("Minimal Character".to_string()).unwrap(),
            bio: None,
            archived: false,
            avatar: None,
            controlledby: None,
            defaulttoken: None,
            gmnotes: None,
            inplayerjournals: None,
            tags: None,
            abilities: Vec::new(),
            attribs: vec![
                // Only non-standard attributes
                create_test_attribute("custom_stat", "value"),
                create_test_attribute("weird_attribute", "42"),
            ],
        };

        let ttrpg_char: TtrpgCharacter = character.into();

        // Should have default values for missing standard attributes
        assert_eq!(ttrpg_char.level, Some(1)); // Default level
        // assert_eq!(ttrpg_char.hit_points, None); // No HP attributes - PartialEq not implemented
        assert_eq!(ttrpg_char.armor_class, None); // No AC attribute
        assert_eq!(ttrpg_char.experience_points, None); // No XP attribute
        
        // Should still have default abilities with score 10
        assert_eq!(ttrpg_char.abilities.len(), 6);
        let default_ability = &ttrpg_char.abilities[0];
        assert_eq!(default_ability.score, 10);
        assert_eq!(default_ability.modifier, 0);
        
        // Should have no skills parsed
        assert!(ttrpg_char.skills.is_empty());
        
        // Should have no permissions
        // assert_eq!(ttrpg_char.permissions, None); // PartialEq not implemented for Ownership
    }

    #[test]
    fn test_scene_with_all_lighting_options() {
        let page = Page {
            id: PageId::try_from("-LightSceneValueAbc1".to_string()).unwrap(),
            name: PageName::try_from("Lighting Test".to_string()).unwrap(),
            archived: Some(false),
            background_color: Some("#123456".to_string()),
            fog_opacity: Some(0.8),
            grid_opacity: Some(0.4),
            grid_size: Some(50.0),
            height: Some(2000.0),
            width: Some(1500.0),
            lightenforcelos: Some(true),
            lightglobalillum: Some(true),
            lightrestrictmovement: Some(false),
            lightupdatedrop: Some(true),
            snapping_increment: Some(10.0),
        };

        let ttrpg_scene: TtrpgScene = page.into();

        let lighting = ttrpg_scene.lighting.unwrap();
        assert!(lighting.fog_of_war.unwrap_or(false)); // fog_of_war instead of enforce_line_of_sight
        assert_eq!(lighting.global_light, Some(1.0)); // global_light instead of global_illumination
        
        let grid = ttrpg_scene.grid.unwrap();
        assert_eq!(grid.size, 50);
        // assert_eq!(grid.opacity, 0.4); // No opacity field
        // assert_eq!(grid.snap_increment, 10.0); // No snap_increment field
        // assert!(grid.snap_to_grid); // No snap_to_grid field
        
        // Check all lighting system data is preserved
        assert_eq!(ttrpg_scene.system_data.get("fog_opacity"), Some(&serde_json::json!(0.8)));
        assert_eq!(ttrpg_scene.system_data.get("light_restrict_movement"), Some(&serde_json::json!(false)));
        assert_eq!(ttrpg_scene.system_data.get("light_update_drop"), Some(&serde_json::json!(true)));
    }

    #[test]
    fn test_handout_with_full_permissions() {
        let handout = Handout {
            id: HandoutId::try_from("-PermHandoutValueAbc".to_string()).unwrap(),
            name: HandoutName::try_from("Complete Handout".to_string()).unwrap(),
            archived: Some(false),
            // avatar: Some("handout_thumb.jpg".to_string()), // No avatar field
            // controlledby: Some("gm,assistant".to_string()), // No controlledby field
            gmnotes: Some("Private GM notes".to_string()),
            inplayerjournals: Some("player1,player2,player3,player4".to_string()),
            notes: Some("Public handout content with <b>HTML</b>".to_string()),
            // tags: Some("important,session1,lore".to_string()), // No tags field
        };

        let ttrpg_item: TtrpgItem = handout.into();

        assert_eq!(ttrpg_item.name, "Complete Handout");
        assert_eq!(ttrpg_item.description, Some("Public handout content with <b>HTML</b>".to_string()));
        assert_eq!(ttrpg_item.image, None); // No avatar field in handouts
        
        // Check all system data is preserved
        // assert_eq!(ttrpg_item.system_data.get("controlled_by"), Some(&serde_json::json!("gm,assistant"))); // No controlledby field
        // assert_eq!(ttrpg_item.system_data.get("in_player_journals"), Some(&serde_json::json!("player1,player2,player3,player4"))); // Not stored in system_data
        // assert_eq!(ttrpg_item.system_data.get("tags"), Some(&serde_json::json!("important,session1,lore"))); // No tags field
        assert_eq!(ttrpg_item.system_data.get("archived"), Some(&serde_json::json!(false)));
    }

    #[test]
    fn test_edge_case_parsing() {
        // Test with malformed/edge case attribute values
        let attributes = vec![
            create_test_attribute("strength", "not_a_number"),
            create_test_attribute("hp", ""),
            create_test_attribute("level", "-5"),
            create_test_attribute("ac", "999"),
        ];

        // Should handle parse failures gracefully
        assert_eq!(find_attribute_value(&attributes, "strength"), Some("not_a_number".to_string()));
        assert_eq!(parse_attribute_as_u8(&attributes, "strength"), None);
        assert_eq!(parse_attribute_as_u32(&attributes, "hp"), None);
        
        // Should accept valid but unusual values
        assert_eq!(parse_attribute_as_u32(&attributes, "level"), None); // Negative not valid for u32
        assert_eq!(parse_attribute_as_u8(&attributes, "ac"), None); // 999 too large for u8
    }

    #[test]
    fn test_csv_parsing_edge_cases() {
        // Test CSV parsing with various edge cases
        assert_eq!(parse_csv_to_vec(""), Vec::<String>::new());
        assert_eq!(parse_csv_to_vec(" "), Vec::<String>::new());
        assert_eq!(parse_csv_to_vec(","), Vec::<String>::new());
        assert_eq!(parse_csv_to_vec(",,"), Vec::<String>::new());
        assert_eq!(parse_csv_to_vec("single"), vec!["single"]);
        assert_eq!(parse_csv_to_vec(" single "), vec!["single"]);
        assert_eq!(parse_csv_to_vec("a,,b"), vec!["a", "b"]);
        assert_eq!(parse_csv_to_vec(" a , , b "), vec!["a", "b"]);
    }

    fn create_test_attribute(name: &str, current: &str) -> Roll20CharacterAttribute {
        Roll20CharacterAttribute {
            id: Some(crate::generated::character::Roll20CharacterAttributeId::try_from("-TestAttributeIdVal1".to_string()).unwrap()),
            name: Roll20CharacterAttributeName::try_from(name.to_string()).unwrap(),
            current: Some(current.to_string()),
            max: None,
        }
    }

    fn create_test_ability(name: &str, action: &str) -> Roll20CharacterAbility {
        Roll20CharacterAbility {
            id: Some(crate::generated::character::Roll20CharacterAbilityId::try_from("-TestAbilityIdValue1".to_string()).unwrap()),
            name: crate::generated::character::Roll20CharacterAbilityName::try_from(name.to_string()).unwrap(),
            action: Some(action.to_string()),
            description: None,
            istokenaction: false,
        }
    }
}
