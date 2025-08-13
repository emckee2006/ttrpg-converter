# Comprehensive Plugin Architecture - Implementation Plan

## 🎯 **ARCHITECTURAL VISION**

**Enable ALL planned features (400 points functionality) with linear complexity scaling (N+M) instead of exponential (N×M)**

### **Core Insight**
"Platform integration" = JSON output formats, not complex API integrations. This makes plugin architecture perfect for:
- Pathbuilder JSON output
- D&D Beyond JSON output  
- HeroLab JSON output
- Fantasy Grounds XML output

## 🏗️ **PLUGIN ARCHITECTURE DESIGN**

```rust
// Universal Converter with Plugin System
pub struct UniversalConverter {
    // Input Format Plugins (N complexity)
    input_plugins: HashMap<InputFormat, Box<dyn InputPlugin>>,
    
    // Sophisticated Processing Middleware (your advanced features)
    rule_engine: Arc<RuleEngine>,           // M7: RETE algorithm, D&D ↔ Pathfinder
    asset_processor: Arc<AssetProcessor>,   // M6: Tile combining, WebP optimization  
    pdf_generator: Arc<PDFGenerator>,       // M10: Professional character sheets
    validation_engine: Arc<ValidationEngine>, // M2: Comprehensive validation
    
    // Output Format Plugins (M complexity)
    output_plugins: HashMap<OutputFormat, Box<dyn OutputPlugin>>,
    
    // Plugin Management
    plugin_manager: Arc<PluginManager>,
    config_manager: Arc<ConfigManager>,
}
```

## 📋 **IMPLEMENTATION PHASES**

### **Phase 1: Plugin Framework Foundation (1 week)**

#### **1.1: Core Plugin Traits**
```rust
// ttrpg-core/src/plugins/mod.rs
use async_trait::async_trait;

#[async_trait]
pub trait InputPlugin: Send + Sync {
    /// Plugin identification
    fn plugin_info(&self) -> PluginInfo;
    
    /// Format detection
    fn can_handle(&self, source: &Path) -> bool;
    
    /// Core parsing functionality
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<UniversalCampaign>;
    async fn discover_assets(&self, campaign: &UniversalCampaign) -> ConversionResult<Vec<AssetInfo>>;
    
    /// Metadata extraction
    fn extract_metadata(&self, source: &Path) -> ConversionResult<CampaignMetadata>;
}

#[async_trait]
pub trait OutputPlugin: Send + Sync {
    /// Plugin identification  
    fn plugin_info(&self) -> PluginInfo;
    fn supported_formats(&self) -> Vec<OutputFormat>;
    
    /// Core generation functionality
    async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        assets: &[ProcessedAsset],
        config: &OutputConfig
    ) -> ConversionResult<OutputBundle>;
    
    /// Writing capabilities
    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        options: &WriteOptions
    ) -> ConversionResult<()>;
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub supported_features: Vec<String>,
}
```

