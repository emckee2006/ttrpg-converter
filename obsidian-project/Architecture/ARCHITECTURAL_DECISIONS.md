# TTRPGConverter - Architectural Decisions

## 🏗️ **CORE ARCHITECTURAL DECISIONS**

### **Decision 1: Clean Slate vs Migration Approach**
**DECISION**: Fresh Pure Rust project using Python as functional reference
**RATIONALE**: 
- Python codebase has fundamental architectural debt (mixed sync/async, global state, etc.)
- PyO3 bindings create unnecessary complexity and performance overhead
- Clean slate enables proper Rust idioms from day 1 (enums, Result chains, async/await)
- Existing Python serves as comprehensive functional specification

**IMPLEMENTATION**: 
- New repository: `ttrpg-converter` (not fork)
- Python codebase preserved as `reference/` for feature parity validation
- No PyO3 - pure Rust throughout

---

### **Decision 2: CLI-First Development Strategy**
**DECISION**: Build CLI foundation first, then add GUI as enhancement layer
**RATIONALE**:
- CLI provides immediate testing and debugging capability
- Core conversion logic isolated from GUI complexity  
- Enables automation, batch processing, and CI/CD integration
- GUI becomes thin layer over proven CLI foundation
- Matches Unix philosophy of composable tools

**IMPLEMENTATION PHASES**:
1. **M1-M2**: Core foundation and conversion engine
2. **M3**: Full-featured CLI with progress bars, colored output, interactive prompts
3. **M4**: Native GUI using egui as enhancement layer

**CLI COMMAND STRUCTURE**:
```bash
ttrpg-cli convert -i campaign.zip -o ./output -s roll20 -t foundry-v12 --force
ttrpg-cli validate -i campaign.zip -s roll20 --detailed
ttrpg-cli info -i campaign.zip -s roll20 --stats
ttrpg-cli config -o ttrpg-config.toml --force
```

---

### **Decision 3: Workspace Architecture**
**DECISION**: Multi-crate workspace with clear separation of concerns
**RATIONALE**:
- Proper dependency boundaries prevent architectural drift
- Enables parallel development and testing
- Clear separation between parsing, conversion, and presentation
- Supports future expansion (new formats, output types)

**CRATE STRUCTURE**:
```
ttrpg-converter/
├── Cargo.toml                 # Workspace definition
├── crates/
│   ├── ttrpg-core/           # 🦀 Core types, traits, error handling
│   ├── ttrpg-formats/        # 📄 File format parsers (Roll20, Foundry)  
│   ├── ttrpg-assets/         # 🗂️ Asset processing and optimization
│   ├── ttrpg-cli/            # ⚡ Command-line interface
│   └── ttrpg-gui/            # 🎨 Native GUI (Phase 2)
└── reference/                # Python codebase for reference
```

**DEPENDENCY FLOW**:
```
ttrpg-cli ────┐
              ├─── ttrpg-core ←─── ttrpg-formats
ttrpg-gui ────┘                           ↑
                                   ttrpg-assets
```

---

### **Decision 4: Error Handling Strategy**
**DECISION**: Comprehensive error handling with `thiserror` and proper error chains
**RATIONALE**:
- Python version has poor error handling (generic exceptions, unclear context)
- `thiserror` provides clean error definitions with automatic Display/Debug
- Proper error chains enable better debugging and user feedback
- `anyhow` for error convenience, `thiserror` for library APIs

**ERROR ARCHITECTURE**:
```rust
#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("IO error: {message}")]
    Io { message: String, #[source] source: std::io::Error },
    
    #[error("JSON parsing failed for {context}: {message}")]
    Json { context: String, message: String, #[source] source: serde_json::Error },
    
    #[error("Validation failed for {entity_type} '{entity_id}': {reason}")]
    Validation { entity_type: String, entity_id: String, reason: String },
}

pub type ConversionResult<T> = Result<T, ConversionError>;
```

---

### **Decision 5: External Dependencies Strategy**
**DECISION**: Minimal, battle-tested dependencies with clear upgrade paths
**RATIONALE**:
- Reduce supply chain security risks
- Minimize compilation time and binary size
- Avoid dependency conflicts and version churn
- Focus on proven crates with strong maintenance

