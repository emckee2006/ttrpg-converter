# M1: C# Foundation & Schema Generation

**Timeline**: 2 weeks  
**Status**: ✅ **COMPLETED**  
**Priority**: Critical Path

## Overview

Establish C# project structure with type-safe models generated from JSON schemas, dependency injection framework, and comprehensive testing foundation.

## Key Deliverables

### Week 1: Project Structure
- ✅ Solution file with 6 C# projects
- ✅ NuGet package management setup
- ✅ Build and CI/CD configuration
- ✅ Code quality tooling (analyzers, formatting)

### Week 2: Schema Generation & Models
- ✅ Generate Roll20 models from JSON schemas
- ✅ Generate Foundry V13 models from JSON schemas
- ✅ Generate Pathbuilder models from JSON schemas
- ✅ Validation attributes and custom converters
- ✅ Unit tests for all generated models

## Project Structure

```
src/
├── TTRPGConverter.Core/           # Models, interfaces, shared types
├── TTRPGConverter.Processing/     # Conversion logic, services
├── TTRPGConverter.CLI/           # Command-line interface
├── TTRPGConverter.GUI/           # Desktop application
tests/
├── TTRPGConverter.Core.Tests/    # Unit tests
├── TTRPGConverter.Processing.Tests/  # Service tests
```

## Dependencies

### Core Libraries
- **System.Text.Json** - High-performance JSON serialization
- **NJsonSchema** - JSON schema to C# code generation
- **Microsoft.Extensions.DependencyInjection** - Built-in DI container
- **Microsoft.Extensions.Logging** - Structured logging
- **SixLabors.ImageSharp** - Professional image processing

### Development Tools
- **xUnit** - Testing framework
- **FluentAssertions** - Readable test assertions
- **Microsoft.NET.Test.Sdk** - Test runner
- **coverlet.collector** - Code coverage

## Schema Generation Strategy

### Input Sources
```csharp
// Roll20 schemas from archived Rust implementation
crates/platforms/ttrpg-roll20/schemas/

// Foundry schemas from archived Rust implementation  
crates/platforms/ttrpg-foundry-common/schemas/

// Pathbuilder schemas from archived Rust implementation
crates/platforms/ttrpg-pathbuilder-pf2e/schemas/
```

### Output Models
```csharp
// TTRPGConverter.Core/Models/Roll20/
Roll20Campaign, Roll20Character, Roll20Page, Roll20Handout

// TTRPGConverter.Core/Models/Foundry/
FoundryWorld, FoundryActor, FoundryItem, FoundryScene, FoundryJournalPage

// TTRPGConverter.Core/Models/Pathbuilder/
PathbuilderCharacter, PathbuilderFeat, PathbuilderSpell
```

## Success Criteria

- [ ] All projects compile without warnings
- [ ] 100% schema coverage for Roll20, Foundry, Pathbuilder
- [ ] Unit tests achieve >95% code coverage
- [ ] CI/CD pipeline validates builds and tests
- [ ] Code quality gates pass (analyzers, formatting)

## Risks & Mitigation

**Risk**: Schema complexity causes generation issues  
**Mitigation**: Manual model creation for complex schemas, incremental approach

**Risk**: Dependency conflicts between libraries  
**Mitigation**: Centralized package management, version pinning

## Next Milestone

→ M2: Roll20 5e → Foundry PF2e conversion pipeline (3 weeks)
