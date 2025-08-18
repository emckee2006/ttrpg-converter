//! Tests for Foundry to TTRPG mappings

use crate::mappings::actor::{calculate_ability_modifier, foundry_type_to_actor_type};
use crate::generated::{
    actor::{FoundryPf2eActor, FoundryPf2eActorId, FoundryPf2eActorName, FoundryPf2eActorType, FoundryPf2eActorSystem, FoundryPf2eActorFolder},
    scene::{FoundryScene, FoundrySceneId, FoundrySceneName, FoundrySceneFolder, ColorHex, FoundrySceneGrid, Position},
    item::{FoundryPf2eItem, FoundryPf2eItemId, FoundryPf2eItemName, FoundryPf2eItemType, FoundryPf2eItemSystem},
    journal::{FoundryJournalPage, FoundryJournalPageId, FoundryJournalPageName, FoundryJournalPageText, FoundryJournalPageType, FoundryJournalPageTextFormat},
    world::{FoundryVttWorld, FoundryVttWorldId, FoundryVttWorldTitle, FoundryVttWorldCoreVersion, FoundryVttWorldSystem, FoundryVttWorldSystemVersion},
};
use ttrpg_types::{TtrpgCharacter, TtrpgScene, TtrpgItem, ActorType, ItemType};
use serde_json::{json, Map};

#[cfg(test)]
mod actor_tests {
    use super::*;

    #[test]
    fn test_basic_actor_mapping() {
        let flags = Map::new();
        let actor = FoundryPf2eActor {
            flags,
            folder: None,
            id: FoundryPf2eActorId::try_from("testactorid12345".to_string()).unwrap(),
            img: Some("avatar.png".to_string()),
            items: vec![],
            name: FoundryPf2eActorName::try_from("Test Character".to_string()).unwrap(),
            ownership: None,
            sort: None,
            stats: None,
            system: FoundryPf2eActorSystem::default(),
            type_: FoundryPf2eActorType::Character,
        };

        let ttrpg_char: TtrpgCharacter = actor.into();

        assert_eq!(ttrpg_char.name, "Test Character");
        assert!(matches!(ttrpg_char.actor_type, Some(ActorType::Character)));
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
    fn test_foundry_type_mapping() {
        assert!(matches!(foundry_type_to_actor_type("character"), ActorType::Character));
        assert!(matches!(foundry_type_to_actor_type("npc"), ActorType::NPC));
        assert!(matches!(foundry_type_to_actor_type("vehicle"), ActorType::Vehicle));
        assert!(matches!(foundry_type_to_actor_type("unknown"), ActorType::Character));
    }
}

#[cfg(test)]
mod scene_tests {
    use super::*;

    #[test]
    fn test_basic_scene_mapping() {
        let flags = Map::new();
        let scene = FoundryScene {
            active: None,
            background_color: None,
            darkness: None,
            flags,
            fog_exploration: None,
            fog_reset: None,
            folder: None,
            foreground: None,
            global_light: None,
            global_light_threshold: None,
            grid: None,
            height: Some(1500.0),
            id: FoundrySceneId::try_from("testsceneid12345".to_string()).unwrap(),
            img: Some("background.jpg".to_string()),
            initial: None,
            journal: None,
            name: FoundrySceneName::try_from("Test Scene".to_string()).unwrap(),
            nav_order: None,
            navigation: None,
            ownership: None,
            padding: None,
            playlist: None,
            playlist_sound: None,
            sort: None,
            stats: None,
            thumb: None,
            token_vision: None,
            weather: None,
            width: Some(2000.0),
        };

        let ttrpg_scene: TtrpgScene = scene.into();

        assert_eq!(ttrpg_scene.name, "Test Scene");
        assert_eq!(ttrpg_scene.background_image, Some("background.jpg".to_string()));
        assert_eq!(ttrpg_scene.width, Some(2000.0));
        assert_eq!(ttrpg_scene.height, Some(1500.0));
    }

    #[test]
    fn test_scene_with_minimal_data() {
        let flags = Map::new();
        let scene = FoundryScene {
            active: None,
            background_color: None,
            darkness: None,
            flags,
            fog_exploration: None,
            fog_reset: None,
            folder: None,
            foreground: None,
            global_light: None,
            global_light_threshold: None,
            grid: None,
            height: None,
            id: FoundrySceneId::try_from("minimalscene1234".to_string()).unwrap(),
            img: None,
            initial: None,
            journal: None,
            name: FoundrySceneName::try_from("Minimal".to_string()).unwrap(),
            nav_order: None,
            navigation: None,
            ownership: None,
            padding: None,
            playlist: None,
            playlist_sound: None,
            sort: None,
            stats: None,
            thumb: None,
            token_vision: None,
            weather: None,
            width: None,
        };

        let ttrpg_scene: TtrpgScene = scene.into();

        assert_eq!(ttrpg_scene.name, "Minimal");
        assert_eq!(ttrpg_scene.background_image, None);
        assert_eq!(ttrpg_scene.width, Some(1920.0));
        assert_eq!(ttrpg_scene.height, Some(1080.0));
    }
}

#[cfg(test)]
mod item_tests {
    use super::*;

