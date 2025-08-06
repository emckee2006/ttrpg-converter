//! Command-line interface

pub mod prelude {
    //! Common imports for this crate
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
