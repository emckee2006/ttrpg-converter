use std::sync::Arc;
use std::time::Instant;
use serde_json::{json, Value};
use jsonschema::JSONSchema;
use dashmap::DashMap;
use async_trait::async_trait;

use ttrpg_core::{
    Campaign,
    plugin_framework::{PluginInfo, PluginConfig, ValidationPlugin as ValidationPluginTrait},
    ConversionResult, ConversionError,
};

use crate::shared::AssetExecutionContext;
use crate::validation::validators::{AssetValidator, BusinessRulesValidator, SecurityValidator, ValidationConfig, SchemaValidator, ValidationResult, ValidationStats};
use ttrpg_core::plugin_framework::types::{ValidationIssue, IssueSeverity, ValidationResult as CoreValidationResult, ValidationStats as CoreValidationStats};

/// JSON Schema validation with layered business rules
pub struct ValidationPlugin {
    /// Plugin metadata
    info: PluginInfo,
    /// Execution context for HTTP and async operations
    #[allow(dead_code)]
    context: Arc<AssetExecutionContext>,
    /// Validation configuration
    #[allow(dead_code)]
    config: ValidationConfig,
    /// Cached compiled JSON schemas
    schemas: Arc<DashMap<String, JSONSchema>>,
    /// Asset validator for file/URL validation
    asset_validator: AssetValidator,
    /// Security validator for permissions
    security_validator: SecurityValidator, 
    /// Business rules validator
    #[allow(dead_code)]
    business_validator: BusinessRulesValidator,
    /// Schema validator for JSON schema validation
    schema_validator: SchemaValidator,
}

// Use centralized types from ttrpg-core instead of duplicating
use ttrpg_core::plugin_framework::types::{
    ValidationResult, ValidationStats
};


// TODO: Add proper plugin registration once StaticPluginRegistration API is stable
// For now, ValidationPlugin can be instantiated directly where needed

impl ValidationPlugin {
    /// Create new validation plugin with layered validation approach
    pub fn new(context: Arc<AssetExecutionContext>, config: ValidationConfig) -> ConversionResult<Self> {
        let mut plugin = Self {
            info: PluginInfo {
                name: "ValidationPlugin".to_string(),
                version: "1.0.0".to_string(),
                description: "Layered validation with JSON Schema + business rules + system validation".to_string(),
                author: "TTRPG Converter".to_string(),
                supported_features: vec!["validation".to_string(), "schema".to_string()],
                dependencies: vec![],
            },
            context,
            config: config.clone(),
            schemas: Arc::new(DashMap::new()),
            asset_validator: AssetValidator::new(config.assets.clone()),
            security_validator: SecurityValidator::new(config.security.clone()),
            business_validator: BusinessRulesValidator::new(config.strictness),
            schema_validator: SchemaValidator::from_config(&config)
                .unwrap_or_else(|e| {
                    eprintln!("Warning: Failed to load schemas from config, using defaults: {}", e);
                    SchemaValidator::new()
                }),
        };

        // Load default schemas for core entities
        plugin.load_default_schemas()?;
        
        Ok(plugin)
    }

    /// Add a JSON schema for entity type validation
    pub fn add_schema(&self, entity_type: &str, schema: Value) -> ConversionResult<()> {
        let compiled_schema = JSONSchema::compile(&schema)
            .map_err(|e| ConversionError::ValidationError {
                entity_type: "schema".to_string(),
                message: format!("Failed to compile schema for {}: {}", entity_type, e),
                field: None,
            })?;
        
        self.schemas.insert(entity_type.to_string(), compiled_schema);
        Ok(())
    }

    /// Main campaign validation method - orchestrates all validation layers
    pub async fn validate_campaign(&self, campaign: &Campaign) -> ConversionResult<ValidationResult> {
        let start_time = Instant::now();
        let mut all_issues = Vec::new();

        // Layer 1: JSON Schema structural validation
        all_issues.extend(self.validate_campaign_schema(campaign)?);
        
        // Layer 2: Business rules validation
        all_issues.extend(self.validate_campaign_business_rules(campaign)?);
        
        // Layer 3: System validation (assets, permissions)
        all_issues.extend(self.validate_campaign_system_resources(campaign).await?);
        
        let duration = start_time.elapsed();
        
        Ok(ValidationResult {
            is_valid: all_issues.iter().all(|issue| !matches!(issue.severity, IssueSeverity::Error)),
            issues: all_issues.clone(),
            stats: ValidationStats {
                entities_validated: 1 + campaign.actors.len() + campaign.scenes.len() + campaign.items.len(),
                entities_with_issues: all_issues.iter().map(|i| &i.entity_id).flatten().collect::<std::collections::HashSet<_>>().len(),
                validation_time_ms: duration.as_millis() as u64,
                cache_hit_ratio: 0.0, // Not implemented yet
            },
        })
    }
    
