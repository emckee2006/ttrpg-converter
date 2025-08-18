//! Core validation utilities for TTRPGConverter
//!
//! This module provides validation functions for common data types
//! used across the plugin ecosystem.

use regex::Regex;
use std::path::Path;
use url::Url;
use validator::ValidationError;

/// Validate a game system identifier
/// 
/// # Errors
/// Returns a validation error if the system ID is invalid
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_system_id;
/// 
/// assert!(validate_system_id("dnd5e").is_ok());
/// assert!(validate_system_id("pathfinder2e").is_ok());
/// assert!(validate_system_id("invalid-system!").is_err());
/// ```
pub fn validate_system_id(system_id: &str) -> Result<(), ValidationError> {
    lazy_static::lazy_static! {
        static ref SYSTEM_ID_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9_]{1,30}$").unwrap();
    }

    if !SYSTEM_ID_REGEX.is_match(system_id) {
        return Err(ValidationError::new("invalid_system_id"));
    }

    Ok(())
}

/// Validate an entity ID (UUID-like or alphanumeric)
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_entity_id;
/// 
/// assert!(validate_entity_id("abc123def").is_ok());
/// assert!(validate_entity_id("550e8400-e29b-41d4-a716-446655440000").is_ok());
/// assert!(validate_entity_id("").is_err());
/// ```
pub fn validate_entity_id(id: &str) -> Result<(), ValidationError> {
    if id.is_empty() || id.len() > 64 {
        return Err(ValidationError::new("invalid_entity_id"));
    }

    lazy_static::lazy_static! {
        static ref ENTITY_ID_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\-_]{1,64}$").unwrap();
    }

    if !ENTITY_ID_REGEX.is_match(id) {
        return Err(ValidationError::new("invalid_entity_id"));
    }

    Ok(())
}

/// Validate asset file extension against allowed types
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_asset_extension;
/// 
/// assert!(validate_asset_extension("image.png").is_ok());
/// assert!(validate_asset_extension("audio.mp3").is_ok());
/// assert!(validate_asset_extension("malware.exe").is_err());
/// ```
pub fn validate_asset_extension(filename: &str) -> Result<(), ValidationError> {
    let path = Path::new(filename);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        // Images
        Some("png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" | "ico") => Ok(()),
        // Audio
        Some("mp3" | "wav" | "ogg" | "m4a" | "flac") => Ok(()),
        // Video
        Some("mp4" | "webm" | "mov" | "avi") => Ok(()),
        // Documents
        Some("pdf" | "txt" | "md" | "html" | "htm") => Ok(()),
        // Archives (for asset packs)
        Some("zip" | "tar" | "gz") => Ok(()),
        _ => Err(ValidationError::new("disallowed_file_extension")),
    }
}

/// Validate asset file size (in bytes)
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_asset_size;
/// 
/// assert!(validate_asset_size(1024 * 1024).is_ok());     // 1MB - OK
/// assert!(validate_asset_size(100 * 1024 * 1024).is_err()); // 100MB - Too large
/// ```
pub fn validate_asset_size(size_bytes: u64) -> Result<(), ValidationError> {
    const MAX_ASSET_SIZE: u64 = 50 * 1024 * 1024; // 50MB limit
    
    if size_bytes > MAX_ASSET_SIZE {
        return Err(ValidationError::new("asset_too_large"));
    }

    Ok(())
}

/// Validate URL domain against allowlist
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_url_domain;
/// 
/// let allowed_domains = vec!["example.com", "cdn.example.com"];
/// assert!(validate_url_domain("https://example.com/image.png", &allowed_domains).is_ok());
/// assert!(validate_url_domain("https://malicious.com/image.png", &allowed_domains).is_err());
/// ```
pub fn validate_url_domain(url: &str, allowed_domains: &[&str]) -> Result<(), ValidationError> {
    let parsed_url = Url::parse(url).map_err(|_| ValidationError::new("invalid_url"))?;
    
    if let Some(domain) = parsed_url.domain() {
        let domain_lower = domain.to_lowercase();
        
        for allowed in allowed_domains {
            let allowed_lower = allowed.to_lowercase();
            if domain_lower == allowed_lower || domain_lower.ends_with(&format!(".{allowed_lower}")) {
                return Ok(());
            }
        }
    }

    Err(ValidationError::new("domain_not_allowed"))
}

