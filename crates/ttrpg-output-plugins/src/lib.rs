//! Output plugins for various target formats
//!
//! This crate provides output plugin implementations for exporting campaign data
//! to various target formats and platforms using the orchestration framework.
//!
//! ## Implemented Plugins
//! - **Logging**: Enhanced console logging with orchestration framework integration
//! - **JSON**: Universal JSON export plugin with orchestration framework integration
//!
//! ## Plugin Architecture
//! All plugins in this crate demonstrate the complete orchestration framework integration pattern:
//! - **Inventory Integration**: Auto-registration and compile-time discovery
//! - **Shaku Integration**: Dependency injection and lifecycle management
//! - **Daggy Integration**: Pipeline coordination capabilities
//!
//! ## Future Plugins
//! - **Foundry**: FoundryVTT world and module export plugins
//! - **PDF**: PDF character sheet and campaign book plugins

// Plugin module declarations
pub mod logging;
// TEMPORARILY COMMENTED OUT: JSON plugin while fixing console logging plugin errors
// pub mod json;

// Re-export commonly used plugins for convenience
pub use logging::console::ConsoleLoggingPlugin;
// TEMPORARILY COMMENTED OUT: JSON plugin while fixing console logging plugin errors
// pub use json::core::JsonExportPlugin;
