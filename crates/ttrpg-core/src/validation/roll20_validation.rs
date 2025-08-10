//! Enhanced validation engine for Roll20 conversions
//!
//! This module provides specialized validation rules for Roll20-to-ttrpg-core conversions,
//! building on the professional validation foundation from M1.3 with enhanced rules
//! specific to Roll20 data integrity, relationship validation, and conversion quality.
//!
//! # Key Features
//!
//! - **Roll20-Specific Rules**: Validates Roll20 JSON structure and required fields
//! - **Conversion Quality**: Ensures data integrity during Roll20→ttrpg-core conversion
//! - **Relationship Validation**: Validates complex relationships between actors, scenes, items
//! - **Performance Optimized**: Efficient validation for large campaign files
//! - **Enhanced Error Reporting**: Detailed context and suggestions for conversion issues
//!
//! # Usage
//!
//! ```rust,no_run
//! use ttrpg_core::validation::roll20_validation::{Roll20ValidationEngine, Roll20ConversionContext};
//! use ttrpg_core::types::Campaign;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut engine = Roll20ValidationEngine::new();
//! let context = Roll20ConversionContext::from_json("{\"campaign\": {...}}")?;
//!
//! let result = engine.validate_roll20_conversion(&context)?;
//! if !result.is_valid {
//!     for issue in result.issues {
//!         eprintln!("Validation error: {}", issue.message);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use crate::error::{ConversionError, ConversionResult};
use crate::types::*;
use regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ============================================================================
// ROLL20 CONVERSION VALIDATION STRUCTS
// ============================================================================

/// Enhanced validation context for Roll20 conversions
#[derive(Debug, Clone)]
pub struct Roll20ConversionContext {
    /// Original Roll20 JSON data
    pub roll20_data: serde_json::Value,
    /// Converted ttrpg-core campaign
    pub converted_campaign: Campaign,
    /// Conversion statistics
    pub conversion_stats: ConversionStats,
    /// Asset references found during conversion
    pub asset_references: Vec<AssetReference>,
}

/// Conversion statistics for validation reporting
#[derive(Debug, Clone, Default)]
pub struct ConversionStats {
    pub total_characters: usize,
    pub converted_actors: usize,
    pub total_pages: usize,
    pub converted_scenes: usize,
    pub total_handouts: usize,
    pub converted_items: usize,
    pub failed_conversions: usize,
    pub warnings_count: usize,
}

/// Asset reference for validation
#[derive(Debug, Clone)]
pub struct AssetReference {
    pub url: String,
    pub asset_type: String,
    pub referenced_by: String,
    pub is_accessible: Option<bool>,
}

/// Enhanced validation issue with Roll20-specific context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20ValidationIssue {
    /// Severity level
    pub severity: ValidationSeverity,
    /// Issue category
    pub category: ValidationCategory,
    /// Human-readable message
    pub message: String,
    /// Detailed context about the issue
    pub context: ValidationContext,
    /// Suggested fix if available
    pub suggested_fix: Option<String>,
    /// Path to the problematic data
    pub data_path: String,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationSeverity {
    /// Critical errors that prevent conversion
    Error,
    /// Non-critical issues that may affect quality
    Warning,
    /// Informational notices
    Info,
}

/// Validation categories for better error organization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ValidationCategory {
    /// Roll20 JSON structure issues
    StructureValidation,
    /// Data type and format validation
    DataValidation,
    /// Conversion quality issues
    Conversion,
    /// Asset and reference validation
    AssetValidation,
    /// Relationship validation between entities
    RelationshipValidation,
    /// Performance and size validation
    PerformanceValidation,
}

/// Detailed validation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationContext {
    /// Entity type being validated
    pub entity_type: String,
    /// Entity ID if available
    pub entity_id: Option<String>,
    /// Entity name if available
    pub entity_name: Option<String>,
    /// Additional context data
    pub additional_data: HashMap<String, serde_json::Value>,
}

