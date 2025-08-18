//! Data validation helpers for Pathbuilder PF2e mappings

#[derive(Debug)]
pub enum ValidationError {
    InvalidLevel(i32),
    InvalidAbilityScore(i32),
    InvalidCurrencyAmount(f64),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidLevel(level) => write!(f, "Invalid level: {}. Must be between 1 and 20", level),
            ValidationError::InvalidAbilityScore(score) => write!(f, "Invalid ability score: {}. Must be between 1 and 30", score),
            ValidationError::InvalidCurrencyAmount(amount) => write!(f, "Invalid currency amount: {}. Must be non-negative", amount),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate character level is within valid PF2e bounds
pub fn validate_character_level(level: i32) -> Result<u8, ValidationError> {
    match level {
        1..=20 => Ok(level as u8),
        _ => Err(ValidationError::InvalidLevel(level)),
    }
}

/// Validate ability score is within valid bounds
pub fn validate_ability_score(score: i32) -> Result<u8, ValidationError> {
    match score {
        1..=30 => Ok(score as u8),
        _ => Err(ValidationError::InvalidAbilityScore(score)),
    }
}

/// Validate currency amount is non-negative
pub fn validate_currency_amount(amount: f64) -> Result<f64, ValidationError> {
    if amount >= 0.0 {
        Ok(amount)
    } else {
        Err(ValidationError::InvalidCurrencyAmount(amount))
    }
}