**CORE DEPENDENCIES** (required):
```toml
# Serialization - industry standard
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime - de facto standard
tokio = { version = "1.0", features = ["full"] }

# Error handling - best practices  
anyhow = "1.0"      # Convenience for applications
thiserror = "1.0"   # Custom errors for libraries

# Logging - structured logging standard
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

**CLI DEPENDENCIES** (CLI-specific):
```toml
# CLI framework - modern derive API
clap = { version = "4.0", features = ["derive", "env"] }
indicatif = "0.17"    # Progress bars
colored = "2.0"       # Terminal colors  
dialoguer = "0.11"    # Interactive prompts
```

**PERFORMANCE DEPENDENCIES** (optimization):
```toml
# Data parallelism
rayon = "1.8"         # Parallel iterators
dashmap = "5.5"       # Concurrent HashMap

# Asset processing
image = "0.25"        # Image manipulation
zip = "2.1"          # ZIP file handling
reqwest = { version = "0.11", features = ["json", "stream"] }
```

**GUI DEPENDENCIES** (Phase 2):
```toml
# Native GUI - modern immediate mode
egui = "0.28"         # Core GUI framework
eframe = "0.28"       # Application framework
```

**REJECTED DEPENDENCIES**:
- `async-std` → Use `tokio` (more mature ecosystem)
- `structopt` → Use `clap` v4 (better derive API)
- `failure` → Use `thiserror` (better error ergonomics)

---

### **Decision 6: Performance Architecture**  
**DECISION**: Parallel processing with memory optimization
**RATIONALE**:
- Python version is CPU-bound on large campaigns (50MB+ files)
- Rust enables zero-cost abstractions and memory safety
- `rayon` provides data parallelism without thread management complexity
- Streaming processing for large files reduces memory usage

**PERFORMANCE PATTERNS**:
```rust
// Parallel entity processing
entities
    .par_iter()                    // Parallel iteration
    .map(|entity| entity.convert()) // Parallel conversion
    .collect::<ConversionResult<Vec<_>>>()?; // Collect results

// Streaming file processing  
let reader = BufReader::new(file);
let mut deserializer = serde_json::Deserializer::from_reader(reader);
while let Ok(entity) = Entity::deserialize(&mut deserializer) {
    process_entity(entity).await?;
}
```

**PERFORMANCE TARGETS**:
- **5-10x faster** than Python version for large campaigns
- **<2 seconds** to parse 50MB Roll20 campaign file
- **Streaming processing** for files >100MB
- **<50MB binary size** for release builds

---

### **Decision 7: Testing Strategy**
**DECISION**: Comprehensive testing with property-based testing and benchmarks
**RATIONALE**:
- Python version lacks comprehensive test coverage
- Property testing catches edge cases that unit tests miss
- Benchmarking ensures performance regression detection
- Integration tests validate end-to-end workflows

**TESTING ARCHITECTURE**:
```rust
// Unit tests - basic functionality
#[cfg(test)]
mod tests {
    #[test]
    fn test_entity_conversion() {
        let roll20_actor = create_test_actor();
        let foundry_actor = roll20_actor.convert(&context)?;
        assert_eq!(foundry_actor.name, roll20_actor.name);
    }
}

// Property tests - edge cases
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_entity_roundtrip(entity in any::<Roll20Actor>()) {
            let converted = entity.convert(&context)?;
            prop_assert!(!converted.name.is_empty());
        }
    }
}

// Benchmarks - performance tracking
#[cfg(test)]
mod benches {
    use criterion::{criterion_group, criterion_main, Criterion};
    
    fn bench_entity_conversion(c: &mut Criterion) {
        c.bench_function("convert_large_campaign", |b| {
            b.iter(|| convert_campaign(&large_campaign))
        });
    }
}
```

---

### **Decision 8: Configuration Management**
**DECISION**: TOML-based configuration with sensible defaults
**RATIONALE**:
- TOML is human-readable and Rust-native
- Sensible defaults minimize configuration burden
- Environment variable support for CI/CD
- Validation at startup prevents runtime errors

**CONFIGURATION STRUCTURE**:
```toml
[conversion]
download_assets = true
max_asset_size = "50MB"
optimize_images = true
thread_count = "auto"  # or specific number

[output]
foundry_version = "v12"  # v10, v11, v12
compress_assets = true
preserve_folder_structure = true

[logging]
level = "info"           # error, warn, info, debug, trace
format = "compact"       # compact, json, pretty
```

---

This comprehensive architectural foundation ensures:
✅ **Junior developers** can implement with clear patterns  
✅ **Performance targets** are achievable with chosen architecture  
✅ **Maintainability** through proper separation of concerns  
✅ **Extensibility** for future format support  
✅ **Rust best practices** integrated throughout
