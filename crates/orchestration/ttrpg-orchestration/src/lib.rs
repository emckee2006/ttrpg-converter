//! Plugin Orchestration and Pipeline Management
//!
//! This crate provides the top-level orchestration system for managing plugin
//! discovery, dependency injection, pipeline execution, and plugin lifecycle.

pub mod discovery;
pub mod pipeline;

pub use discovery::PluginDiscovery;
pub use pipeline::{
    PluginPipeline, PipelineBuilder, PipelineNode, PipelinePluginType, 
    PipelineConfig, PipelineContext, PipelineStats
};

/// Re-exports from plugin traits for convenience
pub use ttrpg_traits::PluginInfo;
