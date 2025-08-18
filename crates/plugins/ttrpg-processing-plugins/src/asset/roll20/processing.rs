//! Roll20-specific asset processing and management
//!
//! This module provides specialized asset processing capabilities for Roll20 campaigns,
//! extending the base RustAssetService with Roll20-specific features including:
//! - Automatic asset discovery from Roll20 campaign data
//! - Bulk parallel downloading with progress tracking
//! - Roll20-specific asset categorization and optimization
//! - Integration with Roll20Parser and validation pipeline

// Updated imports for plugin architecture
// TODO: Re-enable when asset::core module is implemented
// use crate::asset::core::AssetType;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, instrument, warn};

use ttrpg_core::{
    error::{AssetError, AssetResult},
    plugin_framework::{
        interfaces::AssetPlugin,
        types::{AssetInfo, AssetStats},
    },
};

/// Progress callback for asset downloading operations
pub type ProgressCallback = Arc<dyn Fn(AssetDownloadProgress) + Send + Sync>;

/// Asset download progress information
#[derive(Debug, Clone)]
pub struct AssetDownloadProgress {
    /// Current asset being processed
    pub current_asset: String,
    /// Assets completed so far
    pub completed: usize,
    /// Total assets to process
    pub total: usize,
    /// Current asset download progress (0.0 to 1.0)
    pub current_progress: f64,
    /// Overall progress (0.0 to 1.0)
    pub overall_progress: f64,
    /// Download speed in bytes per second
    pub download_speed: u64,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<u64>,
}

/// Roll20-specific asset categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Roll20AssetCategory {
    /// Character avatar images
    CharacterAvatar,
    /// Scene/page background images
    SceneBackground,
    /// Token images for map representation
    TokenImage,
    /// Handout documents and images
    HandoutAsset,
    /// Audio files (music, sound effects)
    AudioFile,
    /// Other/unknown asset type
    Other,
}

/// Roll20 asset information with enhanced metadata
#[derive(Debug, Clone)]
pub struct Roll20AssetInfo {
    /// Roll20 asset ID
    pub id: String,
    /// Asset name/filename
    pub name: String,
    /// Asset URL for downloading
    pub url: String,
    /// Roll20-specific asset category
    pub category: Roll20AssetCategory,
    /// Base asset type (image, audio, etc.)
    pub asset_type: AssetType,
    /// File size in bytes (if known)
    pub size_bytes: Option<u64>,
    /// Associated entity IDs (character, page, etc.)
    pub entity_references: Vec<String>,
    /// Download priority (0 = highest)
    pub priority: u32,
}

/// Roll20 asset processing configuration
#[derive(Debug, Clone)]
pub struct Roll20ProcessorConfig {
    /// Maximum concurrent downloads
    pub max_concurrent_downloads: usize,
    /// Download timeout in seconds
    pub download_timeout_secs: u64,
    /// Maximum retry attempts for failed downloads
    pub max_retries: u32,
    /// Enable image optimization
    pub optimize_images: bool,
    /// Target image format for optimization
    pub target_image_format: Option<String>,
    /// Maximum image resolution (width, height)
    pub max_image_resolution: Option<(u32, u32)>,
    /// Image quality for compression (0-100)
    pub image_quality: u8,
}

impl Default for Roll20ProcessorConfig {
    fn default() -> Self {
        Self {
            max_concurrent_downloads: 8,
            download_timeout_secs: 30,
            max_retries: 3,
            optimize_images: true,
            target_image_format: Some("webp".to_string()),
            max_image_resolution: Some((2048, 2048)),
            image_quality: 85,
        }
    }
}

/// Specialized asset processor for Roll20 campaigns
///
/// Extends RustAssetService with Roll20-specific capabilities including:
/// - Asset discovery from Roll20 campaign data
/// - Bulk parallel downloading with progress tracking
/// - Roll20-specific asset categorization and optimization
/// - Integration with validation pipeline
pub struct Roll20AssetProcessor {
    /// Base asset service for core functionality
    base_service: Arc<RustAssetService>,
    /// Roll20-specific configuration
    config: Roll20ProcessorConfig,
    /// Asset processing statistics
    stats: Arc<Mutex<Roll20ProcessingStats>>,
    // Logger for detailed operation tracking replaced by tracing ecosystem
    // LoggingService field removed - using direct tracing instead
}

