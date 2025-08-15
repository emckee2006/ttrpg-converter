//! Professional async memory caching using `moka` LRU cache
//!
//! This module provides a comprehensive async memory caching system built on top of
//! the professional `moka` LRU cache, replacing basic HashMap caching with
//! enterprise-grade memory management.
//!
//! # Features
//!
//! - **Async LRU Cache**: Professional `moka` cache with async support
//! - **TTL Support**: Automatic expiration with configurable time-to-live
//! - **Memory Management**: Automatic eviction based on size and age
//! - **Performance Monitoring**: Cache hit/miss statistics and performance metrics
//! - **Thread-Safe**: Async-aware thread-safe operations
//! - **Configurable Policies**: Size limits, TTL, eviction policies
//!
//! # Usage
//!
//! ```rust,no_run
//! use ttrpg_assets::memory_cache::{MemoryCache, CacheConfig};
//!
//! # tokio_test::block_on(async {
//! let cache = MemoryCache::new("test_cache".to_string(), CacheConfig::default()).await?;
//!
//! // Cache asset data
//! let asset_data = b"example asset data".to_vec();
//! cache.insert("asset_123".to_string(), asset_data).await;
//! let cached_asset = cache.get(&"asset_123".to_string()).await;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::{debug, info, instrument};
use ttrpg_core::error::AssetResult;

// ============================================================================
// CACHE CONFIGURATION
// ============================================================================

/// Configuration for professional memory cache
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries in cache
    pub max_capacity: u64,

    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: u64,

    /// Time-to-idle for cache entries in seconds (None = no idle timeout)
    pub tti_seconds: Option<u64>,

    /// Initial cache capacity (for performance optimization)
    pub initial_capacity: Option<usize>,

    /// Number of segments for concurrent access optimization
    pub num_segments: Option<usize>,

    /// Enable cache statistics collection
    pub enable_stats: bool,

    /// Cache cleanup interval in seconds
    pub cleanup_interval_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10000,          // 10,000 entries
            ttl_seconds: 3600,            // 1 hour TTL
            tti_seconds: Some(1800),      // 30 minutes idle timeout
            initial_capacity: Some(1000), // Start with 1,000 entries
            num_segments: Some(16),       // 16 segments for concurrency
            enable_stats: true,
            cleanup_interval_seconds: 300, // 5 minutes cleanup
        }
    }
}

// ============================================================================
// CACHE STATISTICS
// ============================================================================

/// Cache performance statistics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total cache requests
    pub total_requests: u64,

    /// Cache hits
    pub hits: u64,

    /// Cache misses
    pub misses: u64,

    /// Current number of entries
    pub entry_count: u64,

    /// Total entries inserted
    pub total_insertions: u64,

    /// Total entries evicted
    pub total_evictions: u64,

    /// Total entries expired by TTL
    pub total_expirations: u64,

    /// Cache hit ratio (0.0 - 1.0)
    pub hit_ratio: f64,

    /// Average access time in microseconds
    pub average_access_time_us: f64,

    /// Memory usage estimate in bytes
    pub estimated_memory_bytes: u64,
}

impl CacheStats {
    /// Calculate hit ratio
    pub fn calculate_hit_ratio(&mut self) {
        if self.total_requests > 0 {
            self.hit_ratio = self.hits as f64 / self.total_requests as f64;
        } else {
            self.hit_ratio = 0.0;
        }
    }

    /// Update with access time
    pub fn update_access_time(&mut self, access_time_us: u64) {
        let total_accesses = self.hits + self.misses;
        if total_accesses > 0 {
            self.average_access_time_us = ((self.average_access_time_us
                * (total_accesses - 1) as f64)
                + access_time_us as f64)
                / total_accesses as f64;
        } else {
            self.average_access_time_us = access_time_us as f64;
        }
    }
}

// ============================================================================
// PROFESSIONAL MEMORY CACHE IMPLEMENTATION
// ============================================================================

/// Professional async memory cache with comprehensive features
#[derive(Debug)]
pub struct MemoryCache<K, V>
where
    K: Hash + Eq + Send + Sync + Clone + 'static,
    V: Send + Sync + Clone + 'static,
{
    /// Moka async cache instance
    cache: Cache<K, V>,

    /// Cache configuration
    config: CacheConfig,

    /// Cache statistics (if enabled)
    stats: Option<Arc<Mutex<CacheStats>>>,

    /// Cache name for logging
    name: String,
}

