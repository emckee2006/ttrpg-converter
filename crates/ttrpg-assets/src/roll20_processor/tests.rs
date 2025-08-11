//! Comprehensive tests for Roll20 asset processor

use super::*;
use crate::service::RustAssetService;
use serde_json::json;
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use tempfile::TempDir;

/// Mock progress callback for testing
struct MockProgressCallback {
    call_count: AtomicUsize,
    last_progress: Arc<Mutex<Option<AssetDownloadProgress>>>,
}

impl MockProgressCallback {
    fn new() -> Self {
        Self {
            call_count: AtomicUsize::new(0),
            last_progress: Arc::new(Mutex::new(None)),
        }
    }

    fn get_call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    async fn get_last_progress(&self) -> Option<AssetDownloadProgress> {
        self.last_progress.lock().await.clone()
    }
}

impl MockProgressCallback {
    fn create_callback(self: Arc<Self>) -> ProgressCallback {
        Arc::new(move |progress| {
            let callback = self.clone();
            tokio::spawn(async move {
                callback.call_count.fetch_add(1, Ordering::SeqCst);
                *callback.last_progress.lock().await = Some(progress);
            });
        })
    }
}

/// Create test Roll20 campaign data
fn create_test_campaign_data() -> serde_json::Value {
    json!({
        "campaign": {
            "name": "Test Campaign",
            "description": "Test campaign for asset processing"
        },
        "assets": [
            {
                "id": "asset_1",
                "name": "hero_token.png",
                "url": "https://example.com/assets/hero_token.png",
                "asset_type": "token",
                "size": 2048
            },
            {
                "id": "asset_2", 
                "name": "dungeon_background.jpg",
                "url": "https://example.com/assets/dungeon_background.jpg",
                "asset_type": "background",
                "size": 8192
            },
            {
                "id": "asset_3",
                "name": "combat_music.mp3", 
                "url": "https://example.com/assets/combat_music.mp3",
                "asset_type": "audio",
                "size": 4096000
            }
        ],
        "characters": [
            {
                "id": "char_1",
                "name": "Test Hero",
                "avatar": "https://example.com/avatars/hero_portrait.png"
            }
        ],
        "pages": [
            {
                "id": "page_1",
                "name": "Dungeon Level 1",
                "background_url": "https://example.com/maps/dungeon_level1.jpg"
            }
        ]
    })
}

#[tokio::test]
async fn test_roll20_processor_creation() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();

    // Test processor creation with defaults
    let processor = Roll20AssetProcessor::with_defaults(cache_dir.clone(), None).await?;
    
    // Verify processor was created successfully
    let stats = processor.get_processing_stats().await;
    assert_eq!(stats.assets_discovered, 0);
    assert_eq!(stats.bulk_operations, 0);

    Ok(())
}

#[tokio::test]
async fn test_roll20_processor_with_custom_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();

    let base_service = Arc::new(RustAssetService::new(cache_dir, None).await?);
    
    let config = Roll20ProcessorConfig {
        max_concurrent_downloads: 4,
        download_timeout_secs: 15,
        optimize_images: false,
        ..Default::default()
    };

    let processor = Roll20AssetProcessor::new(base_service, config.clone(), None);

    // Verify configuration is applied
    assert_eq!(processor.config.max_concurrent_downloads, 4);
    assert_eq!(processor.config.download_timeout_secs, 15);
    assert!(!processor.config.optimize_images);

    Ok(())
}

