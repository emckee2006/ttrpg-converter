//! Test script for processing real Roll20 zip files
//! 
//! Usage: 
//! 1. Place a Roll20 export zip file in the project root
//! 2. Update the ZIP_PATH below to point to your file
//! 3. Run: cargo run --example test_real_roll20

use std::path::Path;
use ttrpg_core::{
    manager::DefaultServiceManager,
    services::ServiceManager,
    types::TargetFormat,
};
use ttrpg_formats::roll20_asset_integration::Roll20AssetPipeline;
use ttrpg_assets::prelude::RustAssetService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // ===========================================
    // UPDATE THESE PATHS FOR YOUR TESTING:
    // ===========================================
    let zip_path = Path::new("./test_campaign.zip");  // <- Put your Roll20 zip file here
    let output_dir = Path::new("./output");            // <- Output directory
    
    println!("🎯 Testing TTRPG Converter M2.4 Export System");
    println!("📁 Input:  {:?}", zip_path);
    println!("📂 Output: {:?}", output_dir);
    
    // Check if zip file exists
    if !zip_path.exists() {
        eprintln!("❌ Error: Zip file not found at {:?}", zip_path);
        eprintln!("📝 Instructions:");
        eprintln!("   1. Export a campaign from Roll20 as a zip file");
        eprintln!("   2. Place it in the project root as 'test_campaign.zip'");
        eprintln!("   3. Run this example again");
        return Ok(());
    }
    
    // Create output directory
    std::fs::create_dir_all(output_dir)?;
    
    // Step 1: Extract zip and find campaign.json
    println!("\n🔧 Step 1: Extracting Roll20 zip file...");
    let temp_dir = extract_zip_file(zip_path).await?;
    let campaign_json = temp_dir.join("campaign.json");
    
    if !campaign_json.exists() {
        eprintln!("❌ Error: campaign.json not found in zip file");
        return Ok(());
    }
    
    println!("✅ Found campaign.json at: {:?}", campaign_json);
    
    // Step 2: Initialize services
    println!("\n🔧 Step 2: Initializing services...");
    let mut service_manager = DefaultServiceManager::new()?;
    service_manager.init_defaults()?;
    
    let asset_service = RustAssetService::new();
    let pipeline = Roll20AssetPipeline::new(
        std::sync::Arc::new(asset_service),
        Some(service_manager.logging()),
    )?;
    
    println!("✅ Services initialized");
    
    // Step 3: Process complete pipeline
    println!("\n🔧 Step 3: Processing Roll20 campaign...");
    let start_time = std::time::Instant::now();
    
    let results = pipeline
        .process_and_export_campaign(
            &campaign_json,
            TargetFormat::JsonExport,
            &output_dir.join("exported_campaign.json"),
            Some(Box::new(|progress| {
                println!("   Progress: {}%", (progress * 100.0) as u32);
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
    println!("   • Success: {}", if results.export_result.success { "✅ YES" } else { "❌ NO" });
    println!("   • Output size: {} bytes", results.export_result.stats.output_size_bytes);
    println!("   • Export time: {}ms", results.export_result.stats.processing_time_ms);
    
    println!("\n📁 Output files created in: {:?}", output_dir);
    
    // Cleanup temp directory
    let _ = std::fs::remove_dir_all(temp_dir);
    
    Ok(())
}

async fn extract_zip_file(zip_path: &Path) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    use std::io::Read;
    
    let temp_dir = std::env::temp_dir().join("roll20_test");
    std::fs::create_dir_all(&temp_dir)?;
    
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => temp_dir.join(path),
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
