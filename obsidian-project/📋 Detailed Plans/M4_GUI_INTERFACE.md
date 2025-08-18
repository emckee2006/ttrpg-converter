# M4: GUI Interface - Junior Developer Implementation Guide

## 🎯 **MILESTONE 4 OVERVIEW**
**Duration**: 2 weeks | **Total Points**: 15 | **Priority**: 🔥 HIGH
**Dependencies**: M3 Platform Format Plugins Complete

Desktop GUI interface with plugin discovery, loading management, and visual campaign conversion workflow.

### 🖥️ **GUI FEATURES**
- **Plugin Discovery UI**: Visual plugin management and loading
- **Conversion Workflow**: Drag-and-drop file conversion interface
- **Progress Tracking**: Real-time conversion progress and status
- **Configuration Management**: User preferences and plugin settings

---

## **T4.1: Core GUI Framework**
**Duration**: 4 days | **Points**: 6 | **Priority**: 🚨 CRITICAL

### **Implementation Steps**

**Step 1: GUI Dependencies**
Update `ttrpg-gui/Cargo.toml`:
```toml
[dependencies]
ttrpg-core = { path = "../core/ttrpg-core" }
egui = "0.24"
eframe = { version = "0.24", default-features = false, features = ["default_fonts", "glow", "persistence"] }
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
rfd = "0.12"  # File dialogs
directories = "5.0"  # Standard directories
```

