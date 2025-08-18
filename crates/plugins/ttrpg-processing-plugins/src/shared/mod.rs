//! Shared execution contexts for processing plugins
//!
//! This module provides shared resources and execution contexts that are used
//! across multiple processing plugins to enable efficient resource utilization
//! and coordinated parallel processing.

use std::sync::Arc;
use tokio::sync::Semaphore;
use dashmap::DashMap;
use jsonschema::JSONSchema;

/// Shared execution context for asset processing plugins
/// 
/// This context provides shared resources for asset-related operations including
/// HTTP client pooling, controlled concurrency, and CPU-bound work coordination.
pub struct AssetExecutionContext {
    /// Single tokio runtime with controlled thread pool
    pub runtime: Arc<tokio::runtime::Runtime>,
    /// Limit concurrent operations (e.g., max 50 concurrent downloads)
    pub semaphore: Arc<Semaphore>,
    /// Shared HTTP client with connection pooling
    pub http_client: Arc<reqwest::Client>,
    /// CPU-bound work pool (for image processing, hashing)
    pub cpu_pool: Arc<rayon::ThreadPool>,
}

impl AssetExecutionContext {
    /// Create new asset execution context with default configuration
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(num_cpus::get())
                .enable_all()
                .build()?
        );
        
        let semaphore = Arc::new(Semaphore::new(50)); // Limit concurrent downloads
        
        let http_client = Arc::new(
            reqwest::ClientBuilder::new()
                .timeout(std::time::Duration::from_secs(30))
                .pool_max_idle_per_host(10)
                .build()?
        );
        
        let cpu_pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .thread_name(|i| format!("asset-cpu-{}", i))
                .build()?
        );
        
        Ok(Self {
            runtime,
            semaphore,
            http_client,
            cpu_pool,
        })
    }
}

impl Default for AssetExecutionContext {
    fn default() -> Self {
        Self::new().expect("Failed to create default AssetExecutionContext")
    }
}

/// Shared execution context for validation plugin
///
/// This context provides shared resources for validation operations including
/// schema caching, CPU pooling for validation work, and rule engine coordination.
pub struct ValidationExecutionContext {
    /// Shared CPU pool for validation work
    pub cpu_pool: Arc<rayon::ThreadPool>,
    /// Schema cache (thread-safe)
    pub schema_cache: Arc<DashMap<String, JSONSchema>>,
    /// Rule engine (thread-safe)
    pub rule_engine: Arc<RuleEngine>,
}

impl ValidationExecutionContext {
    /// Create new validation execution context
    pub fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let cpu_pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .thread_name(|i| format!("validation-cpu-{}", i))
                .build()?
        );
        
        let schema_cache = Arc::new(DashMap::new());
        let rule_engine = Arc::new(RuleEngine::new());
        
        Ok(Self {
            cpu_pool,
            schema_cache,
            rule_engine,
        })
    }
}

impl Default for ValidationExecutionContext {
    fn default() -> Self {
        Self::new().expect("Failed to create default ValidationExecutionContext")
    }
}

/// Simple rule engine for validation logic
#[derive(Debug)]
pub struct RuleEngine {
    rules: DashMap<String, ValidationRule>,
}

impl RuleEngine {
    /// Create new rule engine
    pub fn new() -> Self {
        Self {
            rules: DashMap::new(),
        }
    }
    
    /// Add validation rule
    pub fn add_rule(&self, name: String, rule: ValidationRule) {
        self.rules.insert(name, rule);
    }
    
    /// Execute rule by name
    pub fn execute_rule(&self, name: &str, data: &serde_json::Value) -> ValidationResult {
        match self.rules.get(name) {
            Some(rule) => rule.execute(data),
            None => ValidationResult::error(format!("Rule '{}' not found", name)),
        }
    }
}

/// Validation rule definition
#[derive(Debug, Clone)]
pub struct ValidationRule {
    pub name: String,
    pub description: String,
    pub rule_type: RuleType,
}

impl ValidationRule {
    /// Execute validation rule
    pub fn execute(&self, _data: &serde_json::Value) -> ValidationResult {
        // TODO: Implement rule execution logic
        ValidationResult::success()
    }
}

/// Types of validation rules
#[derive(Debug, Clone)]
pub enum RuleType {
    /// JSON schema validation
    Schema(String), 
    /// Field presence validation
    RequiredField(String),
    /// Data type validation
    DataType(String),
    /// Custom validation function
    Custom(String),
}

/// Validation result from rule execution
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub messages: Vec<String>,
    pub severity: ValidationSeverity,
}

impl ValidationResult {
    /// Create successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            messages: vec![],
            severity: ValidationSeverity::Info,
        }
    }
    
    /// Create error validation result
    pub fn error(message: String) -> Self {
        Self {
            is_valid: false,
            messages: vec![message],
            severity: ValidationSeverity::Error,
        }
    }
}

/// Validation severity levels
#[derive(Debug, Clone, Copy)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
}
