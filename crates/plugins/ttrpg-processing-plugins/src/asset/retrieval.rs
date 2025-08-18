//! AssetRetrievalPlugin - HTTP downloads, ZIP extraction, caching
//!
//! This plugin handles downloading assets from various sources including HTTP URLs,
//! ZIP file extraction, and intelligent caching to avoid redundant downloads.

use crate::shared::AssetExecutionContext;
use ttrpg_core::{
    error::ConversionResult,
    plugin_framework::types::{AssetInfo, AssetStats},
};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use blake3::Hasher;
use moka::future::Cache;

/// AssetRetrievalPlugin for HTTP downloads and caching
pub struct AssetRetrievalPlugin {
    /// Shared execution context
    context: Arc<AssetExecutionContext>,
    /// Asset cache for avoiding redundant downloads
    cache: Cache<String, CachedAsset>,
    /// Configuration for retrieval operations
    config: RetrievalConfig,
}

/// Configuration for asset retrieval
#[derive(Debug, Clone)]
pub struct RetrievalConfig {
    /// Maximum file size to download (bytes)
    pub max_file_size: u64,
    /// Cache directory path
    pub cache_dir: PathBuf,
    /// Enable ZIP extraction
    pub enable_zip_extraction: bool,
    /// Connection timeout (seconds)
    pub timeout_seconds: u64,
    /// Maximum concurrent downloads
    pub max_concurrent_downloads: usize,
}

impl Default for RetrievalConfig {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB
            cache_dir: PathBuf::from("./asset_cache"),
            enable_zip_extraction: true,
            timeout_seconds: 30,
            max_concurrent_downloads: 10,
        }
    }
}

/// Cached asset data
#[derive(Debug, Clone)]
struct CachedAsset {
    /// Asset data
    data: Vec<u8>,
    /// Content hash for verification
    hash: String,
    /// MIME type
    mime_type: Option<String>,
    /// Cache timestamp
    cached_at: std::time::SystemTime,
}

impl AssetRetrievalPlugin {
    /// Create new asset retrieval plugin
    pub fn new() -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let config = RetrievalConfig::default();
        
        // Create cache directory
        std::fs::create_dir_all(&config.cache_dir)?;
        
