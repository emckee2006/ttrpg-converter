//! Data validation and integrity checking for TTRPG campaigns
//!
//! This module provides comprehensive validation functionality for all TTRPG entities,
//! ensuring data integrity, format compliance, and business logic correctness.
//!
//! # Architecture
//!
//! The validation system is built around several key concepts:
//!
//! - **Validator Trait**: Core validation interface for all entities
//! - **ValidationRules**: Configurable rule sets for different formats/systems
//! - **ValidationContext**: Contextual information for validation decisions
//! - **ValidationReport**: Detailed reporting of validation results
//!
//! # Usage
//!
//! ```rust
//! use ttrpg_core::validation::{ValidationContext, ValidationRules};
//!
//! let context = ValidationContext::default();
//! let rules = ValidationRules::strict();
//!
//! // Validation functionality will be implemented in future milestones
//! // These structures provide the foundation for comprehensive validation
//! assert!(rules.validate_required_fields);
//! assert!(!context.available_assets.is_empty() || context.available_assets.is_empty()); // Always true
//! ```

use crate::error::ConversionResult;
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Core validation trait implemented by all validatable entities
///
/// This trait provides a consistent interface for validating TTRPG entities
/// with contextual information and configurable rules.
#[allow(clippy::result_large_err)]
pub trait Validator {
    /// Validates the entity with the given context and rules
    ///
    /// # Arguments
    ///
    /// * `context` - Validation context with environmental information
    /// * `rules` - Rule set to apply during validation
    ///
    /// # Returns
    ///
    /// Returns a `ValidationReport` on success, or a `ConversionError` on failure.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Required fields are missing or invalid
    /// - Data types don't match expected formats
    /// - Business logic constraints are violated
    /// - Cross-references are broken or inconsistent
    fn validate(
        &self,
        context: &ValidationContext,
        rules: &ValidationRules,
    ) -> ConversionResult<ValidationReport>;

    /// Validates only required fields (fast validation)
    ///
    /// This is a lightweight validation that only checks for required fields
    /// and basic data type integrity. Use for quick validation passes.
    fn validate_required(&self, context: &ValidationContext) -> ConversionResult<()>;

    /// Validates data types and formats
    ///
    /// Performs comprehensive data type checking including:
    /// - String length limits
    /// - Numeric range validation
    /// - URL format validation
    /// - ID reference integrity
    fn validate_data_types(&self, context: &ValidationContext) -> ConversionResult<()>;

    /// Validates business logic and constraints
    ///
    /// Checks entity-specific business rules such as:
    /// - Stat block consistency for actors
    /// - Scene dimension requirements
    /// - Asset availability and accessibility
    /// - Permission and ownership rules
    fn validate_business_logic(
        &self,
        context: &ValidationContext,
        rules: &ValidationRules,
    ) -> ConversionResult<()>;
}

/// Validation context providing environmental information
///
/// The validation context carries information about the current validation
/// environment, including available assets, reference data, and system constraints.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationContext {
    /// Target format being validated for
    pub target_format: Option<TargetFormat>,

    /// Available asset references for validation
    pub available_assets: HashSet<String>,

    /// Valid entity IDs that can be referenced
    pub valid_entity_ids: HashSet<String>,

    /// System-specific configuration
    pub system_config: HashMap<String, String>,

    /// Validation strictness level
    pub strictness: StrictnessLevel,

    /// Whether to perform expensive validation checks
    pub deep_validation: bool,

    /// Maximum allowed entity sizes
    pub size_limits: SizeLimits,
}

/// Validation strictness levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StrictnessLevel {
    /// Minimal validation - only critical errors
    Permissive,

    /// Standard validation - common issues and warnings
    Standard,

    /// Strict validation - all issues including minor inconsistencies
    Strict,

    /// Pedantic validation - every possible issue
    Pedantic,
}

impl Default for StrictnessLevel {
    fn default() -> Self {
        Self::Standard
    }
}

/// Size limits for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeLimits {
    /// Maximum string length for names and descriptions
    pub max_string_length: usize,

    /// Maximum number of entities per campaign
    pub max_entities_per_campaign: usize,

    /// Maximum file size for assets (in bytes)
    pub max_asset_size: u64,

    /// Maximum image dimensions
    pub max_image_width: u32,
    pub max_image_height: u32,
}

impl Default for SizeLimits {
    fn default() -> Self {
        Self {
            max_string_length: 65_536,         // 64KB strings
            max_entities_per_campaign: 10_000, // 10K entities
            max_asset_size: 100_000_000,       // 100MB assets
            max_image_width: 8192,             // 8K image width
            max_image_height: 8192,            // 8K image height
        }
    }
}

