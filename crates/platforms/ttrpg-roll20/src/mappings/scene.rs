//! Roll20 Page to TTRPG Scene mappings

use crate::generated::page::Page;
use ttrpg_types::{TtrpgScene, GridConfig, GridType, LightingConfig};
use serde_json::json;
use std::collections::HashMap;

impl From<Page> for TtrpgScene {
    fn from(page: Page) -> Self {
        let mut system_data = HashMap::new();
        
        // Extract basic information
        let id = page.id.to_string();
        let name = page.name.to_string();
        
        // Handle grid configuration
        let grid = page.grid_size.map(|size| GridConfig {
            grid_type: GridType::Square, // Roll20 default
            size: size as u32,
            // Roll20 grid config maps to distance and units
            distance: Some(size as f64),
            units: Some("ft".to_string()), // Default to feet
        });

        // Handle lighting configuration
        let lighting = LightingConfig {
            global_light: Some(if page.lightglobalillum.unwrap_or(false) { 1.0 } else { 0.0 }),
            darkness: Some(0.0), // Roll20 doesn't expose this directly
            fog_of_war: Some(page.lightenforcelos.unwrap_or(false))
        };

        // Store Roll20-specific data in system_data
        if let Some(archived) = page.archived {
            system_data.insert("archived".to_string(), json!(archived));
        }
        if let Some(fog_opacity) = page.fog_opacity {
            system_data.insert("fog_opacity".to_string(), json!(fog_opacity));
        }
        if let Some(light_restrict) = page.lightrestrictmovement {
            system_data.insert("light_restrict_movement".to_string(), json!(light_restrict));
        }
        if let Some(light_update) = page.lightupdatedrop {
            system_data.insert("light_update_drop".to_string(), json!(light_update));
        }

        // Handle dimensions
        let width = page.width; // Roll20 page width
        let height = page.height; // Roll20 page height

        TtrpgScene {
            id,
            name,
            description: None, // Roll20 pages don't have descriptions
            background_image: None, // Roll20 pages use background_color
            background_color: page.background_color,
            width,
            height,
            grid,
            lighting: Some(lighting),
            tokens: Vec::new(), // Tokens stored separately in Roll20
            light_sources: Vec::new(), // Lights stored separately in Roll20
            system_data,
        }
    }
}
