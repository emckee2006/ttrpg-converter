use crate::generated::scene::FoundryScene;
use ttrpg_types::{TtrpgScene, GridType, GridConfig, LightingConfig};
use serde_json::json;
use std::collections::HashMap;

impl From<FoundryScene> for TtrpgScene {
    fn from(scene: FoundryScene) -> Self {
        let mut system_data = HashMap::new();

        // Extract basic information
        let id = scene.id.to_string();
        let name = scene.name.to_string();
        let background_image = scene.img;
        let background_color = scene.background_color;
        let width = Some(scene.width.unwrap_or(1920.0)); // Foundry default
        let height = Some(scene.height.unwrap_or(1080.0)); // Foundry default

        // Handle grid configuration
        let grid = scene.grid.map(|grid_data| {
            let grid_type = foundry_grid_to_ttrpg(0); // TODO: Extract grid type from schema
            
            // Store grid-specific system data
            if let Some(distance) = grid_data.distance {
                system_data.insert("grid_distance".to_string(), json!(distance));
            }
            if let Some(ref units) = grid_data.units {
                system_data.insert("grid_units".to_string(), json!(units));
            }
            if let Some(color) = grid_data.color {
                system_data.insert("grid_color".to_string(), json!(color));
            }
            if let Some(alpha) = grid_data.alpha {
                system_data.insert("grid_alpha".to_string(), json!(alpha));
            }

            GridConfig {
                size: 1,
                grid_type,
                distance: grid_data.distance,
                units: Some(grid_data.units.unwrap_or_else(|| "feet".to_string())),
            }
        });

        // Handle lighting configuration
        let lighting = Some(LightingConfig {
            global_light: scene.global_light.map(|v| if v { 1.0 } else { 0.0 }),
            darkness: scene.darkness.map(|v| v as f64),
            fog_of_war: scene.fog_exploration,
        });

        // Store Foundry-specific data in system_data
        if let Some(padding) = scene.padding {
            system_data.insert("padding".to_string(), json!(padding));
        }
        if let Some(initial) = scene.initial {
            system_data.insert("initial_view_x".to_string(), json!(initial.x));
            system_data.insert("initial_view_y".to_string(), json!(initial.y));
            // TODO: Extract scale from position when available
            let scale = 1.0;
            system_data.insert("initial_scale".to_string(), json!(scale));
        }
        if let Some(active) = scene.active {
            system_data.insert("active".to_string(), json!(active));
        }
        if let Some(navigation) = scene.navigation {
            system_data.insert("navigation".to_string(), json!(navigation));
        }
        if let Some(ownership) = scene.ownership {
            system_data.insert("ownership".to_string(), json!(ownership));
        }
        if let Some(darkness) = scene.darkness {
            system_data.insert("darkness_level".to_string(), json!(darkness));
        }
        if let Some(token_vision) = scene.token_vision {
            system_data.insert("token_vision".to_string(), json!(token_vision));
        }
        
        // Store additional scene data
        system_data.insert("flags".to_string(), json!(scene.flags));

        TtrpgScene {
            description: None,
            id,
            name,
            background_image,
            background_color: background_color.map(|c| format!("{:?}", c)),
            width,
            height,
            grid,
            lighting,
            tokens: Vec::new(),
            light_sources: Vec::new(),
            system_data,
        }
    }
}

pub fn foundry_grid_to_ttrpg(grid_type: i32) -> GridType {
    match grid_type {
        0 => GridType::Square,
        1 => GridType::Hexagonal,
        2 => GridType::Hexagonal, // Flat-topped hex
        _ => GridType::Square, // Default fallback
    }
}