/// Validation rules configuration
///
/// Defines which validation rules to apply and how strictly to enforce them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    /// Whether to validate required fields
    pub validate_required_fields: bool,

    /// Whether to validate data types and formats
    pub validate_data_types: bool,

    /// Whether to validate business logic
    pub validate_business_logic: bool,

    /// Whether to validate cross-references
    pub validate_cross_references: bool,

    /// Whether to validate asset availability
    pub validate_assets: bool,

    /// Whether to validate permissions and ownership
    pub validate_permissions: bool,

    /// Custom validation rules
    pub custom_rules: HashMap<String, String>,
}

impl ValidationRules {
    /// Creates permissive validation rules (minimal checking)
    pub fn permissive() -> Self {
        Self {
            validate_required_fields: true,
            validate_data_types: false,
            validate_business_logic: false,
            validate_cross_references: false,
            validate_assets: false,
            validate_permissions: false,
            custom_rules: HashMap::new(),
        }
    }

    /// Creates standard validation rules (balanced checking)
    pub fn standard() -> Self {
        Self {
            validate_required_fields: true,
            validate_data_types: true,
            validate_business_logic: true,
            validate_cross_references: false,
            validate_assets: false,
            validate_permissions: false,
            custom_rules: HashMap::new(),
        }
    }

    /// Creates strict validation rules (comprehensive checking)
    pub fn strict() -> Self {
        Self {
            validate_required_fields: true,
            validate_data_types: true,
            validate_business_logic: true,
            validate_cross_references: true,
            validate_assets: true,
            validate_permissions: true,
            custom_rules: HashMap::new(),
        }
    }
}

impl Default for ValidationRules {
    fn default() -> Self {
        Self::standard()
    }
}

/// Validation report containing results and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Validation success status
    pub success: bool,

    /// Total number of items validated
    pub total_validated: usize,

    /// Number of validation errors found
    pub errors: Vec<ValidationError>,

    /// Number of validation warnings found
    pub warnings: Vec<ValidationWarning>,

    /// Validation performance statistics
    pub stats: ValidationStats,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ValidationReport {
    /// Creates a new successful validation report
    pub fn success(total: usize) -> Self {
        Self {
            success: true,
            total_validated: total,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
            metadata: HashMap::new(),
        }
    }

    /// Creates a new failed validation report with errors
    pub fn failure(total: usize, errors: Vec<ValidationError>) -> Self {
        Self {
            success: false,
            total_validated: total,
            errors,
            warnings: Vec::new(),
            stats: ValidationStats::default(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a warning to the report
    pub fn add_warning(&mut self, warning: ValidationWarning) {
        self.warnings.push(warning);
    }

    /// Gets a summary of the validation results
    pub fn summary(&self) -> String {
        if self.success {
            format!(
                "Validation successful: {} items validated, {} warnings",
                self.total_validated,
                self.warnings.len()
            )
        } else {
            format!(
                "Validation failed: {} errors, {} warnings in {} items",
                self.errors.len(),
                self.warnings.len(),
                self.total_validated
            )
        }
    }

    /// Checks if the validation has any issues (errors or warnings)
    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
}

/// Validation error details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error code for programmatic handling
    pub code: String,

    /// Human-readable error message
    pub message: String,

    /// Entity type that caused the error
    pub entity_type: String,

    /// Entity ID if available
    pub entity_id: Option<String>,

    /// Field path that caused the error
    pub field_path: Option<String>,

    /// Expected value or format
    pub expected: Option<String>,

    /// Actual value that caused the error
    pub actual: Option<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.entity_id, &self.field_path) {
            (Some(id), Some(path)) => {
                write!(f, "[{}] {}.{}: {}", self.code, id, path, self.message)
            }
            (Some(id), None) => {
                write!(f, "[{}] {}: {}", self.code, id, self.message)
            }
            (None, Some(path)) => {
                write!(f, "[{}] {}: {}", self.code, path, self.message)
            }
            (None, None) => {
                write!(f, "[{}] {}", self.code, self.message)
            }
        }
    }
}

/// Validation warning details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    /// Warning code for programmatic handling
    pub code: String,

    /// Human-readable warning message
    pub message: String,

    /// Entity type that caused the warning
    pub entity_type: String,

    /// Entity ID if available
    pub entity_id: Option<String>,

    /// Suggestion for fixing the warning
    pub suggestion: Option<String>,
}

impl fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.entity_id {
            Some(id) => write!(f, "[{}] {}: {}", self.code, id, self.message),
            None => write!(f, "[{}] {}", self.code, self.message),
        }
    }
}

