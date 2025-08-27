# M6: Advanced Media Processing & Optimization - Junior Developer Implementation Guide

## 🎯 **MILESTONE 6 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 12 | **Priority**: 🟡 MEDIUM

Advanced media processing integrated into the Processing Plugin Architecture - AssetConversionPlugin, AssetResolutionPlugin, and SceneProcessingPlugin with shared execution contexts and parallel processing capabilities.

### 🧪 **TESTING REQUIREMENTS**
**Every M6 task must include comprehensive testing before being marked complete:**
- ✅ **Unit Tests** - Individual image processing function testing (>90% coverage)
- ✅ **Integration Tests** - End-to-end asset processing with real TTRPG assets
- ✅ **Property Tests** - Using `proptest` for image transformation validation
- ✅ **Performance Tests** - Benchmarks for large image processing
- ✅ **Visual Tests** - Image quality validation and regression testing

---

## **T6.1: Advanced TTRPG-Specific Media Processing**
**Duration**: 3 days | **Points**: 6 | **Priority**: 🟡 MEDIUM  
**Dependencies**: M2.0 Processing Plugin Architecture Foundation (Basic image processing already implemented)

Advanced TTRPG-specific processing beyond the basic capabilities provided by AssetConversionPlugin and SceneProcessingPlugin.

### **Implementation Steps for Junior Developer**

**Step 1: Advanced Computer Vision Dependencies**
Update `ttrpg-processing-plugins/Cargo.toml` for specialized features:
```toml
[dependencies]
# Advanced computer vision for TTRPG-specific processing
opencv = "0.88"           # Computer vision for advanced scene analysis
kurbo = "0.10"            # Advanced 2D geometry for complex wall detection
contour = "0.4"           # Contour detection for map boundaries

# Machine learning for content recognition
candle-core = "0.3"       # Lightweight ML for token/asset classification
tract = "0.18"            # ONNX runtime for pre-trained models

# Advanced color analysis
colorgrad = "0.6"         # Color gradient analysis for terrain detection
```

