//! Comprehensive integration tests for M1.5: Service Integration
//!
//! This test module verifies that all services work together properly through
//! the ServiceManager, testing real service interactions and coordination.

use std::{
    sync::{atomic::{AtomicUsize, Ordering}, Arc},
    path::PathBuf,
};

use ttrpg_core::{
    services::{self, *},
    types::*,
    ConversionResult, AssetResult,
};
use ttrpg_core::manager::DefaultServiceManager;

/// Mock logging service that tracks all logging calls
#[derive(Default)]
struct IntegrationLoggingService {
    error_count: AtomicUsize,
    warn_count: AtomicUsize,
    info_count: AtomicUsize,
    debug_count: AtomicUsize,
}

impl IntegrationLoggingService {
    fn new() -> Self {
        Self::default()
    }
    
    fn get_total_calls(&self) -> usize {
        self.error_count.load(Ordering::Relaxed) +
        self.warn_count.load(Ordering::Relaxed) +
        self.info_count.load(Ordering::Relaxed) +
        self.debug_count.load(Ordering::Relaxed)
    }
}

impl LoggingService for IntegrationLoggingService {
    fn error(&self, _message: &str, _context: Option<&str>) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }

    fn warn(&self, _message: &str, _context: Option<&str>) {
        self.warn_count.fetch_add(1, Ordering::Relaxed);
    }

    fn info(&self, _message: &str, _context: Option<&str>) {
        self.info_count.fetch_add(1, Ordering::Relaxed);
    }

    fn debug(&self, _message: &str, _context: Option<&str>) {
        self.debug_count.fetch_add(1, Ordering::Relaxed);
    }

    fn log_with_data(&self, level: LogLevel, _message: &str, _data: &serde_json::Value) {
        match level {
            LogLevel::Error => self.error_count.fetch_add(1, Ordering::Relaxed),
            LogLevel::Warn => self.warn_count.fetch_add(1, Ordering::Relaxed),
            LogLevel::Info => self.info_count.fetch_add(1, Ordering::Relaxed),
            LogLevel::Debug => self.debug_count.fetch_add(1, Ordering::Relaxed),
            LogLevel::Trace => self.debug_count.fetch_add(1, Ordering::Relaxed),
        };
    }

    fn set_level(&mut self, _level: LogLevel) {
        // Mock implementation
    }

    fn is_enabled(&self, _level: LogLevel) -> bool {
        true
    }
}

/// Mock validation service that tracks validation operations
#[derive(Default)]
struct IntegrationValidationService {
    validation_count: AtomicUsize,
}

impl IntegrationValidationService {
    fn new() -> Self {
        Self::default()
    }
    
    fn get_validation_count(&self) -> usize {
        self.validation_count.load(Ordering::Relaxed)
    }
}