/// Enhanced processing statistics for Roll20 assets
#[derive(Debug, Clone, Default)]
pub struct Roll20ProcessingStats {
    /// Base asset statistics
    pub base_stats: AssetStats,
    /// Assets discovered from Roll20 data
    pub assets_discovered: usize,
    /// Assets successfully categorized
    pub assets_categorized: usize,
    /// Assets optimized (compressed, converted)
    pub assets_optimized: usize,
    /// Total optimization time (milliseconds)
    pub optimization_time_ms: u64,
    /// Bulk download operations
    pub bulk_operations: usize,
    /// Average download speed (bytes per second)
    pub average_download_speed: u64,
}

impl Roll20AssetProcessor {
    /// Create a new Roll20 asset processor
    ///
    /// # Arguments
    /// * `base_service` - Base asset service for core functionality
    /// * `config` - Roll20-specific processing configuration
    ///
    /// # Returns
    /// New Roll20AssetProcessor instance
    #[instrument(name = "roll20_processor_new", skip(base_service))]
    pub fn new(base_service: Arc<RustAssetService>, config: Roll20ProcessorConfig) -> Self {
        info!(
            "Creating Roll20AssetProcessor with config: max_concurrent={}, optimize_images={}",
            config.max_concurrent_downloads, config.optimize_images
        );

        Self {
            base_service,
            config,
            stats: Arc::new(Mutex::new(Roll20ProcessingStats::default())),
            // logger field removed - using direct tracing instead
        }
    }

    /// Create processor with default configuration
    pub async fn with_defaults(cache_dir: PathBuf) -> AssetResult<Self> {
        let base_service = Arc::new(RustAssetService::new(cache_dir).await?);
        Ok(Self::new(base_service, Roll20ProcessorConfig::default()))
    }

    /// Discover assets from Roll20 campaign data
    ///
    /// # Arguments
    /// * `campaign_data` - Raw Roll20 campaign data as JSON value
    ///
    /// # Returns
    /// Vector of discovered Roll20 asset information
    ///
    /// # Errors
    /// Returns error if asset discovery fails
    #[instrument(name = "discover_roll20_assets", skip(self, campaign_data))]
    pub async fn discover_assets(
        &self,
        campaign_data: &serde_json::Value,
    ) -> AssetResult<Vec<Roll20AssetInfo>> {
        info!("Beginning Roll20 asset discovery from campaign data");

        let mut discovered_assets = Vec::new();
        let mut entity_asset_map = HashMap::new();

        // Extract assets from Roll20 assets array
        if let Some(assets_array) = campaign_data.get("assets").and_then(|a| a.as_array()) {
            debug!("Found {} assets in Roll20 assets array", assets_array.len());

            for asset_value in assets_array {
                if let Some(asset_info) = self
                    .extract_asset_from_json(asset_value, &mut entity_asset_map)
                    .await?
                {
                    discovered_assets.push(asset_info);
                }
            }
        }

        // Extract character avatar assets
        if let Some(characters) = campaign_data.get("characters").and_then(|c| c.as_array()) {
            debug!("Scanning {} characters for avatar assets", characters.len());

            for character in characters {
                if let Some(avatar_assets) = self.extract_character_assets(character).await? {
                    discovered_assets.extend(avatar_assets);
                }
            }
        }

        // Extract page/scene background assets
        if let Some(pages) = campaign_data.get("pages").and_then(|p| p.as_array()) {
            debug!("Scanning {} pages for background assets", pages.len());

            for page in pages {
                if let Some(page_assets) = self.extract_page_assets(page).await? {
                    discovered_assets.extend(page_assets);
                }
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.assets_discovered = discovered_assets.len();
        }

        info!("Asset discovery complete: found {} total assets", discovered_assets.len());
        Ok(discovered_assets)
    }

    /// Process assets in bulk with progress tracking
    ///
    /// # Arguments
    /// * `assets` - Assets to process
    /// * `progress_callback` - Optional callback for progress updates
    ///
    /// # Returns
    /// Results of asset processing operations
    ///
    /// # Errors
    /// Returns error if bulk processing fails
    #[instrument(name = "process_assets_bulk", skip(self, assets, progress_callback))]
    pub async fn process_assets_bulk(
        &self,
        assets: Vec<Roll20AssetInfo>,
        progress_callback: Option<ProgressCallback>,
    ) -> AssetResult<Vec<AssetResult<AssetInfo>>> {
        let total_assets = assets.len();
        info!("Starting bulk asset processing for {} assets", total_assets);

        // Sort assets by priority (lower number = higher priority)
        let mut sorted_assets = assets;
        sorted_assets.sort_by_key(|a| a.priority);

        let (tx, mut rx) = mpsc::channel(self.config.max_concurrent_downloads);
        let mut results = Vec::new();
        let mut completed = 0;

        // Create semaphore for concurrent download limiting
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.config.max_concurrent_downloads));