/// Comprehensive validation result with detailed reporting
#[derive(Debug, Clone)]
pub struct Roll20ValidationResult {
    /// Whether the validation passed (no errors)
    pub is_valid: bool,
    /// All validation issues found
    pub issues: Vec<Roll20ValidationIssue>,
    /// Aggregated validation statistics
    pub stats: ValidationStats,
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
}

impl Default for Roll20ValidationResult {
    fn default() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
            stats: ValidationStats::default(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }
}

impl Roll20ValidationResult {
    /// Add a validation issue to the result
    pub fn add_issue(&mut self, issue: Roll20ValidationIssue) {
        if matches!(issue.severity, ValidationSeverity::Error) {
            self.is_valid = false;
        }
        self.issues.push(issue);
    }

    /// Check if validation passed (no errors)
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

/// Performance metrics for validation
#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub validation_duration_ms: u64,
    pub entities_validated: usize,
    pub relationships_checked: usize,
    pub assets_verified: usize,
}

/// Validation statistics
#[derive(Debug, Clone, Default)]
pub struct ValidationStats {
    pub total_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
    pub categories: HashMap<ValidationCategory, usize>,
}

// ============================================================================
// ROLL20 VALIDATION ENGINE
// ============================================================================

/// Enhanced validation engine for Roll20 conversions
pub struct Roll20ValidationEngine {
    /// Asset URL patterns for validation
    asset_patterns: Vec<regex::Regex>,
    /// Performance thresholds
    performance_thresholds: PerformanceThresholds,
}

/// Performance thresholds for validation warnings
#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub max_actors: usize,
    pub max_scenes: usize,
    pub max_items: usize,
    pub max_file_size_mb: usize,
    pub max_asset_count: usize,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            max_actors: 1000,
            max_scenes: 100,
            max_items: 5000,
            max_file_size_mb: 100,
            max_asset_count: 1000,
        }
    }
}

impl Roll20ValidationEngine {
    /// Create a new Roll20 validation engine
    pub fn new() -> Self {
        // Asset URL patterns
        let d20_regex = regex::Regex::new(r"https://s3\.amazonaws\.com/files\.d20\.io/.*")
            .expect("Invalid regex pattern for D20 assets");
        let roll20_regex = regex::Regex::new(r"https://.*\.roll20\.net/.*")
            .expect("Invalid regex pattern for Roll20 assets");

        Self {
            asset_patterns: vec![d20_regex, roll20_regex],
            performance_thresholds: PerformanceThresholds::default(),
        }
    }

    /// Validate a complete Roll20 conversion with enhanced rules
    pub fn validate_roll20_conversion(
        &mut self,
        context: &Roll20ConversionContext,
    ) -> ConversionResult<Roll20ValidationResult> {
        let start_time = std::time::Instant::now();
        let mut issues = Vec::new();

        // 1. Validate Roll20 JSON structure
        self.validate_roll20_structure(&context.roll20_data, &mut issues)?;

        // 2. Validate conversion completeness and quality
        self.validate_conversion_quality(context, &mut issues)?;

        // 3. Validate entity relationships
        self.validate_entity_relationships(&context.converted_campaign, &mut issues)?;

        // 4. Validate asset references
        self.validate_asset_references(&context.asset_references, &mut issues)?;

        // 5. Validate performance characteristics
        self.validate_performance_metrics(context, &mut issues)?;

        // Calculate statistics
        let stats = self.calculate_validation_stats(&issues);
        let performance_metrics = PerformanceMetrics {
            validation_duration_ms: start_time.elapsed().as_millis() as u64,
            entities_validated: context.conversion_stats.converted_actors
                + context.conversion_stats.converted_scenes
                + context.conversion_stats.converted_items,
            relationships_checked: self.count_relationships(&context.converted_campaign),
            assets_verified: context.asset_references.len(),
        };

        let result = Roll20ValidationResult {
            is_valid: stats.errors == 0,
            issues,
            stats,
            performance_metrics,
        };

        Ok(result)
    }