        // Initialize asset cache with 1000 entries
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(3600)) // 1 hour TTL
            .build();
        
        Ok(Self {
            context,
            cache,
            config,
        })
    }
    
    /// Create plugin with custom configuration
    pub fn with_config(config: RetrievalConfig) -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        
        // Create cache directory
        std::fs::create_dir_all(&config.cache_dir)?;
        
        let cache = Cache::builder()
            .max_capacity(1000)
            .time_to_live(std::time::Duration::from_secs(3600))
            .build();
        
        Ok(Self {
            context,
            cache,
            config,
        })
    }
    
    /// Download single asset with caching
    pub async fn download_asset(&self, url: &str) -> ConversionResult<Vec<u8>> {
        // Check cache first
        if let Some(cached) = self.cache.get(url).await {
            tracing::info!("Cache hit for asset: {}", url);
            return Ok(cached.data);
        }
        
        // Acquire semaphore permit for controlled concurrency
        let _permit = self.context.semaphore.acquire().await?;
        
        tracing::info!("Downloading asset: {}", url);
        
        // Download with timeout
        let response = self.context.http_client
            .get(url)
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .send()
            .await?;
        
        // Check content length
        if let Some(content_length) = response.content_length() {
            if content_length > self.config.max_file_size {
                return Err(ttrpg_core::error::ConversionError::AssetError(
                    format!("Asset too large: {} bytes (max: {})", content_length, self.config.max_file_size)
                ));
            }
        }
        
        // Get MIME type
        let mime_type = response.headers()
            .get("content-type")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());
        
        // Download bytes
        let bytes = response.bytes().await?;
        let data = bytes.to_vec();
        
        // Verify size after download
        if data.len() as u64 > self.config.max_file_size {
            return Err(ttrpg_core::error::ConversionError::AssetError(
                format!("Downloaded asset too large: {} bytes", data.len())
            ));
        }
        
        // Compute hash for caching
        let hash = self.compute_hash(&data);
        
        // Cache the asset
        let cached_asset = CachedAsset {
            data: data.clone(),
            hash,
            mime_type,
            cached_at: std::time::SystemTime::now(),
        };
        
        self.cache.insert(url.to_string(), cached_asset).await;
        
        tracing::info!("Successfully downloaded and cached asset: {} ({} bytes)", url, data.len());
        
        Ok(data)
    }
    
    /// Download multiple assets in parallel
    pub async fn download_assets_bulk(&self, urls: Vec<String>) -> ConversionResult<Vec<AssetResult>> {
        let mut tasks = Vec::new();
        
        for url in urls {
            let plugin = self.clone();
            let task = tokio::spawn(async move {
                match plugin.download_asset(&url).await {
                    Ok(data) => AssetResult::Success {
                        url: url.clone(),
                        data,
                        size_bytes: data.len() as u64,
                    },
                    Err(e) => AssetResult::Failed {
                        url: url.clone(),
                        error: e.to_string(),
                    },
                }
            });
            tasks.push(task);
        }
        
        // Wait for all downloads to complete
        let results = futures::future::join_all(tasks).await;
        
        Ok(results.into_iter()
            .collect::<Result<Vec<_>, _>>()?
        )
    }
    
    /// Extract ZIP file contents
    pub async fn extract_zip(&self, zip_data: &[u8]) -> ConversionResult<Vec<ExtractedFile>> {
        if !self.config.enable_zip_extraction {
            return Err(ttrpg_core::error::ConversionError::AssetError(
                "ZIP extraction is disabled".to_string()
            ));
        }
        
        let zip_data = zip_data.to_vec();
        
        // Use CPU pool for ZIP extraction (CPU-bound operation)
        let extracted_files = tokio::task::spawn_blocking(move || -> ConversionResult<Vec<ExtractedFile>> {
            use std::io::{Cursor, Read};
            use zip::ZipArchive;
            
            let cursor = Cursor::new(zip_data);
            let mut archive = ZipArchive::new(cursor)?;
            let mut files = Vec::new();
            
            for i in 0..archive.len() {
                let mut file = archive.by_index(i)?;
                
                // Skip directories
                if file.is_dir() {
                    continue;
                }
                
                // Read file contents
                let mut contents = Vec::new();
                file.read_to_end(&mut contents)?;
                
                files.push(ExtractedFile {
                    name: file.name().to_string(),
                    path: file.enclosed_name().map(|p| p.to_path_buf()),
                    data: contents,
                    size: file.size(),
                    compressed_size: file.compressed_size(),
                });
            }
            
            Ok(files)
        }).await??;
        
        tracing::info!("Extracted {} files from ZIP archive", extracted_files.len());
        
        Ok(extracted_files)
    }
    
    /// Get retrieval statistics
    pub async fn get_statistics(&self) -> AssetStats {
        AssetStats {
            assets_processed: 0, // TODO: Track processing stats
            downloads_successful: 0,
            downloads_failed: 0,
            cache_hits: 0,
            total_download_bytes: 0,
            processing_time_ms: 0,
        }
    }
    
    /// Compute BLAKE3 hash for content
    fn compute_hash(&self, data: &[u8]) -> String {
        let mut hasher = Hasher::new();
        hasher.update(data);
        hasher.finalize().to_hex().to_string()
    }
    
    /// Clear cache
    pub async fn clear_cache(&self) {
        self.cache.invalidate_all();
        tracing::info!("Asset cache cleared");
    }
}

impl Clone for AssetRetrievalPlugin {
    fn clone(&self) -> Self {
        Self {
            context: Arc::clone(&self.context),
            cache: self.cache.clone(),
            config: self.config.clone(),
        }
    }
}

/// Result of asset download operation
#[derive(Debug)]
pub enum AssetResult {
    Success {
        url: String,
        data: Vec<u8>, 
        size_bytes: u64,
    },
    Failed {
        url: String,
        error: String,
    },
}

/// Extracted file from ZIP archive
#[derive(Debug)]
pub struct ExtractedFile {
    pub name: String,
    pub path: Option<PathBuf>,
    pub data: Vec<u8>,
    pub size: u64,
    pub compressed_size: u64,
}

// TODO: Auto-register plugin with inventory once PluginInfo structure is defined
// inventory::submit! {
//     ttrpg_core::plugin_framework::PluginInfo {
//         id: "asset_retrieval_plugin",
//         name: "AssetRetrievalPlugin", 
//         version: "1.0.0",
//         description: "HTTP download, caching, and ZIP extraction",
//         plugin_type: crate::orchestration::PluginType::Processing,
//         dependencies: &[],
//         factory: || Box::new(AssetRetrievalPlugin::new().expect("Failed to create AssetRetrievalPlugin")),
//     }
// }
