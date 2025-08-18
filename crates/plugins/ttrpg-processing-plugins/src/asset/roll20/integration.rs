// TODO: Re-enable Roll20 integration once missing types are implemented
// Currently commented out due to unresolved imports for Roll20Parser, Roll20AssetProcessor,
// RustAssetService, RustExportService, and other missing types

/*
//! Roll20 asset processing integration
//!
//! This module provides integration between Roll20Parser, Roll20AssetProcessor, and ValidationEngine
//! to create a complete end-to-end asset processing pipeline for Roll20 campaigns.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info, instrument};

// Roll20AssetProcessor functionality moved to plugin architecture
use ttrpg_core::{
    error::{AssetResult, ConversionResult},
    plugin_framework::{
        interfaces::{AssetPlugin, ExportPlugin, LoggingPlugin},
        types::{AssetInfo, ExportResult},
        // Remove unused import
    },
    types::{Campaign, TargetFormat},
};

// Import paths updated for reorganized module structure
// Roll20Parser now in ttrpg-input-plugins crate

/// Integrated Roll20 asset processing pipeline
///
/// Combines Roll20Parser, Roll20AssetProcessor, and ValidationEngine to provide
/// complete campaign conversion with asset processing and validation.
pub struct Roll20AssetPipeline {
    /// Roll20 campaign parser
    parser: Roll20Parser,
    /// Asset processor for Roll20-specific asset handling
    asset_processor: Arc<Roll20AssetProcessor>,
    /// Logger for pipeline operations
    logger: Option<Arc<dyn LoggingService>>,
}

/// Asset processing results for a campaign
#[derive(Debug)]
pub struct CampaignAssetResults {
    /// Converted campaign data
    pub campaign: Campaign,
    /// Successfully processed assets
    pub processed_assets: Vec<AssetInfo>,
    /// Failed asset downloads
    pub failed_assets: Vec<(Roll20AssetInfo, String)>,
    /// Total assets discovered
    pub total_discovered: usize,
    /// Processing statistics
    pub processing_time_ms: u64,
}

impl Roll20AssetPipeline {
    /// Create a new integrated Roll20 asset pipeline
    ///
    /// # Arguments
    /// * `cache_dir` - Directory for asset caching
    /// * `config` - Roll20 processor configuration
    /// * `service_manager` - Service manager for dependency injection
    ///
    /// # Returns
    /// New Roll20AssetPipeline instance
    ///
    /// # Errors
    /// Returns error if asset processor creation fails
    #[instrument(name = "roll20_pipeline_new", skip(service_manager))]
    pub async fn new(
        cache_dir: PathBuf,
        config: Roll20ProcessorConfig,
        service_manager: Arc<dyn ServiceManager>,
    ) -> AssetResult<Self> {
        info!("Creating Roll20 asset processing pipeline");

        // Get services from service manager
        let validation_service = service_manager.validation();
        let logging_service = Some(service_manager.logging());

        // Create Roll20 parser (only needs validation and logging - no asset service needed)
        let parser = Roll20Parser::new()
            .with_validation(validation_service)
            .with_logging(logging_service.clone().unwrap());

        // Create Roll20 asset processor (this creates the RustAssetService internally)
        let asset_processor = Arc::new(
            Roll20AssetProcessor::with_defaults(cache_dir, logging_service.clone()).await?,
        );

        Ok(Self { parser, asset_processor, logger: logging_service })
    }

    /// Create pipeline with default configuration
    pub async fn with_defaults(
        cache_dir: PathBuf,
        service_manager: Arc<dyn ServiceManager>,
    ) -> AssetResult<Self> {
        Self::new(cache_dir, Roll20ProcessorConfig::default(), service_manager).await
    }

    /// Process a Roll20 campaign file with full asset processing pipeline
    ///
    /// # Arguments
    /// * `campaign_file` - Path to Roll20 campaign JSON file
    /// * `progress_callback` - Optional callback for progress updates
    ///
    /// # Returns
    /// Complete campaign processing results including assets
    ///
    /// # Errors
    /// Returns error if campaign parsing or asset processing fails
    #[instrument(name = "process_campaign_with_assets", skip(self, progress_callback))]
    pub async fn process_campaign_with_assets(
        &self,
        campaign_file: &std::path::Path,
        progress_callback: Option<ProgressCallback>,
    ) -> ConversionResult<CampaignAssetResults> {
        let start_time = std::time::Instant::now();
        info!("Starting Roll20 campaign processing with assets: {:?}", campaign_file);

        // Step 1: Parse the campaign file
        debug!("Step 1: Parsing Roll20 campaign file");
        let campaign = self.parser.parse_campaign_file(campaign_file).await?;

        if let Some(logger) = &self.logger {
            logger.log_with_data(
                ttrpg_core::services::LogLevel::Info,
                "Campaign parsed successfully",
                &serde_json::json!({
                    "actors": campaign.actors.len(),
                    "scenes": campaign.scenes.len(),
                    "items": campaign.items.len()
                }),
            );
        }

        // Step 2: Discover assets from original Roll20 data
        debug!("Step 2: Discovering assets from Roll20 campaign data");
        let campaign_json = self.load_campaign_json(campaign_file)?;
        let discovered_assets = self
            .asset_processor
            .discover_assets(&campaign_json)
            .await
            .map_err(|e| {
                ttrpg_core::error::ConversionError::from_io(
                    std::io::Error::other(format!("Asset discovery failed: {e}")),
                    "asset discovery",
                )
            })?;

        info!("Discovered {} assets for processing", discovered_assets.len());

        // Step 3: Process assets with progress tracking
        debug!("Step 3: Processing assets with bulk download");
        let asset_results = self
            .asset_processor
            .process_assets_bulk(discovered_assets.clone(), progress_callback)
            .await
            .map_err(|e| {
                ttrpg_core::error::ConversionError::from_io(
                    std::io::Error::other(format!("Asset processing failed: {e}")),
                    "asset processing",
                )
            })?;

        // Step 4: Categorize results
        debug!("Step 4: Categorizing asset processing results");
        let mut processed_assets = Vec::new();
        let mut failed_assets = Vec::new();

        for (asset_info, result) in discovered_assets.into_iter().zip(asset_results.into_iter()) {
            match result {
                Ok(asset_data) => {
                    processed_assets.push(asset_data);
                }
                Err(error) => {
                    failed_assets.push((asset_info, error.to_string()));
                }
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        info!(
            "Campaign asset processing complete: {}/{} assets successful in {}ms",
            processed_assets.len(),
            processed_assets.len() + failed_assets.len(),
            processing_time
        );

        // Log results
        if let Some(logger) = &self.logger {
            logger.log_with_data(
                ttrpg_core::services::LogLevel::Info,
                "Asset processing results",
                &serde_json::json!({
                    "successful": processed_assets.len(),
                    "failed": failed_assets.len(),
                    "time_ms": processing_time
                }),
            );
        }

        // Calculate total before moving vectors
        let total_discovered = processed_assets.len() + failed_assets.len();

        Ok(CampaignAssetResults {
            campaign,
            processed_assets,
            failed_assets,
            total_discovered,
            processing_time_ms: processing_time,
        })
    }

    /// Process campaign data directly (useful for testing)
    pub async fn process_campaign_data(
        &self,
        roll20_campaign: Roll20Campaign,
        progress_callback: Option<ProgressCallback>,
    ) -> ConversionResult<CampaignAssetResults> {
        let start_time = std::time::Instant::now();
        info!("Processing Roll20 campaign data directly");

        // Convert to standardized campaign format
        let campaign = self.parser.convert_to_campaign(roll20_campaign.clone())?;

        // Convert Roll20Campaign to JSON for asset discovery
        let campaign_json = serde_json::to_value(&roll20_campaign).map_err(|e| {
            ttrpg_core::error::ConversionError::validation(
                "JSON serialization",
                format!("JSON serialization failed: {e}"),
            )
        })?;

        // Discover and process assets
        let discovered_assets = self
            .asset_processor
            .discover_assets(&campaign_json)
            .await
            .map_err(|e| {
                ttrpg_core::error::ConversionError::from_io(
                    std::io::Error::other(format!("Asset discovery failed: {e}")),
                    "asset discovery",
                )
            })?;

        let asset_results = self
            .asset_processor
            .process_assets_bulk(discovered_assets.clone(), progress_callback)
            .await
            .map_err(|e| {
                ttrpg_core::error::ConversionError::from_io(
                    std::io::Error::other(format!("Asset processing failed: {e}")),
                    "asset processing",
                )
            })?;

        // Categorize results
        let mut processed_assets = Vec::new();
        let mut failed_assets = Vec::new();

        for (asset_info, result) in discovered_assets.into_iter().zip(asset_results.into_iter()) {
            match result {
                Ok(asset_data) => processed_assets.push(asset_data),
                Err(error) => failed_assets.push((asset_info, error.to_string())),
            }
        }

        let processing_time = start_time.elapsed().as_millis() as u64;

        // Calculate total before moving vectors
        let total_discovered = processed_assets.len() + failed_assets.len();

        Ok(CampaignAssetResults {
            campaign,
            processed_assets,
            failed_assets,
            total_discovered,
            processing_time_ms: processing_time,
        })
    }

    /// Get asset processor statistics
    pub async fn get_statistics(&self) -> ttrpg_assets::prelude::Roll20ProcessingStats {
        self.asset_processor.get_processing_stats().await
    }

    // Private helper methods

    fn load_campaign_json(
        &self,
        campaign_file: &std::path::Path,
    ) -> ConversionResult<serde_json::Value> {
        let content = std::fs::read_to_string(campaign_file)
            .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "file reading"))?;

        let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
            ttrpg_core::error::ConversionError::validation(
                "JSON parsing",
                format!("Invalid JSON: {e}"),
            )
        })?;

        Ok(json)
    }

    /// Complete end-to-end workflow: Roll20 JSON → Parse → Validate → Process Assets → Export
    pub async fn process_and_export_campaign(
        &self,
        roll20_file_path: &Path,
        target_format: TargetFormat,
        output_path: &Path,
        progress_callback: Option<ProgressCallback>,
    ) -> ConversionResult<CampaignExportResults> {
        let start_time = std::time::Instant::now();

        if let Some(logger) = &self.logger {
            logger
                .info("Starting complete Roll20 campaign conversion and export", Some("pipeline"));
        }

        // Step 1: Process campaign with assets
        let asset_results = self
            .process_campaign_with_assets(roll20_file_path, progress_callback)
            .await?;

        // Step 2: Create export service
        let export_service = if let Some(logger) = &self.logger {
            RustExportService::with_logging(logger.clone())
        } else {
            RustExportService::new()
        };

        // Step 3: Export campaign to target format
        let export_result = export_service
            .export_campaign(&asset_results.campaign, target_format, output_path)
            .await?;

        let total_time = start_time.elapsed().as_millis() as u64;

        if let Some(logger) = &self.logger {
            logger.log_with_data(
                ttrpg_core::services::LogLevel::Info,
                "Complete campaign conversion and export finished",
                &serde_json::json!({
                    "total_time_ms": total_time,
                    "asset_processing_time_ms": asset_results.processing_time_ms,
                    "export_time_ms": export_result.stats.processing_time_ms,
                    "assets_processed": asset_results.processed_assets.len(),
                    "assets_failed": asset_results.failed_assets.len(),
                    "target_format": target_format,
                    "success": export_result.success
                }),
            );
        }

        Ok(CampaignExportResults {
            asset_results,
            export_result,
            total_processing_time_ms: total_time,
        })
    }

    /// Export an already processed campaign to multiple formats
    pub async fn export_campaign_to_formats(
        &self,
        campaign: &Campaign,
        exports: &[(TargetFormat, PathBuf)],
    ) -> ConversionResult<Vec<ExportResult>> {
        let export_service = if let Some(logger) = &self.logger {
            RustExportService::with_logging(logger.clone())
        } else {
            RustExportService::new()
        };

        let mut results = Vec::new();

        for (target_format, output_path) in exports {
            if let Some(logger) = &self.logger {
                logger.info(&format!("Exporting campaign to {target_format}"), Some("export"));
            }

            match export_service
                .export_campaign(campaign, *target_format, output_path)
                .await
            {
                Ok(result) => results.push(result),
                Err(error) => {
                    if let Some(logger) = &self.logger {
                        logger.error(
                            &format!("Failed to export to {target_format}: {error}"),
                            Some("export"),
                        );
                    }
                    return Err(error);
                }
            }
        }

        Ok(results)
    }

    /// Preview export without actually writing files
    pub fn preview_export(
        &self,
        campaign: &Campaign,
        target_format: TargetFormat,
    ) -> ConversionResult<ttrpg_core::services::ExportPreview> {
        let export_service = if let Some(logger) = &self.logger {
            RustExportService::with_logging(logger.clone())
        } else {
            RustExportService::new()
        };

        export_service.preview_export(campaign, target_format)
    }
}

impl std::fmt::Display for CampaignAssetResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Campaign: {} actors, {} scenes, {} items | Assets: {}/{} successful ({} failed) | Time: {}ms",
               self.campaign.actors.len(),
               self.campaign.scenes.len(),
               self.campaign.items.len(),
               self.processed_assets.len(),
               self.total_discovered,
               self.failed_assets.len(),
               self.processing_time_ms)
    }
}

/// Results from complete campaign processing and export
#[derive(Debug)]
pub struct CampaignExportResults {
    /// Asset processing results
    pub asset_results: CampaignAssetResults,

    /// Export operation results  
    pub export_result: ExportResult,

    /// Total processing time for entire pipeline
    pub total_processing_time_ms: u64,
}

impl std::fmt::Display for CampaignExportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Complete Pipeline: {} | Export: {} → {} ({} bytes) | Total Time: {}ms",
            self.asset_results,
            self.export_result.target_format,
            if self.export_result.success {
                "SUCCESS"
            } else {
                "FAILED"
            },
            self.export_result.stats.output_size_bytes,
            self.total_processing_time_ms
        )
    }
}
*/