        // Spawn download tasks
        let mut tasks = Vec::new();
        for (index, asset) in sorted_assets.into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let base_service = self.base_service.clone();
            let config = self.config.clone();
            let tx = tx.clone();

            let task = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                let result = Self::download_and_process_asset(base_service, asset, config).await;
                let _ = tx.send((index, result)).await;
            });

            tasks.push(task);
        }

        // Drop the sender to close the channel
        drop(tx);

        // Initialize results vector with proper size
        results.resize_with(total_assets, || {
            Err(AssetError::ValidationError {
                asset_path: "unknown".to_string(),
                reason: "Processing incomplete".to_string(),
                expected_type: None,
            })
        });

        while let Some((index, result)) = rx.recv().await {
            results[index] = result;
            completed += 1;

            // Update progress if callback provided
            if let Some(callback) = &progress_callback {
                let progress = AssetDownloadProgress {
                    current_asset: format!("Asset {completed}/{total_assets}"),
                    completed,
                    total: total_assets,
                    current_progress: 1.0,
                    overall_progress: completed as f64 / total_assets as f64,
                    download_speed: 0, // TODO: Calculate actual speed
                    eta_seconds: None, // TODO: Calculate ETA
                };
                callback(progress);
            }
        }

        // Update statistics
        {
            let mut stats = self.stats.lock().await;
            stats.bulk_operations += 1;
        }

        info!(
            "Bulk asset processing complete: {}/{} successful",
            results.iter().filter(|r| r.is_ok()).count(),
            total_assets
        );

        Ok(results)
    }

    /// Get processing statistics
    pub async fn get_processing_stats(&self) -> Roll20ProcessingStats {
        self.stats.lock().await.clone()
    }

    // Private helper methods

    async fn extract_asset_from_json(
        &self,
        asset_json: &serde_json::Value,
        _entity_map: &mut HashMap<String, Vec<String>>,
    ) -> AssetResult<Option<Roll20AssetInfo>> {
        let id = asset_json
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let name = asset_json
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let url = asset_json
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AssetError::ValidationError {
                asset_path: format!("asset_{id}"),
                reason: "Missing URL field in Roll20 asset".to_string(),
                expected_type: Some("URL string".to_string()),
            })?;

        let asset_type_str = asset_json
            .get("asset_type")
            .and_then(|v| v.as_str())
            .unwrap_or("other");

        let category = self.categorize_roll20_asset(asset_type_str, url);
        let asset_type = self.determine_base_asset_type(url, asset_type_str);

        let size_bytes = asset_json.get("size").and_then(|v| v.as_u64());

        Ok(Some(Roll20AssetInfo {
            id,
            name,
            url: url.to_string(),
            category,
            asset_type,
            size_bytes,
            entity_references: Vec::new(), // TODO: Build entity reference map
            priority: self.determine_asset_priority(category),
        }))
    }

    async fn extract_character_assets(
        &self,
        character: &serde_json::Value,
    ) -> AssetResult<Option<Vec<Roll20AssetInfo>>> {
        let avatar_url = character
            .get("avatar")
            .and_then(|v| v.as_str())
            .unwrap_or("https://example.com/unknown.xyz");

        let avatar_type = character
            .get("avatar_type")
            .and_then(|v| v.as_str())
            .unwrap_or("other");

        let avatar_type_enum = match avatar_type.to_lowercase().as_str() {
            "image" => AssetType::Image,
            "audio" => AssetType::Audio,
            _ => AssetType::Attachment,
        };

        let asset_info = Roll20AssetInfo {
            id: "character_avatar".to_string(),
            name: "Character Avatar".to_string(),
            url: avatar_url.to_string(),
            category: Roll20AssetCategory::CharacterAvatar,
            asset_type: avatar_type_enum,
            size_bytes: None,
            entity_references: Vec::new(),
            priority: self.determine_asset_priority(Roll20AssetCategory::CharacterAvatar),
        };

        Ok(Some(vec![asset_info]))
    }

    async fn extract_page_assets(
        &self,
        _page: &serde_json::Value,
    ) -> AssetResult<Option<Vec<Roll20AssetInfo>>> {
        // TODO: Implement page asset extraction (backgrounds, tokens, etc.)
        Ok(None)
    }

    fn categorize_roll20_asset(&self, asset_type: &str, url: &str) -> Roll20AssetCategory {
        match asset_type.to_lowercase().as_str() {
            "character" | "avatar" => Roll20AssetCategory::CharacterAvatar,
            "page" | "background" => Roll20AssetCategory::SceneBackground,
            "token" => Roll20AssetCategory::TokenImage,
            "handout" | "document" => Roll20AssetCategory::HandoutAsset,
            "audio" | "music" | "sound" => Roll20AssetCategory::AudioFile,
            _ => {
                // Try to categorize by URL patterns
                let url_lower = url.to_lowercase();
                if url_lower.contains("token") {
                    Roll20AssetCategory::TokenImage
                } else if url_lower.contains("background") || url_lower.contains("map") {
                    Roll20AssetCategory::SceneBackground
                } else if url_lower.contains("avatar") || url_lower.contains("portrait") {
                    Roll20AssetCategory::CharacterAvatar
                } else {
                    Roll20AssetCategory::Other
                }
            }
        }
    }

    fn determine_base_asset_type(&self, url: &str, asset_type: &str) -> AssetType {
        let url_lower = url.to_lowercase();
        let type_lower = asset_type.to_lowercase();

        if url_lower.ends_with(".png")
            || url_lower.ends_with(".jpg")
            || url_lower.ends_with(".jpeg")
            || url_lower.ends_with(".webp")
            || url_lower.ends_with(".gif")
            || type_lower.contains("image")
        {
            AssetType::Image
        } else if url_lower.ends_with(".mp3")
            || url_lower.ends_with(".ogg")
            || url_lower.ends_with(".wav")
            || type_lower.contains("audio")
            || type_lower.contains("music")
        {
            AssetType::Audio
        } else if url_lower.ends_with(".pdf")
            || url_lower.ends_with(".txt")
            || url_lower.ends_with(".doc")
            || type_lower.contains("document")
        {
            AssetType::Document
        } else {
            AssetType::Attachment
        }
    }

    fn determine_asset_priority(&self, category: Roll20AssetCategory) -> u32 {
        match category {
            Roll20AssetCategory::CharacterAvatar => 1, // High priority
            Roll20AssetCategory::TokenImage => 2,
            Roll20AssetCategory::SceneBackground => 3,
            Roll20AssetCategory::HandoutAsset => 4,
            Roll20AssetCategory::AudioFile => 5,
            Roll20AssetCategory::Other => 6, // Low priority
        }
    }

    async fn download_and_process_asset(
        base_service: Arc<RustAssetService>,
        asset: Roll20AssetInfo,
        _config: Roll20ProcessorConfig,
    ) -> AssetResult<AssetInfo> {
        debug!("Processing Roll20 asset: {} ({})", asset.name, asset.url);

        // Download using base service
        let cached_path = base_service.download_asset(&asset.url, false).await?;

        // Get asset information
        let asset_info = base_service.get_asset_info(&cached_path)?;

        // TODO: Implement asset optimization based on config

        Ok(asset_info)
    }
}

impl std::fmt::Display for Roll20AssetCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterAvatar => write!(f, "Character Avatar"),
            Self::SceneBackground => write!(f, "Scene Background"),
            Self::TokenImage => write!(f, "Token Image"),
            Self::HandoutAsset => write!(f, "Handout Asset"),
            Self::AudioFile => write!(f, "Audio File"),
            Self::Other => write!(f, "Other"),
        }
    }
}

#[cfg(test)]
mod tests;