**Step 2: Main GUI Application**
Create `src/app.rs`:
```rust
use std::path::PathBuf;
use std::sync::Arc;
use eframe::egui;
use ttrpg_core::orchestration::PluginOrchestrator;
use ttrpg_core::plugins::PlatformType;

pub struct TTRPGConverterApp {
    orchestrator: Arc<PluginOrchestrator>,
    current_input_file: Option<PathBuf>,
    current_output_dir: Option<PathBuf>,
    selected_target_platform: PlatformType,
    conversion_progress: ConversionProgress,
    available_plugins: Vec<String>,
    loaded_plugins: Vec<String>,
    show_settings: bool,
    show_about: bool,
}

impl TTRPGConverterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize orchestrator
        let plugins_dir = directories::ProjectDirs::from("com", "ttrpgconverter", "TTRPGConverter")
            .map(|proj_dirs| proj_dirs.data_dir().join("plugins"))
            .unwrap_or_else(|| std::env::current_dir().unwrap().join("plugins"));
            
        let orchestrator = Arc::new(
            tokio::runtime::Handle::current()
                .block_on(PluginOrchestrator::new(plugins_dir))
                .expect("Failed to initialize plugin orchestrator")
        );
        
        Self {
            orchestrator,
            current_input_file: None,
            current_output_dir: None,
            selected_target_platform: PlatformType::FoundryVTT,
            conversion_progress: ConversionProgress::default(),
            available_plugins: Vec::new(),
            loaded_plugins: Vec::new(),
            show_settings: false,
            show_about: false,
        }
    }
    
    fn refresh_available_plugins(&mut self) {
        if let Ok(plugins) = tokio::runtime::Handle::current()
            .block_on(self.orchestrator.discover_available_plugins()) {
            self.available_plugins = plugins;
        }
    }
}

impl eframe::App for TTRPGConverterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_menu_bar(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_main_interface(ui);
        });
        
        if self.show_settings {
            self.show_settings_window(ctx);
        }
        
        if self.show_about {
            self.show_about_window(ctx);
        }
    }
}

impl TTRPGConverterApp {
    fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Campaign...").clicked() {
                        self.open_file_dialog();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Plugins", |ui| {
                    if ui.button("Refresh Available Plugins").clicked() {
                        self.refresh_available_plugins();
                    }
                    
                    ui.separator();
                    
                    ui.label("Available Plugins:");
                    for plugin in &self.available_plugins {
                        ui.label(format!("• {}", plugin));
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });
    }
    
    fn show_main_interface(&mut self, ui: &mut egui::Ui) {
        ui.heading("TTRPG Campaign Converter");
        ui.separator();
        
        // Input file selection
        ui.horizontal(|ui| {
            ui.label("Input file:");
            if ui.button("Browse...").clicked() {
                self.open_file_dialog();
            }
            if let Some(path) = &self.current_input_file {
                ui.label(format!("Selected: {}", path.display()));
            }
        });
        
        ui.add_space(10.0);
        
        // Target platform selection
        ui.horizontal(|ui| {
            ui.label("Convert to:");
            egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", self.selected_target_platform))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_target_platform, PlatformType::FoundryVTT, "Foundry VTT");
                    ui.selectable_value(&mut self.selected_target_platform, PlatformType::Roll20, "Roll20");
                    ui.selectable_value(&mut self.selected_target_platform, PlatformType::Pathbuilder, "Pathbuilder PF2e");
                });
        });
        
        ui.add_space(10.0);
        
        // Output directory selection
        ui.horizontal(|ui| {
            ui.label("Output directory:");
            if ui.button("Browse...").clicked() {
                self.open_output_dialog();
            }
            if let Some(path) = &self.current_output_dir {
                ui.label(format!("Selected: {}", path.display()));
            }
        });
        
        ui.add_space(20.0);
        
        // Conversion button
        let can_convert = self.current_input_file.is_some() && 
                         self.current_output_dir.is_some() &&
                         !self.conversion_progress.is_running;
                         
        if ui.add_enabled(can_convert, egui::Button::new("Convert Campaign")).clicked() {
            self.start_conversion();
        }
        
        ui.add_space(10.0);
        
        // Progress display
        if self.conversion_progress.is_running {
            ui.label("Converting...");
            ui.add(egui::ProgressBar::new(self.conversion_progress.progress).text("Processing"));
            ui.label(&self.conversion_progress.current_step);
        }
        
        if let Some(ref result) = self.conversion_progress.result {
            match result {
                Ok(output_path) => {
                    ui.colored_label(egui::Color32::GREEN, "✅ Conversion completed successfully!");
                    ui.label(format!("Output: {}", output_path.display()));
                }
                Err(error) => {
                    ui.colored_label(egui::Color32::RED, "❌ Conversion failed:");
                    ui.label(error);
                }
            }
        }
        
        ui.add_space(20.0);
        
        // Plugin status
        ui.separator();
        ui.heading("Plugin Status");
        
        ui.label(format!("Available plugins: {}", self.available_plugins.len()));
        ui.label(format!("Loaded plugins: {}", self.loaded_plugins.len()));
    }
    
    fn show_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Plugin Directory:");
                ui.text_edit_singleline(&mut "plugins/".to_string());
                
                ui.separator();
                
                ui.checkbox(&mut true, "Auto-load available plugins");
                ui.checkbox(&mut false, "Enable debug logging");
                ui.checkbox(&mut true, "Show conversion progress");
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        // Save settings
                        self.show_settings = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_settings = false;
                    }
                });
            });
    }
    
    fn show_about_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About TTRPG Converter")
            .open(&mut self.show_about)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("TTRPG Campaign Converter");
                ui.label("Version 1.0.0");
                ui.separator();
                ui.label("A powerful tool for converting campaigns between different tabletop RPG platforms.");
                ui.separator();
                ui.label("Supported platforms:");
                ui.label("• Roll20");
                ui.label("• Foundry VTT");
                ui.label("• Pathbuilder PF2e");
            });
    }
    
    fn open_file_dialog(&mut self) {
        let task = rfd::AsyncFileDialog::new()
            .add_filter("JSON files", &["json"])
            .pick_file();
            
        let orchestrator = self.orchestrator.clone();
        tokio::spawn(async move {
            if let Some(file) = task.await {
                // Handle file selection
                println!("Selected file: {:?}", file.path());
            }
        });
    }
    
    fn open_output_dialog(&mut self) {
        let task = rfd::AsyncFileDialog::new()
            .pick_folder();
            
        tokio::spawn(async move {
            if let Some(folder) = task.await {
                println!("Selected output folder: {:?}", folder.path());
            }
        });
    }
    
    fn start_conversion(&mut self) {
        self.conversion_progress.is_running = true;
        self.conversion_progress.progress = 0.0;
        self.conversion_progress.current_step = "Starting conversion...".to_string();
        
        // In a real implementation, this would spawn an async task
        // and update progress through channels or shared state
    }
}

#[derive(Default)]
struct ConversionProgress {
    is_running: bool,
    progress: f32,
    current_step: String,
    result: Option<Result<PathBuf, String>>,
}
```

---

## **T4.2: Plugin Management UI**
**Duration**: 3 days | **Points**: 4 | **Priority**: 🔥 HIGH

Visual plugin discovery and management interface.

---

## **T4.3: Conversion Workflow UI**  
**Duration**: 4 days | **Points**: 3 | **Priority**: 🔥 HIGH

Drag-and-drop conversion interface with progress tracking.

---

## **T4.4: Configuration Management**
**Duration**: 3 days | **Points**: 2 | **Priority**: 🔥 HIGH  

Settings persistence and user preference management.

### **Success Criteria**
- [ ] ✅ Native desktop GUI launches successfully
- [ ] ✅ Plugin discovery and loading interface functional
- [ ] ✅ File selection and conversion workflow complete
- [ ] ✅ Real-time progress tracking during conversion
- [ ] ✅ Settings persistence across application restarts
- [ ] ✅ Error handling with user-friendly messages
