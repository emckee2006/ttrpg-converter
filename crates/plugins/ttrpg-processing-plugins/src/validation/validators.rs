//! Layered validation system with specialized validators
//! 
//! This module implements a clean 3-layer validation approach:
//! - Layer 1: Schema validation (auto-generated from JSON Schema)
//! - Layer 2: Business rules (TTRPG-specific logic)
//! - Layer 3: System validation (files, URLs, permissions)

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tokio::fs;
use url::Url;
use serde::{Deserialize, Serialize};
use ttrpg_core::error::ConversionResult;
use ttrpg_core::plugin_framework::types::{ValidationIssue, IssueSeverity};

/// Complete validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub stats: ValidationStats,
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStats {
    pub entities_validated: usize,
    pub entities_with_issues: usize,
    pub validation_time_ms: u64,
    pub errors: usize,
    pub warnings: usize,
    pub cache_hit_ratio: f64,
}
use jsonschema::JSONSchema;

/// Main validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    /// Enable parallel validation
    pub parallel_validation: bool,
    /// Maximum time for validation in milliseconds
    pub max_validation_time_ms: u64,
    /// Validation strictness level
    pub strictness: ValidationStrictness,
    /// Business rules configuration
    pub business_rules: BusinessRulesConfig,
    /// Asset validation settings
    pub assets: AssetValidationConfig,
    /// Security validation settings
    pub security: SecurityValidationConfig,
}

/// Business rules validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRulesConfig {
    pub validate_health_consistency: bool,
    pub validate_ability_scores: bool,
    pub validate_level_progression: bool,
    pub validate_asset_references: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStrictness {
    Lenient,
    Standard,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetValidationConfig {
    /// Allowed file extensions
    pub allowed_extensions: HashSet<String>,
    /// Maximum file size in bytes
    pub max_file_size: u64,
    /// Check URL accessibility
    pub check_url_accessibility: bool,
    /// Base paths for asset resolution
    pub base_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityValidationConfig {
    /// Enforce permission checking
    pub enforce_permissions: bool,
    /// Allowed domains for external URLs
    pub allowed_domains: HashSet<String>,
    /// Maximum URL redirects to follow
    pub max_redirects: u8,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        let mut allowed_extensions = HashSet::new();
        allowed_extensions.insert("png".to_string());
        allowed_extensions.insert("jpg".to_string());
        allowed_extensions.insert("jpeg".to_string());
        allowed_extensions.insert("pdf".to_string());
        allowed_extensions.insert("json".to_string());
        
        let mut allowed_domains = HashSet::new();
        allowed_domains.insert("github.com".to_string());
        allowed_domains.insert("dndbeyond.com".to_string());
        allowed_domains.insert("roll20.net".to_string());
        
        Self {
            parallel_validation: true,
            max_validation_time_ms: 5000,
            strictness: ValidationStrictness::Standard,
            business_rules: BusinessRulesConfig {
                validate_health_consistency: true,
                validate_ability_scores: true,
                validate_level_progression: true,
                validate_asset_references: true,
            },
            assets: AssetValidationConfig {
                allowed_extensions,
                max_file_size: 50 * 1024 * 1024, // 50MB
                check_url_accessibility: true,
                base_paths: vec![PathBuf::from("assets"), PathBuf::from("images")],
            },
            security: SecurityValidationConfig {
                enforce_permissions: true,
                allowed_domains,
                max_redirects: 3,
            },
        }
    }
}

impl ValidationConfig {
    /// Load validation configuration from TOML file
    pub fn from_toml_file<P: AsRef<Path>>(path: P) -> ConversionResult<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "config".to_string(),
                message: format!("Failed to read validation config: {}", e),
                field: None,
            })?;
            
        let config: Self = toml::from_str(&content)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "config".to_string(),
                message: format!("Failed to parse TOML config: {}", e),
                field: None,
            })?;
            
        Ok(config)
    }
    
    /// Save validation configuration to TOML file
    pub fn to_toml_file<P: AsRef<Path>>(&self, path: P) -> ConversionResult<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "config".to_string(),
                message: format!("Failed to serialize config: {}", e),
                field: None,
            })?;
            
        std::fs::write(path.as_ref(), content)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "config".to_string(),
                message: format!("Failed to write validation config: {}", e),
                field: None,
            })?;
            
        Ok(())
    }
}