impl<K, V> MemoryCache<K, V>
where
    K: Hash + Eq + Send + Sync + Clone + std::fmt::Debug + 'static,
    V: Send + Sync + Clone + 'static,
{
    /// Create a new professional memory cache
    #[instrument(name = "memory_cache_new", skip(config))]
    pub async fn new(name: String, config: CacheConfig) -> AssetResult<Self> {
        info!(
            "Creating professional memory cache '{}' with {} max capacity",
            name, config.max_capacity
        );

        // Build moka cache with configuration
        let mut cache_builder = Cache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(Duration::from_secs(config.ttl_seconds));

        // Add time-to-idle if configured
        if let Some(tti_seconds) = config.tti_seconds {
            cache_builder = cache_builder.time_to_idle(Duration::from_secs(tti_seconds));
        }

        // Set initial capacity for performance
        if let Some(initial_capacity) = config.initial_capacity {
            cache_builder = cache_builder.initial_capacity(initial_capacity);
        }

        // Configure segments for concurrent access
        if let Some(_num_segments) = config.num_segments {
            // Note: moka doesn't expose segment configuration directly,
            // but uses optimal defaults for concurrent access
        }

        let cache = cache_builder.build();

        // Initialize statistics if enabled
        let stats = if config.enable_stats {
            Some(Arc::new(Mutex::new(CacheStats::default())))
        } else {
            None
        };

        let memory_cache = Self { cache, config: config.clone(), stats, name: name.clone() };

        // Start background cleanup task
        if config.cleanup_interval_seconds > 0 {
            memory_cache.start_cleanup_task().await?;
        }

        info!("Professional memory cache '{}' initialized successfully", name);
        Ok(memory_cache)
    }

    /// Create a cache with default configuration
    pub async fn with_default_config(name: String) -> AssetResult<Self> {
        Self::new(name, CacheConfig::default()).await
    }

    /// Insert a value into the cache
    #[instrument(name = "cache_insert", skip(self, key, value), fields(cache = %self.name))]
    pub async fn insert(&self, key: K, value: V) {
        let start_time = SystemTime::now();

        debug!("Inserting entry into cache '{}'", self.name);
        self.cache.insert(key, value).await;

        // Update statistics
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.total_insertions += 1;
            stats.entry_count = self.cache.entry_count();

            if let Ok(elapsed) = start_time.elapsed() {
                stats.update_access_time(elapsed.as_micros() as u64);
            }
        }
    }

    /// Get a value from the cache
    #[instrument(name = "cache_get", skip(self, key), fields(cache = %self.name))]
    pub async fn get(&self, key: &K) -> Option<V> {
        let start_time = SystemTime::now();

        debug!("Getting entry from cache '{}'", self.name);
        let result = self.cache.get(key).await;

        // Update statistics
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.total_requests += 1;

            if result.is_some() {
                stats.hits += 1;
                debug!("Cache hit in '{}'", self.name);
            } else {
                stats.misses += 1;
                debug!("Cache miss in '{}'", self.name);
            }

            stats.calculate_hit_ratio();

            if let Ok(elapsed) = start_time.elapsed() {
                stats.update_access_time(elapsed.as_micros() as u64);
            }
        }

        result
    }

    /// Get a value or compute it if missing
    #[instrument(name = "cache_get_or_insert_with", skip(self, key, f), fields(cache = %self.name))]
    pub async fn get_with<F>(&self, key: K, f: F) -> V
    where
        F: std::future::Future<Output = V>,
    {
        let start_time = SystemTime::now();

        debug!("Getting or computing entry in cache '{}'", self.name);
        let result = self.cache.get_with(key, f).await;

        // Update statistics (this is a more complex case)
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.total_requests += 1;
            // Note: moka doesn't tell us if this was a hit or compute,
            // so we'll count it as a hit for simplicity
            stats.hits += 1;
            stats.calculate_hit_ratio();

            if let Ok(elapsed) = start_time.elapsed() {
                stats.update_access_time(elapsed.as_micros() as u64);
            }
        }

        result
    }

    /// Remove a value from the cache
    #[instrument(name = "cache_remove", skip(self, key), fields(cache = %self.name))]
    pub async fn remove(&self, key: &K) -> Option<V> {
        debug!("Removing entry from cache '{}'", self.name);
        let result = self.cache.remove(key).await;

        // Update statistics
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.entry_count = self.cache.entry_count();
        }

        result
    }

    /// Check if cache contains a key
    #[instrument(name = "cache_contains", skip(self, key), fields(cache = %self.name))]
    pub async fn contains_key(&self, key: &K) -> bool {
        self.cache.contains_key(key)
    }

    /// Clear all entries from the cache
    #[instrument(name = "cache_clear", skip(self), fields(cache = %self.name))]
    pub async fn clear(&self) {
        info!("Clearing all entries from cache '{}'", self.name);
        self.cache.invalidate_all();

        // Reset statistics
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.entry_count = 0;
        }
    }

    /// Get current number of entries
    pub fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }

    /// Sync cache operations for testing consistency
    /// This method ensures cache operations are fully processed before returning
    pub async fn sync(&self) {
        // Force moka cache to run its internal maintenance tasks
        // This ensures entry counts are accurate for testing
        self.cache.run_pending_tasks().await;
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> Option<CacheStats> {
        self.stats.as_ref().map(|stats| {
            let mut stats = stats.lock().unwrap();
            stats.entry_count = self.cache.entry_count();
            stats.calculate_hit_ratio();
            stats.clone()
        })
    }

    /// Clear cache statistics
    pub fn clear_stats(&self) {
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            *stats = CacheStats::default();
        }
    }

    /// Force run eviction to remove expired entries
    #[instrument(name = "cache_evict_expired", skip(self), fields(cache = %self.name))]
    pub async fn evict_expired(&self) {
        debug!("Running eviction for cache '{}'", self.name);
        self.cache.run_pending_tasks().await;

        // Update eviction statistics
        if let Some(stats) = &self.stats {
            let mut stats = stats.lock().unwrap();
            stats.entry_count = self.cache.entry_count();
            // Note: moka doesn't provide detailed eviction counts
        }
    }

    /// Start background cleanup task
    async fn start_cleanup_task(&self) -> AssetResult<()> {
        let cache_clone = self.cache.clone();
        let interval = self.config.cleanup_interval_seconds;
        let cache_name = self.name.clone();

        tokio::spawn(async move {
            let mut cleanup_interval = tokio::time::interval(Duration::from_secs(interval));

            loop {
                cleanup_interval.tick().await;
                debug!("Running cleanup for cache '{}'", cache_name);
                cache_clone.run_pending_tasks().await;
            }
        });

        debug!("Started cleanup task for cache '{}' with {} second interval", self.name, interval);
        Ok(())
    }
}

