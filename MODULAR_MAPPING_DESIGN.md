# Modular Platform Mapping Design

## Overview
Split large `mappings.rs` files into focused modules per entity type for maintainability, testability, and parallel development.

## File Structure
```
crates/platforms/ttrpg-{platform}/src/
├── lib.rs                    # Re-exports all mapping modules
├── mappings/
│   ├── mod.rs               # Module declarations and shared utilities
│   ├── character.rs         # Character mappings
│   ├── item.rs              # Item mappings
│   ├── scene.rs             # Scene mappings
│   ├── spell.rs             # Spell mappings
│   ├── journal.rs           # Journal mappings
│   ├── campaign.rs          # Campaign mappings
│   └── helpers/
│       ├── mod.rs           # Helper function module declarations
│       ├── conversion.rs    # Common conversion utilities
│       ├── validation.rs    # Data validation helpers
│       └── platform_specific.rs  # Platform-specific utilities
```

## Module Responsibilities

### `character.rs`
- `impl From<PlatformCharacter> for TtrpgCharacter`
- Character-specific helper functions
- Ability score mappings
- Skill conversions
- HP calculations

### `item.rs`
- `impl From<PlatformItem> for TtrpgItem`
- Item type mappings
- Currency conversions
- Weight/bulk calculations
- Rarity mappings

### `scene.rs`
- `impl From<PlatformScene> for TtrpgScene`
- Grid configuration mappings
- Token conversions
- Lighting calculations
- Background image handling

### `spell.rs`
- `impl From<PlatformSpell> for TtrpgSpell`
- Spell school mappings
- Component conversions
- Duration/range parsing
- Spell level normalization

### `journal.rs`
- `impl From<PlatformJournal> for TtrpgJournal`
- Content formatting
- Player visibility mappings
- Page structure handling

### `campaign.rs`
- `impl From<PlatformCampaign> for TtrpgCampaign`
- Metadata mappings
- Collection aggregation
- Version handling

### `helpers/conversion.rs`
```rust
// Common conversion utilities shared across modules
pub fn parse_dice_notation(input: &str) -> Option<DiceRoll>
pub fn normalize_currency(value: f64, from: &str, to: &str) -> f64
pub fn parse_duration(input: &str) -> Option<Duration>
pub fn sanitize_html(input: &str) -> String
```

### `helpers/validation.rs`
```rust
// Data validation helpers
pub fn validate_character_level(level: i32) -> Result<u8, ValidationError>
pub fn validate_ability_score(score: i32) -> Result<u8, ValidationError>
pub fn validate_currency_amount(amount: f64) -> Result<f64, ValidationError>
```

### `helpers/platform_specific.rs`
```rust
// Platform-specific utilities
pub fn parse_roll20_id(id: &str) -> Result<String, ParseError>
pub fn convert_foundry_permission(permission: u8) -> PermissionLevel
pub fn parse_pathbuilder_uuid(uuid: &str) -> Result<String, UuidError>
```

## Import Strategy
Use qualified imports from generated schemas:
```rust
use crate::generated::character::Character as PlatformCharacter;
use crate::generated::item::Item as PlatformItem;
use ttrpg_types::{TtrpgCharacter, TtrpgItem};
use super::helpers::conversion::{parse_dice_notation, normalize_currency};
```

## Testing Strategy
Each module gets its own test suite:
```rust
// In character.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_character_conversion() { /* ... */ }
    
    #[test]
    fn test_ability_score_mapping() { /* ... */ }
    
    #[test]
    fn test_skill_conversion() { /* ... */ }
}
```

## Benefits

### **Maintainability**
- Small, focused files (200-400 lines vs 1000+ lines)
- Clear separation of concerns
- Easy to locate and modify specific entity mappings

### **Parallel Development**
- Different developers can work on different entity types
- Reduced merge conflicts
- Independent testing and validation

### **Code Reuse**
- Shared helper functions across entity types
- Platform-specific utilities can be centralized
- Common validation logic in one place

### **Testing**
- Focused test suites per entity type
- Helper function unit tests
- Better test coverage visibility

### **Performance**
- Conditional compilation possible per entity
- Smaller compilation units
- Better incremental builds

## Migration Plan
1. Create modular structure with placeholder files
2. Move existing mappings to appropriate modules
3. Extract common functionality to helpers
4. Add comprehensive tests per module
5. Update lib.rs to re-export all modules
6. Validate compilation and test coverage

## Implementation Notes
- Each module should be ~200-400 lines max
- Helper functions should be pure functions where possible
- Use type aliases for generated schema types for clarity
- Include comprehensive error handling
- Add doc comments for all public functions
