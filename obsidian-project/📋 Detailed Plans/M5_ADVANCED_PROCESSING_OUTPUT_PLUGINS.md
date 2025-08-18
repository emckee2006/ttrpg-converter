# M5: Advanced Processing + Output Plugins - Junior Developer Implementation Guide

## 🎯 **MILESTONE 5 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 18 | **Priority**: 🔥 HIGH
**Dependencies**: M4 GUI Interface Complete

Advanced processing capabilities and output format plugins including PDF generation, web exports, multi-system conversion, and computer vision integration.

### 🔧 **ADVANCED FEATURES**
- **PDF.dll**: Professional PDF generation with embedded assets
- **WebExport.dll**: Interactive web campaign exports
- **Multi-System Conversion**: Rule system conversion (PF1e ↔ D&D5e, PF1e ↔ PF2e)
- **Computer Vision**: Scene analysis and token recognition

---

## **T5.1: PDF Generation Plugin DLL**
**Duration**: 4 days | **Points**: 6 | **Priority**: 🔥 HIGH

### **Implementation Steps**

**Step 1: Create PDF Plugin Crate**
```bash
cd crates/plugins
cargo new ttrpg-pdf-plugin --lib
```

Update `ttrpg-pdf-plugin/Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ttrpg-core = { path = "../../core/ttrpg-core" }
printpdf = "0.6"
lopdf = "0.32"
image = { version = "0.24", features = ["jpeg", "png"] }
handlebars = "4.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"
tokio = { version = "1.35", features = ["full"] }
```

