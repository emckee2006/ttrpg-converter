//! AssetResolutionPlugin - Deduplication, unique naming, content hashing
//!
//! This plugin handles asset deduplication through content hashing, generates
//! unique names to avoid conflicts, and resolves asset references across entities.

use crate::shared::AssetExecutionContext;
use ttrpg_core::{
    error::ConversionResult,
    plugin_framework::types::AssetInfo,
};
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use blake3::Hasher;
use dashmap::DashMap;
use rayon::prelude::*;

/// AssetResolutionPlugin for deduplication and unique naming
pub struct AssetResolutionPlugin {
    /// Shared execution context
    context: Arc<AssetExecutionContext>,
    /// Hash-to-asset mapping for deduplication
    hash_registry: Arc<DashMap<String, ResolvedAsset>>,
    /// Configuration for resolution operations
    config: ResolutionConfig,
}

/// Configuration for asset resolution
#[derive(Debug, Clone)]
pub struct ResolutionConfig {
    /// Enable content-based deduplication
    pub enable_deduplication: bool,
    /// Naming strategy for assets
    pub naming_strategy: NamingStrategy,
    /// Hash algorithm to use
    pub hash_algorithm: HashAlgorithm,
    /// Minimum file size for deduplication (bytes)
    pub min_deduplication_size: u64,
}

/// Naming strategies for resolved assets
#[derive(Debug, Clone)]
pub enum NamingStrategy {
    /// Use original names with conflict resolution
    PreserveOriginal,
    /// Use content hash as filename
    ContentHash,
    /// Use descriptive names based on content
    Descriptive,
    /// Use sequential numbering
    Sequential,
}

/// Hash algorithms for content identification
#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    /// BLAKE3 (default, fastest)
    Blake3,
    /// SHA256 (widely compatible)
    Sha256,
}

impl Default for ResolutionConfig {
    fn default() -> Self {
        Self {
            enable_deduplication: true,
            naming_strategy: NamingStrategy::PreserveOriginal,
            hash_algorithm: HashAlgorithm::Blake3,
            min_deduplication_size: 1024, // 1KB minimum
        }
    }
}

/// Resolved asset with deduplication information
#[derive(Debug, Clone)]
pub struct ResolvedAsset {
    /// Original asset information
    pub original: AssetInfo,
    /// Content hash for deduplication
    pub content_hash: String,
    /// Resolved unique name
    pub resolved_name: String,
    /// Number of references to this asset
    pub reference_count: usize,
    /// Original URLs that resolve to this asset
    pub source_urls: Vec<String>,
    /// Asset classification
    pub classification: AssetClassification,
}

/// Asset classification for intelligent handling
#[derive(Debug, Clone, PartialEq)]
pub enum AssetClassification {
    /// Character token or avatar
    Token,
    /// Background image
    Background,
    /// Tile for scene construction
    Tile,
    /// Journal or handout image
    Journal,
    /// Audio file
    Audio,
    /// Generic asset
    Generic,
}

impl AssetResolutionPlugin {
    /// Create new asset resolution plugin
    pub fn new() -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let config = ResolutionConfig::default();
        let hash_registry = Arc::new(DashMap::new());
        