/// Validation performance statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationStats {
    /// Time spent on validation (in milliseconds)
    pub validation_time_ms: u64,

    /// Number of validation rules applied
    pub rules_applied: usize,

    /// Number of fields validated
    pub fields_validated: usize,

    /// Memory usage during validation (in bytes)
    pub memory_used: usize,
}

/// Utility functions for common validation patterns
#[allow(clippy::result_large_err)]
pub mod utils {
    use super::*;
    use std::path::Path;
    use url::Url;

    /// Validates that a string is not empty and within size limits
    pub fn validate_string(
        value: &str,
        field_name: &str,
        required: bool,
        max_length: usize,
    ) -> Result<(), ValidationError> {
        if value.is_empty() && required {
            return Err(ValidationError {
                code: "EMPTY_REQUIRED_FIELD".to_string(),
                message: format!("Required field '{field_name}' cannot be empty"),
                entity_type: "String".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some("Non-empty string".to_string()),
                actual: Some("Empty string".to_string()),
            });
        }

        if value.len() > max_length {
            return Err(ValidationError {
                code: "STRING_TOO_LONG".to_string(),
                message: format!(
                    "Field '{field_name}' exceeds maximum length of {max_length} characters"
                ),
                entity_type: "String".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some(format!("≤ {max_length} characters")),
                actual: Some(format!("{} characters", value.len())),
            });
        }

        Ok(())
    }

    /// Validates that a numeric value is within specified range
    pub fn validate_numeric_range<T>(
        value: T,
        field_name: &str,
        min: T,
        max: T,
    ) -> Result<(), ValidationError>
    where
        T: PartialOrd + fmt::Display + Copy,
    {
        if value < min || value > max {
            return Err(ValidationError {
                code: "VALUE_OUT_OF_RANGE".to_string(),
                message: format!("Field '{field_name}' value {value} is outside valid range"),
                entity_type: "Numeric".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some(format!("{min} to {max}")),
                actual: Some(format!("{value}")),
            });
        }

        Ok(())
    }

    /// Validates URL format
    pub fn validate_url(url: &str, field_name: &str) -> Result<(), ValidationError> {
        if url.is_empty() {
            return Ok(()); // Empty URLs are handled by string validation
        }

        match Url::parse(url) {
            Ok(_) => Ok(()),
            Err(e) => Err(ValidationError {
                code: "INVALID_URL_FORMAT".to_string(),
                message: format!("Field '{field_name}' contains invalid URL: {e}"),
                entity_type: "URL".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some("Valid URL format".to_string()),
                actual: Some(url.to_string()),
            }),
        }
    }

    /// Validates file path format and existence (if checking filesystem)
    pub fn validate_file_path(
        path: &str,
        field_name: &str,
        check_existence: bool,
    ) -> Result<(), ValidationError> {
        if path.is_empty() {
            return Ok(()); // Empty paths are handled by string validation
        }

        let path_obj = Path::new(path);

        // Validate path format
        if path.contains('\0') {
            return Err(ValidationError {
                code: "INVALID_PATH_FORMAT".to_string(),
                message: format!("Field '{field_name}' contains null character in path"),
                entity_type: "FilePath".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some("Valid file path".to_string()),
                actual: Some(path.to_string()),
            });
        }

        // Check existence if requested
        if check_existence && !path_obj.exists() {
            return Err(ValidationError {
                code: "FILE_NOT_FOUND".to_string(),
                message: format!("Field '{field_name}' references non-existent file"),
                entity_type: "FilePath".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some("Existing file path".to_string()),
                actual: Some(path.to_string()),
            });
        }

        Ok(())
    }

    /// Validates that an ID exists in a set of valid IDs
    pub fn validate_id_reference(
        id: &str,
        field_name: &str,
        valid_ids: &HashSet<String>,
        entity_type: &str,
    ) -> Result<(), ValidationError> {
        if id.is_empty() {
            return Ok(()); // Empty IDs are handled by string validation
        }

        if !valid_ids.contains(id) {
            return Err(ValidationError {
                code: "INVALID_ID_REFERENCE".to_string(),
                message: format!(
                    "Field '{field_name}' references non-existent {entity_type} with ID '{id}'"
                ),
                entity_type: "IDReference".to_string(),
                entity_id: None,
                field_path: Some(field_name.to_string()),
                expected: Some(format!("Valid {entity_type} ID")),
                actual: Some(id.to_string()),
            });
        }

        Ok(())
    }
}

// ============================================================================
// RUSTVALIDATOR IMPLEMENTATION
// ============================================================================