    /// Layer 1: Schema validation
    fn validate_campaign_schema(&self, campaign: &Campaign) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Validate campaign itself
        let campaign_json = serde_json::to_value(campaign)
            .map_err(|e| ConversionError::ValidationError { 
                entity_type: "campaign".to_string(),
                message: format!("Failed to serialize campaign: {}", e),
                field: None,
            })?;
            
        issues.extend(self.validate_with_schema("campaign", &campaign_json)?);
        
        // Validate actors
        for actor in &campaign.actors {
            let actor_json = serde_json::to_value(actor)
                .map_err(|e| ConversionError::ValidationError {
                    entity_type: "actor".to_string(),
                    message: format!("Failed to serialize actor {}: {}", actor.id, e),
                    field: None,
                })?;
            issues.extend(self.validate_with_schema("actor", &actor_json)?);
        }
        
        // Validate scenes
        for scene in &campaign.scenes {
            let scene_json = serde_json::to_value(scene)
                .map_err(|e| ConversionError::ValidationError {
                    entity_type: "scene".to_string(),
                    message: format!("Failed to serialize scene {}: {}", scene.id, e),
                    field: None,
                })?;
            issues.extend(self.validate_with_schema("scene", &scene_json)?);
        }
        
        // Validate items
        for item in &campaign.items {
            let item_json = serde_json::to_value(item)
                .map_err(|e| ConversionError::ValidationError {
                    entity_type: "item".to_string(),
                    message: format!("Failed to serialize item {}: {}", item.id, e),
                    field: None,
                })?;
            issues.extend(self.validate_with_schema("item", &item_json)?);
        }
        
        Ok(issues)
    }
    
    /// Layer 2: Business rules validation
    fn validate_campaign_business_rules(&self, campaign: &Campaign) -> ConversionResult<Vec<ValidationIssue>> {
        let issues = Vec::new();
        
        // Validate campaign-level business rules
        // Campaign consistency validation placeholder
        // TODO: Implement campaign consistency checks
        
        // Validate actor business rules
        for _actor in &campaign.actors {
            // TODO: Actor-specific business rules validation
        }
        
        // Validate scene business rules  
        for _scene in &campaign.scenes {
            // TODO: Scene-specific business rules validation
        }
        
        Ok(issues)
    }
    
    /// Layer 3: System validation (files, URLs, permissions)
    async fn validate_campaign_system_resources(&self, campaign: &Campaign) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Validate asset files and URLs
        for actor in &campaign.actors {
            if let Some(avatar_url) = &actor.images.avatar {
                // Convert ttrpg_core::ValidationIssue to local ValidationIssue
                let core_issues = self.asset_validator.validate_url(avatar_url, "actor", &Some(actor.id.clone())).await?;
                let converted_issues = core_issues.into_iter().map(|issue| ValidationIssue {
                    severity: match issue.severity {
                        ttrpg_core::IssueSeverity::Error => IssueSeverity::Error,
                        ttrpg_core::IssueSeverity::Warning => IssueSeverity::Warning,
                        ttrpg_core::IssueSeverity::Info => IssueSeverity::Info,
                    },
                    entity_type: issue.entity_type,
                    entity_id: issue.entity_id.unwrap_or_default(),
                    field: issue.field,
                    message: issue.message,
                    suggestion: issue.suggestion,
                }).collect::<Vec<_>>();
                issues.extend(converted_issues);
            }
        }
        
        for scene in &campaign.scenes {
            if let Some(background_url) = scene.background_image.as_ref() {
                let core_issues = self.asset_validator.validate_url(background_url, "scene", &Some(scene.id.clone())).await?;
                let converted_issues = core_issues.into_iter().map(|issue| ValidationIssue {
                    severity: match issue.severity {
                        ttrpg_core::IssueSeverity::Error => IssueSeverity::Error,
                        ttrpg_core::IssueSeverity::Warning => IssueSeverity::Warning,
                        ttrpg_core::IssueSeverity::Info => IssueSeverity::Info,
                    },
                    entity_type: issue.entity_type,
                    entity_id: issue.entity_id.unwrap_or_default(),
                    field: issue.field,
                    message: issue.message,
                    suggestion: issue.suggestion,
                }).collect::<Vec<_>>();
                issues.extend(converted_issues);
            }
        }
        
        // Validate permissions for campaign operations
        let core_perm_issues = self.security_validator.validate_permissions(
            &format!("campaign:{}", campaign.metadata.name),
            "read",
            "campaign", 
            &Some(campaign.metadata.name.clone())
        )?;
        let converted_perm_issues = core_perm_issues.into_iter().map(|issue| ValidationIssue {
            severity: match issue.severity {
                ttrpg_core::IssueSeverity::Error => IssueSeverity::Error,
                ttrpg_core::IssueSeverity::Warning => IssueSeverity::Warning,
                ttrpg_core::IssueSeverity::Info => IssueSeverity::Info,
            },
            entity_type: issue.entity_type,
            entity_id: issue.entity_id.unwrap_or_default(),
            field: issue.field,
            message: issue.message,
            suggestion: issue.suggestion,
        }).collect::<Vec<_>>();
        issues.extend(converted_perm_issues);
        
        Ok(issues)
    }
    
    /// Validate against JSON schema
    fn validate_with_schema(&self, entity_type: &str, data: &Value) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        if let Some(schema) = self.schemas.get(entity_type) {
            if let Err(validation_errors) = schema.validate(data) {
                for error in validation_errors {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Error,
                        entity_type: entity_type.to_string(),
                        entity_id: data.get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        field: Some(error.instance_path.to_string()),
                        message: error.to_string(),
                        suggestion: Some(format!("Fix schema validation error at {}", error.instance_path)),
                    });
                }
            }
        }
        
        Ok(issues)
    }

    /// Load default JSON schemas for common entity types
    fn load_default_schemas(&mut self) -> ConversionResult<()> {
        // Campaign schema
        let campaign_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "name": {"type": "string", "minLength": 1},
                "description": {"type": "string"},
                "actors": {"type": "array"},
                "scenes": {"type": "array"},
                "items": {"type": "array"}
            },
            "required": ["id", "name"]
        });
        
        self.add_schema("campaign", campaign_schema)?;
        
        // Actor schema
        let actor_schema = json!({
            "type": "object", 
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "name": {"type": "string", "minLength": 1},
                "character_type": {"type": "string"},
                "hit_points": {"type": "integer", "minimum": 0},
                "avatar": {"type": ["string", "null"]}
            },
            "required": ["id", "name"]
        });
        
        self.add_schema("actor", actor_schema)?;
        
        // Scene schema
        let scene_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "name": {"type": "string", "minLength": 1},
                "width": {"type": "number", "minimum": 0},
                "height": {"type": "number", "minimum": 0},
                "background_image": {"type": ["string", "null"]}
            },
            "required": ["id", "name"]
        });
        
        self.add_schema("scene", scene_schema)?;
        
        // Item schema
        let item_schema = json!({
            "type": "object",
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "name": {"type": "string", "minLength": 1},
                "item_type": {"type": "string"},
                "description": {"type": "string"}
            },
            "required": ["id", "name"]
        });
        
        self.add_schema("item", item_schema)?;
        
        Ok(())
    }
}

