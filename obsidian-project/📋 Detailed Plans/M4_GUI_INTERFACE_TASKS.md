# M4: GUI Interface Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 4 OVERVIEW** (MASSIVELY EXPANDED SCOPE)
**Duration**: 4 weeks | **Total Points**: 45 | **Priority**: 🔥 HIGH

Advanced native GUI with egui featuring sophisticated batch processing interface, campaign preview, asset optimization settings, and all advanced conversion options from M3 CLI integration.

### 🚨 **EXPANDED SCOPE BASED ON M3 CLI INTEGRATION**
Major scope expansion to provide GUI access to all 30+ CLI features:
- **Advanced Batch Processing Interface**: Drag & drop multiple campaigns, parallel processing visualization
- **Campaign Preview & Validation**: Visual campaign inspection before conversion with asset thumbnails
- **Asset Optimization Settings GUI**: Interactive asset processing configuration with real-time previews
- **Advanced Conversion Options**: GUI access to all wall processing, door detection, and module export options
- **Professional Progress Visualization**: Multi-threaded progress tracking with detailed statistics panels
- **Interactive Campaign Customization**: GUI wizard mode complementing CLI interactive mode
- **Configuration Management UI**: Visual TOML config editor with live validation

### 📐 **PROFESSIONAL GUI FRAMEWORKS**
Eliminate reinvented wheels with battle-tested libraries:
- `egui` (v0.24) - Immediate mode GUI with professional widgets and styling
- `eframe` - Native windowing with cross-platform support
- `rfd` - Native file dialogs for professional file selection
- `image` - Thumbnail generation and preview support
- `tokio` - Async GUI operations without blocking UI thread

---

## **T4.1: egui Application Foundation**
**Duration**: 3 days | **Points**: 7 | **Priority**: 🟡 MEDIUM
**Dependencies**: M3 Complete (CLI foundation)

### **Implementation Steps for Junior Developer**

**Step 1: Set Up ttrpg-gui Crate**
```bash
cd crates\ttrpg-gui
```

Update `Cargo.toml`:
```toml
[dependencies]
egui = { workspace = true }
eframe = { workspace = true }
ttrpg-core = { path = "../ttrpg-core" }
ttrpg-cli = { path = "../ttrpg-cli" }
serde = { workspace = true }
tokio = { workspace = true }

[[bin]]
name = "ttrpg-gui"
path = "src/main.rs"
```

**Step 2: Create Application Structure**
Create `src/app.rs`:
```rust
use eframe::egui;
use ttrpg_core::prelude::*;

pub struct TTRPGConverterApp {
    // Application state
    pub conversion_state: ConversionState,
    pub ui_state: UIState,
    pub config: AppConfig,
}

#[derive(Default)]
pub struct ConversionState {
    pub input_path: Option<String>,
    pub output_path: Option<String>, 
    pub source_format: SourceFormat,
    pub target_format: TargetFormat,
    pub progress: f32,
    pub is_converting: bool,
    pub conversion_result: Option<ConversionResult<()>>,
}

#[derive(Default)]
pub struct UIState {
    pub show_settings: bool,
    pub show_about: bool,
    pub dark_mode: bool,
    pub selected_tab: Tab,
}

#[derive(Default, PartialEq)]
pub enum Tab {
    #[default]
    Convert,
    Validate,
    Info,
    Settings,
}

impl eframe::App for TTRPGConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu_bar(ctx);
        self.show_main_content(ctx);
        self.show_status_bar(ctx);
    }
}
```

**Step 3: Implement Basic UI Layout**
```rust
impl TTRPGConverterApp {
    fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Campaign...").clicked() {
                        self.open_file_dialog();
                    }
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.ui_state.dark_mode, "Dark Mode");
                    if ui.button("Settings").clicked() {
                        self.ui_state.show_settings = true;
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.ui_state.show_about = true;
                    }
                });
            });
        });
    }

    fn show_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.ui_state.selected_tab, Tab::Convert, "Convert");
                ui.selectable_value(&mut self.ui_state.selected_tab, Tab::Validate, "Validate");
                ui.selectable_value(&mut self.ui_state.selected_tab, Tab::Info, "Info");
                ui.selectable_value(&mut self.ui_state.selected_tab, Tab::Settings, "Settings");
            });
            
            ui.separator();
            
            // Tab content
            match self.ui_state.selected_tab {
                Tab::Convert => self.show_convert_tab(ui),
                Tab::Validate => self.show_validate_tab(ui),
                Tab::Info => self.show_info_tab(ui),
                Tab::Settings => self.show_settings_tab(ui),
            }
        });
    }
}
```

**Acceptance Criteria**:
- [ ] Basic egui application with menu bar and tabs
- [ ] Application state management
- [ ] Tab-based interface (Convert, Validate, Info, Settings)
- [ ] Dark/light theme support
- [ ] Proper window sizing and layout

---

## **T4.2: Advanced Batch Processing Interface (CRITICAL NEW FEATURE)**
**Duration**: 6 days | **Points**: 15 | **Priority**: 🚨 CRITICAL
**Dependencies**: T4.1 Complete, M3 CLI Complete

Sophisticated batch processing GUI to complement M3 CLI batch capabilities - completely missing from original plan!

### **Implementation Steps for Junior Developer**

