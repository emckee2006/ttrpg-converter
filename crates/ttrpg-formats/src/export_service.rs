//! Export service implementation for campaign conversion
//!
//! This module provides concrete implementations of the ExportService trait,
//! handling export to various target formats including JSON, YAML, and Foundry VTT.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json;
use tokio::fs;

use ttrpg_core::error::ConversionResult;
use ttrpg_core::services::{
    ExportOptions, ExportPreview, ExportResult, ExportService, ExportStats, LoggingService,
    ValidationResult,
};
use ttrpg_core::types::{Campaign, TargetFormat};

/// Comprehensive export service implementation
///
/// Provides campaign export capabilities for multiple target formats with template system support.
/// Supports JSON export, Foundry VTT formats, and extensible format system.
pub struct RustExportService {
    /// Optional logging service for operation tracking
    logger: Option<Arc<dyn LoggingService>>,

    /// Template cache for format-specific rendering
    #[allow(dead_code)]
    template_cache: HashMap<String, String>,

    /// Export statistics
    stats: ExportStats,
}

impl RustExportService {
    /// Create a new export service
    pub fn new() -> Self {
        Self { logger: None, template_cache: HashMap::new(), stats: ExportStats::default() }
    }

    /// Create export service with logging
    pub fn with_logging(logger: Arc<dyn LoggingService>) -> Self {
        Self { logger: Some(logger), template_cache: HashMap::new(), stats: ExportStats::default() }
    }

    /// Export campaign to JSON format
    async fn export_to_json(
        &self,
        campaign: &Campaign,
        output_path: &Path,
        options: &ExportOptions,
    ) -> ConversionResult<ExportResult> {
        let start_time = std::time::Instant::now();

        if let Some(logger) = &self.logger {
            logger.info("Starting JSON export", Some("export"));
        }

        // Serialize campaign to JSON
        let json_content = if options.pretty_json {
            serde_json::to_string_pretty(campaign)
        } else {
            serde_json::to_string(campaign)
        }
        .map_err(|e| {
            ttrpg_core::error::ConversionError::from_io(
                std::io::Error::other(format!("JSON serialization failed: {e}")),
                "json export",
            )
        })?;

        // Write JSON file
        fs::write(output_path, &json_content).await.map_err(|e| {
            ttrpg_core::error::ConversionError::from_io(e, "json export file write")
        })?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Create export result
        let created_files = vec![output_path.to_path_buf()];

        // Handle asset processing if requested
        if options.include_assets {
            if let Some(logger) = &self.logger {
                logger.debug("Asset export not yet implemented for JSON format", None);
            }
        }

        let stats = ExportStats {
            entities_exported: self.count_campaign_entities(campaign),
            assets_processed: 0, // TODO: Implement asset processing
            processing_time_ms: processing_time,
            output_size_bytes: json_content.len() as u64,
            template_render_time_ms: 0,
            validation_time_ms: 0,
        };

        Ok(ExportResult {
            success: true,
            target_format: TargetFormat::JsonExport,
            output_path: output_path.to_path_buf(),
            created_files,
            stats,
            warnings: Vec::new(),
            error_message: None,
        })
    }

    /// Export campaign to Foundry VTT format
    async fn export_to_foundry(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
        output_path: &Path,
        options: &ExportOptions,
    ) -> ConversionResult<ExportResult> {
        let start_time = std::time::Instant::now();

        if let Some(logger) = &self.logger {
            logger.info(&format!("Starting Foundry VTT export: {target_format}"), Some("export"));
        }

        // Create Foundry-specific data structure
        let foundry_data = self.convert_to_foundry_format(campaign, target_format)?;

        // Serialize to JSON (Foundry uses JSON internally)
        let json_content = if options.pretty_json {
            serde_json::to_string_pretty(&foundry_data)
        } else {
            serde_json::to_string(&foundry_data)
        }
        .map_err(|e| {
            ttrpg_core::error::ConversionError::from_io(
                std::io::Error::other(format!("Foundry serialization failed: {e}")),
                "foundry export",
            )
        })?;

        // Write export file
        fs::write(output_path, &json_content).await.map_err(|e| {
            ttrpg_core::error::ConversionError::from_io(e, "foundry export file write")
        })?;

        let processing_time = start_time.elapsed().as_millis() as u64;

        let stats = ExportStats {
            entities_exported: self.count_campaign_entities(campaign),
            assets_processed: 0, // TODO: Implement asset processing
            processing_time_ms: processing_time,
            output_size_bytes: json_content.len() as u64,
            template_render_time_ms: 0,
            validation_time_ms: 0,
        };

        Ok(ExportResult {
            success: true,
            target_format,
            output_path: output_path.to_path_buf(),
            created_files: vec![output_path.to_path_buf()],
            stats,
            warnings: Vec::new(),
            error_message: None,
        })
    }

