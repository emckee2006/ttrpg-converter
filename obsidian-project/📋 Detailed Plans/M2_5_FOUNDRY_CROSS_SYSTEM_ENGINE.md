# M2.5: Foundry Cross-System Conversion Engine

**Timeline**: 1 week  
**Status**: 🔴 Blocked (requires M2)  
**Priority**: High - System Conversion Foundation

## Overview

Implement the Foundry cross-system conversion engine using interface abstraction and adapter patterns. This milestone enables seamless conversion between different game systems within the Foundry platform (5e → PF2e, PF1e → PF2e, etc.).

## Key Deliverables

### Days 1-2: Interface Architecture
- 🔲 Implement IFoundryActor, IFoundryItem, IFoundryScene interfaces
- 🔲 Create FoundryActorAdapter pattern for POCO → Interface mapping
- 🔲 Establish base converter interface structure
- 🔲 Unit tests for adapter pattern

### Days 3-4: 5e → PF2e System Conversion
- 🔲 Ability score conversion (6 → 6 + additional PF2e stats)
- 🔲 Class mapping with documentation override support  
- 🔲 Feat system translation (5e features → PF2e feats)
- 🔲 Spell system conversion (spell slots → spell points)
- 🔲 Equipment and inventory translation

### Day 5: Testing & Validation
- 🔲 Integration testing for complete conversion pipeline
- 🔲 Performance validation for adapter overhead
- 🔲 Error handling and validation rules
- 🔲 Documentation for junior developers

## Architecture Implementation

### Interface Pattern
```csharp
public interface IFoundryActor
{
    string Name { get; }
    int Level { get; }
    Dictionary<string, object> Abilities { get; }
    List<IFoundryItem> Items { get; }
}

public class FoundryActorAdapter : IFoundryActor
{
    private readonly Foundry.Dnd5e.Actor _actor;
    
    public FoundryActorAdapter(Foundry.Dnd5e.Actor actor) => _actor = actor;
    
    public string Name => _actor.Name;
    public int Level => _actor.Level;
    // Additional property mappings...
}
```

### System Converter
```csharp
public class SystemConverter
{
    public async Task<T> ConvertAsync<T>(IFoundryActor source, string targetSystem) 
        where T : IFoundryActor
    {
        var converter = _converterFactory.GetConverter(targetSystem);
        return await converter.ConvertActorAsync<T>(source);
    }
}
```

## Success Criteria
- All Foundry POCO types can be wrapped as interfaces
- 5e → PF2e conversion maintains data integrity
- Adapter pattern adds <5% performance overhead
- Interface abstraction enables future system additions

## Dependencies
- **Requires**: M2 (Platform conversion foundation)
- **Enables**: M3 (PF1e → PF2e), cross-system conversions
