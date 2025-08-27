# M2.5: System Conversion Engine (5e to PF2e)

**Author:** Gemini & Alithanna
**Date:** 2024-07-26
**Status:** Future Scope (Blocked by M2 Stage 1)
**Depends On:** `M2_ROLL20_5E_TO_FOUNDRY_PF2E.md`

## 1. Overview

This document outlines the implementation plan for the **Stage 2** system conversion engine, a core component of the M2 Milestone. The primary goal is to convert characters and game mechanics from the D&D 5e system to the Pathfinder 2e system within the Foundry VTT platform.

The architecture will be built around a reusable, interface-driven adapter pattern. This ensures that the engine we build for this 5e-to-PF2e conversion can be extended in the future to support other system conversions (e.g., the PF1e-to-PF2e conversion planned for M3).

## 2. Key Deliverables

### Phase 1: Interface Architecture
- 🔲 Implement `IFoundryActor`, `IFoundryItem`, `IFoundryScene` interfaces to abstract away system-specific data structures.
- 🔲 Create a `FoundryActorAdapter` that wraps a `Dnd5eActor` and exposes its data through the `IFoundryActor` interface.
- 🔲 Establish the base `ISystemConverter` interface structure.
- _ Unit tests for the adapter pattern.

### Phase 2: 5e → PF2e System Conversion Logic
- 🔲 Implement the `Dnd5eToPf2eConverter` service.
- 🔲 Map ability scores (6 stats → 6 stats with different modifiers).
- 🔲 Map class features to PF2e class feats, ancestry feats, and skill feats.
- 🔲 Map the 5e spell slot system to the PF2e spell point system.
- 🔲 Translate the 5e action economy (Action, Bonus Action) to the PF2e 3-action economy.

### Phase 3: Testing & Validation
- 🔲 Integration testing for the complete 5e-to-PF2e conversion pipeline.
- 🔲 Performance validation for the adapter pattern overhead.
- 🔲 Error handling and validation for unsupported or ambiguous mappings.

## 3. Architecture Implementation

### Interface Pattern
```csharp
public interface IFoundryActor
{
    string Name { get; }
    int Level { get; }
    IReadOnlyDictionary<string, int> Abilities { get; }
    IReadOnlyList<IFoundryItem> Items { get; }
}

public class Dnd5eActorAdapter : IFoundryActor
{
    private readonly Foundry.Dnd5e.Actor _actor;
    
    public Dnd5eActorAdapter(Foundry.Dnd5e.Actor actor) => _actor = actor;
    
    public string Name => _actor.Name;
    public int Level => _actor.Data.Details.Level.Value;
    // ... additional property mappings ...
}
```

### System Converter
```csharp
public interface ISystemConverter
{
    Task<FoundryPf2eActor> ConvertActorAsync(IFoundryActor sourceActor);
}

public class Dnd5eToPf2eConverter : ISystemConverter
{
    public async Task<FoundryPf2eActor> ConvertActorAsync(IFoundryActor sourceActor)
    {
        // ... conversion logic ...
    }
}
```

This plan provides a clear roadmap for implementing the second major stage of our primary conversion pipeline.
