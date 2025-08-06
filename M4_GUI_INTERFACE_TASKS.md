# M4: GUI Interface Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 4 OVERVIEW**
**Duration**: 3 weeks | **Total Points**: 25 | **Priority**: 🟡 MEDIUM

Native GUI with egui replacing Python GUI, built as enhancement layer over proven CLI foundation.

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

## **T4.2: Convert Tab Implementation**
**Duration**: 4 days | **Points**: 8 | **Priority**: 🟡 MEDIUM
**Dependencies**: T4.1 Complete

### **Implementation Steps**

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