    #[test]
    fn test_weapon_item_mapping() {
        let flags = Map::new();
        let item = FoundryPf2eItem {
            flags,
            folder: None,
            id: FoundryPf2eItemId::try_from("testweaponid1234".to_string()).unwrap(),
            img: Some("sword.png".to_string()),
            name: FoundryPf2eItemName::try_from("Test Sword".to_string()).unwrap(),
            ownership: None,
            sort: None,
            stats: None,
            system: FoundryPf2eItemSystem::default(),
            type_: FoundryPf2eItemType::Weapon,
        };

        let ttrpg_item: TtrpgItem = item.into();

        assert_eq!(ttrpg_item.name, "Test Sword");
        assert_eq!(ttrpg_item.image, Some("sword.png".to_string()));
        assert!(matches!(ttrpg_item.item_type, ItemType::Weapon));
        assert_eq!(ttrpg_item.quantity, 1);
        assert_eq!(ttrpg_item.weight, None);
        assert_eq!(ttrpg_item.requires_attunement, None);
    }

    #[test]
    fn test_equipment_item_mapping() {
        let flags = Map::new();
        let item = FoundryPf2eItem {
            flags,
            folder: None,
            id: FoundryPf2eItemId::try_from("testequipid12345".to_string()).unwrap(),
            img: Some("armor.png".to_string()),
            name: FoundryPf2eItemName::try_from("Test Armor".to_string()).unwrap(),
            ownership: None,
            sort: None,
            stats: None,
            system: FoundryPf2eItemSystem::default(),
            type_: FoundryPf2eItemType::Armor,
        };

        let ttrpg_item: TtrpgItem = item.into();

        assert_eq!(ttrpg_item.name, "Test Armor");
        assert!(matches!(ttrpg_item.item_type, ItemType::Armor));
    }
}

#[cfg(test)]
mod journal_tests {
    use super::*;

    #[test]
    fn test_journal_page_mapping() {
        let flags = Map::new();
        let journal = FoundryJournalPage {
            flags,
            id: FoundryJournalPageId::try_from("testjournalid123".to_string()).unwrap(),
            name: FoundryJournalPageName::try_from("Test Journal".to_string()).unwrap(),
            ownership: None,
            sort: None,
            text: Some(FoundryJournalPageText {
                content: Some("Journal content here".to_string()),
                format: Some(FoundryJournalPageTextFormat::try_from(1).unwrap()),
                markdown: None,
            }),
            image: None,
            src: None,
            title: None,
            type_: FoundryJournalPageType::Text,
            video: None,
        };

        let ttrpg_item: TtrpgItem = journal.into();

        assert_eq!(ttrpg_item.name, "Test Journal");
        assert_eq!(ttrpg_item.description, Some("Journal content here".to_string()));
        assert!(matches!(ttrpg_item.item_type, ItemType::Equipment));
        assert_eq!(ttrpg_item.quantity, 1);
    }

    #[test]
    fn test_journal_with_no_content() {
        let flags = Map::new();
        let journal = FoundryJournalPage {
            flags,
            id: FoundryJournalPageId::try_from("emptyjournal1234".to_string()).unwrap(),
            name: FoundryJournalPageName::try_from("Empty Journal".to_string()).unwrap(),
            ownership: None,
            sort: None,
            text: None,
            image: None,
            src: None,
            title: None,
            type_: FoundryJournalPageType::Text,
            video: None,
        };

        let ttrpg_item: TtrpgItem = journal.into();

        assert_eq!(ttrpg_item.name, "Empty Journal");
        assert_eq!(ttrpg_item.description, None);
    }
}

#[cfg(test)]
mod world_tests {
    use super::*;

    #[test]
    fn test_world_mapping() {
        let world = FoundryVttWorld {
            authors: vec![],
            compatibility: None,
            compatible_core_version: None,
            core_version: FoundryVttWorldCoreVersion::try_from("12.331".to_string()).unwrap(),
            description: Some("A test world".to_string()),
            id: FoundryVttWorldId::try_from("testworldid12345".to_string()).unwrap(),
            join_theme: "default".to_string(),
            last_played: None,
            minimum_core_version: None,
            packs: vec![],
            playtime: Some(0.0),
            system: FoundryVttWorldSystem::try_from("pf2e".to_string()).unwrap(),
            system_version: Some(FoundryVttWorldSystemVersion::try_from("6.0.4".to_string()).unwrap()),
            title: FoundryVttWorldTitle::try_from("Test World".to_string()).unwrap(),
            version: None,
        };

        let ttrpg_item: TtrpgItem = world.into();

        // World title is formatted using debug format
        assert!(ttrpg_item.name.contains("Test World"));
        assert_eq!(ttrpg_item.description, Some("A test world".to_string()));
        assert!(matches!(ttrpg_item.item_type, ItemType::Equipment));
        
        // Check system data preservation
        assert_eq!(ttrpg_item.system_data.get("game_system"), Some(&json!("pf2e")));
        assert_eq!(ttrpg_item.system_data.get("core_version"), Some(&json!("12.331")));
        assert_eq!(ttrpg_item.system_data.get("system_version"), Some(&json!("6.0.4")));
    }
}

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    #[test]
    fn test_actor_with_complex_data() {
        let mut flags = Map::new();
        flags.insert("pf2e".to_string(), json!({"test": "value"}));
        
        let actor = FoundryPf2eActor {
            flags,
            folder: Some(FoundryPf2eActorFolder::try_from("folderid12345678".to_string()).unwrap()),
            id: FoundryPf2eActorId::try_from("complexactorid12".to_string()).unwrap(),
            img: Some("complex.png".to_string()),
            items: vec![],
            name: FoundryPf2eActorName::try_from("Complex Character".to_string()).unwrap(),
            ownership: None,
            sort: Some(100.0),
            stats: None,
            system: FoundryPf2eActorSystem::default(),
            type_: FoundryPf2eActorType::Character,
        };

        let ttrpg_char: TtrpgCharacter = actor.into();

        assert_eq!(ttrpg_char.name, "Complex Character");
        assert!(matches!(ttrpg_char.actor_type, Some(ActorType::Character)));
        
        // Check system data preservation
        // Sort data is not extracted to system_data in current mapping
    }