**Step 2: Create Advanced Scene Analysis Engine**
Create `ttrpg-processing-plugins/src/advanced/scene_analyzer.rs`:
```rust
use opencv::{core, imgproc, imgcodecs};
use kurbo::{BezPath, PathEl};
use candle_core::{Device, Tensor};
use std::collections::HashMap;

/// Advanced TTRPG scene analyzer for complex processing
pub struct TTRPGSceneAnalyzer {
    /// Pre-trained models for content recognition
    token_classifier: Option<Box<dyn TokenClassifier>>,
    terrain_detector: Option<Box<dyn TerrainDetector>>,
}

impl TTRPGSceneAnalyzer {
    pub fn new() -> Self {
        Self {
            token_classifier: Self::load_token_classifier(),
            terrain_detector: Self::load_terrain_detector(),
        }
    }
    
    /// Advanced wall detection using computer vision
    pub async fn detect_complex_walls(&self, scene_image: &DynamicImage) -> Result<Vec<WallSegment>, ProcessingError> {
        // Convert to OpenCV format
        let cv_mat = self.dynamic_image_to_mat(scene_image)?;
        
        // Advanced edge detection with multiple algorithms
        let edges = self.multi_algorithm_edge_detection(&cv_mat)?;
        
        // Line segment detection using Probabilistic Hough Transform
        let line_segments = self.detect_line_segments(&edges)?;
        
        // Filter and classify wall segments
        let wall_segments = self.classify_wall_segments(line_segments)?;
        
        Ok(wall_segments)
    }
    
    /// Intelligent terrain type detection
    pub async fn analyze_terrain_types(&self, scene_image: &DynamicImage) -> Result<HashMap<TerrainType, Vec<Region>>, ProcessingError> {
        if let Some(detector) = &self.terrain_detector {
            detector.detect_terrain_regions(scene_image).await
        } else {
            // Fallback to color-based analysis
            self.color_based_terrain_analysis(scene_image)
        }
    }
    
    /// Advanced token classification and enhancement
    pub async fn classify_and_enhance_tokens(&self, tokens: Vec<TokenCandidate>) -> Result<Vec<EnhancedToken>, ProcessingError> {
        if let Some(classifier) = &self.token_classifier {
            // Use ML-based classification
            let mut enhanced_tokens = Vec::new();
            
            for token in tokens {
                let classification = classifier.classify_token(&token).await?;
                let enhanced = self.enhance_token_based_on_classification(token, classification)?;
                enhanced_tokens.push(enhanced);
            }
            
            Ok(enhanced_tokens)
        } else {
            // Fallback to heuristic-based classification
            self.heuristic_token_classification(tokens)
        }
    }
    
    /// Advanced map boundary detection
    pub async fn detect_map_boundaries(&self, scene_image: &DynamicImage) -> Result<Vec<BoundarySegment>, ProcessingError> {
        // Use contour detection to find map boundaries
        let cv_mat = self.dynamic_image_to_mat(scene_image)?;
        let gray = self.convert_to_grayscale(&cv_mat)?;
        
        // Apply adaptive thresholding
        let binary = self.adaptive_threshold(&gray)?;
        
        // Find contours
        let contours = self.find_contours(&binary)?;
        
        // Filter and classify boundary contours
        let boundaries = self.classify_boundary_contours(contours)?;
        
        Ok(boundaries)
    }
    
    /// Multi-algorithm edge detection for robust wall detection
    fn multi_algorithm_edge_detection(&self, image: &core::Mat) -> Result<core::Mat, ProcessingError> {
        // Combine results from multiple edge detection algorithms
        let canny_edges = self.canny_edge_detection(image)?;
        let sobel_edges = self.sobel_edge_detection(image)?;
        let laplacian_edges = self.laplacian_edge_detection(image)?;
        
        // Combine and weight the results
        self.combine_edge_maps(vec![canny_edges, sobel_edges, laplacian_edges])
    }
            (false, true, _) => Ok(ImageFormat::WebP), // WebP excellent for photos
            (false, false, high) if high > 1000 => Ok(ImageFormat::WebP), // Complex graphics
            (false, false, _) => Ok(ImageFormat::Png), // Simple graphics, keep PNG
        }
    }
    
    /// Convert to WebP with optimal quality settings
    async fn convert_to_webp(&self, img: &DynamicImage, output_path: &Path) -> Result<(), ImageError> {
        let rgba = img.to_rgba8();
        let encoder = Encoder::from_rgba(rgba.as_raw(), img.width(), img.height());
        let webp_data = encoder.encode(self.webp_quality);
        
        tokio::fs::write(output_path, &*webp_data).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use proptest::prelude::*;
    
    #[tokio::test]
    async fn test_image_optimization() {
        let processor = ImageProcessor::new();
        let temp_dir = TempDir::new().unwrap();
        
        // Create test PNG image
        let img = DynamicImage::new_rgba8(512, 512);
        let input_path = temp_dir.path().join("test.png");
        img.save(&input_path).unwrap();
        
        let output_path = processor.optimize_image(&input_path).await.unwrap();
        assert!(output_path.exists());
        
        // Verify output is smaller or same quality
        let output_img = image::open(&output_path).unwrap();
        assert_eq!(output_img.dimensions(), (512, 512));
    }
    
    proptest! {
        #[test]
        fn test_resize_quality_preserving(
            width in 100u32..2000u32,
            height in 100u32..2000u32,
            target_width in 50u32..500u32
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let processor = ImageProcessor::new();
                let img = DynamicImage::new_rgba8(width, height);
                
                let result = processor.resize_with_quality(&img, target_width, target_width).await;
                prop_assert!(result.is_ok());
                
                let resized = result.unwrap();
                prop_assert!(resized.width() <= target_width);
                prop_assert!(resized.height() <= target_width);
            });
        }
    }
}
```