// ============================================================================
// SPECIALIZED CACHE TYPES
// ============================================================================

/// Asset cache for storing downloaded assets
pub type AssetCache = MemoryCache<String, Vec<u8>>;

/// Validation result cache
pub type ValidationCache = MemoryCache<String, bool>;

/// Metadata cache for asset information
pub type MetadataCache = MemoryCache<String, HashMap<String, String>>;

/// URL validation cache
pub type UrlValidationCache = MemoryCache<String, bool>;

// ============================================================================
// CACHE MANAGER
// ============================================================================

/// Central cache manager for coordinating multiple caches
#[derive(Debug)]
pub struct CacheManager {
    /// Asset content cache
    pub assets: AssetCache,

    /// Validation results cache
    pub validations: ValidationCache,

    /// Metadata cache
    pub metadata: MetadataCache,

    /// URL validation cache
    pub url_validations: UrlValidationCache,
}

impl CacheManager {
    /// Create a new cache manager with default configurations
    pub async fn new() -> AssetResult<Self> {
        info!("Initializing professional cache manager with multiple specialized caches");

        let assets = AssetCache::with_default_config("assets".to_string()).await?;
        let validations = ValidationCache::with_default_config("validations".to_string()).await?;
        let metadata = MetadataCache::with_default_config("metadata".to_string()).await?;
        let url_validations =
            UrlValidationCache::with_default_config("url_validations".to_string()).await?;

        info!("Professional cache manager initialized successfully");

        Ok(Self { assets, validations, metadata, url_validations })
    }

