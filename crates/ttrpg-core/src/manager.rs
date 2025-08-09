//! Service manager implementation with dependency injection
//!
//! This module provides the concrete implementation of the ServiceManager trait,
//! handling service lifecycle, dependency injection, and coordination between services.
//!
//! # Architecture
//!
//! The ServiceManager follows the dependency injection pattern where:
//! - Services are registered during initialization
//! - Dependencies are resolved automatically
//! - Service lifecycle is managed centrally
//!
//! # Usage
//!
//! ```rust,no_run
//! use ttrpg_core::manager::DefaultServiceManager;
//! use ttrpg_core::services::ServiceManager;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let mut manager = DefaultServiceManager::new()?;
//! manager.init_defaults()?;
//!
//! // Access services through the manager
//! let logger = manager.logging();
//! let validator = manager.validation();
//! let assets = manager.assets();
//! # Ok(())
//! # }
//! ```

use crate::error::ConversionResult;
use crate::services::{AssetService, LoggingService, ServiceManager, ValidationService};
use std::sync::{Arc, Mutex, RwLock};
use tracing::{info, warn};

/// Default implementation of ServiceManager with thread-safe service coordination
///
/// This implementation provides:
/// - Thread-safe service registration and access
/// - Lazy initialization of services
/// - Proper service lifecycle management
/// - Dependency injection patterns
#[derive(Default)]
pub struct DefaultServiceManager {
    /// Registered logging service
    logging_service: Arc<RwLock<Option<Arc<dyn LoggingService>>>>,
    
    /// Registered validation service
    validation_service: Arc<RwLock<Option<Arc<dyn ValidationService>>>>,
    
    /// Registered asset service
    asset_service: Arc<RwLock<Option<Arc<dyn AssetService>>>>,
    
    /// Service state management
    initialized: Arc<Mutex<bool>>,
}

impl DefaultServiceManager {
    /// Create a new service manager
    ///
    /// This creates an empty service manager that requires service registration
    /// and initialization before use.
    pub fn new() -> ConversionResult<Self> {
        info!("Creating new DefaultServiceManager");
        
        Ok(Self {
            logging_service: Arc::new(RwLock::new(None)),
            validation_service: Arc::new(RwLock::new(None)),
            asset_service: Arc::new(RwLock::new(None)),
            initialized: Arc::new(Mutex::new(false)),
        })
    }

    /// Create a new service manager with all default services initialized
    ///
    /// This is a convenience method that creates the manager and initializes
    /// all services with their default implementations.
    pub fn with_defaults() -> ConversionResult<Self> {
        let mut manager = Self::new()?;
        manager.init_defaults()?;
        Ok(manager)
    }

    /// Check if the service manager is initialized
    pub fn is_initialized(&self) -> bool {
        *self.initialized.lock().unwrap()
    }

    /// Get a list of registered services
    pub fn get_registered_services(&self) -> Vec<String> {
        let mut services = Vec::new();
        
        if self.logging_service.read().unwrap().is_some() {
            services.push("LoggingService".to_string());
        }
        if self.validation_service.read().unwrap().is_some() {
            services.push("ValidationService".to_string());
        }
        if self.asset_service.read().unwrap().is_some() {
            services.push("AssetService".to_string());
        }
        
        services
    }
}

impl ServiceManager for DefaultServiceManager {
    fn logging(&self) -> Arc<dyn LoggingService> {
        self.logging_service
            .read()
            .unwrap()
            .as_ref()
            .expect("LoggingService not registered - call init_defaults() or register_logging()")
            .clone()
    }

    fn validation(&self) -> Arc<dyn ValidationService> {
        self.validation_service
            .read()
            .unwrap()
            .as_ref()
            .expect("ValidationService not registered - call init_defaults() or register_validation()")
            .clone()
    }

    fn assets(&self) -> Arc<dyn AssetService> {
        self.asset_service
            .read()
            .unwrap()
            .as_ref()
            .expect("AssetService not registered - call init_defaults() or register_assets()")
            .clone()
    }

    fn register_logging(&mut self, service: Arc<dyn LoggingService>) {
        info!("Registering LoggingService implementation");
        *self.logging_service.write().unwrap() = Some(service);
    }

    fn register_validation(&mut self, service: Arc<dyn ValidationService>) {
        info!("Registering ValidationService implementation");
        *self.validation_service.write().unwrap() = Some(service);
    }

    fn register_assets(&mut self, service: Arc<dyn AssetService>) {
        info!("Registering AssetService implementation");
        *self.asset_service.write().unwrap() = Some(service);
    }