    /// Convert campaign to Foundry VTT format structure
    fn convert_to_foundry_format(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
    ) -> ConversionResult<serde_json::Value> {
        // Create Foundry-specific world structure
        let mut foundry_world = serde_json::json!({
            "name": campaign.metadata.name,
            "title": campaign.metadata.name,
            "description": "", // Description not available in CampaignMetadata
            "version": match target_format {
                TargetFormat::FoundryV10 => "10",
                TargetFormat::FoundryV11 => "11",
                TargetFormat::FoundryV12 => "12",
                _ => "12", // Default to latest
            },
            "system": "dnd5e", // Default system, could be configurable
            "coreVersion": match target_format {
                TargetFormat::FoundryV10 => "10.291",
                TargetFormat::FoundryV11 => "11.315",
                TargetFormat::FoundryV12 => "12.331",
                _ => "12.331",
            }
        });

        // Convert actors
        let actors: Vec<serde_json::Value> = campaign
            .actors
            .iter()
            .map(|actor| {
                serde_json::json!({
                    "name": actor.name,
                    "type": "character", // Simplified for now
                    "system": {
                        "attributes": actor.attributes,
                    },
                    "img": actor.images.avatar.clone(),
                })
            })
            .collect();

        foundry_world["actors"] = serde_json::Value::Array(actors);

        // Convert scenes
        let scenes: Vec<serde_json::Value> = campaign
            .scenes
            .iter()
            .map(|scene| {
                serde_json::json!({
                    "name": scene.name,
                    "background": {
                        "src": scene.background_image.clone()
                    },
                    "width": scene.dimensions.width,
                    "height": scene.dimensions.height,
                    "tokens": scene.tokens.iter().map(|token| {
                        serde_json::json!({
                            "name": token.id, // Using token.id as name since name field doesn't exist
                            "x": token.position.x,
                            "y": token.position.y,
                            "texture": {
                                "src": token.image.clone()
                            }
                        })
                    }).collect::<Vec<_>>()
                })
            })
            .collect();

        foundry_world["scenes"] = serde_json::Value::Array(scenes);

        Ok(foundry_world)
    }

    /// Count total entities in campaign for statistics
    fn count_campaign_entities(&self, campaign: &Campaign) -> usize {
        (campaign.actors.len()
            + campaign.scenes.len()
            + campaign.items.len()
            + campaign.journal.len()) // rollable_tables field doesn't exist in Campaign struct
            + campaign.playlists.len()
            + campaign.macros.len()
    }

    /// Validate export format compatibility
    fn validate_export_compatibility(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
    ) -> ConversionResult<ValidationResult> {
        let start_time = std::time::Instant::now();
        let mut result = ValidationResult::success();

        // Check format-specific compatibility
        match target_format {
            TargetFormat::JsonExport => {
                // JSON export supports everything
            }
            TargetFormat::FoundryV10 | TargetFormat::FoundryV11 | TargetFormat::FoundryV12 => {
                // Check for Foundry-specific limitations
                if campaign.actors.len() > 1000 {
                    result.add_warning(
                        ttrpg_core::services::ValidationIssue::warning(
                            "campaign",
                            format!(
                                "Large number of actors ({}) may impact Foundry performance",
                                campaign.actors.len()
                            ),
                        )
                        .with_suggestion("Consider splitting into multiple worlds"),
                    );
                }
            }
        }

        result.stats.validation_time_ms = start_time.elapsed().as_millis() as u64;
        Ok(result)
    }

    /// Load available templates for a target format
    fn load_templates(&self, target_format: TargetFormat) -> Vec<String> {
        match target_format {
            TargetFormat::JsonExport => vec!["default".to_string(), "compact".to_string()],
            TargetFormat::FoundryV10 => vec!["dnd5e".to_string(), "pf2e".to_string()],
            TargetFormat::FoundryV11 => vec!["dnd5e".to_string(), "pf2e".to_string()],
            TargetFormat::FoundryV12 => vec!["dnd5e".to_string(), "pf2e".to_string()],
        }
    }
}

impl Default for RustExportService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ExportService for RustExportService {
    async fn export_campaign(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
        output_path: &Path,
    ) -> ConversionResult<ExportResult> {
        let options = ExportOptions::default();
        match target_format {
            TargetFormat::JsonExport => self.export_to_json(campaign, output_path, &options).await,
            TargetFormat::FoundryV10 | TargetFormat::FoundryV11 | TargetFormat::FoundryV12 => {
                self.export_to_foundry(campaign, target_format, output_path, &options)
                    .await
            }
        }
    }