---

## **T6.2: Advanced Tile Combining Engine (CRITICAL MISSING FEATURE)**
**Duration**: 7 days | **Points**: 15 | **Priority**: 🚨 CRITICAL
**Dependencies**: T6.1 Complete

This was completely missing from our original plan but essential for large TTRPG maps!

### **Implementation Steps for Junior Developer**

**Step 1: Tile Detection and Analysis**
Create `ttrpg-assets/src/tile_combiner.rs`:
```rust
use image::{DynamicImage, RgbaImage};
use ndarray::{Array2, Array3};
use std::collections::HashMap;

/// Advanced tile combining engine for large TTRPG maps
pub struct TileCombiner {
    /// Minimum overlap required for tile matching (pixels)
    min_overlap: u32,
    /// Maximum gap to fill between tiles (pixels)
    max_gap_fill: u32,
    /// Quality threshold for seam detection
    seam_threshold: f64,
}

impl TileCombiner {
    pub fn new() -> Self {
        Self {
            min_overlap: 10,
            max_gap_fill: 5,
            seam_threshold: 0.95,
        }
    }
    
    /// Combine multiple tiles into a single large map
    pub async fn combine_tiles(&self, tiles: Vec<TileInfo>) -> Result<DynamicImage, TileError> {
        // Phase 1: Analyze tile relationships and detect overlaps
        let tile_graph = self.analyze_tile_relationships(&tiles).await?;
        
        // Phase 2: Determine optimal arrangement
        let arrangement = self.calculate_optimal_arrangement(&tile_graph)?;
        
        // Phase 3: Perform intelligent stitching with seam elimination
        let combined = self.stitch_tiles_with_seam_elimination(&tiles, &arrangement).await?;
        
        Ok(combined)
    }
    
    /// Detect if multiple images are tiles of a larger map
    pub async fn detect_tile_set(&self, images: &[DynamicImage]) -> Result<Vec<TileRelationship>, TileError> {
        let mut relationships = Vec::new();
        
        for (i, img1) in images.iter().enumerate() {
            for (j, img2) in images.iter().enumerate().skip(i + 1) {
                if let Some(relationship) = self.analyze_tile_relationship(img1, img2).await? {
                    relationships.push(TileRelationship {
                        tile1_index: i,
                        tile2_index: j,
                        relationship_type: relationship.relationship_type,
                        overlap_area: relationship.overlap_area,
                        confidence: relationship.confidence,
                    });
                }
            }
        }
        
        Ok(relationships)
    }
    
    /// Advanced seam detection using computer vision
    async fn detect_seams(&self, img1: &DynamicImage, img2: &DynamicImage) -> Result<Vec<SeamPoint>, TileError> {
        // Convert to arrays for numerical processing
        let array1 = self.image_to_array(img1)?;
        let array2 = self.image_to_array(img2)?;
        
        // Use cross-correlation to find best match points
        let correlation_map = self.compute_correlation_map(&array1, &array2)?;
        
        // Find peak correlation points
        let seam_points = self.extract_seam_points(&correlation_map)?;
        
        Ok(seam_points)
    }
    
    /// Intelligent stitching that eliminates visible seams
    async fn stitch_tiles_with_seam_elimination(
        &self,
        tiles: &[TileInfo],
        arrangement: &TileArrangement
    ) -> Result<DynamicImage, TileError> {
        // Calculate final dimensions
        let (final_width, final_height) = arrangement.calculate_dimensions();
        
        // Create output canvas
        let mut canvas = RgbaImage::new(final_width, final_height);
        
        // Process tiles in dependency order to handle overlaps
        for tile_placement in arrangement.get_placement_order() {
            let tile = &tiles[tile_placement.tile_index];
            let img = &tile.image;
            
            // Apply advanced blending for overlap areas
            self.blend_tile_with_seam_elimination(
                &mut canvas,
                img,
                tile_placement.position,
                &tile_placement.overlap_regions
            ).await?;
        }
        
        Ok(DynamicImage::ImageRgba8(canvas))
    }
}

#[derive(Debug, Clone)]
pub struct TileInfo {
    pub image: DynamicImage,
    pub original_path: PathBuf,
    pub estimated_position: Option<(i32, i32)>,
}

#[derive(Debug, Clone)]
pub struct TileRelationship {
    pub tile1_index: usize,
    pub tile2_index: usize,
    pub relationship_type: RelationshipType,
    pub overlap_area: OverlapArea,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub enum RelationshipType {
    LeftRight,   // tile1 is left of tile2
    TopBottom,   // tile1 is above tile2
    Diagonal,    // tiles connect diagonally
    NoRelation,  // tiles don't connect
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tile_detection() {
        let combiner = TileCombiner::new();
        
        // Create mock tile set - 2x2 grid
        let mut tiles = Vec::new();
        for y in 0..2 {
            for x in 0..2 {
                let img = create_mock_tile(x * 256, y * 256, 256, 256);
                tiles.push(img);
            }
        }
        
        let relationships = combiner.detect_tile_set(&tiles).await.unwrap();
        
        // Should detect 4 relationships in 2x2 grid
        assert!(relationships.len() >= 4);
    }
    
    #[tokio::test]
    async fn test_large_map_combination() {
        let combiner = TileCombiner::new();
        
        // Test combining tiles into large map
        let tiles = create_test_tile_set();
        let combined = combiner.combine_tiles(tiles).await.unwrap();
        
        // Verify dimensions and quality
        assert!(combined.width() >= 512);
        assert!(combined.height() >= 512);
    }
    
    fn create_mock_tile(x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        // Create tile with gradient pattern for testing
        let mut img = RgbaImage::new(width, height);
        for py in 0..height {
            for px in 0..width {
                let r = ((x + px) % 256) as u8;
                let g = ((y + py) % 256) as u8;
                let b = ((x + px + y + py) % 256) as u8;
                img.put_pixel(px, py, image::Rgba([r, g, b, 255]));
            }
        }
        DynamicImage::ImageRgba8(img)
    }
}
```

