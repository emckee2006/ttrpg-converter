# M6: Advanced Media Processing & Optimization - Junior Developer Implementation Guide

## 🎯 **MILESTONE 6 OVERVIEW**
**Duration**: 3 weeks | **Total Points**: 25 | **Priority**: 🔥 HIGH

Advanced image processing, tile combining, and asset optimization engine - the missing core functionality from previous R20Converter.

### 🧪 **TESTING REQUIREMENTS**
**Every M6 task must include comprehensive testing before being marked complete:**
- ✅ **Unit Tests** - Individual image processing function testing (>90% coverage)
- ✅ **Integration Tests** - End-to-end asset processing with real TTRPG assets
- ✅ **Property Tests** - Using `proptest` for image transformation validation
- ✅ **Performance Tests** - Benchmarks for large image processing
- ✅ **Visual Tests** - Image quality validation and regression testing

---

## **T6.1: Professional Image Processing Pipeline**
**Duration**: 5 days | **Points**: 10 | **Priority**: 🔥 HIGH  
**Dependencies**: M1.4 Complete (Basic media processing foundation)

### **Implementation Steps for Junior Developer**

**Step 1: Enhanced Image Processing Dependencies**
Update `ttrpg-assets/Cargo.toml`:
```toml
[dependencies]
# Professional image processing stack (eliminate reinvented wheels)
image = { workspace = true }
imageproc = "0.23"        # Advanced image processing algorithms
photon-rs = "0.3"         # High-performance image effects
webp = { workspace = true }
fast_image_resize = { workspace = true }
resvg = "0.37"            # SVG rendering to raster

# Computer vision for advanced features
ndarray = "0.15"          # N-dimensional arrays for image data
rayon = { workspace = true }  # Parallel processing for images

# Quality and format detection
kamadak-exif = "0.5"      # EXIF metadata handling
rgb = "0.8"               # Color space conversions
```

**Step 2: Create Advanced Image Processor**
Create `ttrpg-assets/src/image_processor.rs`:
```rust
use image::{DynamicImage, ImageFormat, RgbaImage, ImageError};
use imageproc::geometric_transformations::{resize, Interpolation};
use fast_image_resize as fr;
use webp::Encoder;
use resvg::usvg;
use std::path::{Path, PathBuf};

/// Advanced image processing engine
pub struct ImageProcessor {
    /// Quality settings for different output formats
    webp_quality: f32,
    jpeg_quality: u8,
    png_compression: u8,
}

impl ImageProcessor {
    pub fn new() -> Self {
        Self {
            webp_quality: 80.0,  // High quality WebP
            jpeg_quality: 85,    // High quality JPEG
            png_compression: 6,  // Balanced PNG compression
        }
    }
    
    /// Multi-format image optimization with intelligent format selection
    pub async fn optimize_image(&self, input_path: &Path) -> Result<PathBuf, ImageError> {
        let img = image::open(input_path)?;
        let output_path = self.determine_optimal_output_path(input_path, &img)?;
        
        match self.determine_optimal_format(&img)? {
            ImageFormat::WebP => self.convert_to_webp(&img, &output_path).await?,
            ImageFormat::Png => self.optimize_png(&img, &output_path).await?,
            ImageFormat::Jpeg => self.optimize_jpeg(&img, &output_path).await?,
            _ => return Err(ImageError::Unsupported("Unsupported output format".into())),
        }
        
        Ok(output_path)
    }
    
    /// High-performance image resizing with quality preservation
    pub async fn resize_with_quality(&self, img: &DynamicImage, max_width: u32, max_height: u32) -> Result<DynamicImage, ImageError> {
        // Use fast_image_resize for production-quality scaling
        let src_image = fr::Image::from_vec_u8(
            img.width(),
            img.height(),
            img.to_rgba8().into_raw(),
            fr::PixelType::U8x4,
        )?;
        
        let dst_width = std::cmp::min(max_width, img.width());
        let dst_height = std::cmp::min(max_height, img.height());
        
        let mut dst_image = fr::Image::new(
            dst_width,
            dst_height,
            fr::PixelType::U8x4,
        );
        
        // Professional quality resizing algorithm
        let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
        resizer.resize(&src_image.view(), &mut dst_image.view_mut())?;
        
        // Convert back to DynamicImage
        let rgba_img = RgbaImage::from_raw(dst_width, dst_height, dst_image.buffer().to_vec())
            .ok_or_else(|| ImageError::Parameter("Failed to create RgbaImage".into()))?;
            
        Ok(DynamicImage::ImageRgba8(rgba_img))
    }
    
    /// Convert SVG assets to optimized raster formats
    pub async fn convert_svg(&self, svg_path: &Path, target_width: u32) -> Result<PathBuf, ImageError> {
        let svg_data = std::fs::read(svg_path)?;
        let tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())?;
        
        let pixmap_size = tree.svg_node().size.to_screen_size();
        let scale = target_width as f64 / pixmap_size.width() as f64;
        
        let scaled_size = pixmap_size.scale_by(scale).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(scaled_size.width(), scaled_size.height()).unwrap();
        
        resvg::render(&tree, usvg::FitTo::Size(scaled_size.width(), scaled_size.height()), pixmap.as_mut());
        
        let output_path = svg_path.with_extension("png");
        pixmap.save_png(&output_path)?;
        
        Ok(output_path)
    }
    
    /// Intelligent format selection based on image characteristics
    fn determine_optimal_format(&self, img: &DynamicImage) -> Result<ImageFormat, ImageError> {
        // Analyze image characteristics
        let has_transparency = self.has_transparency(img);
        let color_complexity = self.analyze_color_complexity(img);
        let is_photographic = self.is_photographic_content(img);
        
        match (has_transparency, is_photographic, color_complexity) {
            (true, _, _) => Ok(ImageFormat::WebP),  // WebP handles transparency well
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
