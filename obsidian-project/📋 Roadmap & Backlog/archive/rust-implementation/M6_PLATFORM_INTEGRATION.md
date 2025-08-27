# M6: Platform Integration - Junior Developer Implementation Guide

## 🎯 **MILESTONE 6 OVERVIEW**
**Duration**: 4 weeks | **Total Points**: 35 | **Priority**: 🔥 HIGH

Universal platform integration using Processing Plugin Architecture for seamless compatibility across major TTRPG platforms and VTT systems.

### 🌐 **PLATFORM INTEGRATION FEATURES**
- **Multi-VTT Support**: Roll20, Foundry VTT, Fantasy Grounds, Astral Tabletop
- **Mobile Platform APIs**: D&D Beyond, HeroLab Online, Pathbuilder integration
- **Cloud Storage Integration**: Google Drive, Dropbox, OneDrive synchronization
- **OAuth Authentication**: Secure platform authentication flows
- **Platform Detection**: Automatic format detection and conversion suggestions

---

## **T6.1: Universal Platform Abstraction**
**Duration**: 6 days | **Points**: 12 | **Priority**: 🚨 CRITICAL
**Dependencies**: M2.0 Processing Plugin Architecture (AssetRetrievalPlugin + ValidationPlugin)

### **Implementation Steps**

**Step 1: Platform API Dependencies**
Update `ttrpg-processing-plugins/Cargo.toml`:
```toml
[dependencies]
# Universal platform integration
oauth2 = "4.4"                    # OAuth 2.0 authentication flow
reqwest-oauth2 = "0.4"           # OAuth integration with reqwest
async-trait = "0.1"              # Async traits for platform adapters
url = { workspace = true }        # URL parsing and manipulation

# Platform-specific APIs
serde_xml_rs = "0.6"             # XML parsing for Fantasy Grounds
scraper = "0.17"                 # Web scraping for platforms without APIs
base64 = "0.21"                  # Encoding for API authentication
```

**Step 2: Platform Abstraction Layer**
Create `ttrpg-processing-plugins/src/platform/mod.rs`:
```rust
use async_trait::async_trait;
use oauth2::{AuthorizationCode, TokenResponse};

#[async_trait]
pub trait PlatformAdapter {
    async fn authenticate(&self, credentials: &PlatformCredentials) -> Result<AuthToken, PlatformError>;
    async fn fetch_campaign_data(&self, campaign_id: &str, auth: &AuthToken) -> Result<RawCampaignData, PlatformError>;
    async fn upload_campaign(&self, campaign: &Campaign, auth: &AuthToken) -> Result<UploadResult, PlatformError>;
    fn get_platform_info(&self) -> PlatformInfo;
}

pub struct UniversalPlatformManager {
    adapters: HashMap<PlatformType, Box<dyn PlatformAdapter>>,
    asset_retrieval: Arc<AssetRetrievalPlugin>,
    validator: Arc<ValidationPlugin>,
}

impl UniversalPlatformManager {
    pub async fn detect_platform(&self, input_data: &[u8]) -> Result<PlatformType, PlatformError> {
        // Use ValidationPlugin to analyze data structure
        let analysis = self.validator.analyze_data_structure(input_data).await?;
        match analysis.format_signature {
            sig if sig.contains("roll20") => Ok(PlatformType::Roll20),
            sig if sig.contains("foundry") => Ok(PlatformType::FoundryVTT),
            sig if sig.contains("fg") => Ok(PlatformType::FantasyGrounds),
            _ => Err(PlatformError::UnknownFormat),
        }
    }
    
    pub async fn cross_platform_convert(&self, source: PlatformType, target: PlatformType, data: CampaignData) -> Result<CampaignData, PlatformError> {
        // Leverage AssetRetrievalPlugin for platform-specific asset handling
        let processed_assets = self.asset_retrieval.platform_specific_processing(&data.assets, source, target).await?;
        
        // Apply platform-specific transformations
        self.apply_platform_transformations(data, source, target).await
    }
}
```

---