**Step 1: Drag & Drop Campaign Interface**
Create `src/batch_processor.rs`:
```rust
use egui::{DragValue, ProgressBar, Color32, RichText};
use rfd::FileDialog;
use std::path::PathBuf;
use tokio::sync::mpsc;

/// Advanced batch processing interface with drag & drop support
pub struct BatchProcessorUI {
    /// Campaigns queued for processing
    pub campaign_queue: Vec<BatchCampaign>,
    /// Currently processing campaigns
    pub active_campaigns: Vec<ActiveConversion>,
    /// Batch processing settings
    pub batch_settings: BatchSettings,
    /// Processing statistics
    pub batch_stats: BatchStatistics,
    /// UI state
    pub ui_state: BatchUIState,
}

#[derive(Debug, Clone)]
pub struct BatchCampaign {
    pub path: PathBuf,
    pub name: String,
    pub size_mb: f64,
    pub status: CampaignStatus,
    pub estimated_time: Option<std::time::Duration>,
    pub preview_data: Option<CampaignPreview>,
}

#[derive(Debug, Clone)]
pub enum CampaignStatus {
    Queued,
    Processing { progress: f32 },
    Completed { duration: std::time::Duration },
    Failed { error: String },
    Skipped { reason: String },
}

#[derive(Debug, Clone)]
pub struct BatchSettings {
    pub max_parallel: usize,
    pub continue_on_error: bool,
    pub skip_archived: bool,
    pub output_directory: PathBuf,
    pub conversion_config: ConversionConfig,
}

impl BatchProcessorUI {
    /// Render the batch processing interface
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🔄 Batch Campaign Processing");
            ui.add_space(10.0);
            
            // Top controls
            self.show_batch_controls(ui);
            ui.separator();
            
            // Main batch interface with drag & drop
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    self.show_campaign_queue(ui);
                });
            
            ui.separator();
            
            // Progress visualization
            self.show_batch_progress(ui);
            
            ui.separator();
            
            // Statistics and controls
            self.show_batch_statistics(ui);
        });
    }
    
    /// Show batch processing controls
    fn show_batch_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Add campaigns button
            if ui.button("📁 Add Campaigns").clicked() {
                self.open_campaign_selection_dialog();
            }
            
            // Add directory button
            if ui.button("📂 Add Directory").clicked() {
                self.open_directory_selection_dialog();
            }
            
            ui.separator();
            
            // Parallel processing setting
            ui.label("Parallel jobs:");
            ui.add(DragValue::new(&mut self.batch_settings.max_parallel)
                .range(1..=16)
                .prefix("threads: "));
                
            ui.separator();
            
            // Batch processing controls
            let can_start = !self.campaign_queue.is_empty() && self.active_campaigns.is_empty();
            if ui.add_enabled(can_start, egui::Button::new("▶️ Start Batch"))
                .clicked() {
                self.start_batch_processing();
            }
            
            let can_pause = !self.active_campaigns.is_empty();
            if ui.add_enabled(can_pause, egui::Button::new("⏸️ Pause"))
                .clicked() {
                self.pause_batch_processing();
            }
            
            if ui.button("🗑️ Clear All").clicked() {
                self.clear_campaign_queue();
            }
        });
    }
    
    /// Show drag & drop campaign queue
    fn show_campaign_queue(&mut self, ui: &mut egui::Ui) {
        // Drag & drop zone
        let drop_zone = egui::Frame::none()
            .fill(Color32::from_gray(30))
            .stroke(egui::Stroke::new(2.0, Color32::from_gray(100)))
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::same(20.0));
            
        drop_zone.show(ui, |ui| {
            if self.campaign_queue.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.add(egui::Label::new(
                        RichText::new("📦 Drag & Drop Campaign Files Here\n\nOr use the buttons above to select files/directories")
                            .size(16.0)
                            .color(Color32::from_gray(150))
                    ));
                });
            } else {
                // Show campaign list with thumbnails
                for (index, campaign) in self.campaign_queue.iter().enumerate() {
                    self.show_campaign_card(ui, index, campaign);
                }
            }
        });
        
        // Handle drag & drop
        self.handle_drag_and_drop(ui);
    }
    
    /// Show individual campaign card with preview
    fn show_campaign_card(&mut self, ui: &mut egui::Ui, index: usize, campaign: &BatchCampaign) {
        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Campaign thumbnail (if available)
                    if let Some(ref preview) = campaign.preview_data {
                        if let Some(texture) = &preview.thumbnail_texture {
                            ui.add(egui::Image::from_texture(texture)
                                .fit_to_exact_size(egui::Vec2::new(64.0, 64.0))
                                .rounding(egui::Rounding::same(4.0)));
                        }
                    } else {
                        // Placeholder thumbnail
                        ui.add(egui::Label::new("📋").min_size(egui::Vec2::new(64.0, 64.0)));
                    }
                    
                    ui.add_space(10.0);
                    
                    // Campaign details
                    ui.vertical(|ui| {
                        ui.strong(&campaign.name);
                        ui.label(format!("Size: {:.1} MB", campaign.size_mb));
                        ui.label(format!("Path: {}", campaign.path.display()));
                        
                        // Status indicator
                        match &campaign.status {
                            CampaignStatus::Queued => {
                                ui.colored_label(Color32::from_gray(150), "⏳ Queued");
                            },
                            CampaignStatus::Processing { progress } => {
                                ui.horizontal(|ui| {
                                    ui.colored_label(Color32::YELLOW, "🔄 Processing");
                                    ui.add(ProgressBar::new(*progress).desired_width(100.0));
                                });
                            },
                            CampaignStatus::Completed { duration } => {
                                ui.colored_label(Color32::GREEN, 
                                    format!("✅ Completed ({:.1}s)", duration.as_secs_f64()));
                            },
                            CampaignStatus::Failed { error } => {
                                ui.colored_label(Color32::RED, format!("❌ Failed: {}", error));
                            },
                            CampaignStatus::Skipped { reason } => {
                                ui.colored_label(Color32::GRAY, format!("⏭️ Skipped: {}", reason));
                            },
                        }
                    });
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Remove campaign button
                        if ui.small_button("🗑️").clicked() {
                            self.remove_campaign(index);
                        }
                        
                        // Preview campaign button
                        if ui.small_button("👁️").clicked() {
                            self.show_campaign_preview(index);
                        }
                        
                        // Edit settings button
                        if ui.small_button("⚙️").clicked() {
                            self.show_campaign_settings(index);
                        }
                    });
                });
            });
    }
    
    /// Handle drag and drop file operations
    fn handle_drag_and_drop(&mut self, ui: &mut egui::Ui) {
        use egui::*;
        
        // Check for dropped files
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        self.add_campaign_from_path(path.clone());
                    }
                }
            }
        });
        
        // Visual feedback for drag over
        if ui.ctx().input(|i| i.raw.hovered_files.is_empty()) {
            ui.allocate_response(ui.available_size(), Sense::hover());
        } else {
            // Show drag over visual feedback
            ui.allocate_ui_with_layout(
                ui.available_size(),
                Layout::centered_and_justified(Direction::TopDown),
                |ui| {
                    ui.label(
                        RichText::new("📦 Drop files to add to batch queue")
                            .size(18.0)
                            .color(Color32::LIGHT_BLUE)
                    );
                }
            );
        }
    }
}
```