    /// Validate Roll20 JSON structure integrity
    fn validate_roll20_structure(
        &self,
        roll20_data: &serde_json::Value,
        issues: &mut Vec<Roll20ValidationIssue>,
    ) -> ConversionResult<()> {
        // Check for required top-level fields
        if let Some(obj) = roll20_data.as_object() {
            let required_fields = ["campaign", "characters", "pages", "handouts"];
            for field in &required_fields {
                if !obj.contains_key(*field) {
                    issues.push(Roll20ValidationIssue {
                        severity: ValidationSeverity::Error,
                        category: ValidationCategory::StructureValidation,
                        message: format!("Missing required Roll20 field: {field}"),
                        context: ValidationContext {
                            entity_type: "roll20_root".to_string(),
                            entity_id: None,
                            entity_name: None,
                            additional_data: HashMap::new(),
                        },
                        suggested_fix: Some(format!("Ensure Roll20 export includes {field} field")),
                        data_path: format!("$.{field}"),
                    });
                }
            }

            // Validate array fields are actually arrays
            let array_fields = ["characters", "pages", "handouts", "assets"];
            for field in &array_fields {
                if let Some(value) = obj.get(*field) {
                    if !value.is_array() {
                        issues.push(Roll20ValidationIssue {
                            severity: ValidationSeverity::Error,
                            category: ValidationCategory::DataValidation,
                            message: format!("Roll20 field {field} must be an array"),
                            context: ValidationContext {
                                entity_type: "roll20_root".to_string(),
                                entity_id: None,
                                entity_name: None,
                                additional_data: HashMap::new(),
                            },
                            suggested_fix: Some(format!("Check Roll20 export format for {field}")),
                            data_path: format!("$.{field}"),
                        });
                    }
                }
            }
        } else {
            issues.push(Roll20ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::StructureValidation,
                message: "Roll20 data must be a JSON object".to_string(),
                context: ValidationContext {
                    entity_type: "roll20_root".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: HashMap::new(),
                },
                suggested_fix: Some("Ensure Roll20 export is valid JSON".to_string()),
                data_path: "$".to_string(),
            });
        }

