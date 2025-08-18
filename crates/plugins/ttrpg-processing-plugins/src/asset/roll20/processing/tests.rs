//! Tests for Roll20 asset processing functionality

use super::*;
use serde_json::json;
// Remove unused import
use tempfile::TempDir;

#[cfg(test)]
mod roll20_asset_processor_tests {
    use super::*;

    #[tokio::test]
    async fn test_processor_creation() {
        let temp_dir = TempDir::new().unwrap();
        let processor = Roll20AssetProcessor::with_defaults(temp_dir.path().to_path_buf()).await;
        assert!(processor.is_ok());
    }

    #[tokio::test]
    async fn test_asset_discovery_empty_campaign() {
        let temp_dir = TempDir::new().unwrap();
        let processor = Roll20AssetProcessor::with_defaults(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        let empty_campaign = json!({});
        let assets = processor.discover_assets(&empty_campaign).await.unwrap();
        assert_eq!(assets.len(), 0);
    }

    #[tokio::test]
    async fn test_asset_discovery_with_assets() {
        let temp_dir = TempDir::new().unwrap();
        let processor = Roll20AssetProcessor::with_defaults(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        let campaign_with_assets = json!({
            "assets": [
                {
                    "id": "test_asset_1",
                    "name": "Test Avatar",
                    "url": "https://example.com/avatar.png",
                    "asset_type": "character",
                    "size": 1024
                }
            ]
        });

        let assets = processor
            .discover_assets(&campaign_with_assets)
            .await
            .unwrap();
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].id, "test_asset_1");
        assert_eq!(assets[0].category, Roll20AssetCategory::CharacterAvatar);
    }

    #[test]
    fn test_roll20_asset_category_display() {
        assert_eq!(Roll20AssetCategory::CharacterAvatar.to_string(), "Character Avatar");
        assert_eq!(Roll20AssetCategory::SceneBackground.to_string(), "Scene Background");
        assert_eq!(Roll20AssetCategory::TokenImage.to_string(), "Token Image");
        assert_eq!(Roll20AssetCategory::HandoutAsset.to_string(), "Handout Asset");
        assert_eq!(Roll20AssetCategory::AudioFile.to_string(), "Audio File");
        assert_eq!(Roll20AssetCategory::Other.to_string(), "Other");
    }

    #[test]
    fn test_roll20_processor_config_default() {
        let config = Roll20ProcessorConfig::default();
        assert_eq!(config.max_concurrent_downloads, 8);
        assert_eq!(config.download_timeout_secs, 30);
        assert_eq!(config.max_retries, 3);
        assert!(config.optimize_images);
        assert_eq!(config.target_image_format, Some("webp".to_string()));
        assert_eq!(config.max_image_resolution, Some((2048, 2048)));
        assert_eq!(config.image_quality, 85);
    }

    #[tokio::test]
    async fn test_processing_stats() {
        let temp_dir = TempDir::new().unwrap();
        let processor = Roll20AssetProcessor::with_defaults(temp_dir.path().to_path_buf())
            .await
            .unwrap();

        let stats = processor.get_processing_stats().await;
        assert_eq!(stats.assets_discovered, 0);
        assert_eq!(stats.assets_categorized, 0);
        assert_eq!(stats.bulk_operations, 0);
    }
}

#[cfg(test)]
mod roll20_asset_categorization_tests {
    use super::*;

    async fn create_test_processor() -> Roll20AssetProcessor {
        let temp_dir = TempDir::new().unwrap();
        Roll20AssetProcessor::with_defaults(temp_dir.path().to_path_buf())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_categorize_character_assets() {
        let processor = create_test_processor().await;

        let category =
            processor.categorize_roll20_asset("character", "https://example.com/avatar.png");
        assert_eq!(category, Roll20AssetCategory::CharacterAvatar);

        let category =
            processor.categorize_roll20_asset("avatar", "https://example.com/portrait.jpg");
        assert_eq!(category, Roll20AssetCategory::CharacterAvatar);
    }

    #[tokio::test]
    async fn test_categorize_scene_assets() {
        let processor = create_test_processor().await;

        let category = processor.categorize_roll20_asset("page", "https://example.com/map.png");
        assert_eq!(category, Roll20AssetCategory::SceneBackground);

        let category =
            processor.categorize_roll20_asset("background", "https://example.com/scene.jpg");
        assert_eq!(category, Roll20AssetCategory::SceneBackground);
    }

    #[tokio::test]
    async fn test_categorize_token_assets() {
        let processor = create_test_processor().await;

        let category = processor.categorize_roll20_asset("token", "https://example.com/token.png");
        assert_eq!(category, Roll20AssetCategory::TokenImage);

        let category =
            processor.categorize_roll20_asset("other", "https://example.com/token_sprite.png");
        assert_eq!(category, Roll20AssetCategory::TokenImage);
    }

    #[tokio::test]
    async fn test_categorize_audio_assets() {
        let processor = create_test_processor().await;

        let category = processor.categorize_roll20_asset("audio", "https://example.com/music.mp3");
        assert_eq!(category, Roll20AssetCategory::AudioFile);

        let category = processor.categorize_roll20_asset("music", "https://example.com/bgm.ogg");
        assert_eq!(category, Roll20AssetCategory::AudioFile);
    }

    #[tokio::test]
    async fn test_determine_asset_type() {
        let processor = create_test_processor().await;

        let asset_type =
            processor.determine_base_asset_type("https://example.com/image.png", "image");
        assert_eq!(asset_type, AssetType::Image);

        let asset_type =
            processor.determine_base_asset_type("https://example.com/sound.mp3", "audio");
        assert_eq!(asset_type, AssetType::Audio);

        let asset_type =
            processor.determine_base_asset_type("https://example.com/doc.pdf", "document");
        assert_eq!(asset_type, AssetType::Document);

        let asset_type =
            processor.determine_base_asset_type("https://example.com/unknown.xyz", "other");
        assert_eq!(asset_type, AssetType::Attachment);
    }

    #[tokio::test]
    async fn test_asset_priority_assignment() {
        let processor = create_test_processor().await;

        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::CharacterAvatar), 1);
        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::TokenImage), 2);
        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::SceneBackground), 3);
        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::HandoutAsset), 4);
        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::AudioFile), 5);
        assert_eq!(processor.determine_asset_priority(Roll20AssetCategory::Other), 6);
    }
}
