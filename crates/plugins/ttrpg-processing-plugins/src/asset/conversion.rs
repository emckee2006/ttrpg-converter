//! AssetConversionPlugin - Format conversion, image optimization
//!
//! This plugin handles format conversion between different asset types,
//! image optimization for web delivery, and format standardization.

use crate::shared::AssetExecutionContext;
use ttrpg_core::{
    error::ConversionResult,
    plugin_framework::types::AssetInfo,
};
use std::sync::Arc;
use imageproc::image::{DynamicImage, ImageFormat, ImageBuffer, Rgba};
use rayon::prelude::*;

/// AssetConversionPlugin for format conversion and optimization
pub struct AssetConversionPlugin {
    /// Shared execution context
    context: Arc<AssetExecutionContext>,
    /// Configuration for conversion operations
    config: ConversionConfig,
}

/// Configuration for asset conversion
#[derive(Debug, Clone)]
pub struct ConversionConfig {
    /// Target image format for optimization
    pub target_format: TargetFormat,
    /// Image quality settings
    pub quality_settings: QualitySettings,
    /// Enable format conversion
    pub enable_conversion: bool,
    /// Enable image optimization
    pub enable_optimization: bool,
    /// Maximum image dimensions
    pub max_dimensions: Option<(u32, u32)>,
}

/// Target formats for asset conversion
#[derive(Debug, Clone)]
pub enum TargetFormat {
    /// Convert to WebP for better compression
    WebP,
    /// Convert to JPEG for compatibility
    Jpeg,
    /// Convert to PNG for transparency
    Png,
    /// Keep original format
    Original,
    /// Auto-detect best format
    Auto,
}

/// Quality settings for different formats
#[derive(Debug, Clone)]
pub struct QualitySettings {
    /// JPEG quality (1-100)
    pub jpeg_quality: u8,
    /// WebP quality (1-100)
    pub webp_quality: u8,
    /// PNG compression level (0-9)
    pub png_compression: u8,
    /// Enable progressive JPEG
    pub progressive_jpeg: bool,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            target_format: TargetFormat::Auto,
            quality_settings: QualitySettings {
                jpeg_quality: 85,
                webp_quality: 80, 
                png_compression: 6,
                progressive_jpeg: true,
            },
            enable_conversion: true,
            enable_optimization: true,
            max_dimensions: Some((2048, 2048)),
        }
    }
}

/// Converted asset result
#[derive(Debug, Clone)]
pub struct ConvertedAsset {
    /// Original asset information
    pub original: AssetInfo,
    /// Converted asset data
    pub converted_data: Vec<u8>,
    /// New format
    pub new_format: String,
    /// New file extension
    pub new_extension: String,
    /// Compression ratio achieved
    pub compression_ratio: f64,
    /// New dimensions if resized
    pub new_dimensions: Option<(u32, u32)>,
}

impl AssetConversionPlugin {
    /// Create new asset conversion plugin
    pub fn new() -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        let config = ConversionConfig::default();
        