/// Validate dice notation (e.g., "1d20", "2d6+3")
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_dice_notation;
/// 
/// assert!(validate_dice_notation("1d20").is_ok());
/// assert!(validate_dice_notation("2d6+3").is_ok());
/// assert!(validate_dice_notation("invalid").is_err());
/// ```
pub fn validate_dice_notation(dice: &str) -> Result<(), ValidationError> {
    lazy_static::lazy_static! {
        static ref DICE_REGEX: Regex = Regex::new(
            r"^(\d+)?d(\d+)([+\-]\d+)?$"
        ).unwrap();
    }

    if !DICE_REGEX.is_match(dice) {
        return Err(ValidationError::new("invalid_dice_notation"));
    }

    Ok(())
}

/// Validate HTML content for safety (basic XSS prevention)
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_html_safety;
/// 
/// assert!(validate_html_safety("<p>Safe content</p>").is_ok());
/// assert!(validate_html_safety("<script>alert('xss')</script>").is_err());
/// ```
pub fn validate_html_safety(html: &str) -> Result<(), ValidationError> {
    lazy_static::lazy_static! {
        static ref SCRIPT_REGEX: Regex = Regex::new(r"<\s*script[^>]*>").unwrap();
        static ref ON_EVENT_REGEX: Regex = Regex::new(r"\bon\w+\s*=").unwrap();
        static ref JAVASCRIPT_REGEX: Regex = Regex::new(r"javascript\s*:").unwrap();
    }

    let html_lower = html.to_lowercase();

    if SCRIPT_REGEX.is_match(&html_lower) {
        return Err(ValidationError::new("script_tags_not_allowed"));
    }

    if ON_EVENT_REGEX.is_match(&html_lower) {
        return Err(ValidationError::new("event_handlers_not_allowed"));
    }

    if JAVASCRIPT_REGEX.is_match(&html_lower) {
        return Err(ValidationError::new("javascript_urls_not_allowed"));
    }

    Ok(())
}

/// Validate numeric range (e.g., for ability scores, levels)
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::validate_numeric_range;
/// 
/// assert!(validate_numeric_range(15, 3, 18).is_ok());  // D&D ability score
/// assert!(validate_numeric_range(25, 3, 18).is_err()); // Too high
/// ```
pub fn validate_numeric_range<T: PartialOrd>(value: T, min: T, max: T) -> Result<(), ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::new("value_out_of_range"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_system_id() {
        assert!(validate_system_id("dnd5e").is_ok());
        assert!(validate_system_id("pathfinder2e").is_ok());
        assert!(validate_system_id("invalid-system!").is_err());
        assert!(validate_system_id("").is_err());
        assert!(validate_system_id("123invalid").is_err());
    }

    #[test]
    fn test_validate_entity_id() {
        assert!(validate_entity_id("abc123").is_ok());
        assert!(validate_entity_id("550e8400-e29b-41d4-a716-446655440000").is_ok());
        assert!(validate_entity_id("").is_err());
        assert!(validate_entity_id(&"x".repeat(65)).is_err());
    }

    #[test]
    fn test_validate_asset_extension() {
        assert!(validate_asset_extension("test.png").is_ok());
        assert!(validate_asset_extension("audio.mp3").is_ok());
        assert!(validate_asset_extension("malware.exe").is_err());
        assert!(validate_asset_extension("document.pdf").is_ok());
    }

    #[test]
    fn test_validate_asset_size() {
        assert!(validate_asset_size(1024).is_ok());
        assert!(validate_asset_size(100 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_validate_dice_notation() {
        assert!(validate_dice_notation("1d20").is_ok());
        assert!(validate_dice_notation("2d6+3").is_ok());
        assert!(validate_dice_notation("d10").is_ok());
        assert!(validate_dice_notation("invalid").is_err());
        assert!(validate_dice_notation("1d").is_err());
    }

    #[test]
    fn test_validate_html_safety() {
        assert!(validate_html_safety("<p>Safe</p>").is_ok());
        assert!(validate_html_safety("<script>alert('xss')</script>").is_err());
        assert!(validate_html_safety("<div onclick='alert()'>").is_err());
        assert!(validate_html_safety("<a href='javascript:alert()'>").is_err());
    }

    #[test]
    fn test_validate_numeric_range() {
        assert!(validate_numeric_range(15, 3, 18).is_ok());
        assert!(validate_numeric_range(2, 3, 18).is_err());
        assert!(validate_numeric_range(19, 3, 18).is_err());
    }
}
