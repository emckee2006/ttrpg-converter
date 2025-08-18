//! Command-line interface for TTRPG Converter
//!
//! This crate provides command-line tools for testing and using the TTRPG conversion system.

pub mod commands;

pub mod prelude {
    //! Common imports for this crate
    // CLI commands migrated to plugin architecture - no direct command exports needed
}

/// Placeholder for initial development
pub fn placeholder() -> String {
    "This is the ttrpg-cli crate".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder() {
        assert!(!placeholder().is_empty());
    }
}