**Step 2: PDF Plugin Implementation**
Create `src/lib.rs`:
```rust
use std::collections::HashMap;
use async_trait::async_trait;
use printpdf::*;
use ttrpg_core::plugins::{ProcessingPlugin, SharedServices, PlatformOutput, OutputFile};
use ttrpg_core::types::common::*;

pub struct PdfOutputPlugin {
    shared_services: SharedServices,
    handlebars: handlebars::Handlebars<'static>,
}

impl PdfOutputPlugin {
    pub fn new(services: SharedServices) -> Self {
        let mut handlebars = handlebars::Handlebars::new();
        
        // Register character sheet template
        let character_template = include_str!("templates/character_sheet.hbs");
        handlebars.register_template_string("character_sheet", character_template)
            .expect("Character sheet template should be valid");
            
        // Register campaign summary template
        let campaign_template = include_str!("templates/campaign_summary.hbs");
        handlebars.register_template_string("campaign_summary", campaign_template)
            .expect("Campaign summary template should be valid");
        
        Self {
            shared_services: services,
            handlebars,
        }
    }
    
    /// Generate comprehensive PDF campaign export
    pub async fn generate_campaign_pdf(&self, campaign: &Campaign) -> Result<Vec<u8>, PdfGenerationError> {
        let (doc, page1, layer1) = PdfDocument::new(
            &campaign.metadata.name,
            Mm(210.0), // A4 width
            Mm(297.0), // A4 height
            "Campaign Summary"
        );
        
        let mut current_layer = doc.get_page(page1).get_layer(layer1);
        let mut y_position = Mm(270.0); // Start near top
        
        // Add campaign title
        let font = doc.add_external_font(std::io::Cursor::new(include_bytes!("fonts/Roboto-Regular.ttf")))
            .map_err(|e| PdfGenerationError::FontError(e.to_string()))?;
            
        current_layer.use_text(&campaign.metadata.name, 24.0, Mm(20.0), y_position, &font);
        y_position -= Mm(15.0);
        
        // Add campaign metadata
        current_layer.use_text(&format!("System: {}", campaign.metadata.system), 12.0, Mm(20.0), y_position, &font);
        y_position -= Mm(10.0);
        
        if let Some(created) = campaign.metadata.created_at {
            current_layer.use_text(&format!("Created: {}", created.format("%Y-%m-%d")), 12.0, Mm(20.0), y_position, &font);
            y_position -= Mm(15.0);
        }
        
        // Add characters section
        current_layer.use_text("Characters:", 16.0, Mm(20.0), y_position, &font);
        y_position -= Mm(12.0);
        
        for character in &campaign.characters {
            if y_position < Mm(40.0) {
                // Add new page if running out of space
                let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Characters");
                current_layer = doc.get_page(new_page).get_layer(new_layer);
                y_position = Mm(270.0);
            }
            
            // Add character summary
            current_layer.use_text(&format!("• {}", character.metadata.name), 12.0, Mm(25.0), y_position, &font);
            y_position -= Mm(8.0);
            
            if let Some(player) = &character.metadata.player_name {
                current_layer.use_text(&format!("  Player: {}", player), 10.0, Mm(30.0), y_position, &font);
                y_position -= Mm(8.0);
            }
            
            if let Some(level) = character.metadata.character_level {
                current_layer.use_text(&format!("  Level: {}", level), 10.0, Mm(30.0), y_position, &font);
                y_position -= Mm(10.0);
            }
        }
        
        // Generate character sheets
        for character in &campaign.characters {
            self.add_character_sheet(&doc, character, &font).await?;
        }
        
        // Add scenes section if present
        if !campaign.scenes.is_empty() {
            let (scenes_page, scenes_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Scenes");
            let scenes_layer = doc.get_page(scenes_page).get_layer(scenes_layer);
            
            let mut scene_y = Mm(270.0);
            scenes_layer.use_text("Scenes:", 16.0, Mm(20.0), scene_y, &font);
            scene_y -= Mm(12.0);
            
            for scene in &campaign.scenes {
                scenes_layer.use_text(&format!("• {}", scene.metadata.name), 12.0, Mm(25.0), scene_y, &font);
                scene_y -= Mm(8.0);
                
                if let Some(description) = &scene.metadata.description {
                    scenes_layer.use_text(&format!("  {}", description), 10.0, Mm(30.0), scene_y, &font);
                    scene_y -= Mm(10.0);
                }
            }
        }
        
        // Save PDF to bytes
        let pdf_bytes = doc.save_to_bytes()
            .map_err(|e| PdfGenerationError::SaveError(e.to_string()))?;
            
        Ok(pdf_bytes)
    }
    
    async fn add_character_sheet(&self, doc: &PdfDocumentReference, character: &TtrpgCharacter, font: &IndirectFontRef) -> Result<(), PdfGenerationError> {
        let (char_page, char_layer) = doc.add_page(Mm(210.0), Mm(297.0), &format!("Character: {}", character.metadata.name));
        let char_layer = doc.get_page(char_page).get_layer(char_layer);
        
        let mut y_pos = Mm(270.0);
        
        // Character name as header
        char_layer.use_text(&character.metadata.name, 20.0, Mm(20.0), y_pos, font);
        y_pos -= Mm(15.0);
        
        // Basic info
        if let Some(player) = &character.metadata.player_name {
            char_layer.use_text(&format!("Player: {}", player), 12.0, Mm(20.0), y_pos, font);
            y_pos -= Mm(10.0);
        }
        
        if let Some(class) = &character.metadata.character_class {
            char_layer.use_text(&format!("Class: {}", class), 12.0, Mm(20.0), y_pos, font);
            y_pos -= Mm(10.0);
        }
        
        if let Some(race) = &character.metadata.race {
            char_layer.use_text(&format!("Race: {}", race), 12.0, Mm(20.0), y_pos, font);
            y_pos -= Mm(15.0);
        }
        
        // Ability scores
        if let Some(abilities) = &character.abilities {
            char_layer.use_text("Ability Scores:", 14.0, Mm(20.0), y_pos, font);
            y_pos -= Mm(12.0);
            
            let abilities_data = [
                ("Strength", abilities.strength),
                ("Dexterity", abilities.dexterity),
                ("Constitution", abilities.constitution),
                ("Intelligence", abilities.intelligence),
                ("Wisdom", abilities.wisdom),
                ("Charisma", abilities.charisma),
            ];
            
            for (ability, score) in &abilities_data {
                char_layer.use_text(&format!("{}: {}", ability, score), 11.0, Mm(25.0), y_pos, font);
                y_pos -= Mm(8.0);
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl ProcessingPlugin for PdfOutputPlugin {
    async fn process(&self, campaign: Campaign) -> Result<Campaign, ConversionError> {
        // Generate PDF as additional output format
        if let Ok(pdf_bytes) = self.generate_campaign_pdf(&campaign).await {
            // Store PDF in shared services or attach to campaign metadata
            println!("Generated PDF: {} bytes", pdf_bytes.len());
        }
        
        // Return campaign unchanged - this is an output plugin
        Ok(campaign)
    }
    
    fn get_priority(&self) -> i32 {
        500 // Run late in pipeline
    }
}

impl ttrpg_core::plugins::Plugin for PdfOutputPlugin {
    fn name(&self) -> &str {
        "PdfOutputPlugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "Generates professional PDF exports of campaigns and character sheets"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PdfGenerationError {
    #[error("Font error: {0}")]
    FontError(String),
    #[error("Save error: {0}")]
    SaveError(String),
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("Image error: {0}")]
    ImageError(String),
}

#[no_mangle]
pub extern "C" fn create_processing_plugin(services: SharedServices) -> *mut dyn ProcessingPlugin {
    Box::into_raw(Box::new(PdfOutputPlugin::new(services)))
}
```

