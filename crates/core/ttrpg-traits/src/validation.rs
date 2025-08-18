//! Validation plugin trait for data validation

use async_trait::async_trait;
use serde_json::Value as JsonValue;
use std::path::Path;
use ttrpg_core::prelude::*;
use ttrpg_types::Campaign;
use super::{PluginInfo, PluginConfig};

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub entity_type: String,
    pub entity_id: String,
    pub field: Option<String>,
    pub message: String,
    pub suggestion: Option<String>,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<ValidationIssue>,
    pub stats: ValidationStats,
}

/// Validation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ValidationStats {
    pub entities_validated: usize,
    pub entities_with_issues: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub validation_time_ms: u64,
}

/// Validation plugin trait
#[async_trait]
pub trait ValidationPlugin: Send + Sync {
    /// Get plugin metadata
    fn plugin_info(&self) -> PluginInfo;

    /// Initialize the plugin with configuration
    async fn initialize(&mut self, config: PluginConfig) -> ConversionResult<()>;

    /// Plugin cleanup
    async fn cleanup(&mut self) -> ConversionResult<()>;

    /// Validate a complete campaign
    async fn validate_campaign(&self, campaign: &Campaign) -> ConversionResult<ValidationResult>;

    /// Validate campaign file path and format
    async fn validate_file_path(&self, path: &Path) -> ConversionResult<ValidationResult>;

    /// Validate specific data format compatibility  
    async fn validate_format(
        &self,
        format: &str,
        data: &JsonValue,
    ) -> ConversionResult<ValidationResult>;

    /// Get validation capabilities and supported formats
    fn get_supported_formats(&self) -> Vec<String>;

    /// Get validation statistics
    fn get_stats(&self) -> ValidationStats;
}