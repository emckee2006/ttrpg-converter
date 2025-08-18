use crate::generated::actor::{FoundryPf2eActor, FoundryPf2eActorType, FoundryPf2eActorSystem};
use ttrpg_types::{TtrpgCharacter, ActorType, HitPoints, Ability, AbilityType, Skill};
use serde_json::{json, Value};
use std::collections::HashMap;
use ttrpg_types::common::{Ownership, PermissionLevel};

impl From<FoundryPf2eActor> for TtrpgCharacter {
    fn from(actor: FoundryPf2eActor) -> Self {
        let mut system_data = HashMap::new();

        // Extract basic information
        let id = actor.id.to_string();
        let name = actor.name.to_string();
        let image = actor.img;
        
        // Convert actor type - use type from system or default
        let actor_type = foundry_type_to_actor_type(match actor.type_ {
            FoundryPf2eActorType::Character => "character",
            FoundryPf2eActorType::Npc => "npc",
            FoundryPf2eActorType::Vehicle => "vehicle",
            FoundryPf2eActorType::Hazard => "npc", // Map hazard to NPC
            FoundryPf2eActorType::Loot => "character", // Map loot to character
            FoundryPf2eActorType::Familiar => "character", // Map familiar to character
        });
        
        // Extract nested system data safely
        let system = &actor.system;
        
        // Extract biography from nested path
        let biography = None; // TODO: Extract from system struct fields
        
        // Extract hit points
        let hit_points = extract_hit_points_from_system(system);
        
        // Extract abilities
        let abilities = extract_ability_scores_from_system(system);
        
        // Extract skills
        let skills = Vec::new(); // TODO: Implement skill extraction from system
        
        // Extract level and experience
        let level = None; // TODO: Extract from system struct fields
        let experience_points = None; // TODO: Extract from system struct fields
        
        // Extract armor class
        let armor_class = None; // TODO: Extract from system struct fields
        
        // Convert ownership
        let permissions = convert_foundry_ownership(json!(actor.ownership));
        
        // Store Foundry-specific data in system_data
        system_data.insert("flags".to_string(), json!(actor.flags));
        // Effects not available in current schema
        system_data.insert("items".to_string(), json!(actor.items));
        
        TtrpgCharacter {
            notes: None,
            id,
            name,
            biography,
            image,
            actor_type: Some(actor_type),
            level,
            experience_points,
            hit_points: Some(hit_points),
            armor_class,
            abilities,
            skills,
            inspiration: None,
            items: Vec::new(),
            spells: Vec::new(),
            permissions,
            system_data,
        }
    }
}

pub fn foundry_type_to_actor_type(foundry_type: &str) -> ActorType {
    match foundry_type {
        "character" => ActorType::Character,
        "npc" => ActorType::NPC,
        "vehicle" => ActorType::Vehicle,
        _ => ActorType::Character, // Default fallback
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

fn extract_hit_points_from_system(_system: &FoundryPf2eActorSystem) -> HitPoints {
    HitPoints {
        current: 10,
        max: 10,
        temp: None,
    }
}

fn extract_ability_scores_from_system(_system: &FoundryPf2eActorSystem) -> Vec<Ability> {
    let ability_types = [
        ("str", AbilityType::Strength),
        ("dex", AbilityType::Dexterity),
        ("con", AbilityType::Constitution),
        ("int", AbilityType::Intelligence),
        ("wis", AbilityType::Wisdom),
        ("cha", AbilityType::Charisma),
    ];
    
    ability_types.iter().filter_map(|(_key, ability_type)| {
        // TODO: Extract ability scores from system when properly mapped
        let score = 10u8; // Default score
        let modifier = calculate_ability_modifier(score);
        let proficiency = None;
        
        Some(Ability {
            ability_type: ability_type.clone(),
            score,
            modifier,
            proficiency,
        })
    }).collect()
}

fn extract_skills(system: &Value) -> Vec<Skill> {
    let skills_data = match system.get("skills") {
        Some(data) => data,
        None => return Vec::new(),
    };
    
    // TODO: Implement proper skill mapping when SkillType is available
    let skill_mappings = [
        ("acr", "Acrobatics"),
        ("arc", "Arcana"),
        ("ath", "Athletics"),
        ("dec", "Deception"),
        ("his", "History"),
        ("ins", "Insight"),
        ("inti", "Intimidation"),
        ("inv", "Investigation"),
        ("med", "Medicine"),
        ("nat", "Nature"),
        ("prc", "Perception"),
        ("prf", "Performance"),
        ("per", "Persuasion"),
        ("rel", "Religion"),
        ("slt", "Sleight of Hand"),
        ("ste", "Stealth"),
        ("sur", "Survival"),
    ];
    
    skill_mappings.iter().filter_map(|(key, skill_type)| {
        let skill_data = skills_data.get(*key)?;
        let proficiency_bonus = skill_data
            .get("proficiency")
            .and_then(|v| v.as_f64())
            .map(|v| v as f32);
        let modifier = skill_data
            .get("total")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i8;
        
        Some(Skill {
            name: skill_type.to_string(),
            ability: AbilityType::Intelligence, // Default, TODO: map properly
            proficiency: proficiency_bonus,
            bonus: Some(modifier),
        })
    }).collect()
}

pub fn calculate_ability_modifier(score: u8) -> i8 {
    ((score as i16 - 10) / 2) as i8
}

fn convert_foundry_ownership(ownership: Value) -> Option<Ownership> {
    if ownership.is_null() {
        return Some(Ownership {
            owner: "default".to_string(),
            permissions: HashMap::new(),
        });
    }

    let mut permissions = HashMap::new();
    
    if let Some(owner_id) = ownership.get("default").and_then(|v| v.as_str()) {
        if owner_id != "0" {
            permissions.insert(owner_id.to_string(), PermissionLevel::Owner);
            
            // Add viewers and editors to permissions map
            for user in get_users_with_permission(&ownership, 2) {
                permissions.insert(user, PermissionLevel::Observer);
            }
            for user in get_users_with_permission(&ownership, 3) {
                permissions.insert(user, PermissionLevel::Limited);
            }
            
            return Some(Ownership {
                owner: owner_id.to_string(),
                permissions,
            });
        }
    }

    Some(Ownership {
        owner: "default".to_string(),
        permissions,
    })
}

fn get_users_with_permission(ownership: &Value, permission_level: u8) -> Vec<String> {
    let mut users = Vec::new();
    
    if let Some(obj) = ownership.as_object() {
        for (user_id, permission) in obj {
            if user_id != "default" {
                if let Some(level) = permission.as_u64() {
                    if level as u8 == permission_level {
                        users.push(user_id.clone());
                    }
                }
            }
        }
    }
    
    users
}