**Step 2: Memory-Efficient Processing for Massive Maps**
```rust
/// Memory-efficient tile processing for maps larger than available RAM
pub struct StreamingTileCombiner {
    tile_cache: LruCache<PathBuf, DynamicImage>,
    max_memory_usage: usize,
    temp_dir: PathBuf,
}

impl StreamingTileCombiner {
    /// Process enormous maps by streaming tiles through memory
    pub async fn combine_massive_tileset(
        &mut self,
        tile_paths: Vec<PathBuf>,
        output_path: &Path
    ) -> Result<(), TileError> {
        // Process in chunks to stay within memory limits
        let chunk_size = self.calculate_optimal_chunk_size(&tile_paths)?;
        
        for chunk in tile_paths.chunks(chunk_size) {
            // Process chunk and write to temp files
            let chunk_result = self.process_tile_chunk(chunk).await?;
            self.write_chunk_to_temp(&chunk_result).await?;
            
            // Clear cache to free memory
            self.tile_cache.clear();
        }
        
        // Final assembly from temp files
        self.assemble_final_image(output_path).await?;
        
        Ok(())
    }
}
```

This is the critical missing functionality that enables processing the massive battle maps common in TTRPG campaigns!

**Acceptance Criteria for M6:**
- [ ] ✅ Process images up to 50MB efficiently
- [ ] ✅ Combine 4x4+ tile sets into seamless maps  
- [ ] ✅ 95%+ accuracy in tile relationship detection
- [ ] ✅ Invisible seam elimination in final output
- [ ] ✅ Memory usage under 2GB for massive maps
- [ ] ✅ WebP conversion with <10% quality loss
- [ ] ✅ SVG to raster conversion support
