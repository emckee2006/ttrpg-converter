//! Scene data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core scene data structure - unified representation across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtrpgScene {
    /// Scene unique identifier
    pub id: String,
    /// Scene name
    pub name: String,
    /// Scene description
    pub description: Option<String>,
    /// Scene width in pixels
    pub width: Option<f64>,
    /// Scene height in pixels
    pub height: Option<f64>,
    /// Background image path
    pub background_image: Option<String>,
    /// Background color
    pub background_color: Option<String>,
    /// Grid configuration
    pub grid: Option<GridConfig>,
    /// Lighting configuration
    pub lighting: Option<LightingConfig>,
    /// Tokens in the scene
    pub tokens: Vec<Token>,
    /// Light sources in the scene
    pub light_sources: Vec<LightSource>,
    /// System-specific data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Grid configuration for scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
    /// Grid type
    pub grid_type: GridType,
    /// Grid size in pixels
    pub size: u32,
    /// Distance per grid unit
    pub distance: Option<f64>,
    /// Distance units
    pub units: Option<String>,
}

/// Grid types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridType {
    Square,
    Hexagonal,
    None,
}

/// Scene lighting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightingConfig {
    /// Global illumination level
    pub global_light: Option<f64>,
    /// Darkness level
    pub darkness: Option<f64>,
    /// Fog of war enabled
    pub fog_of_war: Option<bool>,
}

/// Token representation in scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Token ID
    pub id: String,
    /// Token name
    pub name: String,
    /// Position
    pub position: Position,
    /// Token size
    pub size: Option<f64>,
    /// Token image
    pub image: Option<String>,
    /// Vision configuration
    pub vision: Option<VisionConfig>,
    /// Token bars (health, etc.)
    pub bars: Vec<TokenBar>,
}

/// Position coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// Vision configuration for tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionConfig {
    /// Vision range
    pub range: Option<f64>,
    /// Vision angle
    pub angle: Option<f64>,
    /// Night vision
    pub night_vision: Option<bool>,
}

/// Token resource bars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBar {
    /// Bar attribute
    pub attribute: String,
    /// Current value
    pub current: Option<f64>,
    /// Maximum value
    pub max: Option<f64>,
}

/// Light source in scenes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightSource {
    /// Light source ID
    pub id: String,
    /// Position
    pub position: Position,
    /// Bright light radius
    pub bright_radius: Option<f64>,
    /// Dim light radius
    pub dim_radius: Option<f64>,
    /// Light color
    pub color: Option<String>,
    /// Light animation
    pub animation: Option<LightAnimation>,
}

/// Light animation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightAnimation {
    /// Animation type
    pub animation_type: LightAnimationType,
    /// Animation speed
    pub speed: Option<f64>,
    /// Animation intensity
    pub intensity: Option<f64>,
}

/// Light animation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightAnimationType {
    None,
    Pulse,
    Wave,
    Chroma,
    Fog,
    Sunburst,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ttrpg_scene_creation() {
        let scene = TtrpgScene {
            id: "scene_001".to_string(),
            name: "Tavern Interior".to_string(),
            description: Some("A cozy tavern with warm lighting".to_string()),
            width: Some(2048.0),
            height: Some(1536.0),
            background_image: Some("tavern_bg.jpg".to_string()),
            background_color: Some("#654321".to_string()),
            grid: Some(GridConfig {
                grid_type: GridType::Square,
                size: 50,
                distance: Some(5.0),
                units: Some("feet".to_string()),
            }),
            lighting: Some(LightingConfig {
                global_light: Some(0.3),
                darkness: Some(0.0),
                fog_of_war: Some(true),
            }),
            tokens: vec![],
            light_sources: vec![],
            system_data: HashMap::new(),
        };

        assert_eq!(scene.name, "Tavern Interior");
        assert_eq!(scene.width, Some(2048.0));
        assert!(scene.grid.is_some());
        assert!(scene.lighting.is_some());
    }

    #[test]
    fn test_grid_config_functionality() {
        let grid = GridConfig {
            grid_type: GridType::Hexagonal,
            size: 40,
            distance: Some(1.0),
            units: Some("meters".to_string()),
        };

        assert_eq!(grid.size, 40);
        assert_eq!(grid.distance, Some(1.0));
        matches!(grid.grid_type, GridType::Hexagonal);
    }

    #[test]
    fn test_token_creation() {
        let token = Token {
            id: "token_001".to_string(),
            name: "Goblin".to_string(),
            position: Position { x: 100.0, y: 150.0 },
            size: Some(1.0),
            image: Some("goblin.png".to_string()),
            vision: Some(VisionConfig {
                range: Some(60.0),
                angle: Some(360.0),
                night_vision: Some(false),
            }),
            bars: vec![
                TokenBar {
                    attribute: "hp".to_string(),
                    current: Some(8.0),
                    max: Some(8.0),
                }
            ],
        };

        assert_eq!(token.name, "Goblin");
        assert_eq!(token.position.x, 100.0);
        assert_eq!(token.bars.len(), 1);
    }

    #[test]
    fn test_light_source_creation() {
        let light = LightSource {
            id: "light_001".to_string(),
            position: Position { x: 200.0, y: 300.0 },
            bright_radius: Some(20.0),
            dim_radius: Some(40.0),
            color: Some("#ff6600".to_string()),
            animation: Some(LightAnimation {
                animation_type: LightAnimationType::Pulse,
                speed: Some(2.0),
                intensity: Some(0.8),
            }),
        };

        assert_eq!(light.bright_radius, Some(20.0));
        assert!(light.animation.is_some());
        matches!(light.animation.as_ref().unwrap().animation_type, LightAnimationType::Pulse);
    }

    #[test]
    fn test_grid_type_variants() {
        let square = GridType::Square;
        let hex = GridType::Hexagonal;
        let none = GridType::None;

        matches!(square, GridType::Square);
        matches!(hex, GridType::Hexagonal);
        matches!(none, GridType::None);
    }

    #[test]
    fn test_light_animation_types() {
        let animations = vec![
            LightAnimationType::None,
            LightAnimationType::Pulse,
            LightAnimationType::Wave,
            LightAnimationType::Chroma,
            LightAnimationType::Fog,
            LightAnimationType::Sunburst,
        ];

        assert_eq!(animations.len(), 6);
    }

    #[test]
    fn test_vision_config() {
        let vision = VisionConfig {
            range: Some(120.0),
            angle: Some(90.0),
            night_vision: Some(true),
        };

        assert_eq!(vision.range, Some(120.0));
        assert_eq!(vision.night_vision, Some(true));
    }
}