        Ok(Self {
            context,
            hash_registry,
            config,
        })
    }
    
    /// Create plugin with custom configuration
    pub fn with_config(config: ResolutionConfig) -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let hash_registry = Arc::new(DashMap::new());
        
        Ok(Self {
            context,
            hash_registry,
            config,
        })
    }
    
    /// Resolve duplicates across multiple assets with parallel processing
    pub async fn resolve_duplicates(&self, assets: Vec<AssetInfo>) -> ConversionResult<Vec<ResolvedAsset>> {
        if assets.is_empty() {
            return Ok(vec![]);
        }
        
        tracing::info!("Resolving duplicates for {} assets", assets.len());
        
        // Use CPU pool for parallel hash computation
        let assets_clone = assets.clone();
        let config = self.config.clone();
        
        let hashed_assets = tokio::task::spawn_blocking(move || -> ConversionResult<Vec<(AssetInfo, String)>> {
            assets_clone.par_iter()
                .map(|asset| {
                    let hash = Self::compute_content_hash(&asset.data, &config.hash_algorithm)?;
                    Ok((asset.clone(), hash))
                })
                .collect::<ConversionResult<Vec<_>>>()
        }).await??;
        
        // Group assets by content hash
        let mut hash_groups: HashMap<String, Vec<AssetInfo>> = HashMap::new();
        for (asset, hash) in hashed_assets {
            hash_groups.entry(hash).or_default().push(asset);
        }
        
        // Resolve each group
        let mut resolved_assets = Vec::new();
        for (content_hash, group_assets) in hash_groups {
            let resolved = self.resolve_asset_group(content_hash, group_assets).await?;
            resolved_assets.push(resolved);
        }
        
        tracing::info!("Resolved {} unique assets from {} originals ({:.1}% deduplication)", 
                      resolved_assets.len(), 
                      assets.len(),
                      (1.0 - resolved_assets.len() as f64 / assets.len() as f64) * 100.0);
        
        Ok(resolved_assets)
    }
    
    /// Resolve a group of assets with the same content hash
    async fn resolve_asset_group(&self, content_hash: String, assets: Vec<AssetInfo>) -> ConversionResult<ResolvedAsset> {
        // Check if we already have this asset resolved
        if let Some(existing) = self.hash_registry.get(&content_hash) {
            let mut updated = existing.clone();
            updated.reference_count += assets.len();
            
            // Add new source URLs
            for asset in &assets {
                if let Some(url) = &asset.source_url {
                    if !updated.source_urls.contains(url) {
                        updated.source_urls.push(url.clone());
                    }
                }
            }
            
            self.hash_registry.insert(content_hash, updated.clone());
            return Ok(updated);
        }
        
        // Take the first asset as the primary one
        let primary_asset = assets[0].clone();
        
        // Classify the asset
        let classification = self.classify_asset(&primary_asset);
        
        // Generate unique name
        let resolved_name = self.generate_unique_name(&primary_asset, &content_hash, &classification);
        
        // Collect all source URLs
        let source_urls: Vec<String> = assets.iter()
            .filter_map(|a| a.source_url.as_ref())
            .cloned()
            .collect();
        
        let resolved = ResolvedAsset {
            original: primary_asset,
            content_hash: content_hash.clone(),
            resolved_name,
            reference_count: assets.len(),
            source_urls,
            classification,
        };
        
        // Cache the resolved asset
        self.hash_registry.insert(content_hash, resolved.clone());
        
        Ok(resolved)
    }
    
    /// Classify asset based on content and metadata
    fn classify_asset(&self, asset: &AssetInfo) -> AssetClassification {
        // Check file extension
        if let Some(name) = &asset.file_name {
            let name_lower = name.to_lowercase();
            
            // Audio files
            if name_lower.ends_with(".mp3") || name_lower.ends_with(".wav") || 
               name_lower.ends_with(".ogg") || name_lower.ends_with(".flac") {
                return AssetClassification::Audio;
            }
            
            // Token indicators
            if name_lower.contains("token") || name_lower.contains("character") || 
               name_lower.contains("avatar") || name_lower.contains("portrait") {
                return AssetClassification::Token;
            }
            
            // Background indicators
            if name_lower.contains("background") || name_lower.contains("map") ||
               name_lower.contains("scene") {
                return AssetClassification::Background;
            }
            
            // Tile indicators
            if name_lower.contains("tile") || name_lower.contains("texture") {
                return AssetClassification::Tile;
            }
            
            // Journal indicators
            if name_lower.contains("handout") || name_lower.contains("journal") ||
               name_lower.contains("note") {
                return AssetClassification::Journal;
            }
        }
        
        // Check asset dimensions for image classification
        if let (Some(width), Some(height)) = (asset.width, asset.height) {
            let aspect_ratio = width as f64 / height as f64;
            
            // Square images are likely tokens
            if (aspect_ratio - 1.0).abs() < 0.1 && width <= 512 && height <= 512 {
                return AssetClassification::Token;
            }
            
            // Large rectangular images are likely backgrounds
            if width > 1000 || height > 1000 {
                return AssetClassification::Background;
            }
            
            // Small rectangular images are likely tiles
            if width <= 256 && height <= 256 {
                return AssetClassification::Tile;
            }
        }
        
        AssetClassification::Generic
    }
    
    /// Generate unique name for asset
    fn generate_unique_name(&self, asset: &AssetInfo, content_hash: &str, classification: &AssetClassification) -> String {
        match self.config.naming_strategy {
            NamingStrategy::PreserveOriginal => {
                // Use original name with hash suffix if conflicts exist
                if let Some(name) = &asset.file_name {
                    if self.is_name_unique(name) {
                        name.clone()
                    } else {
                        let hash_suffix = &content_hash[..8];
                        if let Some(dot_pos) = name.rfind('.') {
                            format!("{}_{}{}", &name[..dot_pos], hash_suffix, &name[dot_pos..])
                        } else {
                            format!("{}_{}", name, hash_suffix)
                        }
                    }
                } else {
                    format!("asset_{}", &content_hash[..12])
                }
            },
            NamingStrategy::ContentHash => {
                let extension = asset.file_name
                    .as_ref()
                    .and_then(|name| name.rfind('.').map(|i| &name[i..]))
                    .unwrap_or("");
                format!("{}{}", content_hash, extension)
            },
            NamingStrategy::Descriptive => {
                let prefix = match classification {
                    AssetClassification::Token => "token",
                    AssetClassification::Background => "background", 
                    AssetClassification::Tile => "tile",
                    AssetClassification::Journal => "journal",
                    AssetClassification::Audio => "audio",
                    AssetClassification::Generic => "asset",
                };
                
                let extension = asset.file_name
                    .as_ref()
                    .and_then(|name| name.rfind('.').map(|i| &name[i..]))
                    .unwrap_or("");
                    
                format!("{}_{}{}", prefix, &content_hash[..8], extension)
            },
            NamingStrategy::Sequential => {
                // TODO: Implement sequential naming with counters
                format!("asset_{}", &content_hash[..8])
            },
        }
    }
    
    /// Check if a name is unique in the registry
    fn is_name_unique(&self, name: &str) -> bool {
        !self.hash_registry.iter()
            .any(|entry| entry.value().resolved_name == name)
    }
    
    /// Compute content hash for asset data
    fn compute_content_hash(data: &[u8], algorithm: &HashAlgorithm) -> ConversionResult<String> {
        match algorithm {
            HashAlgorithm::Blake3 => {
                let mut hasher = Hasher::new();
                hasher.update(data);
                Ok(hasher.finalize().to_hex().to_string())
            },
            HashAlgorithm::Sha256 => {
                use sha2::{Sha256, Digest};
                let mut hasher = Sha256::new(); 
                hasher.update(data);
                Ok(format!("{:x}", hasher.finalize()))
            },
        }
    }
    
    /// Get asset by content hash
    pub fn get_asset_by_hash(&self, content_hash: &str) -> Option<ResolvedAsset> {
        self.hash_registry.get(content_hash).map(|entry| entry.clone())
    }
    
    /// Get resolution statistics
    pub fn get_statistics(&self) -> ResolutionStatistics {
        let total_assets = self.hash_registry.len();
        let total_references: usize = self.hash_registry.iter()
            .map(|entry| entry.reference_count)
            .sum();
        
        ResolutionStatistics {
            unique_assets: total_assets,
            total_references,
            deduplication_ratio: if total_references > 0 {
                1.0 - (total_assets as f64 / total_references as f64)
            } else {
                0.0
            },
            classification_counts: self.get_classification_counts(),
        }
    }
    
    /// Get counts by asset classification
    fn get_classification_counts(&self) -> HashMap<AssetClassification, usize> {
        let mut counts = HashMap::new();
        
        for entry in self.hash_registry.iter() {
            *counts.entry(entry.classification.clone()).or_insert(0) += 1;
        }
        
        counts
    }
    
    /// Clear resolution registry
    pub fn clear_registry(&self) {
        self.hash_registry.clear();
        tracing::info!("Asset resolution registry cleared");
    }
}

