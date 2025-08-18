//! String, URL, and filename sanitization utilities
//!
//! This module provides safe sanitization functions for user-generated content,
//! URLs, filenames, and other potentially unsafe data.

use regex::Regex;
use std::path::{Path, PathBuf};
use url::Url;

/// Sanitize a string for safe use in filenames
/// 
/// # Errors
/// Returns an error if the input is empty after sanitization
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::sanitize_filename;
/// 
/// let result = sanitize_filename("My Campaign: Part 1/2");
/// assert_eq!(result.unwrap(), "My Campaign- Part 1-2");
/// ```
pub fn sanitize_filename(input: &str) -> Result<String, SanitizationError> {
    if input.trim().is_empty() {
        return Err(SanitizationError::EmptyInput);
    }

    // Replace problematic characters with safe alternatives
    let sanitized = input
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' => '-',
            '/' | '\\' => '-',
            '\0'..='\x1f' => '_', // Control characters
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>();

    // Remove leading/trailing whitespace and dots
    let sanitized = sanitized.trim().trim_matches('.');
    
    if sanitized.is_empty() {
        return Err(SanitizationError::EmptyAfterSanitization);
    }

    // Truncate if too long (Windows has 255 char limit)
    let truncated = if sanitized.len() > 250 {
        &sanitized[..250]
    } else {
        sanitized
    };

    Ok(truncated.to_string())
}

/// Sanitize a string for safe use in HTML/XML contexts
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::sanitize_html;
/// 
/// let result = sanitize_html("<script>alert('xss')</script>");
/// assert_eq!(result, "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
/// ```
pub fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Sanitize and validate a URL for asset references
/// 
/// # Errors
/// Returns an error if the URL is invalid or uses a disallowed scheme
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::sanitize_url;
/// 
/// let result = sanitize_url("https://example.com/image.png");
/// assert!(result.is_ok());
/// 
/// let result = sanitize_url("javascript:alert('xss')");
/// assert!(result.is_err());
/// ```
pub fn sanitize_url(input: &str) -> Result<Url, SanitizationError> {
    // Parse the URL
    let url = Url::parse(input).map_err(|_| SanitizationError::InvalidUrl)?;

    // Check allowed schemes
    match url.scheme() {
        "http" | "https" | "file" => Ok(url),
        "data" => {
            // Allow data URLs for images only
            if let Some(mime) = url.path().split(',').next() {
                if mime.starts_with("image/") {
                    Ok(url)
                } else {
                    Err(SanitizationError::DisallowedScheme(url.scheme().to_string()))
                }
            } else {
                Err(SanitizationError::DisallowedScheme(url.scheme().to_string()))
            }
        }
        scheme => Err(SanitizationError::DisallowedScheme(scheme.to_string())),
    }
}

/// Clean and normalize text content (remove excessive whitespace, etc.)
/// 
/// # Examples
/// ```no_run
/// use ttrpg_core::utils::clean_text;
/// 
/// let result = clean_text("  Hello\n\n\nworld!  \t ");
/// assert_eq!(result, "Hello world!");
/// ```
pub fn clean_text(input: &str) -> String {
    lazy_static::lazy_static! {
        static ref MULTIPLE_NEWLINES: Regex = Regex::new(r"\n\s*\n\s*\n+").unwrap();
        static ref MULTIPLE_SPACES: Regex = Regex::new(r"[^\S\n]+").unwrap();
    }

    let cleaned = input.trim();
    let cleaned = MULTIPLE_NEWLINES.replace_all(cleaned, "\n\n");
    let cleaned = MULTIPLE_SPACES.replace_all(&cleaned, " ");
    
    cleaned.to_string()
}

