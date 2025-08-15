//! Tests for Roll20 validation functionality

use super::*;
use serde_json::json;
use ttrpg_core::types::*;

#[cfg(test)]
mod roll20_validation_engine_tests {
    use super::*;

    #[test]
    fn test_validation_engine_creation() {
        let engine = Roll20ValidationEngine::new();
        assert_eq!(engine.asset_patterns.len(), 2);
        assert_eq!(engine.performance_thresholds.max_actors, 1000);
    }

    #[test]
    fn test_default_performance_thresholds() {
        let thresholds = PerformanceThresholds::default();
        assert_eq!(thresholds.max_actors, 1000);
        assert_eq!(thresholds.max_scenes, 100);
        assert_eq!(thresholds.max_items, 5000);
        assert_eq!(thresholds.max_file_size_mb, 100);
        assert_eq!(thresholds.max_asset_count, 1000);
    }

    #[test]
    fn test_validation_stats_calculation() {
        let mut engine = Roll20ValidationEngine::new();
        let issues = vec![
            Roll20ValidationIssue {
                severity: ValidationSeverity::Error,
                category: ValidationCategory::StructureValidation,
                message: "Test error".to_string(),
                context: ValidationContext {
                    entity_type: "test".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: HashMap::new(),
                },
                suggested_fix: None,
                data_path: "$".to_string(),
            },
            Roll20ValidationIssue {
                severity: ValidationSeverity::Warning,
                category: ValidationCategory::DataValidation,
                message: "Test warning".to_string(),
                context: ValidationContext {
                    entity_type: "test".to_string(),
                    entity_id: None,
                    entity_name: None,
                    additional_data: HashMap::new(),
                },
                suggested_fix: None,
                data_path: "$".to_string(),
            },
        ];

        let stats = engine.calculate_validation_stats(&issues);
        assert_eq!(stats.total_issues, 2);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 1);
        assert_eq!(stats.info, 0);
        assert_eq!(stats.categories.len(), 2);
    }

    #[test]
    fn test_roll20_structure_validation_valid() {
        let engine = Roll20ValidationEngine::new();
        let valid_roll20_data = json!({
            "campaign": {"name": "Test Campaign"},
            "characters": [],
            "pages": [],
            "handouts": [],
            "assets": []
        });

        let mut issues = Vec::new();
        let result = engine.validate_roll20_structure(&valid_roll20_data, &mut issues);
        assert!(result.is_ok());
        assert_eq!(issues.len(), 0);
    }

    #[test]
    fn test_roll20_structure_validation_missing_fields() {
        let engine = Roll20ValidationEngine::new();
        let invalid_roll20_data = json!({
            "campaign": {"name": "Test Campaign"}
            // Missing required fields: characters, pages, handouts
        });

        let mut issues = Vec::new();
        let result = engine.validate_roll20_structure(&invalid_roll20_data, &mut issues);
        assert!(result.is_ok());
        assert_eq!(issues.len(), 3); // Missing characters, pages, handouts

        for issue in &issues {
            assert_eq!(issue.severity, ValidationSeverity::Error);
            assert_eq!(issue.category, ValidationCategory::StructureValidation);
        }
    }

    #[test]
    fn test_roll20_structure_validation_invalid_arrays() {
        let engine = Roll20ValidationEngine::new();
        let invalid_roll20_data = json!({
            "campaign": {"name": "Test Campaign"},
            "characters": "not_an_array",
            "pages": [],
            "handouts": [],
            "assets": {}
        });

        let mut issues = Vec::new();
        let result = engine.validate_roll20_structure(&invalid_roll20_data, &mut issues);
        assert!(result.is_ok());
        assert_eq!(issues.len(), 5); // Missing characters, pages, handouts + 2 invalid array types
    }

    #[test]
    fn test_asset_reference_validation() {
        let engine = Roll20ValidationEngine::new();
        let asset_references = vec![
            AssetReference {
                url: "https://s3.amazonaws.com/files.d20.io/image.png".to_string(),
                asset_type: "image".to_string(),
                referenced_by: "character1".to_string(),
                is_accessible: Some(true),
            },
            AssetReference {
                url: "https://invalid-domain.com/suspicious.exe".to_string(),
                asset_type: "unknown".to_string(),
                referenced_by: "character2".to_string(),
                is_accessible: Some(false),
            },
        ];

        let mut issues = Vec::new();
        let result = engine.validate_asset_references(&asset_references, &mut issues);
        assert!(result.is_ok());
        assert_eq!(issues.len(), 1); // Only the invalid URL should trigger a warning
        assert_eq!(issues[0].severity, ValidationSeverity::Warning);
        assert_eq!(issues[0].category, ValidationCategory::AssetValidation);
    }

    #[test]
    fn test_relationship_validation_with_missing_references() {
        let engine = Roll20ValidationEngine::new();

        let mut campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);

        // Add an actor
        let actor = Actor {
            id: "actor1".to_string(),
            name: "Test Actor".to_string(),
            items: vec!["missing_item".to_string()],
            ..Default::default()
        };
        campaign.actors.push(actor);

        // Add a scene with a token referencing missing actor
        let scene = Scene {
            id: "scene1".to_string(),
            name: "Test Scene".to_string(),
            tokens: vec![Token {
                id: "token1".to_string(),
                name: "Test Token".to_string(),
                actor_id: Some("missing_actor".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };
        campaign.scenes.push(scene);

        let mut issues = Vec::new();
        let result = engine.validate_entity_relationships(&campaign, &mut issues);
        assert!(result.is_ok());
        assert_eq!(issues.len(), 2); // Missing actor reference + missing item reference

        // Check that we have both error types
        let error_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .collect();
        let warning_issues: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Warning)
            .collect();
        assert_eq!(error_issues.len(), 1); // Missing actor reference
        assert_eq!(warning_issues.len(), 1); // Missing item reference
    }
}