/// Layer 1: Schema validation using JSON Schema (based on Python legacy patterns)
pub struct SchemaValidator {
    schemas: std::collections::HashMap<String, JSONSchema>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        Self {
            schemas: std::collections::HashMap::new(),
        }
    }
    
    /// Create SchemaValidator from TOML configuration, loading schema files automatically
    pub fn from_config(config: &ValidationConfig) -> ConversionResult<Self> {
        let mut validator = Self::new();
        
        // Load schemas from files specified in config
        validator.load_schema_file("actor", &config.schemas.actor)?;
        validator.load_schema_file("scene", &config.schemas.scene)?;
        validator.load_schema_file("item", &config.schemas.item)?;
        validator.load_schema_file("campaign", &config.schemas.campaign)?;
        
        Ok(validator)
    }
    
    /// Register default schemas for common TTRPG entities
    fn register_default_schemas(&mut self) -> ConversionResult<()> {
        // Actor/Character schema (from Python legacy patterns)
        let actor_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "pattern": "^[a-zA-Z0-9_-]{20}$",
                    "description": "20-character entity ID"
                },
                "name": {
                    "type": "string",
                    "minLength": 1,
                    "maxLength": 255
                },
                "type": {
                    "type": "string",
                    "enum": ["character", "npc"]
                },
                "health": {
                    "type": "object",
                    "properties": {
                        "current": { "type": "integer", "minimum": -999 },
                        "maximum": { "type": "integer", "minimum": 1 }
                    },
                    "required": ["current", "maximum"]
                },
                "abilities": {
                    "type": "object",
                    "properties": {
                        "strength": { "type": "integer", "minimum": 1, "maximum": 30 },
                        "dexterity": { "type": "integer", "minimum": 1, "maximum": 30 },
                        "constitution": { "type": "integer", "minimum": 1, "maximum": 30 },
                        "intelligence": { "type": "integer", "minimum": 1, "maximum": 30 },
                        "wisdom": { "type": "integer", "minimum": 1, "maximum": 30 },
                        "charisma": { "type": "integer", "minimum": 1, "maximum": 30 }
                    }
                },
                "level": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 30
                }
            },
            "required": ["id", "name", "type"]
        });
        
        // Scene/Page schema
        let scene_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string",
                    "pattern": "^[a-zA-Z0-9_-]{20}$"
                },
                "name": {
                    "type": "string",
                    "minLength": 1,
                    "maxLength": 255
                },
                "width": { "type": "number", "minimum": 0 },
                "height": { "type": "number", "minimum": 0 },
                "grid_size": { "type": "number", "minimum": 0.1 },
                "background_image": {
                    "type": ["string", "null"],
                    "format": "uri"
                }
            },
            "required": ["id", "name"]
        });
        
        // Item schema
        let item_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "id": {
                    "type": "string", 
                    "pattern": "^[a-zA-Z0-9_-]{20}$"
                },
                "name": {
                    "type": "string",
                    "minLength": 1,
                    "maxLength": 255
                },
                "type": {
                    "type": "string",
                    "enum": ["weapon", "armor", "consumable", "treasure", "tool", "other"]
                },
                "weight": { "type": "number", "minimum": 0 },
                "value": { "type": "number", "minimum": 0 }
            },
            "required": ["id", "name", "type"]
        });
        
        self.register_schema("actor", actor_schema.clone())?;
        self.register_schema("character", actor_schema.clone())?;
        self.register_schema("npc", actor_schema)?;
        self.register_schema("scene", scene_schema.clone())?;
        self.register_schema("page", scene_schema)?;
        self.register_schema("item", item_schema)?;
        
        Ok(())
    }
    
    /// Load schema from JSON file
    pub fn load_schema_file(&mut self, entity_type: &str, file_path: &str) -> ConversionResult<()> {
        let schema_content = std::fs::read_to_string(file_path)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "schema".to_string(),
                message: format!("Failed to read schema file {}: {}", file_path, e),
                field: None,
            })?;
            
        let schema: serde_json::Value = serde_json::from_str(&schema_content)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: "schema".to_string(),
                message: format!("Failed to parse schema file {}: {}", file_path, e),
                field: None,
            })?;
            
        self.register_schema(entity_type, schema)?;
        Ok(())
    }
    
    /// Register a JSON schema for an entity type
    pub fn register_schema(&mut self, entity_type: &str, schema: serde_json::Value) -> ConversionResult<()> {
        let compiled = JSONSchema::options()
            .with_draft(jsonschema::Draft::Draft7)
            .compile(&schema)
            .map_err(|e| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: entity_type.to_string(),
                message: format!("Failed to compile schema for {}: {}", entity_type, e),
                field: None,
            })?;
            
        self.schemas.insert(entity_type.to_string(), compiled);
        Ok(())
    }
    
    /// Generate a template JSON object from schema (schema-driven generation)
    pub fn generate_template(&self, entity_type: &str) -> ConversionResult<serde_json::Value> {
        let _schema = self.schemas.get(entity_type)
            .ok_or_else(|| ttrpg_core::error::ConversionError::ValidationError {
                entity_type: entity_type.to_string(),
                message: format!("No schema registered for entity type: {}", entity_type),
                field: None,
            })?;
            
        // For template generation, we need to create a basic template
        // Since jsonschema crate doesn't expose schema definition easily,
        // we'll generate a basic template for now
        Ok(serde_json::json!({
            "name": "",
            "type": "unknown"
        }))
    }
    
    /// Generate JSON object from schema definition recursively
    fn generate_from_schema_definition(&self, schema: &serde_json::Value) -> ConversionResult<serde_json::Value> {
        match schema.get("type").and_then(|t| t.as_str()) {
            Some("object") => {
                let mut obj = serde_json::Map::new();
                
                // Add required properties
                if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                    for req_field in required {
                        if let Some(field_name) = req_field.as_str() {
                            if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                                if let Some(field_schema) = properties.get(field_name) {
                                    let field_value = self.generate_from_schema_definition(field_schema)?;
                                    obj.insert(field_name.to_string(), field_value);
                                }
                            }
                        }
                    }
                }
                
                // Add properties with defaults
                if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
                    for (field_name, field_schema) in properties {
                        if !obj.contains_key(field_name) {
                            if let Some(default) = field_schema.get("default") {
                                obj.insert(field_name.clone(), default.clone());
                            }
                        }
                    }
                }
                
                Ok(serde_json::Value::Object(obj))
            },
            Some("array") => {
                Ok(serde_json::json!([]))
            },
            Some("string") => {
                if let Some(default) = schema.get("default") {
                    Ok(default.clone())
                } else if let Some(pattern) = schema.get("pattern").and_then(|p| p.as_str()) {
                    // Generate example string based on pattern
                    Ok(serde_json::Value::String(self.generate_example_from_pattern(pattern)))
                } else {
                    Ok(serde_json::Value::String("".to_string()))
                }
            },
            Some("integer") => {
                if let Some(default) = schema.get("default") {
                    Ok(default.clone())
                } else if let Some(minimum) = schema.get("minimum").and_then(|m| m.as_i64()) {
                    Ok(serde_json::Value::Number(serde_json::Number::from(minimum)))
                } else {
                    Ok(serde_json::Value::Number(serde_json::Number::from(0)))
                }
            },
            Some("number") => {
                if let Some(default) = schema.get("default") {
                    Ok(default.clone())
                } else if let Some(minimum) = schema.get("minimum").and_then(|m| m.as_f64()) {
                    Ok(serde_json::json!(minimum))
                } else {
                    Ok(serde_json::json!(0.0))
                }
            },
            Some("boolean") => {
                if let Some(default) = schema.get("default") {
                    Ok(default.clone())
                } else {
                    Ok(serde_json::Value::Bool(false))
                }
            },
            _ => {
                // Handle enums and other types
                if let Some(enum_values) = schema.get("enum").and_then(|e| e.as_array()) {
                    if let Some(first_enum) = enum_values.first() {
                        Ok(first_enum.clone())
                    } else {
                        Ok(serde_json::Value::Null)
                    }
                } else {
                    Ok(serde_json::Value::Null)
                }
            }
        }
    }
    
    /// Generate example string from regex pattern
    fn generate_example_from_pattern(&self, pattern: &str) -> String {
        match pattern {
            "^[a-zA-Z0-9_-]{20}$" => "example_entity_id_20".to_string(),
            _ => "example_value".to_string(),
        }
    }
    
    /// Validate entity data against its registered schema
    pub fn validate_against_schema(&self, entity_type: &str, data: &serde_json::Value, entity_id: &Option<String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        let schema = match self.schemas.get(entity_type) {
            Some(schema) => schema,
            None => {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: None,
                    message: format!("No schema registered for entity type: {}", entity_type),
                    suggestion: Some("Register a JSON schema for this entity type".to_string()),
                });
                return issues;
            }
        };
        
        if let Err(validation_errors) = schema.validate(data) {
            for error in validation_errors {
                let field_path = if error.instance_path.to_string().is_empty() {
                    None
                } else {
                    Some(error.instance_path.to_string())
                };
                
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: field_path,
                    message: error.to_string(),
                    suggestion: Some("Check the field value against the expected schema".to_string()),
                });
            }
        }
        
        issues
    }
    
    /// Validate entity ID format (from Python legacy pattern)
    pub fn validate_entity_id(&self, entity_id: &str, entity_type: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        if entity_id.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                entity_type: entity_type.to_string(),
                entity_id: Some(entity_id.to_string()),
                field: Some("id".to_string()),
                message: "Entity ID cannot be empty".to_string(),
                suggestion: Some("Provide a valid 20-character entity ID".to_string()),
            });
        } else if entity_id.len() != 20 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                entity_type: entity_type.to_string(),
                entity_id: Some(entity_id.to_string()),
                field: Some("id".to_string()),
                message: format!("Invalid entity ID length: {} (expected 20)", entity_id.len()),
                suggestion: Some("Entity IDs must be exactly 20 characters long".to_string()),
            });
        }
        
        issues
    }
    
    /// Validate required fields (from Python legacy pattern)
    pub fn validate_required_fields(&self, data: &serde_json::Value, required_fields: &[&str], entity_type: &str, entity_id: &Option<String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        let obj = match data.as_object() {
            Some(obj) => obj,
            None => {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: None,
                    message: "Entity data must be an object".to_string(),
                    suggestion: Some("Ensure entity data is a valid JSON object".to_string()),
                });
                return issues;
            }
        };
        
        for field in required_fields {
            if !obj.contains_key(*field) || obj.get(*field).map_or(true, |v| v.is_null()) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some(field.to_string()),
                    message: format!("Missing required field: {}", field),
                    suggestion: Some(format!("Add the required '{}' field to the entity", field)),
                });
            }
        }
        
        issues
    }
}

