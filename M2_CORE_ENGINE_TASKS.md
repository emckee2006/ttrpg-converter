# M2: Core Engine Tasks - Junior Developer Implementation Guide

## 🎯 **MILESTONE 2 OVERVIEW**
**Duration**: 3 weeks | **Total Points**: 35 | **Priority**: 🔥 HIGH

Core conversion engine with Roll20 parser, entity processing pipeline, and Foundry output generation.

---

## **T2.1: Roll20 JSON Parser Implementation**
**Duration**: 4 days | **Points**: 10 | **Priority**: 🔥 HIGH  
**Dependencies**: M1 Complete

### **Implementation Steps for Junior Developer**

**Step 1: Set Up ttrpg-formats Crate**
```bash
cd crates\ttrpg-formats
```

Update `Cargo.toml`:
```toml
[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
ttrpg-core = { path = "../ttrpg-core" }
zip = { workspace = true }
tracing = { workspace = true }
```

**Step 2: Create Roll20 Entity Structures**
Create `src/roll20/entities.rs`:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Character {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub char_type: String,
    pub attribs: Vec<Roll20Attribute>,
    pub abilities: Vec<Roll20Ability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roll20Page {
    pub id: String,
    pub name: String,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub showgrid: Option<bool>,
}
```

**Step 3: Implement Campaign Parser**
Create `src/roll20/parser.rs`:
```rust
pub struct Roll20Parser {
    strict_mode: bool,
}

impl Roll20Parser {
    pub fn parse_zip<P: AsRef<Path>>(&self, path: P) -> ConversionResult<Roll20Campaign> {
        // ZIP file parsing with campaign.json extraction
    }
}
```

**Acceptance Criteria**:
- [ ] Parse complete Roll20 campaign.json files
- [ ] Handle ZIP file extraction 
- [ ] Support all entity types (characters, pages, graphics, etc.)
- [ ] Parse 50MB campaign in <2 seconds
- [ ] Comprehensive error handling for malformed data

---

## **T2.2: Entity Conversion Pipeline**
**Duration**: 3 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: T2.1 Complete

### **Implementation Steps**

**Step 1: Create Entity Framework in ttrpg-core**
```rust
pub trait Entity {
    fn validate(&self) -> ConversionResult<()>;
    fn convert(&self, context: &ConversionContext) -> ConversionResult<FoundryEntity>;
}

pub struct ConversionPipeline {
    thread_count: usize,
}

impl ConversionPipeline {
    pub fn convert_campaign(&self, campaign: Roll20Campaign) -> ConversionResult<FoundryCampaign> {
        // Parallel processing with rayon
    }
}
```

**Step 2: Implement Parallel Processing**
```bash
cd crates\ttrpg-core
```

Add rayon dependency and implement:
```rust
use rayon::prelude::*;

let converted_characters: Vec<_> = campaign.characters
    .par_iter()
    .map(|c| c.convert(context))
    .collect::<ConversionResult<Vec<_>>>()?;
```

**Acceptance Criteria**:
- [ ] 5-10x faster than Python version
- [ ] Parallel entity processing with rayon
- [ ] Memory-efficient for large campaigns
- [ ] Progress tracking support

---

## **T2.3: Foundry VTT Output Generation**
**Duration**: 4 days | **Points**: 10 | **Priority**: 🔥 HIGH
**Dependencies**: T2.2 Complete

### **Implementation Steps**

**Step 1: Create Foundry Entity Structures**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundryActor {
    pub _id: String,
    pub name: String,
    pub type: String,
    pub system: HashMap<String, Value>,
    pub items: Vec<FoundryItem>,
}
```

**Step 2: Implement Conversion Logic**
Map Roll20 entities to Foundry format:
- Characters → Actors with proper system data
- Pages → Scenes with walls and tokens  
- Graphics → Tokens with vision/lighting
- Handouts → Journal entries

**Step 3: Database Generation**
```rust
pub fn generate_foundry_world(campaign: FoundryCampaign) -> ConversionResult<()> {
    // Create world.json, actors.db, items.db, etc.
}
```

**Acceptance Criteria**:
- [ ] Generate complete Foundry VTT world
- [ ] Support v10, v11, v12 formats
- [ ] Proper entity relationships
- [ ] Asset reference handling

---

## **T2.4: Asset Processing Pipeline**  
**Duration**: 3 days | **Points**: 7 | **Priority**: 🟡 MEDIUM
**Dependencies**: T2.1 Complete (can run in parallel with T2.2/T2.3)

### **Implementation Steps**

**Step 1: Set Up ttrpg-assets Crate**
```toml
[dependencies]
reqwest = { workspace = true }
image = { workspace = true }
tokio = { workspace = true }
```

**Step 2: Implement Asset Downloader**
```rust
pub struct AssetDownloader {
    client: reqwest::Client,
    max_size: u64,
}

impl AssetDownloader {
    pub async fn download_asset(&self, url: &str) -> ConversionResult<Vec<u8>> {
        // HTTP download with size limits and retries
    }
}
```

**Step 3: Image Optimization**
```rust
pub fn optimize_image(data: &[u8]) -> ConversionResult<Vec<u8>> {
    // Resize, compress, format conversion
}
```

**Acceptance Criteria**:
- [ ] Parallel asset downloads
- [ ] Image optimization and resizing
- [ ] Size limits and validation
- [ ] Progress tracking integration