#### **1.2: Plugin Manager**
```rust
// ttrpg-core/src/plugins/manager.rs
pub struct PluginManager {
    input_plugins: HashMap<String, Box<dyn InputPlugin>>,
    output_plugins: HashMap<String, Box<dyn OutputPlugin>>,
    middleware_services: MiddlewareServices,
}

pub struct MiddlewareServices {
    pub rule_engine: Arc<dyn RuleEngine>,
    pub asset_processor: Arc<dyn AssetProcessor>, 
    pub pdf_generator: Arc<dyn PDFGenerator>,
    pub validation_engine: Arc<dyn ValidationEngine>,
    pub logging: Arc<dyn LoggingService>,
}

impl PluginManager {
    /// Auto-detect input format and select appropriate plugin
    pub fn detect_input_plugin(&self, source: &Path) -> ConversionResult<&dyn InputPlugin> {
        for plugin in self.input_plugins.values() {
            if plugin.can_handle(source) {
                return Ok(plugin.as_ref());
            }
        }
        Err(ConversionError::UnsupportedInputFormat(source.to_path_buf()))
    }
    
    /// Get output plugin by format
    pub fn get_output_plugin(&self, format: OutputFormat) -> ConversionResult<&dyn OutputPlugin> {
        self.output_plugins
            .get(&format.to_string())
            .map(|p| p.as_ref())
            .ok_or_else(|| ConversionError::UnsupportedOutputFormat(format))
    }
    
    /// Register built-in plugins
    pub fn register_builtin_plugins(&mut self) -> ConversionResult<()> {
        // Input plugins
        self.register_input_plugin(Box::new(Roll20InputPlugin::new()))?;
        self.register_input_plugin(Box::new(FoundryInputPlugin::new()))?;
        
        // Output plugins  
        self.register_output_plugin(Box::new(FoundryOutputPlugin::new()))?;
        self.register_output_plugin(Box::new(PathbuilderOutputPlugin::new()))?;
        self.register_output_plugin(Box::new(DNDBeyondOutputPlugin::new()))?;
        self.register_output_plugin(Box::new(PDFOutputPlugin::new()))?;
        
        Ok(())
    }
}
```

### **Phase 2: Convert Existing Code to Plugins (1 week)**

#### **2.1: Roll20InputPlugin (convert existing Roll20Parser)**
```rust
// ttrpg-plugins/src/input/roll20.rs
pub struct Roll20InputPlugin {
    json_parser: Roll20JsonParser,
    zip_handler: ZipExtractor,
    asset_discoverer: Roll20AssetDiscoverer,
}

#[async_trait]
impl InputPlugin for Roll20InputPlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Roll20 Input".to_string(),
            version: "1.0.0".to_string(),
            description: "Parse Roll20 campaign exports (JSON and ZIP)".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec![
                "json_parsing".to_string(),
                "zip_extraction".to_string(), 
                "asset_discovery".to_string(),
                "character_conversion".to_string(),
                "scene_conversion".to_string(),
                "handout_conversion".to_string(),
            ],
        }
    }
    
    fn can_handle(&self, source: &Path) -> bool {
        // Check for .zip file or extracted campaign.json
        if source.extension().map_or(false, |ext| ext == "zip") {
            return true;
        }
        source.join("campaign.json").exists() || 
        source.file_name().map_or(false, |name| name == "campaign.json")
    }
    
    async fn parse_campaign(&self, source: &Path) -> ConversionResult<UniversalCampaign> {
        // Reuse existing Roll20Parser logic
        let campaign_json = if source.extension().map_or(false, |ext| ext == "zip") {
            self.zip_handler.extract_campaign_json(source).await?
        } else if source.file_name().map_or(false, |name| name == "campaign.json") {
            source.to_path_buf()
        } else {
            source.join("campaign.json")
        };
        
        self.json_parser.parse_file(&campaign_json).await
    }
    
    async fn discover_assets(&self, campaign: &UniversalCampaign) -> ConversionResult<Vec<AssetInfo>> {
        self.asset_discoverer.discover_all_assets(campaign).await
    }
}
```

