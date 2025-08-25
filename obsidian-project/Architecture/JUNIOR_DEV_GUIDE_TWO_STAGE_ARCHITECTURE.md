# Junior Developer Guide: Two-Stage Architecture

## Overview

The TTRPGConverter uses a **two-stage conversion architecture** to handle platform and system conversions separately:

1. **Stage 1: Platform Conversion** - Format translation (Roll20 → Foundry)
2. **Stage 2: System Conversion** - Rule system translation (5e → PF2e)

## Architecture Principles

### Why Two-Stage?

**Problem**: Roll20 5e → Foundry PF2e involves two different types of complexity:
- **Platform differences**: File formats, coordinate systems, asset handling
- **System differences**: Game rules, character sheets, mechanics

**Solution**: Separate these concerns for:
- Better error isolation
- Easier testing and debugging  
- Code reusability (platform conversion works for any system)
- Cleaner interfaces

### Data Flow

```
Roll20 Campaign → [Platform Conversion] → Foundry 5e → [System Conversion] → Foundry PF2e
```

## Implementation Guide

### 1. Generated Models (M1 Foundation)

#### QuickType Generation Strategy
```powershell
# Generate unified per-system models
quicktype --lang csharp --namespace "TTRPGConverter.Core.Models.Foundry.Dnd5e" 
    --out "FoundryDnd5e.cs" 
    "schemas/foundry/common" "schemas/foundry/core" "schemas/foundry/systems/dnd5e"
```

