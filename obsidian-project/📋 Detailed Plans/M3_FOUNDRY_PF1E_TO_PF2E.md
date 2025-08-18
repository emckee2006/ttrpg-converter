# M3: Foundry PF1e → PF2e Conversion

**Timeline**: 2.5 weeks  
**Status**: 🔴 Blocked (requires M2)  
**Priority**: High - Complex System Conversion

## Overview

Convert existing Foundry VTT Pathfinder 1e campaigns to Pathfinder 2e format. This involves the most complex rule system conversion due to fundamental differences between PF1e and PF2e mechanics.

## Key Deliverables

### Week 1: PF1e Input Processing
- 🔲 Foundry world file parsing and validation
- 🔲 PF1e actor data structure understanding
- 🔲 Legacy system data migration preparation
- 🔲 Asset inventory and compatibility check

### Week 2: Core System Translation
- 🔲 Ability score system conversion
- 🔲 Skill system overhaul (many-to-few mapping)
- 🔲 Class feature translation with documentation
- 🔲 Feat system complete restructuring

### Week 2.5: Advanced Conversions
- 🔲 Spell system migration (spell schools → traditions)
- 🔲 Action economy conversion (full/standard → 3-action)
- 🔲 Combat maneuver system translation
- 🔲 Equipment and magic item conversion

## System Conversion Challenges

### Ability Scores
```
PF1e: STR, DEX, CON, INT, WIS, CHA (3d6 range, modifiers every 2 points)
PF2e: Same stats, different modifier calculation and caps
```

### Skill System Overhaul
```
PF1e: 35+ skills → PF2e: 17 skills
- Appraise → Crafting
- Climb, Swim → Athletics  
- Hide, Move Silently → Stealth
- Spot, Listen, Search → Perception
- Many skills consolidated or removed
```

### Action Economy Revolution
```
PF1e: Standard + Move + Swift + Free actions
PF2e: 3 Actions + 1 Reaction + Free actions

Examples:
- Full Attack → 2-3 actions for multiple strikes
- Spell + Move → 2 actions (Cast + Stride)
- Complex maneuvers → Multiple action combinations
```

### Class System Changes
```
PF1e: Base classes with archetypes
PF2e: Classes with modular feat choices

Fighter Example:
PF1e: Base Attack Bonus progression + feats
PF2e: Class DC + class feats + general feats
```

## Conversion Architecture

### Input Processing
```csharp
public class FoundryPf1eParser
{
    public async Task<FoundryPf1eWorld> ParseWorldAsync(string worldPath)
    {
        // Load world.json, actors, items, scenes
        // Validate PF1e system data structure
        return world;
    }
}
```

### System Converter
```csharp
public class Pf1eToPf2eConverter
{
    private readonly Pf1eToPf2eDocumentation _docs;
    
    public async Task<FoundryPf2eActor> ConvertActorAsync(FoundryPf1eActor pf1eActor)
    {
        return pf1eActor.System.Class.Name switch
        {
            "Fighter" => ConvertFighter(pf1eActor),
            "Wizard" => ConvertWizard(pf1eActor),
            "Rogue" => ConvertRogue(pf1eActor),
            _ => ConvertGenericClass(pf1eActor)
        };
    }
    
    private FoundryPf2eActor ConvertFighter(FoundryPf1eActor pf1e)
    {
        // Convert BAB to class DC
        // Translate fighter bonus feats to class feats
        // Convert weapon specializations
        return new FoundryPf2eActor(/* ... */);
    }
}
```

### Documentation-Driven Mapping
```csharp
public class Pf1eToPf2eDocumentation
{
    // Load conversion guides from campaign journals
    public ClassConversionGuide GetClassGuide(string className);
    public FeatMapping GetFeatMapping(string pf1eFeat);
    public SpellConversion GetSpellConversion(string spellName);
    public SkillMapping GetSkillMapping(string pf1eSkill);
}
```

## Complex Mappings

### Skills Consolidation
```csharp
var skillMappings = new Dictionary<string, string>
{
    ["Climb"] = "Athletics",
    ["Swim"] = "Athletics", 
    ["Jump"] = "Athletics",
    ["Hide"] = "Stealth",
    ["Move Silently"] = "Stealth",
    ["Spot"] = "Perception",
    ["Listen"] = "Perception",
    ["Search"] = "Perception",
    ["Appraise"] = "Crafting",
    // Many more...
};
```

### Feat System Translation
```csharp
public class FeatConverter
{
    public List<FoundryPf2eFeat> ConvertPf1eFeats(List<FoundryPf1eFeat> pf1eFeats)
    {
        // Many PF1e feats become class features
        // Some become general feats
        // Prerequisites completely different
        // Some feats split into multiple PF2e feats
    }
}
```

### Spell School → Tradition Mapping
```
PF1e Schools → PF2e Traditions:
- Evocation, Transmutation → Arcane/Primal
- Necromancy, Enchantment → Occult/Divine  
- Multiple valid traditions for many spells
```

## Testing Strategy

### Conversion Accuracy Tests
- Character level progression validation
- Skill point allocation verification
- Feat prerequisite chain validation
- Spell slot/preparation system accuracy

### Integration Tests
- Full party conversion
- Multi-class character handling
- Prestige class translation
- Archetype system migration

## Success Criteria

- [ ] Complete PF1e party converts successfully
- [ ] Character power level approximately maintained
- [ ] All major class features translated or documented
- [ ] Skill system conversion preserves character competencies
- [ ] Action economy conversion maintains tactical options
- [ ] Generated PF2e characters are playable in Foundry

## Documentation Dependencies

- Official PF1e → PF2e conversion guides
- Campaign-specific character notes
- House rule documentation
- Custom content translation guides

## Next Milestone

→ M4: Enhancement Pipeline with compendium matching (2 weeks)