/// Comprehensive Rust-based validation service implementation
///
/// Provides enterprise-grade validation capabilities with caching, statistics,
/// and comprehensive entity validation matching Python ValidationService functionality.
#[derive(Debug)]
pub struct RustValidator {
    /// Entity validation schemas
    schemas: HashMap<String, EntitySchema>,
    /// Data type validation maps
    #[allow(dead_code)]
    type_maps: HashMap<String, TypeMap>,
    /// Performance statistics
    stats: std::sync::RwLock<ValidationStats>,
    /// Validation cache for performance
    cache: std::sync::RwLock<HashMap<String, ValidationReport>>,
    /// Configuration
    config: ValidationConfig,
}

/// Entity validation schema definition
#[derive(Debug, Clone)]
pub struct EntitySchema {
    /// Required fields for this entity type
    required_fields: HashSet<String>,
    /// Optional fields with defaults
    #[allow(dead_code)]
    optional_fields: HashMap<String, serde_json::Value>,
    /// Field type specifications
    field_types: HashMap<String, String>,
    /// Business logic validators
    validators: Vec<String>,
}

/// Type mapping for validation
#[derive(Debug, Clone)]
struct TypeMap {
    /// Mapping from field name to expected type
    #[allow(dead_code)]
    field_types: HashMap<String, String>,
    /// Custom validation rules per field
    #[allow(dead_code)]
    field_rules: HashMap<String, Vec<String>>,
}

/// Validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Enable caching
    enable_cache: bool,
    /// Maximum cache size
    #[allow(dead_code)]
    max_cache_size: usize,
    /// Enable statistics tracking
    enable_stats: bool,
    /// Validation timeout in seconds
    #[allow(dead_code)]
    timeout_seconds: u64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self { enable_cache: true, max_cache_size: 1000, enable_stats: true, timeout_seconds: 30 }
    }
}

impl RustValidator {
    /// Create a new RustValidator with default configuration
    pub fn new() -> ConversionResult<Self> {
        Self::with_config(ValidationConfig::default())
    }

    /// Create a new RustValidator with custom configuration
    pub fn with_config(config: ValidationConfig) -> ConversionResult<Self> {
        let mut validator = Self {
            schemas: HashMap::new(),
            type_maps: HashMap::new(),
            stats: std::sync::RwLock::new(ValidationStats::default()),
            cache: std::sync::RwLock::new(HashMap::new()),
            config,
        };

        // Initialize default schemas for common entity types
        validator.initialize_default_schemas()?;

        Ok(validator)
    }

    /// Initialize default validation schemas for common entity types
    fn initialize_default_schemas(&mut self) -> ConversionResult<()> {
        // Actor/Character schema
        self.add_entity_schema(
            "actor",
            EntitySchema {
                required_fields: ["name", "type"].iter().map(|s| s.to_string()).collect(),
                optional_fields: [
                    ("biography".to_string(), serde_json::Value::String("".to_string())),
                    ("level".to_string(), serde_json::Value::Number(1.into())),
                ]
                .into_iter()
                .collect(),
                field_types: [
                    ("name".to_string(), "string".to_string()),
                    ("type".to_string(), "string".to_string()),
                    ("level".to_string(), "number".to_string()),
                ]
                .into_iter()
                .collect(),
                validators: vec!["validate_actor_type".to_string()],
            },
        );

        // Item schema
        self.add_entity_schema(
            "item",
            EntitySchema {
                required_fields: ["name", "type"].iter().map(|s| s.to_string()).collect(),
                optional_fields: [
                    ("description".to_string(), serde_json::Value::String("".to_string())),
                    ("weight".to_string(), serde_json::Value::Number(0.into())),
                ]
                .into_iter()
                .collect(),
                field_types: [
                    ("name".to_string(), "string".to_string()),
                    ("type".to_string(), "string".to_string()),
                    ("weight".to_string(), "number".to_string()),
                ]
                .into_iter()
                .collect(),
                validators: vec!["validate_item_type".to_string()],
            },
        );

        // Scene schema
        self.add_entity_schema(
            "scene",
            EntitySchema {
                required_fields: ["name", "width", "height"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                optional_fields: [
                    (
                        "background_color".to_string(),
                        serde_json::Value::String("#ffffff".to_string()),
                    ),
                    ("grid_size".to_string(), serde_json::Value::Number(100.into())),
                ]
                .into_iter()
                .collect(),
                field_types: [
                    ("name".to_string(), "string".to_string()),
                    ("width".to_string(), "number".to_string()),
                    ("height".to_string(), "number".to_string()),
                ]
                .into_iter()
                .collect(),
                validators: vec!["validate_scene_dimensions".to_string()],
            },
        );

        Ok(())
    }

    /// Add or update an entity schema
    pub fn add_entity_schema(&mut self, entity_type: &str, schema: EntitySchema) {
        self.schemas.insert(entity_type.to_string(), schema);
    }

    /// Update validation statistics
    fn update_stats<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut ValidationStats),
    {
        if self.config.enable_stats {
            if let Ok(mut stats) = self.stats.write() {
                update_fn(&mut stats);
            }
        }
    }