#### **2.2: FoundryOutputPlugin**
```rust
// ttrpg-plugins/src/output/foundry.rs  
pub struct FoundryOutputPlugin {
    db_writer: FoundryDbWriter,
    manifest_generator: FoundryManifestGenerator,
    asset_mapper: FoundryAssetMapper,
}

#[async_trait]
impl OutputPlugin for FoundryOutputPlugin {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Foundry VTT Output".to_string(),
            version: "1.0.0".to_string(),
            description: "Generate Foundry VTT worlds and modules".to_string(),
            author: "TTRPG Converter Team".to_string(),
            supported_features: vec![
                "foundry_world".to_string(),
                "foundry_module".to_string(),
                "leveldb_generation".to_string(),
                "nedb_generation".to_string(),
                "manifest_generation".to_string(),
            ],
        }
    }
    
    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::FoundryWorld,
            OutputFormat::FoundryModule,
            OutputFormat::FoundryV10,
            OutputFormat::FoundryV11, 
            OutputFormat::FoundryV12,
        ]
    }
    
    async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        assets: &[ProcessedAsset], 
        config: &OutputConfig
    ) -> ConversionResult<OutputBundle> {
        let mut bundle = OutputBundle::new();
        
        match config.output_format {
            OutputFormat::FoundryWorld => {
                // Generate world.json
                let world_manifest = self.manifest_generator.generate_world_manifest(campaign, config)?;
                bundle.add_file("world.json", serde_json::to_string_pretty(&world_manifest)?);
                
                // Convert actors to Foundry format
                let actors_db = self.db_writer.create_actors_db(&campaign.actors, config)?;
                bundle.add_database("data/actors.db", actors_db);
                
                // Convert scenes with asset mapping
                let scenes_db = self.db_writer.create_scenes_db(&campaign.scenes, assets, config)?;
                bundle.add_database("data/scenes.db", scenes_db);
                
                // Map and copy assets
                let asset_mappings = self.asset_mapper.map_assets_for_foundry(assets, config)?;
                for (source_path, target_path) in asset_mappings {
                    bundle.add_asset(target_path, source_path);
                }
            }
            OutputFormat::FoundryModule => {
                // Similar but for module generation
                let module_manifest = self.manifest_generator.generate_module_manifest(campaign, config)?;
                bundle.add_file("module.json", serde_json::to_string_pretty(&module_manifest)?);
                
                // Create compendium packs
                let actor_pack = self.db_writer.create_compendium_pack("actors", &campaign.actors, config)?;
                bundle.add_database("packs/actors.db", actor_pack);
            }
            _ => return Err(ConversionError::UnsupportedOutputFormat(config.output_format.clone()))
        }
        
        Ok(bundle)
    }
}
```

#### **2.3: Simple Output Plugins**
```rust
// ttrpg-plugins/src/output/pathbuilder.rs
pub struct PathbuilderOutputPlugin;

#[async_trait]
impl OutputPlugin for PathbuilderOutputPlugin {
    async fn generate_output(
        &self,
        campaign: &UniversalCampaign,
        _assets: &[ProcessedAsset],
        config: &OutputConfig
    ) -> ConversionResult<OutputBundle> {
        let mut bundle = OutputBundle::new();
        
        // Generate Pathbuilder JSON for each character
        for actor in &campaign.actors {
            if actor.actor_type == ActorType::PlayerCharacter {
                let pathbuilder_json = self.convert_to_pathbuilder_format(actor)?;
                let filename = format!("{}_pathbuilder.json", sanitize_filename(&actor.name));
                bundle.add_file(filename, serde_json::to_string_pretty(&pathbuilder_json)?);
            }
        }
        
        Ok(bundle)
    }
    
    fn convert_to_pathbuilder_format(&self, actor: &Actor) -> ConversionResult<serde_json::Value> {
        // Convert Actor to Pathbuilder JSON format
        let pathbuilder_character = json!({
            "name": actor.name,
            "ancestry": actor.race.as_deref().unwrap_or("Human"),
            "background": actor.background.as_deref().unwrap_or(""),
            "class": actor.class.as_deref().unwrap_or("Fighter"),
            "level": actor.level.unwrap_or(1),
            "abilities": {
                "str": actor.ability_scores.get("strength").unwrap_or(&10),
                "dex": actor.ability_scores.get("dexterity").unwrap_or(&10),
                "con": actor.ability_scores.get("constitution").unwrap_or(&10),
                "int": actor.ability_scores.get("intelligence").unwrap_or(&10),
                "wis": actor.ability_scores.get("wisdom").unwrap_or(&10),
                "cha": actor.ability_scores.get("charisma").unwrap_or(&10),
            },
            "skills": actor.skills,
            // ... more Pathbuilder-specific formatting
        });
        
        Ok(pathbuilder_character)
    }
}
```