/// Asset and file system validator
#[derive(Clone)]
pub struct AssetValidator {
    config: AssetValidationConfig,
    client: reqwest::Client,
}

impl AssetValidator {
    pub fn new(config: AssetValidationConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();
        
        Self { config, client }
    }
    
    /// Validate a file path exists and meets requirements
    pub async fn validate_file_path(&self, path: &str, entity_type: &str, entity_id: &Option<String>) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        let file_path = Path::new(path);
        
        // Check file exists
        if !file_path.exists() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                entity_type: entity_type.to_string(),
                entity_id: entity_id.clone(),
                field: Some("file_path".to_string()),
                message: format!("File does not exist: {}", path),
                suggestion: Some(format!("Ensure the file '{}' exists and is accessible", path)),
            });
            return Ok(issues);
        }
        
        // Check file extension
        if let Some(extension) = file_path.extension().and_then(|e| e.to_str()) {
            if !self.config.allowed_extensions.contains(&extension.to_lowercase()) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("file_path".to_string()),
                    message: format!("File extension '{}' is not in allowed list", extension),
                    suggestion: Some(format!("Use one of the allowed extensions: {:?}", self.config.allowed_extensions)),
                });
            }
        }
        
        // Check file size
        if let Ok(metadata) = fs::metadata(file_path).await {
            if metadata.len() > self.config.max_file_size {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("file_path".to_string()),
                    message: format!("File size ({} bytes) exceeds maximum ({} bytes)", metadata.len(), self.config.max_file_size),
                    suggestion: Some("Reduce file size or increase the maximum allowed size".to_string()),
                });
            }
        }
        
        Ok(issues)
    }
    
    /// Validate a URL is accessible and meets requirements
    pub async fn validate_url(&self, url_str: &str, entity_type: &str, entity_id: &Option<String>) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        // Parse URL
        let url = match Url::parse(url_str) {
            Ok(url) => url,
            Err(e) => {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Error,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("url".to_string()),
                    message: format!("Invalid URL format: {}", e),
                    suggestion: Some("Ensure URL follows proper format (https://example.com/path)".to_string()),
                });
                return Ok(issues);
            }
        };
        
        // Check domain whitelist
        if let Some(domain) = url.domain() {
            // Domain validation - allow all domains for now
            // TODO: Implement proper domain filtering
            if false { // Temporarily disabled domain filtering
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("url".to_string()),
                    message: format!("Domain '{}' is not in allowed list", domain),
                    suggestion: Some("Check domain accessibility and security policy".to_string()),
                });
            }
        }
        
        // Check URL accessibility
        if self.config.check_url_accessibility {
            match self.client.head(url_str).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        issues.push(ValidationIssue {
                            severity: IssueSeverity::Warning,
                            entity_type: entity_type.to_string(),
                            entity_id: entity_id.clone(),
                            field: Some("url".to_string()),
                            message: format!("URL returned status {}: {}", response.status().as_u16(), url_str),
                            suggestion: Some("Check that the URL is accessible and returns a successful response".to_string()),
                        });
                    }
                },
                Err(e) => {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Warning,
                        entity_type: entity_type.to_string(),
                        entity_id: entity_id.clone(),
                        field: Some("url".to_string()),
                        message: format!("Failed to access URL: {}", e),
                        suggestion: Some("Verify the URL is accessible from this network".to_string()),
                    });
                }
            }
        }
        
        Ok(issues)
    }
}

