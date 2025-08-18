//! Hero Lab JSON Output Handler

use async_trait::async_trait;
use std::path::Path;
use tracing::info;

use ttrpg_core::plugin_framework::{OutputPlugin, PluginInfo, ConversionResult, OutputBundle, WriteOptions, OutputConfig};
use ttrpg_core::types::{Campaign, OutputFormat};

#[derive(Debug, Clone)]
pub struct HeroLabJsonHandler;

impl Default for HeroLabJsonHandler {
    fn default() -> Self {
        Self
    }
}

impl HeroLabJsonHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OutputPlugin for HeroLabJsonHandler {
    fn plugin_info(&self) -> PluginInfo {
        PluginInfo {
            name: "Hero Lab JSON Output Plugin".to_string(),
            version: "0.1.0".to_string(),
            description: "Exports campaigns as Hero Lab JSON format".to_string(),
            author: "TTRPG Converter".to_string(),
            supported_features: vec!["json".to_string(), "herolab".to_string()],
            dependencies: Vec::new(),
        }
    }

    fn supported_formats(&self) -> Vec<OutputFormat> {
        vec![OutputFormat::HeroLabJson]
    }

    fn validate_config(&self, _config: &OutputConfig) -> ttrpg_core::plugin_framework::PluginResult<()> {
        Ok(())
    }

    async fn generate_output(
        &self,
        _campaign: &Campaign,
        _assets: &[ttrpg_core::plugin_framework::AssetInfo],
        _config: &OutputConfig,
    ) -> ConversionResult<OutputBundle> {
        info!("Generating Hero Lab JSON output (stub implementation)");
        
        Ok(OutputBundle {
            format: OutputFormat::HeroLabJson,
            files: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        })
    }

    async fn write_output(
        &self,
        _bundle: OutputBundle,
        _target: &Path,
        _options: &WriteOptions,
    ) -> ConversionResult<()> {
        info!("Writing Hero Lab JSON output (stub implementation)");
        Ok(())
    }
}