#[cfg(test)]
mod roll20_validation_result_tests {
    use super::*;

    #[test]
    fn test_validation_result_default() {
        let result = Roll20ValidationResult::default();
        assert!(result.is_valid);
        assert_eq!(result.issues.len(), 0);
        assert_eq!(result.stats.total_issues, 0);
    }

    #[test]
    fn test_validation_result_add_issue() {
        let mut result = Roll20ValidationResult::default();
        assert!(result.is_valid());

        let warning_issue = Roll20ValidationIssue {
            severity: ValidationSeverity::Warning,
            category: ValidationCategory::DataValidation,
            message: "Test warning".to_string(),
            context: ValidationContext {
                entity_type: "test".to_string(),
                entity_id: None,
                entity_name: None,
                additional_data: HashMap::new(),
            },
            suggested_fix: None,
            data_path: "$".to_string(),
        };

        result.add_issue(warning_issue);
        assert!(result.is_valid()); // Still valid with warnings
        assert_eq!(result.issues.len(), 1);

        let error_issue = Roll20ValidationIssue {
            severity: ValidationSeverity::Error,
            category: ValidationCategory::StructureValidation,
            message: "Test error".to_string(),
            context: ValidationContext {
                entity_type: "test".to_string(),
                entity_id: None,
                entity_name: None,
                additional_data: HashMap::new(),
            },
            suggested_fix: None,
            data_path: "$".to_string(),
        };

        result.add_issue(error_issue);
        assert!(!result.is_valid()); // Invalid with errors
        assert_eq!(result.issues.len(), 2);
    }
}

#[cfg(test)]
mod roll20_conversion_context_tests {
    use super::*;

    #[test]
    fn test_conversion_context_from_valid_json() {
        let json_str = r#"{"campaign": {"name": "Test"}, "characters": [], "pages": []}"#;
        let context = Roll20ConversionContext::from_json(json_str);
        assert!(context.is_ok());

        let context = context.unwrap();
        assert_eq!(context.converted_campaign.name, "Default Campaign");
        assert_eq!(context.converted_campaign.source_format, SourceFormat::Roll20);
    }

    #[test]
    fn test_conversion_context_from_invalid_json() {
        let invalid_json = r#"{"invalid": json syntax"#;
        let context = Roll20ConversionContext::from_json(invalid_json);
        assert!(context.is_err());
    }

    #[test]
    fn test_conversion_stats_default() {
        let stats = ConversionStats::default();
        assert_eq!(stats.total_characters, 0);
        assert_eq!(stats.converted_actors, 0);
        assert_eq!(stats.total_pages, 0);
        assert_eq!(stats.converted_scenes, 0);
        assert_eq!(stats.total_handouts, 0);
        assert_eq!(stats.converted_items, 0);
        assert_eq!(stats.failed_conversions, 0);
        assert_eq!(stats.warnings_count, 0);
    }
}

#[cfg(test)]
mod validation_severity_and_category_tests {
    use super::*;

    #[test]
    fn test_validation_severity_equality() {
        assert_eq!(ValidationSeverity::Error, ValidationSeverity::Error);
        assert_eq!(ValidationSeverity::Warning, ValidationSeverity::Warning);
        assert_eq!(ValidationSeverity::Info, ValidationSeverity::Info);
    }

    #[test]
    fn test_validation_category_equality() {
        assert_eq!(
            ValidationCategory::StructureValidation,
            ValidationCategory::StructureValidation
        );
        assert_eq!(ValidationCategory::DataValidation, ValidationCategory::DataValidation);
        assert_eq!(ValidationCategory::Conversion, ValidationCategory::Conversion);
        assert_eq!(ValidationCategory::AssetValidation, ValidationCategory::AssetValidation);
        assert_eq!(
            ValidationCategory::RelationshipValidation,
            ValidationCategory::RelationshipValidation
        );
        assert_eq!(
            ValidationCategory::PerformanceValidation,
            ValidationCategory::PerformanceValidation
        );
    }

