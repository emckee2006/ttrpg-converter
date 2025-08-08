# 🧪 Comprehensive Testing Framework

**Updated: 2024-08-07**  
**Purpose**: Define professional Rust testing standards for all TTRPGConverter milestones

---

## 🎯 **Core Testing Principles**

Following professional Rust development standards, **every milestone must include comprehensive testing before being marked complete**.

### ✅ **Required Testing Components**

1. **Unit Tests** 
   - Test individual functions with edge cases
   - Minimum 80% code coverage
   - Test error conditions and boundary cases
   - Mock external dependencies

2. **Integration Tests**
   - Test service interactions and real-world scenarios
   - Test complete workflows end-to-end
   - Verify service trait implementations
   - Test with real file system operations

3. **Property Tests** (using `proptest`)
   - Property-based testing where applicable
   - Generate random inputs to test invariants
   - Especially useful for data transformation and validation

4. **Benchmarks** (using `criterion`)
   - Performance measurement for critical paths
   - Regression detection for performance
   - HTML reports for detailed analysis

5. **Documentation Tests**
   - Ensure all code examples in documentation work
   - Test public API usage examples
   - Verify documentation stays current

---

## 📦 **Required Dependencies**

Add to `Cargo.toml`:

```toml
[dev-dependencies]
# Property-based testing
proptest = "1.4"

# Benchmarking with HTML reports
criterion = { version = "0.5", features = ["html_reports"] }

# Async testing utilities
tokio-test = "0.4"

# Environment variable testing
temp-env = "0.3"

# Temporary file testing
tempfile = "3.8"

# Additional testing utilities
assert_fs = "1.0"
predicates = "3.0"
```

---

## 🏗️ **Testing Structure**

```
crates/ttrpg-core/
├── src/
│   ├── lib.rs
│   ├── logging.rs
│   └── ...
├── tests/           # Integration tests
│   ├── logging_integration.rs
│   └── ...
├── benches/         # Benchmarks
│   ├── logging_bench.rs
│   └── ...
└── Cargo.toml
```

### Unit Tests
- Located within each module using `#[cfg(test)]`
- Test individual functions and methods
- Mock external dependencies

### Integration Tests
- Located in `tests/` directory
- Test complete workflows
- Use the public API only

### Benchmarks
- Located in `benches/` directory
- Measure performance of critical operations
- Generate HTML reports for analysis

---

## 🎯 **Milestone Completion Criteria**

**A milestone is only complete when ALL criteria are met:**

1. ✅ **Implementation** - Core functionality complete and compiles
2. ✅ **Unit Tests** - Individual function testing (>80% coverage)
3. ✅ **Integration Tests** - Service interaction testing
4. ✅ **Property Tests** - Using `proptest` where applicable
5. ✅ **Benchmarks** - Performance measurement for critical paths
6. ✅ **Documentation** - Comprehensive docs with working examples
7. ✅ **Code Quality** - Zero clippy warnings, proper formatting

---

## 📋 **Testing Templates**

### Unit Test Template

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_function_success_case() {
        // Arrange
        let input = "test input";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_output);
    }
    
    #[test]
    fn test_function_error_case() {
        // Test error conditions
        let invalid_input = "";
        let result = function_under_test(invalid_input);
        assert!(result.is_err());
    }
}
```

### Integration Test Template

```rust
use ttrpg_core::*;
use tempfile::tempdir;

#[tokio::test]
async fn test_service_integration() {
    // Test complete service workflow
    let temp_dir = tempdir().unwrap();
    let service = Service::new(temp_dir.path()).unwrap();
    
    // Test the complete workflow
    let result = service.process_data(test_data).await;
    assert!(result.is_ok());
}
```

### Property Test Template

```rust
#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_property_holds(input in any::<String>()) {
            // Test that some property always holds
            let result = function_under_test(&input);
            prop_assert!(property_check(&result));
        }
    }
}
```

### Benchmark Template

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use ttrpg_core::*;

fn bench_critical_operation(c: &mut Criterion) {
    let test_data = setup_test_data();
    
    c.bench_function("critical_operation", |b| {
        b.iter(|| {
            critical_operation(&test_data)
        });
    });
}

criterion_group!(benches, bench_critical_operation);
criterion_main!(benches);
```

---

## 🚀 **Testing Commands**

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin --out html

# Run benchmarks
cargo bench

# Run clippy (strict mode)
cargo clippy --all-features -- -D warnings

# Check formatting
cargo fmt --check

# Run property tests with more iterations
cargo test -- --ignored
```

---

## 📊 **Quality Metrics**

- **Code Coverage**: Minimum 80% for unit tests
- **Clippy Warnings**: Zero warnings in strict mode (`-D warnings`)
- **Documentation Coverage**: All public APIs documented with examples
- **Benchmark Regression**: Performance within 5% of baseline
- **Property Test Iterations**: Minimum 1000 iterations for property tests

---

This framework ensures professional Rust code quality and reliability across all TTRPGConverter components.
