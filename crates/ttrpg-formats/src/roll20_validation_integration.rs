//! Integration between Roll20Parser and enhanced validation engine
//!
//! This module provides seamless integration between the Roll20Parser from M2.1
//! and the enhanced Roll20ValidationEngine from M2.2, enabling end-to-end
//! validation of Roll20 conversion pipeline.
//!
//! # Key Features
//!
//! - **End-to-End Validation**: Complete validation from Roll20 JSON to ttrpg-core conversion
//! - **Conversion Context Building**: Automatic context creation from parser results
//! - **Asset Reference Extraction**: Automatic asset discovery during parsing
//! - **Performance Metrics**: Detailed performance tracking throughout conversion
//! - **Validation Reports**: Human-readable validation reports with actionable insights
//!
//! # Usage
//!
//! ```rust,no_run
//! use ttrpg_formats::roll20::{Roll20Parser, roll20_validation_integration::ValidatedRoll20Parser};
//! use ttrpg_core::validation::roll20_validation::Roll20ValidationEngine;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let parser = Roll20Parser::new();
//! let mut validator = ValidatedRoll20Parser::new(parser)?;
//! 
//! let validation_result = validator.parse_and_validate_file("campaign.json").await?;
//! if !validation_result.is_valid {
//!     println!("Validation issues found: {}", validation_result.issues.len());
//! }
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use ttrpg_core::validation::roll20_validation::{
    Roll20ValidationEngine, Roll20ConversionContext, Roll20ValidationResult,
    ConversionStats, AssetReference,
};
use ttrpg_core::services::{ValidationService, AssetService, LoggingService};
use ttrpg_core::error::{ConversionError, ConversionResult};
use ttrpg_core::types::Campaign;

use crate::roll20::{Roll20Parser, Roll20Campaign};

// ============================================================================
// VALIDATED ROLL20 PARSER
// ============================================================================

/// Enhanced Roll20 parser with integrated validation
pub struct ValidatedRoll20Parser {
    /// Core Roll20 parser
    parser: Roll20Parser,
    /// Enhanced validation engine
    validation_engine: Roll20ValidationEngine,
    /// Asset references discovered during parsing
    discovered_assets: Vec<AssetReference>,
    /// Conversion statistics
    conversion_stats: ConversionStats,
}

impl ValidatedRoll20Parser {
    /// Create a new validated Roll20 parser
    pub fn new(parser: Roll20Parser) -> ConversionResult<Self> {
        let validation_engine = Roll20ValidationEngine::new()?;
        
        Ok(Self {
            parser,
            validation_engine,
            discovered_assets: Vec::new(),
            conversion_stats: ConversionStats::default(),
        })
    }

    /// Create with services using builder pattern
    pub fn with_validation(mut self, service: Arc<dyn ValidationService>) -> Self {
        self.parser = self.parser.with_validation(service);
        self
    }

    pub fn with_assets(mut self, service: Arc<dyn AssetService>) -> Self {
        self.parser = self.parser.with_assets(service);
        self
    }

    pub fn with_logging(mut self, service: Arc<dyn LoggingService>) -> Self {
        self.parser = self.parser.with_logging(service);
        self
    }

    /// Parse and validate Roll20 campaign file with comprehensive validation
    pub async fn parse_and_validate_file<P: AsRef<Path>>(
        &mut self,
        file_path: P,
    ) -> ConversionResult<ValidationResult> {
        let start_time = std::time::Instant::now();
        
        // Reset stats for this conversion
        self.conversion_stats = ConversionStats::default();
        self.discovered_assets.clear();

        // Log start of validation process
        if let Some(logger) = &self.parser.logging_service {
            logger.info(
                &format!("Starting validated Roll20 conversion: {}", file_path.as_ref().display()),
                Some("validated_roll20_parser"),
            );
        }

        // 1. Read and parse Roll20 file
        let content = tokio::fs::read_to_string(&file_path)
            .await
            .map_err(|e| ConversionError::io_error(
                "ValidatedRoll20Parser", 
                format!("Failed to read Roll20 file: {e}")
            ))?;

        let roll20_data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| ConversionError::validation(
                "ValidatedRoll20Parser",
                format!("Invalid Roll20 JSON: {e}"),
            ))?;

