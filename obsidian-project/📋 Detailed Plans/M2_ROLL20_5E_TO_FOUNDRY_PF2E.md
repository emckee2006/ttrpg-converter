# M2: Roll20 5e → Foundry PF2e Conversion

**Timeline**: 3 weeks  
**Status**: 🔴 Blocked (requires M1)  
**Priority**: Critical Path - First Working Pipeline

## Overview

Implement the primary conversion pipeline: Roll20 D&D 5e campaigns to Foundry VTT Pathfinder 2e format. This includes complex system conversion rules, documentation-driven mapping, and asset processing.

## Key Deliverables

### Week 1: Roll20 Parsing
- 🔲 ZIP file extraction and validation
- 🔲 Campaign.json deserialization to Roll20Campaign model
- 🔲 Character sheet parsing with attribute mapping
- 🔲 Handout and journal entry extraction

### Week 2: System Conversion (5e → PF2e)
- 🔲 Ability score conversion (6 → 6 + additional PF2e stats)
- 🔲 Class mapping with documentation override support
- 🔲 Feat system translation (5e features → PF2e feats)
- 🔲 Spell system conversion (spell slots → spell points)
- 🔲 Equipment and inventory translation

### Week 3: Foundry Output & Assets
- 🔲 FoundryWorld generation with V13 format
- 🔲 Asset downloading and optimization (ImageSharp)
- 🔲 Scene coordinate system conversion
- 🔲 Comprehensive integration testing

## Conversion Architecture

### Input Processing
```csharp
public class Roll20Parser
{
    public async Task<Roll20Campaign> ParseAsync(string zipPath)
    {
        // Extract ZIP, validate structure
        // Parse campaign.json
        // Load character sheets, handouts, assets
        return campaign;
    }
}
```

### System Conversion
```csharp
public class Dnd5eToPf2eConverter
{
    public async Task<FoundryPf2eActor> ConvertCharacterAsync(
        Roll20Character character, 
        ConversionDocumentation docs)
    {
        return character.CharacterClass switch
        {
            "Fighter" => ConvertFighter(character, docs),
            "Wizard" => ConvertWizard(character, docs),
            _ => ConvertGenericClass(character, docs)
        };
    }
}
```

### Documentation Integration
```csharp
public class ConversionDocumentation
{
    // Load from campaign handouts and external guides
    public ClassMapping GetClassMapping(string className);
    public FeatMapping GetFeatMapping(string featName);
    public SpellMapping GetSpellMapping(string spellName);
}
```

## Complex Mappings

### Ability Scores
```
D&D 5e (6 stats) → PF2e (6 + derived)
STR, DEX, CON, INT, WIS, CHA → Same + derived modifiers
```

### Action Economy  
```
5e: Action, Bonus Action, Movement → PF2e: 3 Actions + Reactions
```

### Spell System
```
5e: Spell Slots by Level → PF2e: Spell Points + Focus Points
```

### Class Features
```
5e: Class Features → PF2e: Class Feats + Ancestry Feats + Skill Feats
```

## Asset Processing

### Image Optimization
- Download Roll20 character portraits and tokens
- Convert to WebP format with ImageSharp
- Generate multiple resolutions (thumb, full)
- Integrate with Foundry asset references

### Audio Processing
- Extract background music and sound effects
- Convert to web-compatible formats
- Maintain relative path structure

## Testing Strategy

### Unit Tests
- Individual conversion functions
- Edge cases and error handling
- Documentation parser validation

### Integration Tests  
- Full pipeline with real Roll20 campaigns
- Asset download and processing
- Foundry world validation

### Property-Based Tests
- Conversion reversibility where applicable
- Data integrity preservation
- Performance benchmarks

## Success Criteria

- [ ] Successfully convert complete Roll20 5e campaign
- [ ] All characters converted with <5% data loss
- [ ] Assets downloaded and optimized correctly  
- [ ] Generated Foundry world loads without errors
- [ ] Conversion time <2 minutes for typical campaign
- [ ] Documentation overrides applied correctly

## Input Sources

- Roll20 campaign ZIP exports
- Campaign-specific conversion guides in handouts
- Generic 5e→PF2e mapping documentation
- Asset URLs from Roll20 CDN

## Next Milestone

→ M3: Foundry PF1e → PF2e conversion (2.5 weeks)