### **Phase 3: Universal Conversion Pipeline (1 week)**

#### **3.1: Main Conversion Flow**
```rust
// ttrpg-core/src/converter.rs
impl UniversalConverter {
    pub async fn convert(
        &self,
        source: &Path,
        target: &Path,
        config: ConversionConfig
    ) -> ConversionResult<ConversionReport> {
        // 1. Auto-detect and parse input
        let input_plugin = self.plugin_manager.detect_input_plugin(source)?;
        tracing::info!("Detected input format: {}", input_plugin.plugin_info().name);
        
        let mut campaign = input_plugin.parse_campaign(source).await?;
        let raw_assets = input_plugin.discover_assets(&campaign).await?;
        
        // 2. Apply sophisticated middleware processing
        if config.validation.enabled {
            campaign = self.middleware.validation_engine.validate_and_fix(campaign).await?;
        }
        
        if config.system_conversion.enabled {
            campaign = self.middleware.rule_engine.convert_system(
                campaign, 
                config.system_conversion.source_system,
                config.system_conversion.target_system
            ).await?;
        }
        
        let processed_assets = if config.asset_processing.enabled {
            self.middleware.asset_processor.process_assets(raw_assets, &config.asset_processing).await?
        } else {
            raw_assets.into_iter().map(ProcessedAsset::from).collect()
        };
        
        // 3. Generate outputs using appropriate plugins
        let mut results = Vec::new();
        for output_config in &config.outputs {
            let output_plugin = self.plugin_manager.get_output_plugin(output_config.format)?;
            tracing::info!("Generating {} output", output_plugin.plugin_info().name);
            
            let bundle = output_plugin.generate_output(&campaign, &processed_assets, output_config).await?;
            let output_path = target.join(&output_config.subdirectory);
            output_plugin.write_output(bundle, &output_path, &output_config.write_options).await?;
            
            results.push(OutputResult {
                format: output_config.format.clone(),
                path: output_path,
                plugin_info: output_plugin.plugin_info(),
            });
        }
        
        Ok(ConversionReport {
            source_format: input_plugin.plugin_info(),
            outputs: results,
            campaign_stats: CampaignStats::from(&campaign),
            asset_stats: AssetStats::from(&processed_assets),
            processing_time: start_time.elapsed(),
        })
    }
}
```

## 🔧 **MIGRATION STRATEGY**

### **Rework Assessment**
- **Throwaway Work**: ~8 points (Roll20AssetProcessor wrapper, complex service integration)
- **Refactor Work**: ~10 points (Service manager → Plugin manager, CLI updates)  
- **Preserved Work**: ~25 points (Core types, error handling, Roll20Parser logic)
- **Total Rework**: ~18 points (3-4 weeks)

### **Migration Steps**
1. **Week 1**: Implement plugin framework and plugin manager
2. **Week 2**: Convert Roll20Parser to Roll20InputPlugin, create FoundryOutputPlugin  
3. **Week 3**: Add PathbuilderPlugin, DNDBeyondPlugin, integrate middleware
4. **Week 4**: Update CLI, comprehensive testing, documentation

### **Benefits After Migration**
✅ **Linear complexity scaling** (N+M instead of N×M)  
✅ **All planned sophisticated features** preserved  
✅ **Better maintainability** - plugin isolation  
✅ **Easier testing** - mock plugins  
✅ **Incremental delivery** - ship core first, add plugins later  

## 🎯 **SUCCESS METRICS**

- **Functionality**: All 400 points of planned features achievable
- **Complexity**: Linear scaling confirmed (adding Pathfinder 2e = 1 plugin, not N×M work)
- **Performance**: Plugin overhead <5% vs direct implementation
- **Maintainability**: Plugin failures don't cascade to other plugins
- **Extensibility**: New output format implementable in 1-2 days

This comprehensive plugin architecture gives you the best of both worlds - all your sophisticated features with much better architectural properties!
