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
//! use ttrpg_core::validation::{Validator, ValidationContext, ValidationRules};
//! use ttrpg_core::types::Campaign;
//!
//! let campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);
//! let context = ValidationContext::default();
//! let rules = ValidationRules::strict();
//!
//! match campaign.validate(&context, &rules) {
//!     Ok(report) => println!("Validation passed: {}", report.summary()),
//!     Err(e) => eprintln!("Validation failed: {}", e),
//! }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Campaign, SourceFormat};

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