**Key Points:**
- Each system gets its own namespace (`Foundry.Dnd5e`, `Foundry.Pf2e`)
- Common/core types are duplicated per system to avoid cross-references
- POCOs (Plain Old C# Objects) - no interfaces at generation time

### 2. Platform Conversion (M2)

#### Input: Roll20 ZIP File
```csharp
public class Roll20Parser
{
    public async Task<Roll20Campaign> ParseAsync(string zipPath)
    {
        // Extract ZIP → Parse campaign.json → Deserialize to Roll20 POCOs
    }
}
```

#### Platform Conversion Logic
```csharp
public class PlatformConverter
{
    public async Task<FoundryWorld> ConvertAsync(Roll20Campaign campaign, string targetSystem)
    {
        // Convert coordinate systems
        var scenes = ConvertScenes(campaign.Pages);
        
        // Migrate assets
        var assets = await ProcessAssets(campaign.Assets);
        
        // Convert journal entries
        var journal = ConvertHandouts(campaign.Handouts);
        
        // Generate appropriate Foundry system (5e, pf2e, etc.)
        return new FoundryWorld(scenes, assets, journal, targetSystem);
    }
    
    private List<FoundryScene> ConvertScenes(List<Roll20Page> pages)
    {
        // Roll20 uses pixels, Foundry uses grid units
        // Roll20: (0,0) top-left, Foundry: center-based
    }
}
```

**Output**: Foundry format in the **same system** as input (5e stays 5e)

### 3. System Conversion Engine (M2.5)

#### Interface Abstraction
```csharp
// Interfaces for cross-system compatibility
public interface IFoundryActor
{
    string Name { get; }
    int Level { get; }
    Dictionary<string, object> Abilities { get; }
    List<IFoundryItem> Items { get; }
}

public interface IFoundryItem
{
    string Name { get; }
    string Type { get; }
    Dictionary<string, object> Data { get; }
}
```

#### Adapter Pattern Implementation
```csharp
// Wrap generated POCOs into interfaces
public class FoundryDnd5eActorAdapter : IFoundryActor
{
    private readonly TTRPGConverter.Core.Models.Foundry.Dnd5e.Actor _actor;
    
    public FoundryDnd5eActorAdapter(TTRPGConverter.Core.Models.Foundry.Dnd5e.Actor actor)
    {
        _actor = actor;
    }
    
    public string Name => _actor.Name;
    public int Level => _actor.System?.Details?.Level ?? 1;
    
    // Map 5e-specific data to generic interface
    public Dictionary<string, object> Abilities => new()
    {
        ["strength"] = _actor.System?.Abilities?.Str?.Value ?? 10,
        ["dexterity"] = _actor.System?.Abilities?.Dex?.Value ?? 10,
        // ... more mappings
    };
}
```

#### System Converter
```csharp
public class SystemConverter
{
    public async Task<TPf2eActor> Convert5eToPf2eAsync<TPf2eActor>(IFoundryActor dnd5eActor) 
        where TPf2eActor : IFoundryActor, new()
    {
        var pf2eActor = new TPf2eActor();
        
        // Convert ability scores (same 6 abilities)
        pf2eActor.Abilities = ConvertAbilities(dnd5eActor.Abilities);
        
        // Convert class system
        pf2eActor.Class = MapDnd5eClassToPf2e(dnd5eActor.Class);
        
        // Convert spell system (spell slots → spell points)
        pf2eActor.Spells = ConvertSpellSystem(dnd5eActor.Spells);
        
        return pf2eActor;
    }
}
```

### 4. Complete Pipeline Usage

```csharp
public class ConversionService
{
    public async Task<string> ConvertAsync(string inputPath, ConversionType type)
    {
        switch (type)
        {
            case ConversionType.Roll20ToFoundry5e:
                // Stage 1 only - platform conversion
                var roll20Data = await _roll20Parser.ParseAsync(inputPath);
                var foundry5e = await _platformConverter.ConvertAsync(roll20Data, "dnd5e");
                return await _foundryWriter.WriteAsync(foundry5e);
                
            case ConversionType.Roll20ToFoundryPf2e:
                // Stage 1 + Stage 2 - full pipeline
                var roll20Campaign = await _roll20Parser.ParseAsync(inputPath);
                var foundryDnd5e = await _platformConverter.ConvertAsync(roll20Campaign, "dnd5e");
                
                // Convert POCOs to interfaces
                var dnd5eActors = foundryDnd5e.Actors.Select(a => new FoundryDnd5eActorAdapter(a));
                
                // System conversion
                var pf2eActors = await _systemConverter.ConvertActorsAsync<FoundryPf2eActor>(dnd5eActors);
                
                var foundryPf2e = new FoundryWorld(foundryDnd5e.Scenes, foundryDnd5e.Assets, pf2eActors);
                return await _foundryWriter.WriteAsync(foundryPf2e);
                
            case ConversionType.FoundryPf1eToPf2e:
                // Stage 2 only - system conversion
                var pf1eWorld = await _foundryReader.ReadAsync(inputPath);
                var pf1eActors = pf1eWorld.Actors.Select(a => new FoundryPf1eActorAdapter(a));
                var pf2eActors = await _systemConverter.ConvertActorsAsync<FoundryPf2eActor>(pf1eActors);
                
                var pf2eWorld = new FoundryWorld(pf1eWorld.Scenes, pf1eWorld.Assets, pf2eActors);
                return await _foundryWriter.WriteAsync(pf2eWorld);
        }
    }
}
```

## Testing Strategy

### Unit Testing
```csharp
[Test]
public void Roll20Parser_ShouldParseValidCampaign()
{
    // Test Stage 1 platform conversion only
}

[Test]  
public void SystemConverter_ShouldConvert5eToPf2e()
{
    // Test Stage 2 system conversion only
}
```

### Integration Testing
```csharp
[Test]
public void FullPipeline_ShouldConvertRoll20ToPf2e()
{
    // Test complete Stage 1 + Stage 2 pipeline
}
```

## Common Pitfalls

### ❌ Don't Mix Stages
```csharp
// WRONG: Trying to do platform + system conversion in one step
public FoundryPf2eWorld ConvertDirectly(Roll20Campaign campaign)
{
    // This mixes platform and system concerns - hard to debug and test
}
```

### ✅ Keep Stages Separate
```csharp
// RIGHT: Clear separation of concerns
var foundryDnd5e = await _platformConverter.ConvertAsync(campaign, "dnd5e");
var foundryPf2e = await _systemConverter.ConvertSystemAsync(foundryDnd5e, "pf2e");
```

### ❌ Don't Bypass Interfaces for Cross-System
```csharp
// WRONG: Direct POCO-to-POCO conversion
public FoundryPf2eActor Convert(FoundryDnd5eActor dnd5eActor)
{
    // This creates tight coupling between system types
}
```

### ✅ Use Interface Abstraction
```csharp
// RIGHT: Interface-mediated conversion
public TPf2eActor ConvertAsync<TPf2eActor>(IFoundryActor sourceActor) where TPf2eActor : IFoundryActor
{
    // Loose coupling via interfaces
}
```

## Debugging Tips

### Stage 1 Issues (Platform Conversion)
- Check coordinate system math (Roll20 pixels → Foundry grid)
- Validate asset paths and URLs
- Verify JSON structure matches Foundry V13 format

### Stage 2 Issues (System Conversion)  
- Verify adapter mappings are complete
- Check interface implementations
- Validate system-specific rule conversions (spell slots → spell points)

### Performance Monitoring
- Stage 1: Asset downloading is bottleneck
- Stage 2: Complex rule conversions can be CPU intensive
- Use progress reporting to isolate slow operations

## Next Steps for Junior Developers

1. **Start with M2**: Focus on platform conversion (Stage 1)
2. **Master interfaces**: Understand adapter pattern for Stage 2
3. **Learn system rules**: Study 5e → PF2e conversion mappings
4. **Practice testing**: Write unit tests for individual components
5. **Build end-to-end**: Combine stages for complete pipelines
