# Plugin Architecture Design for Multi-Format TTRPG Converter

## Core Plugin Traits

```rust
use std::path::Path;
use std::collections::HashMap;
use async_trait::async_trait;

/// Input format plugin - handles parsing different campaign formats
#[async_trait]
pub trait InputPlugin: Send + Sync {
    /// Check if this plugin can handle the given source
    fn can_handle(&self, source: &Path) -> bool;
    
    /// Get format name and version info
    fn format_info(&self) -> FormatInfo;
    
    /// Parse campaign data from source
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<Campaign>;
    
    /// Discover all assets referenced in the campaign
    async fn discover_assets(&self, campaign: &Campaign, source: &Path) -> ConversionResult<Vec<AssetInfo>>;
    
    /// Get format-specific metadata
    fn get_metadata(&self, source: &Path) -> ConversionResult<HashMap<String, serde_json::Value>>;
}

/// Output format plugin - handles writing to different target systems
#[async_trait]
pub trait OutputPlugin: Send + Sync {
    /// Get supported output format info
    fn format_info(&self) -> FormatInfo;
    
    /// Check if this plugin supports the target configuration
    fn supports_config(&self, config: &OutputConfig) -> bool;
    
    /// Convert campaign to target format
    async fn convert_campaign(
        &self, 
        campaign: &Campaign, 
        assets: &[ProcessedAsset],
        config: &OutputConfig
    ) -> ConversionResult<OutputBundle>;
    
    /// Write the converted data to target location
    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        options: &WriteOptions
    ) -> ConversionResult<()>;
}

/// Asset processing plugin - handles different optimization strategies
#[async_trait]
pub trait ProcessingPlugin: Send + Sync {
    /// Get processor name and capabilities
    fn processor_info(&self) -> ProcessorInfo;
    
    /// Check if this processor can handle the asset type
    fn can_process(&self, asset: &AssetInfo) -> bool;
    
    /// Process a single asset
    async fn process_asset(
        &self,
        asset: AssetInfo,
        config: &ProcessingConfig
    ) -> ConversionResult<ProcessedAsset>;
    
    /// Batch process multiple assets (optional optimization)
    async fn process_batch(
        &self,
        assets: Vec<AssetInfo>,
        config: &ProcessingConfig,
        progress: Option<Box<dyn ProgressCallback>>
    ) -> ConversionResult<Vec<ProcessedAsset>> {
        // Default implementation - can be overridden for batch optimizations
        let mut results = Vec::with_capacity(assets.len());
        for asset in assets {
            results.push(self.process_asset(asset, config).await?);
        }
        Ok(results)
    }
}
```

## Example Plugin Implementations

### Roll20 Input Plugin
```rust
pub struct Roll20InputPlugin {
    zip_handler: ZipExtractor,
    json_parser: Roll20JsonParser,
}

#[async_trait]
impl InputPlugin for Roll20InputPlugin {
    fn can_handle(&self, source: &Path) -> bool {
        // Check for .zip file or extracted campaign.json
        source.extension().map_or(false, |ext| ext == "zip") ||
        source.join("campaign.json").exists()
    }
    
    fn format_info(&self) -> FormatInfo {
        FormatInfo {
            name: "Roll20".to_string(),
            version: "2024".to_string(),
            supported_features: vec![
                "characters", "scenes", "assets", "handouts", "macros"
            ].into_iter().map(|s| s.to_string()).collect(),
        }
    }
    
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<Campaign> {
        // Handle both zip and extracted directory
        let campaign_json = if source.extension().map_or(false, |ext| ext == "zip") {
            self.zip_handler.extract_campaign_json(source).await?
        } else {
            source.join("campaign.json")
        };
        
        self.json_parser.parse_file(&campaign_json).await
    }
    
    async fn discover_assets(&self, campaign: &Campaign, source: &Path) -> ConversionResult<Vec<AssetInfo>> {
        // Roll20-specific asset discovery logic
        let mut assets = Vec::new();
        
        // Character avatars
        for actor in &campaign.actors {
            if let Some(avatar_url) = &actor.avatar {
                assets.push(AssetInfo {
                    url: avatar_url.clone(),
                    asset_type: AssetType::CharacterAvatar,
                    entity_id: Some(actor.id.clone()),
                    // ... other fields
                });
            }
        }
        
        // Scene backgrounds
        for scene in &campaign.scenes {
            if let Some(bg_url) = &scene.background_url {
                assets.push(AssetInfo {
                    url: bg_url.clone(),
                    asset_type: AssetType::SceneBackground,
                    entity_id: Some(scene.id.clone()),
                    // ... other fields
                });
            }
        }
        
        Ok(assets)
    }
}
```

