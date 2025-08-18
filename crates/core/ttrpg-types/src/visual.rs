//! Visual and positioning types

use serde::{Deserialize, Serialize};

/// Dimensions for objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    /// Width
    pub width: f64,
    /// Height
    pub height: f64,
}

/// Texture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Texture {
    /// Texture source path
    pub src: String,
    /// Texture scaling
    pub scale_x: Option<f64>,
    /// Texture scaling Y
    pub scale_y: Option<f64>,
    /// Texture tint
    pub tint: Option<String>,
}

/// Token aura effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAura {
    /// Aura radius
    pub radius: f64,
    /// Aura color
    pub color: String,
    /// Aura opacity
    pub opacity: Option<f64>,
}