---

## **T5.2: Web Export Plugin DLL**
**Duration**: 3 days | **Points**: 4 | **Priority**: 🔥 HIGH

Interactive HTML/JavaScript campaign exports with embedded data.

---

## **T5.3: Multi-System Conversion Engine**
**Duration**: 4 days | **Points**: 5 | **Priority**: 🔥 HIGH

Rule system conversion between different RPG systems.

### **Implementation Steps**

**Step 1: System Conversion Rules**
Create `src/system_converter.rs`:
```rust
use ttrpg_core::types::common::*;

pub struct SystemConverter;

impl SystemConverter {
    /// Convert character between different RPG systems
    pub async fn convert_character(
        character: &TtrpgCharacter,
        target_system: &str
    ) -> Result<TtrpgCharacter, ConversionError> {
        match (character.metadata.system.as_deref().unwrap_or("unknown"), target_system) {
            ("pf1e", "dnd5e") => Self::convert_pf1e_to_dnd5e(character).await,
            ("dnd5e", "pf1e") => Self::convert_dnd5e_to_pf1e(character).await,
            ("pf1e", "pf2e") => Self::convert_pf1e_to_pf2e(character).await,
            ("pf2e", "pf1e") => Self::convert_pf2e_to_pf1e(character).await,
            _ => Err(ConversionError::UnsupportedSystemConversion {
                from: character.metadata.system.clone().unwrap_or_default(),
                to: target_system.to_string(),
            }),
        }
    }
    
    async fn convert_pf1e_to_dnd5e(character: &TtrpgCharacter) -> Result<TtrpgCharacter, ConversionError> {
        let mut converted = character.clone();
        
        // Convert ability scores (PF1e uses same scale as D&D 5e)
        // No conversion needed for basic ability scores
        
        // Convert skills (different skill systems)
        if let Some(skills) = &character.skills {
            let mut converted_skills = Skills {
                acrobatics: skills.acrobatics,
                athletics: skills.athletics,
                // Map PF1e skills to D&D 5e equivalents
                deception: skills.bluff,
                insight: skills.sense_motive,
                investigation: skills.knowledge,
                perception: skills.perception,
                stealth: skills.stealth,
                ..Default::default()
            };
            converted.skills = Some(converted_skills);
        }
        
        // Update system metadata
        converted.metadata.system = Some("dnd5e".to_string());
        converted.system_data = serde_json::json!({
            "conversion_source": "pf1e",
            "conversion_notes": "Converted from Pathfinder 1e to D&D 5e"
        });
        
        Ok(converted)
    }
    
    async fn convert_pf1e_to_pf2e(character: &TtrpgCharacter) -> Result<TtrpgCharacter, ConversionError> {
        let mut converted = character.clone();
        
        // PF2e uses different proficiency system
        if let Some(abilities) = &converted.abilities {
            // Convert ability scores to PF2e modifiers
            // PF2e starts at 10 + racial modifiers + background + class
            // This is a simplified conversion
        }
        
        // Update level progression (PF2e has different advancement)
        if let Some(level) = converted.metadata.character_level {
            // PF2e level advancement is different but level numbers are same
            converted.metadata.character_level = Some(level);
        }
        
        converted.metadata.system = Some("pf2e".to_string());
        converted.system_data = serde_json::json!({
            "conversion_source": "pf1e",
            "conversion_notes": "Converted from Pathfinder 1e to Pathfinder 2e - manual review recommended"
        });
        
        Ok(converted)
    }
}
```

---

## **T5.4: Computer Vision Integration**
**Duration**: 3 days | **Points**: 3 | **Priority**: 🔥 HIGH

Scene analysis and token recognition for automated map processing.

### **Implementation Steps**