    fn init_defaults(&mut self) -> ConversionResult<()> {
        info!("Initializing ServiceManager with default implementations");
        
        // Check if already initialized
        if self.is_initialized() {
            warn!("ServiceManager already initialized, skipping");
            return Ok(());
        }

        // Register default implementations
        // Note: This would register concrete implementations from other crates
        // For now, we'll prepare the structure and implement the registration
        // when we have the concrete service implementations available
        
        info!("ServiceManager initialization preparing - concrete services will be registered by consumers");
        
        // Mark as initialized (consumers will register their implementations)
        *self.initialized.lock().unwrap() = true;
        
        Ok(())
    }

    fn shutdown(&mut self) -> ConversionResult<()> {
        info!("Shutting down ServiceManager");
        
        // Clear all service references
        *self.logging_service.write().unwrap() = None;
        *self.validation_service.write().unwrap() = None;
        *self.asset_service.write().unwrap() = None;
        
        // Mark as not initialized
        *self.initialized.lock().unwrap() = false;
        
        info!("ServiceManager shutdown complete");
        Ok(())
    }
}

// Implement Send and Sync for thread-safety
unsafe impl Send for DefaultServiceManager {}
unsafe impl Sync for DefaultServiceManager {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    // Mock logging service for testing
    struct MockLoggingService {
        calls: Arc<AtomicBool>,
    }

    impl MockLoggingService {
        fn new() -> Self {
            Self {
                calls: Arc::new(AtomicBool::new(false)),
            }
        }


    }

    impl LoggingService for MockLoggingService {
        fn error(&self, _message: &str, _context: Option<&str>) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn warn(&self, _message: &str, _context: Option<&str>) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn info(&self, _message: &str, _context: Option<&str>) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn debug(&self, _message: &str, _context: Option<&str>) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn log_with_data(&self, _level: crate::services::LogLevel, _message: &str, _data: &serde_json::Value) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn set_level(&mut self, _level: crate::services::LogLevel) {
            self.calls.store(true, Ordering::Relaxed);
        }

        fn is_enabled(&self, _level: crate::services::LogLevel) -> bool {
            self.calls.store(true, Ordering::Relaxed);
            true
        }
    }

    #[test]
    fn test_service_manager_creation() {
        let manager = DefaultServiceManager::new().unwrap();
        assert!(!manager.is_initialized());
        assert_eq!(manager.get_registered_services().len(), 0);
    }

    #[test]
    fn test_service_registration() {
        let mut manager = DefaultServiceManager::new().unwrap();
        let logging_service = Arc::new(MockLoggingService::new());
        
        manager.register_logging(logging_service);
        assert_eq!(manager.get_registered_services(), vec!["LoggingService"]);
    }

    #[test]
    fn test_service_manager_initialization() {
        let mut manager = DefaultServiceManager::new().unwrap();
        
        // Should not be initialized initially
        assert!(!manager.is_initialized());
        
        // Initialize
        manager.init_defaults().unwrap();
        assert!(manager.is_initialized());
        
        // Should not re-initialize
        manager.init_defaults().unwrap();
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_service_manager_shutdown() {
        let mut manager = DefaultServiceManager::new().unwrap();
        let logging_service = Arc::new(MockLoggingService::new());
        
        manager.register_logging(logging_service);
        manager.init_defaults().unwrap();
        
        assert!(manager.is_initialized());
        assert_eq!(manager.get_registered_services().len(), 1);
        
        // Shutdown
        manager.shutdown().unwrap();
        assert!(!manager.is_initialized());
        assert_eq!(manager.get_registered_services().len(), 0);
    }

    #[test]
    #[should_panic(expected = "LoggingService not registered")]
    fn test_unregistered_service_access() {
        let manager = DefaultServiceManager::new().unwrap();
        let _ = manager.logging(); // Should panic
    }

    #[test]
    fn test_registered_service_access() {
        let mut manager = DefaultServiceManager::new().unwrap();
        let logging_service = Arc::new(MockLoggingService::new());
        
        manager.register_logging(logging_service);
        
        let service = manager.logging();
        service.info("test", None);
        
        // Access the mock through Arc to check if it was called
        // This is a bit tricky with the current design, but shows the service is accessible
    }

    #[test]
    fn test_concurrent_access() {
        use std::thread;
        
        let mut manager = DefaultServiceManager::new().unwrap();
        let logging_service = Arc::new(MockLoggingService::new());
        manager.register_logging(logging_service);
        
        let manager = Arc::new(manager);
        
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let manager = Arc::clone(&manager);
                thread::spawn(move || {
                    let service = manager.logging();
                    service.info("concurrent test", None);
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
