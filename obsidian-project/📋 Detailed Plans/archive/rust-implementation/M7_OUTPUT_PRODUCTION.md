# M7: Output + Production - Junior Developer Implementation Guide

## 🎯 **MILESTONE 7 OVERVIEW**
**Duration**: 4 weeks | **Total Points**: 35 | **Priority**: 🚨 CRITICAL

Advanced output formats, production deployment, and comprehensive testing framework - completing the TTRPGConverter for production use.

### 📦 **OUTPUT & PRODUCTION FEATURES**
- **Advanced Output Formats**: PDF generation, interactive web exports, mobile formats
- **Production Infrastructure**: Docker containerization, CI/CD pipeline, monitoring
- **Comprehensive Testing**: End-to-end test suite, performance benchmarks
- **Documentation**: API docs, user guides, deployment guides

---

## **T7.1: Advanced Output Formats**
**Duration**: 8 days | **Points**: 15 | **Priority**: 🔥 HIGH
**Dependencies**: M2.0 Processing Plugin Architecture (AssetConversionPlugin + ValidationPlugin)

### **Implementation Steps**

**Step 1: PDF Generation Dependencies**
Update `ttrpg-processing-plugins/Cargo.toml`:
```toml
[dependencies]
# Advanced PDF generation
printpdf = "0.6"                 # Professional PDF creation
lopdf = "0.32"                   # PDF manipulation and optimization
pdf-writer = "0.8"               # Low-level PDF writing
resvg = "0.35"                   # SVG to PDF conversion

# Web export formats
handlebars = "4.4"               # Template engine for HTML exports
minify-html = "0.11"             # HTML optimization
sass-rs = "0.2"                  # CSS preprocessing

# Mobile formats
zip = "0.6"                      # Archive creation for mobile packages
image = { workspace = true }      # Image optimization for mobile
```

**Step 2: Enhanced Output Plugin System**
```rust
impl AssetConversionPlugin {
    pub async fn generate_pdf_campaign(&self, campaign: &Campaign, template: &PdfTemplate) -> Result<Vec<u8>, ConversionError> {
        use printpdf::*;
        
        let (doc, page1, layer1) = PdfDocument::new(&campaign.name, Mm(210.0), Mm(297.0), "Layer 1");
        
        // Use ValidationPlugin to ensure PDF standards compliance
        self.validation_plugin.validate_pdf_structure(&doc).await?;
        
        // Parallel asset processing for PDF inclusion
        let processed_assets = self.parallel_process_assets_for_pdf(&campaign.assets).await?;
        
        // Generate PDF with proper asset embedding
        self.embed_assets_in_pdf(doc, processed_assets).await
    }
    
    pub async fn generate_interactive_web_export(&self, campaign: &Campaign) -> Result<WebExportPackage, ConversionError> {
        // Generate interactive HTML/JS/CSS package
        let template_engine = handlebars::Handlebars::new();
        let optimized_assets = self.optimize_assets_for_web(&campaign.assets).await?;
        
        WebExportPackage::new()
            .with_html(template_engine.render("campaign", &campaign)?)
            .with_assets(optimized_assets)
            .with_interactive_features()
            .build()
    }
    
    pub async fn generate_mobile_package(&self, campaign: &Campaign, platform: MobilePlatform) -> Result<MobilePackage, ConversionError> {
        match platform {
            MobilePlatform::FoundryMobile => self.generate_foundry_mobile_package(campaign).await,
            MobilePlatform::Roll20Mobile => self.generate_roll20_mobile_package(campaign).await,
            MobilePlatform::Generic => self.generate_generic_mobile_package(campaign).await,
        }
    }
}
```

---

## **T7.2: Production Infrastructure**
**Duration**: 10 days | **Points**: 12 | **Priority**: 🚨 CRITICAL
**Dependencies**: T7.1 Complete

### **Implementation Steps**

**Step 1: Docker Containerization**
Create `Dockerfile`:
```dockerfile
FROM rust:1.75-slim as builder

# Install system dependencies for complex processing
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libopencv-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build optimized release binary
RUN cargo build --release --features production

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libopencv-core4.5d \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ttrpg-converter /usr/local/bin/
COPY --from=builder /app/templates /app/templates

EXPOSE 8080
CMD ["ttrpg-converter", "serve", "--host", "0.0.0.0", "--port", "8080"]
```

