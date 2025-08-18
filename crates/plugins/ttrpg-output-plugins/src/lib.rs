//! Output plugins for various target formats
//!
//! This crate provides output plugin implementations for exporting campaign data
//! to various target formats and platforms.

// Plugin module declarations
pub mod foundry;
pub mod json;
pub mod pdf;
pub mod logging;

// Re-export commonly used plugins for convenience
pub use foundry::{FoundryWorldHandler, FoundryModuleHandler};
pub use json::{UniversalJsonHandler, PathbuilderJsonHandler, DNDBeyondJsonHandler, HeroLabJsonHandler};
pub use pdf::{PDFCharacterSheetsHandler, PDFCampaignBookHandler};
pub use logging::console::ConsoleLoggingPlugin;