        // 2. Parse Roll20 campaign structure
        let roll20_campaign: Roll20Campaign = serde_json::from_value(roll20_data.clone())
            .map_err(|e| ConversionError::validation(
                "ValidatedRoll20Parser",
                format!("Invalid Roll20 campaign structure: {e}"),
            ))?;

        // 3. Update conversion statistics
        self.update_conversion_stats(&roll20_campaign);

        // 4. Convert to ttrpg-core format with asset discovery
        let campaign = self.convert_with_asset_discovery(&roll20_campaign).await?;

        // 5. Create validation context
        let context = Roll20ConversionContext {
            roll20_data,
            converted_campaign: campaign.clone(),
            conversion_stats: self.conversion_stats.clone(),
            asset_references: self.discovered_assets.clone(),
        };

        // 6. Run comprehensive validation
        let validation_result = self.validation_engine.validate_roll20_conversion(&context)?;

        // 7. Log completion
        let duration = start_time.elapsed();
        if let Some(logger) = &self.parser.logging_service {
            logger.info(
                &format!(
                    "Validated Roll20 conversion completed in {:.2}s: {} issues found",
                    duration.as_secs_f64(),
                    validation_result.issues.len()
                ),
                Some("validated_roll20_parser"),
            );

            if !validation_result.is_valid {
                logger.warn(
                    &format!("Validation failed with {} errors", validation_result.stats.errors),
                    Some("validated_roll20_parser"),
                );
            }
        }