**Step 2: Campaign Preview & Validation**
Create `src/campaign_preview.rs`:
```rust
/// Campaign preview and validation system
pub struct CampaignPreviewUI {
    pub preview_data: Option<CampaignPreviewData>,
    pub validation_results: Vec<ValidationIssue>,
    pub asset_thumbnails: Vec<AssetThumbnail>,
    pub statistics: CampaignStatistics,
}

#[derive(Debug, Clone)]
pub struct CampaignPreviewData {
    pub campaign_name: String,
    pub total_entities: usize,
    pub scenes: Vec<ScenePreview>,
    pub characters: Vec<CharacterPreview>, 
    pub assets: Vec<AssetPreview>,
    pub handouts: Vec<HandoutPreview>,
    pub thumbnail_texture: Option<egui::TextureHandle>,
}

impl CampaignPreviewUI {
    /// Show comprehensive campaign preview
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("📋 Campaign Preview")
            .resizable(true)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                if let Some(ref preview) = self.preview_data {
                    self.show_preview_content(ui, preview);
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("No campaign loaded for preview");
                    });
                }
            });
    }
    
    fn show_preview_content(&mut self, ui: &mut egui::Ui, preview: &CampaignPreviewData) {
        // Campaign overview
        ui.heading(&preview.campaign_name);
        ui.add_space(10.0);
        
        // Statistics summary
        ui.horizontal(|ui| {
            ui.label(format!("📊 {} entities total", preview.total_entities));
            ui.separator();
            ui.label(format!("🗺️ {} scenes", preview.scenes.len()));
            ui.separator();
            ui.label(format!("👥 {} characters", preview.characters.len()));
            ui.separator();
            ui.label(format!("🖼️ {} assets", preview.assets.len()));
        });
        
        ui.separator();
        
        // Tabbed preview interface
        egui::containers::TabBar::new("preview_tabs").show(ui, |tab_bar| {
            tab_bar.tab("🗺️ Scenes", |ui| {
                self.show_scenes_preview(ui, &preview.scenes);
            });
            
            tab_bar.tab("👥 Characters", |ui| {
                self.show_characters_preview(ui, &preview.characters);
            });
            
            tab_bar.tab("🖼️ Assets", |ui| {
                self.show_assets_preview(ui, &preview.assets);
            });
            
            tab_bar.tab("⚠️ Validation", |ui| {
                self.show_validation_results(ui);
            });
        });
    }
    
    fn show_assets_preview(&mut self, ui: &mut egui::Ui, assets: &[AssetPreview]) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Asset grid with thumbnails
            let available_width = ui.available_width();
            let thumbnail_size = 120.0;
            let columns = (available_width / (thumbnail_size + 10.0)) as usize;
            
            ui.columns(columns.max(1), |columns| {
                for (index, asset) in assets.iter().enumerate() {
                    let column = &mut columns[index % columns.len()];
                    
                    column.group(|ui| {
                        // Asset thumbnail
                        if let Some(texture) = &asset.thumbnail_texture {
                            ui.add(egui::Image::from_texture(texture)
                                .fit_to_exact_size(egui::Vec2::new(thumbnail_size, thumbnail_size * 0.75))
                                .rounding(egui::Rounding::same(4.0)));
                        } else {
                            ui.allocate_exact_size(
                                egui::Vec2::new(thumbnail_size, thumbnail_size * 0.75),
                                Sense::hover()
                            );
                            ui.label("🖼️ No preview");
                        }
                        
                        // Asset info
                        ui.small(&asset.name);
                        ui.small(format!("{:.1} MB", asset.size_mb));
                        ui.small(&asset.format);
                        
                        // Asset classification
                        let (icon, color) = match asset.classification {
                            AssetClassification::Background => ("🌄", Color32::BLUE),
                            AssetClassification::Token => ("🎭", Color32::GREEN),
                            AssetClassification::Tile => ("🧩", Color32::YELLOW),
                            AssetClassification::Journal => ("📖", Color32::GRAY),
                            AssetClassification::Generic => ("📄", Color32::WHITE),
                        };
                        ui.colored_label(color, format!("{} {:?}", icon, asset.classification));
                    });
                }
            });
        });
    }
}
```

