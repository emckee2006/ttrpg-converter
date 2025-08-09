# M2: Core Engine Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 2 OVERVIEW** (MASSIVELY EXPANDED SCOPE)
**Duration**: 4 weeks | **Total Points**: 55 | **Priority**: 🔥 HIGH

Advanced core conversion engine with Roll20 parser, sophisticated wall processing, intelligent asset processing, and optimized Foundry output generation.

### 🚨 **EXPANDED SCOPE BASED ON PREVIOUS R20CONVERTER ANALYSIS**
Major scope expansion includes previously missing critical features:
- **Advanced Wall Processing**: Minimum wall length filtering, angle optimization, auto-door detection
- **Intelligent Asset Processing**: Asset reference extraction, optimization, background vs tile classification
- **Sophisticated Map Processing**: Wall cleanup for cave maps, boundary wall addition
- **Professional Parsing**: Enhanced error handling, malformed data recovery, performance optimization

### 📐 **PROFESSIONAL GEOMETRY PROCESSING**
Eliminate reinvented wheels with professional libraries:
- `geo` - Computational geometry operations
- `rstar` - R-tree spatial indexing for wall optimization
- `lyon` - 2D graphics tessellation for complex shapes

### 🧪 **TESTING REQUIREMENTS** (Updated 2024-08-07)
**Every M2 task must include comprehensive testing before being marked complete:**
- ✅ **Unit Tests** - Individual function testing (>80% coverage)
- ✅ **Integration Tests** - End-to-end conversion testing with real data
- ✅ **Property Tests** - Using `proptest` for data transformation validation
- ✅ **Benchmarks** - Performance measurement for parsing and conversion
- ✅ **Documentation Tests** - All examples in docs must work

See [TESTING_FRAMEWORK.md](./TESTING_FRAMEWORK.md) for detailed requirements.

---

## **T2.1: Roll20 JSON Parser Implementation**
**Duration**: 4 days | **Points**: 10 | **Priority**: 🔥 HIGH  
**Dependencies**: M1 Complete

### **Implementation Steps for Junior Developer**

**Step 1: Set Up ttrpg-formats Crate**
```bash
cd crates\ttrpg-formats
```

Update `Cargo.toml`:
```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
ttrpg-core = { path = "../ttrpg-core" }
zip = { workspace = true }
tracing = { workspace = true }
```

**Step 2: Create Roll20 Entity Structures**
Create `src/roll20/entities.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Character {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub char_type: String,
    pub attribs: Vec<Roll20Attribute>,
    pub abilities: Vec<Roll20Ability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Page {
    pub id: String,
    pub name: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub showgrid: Option<bool>,
}
```

**Step 3: Implement Campaign Parser**
Create `src/roll20/parser.rs`:
```rust
pub struct Roll20Parser {
    strict_mode: bool,
}

impl Roll20Parser {
    pub fn parse_zip<P: AsRef<Path>>(&self, path: P) -> ConversionResult<Roll20Campaign> {
        // ZIP file parsing with campaign.json extraction
    }
}
```

**Acceptance Criteria (ENHANCED):**
- [ ] Parse complete Roll20 campaign.json files (>99% success rate)
- [ ] Handle ZIP file extraction with compression detection
- [ ] Support all entity types (characters, pages, graphics, handouts, etc.)
- [ ] Parse 50MB campaign in <2 seconds with parallel processing
- [ ] Professional error recovery for malformed JSON data
- [ ] Automatic asset reference extraction and classification
- [ ] Memory usage <200MB for large campaigns
- [ ] Comprehensive validation with detailed error reporting

---

## **T2.2: Advanced Wall Processing Engine (CRITICAL MISSING FEATURE)**
**Duration**: 6 days | **Points**: 15 | **Priority**: 🚨 CRITICAL
**Dependencies**: T2.1 Complete

This was completely missing from our original plan but essential based on previous R20Converter analysis!

### **Implementation Steps for Junior Developer**

