//! Common enumeration types

use serde::{Deserialize, Serialize};

/// Game systems supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameSystem {
    DND5e,
    PF2e,
    PF1e,
    OSE,
    Roll20Generic,
    PathbuilderPF2e,
}

/// Source formats for import
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceFormat {
    Roll20,
    FoundryV9,
    FoundryV10,
    FoundryV11,
    FoundryV12,
    PathbuilderJson,
}

/// Target formats for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetFormat {
    FoundryV12,
    Roll20,
    Json,
    PDF,
}

/// Damage types across systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Bludgeoning,
    Piercing,
    Slashing,
    Fire,
    Cold,
    Lightning,
    Thunder,
    Poison,
    Acid,
    Necrotic,
    Radiant,
    Psychic,
    Force,
}

/// Creature sizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
}

/// Feat types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatType {
    General,
    Combat,
    Skill,
    Heritage,
    Ancestry,
    Class,
}

/// Actor/Character types across systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActorType {
    /// Player Character
    Character,
    /// Non-Player Character
    NPC,
    /// Monster/Creature
    Monster,
    /// Vehicle (ship, mount, etc.)
    Vehicle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_system_variants() {
        let systems = vec![
            GameSystem::DND5e,
            GameSystem::PF1e,
            GameSystem::PF2e,
            GameSystem::OSE,
            GameSystem::Roll20Generic,
            GameSystem::PathbuilderPF2e,
        ];

        assert_eq!(systems.len(), 6);
        matches!(systems[0], GameSystem::DND5e);
        matches!(systems[2], GameSystem::PF2e);
        matches!(systems[5], GameSystem::PathbuilderPF2e);
    }

    #[test]
    fn test_source_format_variants() {
        let formats = vec![
            SourceFormat::Roll20,
            SourceFormat::FoundryV9,
            SourceFormat::FoundryV10,
            SourceFormat::FoundryV11,
            SourceFormat::FoundryV12,
            SourceFormat::PathbuilderJson,
        ];

        assert_eq!(formats.len(), 6);
        matches!(formats[0], SourceFormat::Roll20);
        matches!(formats[1], SourceFormat::FoundryV9);
        matches!(formats[2], SourceFormat::FoundryV10);
    }

    #[test]
    fn test_target_format_variants() {
        let formats = vec![
            TargetFormat::FoundryV12,
            TargetFormat::Roll20,
            TargetFormat::Json,
            TargetFormat::PDF,
        ];

        assert_eq!(formats.len(), 4);
        matches!(formats[0], TargetFormat::FoundryV12);
        matches!(formats[1], TargetFormat::Roll20);
    }

    #[test]
    fn test_damage_type_variants() {
        let damage_types = vec![
            DamageType::Bludgeoning,
            DamageType::Piercing,
            DamageType::Slashing,
            DamageType::Fire,
            DamageType::Cold,
            DamageType::Lightning,
            DamageType::Thunder,
            DamageType::Poison,
            DamageType::Acid,
            DamageType::Necrotic,
            DamageType::Radiant,
            DamageType::Psychic,
            DamageType::Force,
        ];

        assert_eq!(damage_types.len(), 13);
        matches!(damage_types[0], DamageType::Bludgeoning);
        matches!(damage_types[3], DamageType::Fire);
        matches!(damage_types[12], DamageType::Force);
    }

    #[test]
    fn test_creature_size_variants() {
        let sizes = vec![
            CreatureSize::Tiny,
            CreatureSize::Small,
            CreatureSize::Medium,
            CreatureSize::Large,
            CreatureSize::Huge,
            CreatureSize::Gargantuan,
        ];

        assert_eq!(sizes.len(), 6);
        matches!(sizes[0], CreatureSize::Tiny);
        matches!(sizes[2], CreatureSize::Medium);
        matches!(sizes[5], CreatureSize::Gargantuan);
    }

    #[test]
    fn test_feat_type_variants() {
        let feat_types = vec![
            FeatType::General,
            FeatType::Combat,
            FeatType::Skill,
            FeatType::Heritage,
            FeatType::Ancestry,
            FeatType::Class,
        ];

        assert_eq!(feat_types.len(), 6);
        matches!(feat_types[0], FeatType::General);
        matches!(feat_types[1], FeatType::Combat);
        matches!(feat_types[4], FeatType::Ancestry);
    }

    #[test]
    fn test_actor_type_variants() {
        let actor_types = vec![
            ActorType::Character,
            ActorType::NPC,
            ActorType::Monster,
            ActorType::Vehicle,
        ];

        assert_eq!(actor_types.len(), 4);
        matches!(actor_types[0], ActorType::Character);
        matches!(actor_types[1], ActorType::NPC);
        matches!(actor_types[2], ActorType::Monster);
        matches!(actor_types[3], ActorType::Vehicle);
    }

    #[test]
    fn test_enum_serialization() {
        let system = GameSystem::DND5e;
        let format = SourceFormat::Roll20;
        let damage = DamageType::Fire;
        
        let system_json = serde_json::to_string(&system).unwrap();
        let format_json = serde_json::to_string(&format).unwrap();
        let damage_json = serde_json::to_string(&damage).unwrap();
        
        let deserialized_system: GameSystem = serde_json::from_str(&system_json).unwrap();
        let deserialized_format: SourceFormat = serde_json::from_str(&format_json).unwrap();
        let deserialized_damage: DamageType = serde_json::from_str(&damage_json).unwrap();
        
        matches!(deserialized_system, GameSystem::DND5e);
        matches!(deserialized_format, SourceFormat::Roll20);
        matches!(deserialized_damage, DamageType::Fire);
    }
}