**Acceptance Criteria**:
- [ ] Drag & drop interface for multiple campaign files and directories
- [ ] Parallel processing visualization with individual campaign progress bars
- [ ] Campaign preview with asset thumbnails and statistics before conversion
- [ ] Batch processing settings with configurable parallel jobs (1-16 threads)
- [ ] Archive campaign filtering and comprehensive error handling
- [ ] Multi-window support for batch processing workflows
- [ ] Professional progress tracking with ETA and throughput calculations
- [ ] Integration with M3 CLI batch processing capabilities for backend processing

---

## **T4.3: Asset Optimization Settings GUI & Real-time Preview (ADVANCED FEATURE)**
**Duration**: 5 days | **Points**: 10 | **Priority**: 🔥 HIGH
**Dependencies**: T4.1-T4.2 Complete, M3 CLI Complete

Interactive asset processing configuration GUI providing visual access to all M3 CLI asset features.

### **Implementation Steps for Junior Developer**

**Step 1: Asset Optimization Settings Panel**
Create `src/asset_settings.rs`:
```rust
use egui::{DragValue, Checkbox, ComboBox, ColorPicker, Slider};

/// Interactive asset optimization settings with real-time preview
pub struct AssetOptimizationUI {
    pub settings: AssetOptimizationSettings,
    pub preview_state: AssetPreviewState,
    pub sample_assets: Vec<SampleAsset>,
}

#[derive(Debug, Clone)]
pub struct AssetOptimizationSettings {
    // Basic processing options
    pub use_original_urls: bool,
    pub images_as_drawings: bool,
    pub all_backgrounds_as_tiles: bool,
    
    // Format optimization
    pub convert_to_webp: bool,
    pub webp_quality: f32,
    pub generate_thumbnails: bool,
    pub thumbnail_size: u32,
    
    // Size and path limits
    pub max_asset_size_mb: u64,
    pub max_path_length: usize,
    
    // Advanced optimization
    pub optimize_png: bool,
    pub remove_metadata: bool,
    pub resize_large_images: bool,
    pub max_image_dimension: u32,
}

impl AssetOptimizationUI {
    /// Show asset optimization settings with real-time preview
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("🖼️ Asset Optimization Settings")
            .resizable(true)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    // Settings panel (left side)
                    ui.vertical(|ui| {
                        ui.set_width(300.0);
                        self.show_settings_panel(ui);
                    });
                    
                    ui.separator();
                    
                    // Preview panel (right side)
                    ui.vertical(|ui| {
                        ui.set_width(280.0);
                        self.show_preview_panel(ui);
                    });
                });
            });
    }
    
    fn show_settings_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Optimization Settings");
        ui.add_space(10.0);
        
        // Basic processing options
        ui.group(|ui| {
            ui.label("🔧 Processing Strategy");
            ui.add_space(5.0);
            
            ui.checkbox(&mut self.settings.use_original_urls, 
                "Use original image URLs (no local processing)");
            if self.settings.use_original_urls {
                ui.colored_label(egui::Color32::YELLOW, 
                    "⚠️ Requires internet connection during gameplay");
            }
            
            ui.checkbox(&mut self.settings.images_as_drawings, 
                "Convert all images to drawings");
            
            ui.checkbox(&mut self.settings.all_backgrounds_as_tiles, 
                "Treat all backgrounds as tiles");
        });
        
        ui.add_space(10.0);
        
        // Format optimization
        ui.group(|ui| {
            ui.label("📄 Format Optimization");
            ui.add_space(5.0);
            
            ui.checkbox(&mut self.settings.convert_to_webp, 
                "Convert to WebP format (smaller file sizes)");
                
            if self.settings.convert_to_webp {
                ui.horizontal(|ui| {
                    ui.label("Quality:");
                    ui.add(Slider::new(&mut self.settings.webp_quality, 0.1..=1.0)
                        .text("quality")
                        .show_value(true));
                });
                
                // Show estimated size savings
                ui.colored_label(egui::Color32::GREEN, 
                    format!("📉 Estimated savings: {}%", 
                        ((1.0 - self.settings.webp_quality) * 30.0) as i32 + 20));
            }
            
            ui.checkbox(&mut self.settings.generate_thumbnails, 
                "Generate thumbnails for large images");
                
            if self.settings.generate_thumbnails {
                ui.horizontal(|ui| {
                    ui.label("Thumbnail size:");
                    ui.add(DragValue::new(&mut self.settings.thumbnail_size)
                        .range(64..=512)
                        .suffix("px"));
                });
            }
        });
        
        ui.add_space(10.0);
        
        // Size and path limits
        ui.group(|ui| {
            ui.label("📏 Size & Path Limits");
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Max asset size:");
                ui.add(DragValue::new(&mut self.settings.max_asset_size_mb)
                    .range(1..=200)
                    .suffix("MB"));
            });
            
            ui.horizontal(|ui| {
                ui.label("Max path length:");
                ui.add(DragValue::new(&mut self.settings.max_path_length)
                    .range(100..=500)
                    .suffix("chars"));
            });
        });
        
        ui.add_space(10.0);
        
        // Advanced optimization
        ui.group(|ui| {
            ui.label("⚡ Advanced Optimization");
            ui.add_space(5.0);
            
            ui.checkbox(&mut self.settings.optimize_png, 
                "Optimize PNG compression (lossless)");
                
            ui.checkbox(&mut self.settings.remove_metadata, 
                "Remove EXIF metadata (privacy & size)");
                
            ui.checkbox(&mut self.settings.resize_large_images, 
                "Resize oversized images");
                
            if self.settings.resize_large_images {
                ui.horizontal(|ui| {
                    ui.label("Max dimension:");
                    ui.add(DragValue::new(&mut self.settings.max_image_dimension)
                        .range(1024..=8192)
                        .suffix("px"));
                });
            }
        });
    }
    
    fn show_preview_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Real-time Preview");
        ui.add_space(10.0);
        
        if self.sample_assets.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Load a campaign to see optimization preview");
            });
            return;
        }
        
        // Show before/after comparison for sample assets
        for asset in &self.sample_assets {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    // Original asset thumbnail
                    if let Some(texture) = &asset.original_texture {
                        ui.vertical(|ui| {
                            ui.label("Before");
                            ui.add(egui::Image::from_texture(texture)
                                .fit_to_exact_size(egui::Vec2::new(80.0, 60.0)));
                            ui.small(format!("{:.1} MB", asset.original_size_mb));
                            ui.small(&asset.original_format);
                        });
                    }
                    
                    ui.label("→");
                    
                    // Optimized preview
                    if let Some(texture) = &asset.optimized_texture {
                        ui.vertical(|ui| {
                            ui.label("After");
                            ui.add(egui::Image::from_texture(texture)
                                .fit_to_exact_size(egui::Vec2::new(80.0, 60.0)));
                            ui.small(format!("{:.1} MB", asset.optimized_size_mb));
                            ui.small(&asset.optimized_format);
                            
                            // Show savings
                            let savings = ((asset.original_size_mb - asset.optimized_size_mb) 
                                         / asset.original_size_mb) * 100.0;
                            if savings > 0.0 {
                                ui.colored_label(egui::Color32::GREEN, 
                                    format!("📉 -{:.1}%", savings));
                            }
                        });
                    }
                });
                
                ui.small(&asset.name);
            });
        }
        
        // Overall statistics
        ui.separator();
        ui.group(|ui| {
            ui.label("📊 Optimization Summary");
            let total_original = self.sample_assets.iter()
                .map(|a| a.original_size_mb).sum::<f64>();
            let total_optimized = self.sample_assets.iter()
                .map(|a| a.optimized_size_mb).sum::<f64>();
            let total_savings = ((total_original - total_optimized) / total_original) * 100.0;
            
            ui.label(format!("Original size: {:.1} MB", total_original));
            ui.label(format!("Optimized size: {:.1} MB", total_optimized));
            ui.colored_label(egui::Color32::GREEN,
                format!("Total savings: {:.1}%", total_savings));
        });
    }
}
```