    async fn export_with_template(
        &self,
        campaign: &Campaign,
        _template_path: &Path,
        target_format: TargetFormat,
        output_path: &Path,
        options: &ExportOptions,
    ) -> ConversionResult<ExportResult> {
        // For now, delegate to standard export
        // TODO: Implement template system
        match target_format {
            TargetFormat::JsonExport => self.export_to_json(campaign, output_path, options).await,
            TargetFormat::FoundryV10 | TargetFormat::FoundryV11 | TargetFormat::FoundryV12 => {
                self.export_to_foundry(campaign, target_format, output_path, options)
                    .await
            }
        }
    }

    fn validate_export_format(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
    ) -> ConversionResult<ValidationResult> {
        self.validate_export_compatibility(campaign, target_format)
    }

    fn get_available_templates(&self, target_format: TargetFormat) -> Vec<String> {
        self.load_templates(target_format)
    }

    fn preview_export(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
    ) -> ConversionResult<ExportPreview> {
        let estimated_size = match target_format {
            TargetFormat::JsonExport => {
                // Rough estimate for JSON size
                (campaign.actors.len() + campaign.scenes.len()) * 1024
            }
            _ => {
                // Conservative estimate for other formats
                (campaign.actors.len() + campaign.scenes.len()) * 2048
            }
        };

        Ok(ExportPreview {
            target_format,
            estimated_size_bytes: estimated_size as u64,
            files_to_create: vec![PathBuf::from("campaign.json")],
            assets_to_process: campaign.assets.references.len(), // Using references field instead of assets
            compatibility_warnings: Vec::new(),
            template_info: Some("Using default template".to_string()),
        })
    }

    fn get_export_stats(&self) -> ExportStats {
        self.stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ttrpg_core::types::{Actor, ActorType, CampaignMetadata, Scene, SourceFormat};

    #[tokio::test]
    async fn test_export_service_creation() {
        let service = RustExportService::new();
        assert!(service.logger.is_none());
    }

    #[tokio::test]
    async fn test_json_export() {
        let service = RustExportService::new();
        let campaign = create_test_campaign();

        let temp_dir = std::env::temp_dir();
        let output_path = temp_dir.join("test_export.json");

        let result = service
            .export_campaign(&campaign, TargetFormat::JsonExport, &output_path)
            .await
            .unwrap();

        assert!(result.success);
        assert_eq!(result.target_format, TargetFormat::JsonExport);
        assert!(output_path.exists());

        // Cleanup
        let _ = std::fs::remove_file(&output_path);
    }

    #[test]
    fn test_validation() {
        let service = RustExportService::new();
        let campaign = create_test_campaign();

        let result = service
            .validate_export_format(&campaign, TargetFormat::JsonExport)
            .unwrap();

        assert!(result.is_valid);
    }

    #[test]
    fn test_preview_export() {
        let service = RustExportService::new();
        let campaign = create_test_campaign();

        let preview = service
            .preview_export(&campaign, TargetFormat::JsonExport)
            .unwrap();

        assert_eq!(preview.target_format, TargetFormat::JsonExport);
        assert!(preview.estimated_size_bytes > 0);
    }

    fn create_test_campaign() -> Campaign {
        Campaign {
            metadata: CampaignMetadata {
                name: "Test Campaign".to_string(),
                source_format: SourceFormat::Roll20,
                schema_version: Some("1.0.0".to_string()),
                created_at: Some(chrono::Utc::now()),
                stats: Default::default(),
                custom_properties: std::collections::HashMap::new(),
            },
            actors: vec![Actor {
                id: "test_actor".to_string(),
                name: "Test Actor".to_string(),
                actor_type: ActorType::Pc, // Using Pc instead of Player
                images: ttrpg_core::types::ActorImages::default(),
                attributes: std::collections::HashMap::new(),
                items: Vec::new(),
                spells: Vec::new(),
                features: Vec::new(),
                biography: String::new(),
                notes: String::new(),
                permissions: Default::default(),
                source_data: std::collections::HashMap::new(),
            }],
            scenes: vec![Scene {
                id: "test_scene".to_string(),
                name: "Test Scene".to_string(),
                background_image: None,
                dimensions: ttrpg_core::types::SceneDimensions {
                    width: 4000,
                    height: 4000,
                    scale: 1.0,
                    grid_size: 100.0,
                },
                grid: ttrpg_core::types::GridConfig {
                    grid_type: ttrpg_core::types::GridType::Square,
                    size: 100.0,
                    color: "#000000".to_string(),
                    opacity: 0.5,
                },
                lighting: ttrpg_core::types::LightingConfig {
                    global_light: 0.5,
                    darkness: 0.0,
                    fog_of_war: false,
                },
                tokens: Vec::new(),
                walls: Vec::new(),
                audio: None,
                notes: String::new(),
                source_data: std::collections::HashMap::new(),
                permissions: Default::default(),
            }],
            items: Vec::new(),
            journal: Vec::new(),
            tables: Vec::new(),
            playlists: Vec::new(),
            macros: Vec::new(),
            folders: Vec::new(),
            assets: Default::default(),
        }
    }
}
