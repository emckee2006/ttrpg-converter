//! Test command for processing real Roll20 zip files
//!
//! This module provides a command-line interface for testing our complete
//! Roll20 conversion pipeline with real campaign export files.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tracing::info;
use zip::ZipArchive;

use ttrpg_core::{
    error::ConversionResult, manager::DefaultServiceManager, services::ServiceManager,
    types::TargetFormat,
};
use ttrpg_formats::roll20_asset_integration::Roll20AssetPipeline;
// Asset service imports handled through pipeline

/// Test Roll20 zip file processing
pub struct Roll20TestCommand {
    #[allow(dead_code)]
    service_manager: DefaultServiceManager,
}

impl Roll20TestCommand {
    /// Create new test command with initialized services
    pub fn new() -> ConversionResult<Self> {
        let mut service_manager = DefaultServiceManager::new()?;
        service_manager.init_defaults()?;

        Ok(Self { service_manager })
    }

    /// Process a Roll20 zip file through the complete pipeline
    pub async fn process_roll20_zip(
        &self,
        zip_path: &Path,
        output_dir: &Path,
        target_format: TargetFormat,
    ) -> ConversionResult<()> {
        info!("Starting Roll20 zip file processing: {:?}", zip_path);

        // Step 1: Extract zip file
        let temp_dir = self.extract_zip(zip_path).await?;
        info!("Extracted zip to: {:?}", temp_dir);

        // Step 2: Find campaign.json
        let campaign_json = temp_dir.join("campaign.json");
        if !campaign_json.exists() {
            return Err(Box::new(ttrpg_core::error::ConversionError::validation(
                "zip_structure",
                "campaign.json not found in zip file".to_string(),
            )));
        }

        // Step 3: Create pipeline with services
        let cache_dir = std::env::temp_dir().join("ttrpg_cache");
        let config = ttrpg_assets::prelude::Roll20ProcessorConfig::default();
        let service_manager_arc: Arc<dyn ServiceManager> =
            Arc::new(DefaultServiceManager::with_defaults().unwrap());
        let pipeline = Roll20AssetPipeline::new(cache_dir, config, service_manager_arc)
            .await
            .map_err(|e| {
                Box::new(ttrpg_core::error::ConversionError::from_io(
                    std::io::Error::other(format!("Pipeline creation failed: {e}")),
                    "pipeline_creation",
                ))
            })?;

        // Step 4: Process campaign with assets
        let results = pipeline
            .process_and_export_campaign(
                &campaign_json,
                target_format,
                &output_dir.join("exported_campaign.json"),
                Some(Arc::new(|progress| {
                    info!("Progress: {}%", (progress.overall_progress * 100.0) as u32);
                })),
            )
            .await?;

        // Step 5: Display results
        info!("Processing complete!");
        info!(
            "Campaign: {} actors, {} scenes, {} items",
            results.asset_results.campaign.actors.len(),
            results.asset_results.campaign.scenes.len(),
            results.asset_results.campaign.items.len()
        );
        info!(
            "Assets: {}/{} processed successfully",
            results.asset_results.processed_assets.len(),
            results.asset_results.total_discovered
        );
        info!(
            "Export: {} ({})",
            if results.export_result.success {
                "SUCCESS"
            } else {
                "FAILED"
            },
            results.export_result.target_format
        );
        info!("Total time: {}ms", results.total_processing_time_ms);

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        Ok(())
    }

    /// Extract zip file to temporary directory
    async fn extract_zip(&self, zip_path: &Path) -> ConversionResult<PathBuf> {
        let temp_dir = std::env::temp_dir().join("roll20_test");
        fs::create_dir_all(&temp_dir)
            .await
            .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "temp_dir_creation"))?;

        let file = std::fs::File::open(zip_path)
            .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "zip_file_open"))?;

        let mut archive = ZipArchive::new(file).map_err(|e| {
            ttrpg_core::error::ConversionError::validation(
                "zip_archive",
                format!("Failed to read zip archive: {e}"),
            )
        })?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| {
                ttrpg_core::error::ConversionError::validation(
                    "zip_extraction",
                    format!("Failed to extract file {i}: {e}"),
                )
            })?;

            let outpath = match file.enclosed_name() {
                Some(path) => temp_dir.join(path),
                None => continue,
            };

            if file.name().ends_with('/') {
                std::fs::create_dir_all(&outpath)
                    .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "dir_creation"))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(p).map_err(|e| {
                            ttrpg_core::error::ConversionError::from_io(e, "parent_dir_creation")
                        })?;
                    }
                }
                let mut outfile = std::fs::File::create(&outpath)
                    .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "file_creation"))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "file_copy"))?;
            }
        }

        Ok(temp_dir)
    }

    /// List contents of a Roll20 zip file for inspection
    pub fn inspect_zip(&self, zip_path: &Path) -> ConversionResult<()> {
        let file = std::fs::File::open(zip_path)
            .map_err(|e| ttrpg_core::error::ConversionError::from_io(e, "zip_file_open"))?;

        let mut archive = ZipArchive::new(file).map_err(|e| {
            ttrpg_core::error::ConversionError::validation(
                "zip_archive",
                format!("Failed to read zip archive: {e}"),
            )
        })?;

        info!("Roll20 zip file contents:");
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| {
                ttrpg_core::error::ConversionError::validation(
                    "zip_inspection",
                    format!("Failed to read file {i}: {e}"),
                )
            })?;

            info!("  {} ({} bytes)", file.name(), file.size());
        }

        Ok(())
    }
}

/// Helper function to create test command and process file
pub async fn test_roll20_file(
    zip_path: &Path,
    output_dir: &Path,
    target_format: TargetFormat,
) -> ConversionResult<()> {
    let command = Roll20TestCommand::new()?;

    // First inspect the zip contents
    info!("Inspecting Roll20 zip file...");
    command.inspect_zip(zip_path)?;

    // Then process it
    command
        .process_roll20_zip(zip_path, output_dir, target_format)
        .await
}