**Step 2: Advanced Conversion Options GUI**
Create `src/conversion_options.rs`:
```rust
/// GUI for all 30+ advanced conversion options from M3 CLI
pub struct AdvancedConversionOptionsUI {
    pub options: AdvancedConversionOptions,
    pub presets: Vec<ConversionPreset>,
    pub active_preset: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AdvancedConversionOptions {
    // Campaign customization
    pub campaign_title: String,
    pub description: String,
    pub password: String,
    
    // Wall processing (from M2 Advanced Wall Processing)
    pub restrict_movement: bool,
    pub add_walls_around_map: bool,
    pub cleanup_scenes: bool,
    pub minimum_wall_length: f64,
    pub maximum_wall_angle: f64,
    
    // Door detection
    pub auto_doors: bool,
    pub door_color: egui::Color32,
    pub secret_door_color: egui::Color32,
    
    // Map features
    pub enable_fog: Option<bool>, // None = don't change, Some(bool) = set value
    pub force_hp_token_bar1: bool,
    pub force_hp_token_bar2: bool,
    pub grid_size: Option<f64>,
    pub grid_units: Option<f64>,
    
    // Module export
    pub export_as_module: bool,
    pub disable_module_actors: bool,
    pub disable_module_items: bool,
    pub disable_module_scenes: bool,
    pub disable_module_journals: bool,
    
    // Advanced options
    pub debug_page: bool,
    pub threads: Option<usize>,
    pub verbose_progress: bool,
}

impl AdvancedConversionOptionsUI {
    /// Show comprehensive conversion options GUI
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Advanced Conversion Options")
            .resizable(true)
            .default_size([700.0, 600.0])
            .show(ctx, |ui| {
                // Preset management
                self.show_preset_management(ui);
                ui.separator();
                
                // Tabbed options interface
                egui::containers::TabBar::new("conversion_tabs").show(ui, |tab_bar| {
                    tab_bar.tab("🏰 Campaign", |ui| {
                        self.show_campaign_options(ui);
                    });
                    
                    tab_bar.tab("🧱 Walls & Doors", |ui| {
                        self.show_wall_options(ui);
                    });
                    
                    tab_bar.tab("🗺️ Map Features", |ui| {
                        self.show_map_options(ui);
                    });
                    
                    tab_bar.tab("📦 Module Export", |ui| {
                        self.show_module_options(ui);
                    });
                    
                    tab_bar.tab("🔧 Advanced", |ui| {
                        self.show_advanced_options(ui);
                    });
                });
            });
    }
    
    fn show_wall_options(&mut self, ui: &mut egui::Ui) {
        ui.heading("🧱 Wall Processing Options");
        ui.add_space(10.0);
        
        // Wall behavior
        ui.group(|ui| {
            ui.label("Wall Behavior");
            ui.checkbox(&mut self.options.restrict_movement, 
                "Walls restrict token movement");
            ui.checkbox(&mut self.options.add_walls_around_map, 
                "Add walls around map boundaries");
            ui.checkbox(&mut self.options.cleanup_scenes, 
                "Clean up cave scenes (optimize thousands of walls)");
                
            if self.options.cleanup_scenes {
                ui.colored_label(egui::Color32::YELLOW,
                    "⚡ Enables advanced wall optimization from M2 Core Engine");
            }
        });
        
        ui.add_space(10.0);
        
        // Wall optimization settings
        ui.group(|ui| {
            ui.label("Optimization Settings");
            
            ui.horizontal(|ui| {
                ui.label("Minimum wall length:");
                ui.add(DragValue::new(&mut self.options.minimum_wall_length)
                    .range(1.0..=100.0)
                    .suffix("px"));
                ui.small("(removes tiny wall segments)");
            });
            
            ui.horizontal(|ui| {
                ui.label("Maximum merge angle:");
                ui.add(DragValue::new(&mut self.options.maximum_wall_angle)
                    .range(1.0..=90.0)
                    .suffix("°"));
                ui.small("(merges nearly collinear walls)");
            });
        });
        
        ui.add_space(10.0);
        
        // Door detection
        ui.group(|ui| {
            ui.label("🚪 Door Detection");
            ui.checkbox(&mut self.options.auto_doors, 
                "Automatically detect doors from wall colors");
                
            if self.options.auto_doors {
                ui.horizontal(|ui| {
                    ui.label("Door color:");
                    ui.color_edit_button_srgba(&mut self.options.door_color);
                    ui.small("(typically red #ff0000)");
                });
                
                ui.horizontal(|ui| {
                    ui.label("Secret door color:");
                    ui.color_edit_button_srgba(&mut self.options.secret_door_color);
                    ui.small("(typically green #00ff00)");
                });
            }
        });
    }
    
    fn show_preset_management(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Preset:");
            
            ComboBox::from_id_source("conversion_preset")
                .selected_text(self.active_preset.as_deref().unwrap_or("Custom"))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.active_preset, None, "Custom");
                    for preset in &self.presets {
                        ui.selectable_value(&mut self.active_preset, 
                            Some(preset.name.clone()), &preset.name);
                    }
                });
                
            if ui.button("💾 Save Preset").clicked() {
                self.save_current_as_preset();
            }
            
            if ui.button("🗑️ Delete Preset").clicked() {
                self.delete_current_preset();
            }
        });
    }
}
```