**Step 1: Professional Geometry Processing Dependencies**
Update `ttrpg-formats/Cargo.toml`:
```toml
[dependencies]
# Professional geometry processing (eliminate reinvented wheels)
geo = "0.27"              # Computational geometry operations
rstar = "0.12"            # R-tree spatial indexing for wall optimization
lyon = "1.0"              # 2D graphics tessellation for complex shapes
kdtree = "0.6"            # K-d tree for spatial queries

# Mathematical operations
num-traits = "0.2"        # Numeric trait abstractions
approx = "0.5"            # Floating point comparisons
```

**Step 2: Advanced Wall Processing Engine**
Create `ttrpg-formats/src/roll20/wall_processor.rs`:
```rust
use geo::{Point, Line, Polygon, algorithm::euclidean_distance::EuclideanDistance};
use rstar::{RTree, AABB};
use std::collections::HashMap;

/// Advanced wall processing engine with optimization algorithms
pub struct WallProcessor {
    /// Minimum wall length threshold (pixels)
    min_wall_length: f64,
    /// Maximum angle difference for wall merging (degrees)
    max_wall_angle: f64,
    /// Door detection configuration
    door_config: DoorDetectionConfig,
    /// Spatial index for efficient wall queries
    spatial_index: Option<RTree<WallSegment>>,
}

#[derive(Debug, Clone)]
pub struct WallSegment {
    pub id: String,
    pub start: Point<f64>,
    pub end: Point<f64>,
    pub color: Option<String>,
    pub wall_type: WallType,
    pub restricts_movement: bool,
    pub blocks_vision: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WallType {
    Wall,
    Door,
    SecretDoor,
    Window,
    Terrain,
}

impl WallProcessor {
    /// Process and optimize walls with advanced algorithms
    pub fn process_walls(&mut self, walls: Vec<Roll20Wall>) -> ConversionResult<Vec<FoundryWall>> {
        // Phase 1: Convert Roll20 walls to internal representation
        let wall_segments = self.convert_to_segments(walls)?;
        
        // Phase 2: Build spatial index for efficient queries
        self.build_spatial_index(&wall_segments);
        
        // Phase 3: Remove tiny walls (noise reduction)
        let filtered_walls = self.filter_tiny_walls(wall_segments)?;
        
        // Phase 4: Detect and classify doors
        let walls_with_doors = self.detect_doors(filtered_walls)?;
        
        // Phase 5: Optimize wall angles and merge collinear segments
        let optimized_walls = self.optimize_wall_angles(walls_with_doors)?;
        
        // Phase 6: Add boundary walls if requested
        let final_walls = self.add_boundary_walls_if_needed(optimized_walls)?;
        
        // Phase 7: Convert to Foundry format
        let foundry_walls = self.convert_to_foundry_walls(final_walls)?;
        
        Ok(foundry_walls)
    }
    
    /// Optimize cave maps by reducing thousands of tiny walls to hundreds
    pub fn optimize_cave_walls(&mut self, walls: Vec<Roll20Wall>) -> ConversionResult<Vec<FoundryWall>> {
        info!("Starting cave wall optimization for {} walls", walls.len());
        
        // Convert to segments for processing
        let wall_segments = self.convert_to_segments(walls)?;
        
        // Build spatial index for efficient neighbor queries
        self.build_spatial_index(&wall_segments);
        
        // Phase 1: Cluster nearby wall segments
        let clusters = self.cluster_nearby_walls(&wall_segments, 5.0)?; // 5px clustering
        
        // Phase 2: Simplify each cluster using Douglas-Peucker algorithm
        let simplified_clusters = self.simplify_wall_clusters(clusters, 2.0)?; // 2px tolerance
        
        // Phase 3: Merge collinear segments within clusters
        let merged_walls = self.merge_collinear_in_clusters(simplified_clusters)?;
        
        // Phase 4: Convert back to Foundry walls
        let foundry_walls = self.convert_to_foundry_walls(merged_walls)?;
        
        info!("Cave optimization complete: {} walls → {} walls ({:.1}% reduction)", 
              wall_segments.len(), 
              foundry_walls.len(),
              (1.0 - foundry_walls.len() as f64 / wall_segments.len() as f64) * 100.0);
              
        Ok(foundry_walls)
    }
}
```