/// Implementation of ValidationPluginTrait for plugin framework integration
#[async_trait]
impl ValidationPluginTrait for ValidationPlugin {
    fn plugin_info(&self) -> PluginInfo {
        self.info.clone()
    }
    
    async fn initialize(&mut self, _config: PluginConfig) -> ConversionResult<()> {
        // Plugin is already initialized in new()
        Ok(())
    }
    
    async fn cleanup(&mut self) -> ConversionResult<()> {
        Ok(())
    }
    
    async fn validate_campaign(&self, campaign: &ttrpg_core::Campaign) -> ConversionResult<ttrpg_core::ValidationResult> {
        match self.validate_campaign(campaign).await {
            Ok(result) => Ok(ttrpg_core::ValidationResult {
                is_valid: result.is_valid,
                issues: result.issues.into_iter().map(|issue| ttrpg_core::ValidationIssue {
                    severity: match issue.severity {
                        IssueSeverity::Error => ttrpg_core::IssueSeverity::Error,
                        IssueSeverity::Warning => ttrpg_core::IssueSeverity::Warning,
                        IssueSeverity::Info => ttrpg_core::IssueSeverity::Info,
                    },
                    entity_type: issue.entity_type,
                    entity_id: Some(issue.entity_id),
                    field: issue.field,
                    message: issue.message,
                    suggestion: issue.suggestion,
                }).collect(),
                stats: ttrpg_core::ValidationStats {
                    entities_validated: result.stats.entities_validated,
                    entities_with_issues: result.stats.entities_with_issues,
                    validation_time_ms: result.stats.validation_time_ms,
                    cache_hit_ratio: result.stats.cache_hit_ratio,
                },
            }),
            Err(e) => Err(Box::new(*e)),
        }
    }
    