**Acceptance Criteria**:
- [ ] Interactive asset optimization settings with real-time preview
- [ ] WebP conversion settings with quality slider and size estimation
- [ ] GUI access to all 30+ advanced CLI options from M3 organized in logical tabs
- [ ] Wall processing options with color picker for door detection
- [ ] Campaign customization GUI with validation
- [ ] Module export options with selective component disabling
- [ ] Preset management system for saving/loading conversion configurations
- [ ] Real-time preview of optimization effects on sample assets

---

### **🎯 M4 COMPLETION SUMMARY**
**Advanced GUI Interface with Comprehensive Feature Access:**
- ✅ **T4.1**: egui Application Foundation with professional styling
- ✅ **T4.2**: Advanced Batch Processing Interface with drag & drop campaigns
- ✅ **T4.3**: Asset Optimization Settings GUI with real-time preview
- ✅ **T4.4**: Advanced Conversion Options GUI providing access to all 30+ M3 CLI features

### **Professional GUI Standards Achieved:**
- [ ] **Drag & Drop Batch Processing**: Parallel campaign processing with visual progress
- [ ] **Campaign Preview & Validation**: Asset thumbnails and comprehensive statistics  
- [ ] **Interactive Asset Optimization**: Real-time preview with before/after comparisons
- [ ] **Complete CLI Feature Access**: GUI for all 30+ advanced command-line options
- [ ] **Professional Progress Visualization**: Multi-threaded progress tracking
- [ ] **Configuration Management**: Preset system with save/load capabilities
- [ ] **Cross-platform Native GUI**: Modern egui interface with professional styling

### **Implementation Steps**

{{ ... }}
**Step 1: File Selection UI**
```rust
impl TTRPGConverterApp {
    fn show_convert_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Campaign Conversion");
        ui.add_space(10.0);
        
        // Input file selection
        ui.horizontal(|ui| {
            ui.label("Input Campaign:");
            if ui.button("Browse...").clicked() {
                self.open_file_dialog();
            }
        });
        
        if let Some(path) = &self.conversion_state.input_path {
            ui.label(format!("Selected: {}", path));
        }
        
        ui.add_space(10.0);
        
        // Output directory selection
        ui.horizontal(|ui| {
            ui.label("Output Directory:");
            if ui.button("Browse...").clicked() {
                self.open_folder_dialog();
            }
        });
        
        if let Some(path) = &self.conversion_state.output_path {
            ui.label(format!("Output: {}", path));
        }
        
        ui.add_space(20.0);
        
        // Format selection
        self.show_format_selection(ui);
        
        ui.add_space(20.0);
        
        // Conversion options
        self.show_conversion_options(ui);
        
        ui.add_space(30.0);
        
        // Convert button and progress
        self.show_convert_controls(ui);
    }

    fn show_format_selection(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Source Format:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.conversion_state.source_format))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.conversion_state.source_format, SourceFormat::Roll20, "Roll20");
                    ui.selectable_value(&mut self.conversion_state.source_format, SourceFormat::FoundryVtt, "Foundry VTT");
                });
        });
        
        ui.horizontal(|ui| {
            ui.label("Target Format:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.conversion_state.target_format))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.conversion_state.target_format, TargetFormat::FoundryV12, "Foundry v12");
                    ui.selectable_value(&mut self.conversion_state.target_format, TargetFormat::FoundryV11, "Foundry v11");
                    ui.selectable_value(&mut self.conversion_state.target_format, TargetFormat::FoundryV10, "Foundry v10");
                });
        });
    }
}
```