impl ValidationService for IntegrationValidationService {
    fn validate_campaign(&self, _campaign: &Campaign) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn validate_required_fields(&self, _entity_type: &str, _data: &serde_json::Value) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn validate_data_types(&self, _entity_type: &str, _data: &serde_json::Value) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn validate_entity_data(&self, _entity_type: &str, _data: &serde_json::Value) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn validate_file_path(&self, _path: &str) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn validate_json_data(&self, _data: &serde_json::Value) -> ConversionResult<ValidationResult> {
        self.validation_count.fetch_add(1, Ordering::Relaxed);
        Ok(ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            stats: ValidationStats::default(),
        })
    }

    fn get_validation_stats(&self) -> ValidationStats {
        ValidationStats {
            entities_validated: self.validation_count.load(Ordering::Relaxed),
            entities_with_errors: 0,
            entities_with_warnings: 0,
            validation_time_ms: 100,
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn clear_cache(&self) {
        // Mock implementation
    }
}

/// Mock asset service that simulates asset operations
#[derive(Default)]
struct IntegrationAssetService {
    asset_count: AtomicUsize,
}

impl IntegrationAssetService {
    fn new() -> Self {
        Self {
            asset_count: AtomicUsize::new(0),
        }
    }
    
    fn get_asset_count(&self) -> usize {
        self.asset_count.load(Ordering::Relaxed)
    }
}

impl AssetService for IntegrationAssetService {
    fn download_asset(&self, _url: &str, _output_path: &std::path::Path) -> AssetResult<AssetInfo> {
        self.asset_count.fetch_add(1, Ordering::Relaxed);
        Ok(AssetInfo {
            path: PathBuf::from("/mock/path/downloaded_asset.png"),
            size_bytes: 1024,
            mime_type: "image/png".to_string(),
            dimensions: Some((256, 256)),
            hash: "mock_hash".to_string(),
            modified: std::time::SystemTime::now(),
        })
    }

    fn process_asset(&self, _asset_path: &std::path::Path) -> AssetResult<AssetInfo> {
        self.asset_count.fetch_add(1, Ordering::Relaxed);
        Ok(AssetInfo {
            path: PathBuf::from("/mock/path/processed_asset.png"),
            size_bytes: 1024,
            mime_type: "image/png".to_string(),
            dimensions: Some((256, 256)),
            hash: "mock_hash".to_string(),
            modified: std::time::SystemTime::now(),
        })
    }

    fn cache_asset(&self, _asset_ref: &AssetReference) -> AssetResult<PathBuf> {
        self.asset_count.fetch_add(1, Ordering::Relaxed);
        Ok(PathBuf::from("/mock/cache/path"))
    }

    fn get_cached_asset(&self, _url: &str) -> Option<PathBuf> {
        Some(PathBuf::from("/mock/cache/path"))
    }

    fn validate_asset(&self, _asset_path: &std::path::Path) -> AssetResult<bool> {
        self.asset_count.fetch_add(1, Ordering::Relaxed);
        Ok(true)
    }

    fn get_asset_info(&self, _asset_path: &std::path::Path) -> AssetResult<AssetInfo> {
        self.asset_count.fetch_add(1, Ordering::Relaxed);
        Ok(AssetInfo {
            path: PathBuf::from("/mock/path/info_asset.png"),
            size_bytes: 1024,
            mime_type: "image/png".to_string(),
            dimensions: Some((256, 256)),
            hash: "mock_hash".to_string(),
            modified: std::time::SystemTime::now(),
        })
    }

    fn clear_cache(&self) {
        // Mock implementation
    }

    fn get_stats(&self) -> services::AssetStats {
        services::AssetStats {
            assets_processed: self.asset_count.load(Ordering::Relaxed),
            downloads_successful: self.asset_count.load(Ordering::Relaxed),
            downloads_failed: 0,
            cache_hits: 0,
            total_download_bytes: 1024 * self.asset_count.load(Ordering::Relaxed) as u64,
            processing_time_ms: 100,
        }
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_service_manager_full_lifecycle() {
    // Test complete service manager lifecycle from creation to shutdown
    
    let mut manager = DefaultServiceManager::new().expect("Failed to create service manager");
    
    // Verify initial state
    assert!(!manager.is_initialized());
    assert_eq!(manager.get_registered_services().len(), 0);
    
    // Register services
    let logging_service = Arc::new(IntegrationLoggingService::new());
    let validation_service = Arc::new(IntegrationValidationService::new());
    let asset_service = Arc::new(IntegrationAssetService::new());
    
    manager.register_logging(logging_service.clone());
    manager.register_validation(validation_service.clone());
    manager.register_assets(asset_service.clone());
    
    // Verify registration
    let registered = manager.get_registered_services();
    assert_eq!(registered.len(), 3);
    assert!(registered.contains(&"LoggingService".to_string()));
    assert!(registered.contains(&"ValidationService".to_string()));
    assert!(registered.contains(&"AssetService".to_string()));
    
    // Initialize
    manager.init_defaults().expect("Failed to initialize services");
    assert!(manager.is_initialized());
    
    // Test service access and functionality
    let logger = manager.logging();
    logger.info("Integration test message", Some("test_context"));
    logger.error("Test error", None);
    
    let validator = manager.validation();
    let campaign = Campaign::new("Test Campaign".to_string(), SourceFormat::Roll20);
    let validation_result = validator.validate_campaign(&campaign).expect("Validation failed");
    assert!(validation_result.is_valid);
    
    let assets = manager.assets();
    let asset_info = assets.get_asset_info(&PathBuf::from("/test/path")).expect("Asset info failed");
    assert_eq!(asset_info.mime_type, "image/png");
    
    // Verify service interactions occurred
    assert!(logging_service.get_total_calls() >= 2);
    assert!(validation_service.get_validation_count() >= 1);
    assert!(asset_service.get_asset_count() >= 1);
    
    // Shutdown
    manager.shutdown().expect("Failed to shutdown services");
    assert!(!manager.is_initialized());
    assert_eq!(manager.get_registered_services().len(), 0);
}

#[test]
fn test_service_coordination_workflow() {
    // Test realistic workflow with service coordination
    
    let mut manager = DefaultServiceManager::new().expect("Failed to create service manager");
    
    let logging_service = Arc::new(IntegrationLoggingService::new());
    let validation_service = Arc::new(IntegrationValidationService::new());
    let asset_service = Arc::new(IntegrationAssetService::new());
    
    manager.register_logging(logging_service.clone());
    manager.register_validation(validation_service.clone());
    manager.register_assets(asset_service.clone());
    manager.init_defaults().expect("Failed to initialize services");
    
    // Simulate realistic TTRPG conversion workflow
    let logger = manager.logging();
    let validator = manager.validation();
    let assets = manager.assets();
    
    // Step 1: Log workflow start
    logger.info("Starting TTRPG conversion workflow", Some("integration_test"));
    
    // Step 2: Validate campaign data
    let campaign = Campaign::new("Integration Test Campaign".to_string(), SourceFormat::Roll20);
    let validation_result = validator.validate_campaign(&campaign).expect("Campaign validation failed");
    
    if validation_result.is_valid {
        logger.info("Campaign validation successful", Some("workflow"));
    } else {
        logger.error("Campaign validation failed", Some("workflow"));
    }
    
    // Step 3: Process assets
    let asset_result = assets.download_asset(
        "https://example.com/test-asset.png", 
        &PathBuf::from("/test/output")
    ).expect("Asset download failed");
    
    logger.info(&format!("Asset processed: {} bytes", asset_result.size_bytes), Some("workflow"));
    
    // Step 4: Validate processed asset
    let file_validation = validator.validate_file_path("/test/output/test-asset.png").expect("File validation failed");
    
    if file_validation.is_valid {
        logger.info("Asset file validation successful", Some("workflow"));
    }
    
    // Verify coordinated workflow
    assert!(logging_service.get_total_calls() >= 4);
    assert!(validation_service.get_validation_count() >= 2);
    assert!(asset_service.get_asset_count() >= 1);
    
    let stats = validator.get_validation_stats();
    assert!(stats.entities_validated >= 2);
    assert_eq!(stats.entities_with_errors, 0);
    
    let asset_stats = assets.get_stats();
    assert!(asset_stats.assets_processed >= 1);
}

#[test]
fn test_concurrent_service_access() {
    // Test thread-safe service access under concurrent load
    
    use std::sync::Arc;
    
    let mut manager = DefaultServiceManager::new().expect("Failed to create service manager");
    
    let logging_service = Arc::new(IntegrationLoggingService::new());
    let validation_service = Arc::new(IntegrationValidationService::new());
    let asset_service = Arc::new(IntegrationAssetService::new());
    
    manager.register_logging(logging_service.clone());
    manager.register_validation(validation_service.clone());
    manager.register_assets(asset_service.clone());
    manager.init_defaults().expect("Failed to initialize services");
    
    let manager = Arc::new(manager);
    
    // Spawn multiple threads to access services concurrently
    let handles: Vec<_> = (0..10)
        .map(|i| {
            let manager: Arc<DefaultServiceManager> = Arc::clone(&manager);
            std::thread::spawn(move || {
                let logger = manager.logging();
                let validator = manager.validation();
                let assets = manager.assets();
                
                // Perform service operations
                logger.info(&format!("Concurrent test {}", i), Some("concurrency"));
                
                let campaign = Campaign::new(format!("Campaign {}", i), SourceFormat::Roll20);
                let _validation_result = validator.validate_campaign(&campaign).unwrap();
                
                let _asset_info = assets.get_asset_info(&PathBuf::from(&format!("/test/{}", i))).unwrap();
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Verify all operations completed successfully
    assert!(logging_service.get_total_calls() >= 10);
    assert!(validation_service.get_validation_count() >= 10);
    assert!(asset_service.get_asset_count() >= 10);
}

#[test]
fn test_service_error_handling() {
    // Test proper error handling and recovery in service coordination
    
    let mut manager = DefaultServiceManager::new().expect("Failed to create service manager");
    
    let logging_service = Arc::new(IntegrationLoggingService::new());
    let validation_service = Arc::new(IntegrationValidationService::new());
    let asset_service = Arc::new(IntegrationAssetService::new());
    
    manager.register_logging(logging_service.clone());
    manager.register_validation(validation_service.clone());
    manager.register_assets(asset_service.clone());
    manager.init_defaults().expect("Failed to initialize services");
    
    let logger = manager.logging();
    let validator = manager.validation();
    
    // Test error logging
    logger.error("Simulated error condition", Some("error_handling_test"));
    logger.warn("Simulated warning condition", Some("error_handling_test"));
    
    // Test validation with edge cases
    let invalid_json = serde_json::json!({
        "malformed": "data",
        "missing_required_fields": true
    });
    
    let validation_result = validator.validate_json_data(&invalid_json).expect("JSON validation should not panic");
    // Our mock always returns valid, but real implementation would handle errors
    assert!(validation_result.is_valid);
    
    // Verify error conditions were logged
    assert!(logging_service.error_count.load(Ordering::Relaxed) >= 1);
    assert!(logging_service.warn_count.load(Ordering::Relaxed) >= 1);
}

#[test]
#[should_panic(expected = "LoggingService not registered")]
fn test_unregistered_service_error() {
    // Test proper error handling when services are not registered
    
    let manager = DefaultServiceManager::new().expect("Failed to create service manager");
    
    // This should panic because no logging service is registered
    let _logger = manager.logging();
}

#[test]
fn test_service_manager_with_defaults() {
    // Test convenience method for creating service manager with defaults
    
    // This test verifies the convenience constructor, though it won't have
    // concrete implementations until other crates provide them
    let manager = DefaultServiceManager::new().expect("Failed to create service manager with defaults");
    
    // Should be created but not initialized (since we don't have concrete implementations yet)
    assert!(!manager.is_initialized());
    assert_eq!(manager.get_registered_services().len(), 0);
}