        Ok(())
    }

    /// Validate conversion completeness and quality
    fn validate_conversion_quality(
        &self,
        context: &Roll20ConversionContext,
        issues: &mut Vec<Roll20ValidationIssue>,
    ) -> ConversionResult<()> {
        let stats = &context.conversion_stats;

        // Check conversion completeness
        if stats.total_characters > 0 && stats.converted_actors == 0 {
            issues.push(Roll20ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::Conversion,
                message: "No characters were successfully converted to actors".to_string(),
                context: ValidationContext {
                    entity_type: "conversion_stats".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: [
                        ("total_characters".to_string(), stats.total_characters.into()),
                        ("converted_actors".to_string(), stats.converted_actors.into()),
                    ]
                    .into(),
                },
                suggested_fix: Some("Check character data format and conversion logic".to_string()),
                data_path: "$.characters".to_string(),
            });
        }

        // Check conversion rates
        if stats.total_characters > 0 {
            let conversion_rate = (stats.converted_actors as f64) / (stats.total_characters as f64);
            if conversion_rate < 0.8 {
                issues.push(Roll20ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: ValidationCategory::Conversion,
                    message: format!(
                        "Low character conversion rate: {:.1}%",
                        conversion_rate * 100.0
                    ),
                    context: ValidationContext {
                        entity_type: "conversion_stats".to_string(),
                        entity_id: None,
                        entity_name: None,
                        additional_data: [
                            ("conversion_rate".to_string(), conversion_rate.into()),
                            ("failed_conversions".to_string(), stats.failed_conversions.into()),
                        ]
                        .into(),
                    },
                    suggested_fix: Some(
                        "Review failed character conversions for data issues".to_string(),
                    ),
                    data_path: "$.characters".to_string(),
                });
            }
        }

        // Similar validation for pages and handouts
        if stats.total_pages > 0 {
            let scene_conversion_rate =
                (stats.converted_scenes as f64) / (stats.total_pages as f64);
            if scene_conversion_rate < 0.9 {
                issues.push(Roll20ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: ValidationCategory::Conversion,
                    message: format!(
                        "Low scene conversion rate: {:.1}%",
                        scene_conversion_rate * 100.0
                    ),
                    context: ValidationContext {
                        entity_type: "conversion_stats".to_string(),
                        entity_id: None,
                        entity_name: None,
                        additional_data: [(
                            "scene_conversion_rate".to_string(),
                            scene_conversion_rate.into(),
                        )]
                        .into(),
                    },
                    suggested_fix: Some(
                        "Review failed scene conversions for data issues".to_string(),
                    ),
                    data_path: "$.pages".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate relationships between entities
    fn validate_entity_relationships(
        &self,
        campaign: &Campaign,
        issues: &mut Vec<Roll20ValidationIssue>,
    ) -> ConversionResult<()> {
        // Collect all entity IDs for reference validation
        let actor_ids: HashSet<String> = campaign.actors.iter().map(|a| a.id.clone()).collect();
        let _scene_ids: HashSet<String> = campaign.scenes.iter().map(|s| s.id.clone()).collect();
        let item_ids: HashSet<String> = campaign.items.iter().map(|i| i.id.clone()).collect();

        // Validate actor references in scenes (tokens)
        for scene in &campaign.scenes {
            for token in &scene.tokens {
                if let Some(ref actor_id) = token.actor_id {
                    if !actor_ids.contains(actor_id) {
                        issues.push(Roll20ValidationIssue {
                            severity: ValidationSeverity::Error,
                            category: ValidationCategory::RelationshipValidation,
                            message: format!("Token references non-existent actor: {actor_id}"),
                            context: ValidationContext {
                                entity_type: "scene".to_string(),
                                entity_id: Some(scene.id.clone()),
                                entity_name: Some(scene.name.clone()),
                                additional_data: [
                                    ("token_id".to_string(), token.id.clone().into()),
                                    ("missing_actor_id".to_string(), actor_id.clone().into()),
                                ]
                                .into(),
                            },
                            suggested_fix: Some(
                                "Ensure all referenced actors exist or remove invalid tokens"
                                    .to_string(),
                            ),
                            data_path: format!(
                                "$.scenes[?(@.id=='{}')].tokens[?(@.actor_id=='{}')]",
                                scene.id, actor_id
                            ),
                        });
                    }
                }
            }
        }

        // Validate item references in actors
        for actor in &campaign.actors {
            for item_id in &actor.items {
                if !item_ids.contains(item_id) {
                    issues.push(Roll20ValidationIssue {
                        severity: ValidationSeverity::Warning,
                        category: ValidationCategory::RelationshipValidation,
                        message: format!("Actor references potentially missing item: {item_id}"),
                        context: ValidationContext {
                            entity_type: "actor".to_string(),
                            entity_id: Some(actor.id.clone()),
                            entity_name: Some(actor.name.clone()),
                            additional_data: [(
                                "missing_item_id".to_string(),
                                item_id.clone().into(),
                            )]
                            .into(),
                        },
                        suggested_fix: Some(
                            "Verify item exists in campaign or remove reference".to_string(),
                        ),
                        data_path: format!(
                            "$.actors[?(@.id=='{}')].items[?(@=='{}')]",
                            actor.id, item_id
                        ),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate asset references and accessibility
    fn validate_asset_references(
        &self,
        asset_references: &[AssetReference],
        issues: &mut Vec<Roll20ValidationIssue>,
    ) -> ConversionResult<()> {
        for asset_ref in asset_references {
            // Validate URL format using url field
            let is_valid_roll20_url = self
                .asset_patterns
                .iter()
                .any(|pattern| pattern.is_match(&asset_ref.url));

            if !is_valid_roll20_url {
                issues.push(Roll20ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    category: ValidationCategory::AssetValidation,
                    message: format!("Non-standard Roll20 asset URL: {}", asset_ref.url),
                    context: ValidationContext {
                        entity_type: "asset_reference".to_string(),
                        entity_id: None,
                        entity_name: None,
                        additional_data: [
                            ("asset_url".to_string(), asset_ref.url.clone().into()),
                            ("asset_type".to_string(), asset_ref.asset_type.clone().into()),
                        ]
                        .into(),
                    },
                    suggested_fix: Some(
                        "Verify asset URL is accessible and consider downloading".to_string(),
                    ),
                    data_path: format!("asset_references[?(@.url=='{}')]", asset_ref.url),
                });
            }
        }

        Ok(())
    }

    /// Validate performance characteristics
    fn validate_performance_metrics(
        &self,
        context: &Roll20ConversionContext,
        issues: &mut Vec<Roll20ValidationIssue>,
    ) -> ConversionResult<()> {
        let stats = &context.conversion_stats;
        let thresholds = &self.performance_thresholds;

        // Check entity count thresholds
        if stats.converted_actors > thresholds.max_actors {
            issues.push(Roll20ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: ValidationCategory::PerformanceValidation,
                message: format!(
                    "Large number of actors ({}), may impact performance",
                    stats.converted_actors
                ),
                context: ValidationContext {
                    entity_type: "performance_metrics".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: [
                        ("actor_count".to_string(), stats.converted_actors.into()),
                        ("threshold".to_string(), thresholds.max_actors.into()),
                    ]
                    .into(),
                },
                suggested_fix: Some(
                    "Consider splitting large campaigns or optimizing actor data".to_string(),
                ),
                data_path: "$.actors".to_string(),
            });
        }

        if stats.converted_scenes > thresholds.max_scenes {
            issues.push(Roll20ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: ValidationCategory::PerformanceValidation,
                message: format!(
                    "Large number of scenes ({}), may impact performance",
                    stats.converted_scenes
                ),
                context: ValidationContext {
                    entity_type: "performance_metrics".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: [
                        ("scene_count".to_string(), stats.converted_scenes.into()),
                        ("threshold".to_string(), thresholds.max_scenes.into()),
                    ]
                    .into(),
                },
                suggested_fix: Some(
                    "Consider organizing scenes into separate campaigns".to_string(),
                ),
                data_path: "$.scenes".to_string(),
            });
        }

        if context.asset_references.len() > thresholds.max_asset_count {
            issues.push(Roll20ValidationIssue {
                severity: ValidationSeverity::Info,
                category: ValidationCategory::PerformanceValidation,
                message: format!(
                    "Large number of assets ({}), download may take time",
                    context.asset_references.len()
                ),
                context: ValidationContext {
                    entity_type: "performance_metrics".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: [
                        ("asset_count".to_string(), context.asset_references.len().into()),
                        ("threshold".to_string(), thresholds.max_asset_count.into()),
                    ]
                    .into(),
                },
                suggested_fix: Some("Consider selective asset downloading".to_string()),
                data_path: "asset_references".to_string(),
            });
        }

        Ok(())
    }

    /// Calculate validation statistics
    fn calculate_validation_stats(&self, issues: &[Roll20ValidationIssue]) -> ValidationStats {
        let mut stats = ValidationStats { total_issues: issues.len(), ..Default::default() };

        for issue in issues {
            match issue.severity {
                ValidationSeverity::Error => stats.errors += 1,
                ValidationSeverity::Warning => stats.warnings += 1,
                ValidationSeverity::Info => stats.info += 1,
            }

            *stats.categories.entry(issue.category.clone()).or_insert(0) += 1;
        }

        stats
    }

    /// Count relationships in campaign for metrics
    fn count_relationships(&self, campaign: &Campaign) -> usize {
        let mut count = 0;

        // Count tokens referencing actors
        for scene in &campaign.scenes {
            count += scene.tokens.len();
        }

        // Count items referenced by actors
        for actor in &campaign.actors {
            count += actor.items.len();
        }

        count
    }
}

impl Default for Roll20ValidationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl Roll20ConversionContext {
    /// Create Roll20 conversion context from JSON string
    pub fn from_json(json_str: &str) -> ConversionResult<Self> {
        let roll20_data: serde_json::Value = serde_json::from_str(json_str).map_err(|e| {
            ConversionError::validation("Roll20ConversionContext", format!("Invalid JSON: {e}"))
        })?;

        // This would typically be filled by the actual conversion process
        Ok(Self {
            roll20_data,
            converted_campaign: Campaign::new("Default Campaign".to_string(), SourceFormat::Roll20),
            conversion_stats: ConversionStats::default(),
            asset_references: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests;