### Foundry VTT Output Plugin
```rust
pub struct FoundryOutputPlugin {
    db_writer: FoundryDbWriter,
    manifest_generator: ManifestGenerator,
}

#[async_trait]
impl OutputPlugin for FoundryOutputPlugin {
    fn format_info(&self) -> FormatInfo {
        FormatInfo {
            name: "Foundry VTT".to_string(),
            version: "v11-v12".to_string(),
            supported_features: vec![
                "worlds", "modules", "compendiums", "actors", "scenes"
            ].into_iter().map(|s| s.to_string()).collect(),
        }
    }
    
    fn supports_config(&self, config: &OutputConfig) -> bool {
        matches!(config.output_type, OutputType::FoundryWorld | OutputType::FoundryModule)
    }
    
    async fn convert_campaign(
        &self,
        campaign: &Campaign,
        assets: &[ProcessedAsset],
        config: &OutputConfig
    ) -> ConversionResult<OutputBundle> {
        let mut bundle = OutputBundle::new();
        
        match config.output_type {
            OutputType::FoundryWorld => {
                // Generate world.json
                let world_data = self.generate_world_manifest(campaign, config)?;
                bundle.add_file("world.json", serde_json::to_string_pretty(&world_data)?);
                
                // Convert actors to Foundry format
                let actors_db = self.convert_actors_to_foundry(&campaign.actors)?;
                bundle.add_db("actors.db", actors_db);
                
                // Convert scenes
                let scenes_db = self.convert_scenes_to_foundry(&campaign.scenes, assets)?;
                bundle.add_db("scenes.db", scenes_db);
            }
            OutputType::FoundryModule => {
                // Generate module.json
                let module_data = self.generate_module_manifest(campaign, config)?;
                bundle.add_file("module.json", serde_json::to_string_pretty(&module_data)?);
                
                // Create compendium packs
                let actor_pack = self.create_actor_compendium(&campaign.actors)?;
                bundle.add_file("packs/actors.db", actor_pack);
            }
            _ => return Err(ConversionError::UnsupportedOutput(config.output_type.clone()))
        }
        
        Ok(bundle)
    }
}
```

## Universal Converter Pipeline

```rust
pub struct UniversalConverter {
    input_plugins: HashMap<String, Box<dyn InputPlugin>>,
    output_plugins: HashMap<String, Box<dyn OutputPlugin>>,
    processing_plugins: Vec<Box<dyn ProcessingPlugin>>,
    service_manager: Arc<dyn ServiceManager>,
}

impl UniversalConverter {
    pub async fn convert(
        &self,
        source: &Path,
        target: &Path,
        config: ConversionConfig
    ) -> ConversionResult<ConversionReport> {
        // 1. Auto-detect input format
        let input_plugin = self.detect_input_format(source)?;
        
        // 2. Parse campaign
        let campaign = input_plugin.parse_campaign(source).await?;
        
        // 3. Discover assets
        let assets = input_plugin.discover_assets(&campaign, source).await?;
        
        // 4. Process assets based on config
        let processed_assets = self.process_assets(assets, &config.processing).await?;
        
        // 5. Convert to target format
        let output_plugin = self.get_output_plugin(&config.output_format)?;
        let bundle = output_plugin.convert_campaign(&campaign, &processed_assets, &config.output).await?;
        
        // 6. Write output
        output_plugin.write_output(bundle, target, &config.write_options).await?;
        
        Ok(ConversionReport {
            source_format: input_plugin.format_info(),
            target_format: output_plugin.format_info(),
            assets_processed: processed_assets.len(),
            // ... other stats
        })
    }
    
    fn detect_input_format(&self, source: &Path) -> ConversionResult<&dyn InputPlugin> {
        for plugin in self.input_plugins.values() {
            if plugin.can_handle(source) {
                return Ok(plugin.as_ref());
            }
        }
        Err(ConversionError::UnsupportedInputFormat(source.to_path_buf()))
    }
}
```

## Configuration Examples

```rust
// Roll20 → Foundry World conversion
let config = ConversionConfig {
    output_format: "foundry".to_string(),
    output: OutputConfig {
        output_type: OutputType::FoundryWorld,
        foundry_version: "v12".to_string(),
        optimize_assets: true,
        include_folders: true,
        // Foundry-specific settings
        foundry_settings: FoundryWorldConfig {
            system: "dnd5e".to_string(),
            core_version: "12.331".to_string(),
        }
    },
    processing: ProcessingConfig {
        asset_optimization: AssetOptimization {
            compress_images: true,
            max_image_size: (2048, 2048),
            convert_to_webp: true,
        },
        parallel_processing: true,
        max_concurrent_downloads: 10,
    },
    write_options: WriteOptions {
        create_backup: true,
        overwrite_existing: false,
    }
};
```

This architecture gives you:
- **Single codebase** handles all format combinations
- **Easy extensibility** - new formats = new plugins
- **Composable processing** - mix asset optimizations as needed  
- **Configuration-driven** - same code, different behavior
- **Testable** - mock plugins for unit tests