/// Sanitize a path to prevent directory traversal attacks
/// 
/// # Errors
/// Returns an error if the path contains parent directory references
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::sanitize_path;
/// 
/// let result = sanitize_path("assets/images/dragon.png");
/// assert!(result.is_ok());
/// 
/// let result = sanitize_path("../../../etc/passwd");
/// assert!(result.is_err());
/// ```
pub fn sanitize_path(input: &str) -> Result<PathBuf, SanitizationError> {
    let path = Path::new(input);
    
    // Check for parent directory references
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                return Err(SanitizationError::UnsafePath("Parent directory reference".to_string()));
            }
            std::path::Component::RootDir => {
                return Err(SanitizationError::UnsafePath("Absolute path not allowed".to_string()));
            }
            _ => {}
        }
    }

    // Normalize the path
    let normalized = path.components().collect::<PathBuf>();
    Ok(normalized)
}

/// Extract and sanitize asset filename from URL or path
/// 
/// # Examples
/// ```
/// use ttrpg_core::utils::extract_asset_filename;
/// 
/// let result = extract_asset_filename("https://example.com/path/dragon.png?v=1");
/// assert_eq!(result, "dragon.png");
/// ```
pub fn extract_asset_filename(url_or_path: &str) -> String {
    // Try parsing as URL first
    if let Ok(url) = Url::parse(url_or_path) {
        if let Some(mut segments) = url.path_segments() {
            if let Some(filename) = segments.next_back() {
                if !filename.is_empty() {
                    return sanitize_filename(filename).unwrap_or_else(|_| "asset".to_string());
                }
            }
        }
        // If URL has no filename in path, return default
        return "asset".to_string();
    }

    // Fall back to path handling
    if let Some(filename) = Path::new(url_or_path).file_name() {
        if let Some(filename_str) = filename.to_str() {
            return sanitize_filename(filename_str).unwrap_or_else(|_| "asset".to_string());
        }
    }

    // Last resort
    "asset".to_string()
}

/// Errors that can occur during sanitization
#[derive(Debug, thiserror::Error)]
pub enum SanitizationError {
    #[error("Input is empty")]
    EmptyInput,
    
    #[error("Input is empty after sanitization")]
    EmptyAfterSanitization,
    
    #[error("Invalid URL format")]
    InvalidUrl,
    
    #[error("URL scheme not allowed: {0}")]
    DisallowedScheme(String),
    
    #[error("Unsafe path: {0}")]
    UnsafePath(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("valid_name.txt").unwrap(), "valid_name.txt");
        assert_eq!(sanitize_filename("invalid:name<>").unwrap(), "invalid-name--");
        assert_eq!(sanitize_filename("path/to/file.txt").unwrap(), "path-to-file.txt");
        assert!(sanitize_filename("").is_err());
        assert!(sanitize_filename("   ").is_err());
    }

    #[test]
    fn test_sanitize_html() {
        assert_eq!(sanitize_html("<div>Hello</div>"), "&lt;div&gt;Hello&lt;/div&gt;");
        assert_eq!(sanitize_html("'quoted'"), "&#x27;quoted&#x27;");
        assert_eq!(sanitize_html("normal text"), "normal text");
    }

    #[test]
    fn test_sanitize_url() {
        assert!(sanitize_url("https://example.com/test.png").is_ok());
        assert!(sanitize_url("http://example.com/test.png").is_ok());
        assert!(sanitize_url("javascript:alert('xss')").is_err());
        assert!(sanitize_url("data:image/png;base64,iVBOR...").is_ok());
        assert!(sanitize_url("data:text/plain;base64,SGVsbG8=").is_err());
    }

    #[test]
    fn test_clean_text() {
        assert_eq!(clean_text("  hello   world  "), "hello world");
        assert_eq!(clean_text("line1\n\n\n\nline2"), "line1\n\nline2");
        assert_eq!(clean_text("normal text"), "normal text");
    }

    #[test]
    fn test_sanitize_path() {
        assert!(sanitize_path("assets/image.png").is_ok());
        assert!(sanitize_path("../config.txt").is_err());
        assert!(sanitize_path("/etc/passwd").is_err());
        assert!(sanitize_path("./assets/image.png").is_ok());
    }

    #[test]
    fn test_extract_asset_filename() {
        assert_eq!(extract_asset_filename("https://example.com/path/image.png"), "image.png");
        assert_eq!(extract_asset_filename("assets/local/image.jpg"), "image.jpg");
        assert_eq!(extract_asset_filename("https://example.com/"), "asset");
    }
}