impl Clone for AssetResolutionPlugin {
    fn clone(&self) -> Self {
        Self {
            context: Arc::clone(&self.context),
            hash_registry: Arc::clone(&self.hash_registry),
            config: self.config.clone(),
        }
    }
}

/// Statistics for asset resolution operations
#[derive(Debug)]
pub struct ResolutionStatistics {
    /// Number of unique assets after deduplication
    pub unique_assets: usize,
    /// Total number of asset references
    pub total_references: usize,
    /// Deduplication ratio (0.0 = no deduplication, 1.0 = maximum deduplication)
    pub deduplication_ratio: f64,
    /// Count of assets by classification
    pub classification_counts: HashMap<AssetClassification, usize>,
}

// TODO: Auto-register plugin with inventory once PluginInfo structure is defined
// inventory::submit! {
//     ttrpg_core::plugin_framework::PluginInfo {
//         id: "asset_resolution_plugin",
//         name: "AssetResolutionPlugin",
//         version: "1.0.0", 
//         description: "Deduplication, unique naming, and content hashing",
//         plugin_type: ttrpg_core::plugin_framework::PluginType::Processing,
//         dependencies: &["asset_retrieval_plugin"],
//         factory: || Box::new(AssetResolutionPlugin::new().expect("Failed to create AssetResolutionPlugin")),
//     }
// }
