//! Roll20 Character to TTRPG Character mappings

use crate::generated::character::{Roll20CharacterSheet, Roll20CharacterAttribute};
use ttrpg_types::{TtrpgCharacter, HitPoints, Ability, AbilityType, Skill, ActorType, Ownership, PermissionLevel};
use serde_json::json;
use std::collections::HashMap;

impl From<Roll20CharacterSheet> for TtrpgCharacter {
    fn from(character: Roll20CharacterSheet) -> Self {
        let mut system_data = HashMap::new();
        
        // Extract basic information
        let id = character.id.to_string();
        let name = character.name.to_string();
        let biography = character.bio;
        let image = character.avatar;
        let notes = character.gmnotes;
        
        // Build ownership information
        let ownership = {
            let controlled_by = character.controlledby.as_ref().cloned().unwrap_or_default();
            if !controlled_by.is_empty() {
                let mut permissions = HashMap::new();
                let viewers = parse_csv_to_vec(&character.inplayerjournals.as_ref().cloned().unwrap_or_default());
                for viewer in viewers {
                    permissions.insert(viewer, PermissionLevel::Observer);
                }
                permissions.insert(controlled_by.clone(), PermissionLevel::Owner);
                
                Some(Ownership {
                    owner: controlled_by.clone(),
                    permissions,
                })
            } else {
                None
            }
        };

        // Store Roll20-specific data
        system_data.insert("archived".to_string(), json!(character.archived));
        if let Some(controlled) = &character.controlledby {
            system_data.insert("controlled_by".to_string(), json!(controlled));
        }
        if let Some(in_journals) = &character.inplayerjournals {
            system_data.insert("in_player_journals".to_string(), json!(in_journals));
        }
        if let Some(default_token) = &character.defaulttoken {
            system_data.insert("default_token".to_string(), json!(default_token));
        }
        if let Some(tags) = &character.tags {
            system_data.insert("tags".to_string(), json!(tags));
        }

        // Store abilities (macros) as system data
        system_data.insert("abilities".to_string(), json!(character.abilities));

        // Parse attributes into character fields
        let abilities = parse_attributes_to_abilities(&character.attribs);
        let skills = parse_attributes_to_skills(&character.attribs);
        let hit_points = parse_hit_points_from_attributes(&character.attribs);
        let level = parse_attribute_as_u32(&character.attribs, "level").or(Some(1));
        let experience_points = parse_attribute_as_u32(&character.attribs, "experience")
            .or_else(|| parse_attribute_as_u32(&character.attribs, "xp"));
        let armor_class = parse_attribute_as_u8(&character.attribs, "ac")
            .or_else(|| parse_attribute_as_u8(&character.attribs, "armor_class"));

        TtrpgCharacter {
            id,
            name,
            biography,
            level,
            actor_type: Some(ActorType::Character),
            image,
            permissions: ownership,
            experience_points,
            inspiration: None, // Roll20 doesn't have inspiration by default
            notes,
            hit_points,
            abilities,
            skills,
            items: Vec::new(), // Items stored separately in Roll20
            spells: Vec::new(), // Spells stored separately in Roll20
            armor_class,
            system_data,
        }
    }
}

/// Helper functions for parsing Roll20 attributes

fn parse_attributes_to_abilities(attribs: &[Roll20CharacterAttribute]) -> Vec<Ability> {
    let ability_names = [
        ("strength", "str", AbilityType::Strength),
        ("dexterity", "dex", AbilityType::Dexterity),
        ("constitution", "con", AbilityType::Constitution),
        ("intelligence", "int", AbilityType::Intelligence),
        ("wisdom", "wis", AbilityType::Wisdom),
        ("charisma", "cha", AbilityType::Charisma),
    ];

    ability_names.iter().filter_map(|(long_name, short_name, ability_type)| {
        let score = find_attribute_value(attribs, long_name)
            .or_else(|| find_attribute_value(attribs, short_name))
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(10); // Default ability score

        let modifier = calculate_ability_modifier(score);
        
        Some(Ability {
            ability_type: ability_type.clone(),
            score,
            modifier,
            proficiency: None, // Roll20 doesn't have universal proficiency bonuses
        })
    }).collect()
}

fn parse_attributes_to_skills(attribs: &[Roll20CharacterAttribute]) -> Vec<Skill> {
    // Common skill names in Roll20 character sheets
    let common_skills = [
        ("acrobatics", AbilityType::Dexterity),
        ("animal_handling", AbilityType::Wisdom),
        ("arcana", AbilityType::Intelligence),
        ("athletics", AbilityType::Strength),
        ("deception", AbilityType::Charisma),
        ("history", AbilityType::Intelligence),
        ("insight", AbilityType::Wisdom),
        ("intimidation", AbilityType::Charisma),
        ("investigation", AbilityType::Intelligence),
        ("medicine", AbilityType::Wisdom),
        ("nature", AbilityType::Intelligence),
        ("perception", AbilityType::Wisdom),
        ("performance", AbilityType::Charisma),
        ("persuasion", AbilityType::Charisma),
        ("religion", AbilityType::Intelligence),
        ("sleight_of_hand", AbilityType::Dexterity),
        ("stealth", AbilityType::Dexterity),
        ("survival", AbilityType::Wisdom),
    ];

    common_skills.iter().filter_map(|(skill_name, ability)| {
        find_attribute_value(attribs, skill_name)
            .and_then(|s| s.parse::<i8>().ok())
            .map(|bonus| Skill {
                name: skill_name.replace('_', " ").to_string(),
                ability: ability.clone(),
                proficiency: None, // Would need to calculate from bonus and ability modifier
                bonus: Some(bonus),
            })
    }).collect()
}

fn parse_hit_points_from_attributes(attribs: &[Roll20CharacterAttribute]) -> Option<HitPoints> {
    let max_hp = find_attribute_value(attribs, "hp")
        .or_else(|| find_attribute_value(attribs, "hit_points"))
        .or_else(|| find_attribute_value(attribs, "hp_max"))
        .and_then(|s| s.parse::<i32>().ok())?;

    let current_hp = find_attribute_value(attribs, "hp_current")
        .or_else(|| find_attribute_value(attribs, "current_hp"))
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(max_hp); // Default to max if current not specified

    let temp_hp = find_attribute_value(attribs, "hp_temp")
        .or_else(|| find_attribute_value(attribs, "temp_hp"))
        .and_then(|s| s.parse::<i32>().ok());

    Some(HitPoints {
        current: current_hp,
        max: max_hp,
        temp: temp_hp,
    })
}

pub fn find_attribute_value(attribs: &[Roll20CharacterAttribute], name: &str) -> Option<String> {
    attribs.iter()
        .find(|attr| attr.name.to_string().to_lowercase() == name.to_lowercase())
        .and_then(|attr| attr.current.clone())
}

pub fn parse_attribute_as_u32(attribs: &[Roll20CharacterAttribute], name: &str) -> Option<u32> {
    find_attribute_value(attribs, name)
        .and_then(|s| s.parse::<u32>().ok())
}

pub fn parse_attribute_as_u8(attribs: &[Roll20CharacterAttribute], name: &str) -> Option<u8> {
    find_attribute_value(attribs, name)
        .and_then(|s| s.parse::<u8>().ok())
}

pub fn calculate_ability_modifier(score: u8) -> i8 {
    ((score as i16 - 10) / 2) as i8
}

pub fn parse_csv_to_vec(csv: &str) -> Vec<String> {
    if csv.is_empty() {
        Vec::new()
    } else {
        csv.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
