//! ReferenceTrackingPlugin - Cross-entity asset reference tracking and validation
//!
//! This plugin tracks asset references across different entities, validates reference
//! integrity, and provides reference mapping for asset dependencies.

use crate::shared::AssetExecutionContext;
use ttrpg_core::{
    error::ConversionResult,
    plugin_framework::types::AssetInfo,
};
use std::sync::Arc;
use std::collections::{HashMap, HashSet, BTreeMap};
use dashmap::DashMap;
use rayon::prelude::*;

/// ReferenceTrackingPlugin for cross-entity asset reference management
pub struct ReferenceTrackingPlugin {
    /// Shared execution context
    context: Arc<AssetExecutionContext>,
    /// Asset reference registry
    reference_registry: Arc<DashMap<String, AssetReferenceInfo>>,
    /// Entity-to-asset mapping
    entity_asset_map: Arc<DashMap<String, HashSet<String>>>,
    /// Asset-to-entity reverse mapping
    asset_entity_map: Arc<DashMap<String, HashSet<String>>>,
    /// Configuration for reference tracking
    config: ReferenceTrackingConfig,
}

/// Configuration for reference tracking operations
#[derive(Debug, Clone)]
pub struct ReferenceTrackingConfig {
    /// Enable reference validation
    pub enable_validation: bool,
    /// Track unused assets
    pub track_unused_assets: bool,
    /// Enable dependency resolution
    pub enable_dependency_resolution: bool,
    /// Maximum reference depth for circular dependency detection
    pub max_reference_depth: usize,
}

impl Default for ReferenceTrackingConfig {
    fn default() -> Self {
        Self {
            enable_validation: true,
            track_unused_assets: true,
            enable_dependency_resolution: true,
            max_reference_depth: 10,
        }
    }
}

/// Asset reference information
#[derive(Debug, Clone)]
pub struct AssetReferenceInfo {
    /// Asset identifier
    pub asset_id: String,
    /// Asset path or URL
    pub asset_path: String,
    /// Entities that reference this asset
    pub referencing_entities: HashSet<String>,
    /// Reference type classification
    pub reference_types: HashSet<ReferenceType>,
    /// Reference count
    pub reference_count: usize,
    /// Last validation timestamp
    pub last_validated: Option<std::time::SystemTime>,
    /// Validation status
    pub validation_status: ValidationStatus,
}

/// Types of asset references
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ReferenceType {
    /// Character token/avatar
    CharacterToken,
    /// Scene background
    Background,
    /// Tile asset
    Tile,
    /// Journal/handout image
    Journal,
    /// Audio file
    Audio,
    /// Template/macro asset
    Template,
    /// Custom/unknown reference
    Custom(String),
}

/// Validation status for asset references
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    /// Reference is valid and accessible
    Valid,
    /// Reference is missing/broken
    Missing,
    /// Reference is inaccessible
    Inaccessible,
    /// Reference has not been validated
    Unvalidated,
    /// Validation failed with error
    Error(String),
}

/// Reference tracking result for processed assets
#[derive(Debug, Clone)]
pub struct TrackedAssetReferences {
    /// Total assets tracked
    pub total_assets: usize,
    /// Valid references
    pub valid_references: usize,
    /// Missing references
    pub missing_references: usize,
    /// Unused assets
    pub unused_assets: Vec<String>,
    /// Reference dependency graph
    pub dependency_graph: HashMap<String, Vec<String>>,
    /// Circular dependencies detected
    pub circular_dependencies: Vec<Vec<String>>,
    /// Reference statistics by type
    pub reference_type_stats: HashMap<ReferenceType, usize>,
}

impl ReferenceTrackingPlugin {
    /// Create new reference tracking plugin
    pub fn new() -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let config = ReferenceTrackingConfig::default();
        let reference_registry = Arc::new(DashMap::new());
        let entity_asset_map = Arc::new(DashMap::new());
        let asset_entity_map = Arc::new(DashMap::new());
        