    #[test]
    fn test_scene_with_full_configuration() {
        let mut flags = Map::new();
        flags.insert("core".to_string(), json!({"test": true}));
        
        let scene = FoundryScene {
            active: Some(true),
            background_color: Some(ColorHex::try_from("#123456".to_string()).unwrap()),
            darkness: Some(0.8),
            flags,
            fog_exploration: Some(false),
            fog_reset: Some(1234567890.0),
            folder: Some(FoundrySceneFolder::try_from("scenefolderid123".to_string()).unwrap()),
            foreground: None,
            global_light: Some(false),
            global_light_threshold: Some(0.25),
            grid: Some(FoundrySceneGrid {
                alpha: None,
                color: None,
                distance: Some(100.0),
                size: Some(50.0),
                type_: None,
                units: None,
            }),
            height: Some(3000.0),
            id: FoundrySceneId::try_from("fullsceneid12345".to_string()).unwrap(),
            img: Some("scene.jpg".to_string()),
            initial: Some(Position {
                x: Some(0.0),
                y: Some(0.0),
            }),
            journal: None,
            name: FoundrySceneName::try_from("Full Scene".to_string()).unwrap(),
            nav_order: None,
            navigation: None,
            ownership: None,
            padding: Some(0.1),
            playlist: None,
            playlist_sound: None,
            sort: Some(200.0),
            stats: None,
            thumb: None,
            token_vision: Some(true),
            weather: None,
            width: Some(4000.0),
        };

        let ttrpg_scene: TtrpgScene = scene.into();

        assert_eq!(ttrpg_scene.name, "Full Scene");
        assert_eq!(ttrpg_scene.width, Some(4000.0));
        assert_eq!(ttrpg_scene.height, Some(3000.0));
        assert!(ttrpg_scene.background_color.as_ref().map(|c| c.contains("#123456")).unwrap_or(false));
        
        // Check system data preservation
        assert_eq!(ttrpg_scene.system_data.get("token_vision"), Some(&json!(true)));
    }

    #[test]
    fn test_item_with_system_data() {
        let mut flags = Map::new();
        flags.insert("pf2e".to_string(), json!({"rarity": "rare"}));
        
        let item = FoundryPf2eItem {
            flags,
            folder: None,
            id: FoundryPf2eItemId::try_from("magicitemid12345".to_string()).unwrap(),
            img: Some("magic-sword.png".to_string()),
            name: FoundryPf2eItemName::try_from("Magic Sword".to_string()).unwrap(),
            ownership: None,
            sort: Some(50.0),
            stats: None,
            system: FoundryPf2eItemSystem::default(),
            type_: FoundryPf2eItemType::Weapon,
        };

        let ttrpg_item: TtrpgItem = item.into();

        assert_eq!(ttrpg_item.name, "Magic Sword");
        assert!(matches!(ttrpg_item.item_type, ItemType::Weapon));
        
        // Check system data preservation  
        assert_eq!(ttrpg_item.system_data.get("flags"), Some(&json!({"pf2e": {"rarity": "rare"}})));
    }

    #[test]
    fn test_edge_case_empty_names() {
        let flags = Map::new();
        let actor = FoundryPf2eActor {
            flags,
            folder: None,
            id: FoundryPf2eActorId::try_from("emptynameact1234".to_string()).unwrap(),
            img: None,
            items: vec![],
            name: FoundryPf2eActorName::try_from("A".to_string()).unwrap(),
            ownership: None,
            sort: None,
            stats: None,
            system: FoundryPf2eActorSystem::default(),
            type_: FoundryPf2eActorType::Character,
        };

        let ttrpg_char: TtrpgCharacter = actor.into();
        
        // Empty name should be preserved
        assert_eq!(ttrpg_char.name, "A");
        assert_eq!(ttrpg_char.image, None);
    }
}