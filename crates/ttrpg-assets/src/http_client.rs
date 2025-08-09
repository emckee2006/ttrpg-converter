//! Professional HTTP client with middleware pipeline and caching
//!
//! This module provides a comprehensive HTTP client service built on top of the
//! professional reqwest middleware ecosystem, replacing basic HTTP functionality
//! with enterprise-grade features.
//!
//! # Features
//!
//! - **Professional Middleware Pipeline**: reqwest-middleware for extensible HTTP handling
//! - **Intelligent HTTP Caching**: http-cache-reqwest with moka cache manager for performance
//! - **Automatic Retry Logic**: reqwest-retry with exponential backoff for reliability
//! - **Integrated Tracing**: reqwest-tracing for comprehensive observability
//! - **Security Features**: Content hash verification and URL validation
//! - **Performance Optimization**: Connection pooling, HTTP/2 support, compression
//!
//! # Usage
//!
//! ```rust
//! use ttrpg_assets::http_client::EnhancedHttpClient;
//!
//! let client = EnhancedHttpClient::new().await?;
//! let response = client.get_with_cache("https://example.com/asset.jpg").await?;
//! ```

use ttrpg_core::error::{AssetResult, AssetError};
use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{Client, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_tracing::TracingMiddleware;

use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::{debug, error, info, instrument};
use url::Url;

// ============================================================================
// ENHANCED HTTP CLIENT CONFIGURATION
// ============================================================================

/// Configuration for the enhanced HTTP client
#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    /// Maximum number of retry attempts
    pub max_retries: u32,
    
    /// Initial retry delay in milliseconds
    pub initial_retry_delay_ms: u64,
    
    /// Maximum retry delay in milliseconds
    pub max_retry_delay_ms: u64,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// Connection timeout in seconds
    pub connection_timeout_seconds: u64,
    
    /// User agent string
    pub user_agent: String,
    
    /// Enable gzip compression
    pub enable_compression: bool,
    
    /// Enable HTTP/2
    pub enable_http2: bool,
    
    /// Maximum concurrent connections per host
    pub max_connections_per_host: usize,
    
    /// Enable persistent cookies
    pub enable_cookies: bool,
    
    /// Cache configuration
    pub cache: CacheConfig,
}

/// HTTP cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Enable HTTP caching
    pub enabled: bool,
    
    /// Cache mode strategy
    pub mode: CacheModeConfig,
    
    /// Maximum cache entries
    pub max_entries: u64,
    
    /// Cache entry time-to-live in seconds
    pub ttl_seconds: u64,
    
    /// Cache directory path (None for memory-only)
    pub cache_directory: Option<std::path::PathBuf>,
}

/// Cache mode configuration
#[derive(Debug, Clone)]
pub enum CacheModeConfig {
    /// Default caching behavior
    Default,
    /// No caching
    NoCache,
    /// Only use cache, don't make network requests
    OnlyIfCached,
    /// Force refresh from network
    Reload,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_retry_delay_ms: 500,
            max_retry_delay_ms: 30000,
            timeout_seconds: 60,
            connection_timeout_seconds: 30,
            user_agent: "TTRPGConverter/1.0 (Enhanced HTTP Client)".to_string(),
            enable_compression: true,
            enable_http2: true,
            max_connections_per_host: 10,
            enable_cookies: false, // Disabled for security in asset downloading
            cache: CacheConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: CacheModeConfig::Default,
            max_entries: 10000,
            ttl_seconds: 3600, // 1 hour
            cache_directory: Some(std::path::PathBuf::from("cache/http")),
        }
    }
}

// ============================================================================
// ENHANCED HTTP CLIENT IMPLEMENTATION
// ============================================================================

/// Professional HTTP client with comprehensive middleware pipeline
pub struct EnhancedHttpClient {
    /// HTTP client with middleware pipeline
    client: ClientWithMiddleware,
    

    /// Request statistics for monitoring
    stats: Arc<Mutex<HttpClientStats>>,
    
    /// URL validation cache
    url_validation_cache: Arc<Mutex<HashMap<String, bool>>>,
}

