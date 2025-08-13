//! CLI entry point for testing Roll20 zip files
//!
//! Usage: cargo run -p ttrpg-cli <path-to-zip-file>

use std::path::Path;
use std::sync::Arc;
use ttrpg_core::{
    manager::DefaultServiceManager,
    // ServiceManager trait used via Arc<dyn ServiceManager>
    types::TargetFormat,
};
use ttrpg_formats::roll20_asset_integration::Roll20AssetPipeline;
// Asset service handled through pipeline

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging will be initialized by the service manager

    // Get command line arguments
    let args: Vec<String> = std::env::args().collect();

    let zip_path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        // Default to looking for test_campaign.zip in current directory
        Path::new("./test_campaign.zip")
    };

    let output_dir = Path::new("./output");

    println!("🎯 Testing TTRPG Converter M2.4 Export System");
    println!("📁 Input:  {zip_path:?}");
    println!("📂 Output: {output_dir:?}");

    // Check if zip file exists
    if !zip_path.exists() {
        eprintln!("❌ Error: Zip file not found at {zip_path:?}");
        eprintln!("📝 Instructions:");
        eprintln!("   1. Export a campaign from Roll20 as a zip file");
        eprintln!("   2. Either:");
        eprintln!("      - Place it as 'test_campaign.zip' in project root, OR");
        eprintln!("      - Run: cargo run -p ttrpg-cli path/to/your/file.zip");
        return Ok(());
    }

    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    // Step 1: Determine input type and get campaign directory
    let campaign_dir = if zip_path.is_file() && zip_path.extension().is_some_and(|ext| ext == "zip")
    {
        println!("\n🔧 Step 1: Processing Roll20 zip file...");
        // For zip files, extract only campaign.json first to validate
        extract_campaign_json_only(zip_path).await?
    } else if zip_path.is_dir() {
        println!("\n🔧 Step 1: Using extracted Roll20 campaign directory...");
        zip_path.to_path_buf()
    } else {
        return Err("Input must be either a .zip file or a directory containing campaign.json"
            .to_string()
            .into());
    };
    let campaign_json = campaign_dir.join("campaign.json");

    if !campaign_json.exists() {
        eprintln!("❌ Error: campaign.json not found in zip file");
        eprintln!("📋 Zip contents:");
        list_zip_contents(zip_path)?;
        return Ok(());
    }

    println!("✅ Found campaign.json");

    // Step 2: Initialize services
    println!("\n🔧 Step 2: Initializing services...");

    // Use service manager as designed - with proper service coordination and dependency injection
    let service_manager = DefaultServiceManager::with_defaults()?;
    let cache_dir = std::env::temp_dir().join("ttrpg_cache");
    let config = ttrpg_assets::prelude::Roll20ProcessorConfig::default();

    let service_manager_arc = Arc::new(service_manager);
    let pipeline = Roll20AssetPipeline::new(cache_dir, config, service_manager_arc).await?;

    println!("✅ Services initialized");

    // Step 3: Process complete pipeline
    println!("\n🔧 Step 3: Processing Roll20 campaign...");
    let start_time = std::time::Instant::now();

    let results = pipeline
        .process_and_export_campaign(
            &campaign_json,
            TargetFormat::JsonExport,
            &output_dir.join("exported_campaign.json"),
            Some(Arc::new(|progress| {
                println!("   Progress: {}%", (progress.overall_progress * 100.0) as u32);
            })),
        )
        .await?;

    let total_time = start_time.elapsed();

    // Step 4: Display results
    println!("\n🎉 PROCESSING COMPLETE!");
    println!("⏱️  Total time: {:.2}s", total_time.as_secs_f64());
    println!();
    println!("📊 Campaign Statistics:");
    println!("   • Actors: {}", results.asset_results.campaign.actors.len());
    println!("   • Scenes: {}", results.asset_results.campaign.scenes.len());
    println!("   • Items:  {}", results.asset_results.campaign.items.len());
    println!();
    println!("📦 Asset Processing:");
    println!("   • Assets discovered: {}", results.asset_results.total_discovered);
    println!("   • Assets processed:  {}", results.asset_results.processed_assets.len());
    println!("   • Assets failed:     {}", results.asset_results.failed_assets.len());
    println!("   • Processing time:   {}ms", results.asset_results.processing_time_ms);
    println!();
    println!("🎯 Export Results:");
    println!("   • Format: {}", results.export_result.target_format);
    println!(
        "   • Success: {}",
        if results.export_result.success {
            "✅ YES"
        } else {
            "❌ NO"
        }
    );
    println!("   • Output size: {} bytes", results.export_result.stats.output_size_bytes);
    println!("   • Export time: {}ms", results.export_result.stats.processing_time_ms);

    println!("\n📁 Output files created in: {output_dir:?}");

    // Cleanup temp directory only if we extracted files
    if campaign_dir
        .file_name()
        .is_some_and(|name| name == "roll20_test")
    {
        let _ = std::fs::remove_dir_all(&campaign_dir);
    }

    Ok(())
}

// Full zip extraction function (kept for future use if needed)
#[allow(dead_code)]
async fn extract_zip_file(
    zip_path: &Path,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir().join("roll20_test");
    std::fs::create_dir_all(&temp_dir)?;

    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
        std::fs::create_dir_all(&temp_dir)?;
    }

    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                let sanitized_path = sanitize_path_for_windows(path);
                temp_dir.join(sanitized_path)
            }
            None => continue,
        };

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(temp_dir)
}

/// Extract only campaign.json from zip file for validation (efficient approach)
async fn extract_campaign_json_only(
    zip_path: &Path,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir().join("roll20_test");
    std::fs::create_dir_all(&temp_dir)?;

    // Clear any existing temp directory
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
        std::fs::create_dir_all(&temp_dir)?;
    }

    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // Find and extract only campaign.json
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name() == "campaign.json" {
            let outpath = temp_dir.join("campaign.json");
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
            println!("   Found and extracted campaign.json");
            break;
        }
    }

    Ok(temp_dir)
}

/// Sanitize file path for Windows compatibility
#[allow(dead_code)]
fn sanitize_path_for_windows(path: &std::path::Path) -> std::path::PathBuf {
    let path_str = path.to_string_lossy();

    // Replace invalid Windows characters: < > : " | ? * \0
    let sanitized: String = path_str
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0' => '_',
            _ => c,
        })
        .collect();

    std::path::PathBuf::from(sanitized)
}

fn list_zip_contents(zip_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        println!("   • {} ({} bytes)", file.name(), file.size());
    }

    Ok(())
}