**Step 3: Door Detection System**
Advanced door detection based on colors and heuristics from previous R20Converter:
```rust
#[derive(Debug, Clone)]
pub struct DoorDetectionConfig {
    pub auto_detect_doors: bool,
    pub door_color: Option<String>,      // e.g., "#ff0000" for red doors
    pub secret_door_color: Option<String>, // e.g., "#00ff00" for green secret doors
    pub door_length_threshold: f64,      // Doors are usually shorter than walls
}

impl WallProcessor {
    /// Detect doors based on color and length heuristics
    fn detect_doors(&self, mut walls: Vec<WallSegment>) -> ConversionResult<Vec<WallSegment>> {
        if !self.door_config.auto_detect_doors {
            return Ok(walls);
        }
        
        let mut doors_detected = 0;
        let mut secret_doors_detected = 0;
        
        for wall in &mut walls {
            // Check door color detection
            if let Some(ref color) = wall.color {
                if let Some(ref door_color) = self.door_config.door_color {
                    if color.eq_ignore_ascii_case(door_color) {
                        wall.wall_type = WallType::Door;
                        doors_detected += 1;
                        continue;
                    }
                }
                
                if let Some(ref secret_color) = self.door_config.secret_door_color {
                    if color.eq_ignore_ascii_case(secret_color) {
                        wall.wall_type = WallType::SecretDoor;
                        secret_doors_detected += 1;
                        continue;
                    }
                }
            }
            
            // Check door length heuristics
            let length = wall.start.euclidean_distance(&wall.end);
            if length <= self.door_config.door_length_threshold {
                // Short segments are likely doors
                wall.wall_type = WallType::Door;
                doors_detected += 1;
            }
        }
        
        info!("Detected {} doors and {} secret doors", doors_detected, secret_doors_detected);
        
        Ok(walls)
    }
}
```