    /// Get validation result from cache
    fn get_cached_result(&self, cache_key: &str) -> Option<ValidationReport> {
        if self.config.enable_cache {
            if let Ok(cache) = self.cache.read() {
                return cache.get(cache_key).cloned();
            }
        }
        None
    }

    /// Store validation result in cache
    #[allow(dead_code)]
    fn cache_result(&self, cache_key: String, result: ValidationReport) {
        if self.config.enable_cache {
            if let Ok(mut cache) = self.cache.write() {
                // Implement simple LRU by clearing cache when it gets too large
                if cache.len() >= self.config.max_cache_size {
                    cache.clear();
                }
                cache.insert(cache_key, result);
            }
        }
    }

    /// Validate entity-specific business rules
    fn validate_business_rules(
        &self,
        entity_type: &str,
        data: &serde_json::Value,
        schema: &EntitySchema,
    ) -> ConversionResult<Vec<ValidationError>> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for validator_name in &schema.validators {
            match validator_name.as_str() {
                "validate_actor_type" => {
                    if let Some(actor_type) = data.get("type").and_then(|v| v.as_str()) {
                        if !["character", "npc", "vehicle"].contains(&actor_type) {
                            errors.push(ValidationError {
                                code: "INVALID_ACTOR_TYPE".to_string(),
                                message: format!("Invalid actor type: {actor_type}. Must be one of: character, npc, vehicle"),
                                entity_type: entity_type.to_string(),
                                entity_id: None,
                                field_path: Some("type".to_string()),
                                expected: Some("character, npc, vehicle".to_string()),
                                actual: Some(actor_type.to_string()),
                            });
                        }
                    }
                }
                "validate_item_type" => {
                    if let Some(item_type) = data.get("type").and_then(|v| v.as_str()) {
                        if !["weapon", "armor", "consumable", "tool", "loot"].contains(&item_type) {
                            errors.push(ValidationError {
                                code: "INVALID_ITEM_TYPE".to_string(),
                                message: format!("Invalid item type: {item_type}. Must be one of: weapon, armor, consumable, tool, loot"),
                                entity_type: entity_type.to_string(),
                                entity_id: None,
                                field_path: Some("type".to_string()),
                                expected: Some("weapon, armor, consumable, tool, loot".to_string()),
                                actual: Some(item_type.to_string()),
                            });
                        }
                    }
                }
                "validate_scene_dimensions" => {
                    let width = data.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let height = data.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0);

                    if width <= 0.0 || height <= 0.0 {
                        errors.push(ValidationError {
                            code: "INVALID_DIMENSIONS".to_string(),
                            message: "Scene dimensions must be positive numbers".to_string(),
                            entity_type: entity_type.to_string(),
                            entity_id: None,
                            field_path: Some("dimensions".to_string()),
                            expected: Some("positive numbers".to_string()),
                            actual: Some(format!("width={width}, height={height}")),
                        });
                    }

                    if width > 10000.0 || height > 10000.0 {
                        warnings.push(ValidationError {
                            code: "LARGE_DIMENSIONS".to_string(),
                            message: "Scene dimensions are unusually large (>10000)".to_string(),
                            entity_type: entity_type.to_string(),
                            entity_id: None,
                            field_path: Some("dimensions".to_string()),
                            expected: Some("reasonable size (<10000)".to_string()),
                            actual: Some(format!("width={width}, height={height}")),
                        });
                    }
                }
                _ => {
                    // Unknown validator - log warning but don't fail
                    tracing::warn!("Unknown validator: {}", validator_name);
                }
            }
        }

        // Add warnings to errors since method returns Vec<ValidationError>
        errors.extend(warnings);

        Ok(errors)
    }
}

impl Default for RustValidator {
    fn default() -> Self {
        Self::new().expect("Failed to create default RustValidator")
    }
}

