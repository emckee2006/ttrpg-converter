# RustValidator System Documentation

## Overview

The RustValidator is a comprehensive validation framework for TTRPG (Tabletop Role-Playing Game) data conversion. It provides robust validation capabilities for campaigns, characters, and other game entities with performance optimization through caching and statistics tracking.

## Architecture

### Core Components

- **RustValidator**: Main validation engine implementing the `Validator` trait
- **EntitySchema**: Configuration for entity-specific validation rules  
- **ValidationConfig**: Global validation settings and performance options
- **ValidationStats**: Performance metrics and validation statistics
- **ValidationCache**: Performance optimization through result caching

### Key Features

- ✅ **Multi-entity validation**: Campaigns, characters, NPCs, vehicles
- ✅ **Schema-based validation**: Flexible entity schemas with field type checking
- ✅ **Performance caching**: Configurable result caching for repeated validations
- ✅ **Statistics tracking**: Detailed metrics on validation performance
- ✅ **JSON validation**: Deep structure validation with nesting checks
- ✅ **File path validation**: Secure path validation for assets
- ✅ **Error reporting**: Comprehensive error and warning reporting with suggestions

## Usage

### Basic Usage

```rust
use ttrpg_core::validation::RustValidator;
use ttrpg_core::services::Validator;

// Create validator with default configuration
let validator = RustValidator::new();

// Validate a campaign
let campaign_data = serde_json::json!({
    "metadata": {
        "name": "Epic Adventure",
        "version": "1.0"
    },
    "characters": []
});

let result = validator.validate_campaign(&campaign_data)?;
if result.is_valid {
    println!("Campaign is valid!");
} else {
    for error in result.errors {
        println!("Error: {}", error.message);
    }
}
```

### Advanced Configuration

```rust
use ttrpg_core::validation::{RustValidator, ValidationConfig};

// Create custom configuration
let config = ValidationConfig {
    enable_cache: true,
    max_cache_size: 1000,
    enable_stats: true,
    timeout_seconds: 30,
};

let validator = RustValidator::with_config(config);
```

### Entity Schema Definition

```rust
use ttrpg_core::validation::EntitySchema;
use std::collections::{HashMap, HashSet};

// Define schema for custom entity type
let mut required_fields = HashSet::new();
required_fields.insert("name".to_string());
required_fields.insert("type".to_string());

let mut field_types = HashMap::new();
field_types.insert("name".to_string(), "string".to_string());
field_types.insert("level".to_string(), "number".to_string());

let schema = EntitySchema {
    required_fields,
    optional_fields: HashMap::new(),
    field_types,
    validators: vec![],
};

validator.add_entity_schema("custom_character", schema);
```

## Validation Types

### 1. Campaign Validation
- Validates campaign metadata and structure
- Checks for required fields: name, version, etc.
- Validates character references and data integrity

### 2. Character Validation  
- Validates character attributes and stats
- Checks class/race compatibility
- Validates equipment and inventory

### 3. Data Type Validation
- Schema-based field type checking
- Custom validation rules per entity type
- Flexible type mapping system

### 4. Entity Data Validation
- Cross-entity relationship validation
- Business logic validation
- Data consistency checks

### 5. File Path Validation
- Secure path validation for assets
- Directory traversal prevention
- File existence verification

### 6. JSON Structure Validation
- Deep nesting validation (max 10 levels)
- Empty object detection
- Null value handling

## Error Handling

The validation system provides comprehensive error reporting:

```rust
// ValidationResult structure
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
    pub stats: ValidationStats,
}

// ValidationIssue structure
pub struct ValidationIssue {
    pub severity: IssueSeverity,  // Error, Warning, Info
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub field: Option<String>,
    pub message: String,
    pub suggestion: Option<String>,
}
```

## Performance Features

### Caching System
- Configurable result caching for repeated validations
- Cache key generation based on validation context
- Automatic cache cleanup and size management

### Statistics Tracking
```rust
pub struct ValidationStats {
    pub entities_validated: usize,
    pub entities_with_errors: usize, 
    pub entities_with_warnings: usize,
    pub validation_time_ms: u64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}
```

## Best Practices

### 1. Configuration
- Enable caching for repeated validations
- Set appropriate cache size limits
- Enable statistics for performance monitoring

### 2. Schema Design
- Define comprehensive entity schemas
- Include both required and optional fields
- Specify clear field type expectations

### 3. Error Handling
- Always check `ValidationResult.is_valid`
- Process both errors and warnings appropriately
- Use suggestions to guide user corrections

### 4. Performance
- Monitor validation statistics
- Adjust cache settings based on usage patterns
- Consider async validation for large datasets

## Implementation Status

✅ **Core validation engine**: Complete and fully tested  
✅ **Entity schema system**: Implemented with full type checking  
✅ **Performance caching**: Functional with configurable limits  
✅ **Statistics tracking**: Comprehensive metrics collection  
✅ **Error reporting**: Detailed error/warning/suggestion system  
✅ **Multi-entity support**: Campaign, character, and custom entities  

## Future Enhancements

- Async validation support for large datasets
- Custom validation rule plugins
- Enhanced schema inheritance
- Real-time validation for interactive editors
- Integration with external validation services

## Troubleshooting

### Common Issues

1. **Schema Mismatch**: Ensure entity schemas match expected data structure
2. **Cache Performance**: Monitor cache hit rates and adjust size accordingly  
3. **Memory Usage**: Consider disabling cache for memory-constrained environments
4. **Validation Speed**: Enable caching and monitor statistics for bottlenecks

### Debug Mode

Enable debug logging to troubleshoot validation issues:
```rust
env_logger::init();
// Validation operations will now log debug information
```

## Contributing

When extending the validation system:
1. Add comprehensive tests for new validators
2. Update entity schemas as needed
3. Maintain performance benchmarks
4. Document new validation rules and options