**Step 2: CI/CD Pipeline**
Create `.github/workflows/production-deploy.yml`:
```yaml
name: Production Deploy

on:
  push:
    branches: [ main ]
    tags: [ 'v*' ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      
      - name: Run comprehensive test suite
        run: |
          cargo test --all-features
          cargo bench -- --test
          
  security-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Security audit
        run: |
          cargo install cargo-audit
          cargo audit
          
  docker-build:
    needs: [test, security-audit]
    runs-on: ubuntu-latest
    steps:
      - name: Build and push Docker image
        uses: docker/build-push-action@v5
        with:
          push: true
          tags: |
            ttrpgconverter/converter:latest
            ttrpgconverter/converter:${{ github.sha }}
```

**Step 3: Monitoring & Observability**
```rust
use tracing::{info, error, instrument};
use metrics::{counter, histogram, gauge};

impl ProductionMetrics {
    #[instrument]
    pub async fn track_conversion_metrics(&self, conversion: &ConversionResult) {
        counter!("conversions_total", 1, "status" => conversion.status.to_string());
        histogram!("conversion_duration_seconds", conversion.duration.as_secs_f64());
        gauge!("active_conversions", self.get_active_conversions() as f64);
        
        if conversion.status.is_error() {
            error!(
                error = %conversion.error,
                campaign_size = conversion.campaign_size,
                "Conversion failed"
            );
        } else {
            info!(
                campaign_size = conversion.campaign_size,
                duration_ms = conversion.duration.as_millis(),
                "Conversion completed successfully"
            );
        }
    }
}
```

---

## **T7.3: Comprehensive Testing Framework**
**Duration**: 6 days | **Points**: 8 | **Priority**: 🔥 HIGH
**Dependencies**: T7.2 Complete

### **Implementation Steps**

**Step 1: End-to-End Test Suite**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_full_conversion_pipeline() {
        let test_campaign = load_test_campaign("complex_roll20_campaign.json");
        
        // Test full Processing Plugin Architecture pipeline
        let mut coordinator = AssetProcessingCoordinator::new().await;
        
        let result = coordinator
            .process_campaign(test_campaign)
            .await
            .expect("Full pipeline should succeed");
            
        // Validate all processing steps completed successfully
        assert!(result.validation_passed);
        assert!(!result.processed_assets.is_empty());
        assert!(result.conversion_successful);
        
        // Performance assertions
        assert!(result.processing_time < Duration::from_secs(30));
        assert!(result.memory_usage_mb < 1000);
    }
    
    #[tokio::test]
    async fn test_platform_integration_e2e() {
        // Test complete platform integration workflow
        let platform_manager = UniversalPlatformManager::new().await;
        
        // Test Roll20 → Foundry conversion
        let result = platform_manager
            .cross_platform_convert(PlatformType::Roll20, PlatformType::FoundryVTT, test_campaign)
            .await
            .expect("Cross-platform conversion should succeed");
            
        assert!(result.assets_converted);
        assert!(result.validation_passed);
    }
}
```

**Step 2: Performance Benchmarking**
```rust
#[cfg(test)]
mod benchmarks {
    use criterion::{criterion_group, criterion_main, Criterion};
    
    fn benchmark_processing_plugins(c: &mut Criterion) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        c.bench_function("asset_retrieval_plugin", |b| {
            b.to_async(&rt).iter(|| async {
                let plugin = AssetRetrievalPlugin::new().await;
                plugin.process_assets(black_box(large_asset_set())).await
            });
        });
        
        c.bench_function("validation_plugin", |b| {
            b.to_async(&rt).iter(|| async {
                let plugin = ValidationPlugin::new().await;
                plugin.validate_campaign(black_box(complex_campaign())).await
            });
        });
        
        c.bench_function("full_pipeline_benchmark", |b| {
            b.to_async(&rt).iter(|| async {
                let coordinator = AssetProcessingCoordinator::new().await;
                coordinator.process_campaign(black_box(large_campaign())).await
            });
        });
    }
    
    criterion_group!(benches, benchmark_processing_plugins);
    criterion_main!(benches);
}
```

### **Success Criteria**
- [ ] ✅ PDF generation with embedded assets and proper formatting
- [ ] ✅ Interactive web exports with full campaign functionality  
- [ ] ✅ Docker containerization with optimized image (<500MB)
- [ ] ✅ CI/CD pipeline with automated testing and deployment
- [ ] ✅ Comprehensive monitoring and observability
- [ ] ✅ End-to-end test coverage >90% with performance benchmarks
- [ ] ✅ Production deployment ready with security audit passed
- [ ] ✅ Complete API documentation and user guides
