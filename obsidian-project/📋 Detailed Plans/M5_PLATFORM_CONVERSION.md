# M5: Platform Conversion (Eliminated - Absorbed into M2)

**Timeline**: 1.5 weeks  
**Status**: ✅ Completed via M2 Platform Conversion Architecture  
**Priority**: Medium - Platform Migration
**Rationale**: Two-stage architecture makes separate platform milestone redundant

## Overview

Convert Roll20 campaigns to Foundry VTT while preserving the original game system (no system conversion). Focus on platform-specific format translation, coordinate system conversion, and asset migration.

## Key Deliverables

### Week 1: Format Translation
- 🔲 Roll20 → Foundry platform conversion (D&D 5e stays 5e)
- 🔲 Scene coordinate system translation
- 🔲 Token positioning and scaling conversion
- 🔲 Journal entry format migration

### Week 1.5: System Detection & Assets
- 🔲 Automatic game system detection from Roll20 data
- 🔲 System-specific Foundry world template selection
- 🔲 Asset path translation and validation
- 🔲 Lighting and wall data conversion

## Architecture Decision

The two-stage conversion approach eliminates the need for a separate platform conversion milestone:

### Stage 1: Platform Conversion (M2)
- Roll20 → Foundry format translation
- Coordinate system conversion
- Asset processing pipeline
- **Preserves original game system** (5e stays 5e)

### Stage 2: System Conversion (M2.5)
- Foundry 5e → Foundry PF2e conversion
- Interface-based cross-system translation
- System-specific rule mappings

## Integration Benefits

1. **Simplified Workflow**: Single pipeline handles both platform and system conversion
2. **Reduced Redundancy**: No duplicate coordinate/asset processing logic
3. **Better Error Handling**: Unified error reporting across conversion stages
4. **Improved Testing**: Combined integration testing catches more edge cases

## Migration Path
- **M5 Tasks → M2**: Coordinate translation, asset processing
- **M5 Logic → M2.5**: Cross-system conversion engine
- **Timeline Savings**: 1 week eliminated, redistributed to M2/M2.5

## Platform Conversion Architecture

### System Detection
```csharp
public class GameSystemDetector
{
    public GameSystem DetectSystem(Roll20Campaign campaign)
    {
        // Analyze character sheet structure
        var sheetType = AnalyzeCharacterSheets(campaign.Characters);
        
        return sheetType switch
        {
            "dnd5e" => GameSystem.Dnd5e,
            "pf2e" => GameSystem.Pf2e,
            "pf1e" => GameSystem.Pf1e,
            "ose" => GameSystem.OldSchoolEssentials,
            _ => GameSystem.Generic
        };
    }
}
```

### Platform Converter
```csharp
public class PlatformConverter
{
    public async Task<FoundryWorld> ConvertToFoundryAsync(
        Roll20Campaign r20Campaign, 
        GameSystem detectedSystem)
    {
        return detectedSystem switch
        {
            GameSystem.Dnd5e => await ConvertDnd5eToFoundry(r20Campaign),
            GameSystem.Pf2e => await ConvertPf2eToFoundry(r20Campaign),
            GameSystem.Pf1e => await ConvertPf1eToFoundry(r20Campaign),
            _ => await ConvertGenericToFoundry(r20Campaign)
        };
    }
}
```

## Coordinate System Translation

### Roll20 vs Foundry Coordinates
```
Roll20: Grid units with variable pixels per unit
Foundry: Standardized grid with consistent scaling

Conversion Formula:
foundryX = (roll20X / roll20GridSize) * foundryGridSize
foundryY = (roll20Y / roll20GridSize) * foundryGridSize
```

### Scene Conversion
```csharp
public class SceneConverter
{
    public FoundryScene ConvertScene(Roll20Page r20Page)
    {
        return new FoundryScene
        {
            Name = r20Page.Name,
            Width = ConvertDimension(r20Page.Width, r20Page.Scale),
            Height = ConvertDimension(r20Page.Height, r20Page.Scale),
            Grid = new FoundryGrid
            {
                Size = DetermineFoundryGridSize(r20Page.GridType),
                Type = ConvertGridType(r20Page.GridType)
            },
            Tokens = r20Page.Tokens.Select(ConvertToken).ToList(),
            Walls = r20Page.DynamicLighting?.Walls.Select(ConvertWall).ToList()
        };
    }
}
```

## System-Specific Conversions

### D&D 5e Platform Migration
- Character sheet data structure preservation
- Spell slot formatting compatibility
- Initiative and combat tracker setup
- Foundry 5e system module configuration

### Pathfinder 2e Platform Migration  
- Action economy display preservation
- Three-action system visualization
- Conditions and effects translation
- PF2e system module integration

### Generic System Fallback
```csharp
public class GenericSystemConverter
{
    public FoundryActor ConvertGenericCharacter(Roll20Character character)
    {
        // Convert to simple Foundry actor
        // Preserve attributes as key-value pairs
        // Maintain basic functionality without system specifics
        return new FoundryActor
        {
            Name = character.Name,
            Type = "character",
            System = new Dictionary<string, object>(character.Attributes)
        };
    }
}
```

## Asset Migration

### Path Translation
```csharp
public class AssetPathConverter
{
    public string ConvertRoll20AssetPath(string roll20Path)
    {
        // Roll20: https://s3.amazonaws.com/files.d20.io/images/...
        // Foundry: worlds/[world]/assets/images/...
        
        if (roll20Path.StartsWith("https://s3.amazonaws.com/files.d20.io/"))
        {
            return ConvertToLocalFoundryPath(roll20Path);
        }
        
        return roll20Path; // Keep external URLs as-is
    }
}
```

### Asset Download Strategy
- Download Roll20 hosted assets locally
- Maintain original folder structure where possible
- Generate asset manifest for reference
- Validate asset accessibility and format

## Quality Assurance

### Validation Tests
- Scene dimensions and token positions accurate
- All character data preserved correctly
- Asset links functional in Foundry
- System detection accuracy >95%

### Integration Tests
- Full campaign conversion end-to-end
- Multi-scene world with complex layouts
- Character sheets with extensive data
- Asset-heavy campaigns with images/audio

## Success Criteria

- [ ] All supported game systems detect correctly
- [ ] Scene layouts preserve visual accuracy
- [ ] Character data transfers without loss
- [ ] Assets download and display properly
- [ ] Conversion time <3 minutes for typical campaign
- [ ] Generated world loads without errors in Foundry

## Supported Game Systems

**Primary Support:**
- D&D 5th Edition
- Pathfinder 2nd Edition
- Pathfinder 1st Edition

**Secondary Support:**
- Old School Essentials
- Generic/Custom systems

## Next Milestone

→ M6: GUI Application with user-friendly interface (2 weeks)
