//! TTRPG Plugin Traits
//!
//! Core plugin traits and interfaces for the TTRPG converter system.
//! This crate provides essential plugin interfaces that depend on ttrpg-core for error handling.

pub mod asset;
pub mod export;
pub mod input;
pub mod output;
pub mod processing;
pub mod validation;

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::PathBuf;
use ttrpg_core::prelude::*;

/// Core plugin information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub supported_features: Vec<String>,
    pub dependencies: Vec<String>,
}

impl PluginInfo {
    /// Create new plugin info
    pub fn new(name: &str, version: &str, description: &str, author: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            supported_features: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    /// Add a feature (builder pattern)
    pub fn with_feature(mut self, feature: &str) -> Self {
        self.supported_features.push(feature.to_string());
        self
    }

    /// Add multiple features (builder pattern)
    pub fn with_features(mut self, features: Vec<String>) -> Self {
        self.supported_features.extend(features);
        self
    }

    /// Add a dependency (builder pattern)
    pub fn with_dependency(mut self, dependency: &str) -> Self {
        self.dependencies.push(dependency.to_string());
        self
    }
}

/// Plugin configuration
#[derive(Debug, Clone, Default)]
pub struct PluginConfig {
    pub config_data: HashMap<String, JsonValue>,
    pub cache_dir: Option<PathBuf>,
    pub temp_dir: Option<PathBuf>,
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}

/// Logging statistics
#[derive(Debug, Clone, Default)]
pub struct LoggingStats {
    pub messages_logged: usize,
    pub errors_logged: usize,
    pub warnings_logged: usize,
    pub start_time: Option<std::time::Instant>,
}

// Re-exports for convenience
pub use asset::*;
pub use export::*;
pub use input::*;
pub use output::*;
pub use processing::*;
pub use validation::*;