**Step 2: Drag-and-Drop Support**
```rust
fn handle_drag_and_drop(&mut self, ctx: &egui::Context) {
    use egui::*;
    
    // Check for dropped files
    ctx.input(|i| {
        if !i.raw.dropped_files.is_empty() {
            for file in &i.raw.dropped_files {
                if let Some(path) = &file.path {
                    if path.extension().map_or(false, |ext| ext == "zip") {
                        self.conversion_state.input_path = Some(path.display().to_string());
                    }
                }
            }
        }
    });
    
    // Show drag-and-drop hint
    if self.conversion_state.input_path.is_none() {
        let painter = ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("drag_hint")));
        let rect = ctx.available_rect();
        
        painter.rect_stroke(
            rect.shrink(20.0),
            Rounding::default(),
            Stroke::new(2.0, Color32::GRAY)
        );
        
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "Drag and drop campaign file here",
            FontId::default(),
            Color32::GRAY,
        );
    }
}
```

**Step 3: Real-time Progress Tracking**
```rust
fn show_convert_controls(&mut self, ui: &mut egui::Ui) {
    let can_convert = self.conversion_state.input_path.is_some() 
        && self.conversion_state.output_path.is_some()
        && !self.conversion_state.is_converting;
    
    ui.add_enabled_ui(can_convert, |ui| {
        if ui.button("Start Conversion").clicked() {
            self.start_conversion();
        }
    });
    
    if self.conversion_state.is_converting {
        ui.add_space(10.0);
        
        let progress = self.conversion_state.progress;
        ui.add(egui::ProgressBar::new(progress)
            .text(format!("Converting... {:.0}%", progress * 100.0)));
        
        // Cancel button
        if ui.button("Cancel").clicked() {
            self.cancel_conversion();
        }
    }
    
    // Show results
    if let Some(result) = &self.conversion_state.conversion_result {
        ui.add_space(10.0);
        match result {
            Ok(()) => {
                ui.colored_label(egui::Color32::GREEN, "✓ Conversion completed successfully!");
            }
            Err(e) => {
                ui.colored_label(egui::Color32::RED, format!("✗ Conversion failed: {}", e));
            }
        }
    }
}
```

**Acceptance Criteria**:
- [ ] File and folder selection dialogs
- [ ] Drag-and-drop support for campaign files
- [ ] Format selection with combo boxes
- [ ] Real-time progress bars with cancel option
- [ ] Conversion results display
- [ ] Input validation and error messages

---

## **T4.3: Settings and Configuration Panel**
**Duration**: 2 days | **Points**: 5 | **Priority**: 🟡 MEDIUM
**Dependencies**: T4.2 Complete

### **Implementation Steps**

**Step 1: Settings UI**
```rust
fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.add_space(10.0);
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Conversion settings
        ui.group(|ui| {
            ui.label(RichText::new("Conversion Settings").strong());
            ui.add_space(5.0);
            
            ui.checkbox(&mut self.config.download_assets, "Download external assets");
            ui.checkbox(&mut self.config.optimize_images, "Optimize images");
            ui.checkbox(&mut self.config.preserve_folders, "Preserve folder structure");
            
            ui.horizontal(|ui| {
                ui.label("Max asset size:");
                ui.add(egui::Slider::new(&mut self.config.max_asset_mb, 1..=500)
                    .suffix(" MB"));
            });
            
            ui.horizontal(|ui| {
                ui.label("Thread count:");
                if self.config.thread_count == 0 {
                    ui.label("Auto");
                } else {
                    ui.label(self.config.thread_count.to_string());
                }
                ui.add(egui::Slider::new(&mut self.config.thread_count, 0..=16)
                    .text("threads"));
            });
        });
        
        ui.add_space(10.0);
        
        // Output settings
        ui.group(|ui| {
            ui.label(RichText::new("Output Settings").strong());
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                ui.label("Default Foundry version:");
                egui::ComboBox::from_label("")
                    .selected_text(&self.config.default_foundry_version)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.default_foundry_version, "v12".to_string(), "v12");
                        ui.selectable_value(&mut self.config.default_foundry_version, "v11".to_string(), "v11");
                        ui.selectable_value(&mut self.config.default_foundry_version, "v10".to_string(), "v10");
                    });
            });
            
            ui.checkbox(&mut self.config.compress_output, "Compress output files");
            ui.checkbox(&mut self.config.backup_existing, "Backup existing files");
        });
        
        ui.add_space(10.0);
        
        // UI settings
        ui.group(|ui| {
            ui.label(RichText::new("Interface Settings").strong());
            ui.add_space(5.0);
            
            ui.checkbox(&mut self.ui_state.dark_mode, "Dark mode");
            ui.checkbox(&mut self.config.show_advanced_options, "Show advanced options");
            ui.checkbox(&mut self.config.confirm_operations, "Confirm destructive operations");
        });
    });
    
    ui.add_space(20.0);
    
    ui.horizontal(|ui| {
        if ui.button("Save Settings").clicked() {
            self.save_settings();
        }
        if ui.button("Reset to Defaults").clicked() {
            self.reset_settings();
        }
        if ui.button("Load from File").clicked() {
            self.load_settings_file();
        }
    });
}
```