    /// Create cache manager with custom configurations
    pub async fn with_configs(
        asset_config: CacheConfig,
        validation_config: CacheConfig,
        metadata_config: CacheConfig,
        url_validation_config: CacheConfig,
    ) -> AssetResult<Self> {
        let assets = AssetCache::new("assets".to_string(), asset_config).await?;
        let validations =
            ValidationCache::new("validations".to_string(), validation_config).await?;
        let metadata = MetadataCache::new("metadata".to_string(), metadata_config).await?;
        let url_validations =
            UrlValidationCache::new("url_validations".to_string(), url_validation_config).await?;

        Ok(Self { assets, validations, metadata, url_validations })
    }

    /// Get comprehensive cache statistics
    pub fn get_all_stats(&self) -> HashMap<String, CacheStats> {
        let mut all_stats = HashMap::new();

        if let Some(stats) = self.assets.get_stats() {
            all_stats.insert("assets".to_string(), stats);
        }

        if let Some(stats) = self.validations.get_stats() {
            all_stats.insert("validations".to_string(), stats);
        }

        if let Some(stats) = self.metadata.get_stats() {
            all_stats.insert("metadata".to_string(), stats);
        }

        if let Some(stats) = self.url_validations.get_stats() {
            all_stats.insert("url_validations".to_string(), stats);
        }

        all_stats
    }

    /// Clear all caches
    pub async fn clear_all(&self) {
        info!("Clearing all managed caches");

        self.assets.clear().await;
        self.validations.clear().await;
        self.metadata.clear().await;
        self.url_validations.clear().await;

        info!("All managed caches cleared successfully");
    }

    /// Run eviction on all caches
    pub async fn evict_all_expired(&self) {
        debug!("Running eviction on all managed caches");

        self.assets.evict_expired().await;
        self.validations.evict_expired().await;
        self.metadata.evict_expired().await;
        self.url_validations.evict_expired().await;

        debug!("Eviction completed on all managed caches");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_cache_creation() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_cache_insert_and_get() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.sync().await; // Ensure cache operations are synchronized
        let result = cache.get(&"key1".to_string()).await;

        assert_eq!(result, Some("value1".to_string()));
        assert_eq!(cache.entry_count(), 1);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        let result = cache.get(&"nonexistent".to_string()).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_cache_remove() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        cache.insert("key1".to_string(), "value1".to_string()).await;
        let removed = cache.remove(&"key1".to_string()).await;
        let after_removal = cache.get(&"key1".to_string()).await;

        assert_eq!(removed, Some("value1".to_string()));
        assert_eq!(after_removal, None);
        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.insert("key2".to_string(), "value2".to_string()).await;
        cache.sync().await; // Ensure cache operations are synchronized

        assert_eq!(cache.entry_count(), 2);

        cache.clear().await;
        cache.sync().await; // Ensure clear operation is synchronized
        assert_eq!(cache.entry_count(), 0);
    }

    #[tokio::test]
    async fn test_cache_statistics() {
        let cache: MemoryCache<String, String> =
            MemoryCache::with_default_config("test".to_string())
                .await
                .unwrap();

        // Test cache miss
        cache.get(&"nonexistent".to_string()).await;

        // Test cache hit
        cache.insert("key1".to_string(), "value1".to_string()).await;
        cache.get(&"key1".to_string()).await;

        let stats = cache.get_stats().unwrap();
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_ratio - 0.5).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_cache_manager() {
        let manager = CacheManager::new().await.unwrap();

        // Test asset cache
        manager
            .assets
            .insert("asset1".to_string(), vec![1, 2, 3])
            .await;
        let asset = manager.assets.get(&"asset1".to_string()).await;
        assert_eq!(asset, Some(vec![1, 2, 3]));

        // Test validation cache
        manager.validations.insert("valid1".to_string(), true).await;
        let validation = manager.validations.get(&"valid1".to_string()).await;
        assert_eq!(validation, Some(true));

        // Test clear all
        manager.clear_all().await;
        assert_eq!(manager.assets.entry_count(), 0);
        assert_eq!(manager.validations.entry_count(), 0);
    }
}