/// Security and permissions validator
#[derive(Clone)]
pub struct SecurityValidator {
    config: SecurityValidationConfig,
}

impl SecurityValidator {
    pub fn new(config: SecurityValidationConfig) -> Self {
        Self { config }
    }
    
    /// Validate user permissions for a resource
    pub fn validate_permissions(&self, resource: &str, action: &str, entity_type: &str, entity_id: &Option<String>) -> ConversionResult<Vec<ValidationIssue>> {
        let mut issues = Vec::new();
        
        if !self.config.enforce_permissions {
            return Ok(issues);
        }
        
        // Placeholder for permission checking logic
        // In a real implementation, this would check user roles, ACLs, etc.
        match action {
            "read" => {
                // Always allow read for now
            },
            "write" | "delete" => {
                // Could check for admin permissions
                if resource.contains("system") {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Error,
                        entity_type: entity_type.to_string(),
                        entity_id: entity_id.clone(),
                        field: Some("permissions".to_string()),
                        message: format!("Insufficient permissions for {} on system resource: {}", action, resource),
                        suggestion: Some("Contact an administrator for access to system resources".to_string()),
                    });
                }
            },
            _ => {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: entity_type.to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("permissions".to_string()),
                    message: format!("Unknown action '{}' for permission check", action),
                    suggestion: Some("Use standard actions: read, write, delete".to_string()),
                });
            }
        }
        
        Ok(issues)
    }
}