**Acceptance Criteria**:
- [ ] Advanced wall processing with minimum length filtering (35px default)
- [ ] Angle optimization with maximum wall angle threshold (30° default)
- [ ] Auto-door detection by color (#ff0000 red, #00ff00 secret)
- [ ] Cave map optimization: thousands → hundreds (>80% reduction)
- [ ] Spatial indexing for O(log n) wall neighbor queries
- [ ] Boundary wall addition around map edges
- [ ] Professional geometry operations using `geo` crate
- [ ] Comprehensive testing with cave map test data

---

## **T2.3: Entity Conversion Pipeline**
**Duration**: 3 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: T2.2 Complete

### **Implementation Steps**

**Step 1: Create Entity Framework in ttrpg-core**
```rust
pub trait Entity {
    fn validate(&self) -> ConversionResult<()>;
    fn convert(&self, context: &ConversionContext) -> ConversionResult<FoundryEntity>;
}

pub struct ConversionPipeline {
    thread_count: usize,
}

impl ConversionPipeline {
    pub fn convert_campaign(&self, campaign: Roll20Campaign) -> ConversionResult<FoundryCampaign> {
        // Parallel processing with rayon
    }
}
```

**Step 2: Implement Parallel Processing**
```bash
cd crates\ttrpg-core
```

Add rayon dependency and implement:
```rust
use rayon::prelude::*;

let converted_characters: Vec<_> = campaign.characters
    .par_iter()
    .map(|c| c.convert(context))
    .collect::<ConversionResult<Vec<_>>>()?;
```

**Acceptance Criteria**:
- [ ] 5-10x faster than Python version
- [ ] Parallel entity processing with rayon
- [ ] Memory-efficient for large campaigns
- [ ] Progress tracking support

---

## ---

## **T2.4: Intelligent Asset Processing Engine (CRITICAL MISSING FEATURE)**
**Duration**: 4 days | **Points**: 12 | **Priority**: 🚨 CRITICAL
**Dependencies**: T2.2, T2.3 Complete, M1.4 Complete (Professional Asset Service)

This integrates with our enhanced M1.4 RustAssetService for intelligent asset processing during conversion.

### **Implementation Steps for Junior Developer**

**Step 1: Asset Processing Integration**
Create `ttrpg-formats/src/roll20/asset_processor.rs`:
```rust
use ttrpg_assets::RustAssetService;
use std::sync::Arc;

/// Intelligent asset processor that integrates with conversion pipeline
pub struct AssetProcessor {
    asset_service: Arc<RustAssetService>,
    processing_config: AssetProcessingConfig,
}

#[derive(Debug, Clone)]
pub struct AssetProcessingConfig {
    pub optimize_during_conversion: bool,
    pub convert_to_webp: bool,
    pub generate_thumbnails: bool,
    pub classify_backgrounds_as_tiles: bool,
    pub use_original_urls: bool,
    pub max_asset_size_mb: u64,
}

impl AssetProcessor {
    /// Process all assets during campaign conversion
    pub async fn process_campaign_assets(&self, campaign: &Roll20Campaign) -> ConversionResult<AssetProcessingResult> {
        let mut results = AssetProcessingResult::new();
        
        // Phase 1: Extract and classify all asset references
        let asset_refs = self.extract_asset_references(campaign)?;
        info!("Found {} asset references for processing", asset_refs.len());
        
        // Phase 2: Process assets in parallel with intelligent optimization
        let processed_assets = self.process_assets_parallel(asset_refs).await?;
        
        // Phase 3: Apply asset classification rules
        let classified_assets = self.classify_assets(processed_assets)?;
        
        // Phase 4: Generate asset mappings for Foundry
        let foundry_mappings = self.generate_foundry_mappings(classified_assets)?;
        
        results.mappings = foundry_mappings;
        results.statistics = self.generate_processing_statistics(&results)?;
        
        Ok(results)
    }
    
    /// Classify assets as backgrounds vs tiles based on usage context
    fn classify_assets(&self, assets: Vec<ProcessedAsset>) -> ConversionResult<Vec<ClassifiedAsset>> {
        let mut classified = Vec::new();
        
        for asset in assets {
            let classification = if self.processing_config.classify_backgrounds_as_tiles {
                // Force all backgrounds to be tiles (--all-backgrounds-as-tiles)
                AssetClassification::Tile
            } else {
                // Intelligent classification based on context
                self.intelligent_asset_classification(&asset)?
            };
            
            classified.push(ClassifiedAsset {
                asset,
                classification,
                foundry_usage: self.determine_foundry_usage(&classification)?,
            });
        }
        
        Ok(classified)
    }
    
    /// Intelligent asset classification based on Roll20 usage context
    fn intelligent_asset_classification(&self, asset: &ProcessedAsset) -> ConversionResult<AssetClassification> {
        // Analyze asset usage in Roll20 context
        match asset.roll20_usage.as_str() {
            "page_background" => {
                // Page backgrounds can be either backgrounds or tiles depending on size and repetition
                if asset.dimensions.width > 2000 || asset.dimensions.height > 2000 {
                    Ok(AssetClassification::Background) // Large images are usually backgrounds
                } else {
                    Ok(AssetClassification::Tile) // Smaller images might be tiles
                }
            },
            "character_avatar" | "character_token" => Ok(AssetClassification::Token),
            "handout_image" => Ok(AssetClassification::Journal),
            "graphic_object" => {
                // Graphics can be tokens, tiles, or decorative elements
                if asset.file_name.contains("token") || asset.file_name.contains("character") {
                    Ok(AssetClassification::Token)
                } else {
                    Ok(AssetClassification::Tile)
                }
            },
            _ => Ok(AssetClassification::Generic),
        }
    }
    
    /// Process assets in parallel with intelligent optimization
    async fn process_assets_parallel(&self, asset_refs: Vec<AssetReference>) -> ConversionResult<Vec<ProcessedAsset>> {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(10)); // Limit concurrent downloads
        let mut tasks = Vec::new();
        
        for asset_ref in asset_refs {
            let asset_service = Arc::clone(&self.asset_service);
            let config = self.processing_config.clone();
            let permit = Arc::clone(&semaphore);
            
            let task = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                
                // Download and process asset using enhanced M1.4 RustAssetService
                let processed = asset_service.download_and_process_asset(
                    &asset_ref.url,
                    &asset_ref.asset_type,
                    &config
                ).await?;
                
                Ok::<ProcessedAsset, AssetError>(ProcessedAsset {
                    original_ref: asset_ref,
                    local_path: processed.local_path,
                    optimized_path: processed.optimized_path,
                    dimensions: processed.dimensions,
                    file_size: processed.file_size,
                    format: processed.format,
                    roll20_usage: processed.context,
                })
            });
            
            tasks.push(task);
        }
        
        // Wait for all processing to complete
        let results = futures::future::try_join_all(tasks).await?;
        let processed_assets: Result<Vec<_>, _> = results.into_iter().collect();
        
        Ok(processed_assets?)
    }
}
```

**Step 2: Asset Classification Engine**
Advanced asset classification based on context and heuristics:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AssetClassification {
    Background,     // Scene backgrounds
    Tile,          // Map tiles and terrain
    Token,         // Character and NPC tokens
    Journal,       // Handout images
    Drawing,       // Images as drawings (--images-as-drawings)
    Generic,       // Other assets
}

#[derive(Debug, Clone)]
pub struct ClassifiedAsset {
    pub asset: ProcessedAsset,
    pub classification: AssetClassification,
    pub foundry_usage: FoundryAssetUsage,
    pub optimization_applied: Vec<OptimizationType>,
}

impl AssetProcessor {
    /// Generate Foundry-specific asset mappings
    fn generate_foundry_mappings(&self, classified_assets: Vec<ClassifiedAsset>) -> ConversionResult<HashMap<String, FoundryAssetMapping>> {
        let mut mappings = HashMap::new();
        
        for classified_asset in classified_assets {
            let foundry_mapping = match classified_asset.classification {
                AssetClassification::Background => {
                    FoundryAssetMapping {
                        foundry_path: format!("worlds/converted-campaign/scenes/{}", classified_asset.asset.file_name),
                        asset_type: "background".to_string(),
                        usage_context: "scene_background".to_string(),
                        optimizations: classified_asset.optimization_applied.clone(),
                    }
                },
                AssetClassification::Token => {
                    FoundryAssetMapping {
                        foundry_path: format!("worlds/converted-campaign/tokens/{}", classified_asset.asset.file_name),
                        asset_type: "token".to_string(),
                        usage_context: "actor_token".to_string(),
                        optimizations: classified_asset.optimization_applied.clone(),
                    }
                },
                AssetClassification::Tile => {
                    FoundryAssetMapping {
                        foundry_path: format!("worlds/converted-campaign/tiles/{}", classified_asset.asset.file_name),
                        asset_type: "tile".to_string(),
                        usage_context: "scene_tile".to_string(),
                        optimizations: classified_asset.optimization_applied.clone(),
                    }
                },
                // ... other classifications
                _ => continue,
            };
            
            mappings.insert(classified_asset.asset.original_ref.url.clone(), foundry_mapping);
        }
        
        Ok(mappings)
    }
}
```

**Acceptance Criteria**:
- [ ] Intelligent asset classification (backgrounds vs tiles vs tokens)
- [ ] Parallel asset processing with configurable concurrency (10 default)
- [ ] Integration with enhanced M1.4 RustAssetService for professional caching
- [ ] Support for --all-backgrounds-as-tiles CLI option
- [ ] Support for --images-as-drawings CLI option
- [ ] Asset optimization during conversion (WebP, thumbnail generation)
- [ ] Asset path optimization and Foundry mapping generation
- [ ] Memory usage <1GB for 1000+ asset campaigns
- [ ] Processing statistics and optimization reporting

---

## **T2.5: Enhanced Foundry VTT Output Generation**
**Duration**: 4 days | **Points**: 10 | **Priority**: 🔥 HIGH
**Dependencies**: T2.2 Complete

### **Implementation Steps**

**Step 1: Create Foundry Entity Structures**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundryActor {
    pub _id: String,
    pub name: String,
    pub type: String,
    pub system: HashMap<String, Value>,
    pub items: Vec<FoundryItem>,
}
```

**Step 2: Implement Conversion Logic**
Map Roll20 entities to Foundry format:
- Characters → Actors with proper system data
- Pages → Scenes with walls and tokens  
- Graphics → Tokens with vision/lighting
- Handouts → Journal entries

**Step 3: Database Generation**
```rust
pub fn generate_foundry_world(campaign: FoundryCampaign) -> ConversionResult<()> {
    // Create world.json, actors.db, items.db, etc.
    // Generate optimized Foundry VTT world with advanced features
    Ok(())
}
```

**Acceptance Criteria (ENHANCED WITH ADVANCED FEATURES):**
- [ ] Generate valid Foundry VTT world.json with optimized wall structures
- [ ] Support all major entity types with intelligent asset processing
- [ ] Maintain data relationships and references with asset path optimization
- [ ] Advanced wall processing: minimum length filtering, angle optimization, auto-door detection
- [ ] Intelligent asset classification (backgrounds vs tiles vs tokens)
- [ ] Cave map optimization: reduce thousands of walls to hundreds
- [ ] Generate 10MB world with 1000+ walls in <5 seconds
- [ ] Validate output against Foundry schema with comprehensive error reporting
- [ ] Asset processing statistics and optimization reporting
- [ ] Memory usage <500MB for large campaigns with thousands of walls
- [ ] Support for all advanced CLI options from previous R20Converter

---

### **M2 Timeline (EXPANDED SCOPE):**
- **Week 1**: Roll20 Parser with error recovery + Professional geometry dependencies
- **Week 2**: Advanced Wall Processing Engine with cave optimization + Door detection  
- **Week 3**: Entity Conversion Pipeline + Intelligent Asset Processing integration
- **Week 4**: Enhanced Foundry Output + Comprehensive testing + Performance optimization

### **Performance Targets (BASED ON PREVIOUS R20CONVERTER):**
- [ ] Parse 50MB campaigns in <2 seconds
- [ ] Process 10,000+ walls with cave optimization in <10 seconds
- [ ] Wall reduction efficiency: thousands → hundreds (>80% reduction for cave maps)
- [ ] Asset processing: >100 assets in parallel with <1GB memory usage
- [ ] Door detection accuracy: >95% with color-based detection
- [ ] Foundry world generation: <5 seconds for complex campaigns
- [ ] Memory efficiency: <500MB peak usage for large campaigns

---

### **🎯 M2 COMPLETION CHECKLIST**
- [ ] T2.1: Roll20 JSON Parser with professional error recovery
- [ ] T2.2: Advanced Wall Processing Engine with cave optimization
- [ ] T2.3: Entity Conversion Pipeline with enhanced framework
- [ ] T2.4: Intelligent Asset Processing Engine integration
- [ ] T2.5: Enhanced Foundry Output Generation with advanced features
- [ ] Comprehensive testing suite covering all advanced features
- [ ] Performance benchmarking achieving all targets above
```rust
pub struct AssetDownloader {
    client: reqwest::Client,
    max_size: u64,
}

impl AssetDownloader {
    pub async fn download_asset(&self, url: &str) -> ConversionResult<Vec<u8>> {
        // HTTP download with size limits and retries
    }
}
```

**Step 3: Image Optimization**
```rust
pub fn optimize_image(data: &[u8]) -> ConversionResult<Vec<u8>> {
    // Resize, compress, format conversion
}
```

**Acceptance Criteria**:
- [ ] Parallel asset downloads
- [ ] Image optimization and resizing
- [ ] Size limits and validation
- [ ] Progress tracking integration