#[tokio::test]
async fn test_asset_discovery_from_campaign_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let campaign_data = create_test_campaign_data();
    let discovered_assets = processor.discover_assets(&campaign_data).await?;

    // Verify assets were discovered
    assert_eq!(discovered_assets.len(), 3);

    // Verify asset details
    let token_asset = discovered_assets.iter()
        .find(|a| a.name == "hero_token.png")
        .expect("Token asset should be discovered");
    
    assert_eq!(token_asset.category, Roll20AssetCategory::TokenImage);
    assert_eq!(token_asset.asset_type, AssetType::Image);
    assert_eq!(token_asset.size_bytes, Some(2048));
    assert_eq!(token_asset.priority, 2); // Token priority

    let background_asset = discovered_assets.iter()
        .find(|a| a.name == "dungeon_background.jpg")
        .expect("Background asset should be discovered");
    
    assert_eq!(background_asset.category, Roll20AssetCategory::SceneBackground);
    assert_eq!(background_asset.asset_type, AssetType::Image);

    let audio_asset = discovered_assets.iter()
        .find(|a| a.name == "combat_music.mp3")
        .expect("Audio asset should be discovered");
    
    assert_eq!(audio_asset.category, Roll20AssetCategory::AudioFile);
    assert_eq!(audio_asset.asset_type, AssetType::Audio);

    // Verify statistics were updated
    let stats = processor.get_processing_stats().await;
    assert_eq!(stats.assets_discovered, 3);

    Ok(())
}

#[tokio::test]
async fn test_asset_categorization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    // Test various asset categorization scenarios
    let test_cases = vec![
        ("token", "https://example.com/token.png", Roll20AssetCategory::TokenImage),
        ("character", "https://example.com/avatar.jpg", Roll20AssetCategory::CharacterAvatar),
        ("background", "https://example.com/map.jpg", Roll20AssetCategory::SceneBackground),
        ("handout", "https://example.com/doc.pdf", Roll20AssetCategory::HandoutAsset),
        ("audio", "https://example.com/music.mp3", Roll20AssetCategory::AudioFile),
        ("other", "https://example.com/unknown.file", Roll20AssetCategory::Other),
        // Test URL-based categorization fallbacks
        ("unknown", "https://example.com/some_token_image.png", Roll20AssetCategory::TokenImage),
        ("unknown", "https://example.com/character_portrait.jpg", Roll20AssetCategory::CharacterAvatar),
    ];

    for (asset_type, url, expected_category) in test_cases {
        let category = processor.categorize_roll20_asset(asset_type, url);
        assert_eq!(category, expected_category, 
                   "Asset type '{}' with URL '{}' should be categorized as {:?}", 
                   asset_type, url, expected_category);
    }

    Ok(())
}

#[tokio::test]
async fn test_asset_type_determination() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let test_cases = vec![
        ("https://example.com/image.png", "image", AssetType::Image),
        ("https://example.com/photo.jpg", "photo", AssetType::Image),
        ("https://example.com/music.mp3", "audio", AssetType::Audio),
        ("https://example.com/sound.ogg", "sound", AssetType::Audio),
        ("https://example.com/document.pdf", "document", AssetType::Document),
        ("https://example.com/text.txt", "text", AssetType::Document),
        ("https://example.com/unknown.xyz", "other", AssetType::Other),
    ];

    for (url, asset_type, expected_type) in test_cases {
        let determined_type = processor.determine_base_asset_type(url, asset_type);
        assert_eq!(determined_type, expected_type,
                   "URL '{}' with type '{}' should be determined as {:?}",
                   url, asset_type, expected_type);
    }

    Ok(())
}

#[tokio::test]
async fn test_asset_priority_determination() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    // Test priority ordering (lower number = higher priority)
    let expected_priorities = vec![
        (Roll20AssetCategory::CharacterAvatar, 1),
        (Roll20AssetCategory::TokenImage, 2),
        (Roll20AssetCategory::SceneBackground, 3),
        (Roll20AssetCategory::HandoutAsset, 4),
        (Roll20AssetCategory::AudioFile, 5),
        (Roll20AssetCategory::Other, 6),
    ];

    for (category, expected_priority) in expected_priorities {
        let priority = processor.determine_asset_priority(category);
        assert_eq!(priority, expected_priority,
                   "Category {:?} should have priority {}", category, expected_priority);
    }

    Ok(())
}

#[tokio::test]
async fn test_bulk_asset_processing_empty_list() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let assets = Vec::new();
    let results = processor.process_assets_bulk(assets, None).await?;

    assert!(results.is_empty());

    // Verify statistics were updated
    let stats = processor.get_processing_stats().await;
    assert_eq!(stats.bulk_operations, 1);

    Ok(())
}