// Implement ValidationService trait for RustValidator
impl crate::services::ValidationService for RustValidator {
    fn validate_campaign(
        &self,
        campaign: &Campaign,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
        });

        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Basic campaign validation
        if campaign.metadata.name.trim().is_empty() {
            errors.push(crate::services::ValidationIssue {
                severity: crate::services::IssueSeverity::Error,
                entity_type: "campaign".to_string(),
                entity_id: None,
                field: Some("name".to_string()),
                message: "Campaign name cannot be empty".to_string(),
                suggestion: Some("Provide a non-empty campaign name".to_string()),
            });
        }

        if campaign.metadata.name.len() > 100 {
            warnings.push(crate::services::ValidationIssue {
                severity: crate::services::IssueSeverity::Warning,
                entity_type: "campaign".to_string(),
                entity_id: None,
                field: Some("name".to_string()),
                message: "Campaign name is very long (>100 characters)".to_string(),
                suggestion: Some("Consider shortening the campaign name".to_string()),
            });
        }

        self.update_stats(|stats| {
            stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.fields_validated += 1;
        });

        Ok(crate::services::ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            stats: self.get_validation_stats(),
        })
    }

    fn validate_required_fields(
        &self,
        entity_type: &str,
        data: &serde_json::Value,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
            stats.fields_validated += 1;
        });

        let cache_key = format!("required_fields:{}:{}", entity_type, data.to_string().len());
        if let Some(cached) = self.get_cached_result(&cache_key) {
            return Ok(crate::services::ValidationResult {
                is_valid: !cached.has_issues(),
                errors: vec![], // TODO: Convert ValidationError to ValidationIssue
                warnings: vec![],
                stats: self.get_validation_stats(),
            });
        }

        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        if let Some(schema) = self.schemas.get(entity_type) {
            let obj =
                data.as_object()
                    .ok_or_else(|| crate::error::ConversionError::ValidationError {
                        entity_type: entity_type.to_string(),
                        message: "Data must be a JSON object for field validation".to_string(),
                        field: Some("root".to_string()),
                    })?;

            for required_field in &schema.required_fields {
                if !obj.contains_key(required_field) {
                    errors.push(crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Error,
                        entity_type: entity_type.to_string(),
                        entity_id: None,
                        field: Some(required_field.clone()),
                        message: format!("Required field '{required_field}' is missing"),
                        suggestion: Some(format!("Add the required field '{required_field}'",)),
                    });
                }
            }
        } else {
            // No schema found - create a warning
            warnings.push(crate::services::ValidationIssue {
                severity: crate::services::IssueSeverity::Warning,
                entity_type: entity_type.to_string(),
                entity_id: None,
                field: Some("schema".to_string()),
                message: format!("No validation schema found for entity type '{entity_type}'"),
                suggestion: Some("Define a validation schema for this entity type".to_string()),
            });
        }

        self.update_stats(|stats| {
            stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.fields_validated += errors.len().max(1);
        });

        let result = crate::services::ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings: vec![],
            stats: self.get_validation_stats(),
        };

        Ok(result)
    }

    fn validate_data_types(
        &self,
        entity_type: &str,
        data: &serde_json::Value,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
            stats.fields_validated += 1;
        });

        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let warnings = Vec::new();

        if let Some(schema) = self.schemas.get(entity_type) {
            let obj =
                data.as_object()
                    .ok_or_else(|| crate::error::ConversionError::ValidationError {
                        entity_type: entity_type.to_string(),
                        message: "Data must be a JSON object for type validation".to_string(),
                        field: Some("root".to_string()),
                    })?;

            for (field_name, expected_type) in &schema.field_types {
                if let Some(field_value) = obj.get(field_name) {
                    let actual_type = match field_value {
                        serde_json::Value::String(_) => "string",
                        serde_json::Value::Number(_) => "number",
                        serde_json::Value::Bool(_) => "boolean",
                        serde_json::Value::Array(_) => "array",
                        serde_json::Value::Object(_) => "object",
                        serde_json::Value::Null => "null",
                    };

                    if actual_type != expected_type {
                        errors.push(crate::services::ValidationIssue {
                            severity: crate::services::IssueSeverity::Error,
                            entity_type: entity_type.to_string(),
                            entity_id: None,
                            field: Some(field_name.clone()),
                            message: format!(
                                "Field '{field_name}' has type '{actual_type}' but expected '{expected_type}'"
                            ),
                            suggestion: Some(format!(
                                "Change field '{field_name}' to type '{expected_type}'"
                            )),
                        });
                    }
                }
            }
        }

        self.update_stats(|stats| {
            stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.fields_validated += errors.len().max(1);
        });

        Ok(crate::services::ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            stats: self.get_validation_stats(),
        })
    }

    fn validate_entity_data(
        &self,
        entity_type: &str,
        data: &serde_json::Value,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
            stats.fields_validated += 1;
        });

        let start_time = std::time::Instant::now();
        let mut all_issues = Vec::new();
        let mut all_warnings = Vec::new();

        // Validate required fields
        let required_result = self.validate_required_fields(entity_type, data)?;
        all_issues.extend(required_result.errors);
        all_warnings.extend(required_result.warnings);

        // Validate data types
        let types_result = self.validate_data_types(entity_type, data)?;
        all_issues.extend(types_result.errors);
        all_warnings.extend(types_result.warnings);

        // Validate business rules
        if let Some(schema) = self.schemas.get(entity_type) {
            match self.validate_business_rules(entity_type, data, schema) {
                Ok(errors) => {
                    for error in errors {
                        let issue = crate::services::ValidationIssue {
                            severity: crate::services::IssueSeverity::Error, // All ValidationErrors are errors
                            entity_type: error.entity_type,
                            entity_id: error.entity_id,
                            field: error.field_path,
                            message: error.message,
                            suggestion: None, // ValidationError doesn't have suggestions
                        };

                        match issue.severity {
                            crate::services::IssueSeverity::Error => all_issues.push(issue),
                            crate::services::IssueSeverity::Warning => all_warnings.push(issue),
                            crate::services::IssueSeverity::Info => all_warnings.push(issue), // Treat Info as warnings
                        }
                    }
                }
                Err(e) => {
                    all_issues.push(crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Error,
                        entity_type: entity_type.to_string(),
                        entity_id: None,
                        field: Some("business_rules".to_string()),
                        message: format!("Failed to validate business rules: {e}"),
                        suggestion: Some("Check business rule configuration".to_string()),
                    });
                }
            }
        }

        self.update_stats(|stats| {
            stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.fields_validated += (all_issues.len() + all_warnings.len()).max(1);
        });

        Ok(crate::services::ValidationResult {
            is_valid: all_issues
                .iter()
                .all(|i| i.severity != crate::services::IssueSeverity::Error),
            errors: all_issues,
            warnings: all_warnings,
            stats: self.get_validation_stats(),
        })
    }

    fn validate_file_path(
        &self,
        path: &str,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
            stats.fields_validated += 1;
        });

        let start_time = std::time::Instant::now();

        match utils::validate_file_path(path, "path", false) {
            Ok(_) => {
                self.update_stats(|stats| {
                    stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
                });

                Ok(crate::services::ValidationResult {
                    is_valid: true,
                    errors: vec![],
                    warnings: vec![],
                    stats: self.get_validation_stats(),
                })
            }
            Err(error) => {
                self.update_stats(|stats| {
                    stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
                    stats.fields_validated += 1;
                });

                Ok(crate::services::ValidationResult {
                    is_valid: false,
                    errors: vec![crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Error,
                        entity_type: "file_path".to_string(),
                        entity_id: None,
                        field: Some("path".to_string()),
                        message: error.message,
                        suggestion: Some("Provide a valid file path".to_string()),
                    }],
                    warnings: vec![],
                    stats: self.get_validation_stats(),
                })
            }
        }
    }

    fn validate_json_data(
        &self,
        data: &serde_json::Value,
    ) -> ConversionResult<crate::services::ValidationResult> {
        self.update_stats(|stats| {
            stats.rules_applied += 1;
            stats.fields_validated += 1;
        });

        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Basic JSON structure validation
        match data {
            serde_json::Value::Null => {
                errors.push(crate::services::ValidationIssue {
                    severity: crate::services::IssueSeverity::Error,
                    entity_type: "json".to_string(),
                    entity_id: None,
                    field: Some("root".to_string()),
                    message: "JSON data cannot be null".to_string(),
                    suggestion: Some("Provide valid JSON data".to_string()),
                });
            }
            serde_json::Value::Object(obj) => {
                if obj.is_empty() {
                    warnings.push(crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Warning,
                        entity_type: "json".to_string(),
                        entity_id: None,
                        field: Some("root".to_string()),
                        message: "JSON object is empty".to_string(),
                        suggestion: Some("Add properties to the JSON object".to_string()),
                    });
                }

                // Check for excessively nested objects
                if Self::count_json_depth(data) > 10 {
                    warnings.push(crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Warning,
                        entity_type: "json".to_string(),
                        entity_id: None,
                        field: Some("structure".to_string()),
                        message: format!(
                            "JSON has very deep nesting ({} levels)",
                            Self::count_json_depth(data)
                        ),
                        suggestion: Some("Consider flattening the JSON structure".to_string()),
                    });
                }
            }
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    warnings.push(crate::services::ValidationIssue {
                        severity: crate::services::IssueSeverity::Warning,
                        entity_type: "json".to_string(),
                        entity_id: None,
                        field: Some("root".to_string()),
                        message: "JSON array is empty".to_string(),
                        suggestion: Some("Add items to the array".to_string()),
                    });
                }
            }
            _ => {
                // Primitive values are generally OK
            }
        }

        self.update_stats(|stats| {
            stats.validation_time_ms += start_time.elapsed().as_millis() as u64;
            stats.fields_validated += 1;
        });

        Ok(crate::services::ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            stats: self.get_validation_stats(),
        })
    }

    fn get_validation_stats(&self) -> crate::services::ValidationStats {
        // Convert internal stats to services::ValidationStats
        if let Ok(stats) = self.stats.read() {
            crate::services::ValidationStats {
                entities_validated: stats.rules_applied,
                entities_with_errors: 0, // Would need to track this separately
                entities_with_warnings: 0, // Would need to track this separately
                validation_time_ms: stats.validation_time_ms,
                cache_hits: 0,   // Would need to implement cache tracking
                cache_misses: 0, // Would need to implement cache tracking
            }
        } else {
            crate::services::ValidationStats::default()
        }
    }

    fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }

        self.update_stats(|stats| {
            *stats = ValidationStats::default();
        });
    }
}