## **T6.2: OAuth Integration & Cloud Storage**
**Duration**: 8 days | **Points**: 15 | **Priority**: 🔥 HIGH
**Dependencies**: T6.1 Complete

### **Implementation Steps**

**Step 1: OAuth Flow Implementation**
```rust
pub struct OAuthManager {
    clients: HashMap<PlatformType, oauth2::Client>,
}

impl OAuthManager {
    pub async fn initiate_auth(&self, platform: PlatformType) -> Result<String, OAuthError> {
        let client = self.clients.get(&platform).ok_or(OAuthError::UnsupportedPlatform)?;
        
        let (auth_url, _csrf_token) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_scope(oauth2::Scope::new("campaign:read".to_string()))
            .add_scope(oauth2::Scope::new("campaign:write".to_string()))
            .url();
            
        Ok(auth_url.to_string())
    }
    
    pub async fn complete_auth(&self, platform: PlatformType, auth_code: &str) -> Result<AuthToken, OAuthError> {
        let client = self.clients.get(&platform).unwrap();
        let token = client
            .exchange_code(AuthorizationCode::new(auth_code.to_string()))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;
            
        Ok(AuthToken::from(token))
    }
}
```

**Step 2: Cloud Storage Integration**
```rust
pub enum CloudProvider {
    GoogleDrive,
    Dropbox,
    OneDrive,
}

pub struct CloudStorageManager {
    oauth_manager: Arc<OAuthManager>,
    asset_retrieval: Arc<AssetRetrievalPlugin>,
}

impl CloudStorageManager {
    pub async fn sync_campaign_to_cloud(&self, campaign: &Campaign, provider: CloudProvider) -> Result<SyncResult, CloudError> {
        // Use AssetRetrievalPlugin for efficient cloud upload
        let auth_token = self.oauth_manager.get_token(provider.into()).await?;
        self.asset_retrieval.upload_assets_to_cloud(&campaign.assets, provider, &auth_token).await
    }
    
    pub async fn auto_backup_campaign(&self, campaign_id: &str) -> Result<BackupResult, CloudError> {
        // Automatic backup to user's configured cloud storage
        for provider in self.get_configured_providers().await? {
            self.sync_campaign_to_cloud(&campaign, provider).await?;
        }
        Ok(BackupResult::Success)
    }
}
```

---

## **T6.3: Mobile Platform APIs**
**Duration**: 6 days | **Points**: 8 | **Priority**: 🟡 MEDIUM
**Dependencies**: T6.2 Complete

### **Implementation Steps**

**Step 1: Mobile Platform Adapters**
```rust
pub struct DnDBeyondAdapter {
    oauth_manager: Arc<OAuthManager>,
    validator: Arc<ValidationPlugin>,
}

impl PlatformAdapter for DnDBeyondAdapter {
    async fn fetch_campaign_data(&self, campaign_id: &str, auth: &AuthToken) -> Result<RawCampaignData, PlatformError> {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("https://www.dndbeyond.com/api/campaigns/{}", campaign_id))
            .bearer_auth(auth.token())
            .send()
            .await?;
            
        let raw_data = response.bytes().await?;
        
        // Use ValidationPlugin to validate D&D Beyond format
        self.validator.validate_platform_data(&raw_data, PlatformType::DnDBeyond).await?;
        
        Ok(RawCampaignData::new(raw_data.to_vec(), PlatformType::DnDBeyond))
    }
}

pub struct PathbuilderAdapter {
    // Similar implementation for Pathbuilder mobile app integration
}
```

### **Success Criteria**
- [ ] ✅ OAuth authentication working for 4+ major platforms
- [ ] ✅ Cross-platform campaign conversion (Roll20 ↔ Foundry ↔ Fantasy Grounds)  
- [ ] ✅ Cloud storage synchronization with Google Drive, Dropbox, OneDrive
- [ ] ✅ Mobile platform integration (D&D Beyond, Pathbuilder)
- [ ] ✅ Automatic platform detection with >95% accuracy
- [ ] ✅ Comprehensive error handling and retry logic for network operations
- [ ] ✅ Performance testing with large campaigns across platforms