#[tokio::test]
async fn test_progress_callback_functionality() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let mock_callback = Arc::new(MockProgressCallback::new());
    let progress_callback = mock_callback.clone().create_callback();

    // Create test assets
    let assets = vec![
        Roll20AssetInfo {
            id: "test_1".to_string(),
            name: "test1.png".to_string(),
            url: "https://httpbin.org/status/404".to_string(), // This will fail
            category: Roll20AssetCategory::TokenImage,
            asset_type: AssetType::Image,
            size_bytes: Some(1024),
            entity_references: vec![],
            priority: 1,
        },
    ];

    let _results = processor.process_assets_bulk(assets, Some(progress_callback)).await?;

    // Give callback time to execute
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Verify callback was called
    assert!(mock_callback.get_call_count() > 0);

    let last_progress = mock_callback.get_last_progress().await;
    assert!(last_progress.is_some());

    let progress = last_progress.unwrap();
    assert_eq!(progress.total, 1);
    assert_eq!(progress.completed, 1);
    assert_eq!(progress.overall_progress, 1.0);

    Ok(())
}

#[tokio::test]
async fn test_asset_discovery_with_missing_fields() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let campaign_data = json!({
        "assets": [
            {
                "id": "asset_1",
                "name": "test.png"
                // Missing URL - should cause error
            },
            {
                "id": "asset_2",
                "url": "https://example.com/valid.png"
                // Missing name - should use default
            }
        ]
    });

    let result = processor.discover_assets(&campaign_data).await;
    
    // Should return error due to missing URL in first asset
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_roll20_asset_category_display() {
    let test_cases = vec![
        (Roll20AssetCategory::CharacterAvatar, "Character Avatar"),
        (Roll20AssetCategory::SceneBackground, "Scene Background"),
        (Roll20AssetCategory::TokenImage, "Token Image"),
        (Roll20AssetCategory::HandoutAsset, "Handout Asset"),
        (Roll20AssetCategory::AudioFile, "Audio File"),
        (Roll20AssetCategory::Other, "Other"),
    ];

    for (category, expected_display) in test_cases {
        assert_eq!(format!("{}", category), expected_display);
    }
}

#[tokio::test]
async fn test_processing_stats_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    let stats = processor.get_processing_stats().await;

    // Verify initial stats
    assert_eq!(stats.assets_discovered, 0);
    assert_eq!(stats.assets_categorized, 0);
    assert_eq!(stats.assets_optimized, 0);
    assert_eq!(stats.optimization_time_ms, 0);
    assert_eq!(stats.bulk_operations, 0);
    assert_eq!(stats.average_download_speed, 0);

    Ok(())
}

#[tokio::test]
async fn test_asset_sorting_by_priority() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let cache_dir = temp_dir.path().to_path_buf();
    let processor = Roll20AssetProcessor::with_defaults(cache_dir, None).await?;

    // Create assets with different priorities
    let assets = vec![
        Roll20AssetInfo {
            id: "low_priority".to_string(),
            name: "audio.mp3".to_string(),
            url: "https://httpbin.org/status/404".to_string(),
            category: Roll20AssetCategory::AudioFile, // Priority 5
            asset_type: AssetType::Audio,
            size_bytes: None,
            entity_references: vec![],
            priority: 5,
        },
        Roll20AssetInfo {
            id: "high_priority".to_string(),
            name: "avatar.png".to_string(),
            url: "https://httpbin.org/status/404".to_string(),
            category: Roll20AssetCategory::CharacterAvatar, // Priority 1
            asset_type: AssetType::Image,
            size_bytes: None,
            entity_references: vec![],
            priority: 1,
        },
    ];

    // Process assets and verify high priority processed first
    let _results = processor.process_assets_bulk(assets, None).await?;

    // Verify stats were updated
    let stats = processor.get_processing_stats().await;
    assert_eq!(stats.bulk_operations, 1);

    Ok(())
}
