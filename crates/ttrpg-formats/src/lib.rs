//! File format parsers and converters
//!
//! This crate provides parsers and converters for various TTRPG file formats,
//! including Roll20 campaigns, D&D Beyond exports, and other popular platforms.

pub mod roll20;
pub mod roll20_asset_integration;

pub mod prelude {
    //! Common imports for this crate
    pub use crate::roll20::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_roll20_module_available() {
        // Test that the roll20 module is accessible
        let parser = crate::roll20::Roll20Parser::new();
        assert!(parser.validation_service.is_none());
    }
}
