//! Pathbuilder PF2e specific utilities

#[derive(Debug)]
pub enum ParseError {
    InvalidIdFormat(String),
    InvalidUuidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidIdFormat(id) => write!(f, "Invalid Pathbuilder ID format: {}", id),
            ParseError::InvalidUuidFormat(uuid) => write!(f, "Invalid UUID format: {}", uuid),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse Pathbuilder database ID
pub fn parse_pathbuilder_id(id: &str) -> Result<String, ParseError> {
    if id.trim().is_empty() {
        return Err(ParseError::InvalidIdFormat("Empty ID".to_string()));
    }
    
    // For now, just validate it's not empty and return
    Ok(id.trim().to_string())
}

/// Parse Pathbuilder UUID format
pub fn parse_pathbuilder_uuid(uuid: &str) -> Result<String, ParseError> {
    if uuid.trim().is_empty() {
        return Err(ParseError::InvalidUuidFormat("Empty UUID".to_string()));
    }
    
    // Basic UUID format validation could be added here
    Ok(uuid.trim().to_string())
}

/// Convert Pathbuilder trait strings to internal format
pub fn parse_pathbuilder_traits(traits: &[String]) -> Vec<String> {
    traits.iter().map(|t| t.trim().to_string()).collect()
}