**Step 2: Configuration Persistence**
```rust
impl TTRPGConverterApp {
    fn save_settings(&self) {
        if let Ok(config_dir) = directories::ProjectDirs::from("com", "ttrpg", "converter") {
            let config_path = config_dir.config_dir().join("gui_config.json");
            
            if let Ok(json) = serde_json::to_string_pretty(&self.config) {
                if let Err(e) = std::fs::write(&config_path, json) {
                    eprintln!("Failed to save settings: {}", e);
                } else {
                    println!("Settings saved to: {}", config_path.display());
                }
            }
        }
    }
    
    fn load_settings(&mut self) {
        if let Ok(config_dir) = directories::ProjectDirs::from("com", "ttrpg", "converter") {
            let config_path = config_dir.config_dir().join("gui_config.json");
            
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    self.config = config;
                }
            }
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Comprehensive settings panel with all options
- [ ] Settings persistence to JSON file
- [ ] Reset to defaults functionality
- [ ] Import/export settings
- [ ] Immediate UI updates when settings change
- [ ] Validation for invalid settings values

---

## **T4.4: Enhanced UX Features**
**Duration**: 3 days | **Points**: 5 | **Priority**: 🟢 LOW
**Dependencies**: T4.3 Complete

### **Implementation Steps**

**Step 1: Campaign Preview**
```rust
fn show_info_tab(&mut self, ui: &mut egui::Ui) {
    ui.heading("Campaign Information");
    ui.add_space(10.0);
    
    if let Some(campaign_info) = &self.campaign_preview {
        self.show_campaign_preview(ui, campaign_info);
    } else if let Some(path) = &self.conversion_state.input_path {
        ui.horizontal(|ui| {
            if ui.button("Analyze Campaign").clicked() {
                self.analyze_campaign(path);
            }
            ui.spinner(); // Show while analyzing
        });
    } else {
        ui.label("No campaign file selected");
    }
}

fn show_campaign_preview(&self, ui: &mut egui::Ui, info: &CampaignInfo) {
    // Campaign overview
    ui.group(|ui| {
        ui.label(RichText::new(&info.name).heading());
        ui.label(format!("Format: {}", info.format));
        ui.label(format!("Size: {:.1} MB", info.size_mb));
    });
    
    ui.add_space(10.0);
    
    // Entity statistics table
    egui::Grid::new("stats_grid").show(ui, |ui| {
        ui.label(RichText::new("Entity Type").strong());
        ui.label(RichText::new("Count").strong());
        ui.label(RichText::new("Size").strong());
        ui.end_row();
        
        for stat in &info.entity_stats {
            ui.label(&stat.entity_type);
            ui.label(stat.count.to_string());
            ui.label(format!("{:.1} MB", stat.size_mb));
            ui.end_row();
        }
    });
    
    ui.add_space(10.0);
    
    // Asset information
    if !info.assets.is_empty() {
        ui.collapsing("Assets", |ui| {
            for asset in &info.assets {
                ui.horizontal(|ui| {
                    ui.label(&asset.name);
                    ui.label(format!("({:.1} KB)", asset.size_kb));
                    if asset.is_external {
                        ui.colored_label(egui::Color32::YELLOW, "External");
                    }
                });
            }
        });
    }
}
```

**Step 2: Error Handling and User Feedback**
```rust
fn show_error_dialog(&mut self, ctx: &egui::Context) {
    if let Some(error) = &self.current_error {
        egui::Window::new("Error")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(RichText::new("An error occurred:").color(egui::Color32::RED));
                ui.add_space(10.0);
                
                ui.label(&error.message);
                
                if !error.details.is_empty() {
                    ui.add_space(10.0);
                    ui.collapsing("Details", |ui| {
                        ui.label(&error.details);
                    });
                }
                
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        self.current_error = None;
                    }
                    if ui.button("Copy Error").clicked() {
                        ui.ctx().output_mut(|o| {
                            o.copied_text = format!("{}\n\n{}", error.message, error.details);
                        });
                    }
                });
            });
    }
}
```

**Step 3: Keyboard Shortcuts and Accessibility**
```rust
fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
    ctx.input_mut(|i| {
        // Ctrl+O - Open file
        if i.consume_key(egui::Modifiers::CTRL, egui::Key::O) {
            self.open_file_dialog();
        }
        
        // Ctrl+S - Save settings
        if i.consume_key(egui::Modifiers::CTRL, egui::Key::S) {
            self.save_settings();
        }
        
        // F1 - Show help
        if i.consume_key(egui::Modifiers::NONE, egui::Key::F1) {
            self.ui_state.show_about = true;
        }
        
        // Ctrl+Q - Quit
        if i.consume_key(egui::Modifiers::CTRL, egui::Key::Q) {
            std::process::exit(0);
        }
    });
}
```

**Acceptance Criteria**:
- [ ] Campaign preview with detailed statistics
- [ ] Error dialogs with copy-to-clipboard
- [ ] Keyboard shortcuts for common operations
- [ ] Tooltips and help text throughout UI
- [ ] Responsive design for different window sizes
- [ ] Status bar with current operation info