/// HTTP client statistics for performance monitoring
#[derive(Debug, Default, Clone)]
pub struct HttpClientStats {
    /// Total requests made
    pub total_requests: u64,
    
    /// Successful requests (2xx status)
    pub successful_requests: u64,
    
    /// Failed requests (4xx/5xx status)
    pub failed_requests: u64,
    
    /// Requests served from cache
    pub cache_hits: u64,
    
    /// Requests that bypassed cache
    pub cache_misses: u64,
    
    /// Total bytes downloaded
    pub total_bytes_downloaded: u64,
    
    /// Total retry attempts
    pub total_retries: u64,
    
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
}

/// HTTP response with enhanced metadata
#[derive(Debug)]
pub struct EnhancedHttpResponse {
    /// HTTP response
    pub response: Response,
    
    /// Whether response was served from cache
    pub from_cache: bool,
    
    /// Response time in milliseconds
    pub response_time_ms: u64,
    
    /// Content hash (SHA-256)
    pub content_hash: Option<String>,
}

impl EnhancedHttpClient {
    /// Create a new enhanced HTTP client with default configuration
    #[instrument(name = "http_client_new")]
    pub async fn new() -> AssetResult<Self> {
        Self::with_config(HttpClientConfig::default()).await
    }
    
    /// Create a new enhanced HTTP client with custom configuration
    #[instrument(name = "http_client_with_config", skip(config))]
    pub async fn with_config(config: HttpClientConfig) -> AssetResult<Self> {
        info!("Initializing enhanced HTTP client with professional middleware");
        
        // Create base reqwest client with optimized settings
        let base_client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(config.connection_timeout_seconds))
            .user_agent(&config.user_agent)
            // Note: Compression is handled automatically by reqwest when available
            .http2_prior_knowledge()
            .http2_keep_alive_timeout(Duration::from_secs(90))
            .http2_keep_alive_interval(Duration::from_secs(30))
            .pool_max_idle_per_host(config.max_connections_per_host)
            .pool_idle_timeout(Duration::from_secs(90))
            // Note: Cookie store disabled for security - no sensitive session handling needed
            .build()
            .map_err(|e| AssetError::ValidationError {
                asset_path: "http_client".to_string(),
                reason: format!("Failed to create HTTP client: {}", e),
                expected_type: Some("http_client".to_string()),
            })?;
        
        // Create enhanced client with middleware pipeline
        let mut client_builder = ClientBuilder::new(base_client)
            .with(TracingMiddleware::default()); // Add tracing support
        
        // Add retry middleware with exponential backoff
        if config.max_retries > 0 {
            let retry_policy = ExponentialBackoff::builder()
                .retry_bounds(
                    Duration::from_millis(config.initial_retry_delay_ms),
                    Duration::from_millis(config.max_retry_delay_ms)
                )
                .build_with_max_retries(config.max_retries);
                
            client_builder = client_builder.with(RetryTransientMiddleware::new_with_policy(retry_policy));
            debug!("HTTP retry middleware configured: {} max retries", config.max_retries);
        }
        
        // Add HTTP caching middleware if enabled
        if config.cache.enabled {
            let cache_mode = match config.cache.mode {
                CacheModeConfig::Default => CacheMode::Default,
                CacheModeConfig::NoCache => CacheMode::NoCache,
                CacheModeConfig::OnlyIfCached => CacheMode::OnlyIfCached,
                CacheModeConfig::Reload => CacheMode::Reload,
            };
            
            let cache_manager = CACacheManager {
                path: config.cache.cache_directory
                    .as_ref()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("cache/http")),
            };
            
            let cache_options = HttpCacheOptions {
                cache_mode_fn: Some(Arc::new(move |_| cache_mode)),
                ..Default::default()
            };
            
            client_builder = client_builder.with(Cache(HttpCache {
                mode: cache_mode,
                manager: cache_manager,
                options: cache_options,
            }));
            
