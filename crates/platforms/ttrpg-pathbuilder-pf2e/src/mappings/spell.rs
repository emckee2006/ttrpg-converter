//! Spell mappings for Pathbuilder PF2e

use crate::schemas::custom_spell::CustomSpell;
use ttrpg_types::spell::{TtrpgSpell, SpellComponents, SpellSchool};
use std::collections::HashMap;
use serde_json;

/// Convert Pathbuilder CustomSpell to unified TtrpgSpell
impl From<CustomSpell> for TtrpgSpell {
    fn from(spell: CustomSpell) -> Self {
        let mut system_data = HashMap::new();
        system_data.insert("timestamp".to_string(), serde_json::json!(format!("{:?}", spell.timestamp)));
        
        Self {
            id: String::from(spell.id),
            name: String::from(spell.name),
            description: String::from(spell.description),
            level: spell.level as u8, // Direct i64 to u8 cast
            school: SpellSchool::Evocation, // Default school
            casting_time: None, // TODO: Parse from cast field
            range: None, // TODO: Parse from range field
            components: Some(SpellComponents {
                verbal: false,
                somatic: false,
                material: false,
                material_description: None,
            }),
            duration: None, // TODO: Parse from duration field
            ritual: Some(false), // PF2e doesn't use ritual spells
            concentration: Some(false), // PF2e uses sustained spells instead
            damage: None, // TODO: Parse from spell effects
            save_type: None, // TODO: Parse from spell description
            system_data,
        }
    }
}