        Ok(Self {
            context,
            reference_registry,
            entity_asset_map,
            asset_entity_map,
            config,
        })
    }
    
    /// Create plugin with custom configuration
    pub fn with_config(config: ReferenceTrackingConfig) -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let reference_registry = Arc::new(DashMap::new());
        let entity_asset_map = Arc::new(DashMap::new());
        let asset_entity_map = Arc::new(DashMap::new());
        
        Ok(Self {
            context,
            reference_registry,
            entity_asset_map,
            asset_entity_map,
            config,
        })
    }
    
    /// Track asset references from campaign data
    pub async fn track_references(&self, campaign_data: &serde_json::Value, assets: &[AssetInfo]) -> ConversionResult<TrackedAssetReferences> {
        tracing::info!("Tracking asset references for {} assets", assets.len());
        
        // Build asset ID to path mapping
        let asset_map: HashMap<String, String> = assets.iter()
            .enumerate()
            .map(|(i, asset)| {
                let asset_id = asset.file_name
                    .clone()
                    .unwrap_or_else(|| format!("asset_{}", i));
                let asset_path = asset.source_url
                    .clone()
                    .unwrap_or_else(|| format!("local://{}", asset_id));
                (asset_id, asset_path)
            })
            .collect();
        
        // Extract references from campaign data
        let references = self.extract_references_from_campaign(campaign_data, &asset_map).await?;
        
        // Update reference registry
        for (asset_id, ref_info) in references {
            self.reference_registry.insert(asset_id.clone(), ref_info.clone());
            
            // Update entity-asset mappings
            for entity_id in &ref_info.referencing_entities {
                self.entity_asset_map
                    .entry(entity_id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(asset_id.clone());
                
                self.asset_entity_map
                    .entry(asset_id.clone())
                    .or_insert_with(HashSet::new)
                    .insert(entity_id.clone());
            }
        }
        
        // Validate references if enabled
        if self.config.enable_validation {
            self.validate_all_references(assets).await?;
        }
        
        // Generate tracking results
        self.generate_tracking_results(&asset_map).await
    }
    
    /// Extract asset references from campaign JSON data
    async fn extract_references_from_campaign(
        &self, 
        campaign_data: &serde_json::Value, 
        asset_map: &HashMap<String, String>
    ) -> ConversionResult<HashMap<String, AssetReferenceInfo>> {
        let mut references = HashMap::new();
        
        // Extract from different campaign sections
        if let Some(objects) = campaign_data.as_object() {
            // Process characters
            if let Some(characters) = objects.get("characters").and_then(|v| v.as_array()) {
                self.extract_references_from_entities(
                    characters, 
                    "character", 
                    ReferenceType::CharacterToken, 
                    asset_map, 
                    &mut references
                ).await?;
            }
            
            // Process scenes/pages
            if let Some(pages) = objects.get("pages").and_then(|v| v.as_array()) {
                self.extract_references_from_entities(
                    pages, 
                    "page", 
                    ReferenceType::Background, 
                    asset_map, 
                    &mut references
                ).await?;
            }
            
            // Process handouts
            if let Some(handouts) = objects.get("handouts").and_then(|v| v.as_array()) {
                self.extract_references_from_entities(
                    handouts, 
                    "handout", 
                    ReferenceType::Journal, 
                    asset_map, 
                    &mut references
                ).await?;
            }
            
            // Process jukebox/audio
            if let Some(jukebox) = objects.get("jukebox").and_then(|v| v.as_array()) {
                self.extract_references_from_entities(
                    jukebox, 
                    "audio", 
                    ReferenceType::Audio, 
                    asset_map, 
                    &mut references
                ).await?;
            }
        }
        
        Ok(references)
    }
    
    /// Extract references from specific entity array
    async fn extract_references_from_entities(
        &self,
        entities: &[serde_json::Value],
        entity_type: &str,
        default_ref_type: ReferenceType,
        asset_map: &HashMap<String, String>,
        references: &mut HashMap<String, AssetReferenceInfo>,
    ) -> ConversionResult<()> {
        for (i, entity) in entities.iter().enumerate() {
            let entity_id = entity.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| entity.get("_id").and_then(|v| v.as_str()).unwrap_or(""))
                .to_string();
            
            if entity_id.is_empty() {
                continue;
            }
            
            // Extract asset references from entity
            let asset_refs = self.extract_asset_references_from_entity(entity, &default_ref_type)?;
            
            for asset_ref in asset_refs {
                // Try to match asset reference to known assets
                if let Some(asset_path) = asset_map.get(&asset_ref.asset_id) {
                    let ref_info = references.entry(asset_ref.asset_id.clone())
                        .or_insert_with(|| AssetReferenceInfo {
                            asset_id: asset_ref.asset_id.clone(),
                            asset_path: asset_path.clone(),
                            referencing_entities: HashSet::new(),
                            reference_types: HashSet::new(),
                            reference_count: 0,
                            last_validated: None,
                            validation_status: ValidationStatus::Unvalidated,
                        });
                    
                    ref_info.referencing_entities.insert(entity_id.clone());
                    ref_info.reference_types.insert(asset_ref.reference_type);
                    ref_info.reference_count += 1;
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract asset references from single entity
    fn extract_asset_references_from_entity(
        &self, 
        entity: &serde_json::Value, 
        default_ref_type: &ReferenceType
    ) -> ConversionResult<Vec<AssetReference>> {
        let mut refs = Vec::new();
        
        // Common asset reference fields
        let asset_fields = [
            ("avatar", ReferenceType::CharacterToken),
            ("imgsrc", ReferenceType::CharacterToken),
            ("token_imgsrc", ReferenceType::CharacterToken),
            ("background_image", ReferenceType::Background),
            ("thumbnail", ReferenceType::Background),
            ("image", ReferenceType::Journal),
            ("notes", ReferenceType::Journal),
            ("playing", ReferenceType::Audio),
            ("track_id", ReferenceType::Audio),
        ];
        
        if let Some(obj) = entity.as_object() {
            for (field_name, ref_type) in &asset_fields {
                if let Some(value) = obj.get(*field_name) {
                    if let Some(asset_url) = value.as_str() {
                        if !asset_url.is_empty() && asset_url.starts_with("http") {
                            // Extract asset ID from URL
                            let asset_id = self.extract_asset_id_from_url(asset_url);
                            refs.push(AssetReference {
                                asset_id,
                                reference_type: ref_type.clone(),
                            });
                        }
                    }
                }
            }
            
            // Look for nested asset references
            self.extract_nested_references(obj, &mut refs, default_ref_type)?;
        }
        
        Ok(refs)
    }
    
    /// Extract nested asset references from complex objects
    fn extract_nested_references(
        &self,
        obj: &serde_json::Map<String, serde_json::Value>,
        refs: &mut Vec<AssetReference>,
        default_ref_type: &ReferenceType,
    ) -> ConversionResult<()> {
        // Recursively search for URL-like strings
        for (key, value) in obj {
            match value {
                serde_json::Value::String(s) => {
                    if s.starts_with("http") && (s.contains("image") || s.contains("audio")) {
                        let asset_id = self.extract_asset_id_from_url(s);
                        let ref_type = if s.contains("audio") {
                            ReferenceType::Audio
                        } else {
                            default_ref_type.clone()
                        };
                        refs.push(AssetReference {
                            asset_id,
                            reference_type: ref_type,
                        });
                    }
                },
                serde_json::Value::Object(nested_obj) => {
                    self.extract_nested_references(nested_obj, refs, default_ref_type)?;
                },
                serde_json::Value::Array(arr) => {
                    for item in arr {
                        if let serde_json::Value::Object(nested_obj) = item {
                            self.extract_nested_references(nested_obj, refs, default_ref_type)?;
                        }
                    }
                },
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Extract asset ID from URL
    fn extract_asset_id_from_url(&self, url: &str) -> String {
        // Extract filename from URL
        url.split('/')
            .last()
            .unwrap_or(url)
            .split('?')
            .next()
            .unwrap_or(url)
            .to_string()
    }
    
    /// Validate all tracked references
    async fn validate_all_references(&self, assets: &[AssetInfo]) -> ConversionResult<()> {
        let asset_ids: HashSet<String> = assets.iter()
            .enumerate()
            .map(|(i, asset)| {
                asset.file_name
                    .clone()
                    .unwrap_or_else(|| format!("asset_{}", i))
            })
            .collect();
        
        // Validate each reference in parallel
        let validation_tasks: Vec<_> = self.reference_registry.iter()
            .map(|entry| {
                let asset_id = entry.key().clone();
                let is_valid = asset_ids.contains(&asset_id);
                (asset_id, is_valid)
            })
            .collect();
        
        for (asset_id, is_valid) in validation_tasks {
            if let Some(mut ref_info) = self.reference_registry.get_mut(&asset_id) {
                ref_info.validation_status = if is_valid {
                    ValidationStatus::Valid
                } else {
                    ValidationStatus::Missing
                };
                ref_info.last_validated = Some(std::time::SystemTime::now());
            }
        }
        
        Ok(())
    }
    
    /// Generate tracking results summary
    async fn generate_tracking_results(&self, asset_map: &HashMap<String, String>) -> ConversionResult<TrackedAssetReferences> {
        let total_assets = asset_map.len();
        let mut valid_references = 0;
        let mut missing_references = 0;
        let mut reference_type_stats = HashMap::new();
        let mut unused_assets = Vec::new();
        
        // Collect statistics
        for entry in self.reference_registry.iter() {
            match entry.validation_status {
                ValidationStatus::Valid => valid_references += 1,
                ValidationStatus::Missing => missing_references += 1,
                _ => {}
            }
            
            for ref_type in &entry.reference_types {
                *reference_type_stats.entry(ref_type.clone()).or_insert(0) += 1;
            }
        }
        
        // Find unused assets
        if self.config.track_unused_assets {
            for asset_id in asset_map.keys() {
                if !self.reference_registry.contains_key(asset_id) {
                    unused_assets.push(asset_id.clone());
                }
            }
        }
        
        // Build dependency graph
        let dependency_graph = self.build_dependency_graph().await?;
        
        // Detect circular dependencies
        let circular_dependencies = self.detect_circular_dependencies(&dependency_graph)?;
        
        Ok(TrackedAssetReferences {
            total_assets,
            valid_references,
            missing_references,
            unused_assets,
            dependency_graph,
            circular_dependencies,
            reference_type_stats,
        })
    }
    
    /// Build asset dependency graph
    async fn build_dependency_graph(&self) -> ConversionResult<HashMap<String, Vec<String>>> {
        let mut graph = HashMap::new();
        
        // Build entity -> asset dependencies
        for entry in self.entity_asset_map.iter() {
            let entity_id = entry.key().clone();
            let asset_ids: Vec<String> = entry.value().iter().cloned().collect();
            graph.insert(entity_id, asset_ids);
        }
        
        Ok(graph)
    }
    
    /// Detect circular dependencies in reference graph
    fn detect_circular_dependencies(&self, graph: &HashMap<String, Vec<String>>) -> ConversionResult<Vec<Vec<String>>> {
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node in graph.keys() {
            if !visited.contains(node) {
                if let Some(cycle) = self.dfs_detect_cycle(node, graph, &mut visited, &mut rec_stack, Vec::new()) {
                    circular_deps.push(cycle);
                }
            }
        }
        
        Ok(circular_deps)
    }
    
    /// DFS to detect cycles in dependency graph
    fn dfs_detect_cycle(
        &self,
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        mut path: Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if let Some(cycle) = self.dfs_detect_cycle(neighbor, graph, visited, rec_stack, path.clone()) {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    let cycle_start = path.iter().position(|x| x == neighbor).unwrap_or(0);
                    return Some(path[cycle_start..].to_vec());
                }
            }
        }
        
        rec_stack.remove(node);
        None
    }
    
    /// Get reference information for specific asset
    pub fn get_asset_references(&self, asset_id: &str) -> Option<AssetReferenceInfo> {
        self.reference_registry.get(asset_id).map(|entry| entry.clone())
    }
    
    /// Get all references for specific entity
    pub fn get_entity_references(&self, entity_id: &str) -> Vec<String> {
        self.entity_asset_map
            .get(entity_id)
            .map(|entry| entry.iter().cloned().collect())
            .unwrap_or_default()
    }
    
    /// Clear reference tracking data
    pub fn clear_tracking_data(&self) {
        self.reference_registry.clear();
        self.entity_asset_map.clear();
        self.asset_entity_map.clear();
        tracing::info!("Reference tracking data cleared");
    }
}

impl Clone for ReferenceTrackingPlugin {
    fn clone(&self) -> Self {
        Self {
            context: Arc::clone(&self.context),
            reference_registry: Arc::clone(&self.reference_registry),
            entity_asset_map: Arc::clone(&self.entity_asset_map),
            asset_entity_map: Arc::clone(&self.asset_entity_map),
            config: self.config.clone(),
        }
    }
}

/// Asset reference extracted from entity
#[derive(Debug, Clone)]
struct AssetReference {
    /// Asset identifier
    asset_id: String,
    /// Reference type
    reference_type: ReferenceType,
}

// TODO: Auto-register plugin with inventory once PluginInfo structure is defined
// inventory::submit! {
//     ttrpg_core::plugin_framework::PluginInfo {
//         id: "reference_tracking_plugin",
//         name: "ReferenceTrackingPlugin",
//         version: "1.0.0",
//         description: "Cross-entity asset reference tracking and validation",
//         plugin_type: ttrpg_core::plugin_framework::PluginType::Processing,
//         dependencies: &["scene_processing_plugin"],
//         factory: || Box::new(ReferenceTrackingPlugin::new().expect("Failed to create ReferenceTrackingPlugin")),
//     }
// }
