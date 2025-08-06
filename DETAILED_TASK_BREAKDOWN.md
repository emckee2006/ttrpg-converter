# TTRPGConverter - Junior Developer Task Breakdown

## 🎯 CLI-First Development Strategy

**Decision**: Build CLI first, then GUI as enhancement layer
- CLI provides immediate testing and debugging capability  
- Core conversion logic isolated from GUI complexity
- CLI enables automation and batch processing
- GUI becomes thin layer over proven CLI foundation

---

## 📋 MILESTONE 1: FOUNDATION (Week 1-2)

### **T1.1: Project Workspace Setup** 
**Duration**: 1 day | **Points**: 3 | **Priority**: 🚨 CRITICAL

**Junior Dev Steps**:
1. `mkdir ttrpg-converter-dev && cd ttrpg-converter-dev`
2. Create workspace `Cargo.toml` with 5 crates: `ttrpg-core`, `ttrpg-cli`, `ttrpg-formats`, `ttrpg-assets`, `ttrpg-gui`
3. Set up each crate with basic `Cargo.toml` and `src/lib.rs`
4. Configure `.gitignore` and `.gitattributes` for cross-platform
5. Test with `cargo check --workspace`

**Files to Create**:
- `Cargo.toml` (workspace)
- `crates/*/Cargo.toml` (5 crates)
- `.gitignore`, `.gitattributes`

**Acceptance**: All crates compile successfully

---

### **T1.2: Core Error System**
**Duration**: 2 days | **Points**: 5 | **Priority**: 🚨 CRITICAL

**Junior Dev Steps**:
1. Create `ttrpg-core/src/error.rs` with `thiserror` derive macros
2. Define `ConversionError` enum with all error types
3. Add `ConversionResult<T>` type alias
4. Implement `ErrorContext` trait for error chaining
5. Write comprehensive tests for all error types

**Exact Code Patterns**:
```rust
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("IO error: {message}")]
    Io { message: String, #[source] source: std::io::Error },
    
    #[error("Validation failed for {entity_type} '{entity_id}': {reason}")]
    Validation { entity_type: String, entity_id: String, reason: String },
}
```

**Acceptance**: All error types have tests, proper Display/Debug

---

### **T1.3: CLI Argument Parsing**
**Duration**: 1 day | **Points**: 4 | **Priority**: 🔥 HIGH

**Junior Dev Steps**:
1. Create `ttrpg-cli/src/cli.rs` with `clap` derive API
2. Define `Cli` struct with subcommands: `convert`, `validate`, `info`, `config`
3. Add `SourceFormat` and `TargetFormat` enums with `ValueEnum` derive
4. Implement proper help text and examples
5. Test CLI parsing with unit tests

**Exact Command Structure**:
```bash
ttrpg-cli convert -i campaign.zip -o ./output -s roll20 -t foundry-v12
ttrpg-cli validate -i campaign.zip -s roll20 --detailed
ttrpg-cli info -i campaign.zip -s roll20 --stats
```

**Acceptance**: CLI help works, all arguments parse correctly

---

## 📋 MILESTONE 2: CORE CONVERSION (Week 3-4)

### **T2.1: Roll20 JSON Parser** 
**Duration**: 3 days | **Points**: 8 | **Priority**: 🔥 HIGH

**Junior Dev Steps**:
1. Create `ttrpg-formats/src/roll20/mod.rs` module structure
2. Define Roll20 entity structs with `serde` derive macros
3. Implement `Roll20Parser` with async JSON parsing
4. Add comprehensive error handling for malformed data
5. Test with real Roll20 campaign files

**Exact Struct Patterns**:
```rust
#[derive(Debug, Deserialize)]
pub struct Roll20Campaign {
    pub schema_version: String,
    pub name: String,
    pub characters: Vec<Roll20Character>,
    pub handouts: Vec<Roll20Handout>,
    pub pages: Vec<Roll20Page>,
}
```

**Performance Target**: Parse 50MB campaign in <2 seconds

---

### **T2.2: Entity Conversion Pipeline**
**Duration**: 4 days | **Points**: 10 | **Priority**: 🔥 HIGH  

**Junior Dev Steps**:
1. Create `Entity` trait in `ttrpg-core` with `validate()`, `convert()` methods
2. Implement `ConversionPipeline` using `rayon` for parallel processing
3. Add progress tracking with `indicatif` progress bars
4. Create `BatchProcessor` for memory-efficient processing
5. Test pipeline with sample data, measure performance

**Exact Pipeline Pattern**:
```rust
pub trait Entity {
    fn validate(&self) -> ConversionResult<()>;
    fn convert(&self, context: &ConversionContext) -> ConversionResult<FoundryEntity>;
}
```

**Performance Target**: 5-10x faster than Python version

---

This level of detail continues for all tasks, with specific:
- ✅ **Exact commands to run**
- ✅ **File paths to create** 
- ✅ **Code patterns to implement**
- ✅ **Dependencies to add**
- ✅ **Tests to write**
- ✅ **Performance targets**

Would you like me to continue with the complete detailed breakdown for all milestones?
