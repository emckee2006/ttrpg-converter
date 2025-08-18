//! Common conversion utilities for Pathbuilder PF2e mappings

/// Parse dice notation from Pathbuilder format
/// 
/// # Examples
/// ```
/// use ttrpg_pathbuilder_pf2e::mappings::helpers::parse_dice_notation;
/// 
/// let dice = parse_dice_notation("1d8+2");
/// assert!(dice.is_some());
/// ```
pub fn parse_dice_notation(input: &str) -> Option<String> {
    // Simple validation - just return cleaned input for now
    if input.trim().is_empty() {
        None
    } else {
        Some(input.trim().to_string())
    }
}

/// Normalize currency values between different systems
pub fn normalize_currency(value: f64, _from: &str, _to: &str) -> f64 {
    // For now, assume direct conversion
    value
}

/// Parse duration strings from Pathbuilder format
pub fn parse_duration(input: &str) -> Option<String> {
    if input.trim().is_empty() {
        None
    } else {
        Some(input.trim().to_string())
    }
}

/// Sanitize HTML content for safe display
pub fn sanitize_html(input: &str) -> String {
    // Basic HTML tag removal for safety
    input.replace('<', "&lt;").replace('>', "&gt;")
}