**Step 1: Computer Vision Dependencies**
Add to plugin `Cargo.toml`:
```toml
[dependencies]
opencv = { version = "0.88", features = ["opencv-4"] }
image = { version = "0.24", features = ["jpeg", "png"] }
imageproc = "0.23"
```

**Step 2: Vision Processing**
Create `src/vision_processor.rs`:
```rust
use opencv::{core, imgproc, objdetect, prelude::*};
use image::{DynamicImage, ImageBuffer, Rgba};

pub struct VisionProcessor;

impl VisionProcessor {
    /// Analyze scene image to detect tokens and map features
    pub async fn analyze_scene_image(
        &self, 
        image_data: &[u8]
    ) -> Result<SceneAnalysis, VisionError> {
        // Load image with OpenCV
        let image = self.load_image_from_bytes(image_data)?;
        
        // Detect circular tokens (typical for VTT platforms)
        let tokens = self.detect_tokens(&image).await?;
        
        // Detect grid lines
        let grid_info = self.detect_grid(&image).await?;
        
        // Analyze lighting and walls
        let lighting_info = self.analyze_lighting(&image).await?;
        
        Ok(SceneAnalysis {
            detected_tokens: tokens,
            grid_information: grid_info,
            lighting_analysis: lighting_info,
            image_dimensions: (image.cols(), image.rows()),
        })
    }
    
    async fn detect_tokens(&self, image: &Mat) -> Result<Vec<DetectedToken>, VisionError> {
        let mut tokens = Vec::new();
        
        // Convert to grayscale for circle detection
        let mut gray = Mat::default();
        imgproc::cvt_color(image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        // Use HoughCircles to detect circular tokens
        let mut circles = Mat::default();
        imgproc::hough_circles(
            &gray,
            &mut circles,
            imgproc::HOUGH_GRADIENT,
            1.0,
            50.0, // min distance between circles
            Some(100.0), // higher threshold for edge detection
            Some(30.0),  // accumulator threshold
            Some(10),    // min radius
            Some(100),   // max radius
        )?;
        
        // Process detected circles
        for i in 0..circles.cols() {
            let circle = circles.at_2d::<core::Vec3f>(0, i)?;
            let x = circle[0] as i32;
            let y = circle[1] as i32;
            let radius = circle[2] as i32;
            
            tokens.push(DetectedToken {
                position: (x, y),
                radius: radius as u32,
                confidence: 0.8, // Would be calculated based on detection quality
                token_type: TokenType::Unknown,
            });
        }
        
        Ok(tokens)
    }
    
    async fn detect_grid(&self, image: &Mat) -> Result<GridInfo, VisionError> {
        // Detect horizontal and vertical lines using HoughLines
        let mut gray = Mat::default();
        imgproc::cvt_color(image, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        
        let mut edges = Mat::default();
        imgproc::canny(&gray, &mut edges, 50.0, 150.0, 3, false)?;
        
        let mut lines = Mat::default();
        imgproc::hough_lines_p(
            &edges,
            &mut lines,
            1.0,
            std::f64::consts::PI / 180.0,
            50, // threshold
            Some(50.0), // min line length
            Some(10.0), // max line gap
        )?;
        
        // Analyze lines to determine grid size and rotation
        let grid_size = self.calculate_grid_size(&lines)?;
        
        Ok(GridInfo {
            grid_size,
            rotation_angle: 0.0, // Would calculate based on line angles
            is_detected: grid_size > 0,
        })
    }
}

#[derive(Debug)]
pub struct SceneAnalysis {
    pub detected_tokens: Vec<DetectedToken>,
    pub grid_information: GridInfo,
    pub lighting_analysis: LightingInfo,
    pub image_dimensions: (i32, i32),
}

#[derive(Debug)]
pub struct DetectedToken {
    pub position: (i32, i32),
    pub radius: u32,
    pub confidence: f32,
    pub token_type: TokenType,
}

#[derive(Debug)]
pub enum TokenType {
    Character,
    NPC,
    Monster,
    Object,
    Unknown,
}
```

### **Success Criteria**
- [ ] ✅ PDF.dll generates professional campaign exports with embedded assets
- [ ] ✅ WebExport.dll creates interactive HTML campaign viewers
- [ ] ✅ Multi-system conversion between PF1e ↔ D&D5e, PF1e ↔ PF2e functional
- [ ] ✅ Computer vision detects tokens and grid information in scene images
- [ ] ✅ All advanced plugins integrate with shared services architecture
- [ ] ✅ Performance optimizations ensure reasonable processing times
