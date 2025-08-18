//! Logging plugins module
//!
//! This module contains logging plugin implementations with orchestration framework integration.

pub mod console;

// Re-export the main logging plugin
pub use console::ConsoleLoggingPlugin;