        Ok(ValidationResult {
            campaign,
            validation_result,
            conversion_duration: duration,
            file_path: file_path.as_ref().to_path_buf(),
        })
    }

    /// Parse Roll20 JSON string and validate
    pub async fn parse_and_validate_json(
        &mut self,
        json_content: &str,
    ) -> ConversionResult<ValidationResult> {
        let start_time = std::time::Instant::now();

        // Reset stats
        self.conversion_stats = ConversionStats::default();
        self.discovered_assets.clear();

        // Parse JSON
        let roll20_data: serde_json::Value = serde_json::from_str(json_content)
            .map_err(|e| ConversionError::validation(
                "ValidatedRoll20Parser",
                format!("Invalid Roll20 JSON: {e}"),
            ))?;

        let roll20_campaign: Roll20Campaign = serde_json::from_value(roll20_data.clone())
            .map_err(|e| ConversionError::validation(
                "ValidatedRoll20Parser", 
                format!("Invalid Roll20 campaign structure: {e}"),
            ))?;

        // Update stats and convert
        self.update_conversion_stats(&roll20_campaign);
        let campaign = self.convert_with_asset_discovery(&roll20_campaign).await?;

        // Validate
        let context = Roll20ConversionContext {
            roll20_data,
            converted_campaign: campaign.clone(),
            conversion_stats: self.conversion_stats.clone(),
            asset_references: self.discovered_assets.clone(),
        };

        let validation_result = self.validation_engine.validate_roll20_conversion(&context)?;

        Ok(ValidationResult {
            campaign,
            validation_result,
            conversion_duration: start_time.elapsed(),
            file_path: std::path::PathBuf::from("<json_string>"),
        })
    }

    /// Update conversion statistics from Roll20 campaign
    fn update_conversion_stats(&mut self, roll20_campaign: &Roll20Campaign) {
        self.conversion_stats.total_characters = roll20_campaign.characters.len();
        self.conversion_stats.total_pages = roll20_campaign.pages.len();
        self.conversion_stats.total_handouts = roll20_campaign.handouts.len();
    }

    /// Convert Roll20 campaign with asset discovery
    async fn convert_with_asset_discovery(
        &mut self,
        roll20_campaign: &Roll20Campaign,
    ) -> ConversionResult<Campaign> {
        // Start with basic conversion
        let mut campaign = Campaign {
            metadata: self.create_campaign_metadata(&roll20_campaign.campaign)?,
            actors: Vec::new(),
            scenes: Vec::new(),
            items: Vec::new(),
            journal: Vec::new(),
            tables: Vec::new(),
            playlists: Vec::new(),
            macros: Vec::new(),
            folders: Vec::new(),
            assets: Vec::new(),
        };

        // Convert characters to actors with asset discovery
        for character in &roll20_campaign.characters {
            match self.parser.convert_character_to_actor(character.clone()) {
                Ok(actor) => {
                    // Discover assets from actor
                    self.discover_actor_assets(&actor);
                    campaign.actors.push(actor);
                    self.conversion_stats.converted_actors += 1;
                }
                Err(e) => {
                    self.conversion_stats.failed_conversions += 1;
                    if let Some(logger) = &self.parser.logging_service {
                        logger.warn(
                            &format!("Failed to convert character '{}': {e}", character.name),
                            Some("validated_roll20_parser"),
                        );
                    }
                }
            }
        }

        // Convert pages to scenes with asset discovery
        for page in &roll20_campaign.pages {
            match self.parser.convert_page_to_scene(page.clone()) {
                Ok(scene) => {
                    // Discover assets from scene
                    self.discover_scene_assets(&scene);
                    campaign.scenes.push(scene);
                    self.conversion_stats.converted_scenes += 1;
                }
                Err(e) => {
                    self.conversion_stats.failed_conversions += 1;
                    if let Some(logger) = &self.parser.logging_service {
                        logger.warn(
                            &format!("Failed to convert page '{}': {e}", page.name),
                            Some("validated_roll20_parser"),
                        );
                    }
                }
            }
        }

        // Convert handouts to items with asset discovery
        for handout in &roll20_campaign.handouts {
            match self.parser.convert_handout_to_item(handout.clone()) {
                Ok(item) => {
                    // Discover assets from item
                    self.discover_item_assets(&item);
                    campaign.items.push(item);
                    self.conversion_stats.converted_items += 1;
                }
                Err(e) => {
                    self.conversion_stats.failed_conversions += 1;
                    if let Some(logger) = &self.parser.logging_service {
                        logger.warn(
                            &format!("Failed to convert handout '{}': {e}", handout.name),
                            Some("validated_roll20_parser"),
                        );
                    }
                }
            }
        }

        Ok(campaign)
    }

    /// Create campaign metadata from Roll20 campaign info
    fn create_campaign_metadata(
        &self,
        campaign_info: &crate::roll20::Roll20Campaign,
    ) -> ConversionResult<ttrpg_core::types::CampaignMetadata> {
        Ok(ttrpg_core::types::CampaignMetadata {
            name: campaign_info.campaign.name.clone(),
            source_format: "roll20".to_string(),
            created_at: Some(chrono::Utc::now()),
            schema_version: Some("1.0".to_string()),
            stats: HashMap::new(),
        })
    }

    /// Discover assets from actor
    fn discover_actor_assets(&mut self, actor: &ttrpg_core::types::Actor) {
        // Discover avatar/image assets
        for image in &actor.images {
            if !image.is_empty() {
                self.discovered_assets.push(AssetReference {
                    url: image.clone(),
                    asset_type: "image".to_string(),
                    referenced_by: format!("actor:{}", actor.id),
                    is_accessible: None, // To be checked later by asset service
                });
            }
        }
    }

    /// Discover assets from scene
    fn discover_scene_assets(&mut self, scene: &ttrpg_core::types::Scene) {
        // Discover background images
        if let Some(bg_image) = &scene.background_image {
            self.discovered_assets.push(AssetReference {
                url: bg_image.clone(),
                asset_type: "background".to_string(),
                referenced_by: format!("scene:{}", scene.id),
                is_accessible: None,
            });
        }

        // Discover token images
        for token in &scene.tokens {
            if !token.image.is_empty() {
                self.discovered_assets.push(AssetReference {
                    url: token.image.clone(),
                    asset_type: "token".to_string(),
                    referenced_by: format!("scene:{}:token:{}", scene.id, token.id),
                    is_accessible: None,
                });
            }
        }
    }

    /// Discover assets from item
    fn discover_item_assets(&mut self, item: &ttrpg_core::types::Item) {
        // Discover item images
        if let Some(image) = &item.image {
            self.discovered_assets.push(AssetReference {
                url: image.clone(),
                asset_type: "item".to_string(),
                referenced_by: format!("item:{}", item.id),
                is_accessible: None,
            });
        }
    }

    /// Get discovered asset references
    pub fn get_discovered_assets(&self) -> &[AssetReference] {
        &self.discovered_assets
    }

    /// Get conversion statistics
    pub fn get_conversion_stats(&self) -> &ConversionStats {
        &self.conversion_stats
    }
}

