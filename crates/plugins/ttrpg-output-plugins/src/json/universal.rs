//! Universal JSON Output Handler

use async_trait::async_trait;
use std::path::Path;
use tracing::info;

use ttrpg_core::plugin_framework::{OutputPlugin, PluginInfo, ConversionResult, OutputBundle, WriteOptions, OutputConfig};
use ttrpg_core::types::{Campaign, OutputFormat};

#[derive(Debug, Clone)]
pub struct UniversalJsonHandler;

impl Default for UniversalJsonHandler {
    fn default() -> Self {
        Self
    }
}

impl UniversalJsonHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OutputPlugin for UniversalJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Universal JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Exports campaigns as universal JSON format".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["json".to_string(), "universal".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::UniversalJson]
    }

    fn validate_config(&self, _config: &OutputConfig) -> ttrpg_core::plugin_framework::PluginResult<()> {
        Ok(())
    }

    async fn generate_output(
        &self,
        campaign: &Campaign,
        _assets: &[ttrpg_core::plugin_framework::AssetInfo],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        info!("Generating universal JSON output");
        
        let json_content = serde_json::to_string_pretty(campaign)
            .map_err(|e| ttrpg_core::plugin_framework::ConversionError::from_json(e, "Universal JSON export", None))?;
        
        let mut files = std::collections::HashMap::new();
        files.insert("campaign.json".to_string(), json_content.into_bytes());
        
        Ok(OutputBundle {
            format: OutputFormat::UniversalJson,
            files,
            metadata: std::collections::HashMap::new(),
        })
    }

    async fn write_output(
        &self,
        bundle: OutputBundle,
        target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        info!("Writing universal JSON output to {}", target.display());
        
        std::fs::create_dir_all(target)
            .map_err(|e| ttrpg_core::plugin_framework::ConversionError::from_io(e, target.to_string_lossy().to_string()))?;
        
        for (filename, content) in bundle.files {
            let file_path = target.join(filename);
            std::fs::write(&file_path, content)
                .map_err(|e| ttrpg_core::plugin_framework::ConversionError::from_io(e, file_path.to_string_lossy().to_string()))?;
        }
        
        Ok(())
    }
}
