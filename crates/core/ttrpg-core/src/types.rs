//! Core data types and structures for TTRPGConverter
//!
//! This module provides basic types that don't depend on external schemas
//! or plugin framework types to avoid circular dependencies.

use serde::{Deserialize, Serialize};

/// Source format enumeration for campaigns
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceFormat {
    Roll20,
    FoundryV9,
    FoundryV10,
    FoundryV11,
    FoundryV12,
    PathbuilderJson,
}

/// Target format enumeration for output
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetFormat {
    FoundryV9,
    FoundryV10,
    FoundryV11,
    FoundryV12,
    Roll20,
    Json,
}

impl std::fmt::Display for SourceFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFormat::Roll20 => write!(f, "Roll20"),
            SourceFormat::FoundryV9 => write!(f, "Foundry VTT v9"),
            SourceFormat::FoundryV10 => write!(f, "Foundry VTT v10"),
            SourceFormat::FoundryV11 => write!(f, "Foundry VTT v11"),
            SourceFormat::FoundryV12 => write!(f, "Foundry VTT v12"),
            SourceFormat::PathbuilderJson => write!(f, "Pathbuilder JSON"),
        }
    }
}

impl std::fmt::Display for TargetFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetFormat::FoundryV9 => write!(f, "Foundry VTT v9"),
            TargetFormat::FoundryV10 => write!(f, "Foundry VTT v10"),
            TargetFormat::FoundryV11 => write!(f, "Foundry VTT v11"),
            TargetFormat::FoundryV12 => write!(f, "Foundry VTT v12"),
            TargetFormat::Roll20 => write!(f, "Roll20"),
            TargetFormat::Json => write!(f, "JSON"),
        }
    }
}