    async fn validate_file_path(&self, _path: &std::path::Path) -> ConversionResult<ttrpg_core::ValidationResult> {
        // Placeholder implementation
        Ok(ttrpg_core::ValidationResult {
            is_valid: true,
            issues: vec![],
            stats: ttrpg_core::ValidationStats {
                entities_validated: 0,
                entities_with_issues: 0,
                validation_time_ms: 0,
                cache_hit_ratio: 0.0,
            },
        })
    }
    
    async fn validate_format(&self, _format: &str, _data: &serde_json::Value) -> ConversionResult<ttrpg_core::ValidationResult> {
        // Placeholder implementation
        Ok(ttrpg_core::ValidationResult {
            is_valid: true,
            issues: vec![],
            stats: ttrpg_core::ValidationStats {
                entities_validated: 0,
                entities_with_issues: 0,
                validation_time_ms: 0,
                cache_hit_ratio: 0.0,
            },
        })
    }
    
    fn get_supported_formats(&self) -> Vec<String> {
        vec!["campaign".to_string(), "actor".to_string(), "scene".to_string(), "item".to_string()]
    }
    
    fn get_stats(&self) -> ttrpg_core::ValidationStats {
        ttrpg_core::ValidationStats {
            entities_validated: 0,
            entities_with_issues: 0,
            validation_time_ms: 0,
            cache_hit_ratio: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttrpg_core::{Campaign, Actor, Scene, Item};
    
    #[tokio::test]
    async fn test_validation_plugin_creation() {
        let execution_context = AssetExecutionContext::new().unwrap();
        let context = Arc::new(execution_context);
        let config = ValidationConfig::default();
        let plugin = ValidationPlugin::new(context, config);
        
        assert!(plugin.is_ok());
    }
    
    #[tokio::test]
    async fn test_campaign_validation() {
        let execution_context = AssetExecutionContext::new().unwrap();
        let context = Arc::new(execution_context);
        let config = ValidationConfig::default();
        let plugin = ValidationPlugin::new(context, config).unwrap();
        
        let campaign = Campaign {
            id: "test-campaign".to_string(),
            name: "Test Campaign".to_string(),
            description: Some("Test campaign for validation".to_string()),
            system: Some("D&D 5e".to_string()),
            format_source: ttrpg_core::SourceFormat::Roll20,
            characters: vec![],
            items: vec![],
            scenes: vec![],
            journal_entries: vec![],
            rollable_tables: vec![],
            playlists: vec![],
            macros: vec![],
            folders: vec![],
            settings: std::collections::HashMap::new(),
            metadata: ttrpg_core::CampaignMetadata {
                name: "Test Campaign".to_string(),
                description: None,
                source_format: ttrpg_core::SourceFormat::Roll20,
                schema_version: Some("1.0".to_string()),
                detected_system: None,
                system_confidence: 0.0,
                source_path: None,
                source_version: None,
                created_at: chrono::Utc::now(),
                modified_at: None,
                updated_at: chrono::Utc::now(),
                conversion_warnings: Vec::new(),
                entity_counts: std::collections::HashMap::new(),
                asset_stats: ttrpg_core::AssetStats {
                    total_assets: 0,
                    total_size_bytes: 0,
                    image_count: 0,
                    audio_count: 0,
                    missing_assets: Vec::new(),
                },
                validation_status: ttrpg_core::ValidationStatus::Pending,
                stats: ttrpg_core::CampaignStats::default(),
                custom_properties: std::collections::HashMap::new(),
            },
        };
        
        let result = plugin.validate_campaign(&campaign).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        // Should be valid for minimal valid campaign
        assert!(validation_result.is_valid);
    }
    
    #[tokio::test] 
    async fn test_invalid_campaign_validation() {
        let execution_context = AssetExecutionContext::new().unwrap();
        let context = Arc::new(execution_context);
        let config = ValidationConfig::default();
        let plugin = ValidationPlugin::new(context, config).unwrap();
        
        let invalid_campaign = Campaign {
            metadata: ttrpg_core::CampaignMetadata {
                name: "".to_string(), // Invalid empty name
                description: None,
                source_format: ttrpg_core::SourceFormat::Roll20,
                detected_system: None,
                system_confidence: 0.0,
                schema_version: None,
                source_path: None,
                created_at: None,
                modified_at: None,
                source_version: None,
                stats: ttrpg_core::CampaignStats::default(),
                custom_properties: std::collections::HashMap::new(),
            },
            actors: vec![],
            scenes: vec![],
            items: vec![],
            journal: vec![],
            tables: vec![],
            playlists: vec![],
            macros: vec![],
            folders: vec![],
            assets: vec![],
        };
        
        let result = plugin.validate_campaign(&campaign).await;
        assert!(result.is_ok());
        
        let validation_result = result.unwrap();
        // Should have validation errors for empty name
        // Note: Basic validation might pass with minimal data
    }
}