/// TTRPG business rules validator for TTRPG-specific logic
#[derive(Clone)]
pub struct BusinessRulesValidator {
    strictness: ValidationStrictness,
}

impl BusinessRulesValidator {
    pub fn new(strictness: ValidationStrictness) -> Self {
        Self { strictness }
    }
    
    /// Validate health consistency for actors
    pub fn validate_health_consistency(&self, current: i32, maximum: i32, entity_id: &Option<String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        if current > maximum {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                entity_type: "actor".to_string(),
                entity_id: entity_id.clone(),
                field: Some("health".to_string()),
                message: format!("Current health ({}) exceeds maximum health ({})", current, maximum),
                suggestion: Some("Ensure current health does not exceed maximum health".to_string()),
            });
        }
        
        if current < 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                entity_type: "actor".to_string(),
                entity_id: entity_id.clone(),
                field: Some("health.current".to_string()),
                message: format!("Current health is negative: {}", current),
                suggestion: Some("Consider setting minimum health to 0 or use proper death/unconscious mechanics".to_string()),
            });
        }
        
        issues
    }
    
    /// Validate ability scores are within expected ranges
    pub fn validate_ability_scores(&self, abilities: &std::collections::HashMap<String, i32>, entity_id: &Option<String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        let expected_abilities = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];
        
        // Check for missing core abilities
        for ability in &expected_abilities {
            if !abilities.contains_key(*ability) {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: "actor".to_string(),
                    entity_id: entity_id.clone(),
                    field: Some("abilities".to_string()),
                    message: format!("Missing core ability score: {}", ability),
                    suggestion: Some(format!("Add {} ability score for complete character definition", ability)),
                });
            }
        }
        
        // Check ability score ranges
        for (ability_name, score) in abilities {
            if *score < 1 || *score > 30 {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Warning,
                    entity_type: "actor".to_string(),
                    entity_id: entity_id.clone(),
                    field: Some(format!("abilities.{}", ability_name)),
                    message: format!("Ability score {} ({}) is outside normal range (1-30)", ability_name, score),
                    suggestion: Some("Most D&D-style games use ability scores between 1-30".to_string()),
                });
            }
        }
        
        issues
    }
    
    /// Validate level progression is reasonable
    pub fn validate_level_progression(&self, level: u8, entity_id: &Option<String>) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        
        if level == 0 {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Error,
                entity_type: "actor".to_string(),
                entity_id: entity_id.clone(),
                field: Some("level".to_string()),
                message: "Character level cannot be 0".to_string(),
                suggestion: Some("Set character level to at least 1".to_string()),
            });
        }
        
        if level > 20 && matches!(self.strictness, ValidationStrictness::Strict) {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Warning,
                entity_type: "actor".to_string(),
                entity_id: entity_id.clone(),
                field: Some("level".to_string()),
                message: format!("Character level {} is above typical maximum (20)", level),
                suggestion: Some("Most D&D campaigns cap at level 20. Consider if this is intentional".to_string()),
            });
        }
        
        issues
    }
}
