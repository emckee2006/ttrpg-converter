//! Validation types

use serde::{Deserialize, Serialize};

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Issue severity
    pub severity: IssueSeverity,
    /// Issue message
    pub message: String,
    /// Issue location
    pub location: Option<String>,
}

/// Issue severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation issues found
    pub issues: Vec<ValidationIssue>,
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStats {
    /// Total items validated
    pub total_validated: usize,
    /// Number of errors
    pub errors: usize,
    /// Number of warnings
    pub warnings: usize,
}

impl ValidationResult {
    /// Create a new successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
        }
    }

    /// Create a new validation result with issues
    pub fn with_issues(issues: Vec<ValidationIssue>) -> Self {
        let is_valid = !issues.iter().any(|issue| matches!(issue.severity, IssueSeverity::Error));
        Self { is_valid, issues }
    }

    /// Add an issue to the validation result
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        if matches!(issue.severity, IssueSeverity::Error) {
            self.is_valid = false;
        }
        self.issues.push(issue);
    }
}

impl ValidationIssue {
    /// Create a new error issue
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Error,
            message: message.into(),
            location: None,
        }
    }

    /// Create a new warning issue
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Warning,
            message: message.into(),
            location: None,
        }
    }

    /// Create a new info issue
    pub fn info(message: impl Into<String>) -> Self {
        Self {
            severity: IssueSeverity::Info,
            message: message.into(),
            location: None,
        }
    }

    /// Add location information to the issue
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }
}

impl ValidationStats {
    /// Create new empty validation stats
    pub fn new() -> Self {
        Self {
            total_validated: 0,
            errors: 0,
            warnings: 0,
        }
    }

    /// Update stats from a validation result
    pub fn update_from_result(&mut self, result: &ValidationResult) {
        self.total_validated += 1;
        
        for issue in &result.issues {
            match issue.severity {
                IssueSeverity::Error => self.errors += 1,
                IssueSeverity::Warning => self.warnings += 1,
                IssueSeverity::Info => {},
            }
        }
    }

    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_validated == 0 {
            return 0.0;
        }
        
        let successful = self.total_validated - self.errors;
        (successful as f64 / self.total_validated as f64) * 100.0
    }
}

impl Default for ValidationStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.is_valid);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_validation_result_with_warnings() {
        let issues = vec![
            ValidationIssue::warning("This is a warning"),
            ValidationIssue::info("This is info"),
        ];
        let result = ValidationResult::with_issues(issues);
        
        assert!(result.is_valid); // Warnings don't make it invalid
        assert_eq!(result.issues.len(), 2);
    }

    #[test]
    fn test_validation_result_with_errors() {
        let issues = vec![
            ValidationIssue::error("This is an error"),
            ValidationIssue::warning("This is a warning"),
        ];
        let result = ValidationResult::with_issues(issues);
        
        assert!(!result.is_valid); // Errors make it invalid
        assert_eq!(result.issues.len(), 2);
    }

    #[test]
    fn test_validation_result_add_issue() {
        let mut result = ValidationResult::success();
        assert!(result.is_valid);
        
        result.add_issue(ValidationIssue::warning("Warning added"));
        assert!(result.is_valid); // Still valid with warning
        assert_eq!(result.issues.len(), 1);
        
        result.add_issue(ValidationIssue::error("Error added"));
        assert!(!result.is_valid); // Now invalid with error
        assert_eq!(result.issues.len(), 2);
    }

    #[test]
    fn test_validation_issue_creation() {
        let error = ValidationIssue::error("Test error");
        assert_eq!(error.message, "Test error");
        matches!(error.severity, IssueSeverity::Error);
        assert!(error.location.is_none());

        let warning = ValidationIssue::warning("Test warning")
            .with_location("line 42");
        assert_eq!(warning.message, "Test warning");
        matches!(warning.severity, IssueSeverity::Warning);
        assert_eq!(warning.location, Some("line 42".to_string()));

        let info = ValidationIssue::info("Test info");
        assert_eq!(info.message, "Test info");
        matches!(info.severity, IssueSeverity::Info);
    }

    #[test]
    fn test_validation_stats_new() {
        let stats = ValidationStats::new();
        assert_eq!(stats.total_validated, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.warnings, 0);
        assert_eq!(stats.success_rate(), 0.0);
    }

    #[test]
    fn test_validation_stats_update() {
        let mut stats = ValidationStats::new();
        
        // Add successful result
        let success = ValidationResult::success();
        stats.update_from_result(&success);
        assert_eq!(stats.total_validated, 1);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.success_rate(), 100.0);

        // Add result with warning
        let warning_result = ValidationResult::with_issues(vec![
            ValidationIssue::warning("Test warning")
        ]);
        stats.update_from_result(&warning_result);
        assert_eq!(stats.total_validated, 2);
        assert_eq!(stats.warnings, 1);
        assert_eq!(stats.success_rate(), 100.0); // Warnings don't affect success rate

        // Add result with error
        let error_result = ValidationResult::with_issues(vec![
            ValidationIssue::error("Test error"),
            ValidationIssue::warning("Another warning"),
        ]);
        stats.update_from_result(&error_result);
        assert_eq!(stats.total_validated, 3);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.warnings, 2);
        assert!((stats.success_rate() - 66.66666666666667).abs() < 0.0001); // 2/3 success rate
    }

    #[test]
    fn test_issue_severity_variants() {
        let error_issue = ValidationIssue::error("Error message");
        let warning_issue = ValidationIssue::warning("Warning message");
        let info_issue = ValidationIssue::info("Info message");

        matches!(error_issue.severity, IssueSeverity::Error);
        matches!(warning_issue.severity, IssueSeverity::Warning);
        matches!(info_issue.severity, IssueSeverity::Info);
    }

    #[test]
    fn test_validation_serialization() {
        let issue = ValidationIssue::error("Test error")
            .with_location("test_file.rs:42");
        
        let result = ValidationResult::with_issues(vec![issue]);
        
        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: ValidationResult = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(result.is_valid, deserialized.is_valid);
        assert_eq!(result.issues.len(), deserialized.issues.len());
        assert_eq!(result.issues[0].message, deserialized.issues[0].message);
    }

    #[test]
    fn test_validation_stats_default() {
        let stats: ValidationStats = Default::default();
        assert_eq!(stats.total_validated, 0);
        assert_eq!(stats.errors, 0);
        assert_eq!(stats.warnings, 0);
    }
}