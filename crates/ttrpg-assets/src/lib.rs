//! Asset processing and management
//!
//! This crate provides comprehensive asset management capabilities including:
//! - High-performance asset downloading and caching via `RustAssetService`
//! - Image, audio, and document processing
//! - Cache management with size and age limits
//! - HTTP client integration for remote asset retrieval
//!
//! # Usage
//!
//! ```rust,no_run
//! use ttrpg_assets::service::RustAssetService;
//! use std::path::PathBuf;
//!
//! # tokio_test::block_on(async {
//! let cache_dir = PathBuf::from("./asset_cache");
//! let service = RustAssetService::new(cache_dir, None).await?;
//!
//! // Download and cache an asset
//! let cached_path = service.download_asset("https://example.com/token.png", false).await?;
//! println!("Asset cached at: {:?}", cached_path);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # });
//! ```

pub mod http_client;
pub mod memory_cache;
pub mod service;

pub mod prelude {
    //! Common imports for this crate
    pub use crate::http_client::{EnhancedHttpClient, HttpClientConfig};
    pub use crate::memory_cache::{CacheConfig, CacheManager, MemoryCache};
    pub use crate::service::{AssetType, RustAssetService};
}

// Re-export main service for easy access
pub use service::{AssetType, RustAssetService};