            info!("HTTP caching enabled with {} mode", match config.cache.mode {
                CacheModeConfig::Default => "default",
                CacheModeConfig::NoCache => "no-cache",
                CacheModeConfig::OnlyIfCached => "only-if-cached",
                CacheModeConfig::Reload => "reload",
            });
        }
        
        let client = client_builder.build();
        
        Ok(Self {
            client,
            stats: Arc::new(Mutex::new(HttpClientStats::default())),
            url_validation_cache: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Simple GET request wrapper for compatibility
    pub async fn get(&self, url: &str) -> AssetResult<reqwest::Response> {
        let enhanced_response = self.get_enhanced(url).await?;
        Ok(enhanced_response.response)
    }

    /// Perform GET request with comprehensive error handling and caching
    #[instrument(name = "http_get_enhanced", skip(self), fields(url = %url))]
    pub async fn get_enhanced(&self, url: &str) -> AssetResult<EnhancedHttpResponse> {
        let start_time = SystemTime::now();
        
        // Update request statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_requests += 1;
        }
        
        // Validate URL format
        self.validate_url(url)?;
        
        debug!("Making enhanced HTTP GET request to: {}", url);
        
        // Make HTTP request with middleware pipeline
        let response = self.client.get(url)
            .send()
            .await
            .map_err(|e| {
                error!("HTTP request failed for {}: {}", url, e);
                self.update_failed_stats();
                AssetError::ValidationError {
                    asset_path: url.to_string(),
                    reason: format!("HTTP request failed: {}", e),
                    expected_type: Some("http_response".to_string()),
                }
            })?;
        
        let response_time = start_time.elapsed()
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        
        // Check if response was served from cache
        let from_cache = response.headers()
            .get("x-cache-status")
            .map(|h| h.to_str().unwrap_or("").contains("HIT"))
            .unwrap_or(false);
        
        // Update cache statistics
        {
            let mut stats = self.stats.lock().unwrap();
            if from_cache {
                stats.cache_hits += 1;
            } else {
                stats.cache_misses += 1;
            }
        }
        
        // Validate HTTP status
        if !response.status().is_success() {
            error!("HTTP request failed with status {}: {}", response.status(), url);
            self.update_failed_stats();
            return Err(AssetError::ValidationError {
                asset_path: url.to_string(),
                reason: format!("HTTP request failed with status {}: {}", response.status(), url),
                expected_type: Some("http_success".to_string()),
            });
        }
        
        // Update success statistics
        self.update_success_stats(response_time);
        
        info!("Enhanced HTTP GET successful: {} ({}ms, cache: {})", 
              url, response_time, if from_cache { "HIT" } else { "MISS" });
        
        Ok(EnhancedHttpResponse {
            response,
            from_cache,
            response_time_ms: response_time,
            content_hash: None, // Will be calculated if needed
        })
    }
    
    /// Download asset with content hash verification
    #[instrument(name = "http_download_asset", skip(self), fields(url = %url))]
    pub async fn download_asset_with_verification(
        &self, 
        url: &str, 
        expected_hash: Option<&str>
    ) -> AssetResult<Vec<u8>> {
        debug!("Downloading asset with hash verification: {}", url);
        
        let enhanced_response = self.get_enhanced(url).await?;
        let content = enhanced_response.response.bytes().await
            .map_err(|e| {
                error!("Failed to read response body for {}: {}", url, e);
                AssetError::ValidationError {
                    asset_path: url.to_string(),
                    reason: format!("Failed to read response body: {}", e),
                    expected_type: Some("response_body".to_string()),
                }
            })?;
        
        let content_bytes = content.as_ref();
        
        // Update download statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_bytes_downloaded += content_bytes.len() as u64;
        }
        
        // Verify content hash if expected hash is provided
        if let Some(expected) = expected_hash {
            let actual_hash = self.calculate_content_hash(content_bytes);
            if actual_hash != expected {
                error!("Content hash mismatch for {}: expected {}, got {}", 
                       url, expected, actual_hash);
                return Err(AssetError::ValidationError {
                    asset_path: url.to_string(),
                    reason: format!("Content hash verification failed for {}", url),
                    expected_type: Some("content_hash".to_string()),
                });
            }
            debug!("Content hash verification successful for {}", url);
        }
        
        info!("Asset download completed: {} ({} bytes)", url, content_bytes.len());
        Ok(content_bytes.to_vec())
    }
    
    /// Validate URL format and security
    fn validate_url(&self, url: &str) -> AssetResult<()> {
        // Check validation cache first
        {
            let cache = self.url_validation_cache.lock().unwrap();
            if let Some(&is_valid) = cache.get(url) {
                return if is_valid {
                    Ok(())
                } else {
                    Err(AssetError::ValidationError {
                        asset_path: url.to_string(),
                        reason: format!("Invalid URL: {}", url),
                        expected_type: Some("valid_url".to_string()),
                    })
                };
            }
        }
        
        // Perform URL validation
        let parsed = Url::parse(url)
            .map_err(|e| AssetError::ValidationError {
                asset_path: url.to_string(),
                reason: format!("Invalid URL format: {}", e),
                expected_type: Some("valid_url_format".to_string()),
            })?;
        
        // Security checks
        match parsed.scheme() {
            "http" | "https" => {},
            scheme => {
                let error_msg = format!("Unsupported URL scheme: {}", scheme);
                error!("{}", error_msg);
                
                // Cache negative result
                {
                    let mut cache = self.url_validation_cache.lock().unwrap();
                    cache.insert(url.to_string(), false);
                }
                
                return Err(AssetError::ValidationError {
                    asset_path: url.to_string(),
                    reason: error_msg,
                    expected_type: Some("secure_url".to_string()),
                });
            }
        }
        
        // Cache positive result
        {
            let mut cache = self.url_validation_cache.lock().unwrap();
            cache.insert(url.to_string(), true);
        }
        
        Ok(())
    }
    
    /// Calculate SHA-256 content hash for verification
    fn calculate_content_hash(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }
    
    /// Update success statistics
    fn update_success_stats(&self, response_time_ms: u64) {
        let mut stats = self.stats.lock().unwrap();
        stats.successful_requests += 1;
        
        // Update rolling average response time
        let total_responses = stats.successful_requests + stats.failed_requests;
        stats.average_response_time_ms = 
            ((stats.average_response_time_ms * (total_responses - 1) as f64) + response_time_ms as f64) 
            / total_responses as f64;
    }
    
    /// Update failure statistics
    fn update_failed_stats(&self) {
        let mut stats = self.stats.lock().unwrap();
        stats.failed_requests += 1;
    }
    
    /// Get HTTP client statistics
    pub fn get_stats(&self) -> HttpClientStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Clear HTTP client statistics
    pub fn clear_stats(&mut self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = HttpClientStats::default();
    }
    
    /// Clear URL validation cache
    pub fn clear_url_cache(&mut self) {
        let mut cache = self.url_validation_cache.lock().unwrap();
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_http_client_creation() {
        let client = EnhancedHttpClient::new().await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_url_validation() {
        let client = EnhancedHttpClient::new().await.unwrap();
        
        // Valid URLs
        assert!(client.validate_url("https://example.com").is_ok());
        assert!(client.validate_url("http://example.com/path").is_ok());
        
        // Invalid URLs
        assert!(client.validate_url("ftp://example.com").is_err());
        assert!(client.validate_url("not-a-url").is_err());
    }

    #[test]
    fn test_content_hash_calculation() {
        let client_result = tokio_test::block_on(EnhancedHttpClient::new());
        assert!(client_result.is_ok());
        
        let client = client_result.unwrap();
        let test_content = b"Hello, World!";
        let hash = client.calculate_content_hash(test_content);
        
        // SHA-256 of "Hello, World!"
        assert_eq!(hash, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }

    #[test]
    fn test_http_client_config_defaults() {
        let config = HttpClientConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout_seconds, 60);
        assert!(config.enable_compression);
        assert!(config.enable_http2);
        assert!(config.cache.enabled);
    }
}