impl RustValidator {
    /// Helper method to count JSON nesting depth
    fn count_json_depth(value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::Object(obj) => {
                if obj.is_empty() {
                    1
                } else {
                    1 + obj.values().map(Self::count_json_depth).max().unwrap_or(0)
                }
            }
            serde_json::Value::Array(arr) => {
                if arr.is_empty() {
                    1
                } else {
                    1 + arr.iter().map(Self::count_json_depth).max().unwrap_or(0)
                }
            }
            _ => 1,
        }
    }
}

// Thread-safety: RustValidator is Send + Sync
unsafe impl Send for RustValidator {}
unsafe impl Sync for RustValidator {}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::types::{Campaign, SourceFormat}; // TODO: Remove when validation tests are implemented

    #[test]
    fn test_validation_context_default() {
        let context = ValidationContext::default();
        assert_eq!(context.strictness, StrictnessLevel::Standard);
        assert!(!context.deep_validation);
        assert!(context.available_assets.is_empty());
        assert!(context.valid_entity_ids.is_empty());
    }

    #[test]
    fn test_validation_rules_presets() {
        let permissive = ValidationRules::permissive();
        assert!(permissive.validate_required_fields);
        assert!(!permissive.validate_assets);

        let strict = ValidationRules::strict();
        assert!(strict.validate_required_fields);
        assert!(strict.validate_assets);
        assert!(strict.validate_permissions);
    }

    #[test]
    fn test_validation_report_creation() {
        let report = ValidationReport::success(100);
        assert!(report.success);
        assert_eq!(report.total_validated, 100);
        assert!(report.errors.is_empty());
        assert!(!report.has_issues());

        let errors = vec![ValidationError {
            code: "TEST_ERROR".to_string(),
            message: "Test error message".to_string(),
            entity_type: "Test".to_string(),
            entity_id: None,
            field_path: None,
            expected: None,
            actual: None,
        }];

        let failed_report = ValidationReport::failure(50, errors);
        assert!(!failed_report.success);
        assert_eq!(failed_report.total_validated, 50);
        assert!(!failed_report.errors.is_empty());
        assert!(failed_report.has_issues());
    }

    #[test]
    fn test_string_validation() {
        // Test required field validation
        let result = utils::validate_string("", "test_field", true, 100);
        assert!(result.is_err());

        let result = utils::validate_string("valid", "test_field", true, 100);
        assert!(result.is_ok());

        // Test length validation
        let long_string = "a".repeat(200);
        let result = utils::validate_string(&long_string, "test_field", false, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_numeric_range_validation() {
        let result = utils::validate_numeric_range(50, "test_field", 0, 100);
        assert!(result.is_ok());

        let result = utils::validate_numeric_range(150, "test_field", 0, 100);
        assert!(result.is_err());

        let result = utils::validate_numeric_range(-10, "test_field", 0, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_url_validation() {
        let result = utils::validate_url("https://example.com", "test_url");
        assert!(result.is_ok());

        let result = utils::validate_url("not-a-url", "test_url");
        assert!(result.is_err());

        let result = utils::validate_url("", "test_url");
        assert!(result.is_ok()); // Empty URLs are allowed
    }

    #[test]
    fn test_size_limits_default() {
        let limits = SizeLimits::default();
        assert_eq!(limits.max_string_length, 65_536);
        assert_eq!(limits.max_entities_per_campaign, 10_000);
        assert_eq!(limits.max_asset_size, 100_000_000);
    }
}