// ============================================================================
// VALIDATION RESULT TYPES
// ============================================================================

/// Complete validation result including campaign and validation details
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Converted campaign
    pub campaign: Campaign,
    /// Detailed validation results
    pub validation_result: Roll20ValidationResult,
    /// Time taken for conversion and validation
    pub conversion_duration: std::time::Duration,
    /// Source file path
    pub file_path: std::path::PathBuf,
}

impl ValidationResult {
    /// Check if conversion and validation were successful
    pub fn is_successful(&self) -> bool {
        self.validation_result.is_valid
    }

    /// Get summary of validation issues
    pub fn get_issues_summary(&self) -> String {
        let result = &self.validation_result;
        format!(
            "Validation completed: {} errors, {} warnings, {} info messages",
            result.stats.errors, result.stats.warnings, result.stats.info
        )
    }

    /// Get human-readable validation report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        let result = &self.validation_result;

        // Header
        report.push_str(&format!(
            "=== Roll20 Conversion Validation Report ===\n\n"
        ));
        report.push_str(&format!("File: {}\n", self.file_path.display()));
        report.push_str(&format!("Duration: {:.2}s\n", self.conversion_duration.as_secs_f64()));
        report.push_str(&format!("Status: {}\n\n", 
            if result.is_valid { "✅ VALID" } else { "❌ INVALID" }
        ));

        // Statistics
        report.push_str("=== Conversion Statistics ===\n");
        report.push_str(&format!("Entities Validated: {}\n", result.performance_metrics.entities_validated));
        report.push_str(&format!("Relationships Checked: {}\n", result.performance_metrics.relationships_checked));
        report.push_str(&format!("Assets Verified: {}\n", result.performance_metrics.assets_verified));
        report.push_str(&format!("Validation Duration: {}ms\n\n", result.performance_metrics.validation_duration_ms));

        // Issue Summary
        report.push_str("=== Issue Summary ===\n");
        report.push_str(&format!("Total Issues: {}\n", result.stats.total_issues));
        report.push_str(&format!("Errors: {}\n", result.stats.errors));
        report.push_str(&format!("Warnings: {}\n", result.stats.warnings));
        report.push_str(&format!("Info: {}\n\n", result.stats.info));

        // Detailed Issues
        if !result.issues.is_empty() {
            report.push_str("=== Detailed Issues ===\n\n");
            for (i, issue) in result.issues.iter().enumerate() {
                let severity_symbol = match issue.severity {
                    ttrpg_core::validation::roll20_validation::ValidationSeverity::Error => "❌",
                    ttrpg_core::validation::roll20_validation::ValidationSeverity::Warning => "⚠️",
                    ttrpg_core::validation::roll20_validation::ValidationSeverity::Info => "ℹ️",
                };
                
                report.push_str(&format!("{}. {} [{:?}] {}\n", 
                    i + 1, severity_symbol, issue.category, issue.message));
                
                if let Some(entity_name) = &issue.context.entity_name {
                    report.push_str(&format!("   Entity: {}\n", entity_name));
                }
                
                if let Some(suggested_fix) = &issue.suggested_fix {
                    report.push_str(&format!("   Suggestion: {}\n", suggested_fix));
                }
                
                report.push_str(&format!("   Path: {}\n\n", issue.data_path));
            }
        }

        report
    }
}