    #[test]
    fn test_validation_category_hash() {
        let mut categories = HashSet::new();
        categories.insert(ValidationCategory::StructureValidation);
        categories.insert(ValidationCategory::DataValidation);
        categories.insert(ValidationCategory::StructureValidation); // Duplicate

        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&ValidationCategory::StructureValidation));
        assert!(categories.contains(&ValidationCategory::DataValidation));
    }
}

#[cfg(test)]
mod performance_metrics_tests {
    use super::*;

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.validation_duration_ms, 0);
        assert_eq!(metrics.entities_validated, 0);
        assert_eq!(metrics.relationships_checked, 0);
        assert_eq!(metrics.assets_verified, 0);
    }

    #[test]
    fn test_validation_stats_default() {
        let stats = ValidationStats::default();
        assert_eq!(stats.total_issues, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.warnings, 0);
        assert_eq!(stats.info, 0);
        assert_eq!(stats.categories.len(), 0);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn create_test_campaign_with_issues() -> Campaign {
        let mut campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);

        // Add actors with references to missing items
        let actor = Actor {
            id: "actor1".to_string(),
            name: "Test Actor".to_string(),
            items: vec!["missing_item1".to_string(), "missing_item2".to_string()],
            ..Default::default()
        };
        campaign.actors.push(actor);

        // Add scenes with tokens referencing missing actors
        let scene = Scene {
            id: "scene1".to_string(),
            name: "Test Scene".to_string(),
            tokens: vec![
                Token {
                    id: "token1".to_string(),
                    name: "Valid Token".to_string(),
                    actor_id: Some("actor1".to_string()), // Valid reference
                    ..Default::default()
                },
                Token {
                    id: "token2".to_string(),
                    name: "Invalid Token".to_string(),
                    actor_id: Some("missing_actor".to_string()), // Invalid reference
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        campaign.scenes.push(scene);

        campaign
    }

    #[test]
    fn test_comprehensive_validation() {
        let mut engine = Roll20ValidationEngine::new();

        let roll20_data = json!({
            "campaign": {"name": "Test Campaign"},
            "characters": [{"id": "char1", "name": "Character 1"}],
            "pages": [{"id": "page1", "name": "Page 1"}],
            "handouts": [],
            "assets": []
        });

        let context = Roll20ConversionContext {
            roll20_data,
            converted_campaign: create_test_campaign_with_issues(),
            conversion_stats: ConversionStats {
                total_characters: 1,
                converted_actors: 1,
                total_pages: 1,
                converted_scenes: 1,
                total_handouts: 0,
                converted_items: 0,
                failed_conversions: 0,
                warnings_count: 0,
            },
            asset_references: vec![AssetReference {
                url: "https://s3.amazonaws.com/files.d20.io/valid.png".to_string(),
                asset_type: "image".to_string(),
                referenced_by: "actor1".to_string(),
                is_accessible: Some(true),
            }],
        };

        let result = engine.validate_roll20_conversion(&context);
        assert!(result.is_ok());

        let validation_result = result.unwrap();

        // Should find relationship validation issues (missing item references + missing actor reference)
        assert!(!validation_result.is_valid()); // Should have errors due to missing actor reference
        assert!(validation_result.issues.len() > 0);

        // Check performance metrics are populated
        assert!(validation_result.performance_metrics.validation_duration_ms > 0);
        assert_eq!(validation_result.performance_metrics.entities_validated, 2); // 1 actor + 1 scene
        assert_eq!(validation_result.performance_metrics.assets_verified, 1);
    }

    #[test]
    fn test_performance_validation_triggers() {
        let mut engine = Roll20ValidationEngine::new();

        // Override performance thresholds for testing
        engine.performance_thresholds = PerformanceThresholds {
            max_actors: 1,
            max_scenes: 1,
            max_items: 1,
            max_file_size_mb: 1,
            max_asset_count: 1,
        };

        let context = Roll20ConversionContext {
            roll20_data: json!({}),
            converted_campaign: Campaign::new("Test".to_string(), SourceFormat::Roll20),
            conversion_stats: ConversionStats {
                converted_actors: 2, // Exceeds threshold of 1
                converted_scenes: 2, // Exceeds threshold of 1
                ..Default::default()
            },
            asset_references: vec![
                AssetReference {
                    url: "https://example.com/asset1.png".to_string(),
                    asset_type: "image".to_string(),
                    referenced_by: "test".to_string(),
                    is_accessible: Some(true),
                },
                AssetReference {
                    url: "https://example.com/asset2.png".to_string(),
                    asset_type: "image".to_string(),
                    referenced_by: "test".to_string(),
                    is_accessible: Some(true),
                }, // Exceeds threshold of 1
            ],
        };

        let result = engine.validate_roll20_conversion(&context);
        assert!(result.is_ok());

        let validation_result = result.unwrap();

        // Should find performance warnings
        let perf_issues: Vec<_> = validation_result
            .issues
            .iter()
            .filter(|i| i.category == ValidationCategory::PerformanceValidation)
            .collect();

        assert!(perf_issues.len() >= 2); // Should warn about actor count and scene count
    }
}
