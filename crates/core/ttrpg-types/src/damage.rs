//! Damage and combat data structures

use serde::{Deserialize, Serialize};
use crate::enums::DamageType;

/// Damage information for items and spells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageInfo {
    /// Damage dice formula (e.g., "1d8", "2d6+3")
    pub dice: String,
    /// Damage type
    pub damage_type: DamageType,
    /// Additional damage bonus
    pub bonus: Option<i32>,
    /// Whether damage is versatile
    pub versatile: Option<String>,
}

/// Armor class information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorClass {
    /// Base AC value
    pub base: u8,
    /// AC bonus from dexterity
    pub dex_bonus: Option<i8>,
    /// Maximum dex bonus allowed
    pub max_dex: Option<i8>,
    /// AC calculation formula
    pub formula: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_info_creation() {
        let damage = DamageInfo {
            dice: "1d8".to_string(),
            damage_type: DamageType::Slashing,
            bonus: Some(3),
            versatile: Some("1d10".to_string()),
        };

        assert_eq!(damage.dice, "1d8");
        assert_eq!(damage.bonus, Some(3));
        assert!(damage.versatile.is_some());
    }

    #[test]
    fn test_armor_class_creation() {
        let ac = ArmorClass {
            base: 16,
            dex_bonus: Some(2),
            max_dex: Some(2),
            formula: Some("10 + armor + dex".to_string()),
        };

        assert_eq!(ac.base, 16);
        assert_eq!(ac.dex_bonus, Some(2));
    }
}