        Ok(Self {
            context,
            config,
        })
    }
    
    /// Create plugin with custom configuration
    pub fn with_config(config: ConversionConfig) -> ConversionResult<Self> {
        let context = Arc::new(AssetExecutionContext::default());
        
        Ok(Self {
            context,
            config,
        })
    }
    
    /// Convert multiple assets formats in parallel
    pub async fn convert_formats(&self, assets: Vec<AssetInfo>) -> ConversionResult<Vec<ConvertedAsset>> {
        if assets.is_empty() {
            return Ok(vec![]);
        }
        
        if !self.config.enable_conversion {
            // Return original assets as "converted"
            return Ok(assets.into_iter().map(|asset| ConvertedAsset {
                converted_data: asset.data.clone(),
                new_format: asset.mime_type.clone().unwrap_or_default(),
                new_extension: self.get_file_extension(&asset).unwrap_or_default(),
                compression_ratio: 1.0,
                new_dimensions: asset.width.zip(asset.height).map(|(w, h)| (w, h)),
                original: asset,
            }).collect());
        }
        
        tracing::info!("Converting formats for {} assets", assets.len());
        
        // Use CPU pool for parallel image processing
        let assets_clone = assets.clone();
        let config = self.config.clone();
        
        let converted_assets = tokio::task::spawn_blocking(move || -> ConversionResult<Vec<ConvertedAsset>> {
            assets_clone.par_iter()
                .map(|asset| Self::convert_single_asset(asset, &config))
                .collect::<ConversionResult<Vec<_>>>()
        }).await??;
        
        let total_original_size: u64 = assets.iter().map(|a| a.data.len() as u64).sum();
        let total_converted_size: u64 = converted_assets.iter().map(|a| a.converted_data.len() as u64).sum();
        let overall_compression = if total_original_size > 0 {
            total_converted_size as f64 / total_original_size as f64
        } else {
            1.0
        };
        
        tracing::info!("Converted {} assets with {:.1}% size reduction", 
                      converted_assets.len(),
                      (1.0 - overall_compression) * 100.0);
        
        Ok(converted_assets)
    }
    
    /// Convert single asset format
    fn convert_single_asset(asset: &AssetInfo, config: &ConversionConfig) -> ConversionResult<ConvertedAsset> {
        // Check if asset is an image
        if !Self::is_image_asset(asset) {
            // Non-image assets pass through unchanged
            return Ok(ConvertedAsset {
                converted_data: asset.data.clone(),
                new_format: asset.mime_type.clone().unwrap_or_default(),
                new_extension: Self::get_file_extension_static(asset).unwrap_or_default(),
                compression_ratio: 1.0,
                new_dimensions: asset.width.zip(asset.height).map(|(w, h)| (w, h)),
                original: asset.clone(),
            });
        }
        
        // Load image
        let image = image::load_from_memory(&asset.data)
            .map_err(|e| ttrpg_core::error::ConversionError::AssetError(
                format!("Failed to load image: {}", e)
            ))?;
        
        // Resize if needed
        let resized_image = if let Some((max_width, max_height)) = config.max_dimensions {
            if image.width() > max_width || image.height() > max_height {
                tracing::debug!("Resizing image from {}x{} to fit {}x{}", 
                               image.width(), image.height(), max_width, max_height);
                image.resize(max_width, max_height, image::imageops::FilterType::Lanczos3)
            } else {
                image
            }
        } else {
            image
        };
        
        // Determine target format
        let target_format = Self::determine_target_format(&config.target_format, asset);
        
        // Convert image
        let (converted_data, new_format, new_extension) = Self::convert_image_to_format(
            &resized_image, 
            &target_format, 
            &config.quality_settings
        )?;
        
        let compression_ratio = converted_data.len() as f64 / asset.data.len() as f64;
        let new_dimensions = Some((resized_image.width(), resized_image.height()));
        
        Ok(ConvertedAsset {
            original: asset.clone(),
            converted_data,
            new_format,
            new_extension,
            compression_ratio,
            new_dimensions,
        })
    }
    
    /// Check if asset is an image
    fn is_image_asset(asset: &AssetInfo) -> bool {
        if let Some(mime_type) = &asset.mime_type {
            return mime_type.starts_with("image/");
        }
        
        if let Some(file_name) = &asset.file_name {
            let name_lower = file_name.to_lowercase();
            return name_lower.ends_with(".jpg") || name_lower.ends_with(".jpeg") || 
                   name_lower.ends_with(".png") || name_lower.ends_with(".gif") ||
                   name_lower.ends_with(".bmp") || name_lower.ends_with(".webp");
        }
        
        false
    }
    
    /// Determine target format based on configuration and asset properties
    fn determine_target_format(target_format: &TargetFormat, asset: &AssetInfo) -> ImageFormat {
        match target_format {
            TargetFormat::WebP => ImageFormat::WebP,
            TargetFormat::Jpeg => ImageFormat::Jpeg,
            TargetFormat::Png => ImageFormat::Png,
            TargetFormat::Original => {
                // Try to preserve original format
                if let Some(mime_type) = &asset.mime_type {
                    match mime_type.as_str() {
                        "image/jpeg" => ImageFormat::Jpeg,
                        "image/png" => ImageFormat::Png,
                        "image/webp" => ImageFormat::WebP,
                        "image/gif" => ImageFormat::Gif,
                        "image/bmp" => ImageFormat::Bmp,
                        _ => ImageFormat::Png, // Default fallback
                    }
                } else {
                    ImageFormat::Png
                }
            },
            TargetFormat::Auto => {
                // Auto-detect best format based on content
                if Self::has_transparency(asset) {
                    ImageFormat::Png // Preserve transparency
                } else {
                    ImageFormat::Jpeg // Better compression for photos
                }
            },
        }
    }
    
    /// Check if asset likely has transparency
    fn has_transparency(asset: &AssetInfo) -> bool {
        if let Some(mime_type) = &asset.mime_type {
            return mime_type == "image/png" || mime_type == "image/gif";
        }
        
        if let Some(file_name) = &asset.file_name {
            let name_lower = file_name.to_lowercase();
            return name_lower.ends_with(".png") || name_lower.ends_with(".gif");
        }
        
        false
    }
    
    /// Convert image to specified format
    fn convert_image_to_format(
        image: &DynamicImage, 
        format: &ImageFormat, 
        quality: &QualitySettings
    ) -> ConversionResult<(Vec<u8>, String, String)> {
        let mut buffer = Vec::new();
        
        match format {
            ImageFormat::Jpeg => {
                let mut cursor = std::io::Cursor::new(&mut buffer);
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality.jpeg_quality);
                image.write_with_encoder(encoder)
                    .map_err(|e| ttrpg_core::error::ConversionError::AssetError(format!("JPEG encoding failed: {}", e)))?;
                Ok((buffer, "image/jpeg".to_string(), "jpg".to_string()))
            },
            ImageFormat::Png => {
                let mut cursor = std::io::Cursor::new(&mut buffer);
                let encoder = image::codecs::png::PngEncoder::new_with_quality(
                    &mut cursor, 
                    image::codecs::png::CompressionType::Default,
                    image::codecs::png::FilterType::Adaptive
                );
                image.write_with_encoder(encoder)
                    .map_err(|e| ttrpg_core::error::ConversionError::AssetError(format!("PNG encoding failed: {}", e)))?;
                Ok((buffer, "image/png".to_string(), "png".to_string()))
            },
            ImageFormat::WebP => {
                // WebP encoding would require additional dependencies
                // For now, fallback to JPEG
                let mut cursor = std::io::Cursor::new(&mut buffer);
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut cursor, quality.webp_quality);
                image.write_with_encoder(encoder)
                    .map_err(|e| ttrpg_core::error::ConversionError::AssetError(format!("WebP fallback encoding failed: {}", e)))?;
                Ok((buffer, "image/jpeg".to_string(), "jpg".to_string()))
            },
            _ => {
                // Default to PNG for other formats
                let mut cursor = std::io::Cursor::new(&mut buffer);
                let encoder = image::codecs::png::PngEncoder::new(&mut cursor);
                image.write_with_encoder(encoder)
                    .map_err(|e| ttrpg_core::error::ConversionError::AssetError(format!("Default PNG encoding failed: {}", e)))?;
                Ok((buffer, "image/png".to_string(), "png".to_string()))
            }
        }
    }
    
    /// Get file extension from asset
    fn get_file_extension(&self, asset: &AssetInfo) -> Option<String> {
        Self::get_file_extension_static(asset)
    }
    
    /// Get file extension from asset (static version)
    fn get_file_extension_static(asset: &AssetInfo) -> Option<String> {
        asset.file_name
            .as_ref()
            .and_then(|name| name.rfind('.'))
            .map(|i| asset.file_name.as_ref().unwrap()[i+1..].to_string())
    }
    
    /// Get conversion statistics
    pub fn get_statistics(&self) -> ConversionStatistics {
        ConversionStatistics {
            assets_converted: 0, // TODO: Track conversion stats
            total_size_reduction: 0,
            average_compression_ratio: 0.0,
            formats_processed: std::collections::HashMap::new(),
        }
    }
}

impl Clone for AssetConversionPlugin {
    fn clone(&self) -> Self {
        Self {
            context: Arc::clone(&self.context),
            config: self.config.clone(),
        }
    }
}

/// Statistics for asset conversion operations
#[derive(Debug)]
pub struct ConversionStatistics {
    /// Number of assets converted
    pub assets_converted: usize,
    /// Total size reduction in bytes
    pub total_size_reduction: u64,
    /// Average compression ratio
    pub average_compression_ratio: f64,
    /// Formats processed and their counts
    pub formats_processed: std::collections::HashMap<String, usize>,
}

// TODO: Auto-register plugin with inventory once PluginInfo structure is defined
// inventory::submit! {
//     ttrpg_core::plugin_framework::PluginInfo {
//         id: "asset_conversion_plugin",
//         name: "AssetConversionPlugin",
//         version: "1.0.0",
//         description: "Format conversion and image optimization", 
//         plugin_type: ttrpg_core::plugin_framework::PluginType::Processing,
//         dependencies: &["asset_resolution_plugin"],
//         factory: || Box::new(AssetConversionPlugin::new().expect("Failed to create AssetConversionPlugin")),
//     }
// }
