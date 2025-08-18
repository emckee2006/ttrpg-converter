//! Spell data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core spell data structure - unified representation across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtrpgSpell {
    /// Spell unique identifier
    pub id: String,
    /// Spell name
    pub name: String,
    /// Spell description
    pub description: String,
    /// Spell level
    pub level: u8,
    /// Spell school
    pub school: SpellSchool,
    /// Casting time
    pub casting_time: Option<String>,
    /// Spell range
    pub range: Option<String>,
    /// Spell components
    pub components: Option<SpellComponents>,
    /// Spell duration
    pub duration: Option<String>,
    /// Ritual spell
    pub ritual: Option<bool>,
    /// Concentration required
    pub concentration: Option<bool>,
    /// Damage
    pub damage: Option<String>,
    /// Save type
    pub save_type: Option<String>,
    /// System-specific additional data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Spell schools across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
    // PF2e specific
    Cantrip,
    Focus,
}

/// Spell components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellComponents {
    /// Verbal component required
    pub verbal: bool,
    /// Somatic component required
    pub somatic: bool,
    /// Material component required
    pub material: bool,
    /// Material component description
    pub material_description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ttrpg_spell_creation() {
        let spell = TtrpgSpell {
            id: "spell_001".to_string(),
            name: "Fireball".to_string(),
            description: "A bright flash of fire erupts from your fingertips".to_string(),
            level: 3,
            school: SpellSchool::Evocation,
            casting_time: Some("1 action".to_string()),
            range: Some("150 feet".to_string()),
            components: Some(SpellComponents {
                verbal: true,
                somatic: true,
                material: true,
                material_description: Some("a tiny ball of bat guano and sulfur".to_string()),
            }),
            duration: Some("Instantaneous".to_string()),
            ritual: Some(false),
            concentration: Some(false),
            damage: Some("8d6 fire".to_string()),
            save_type: Some("Dexterity".to_string()),
            system_data: HashMap::new(),
        };

        assert_eq!(spell.name, "Fireball");
        assert_eq!(spell.level, 3);
        matches!(spell.school, SpellSchool::Evocation);
        assert_eq!(spell.ritual, Some(false));
    }

    #[test]
    fn test_spell_components() {
        let components = SpellComponents {
            verbal: true,
            somatic: false,
            material: true,
            material_description: Some("a pinch of diamond dust worth 25 gp".to_string()),
        };

        assert!(components.verbal);
        assert!(!components.somatic);
        assert!(components.material);
        assert!(components.material_description.is_some());
    }

    #[test]
    fn test_spell_school_variants() {
        let schools = vec![
            SpellSchool::Abjuration,
            SpellSchool::Conjuration,
            SpellSchool::Divination,
            SpellSchool::Enchantment,
            SpellSchool::Evocation,
            SpellSchool::Illusion,
            SpellSchool::Necromancy,
            SpellSchool::Transmutation,
            SpellSchool::Cantrip,
            SpellSchool::Focus,
        ];

        assert_eq!(schools.len(), 10);
        matches!(schools[0], SpellSchool::Abjuration);
        matches!(schools[4], SpellSchool::Evocation);
        matches!(schools[8], SpellSchool::Cantrip);
    }

    #[test]
    fn test_ritual_spell() {
        let spell = TtrpgSpell {
            id: "detect_magic".to_string(),
            name: "Detect Magic".to_string(),
            description: "For the duration, you sense the presence of magic".to_string(),
            level: 1,
            school: SpellSchool::Divination,
            casting_time: Some("1 action".to_string()),
            range: Some("Self".to_string()),
            components: Some(SpellComponents {
                verbal: true,
                somatic: true,
                material: false,
                material_description: None,
            }),
            duration: Some("Concentration, up to 10 minutes".to_string()),
            ritual: Some(true),
            concentration: Some(true),
            damage: None,
            save_type: None,
            system_data: HashMap::new(),
        };

        assert_eq!(spell.ritual, Some(true));
        assert_eq!(spell.concentration, Some(true));
        assert!(spell.damage.is_none());
    }

    #[test]
    fn test_spell_serialization() {
        let spell = TtrpgSpell {
            id: "magic_missile".to_string(),
            name: "Magic Missile".to_string(),
            description: "You create three glowing darts of magical force".to_string(),
            level: 1,
            school: SpellSchool::Evocation,
            casting_time: Some("1 action".to_string()),
            range: Some("120 feet".to_string()),
            components: Some(SpellComponents {
                verbal: true,
                somatic: true,
                material: false,
                material_description: None,
            }),
            duration: Some("Instantaneous".to_string()),
            ritual: Some(false),
            concentration: Some(false),
            damage: Some("1d4+1 force per dart".to_string()),
            save_type: None,
            system_data: HashMap::new(),
        };

        let serialized = serde_json::to_string(&spell).unwrap();
        let deserialized: TtrpgSpell = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(spell.id, deserialized.id);
        assert_eq!(spell.name, deserialized.name);
    }
}