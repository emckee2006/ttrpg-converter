# C# Migration Rationale

## Decision Summary

**Date**: August 2025  
**Status**: ✅ Approved - Migration Complete

## Why C# Over Rust

### Development Velocity
- **12 weeks** estimated timeline vs 26 weeks for Rust
- Faster prototyping and iteration
- Superior tooling and IDE support

### Libraries & Ecosystem
- **ImageSharp** - Professional image processing library
- **System.Text.Json** - Built-in high-performance JSON
- **Built-in dependency injection** - No external frameworks needed
- **NJsonSchema** - Code generation from JSON schemas

### Architecture Simplification
- **Static pipeline** instead of complex DLL plugin system
- Direct service composition vs plugin discovery/loading
- Cleaner project structure (6 projects vs 14+ crates)

### Deployment Options
- **NativeAOT** - 8MB single executable, ~25ms startup
- **Self-contained** - 12MB with .NET runtime included
- **Framework-dependent** - 2MB requiring .NET installation

## Implementation Strategy

### Phase 1: Foundation (2 weeks)
- Project structure setup
- Schema-to-C# code generation
- Basic dependency injection

### Phase 2: Core Conversion (3 weeks)
- Roll20 5e → Foundry PF2e pipeline
- Documentation-driven conversion
- Asset processing integration

### Phase 3: Advanced Features (7 weeks)
- Additional conversion pipelines
- Enhancement and optimization
- GUI application

## Preserved Assets

- **Planning documentation** - Obsidian vault continues
- **Reference code** - Python examples for guidance
- **Rust implementation** - Archived in `rust-implementation` branch
- **Conversion guides** - Platform-agnostic documentation

## Success Criteria

- Working Roll20 → Foundry conversion by Week 5
- Complete feature parity by Week 12
- Professional deployment options
- Comprehensive testing coverage
