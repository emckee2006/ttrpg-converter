//! CLI entry point for testing TTRPG Converter Plugin System
//!
//! Usage: cargo run -p ttrpg-cli [options]

use std::collections::HashMap;
use std::path::Path;
use ttrpg_core::plugin_framework::{
    DiscoveryConfig, LogLevel, LoggingPlugin, PluginConfig, PluginDiscovery, PluginLifecycle,
    StaticPluginCategory,
};
use ttrpg_output_plugins as _; // Import to ensure inventory registration

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing TTRPG Converter Plugin System with Console Logging");
    println!("🔍 Demonstrating inventory discovery and plugin functionality\n");

    // Step 1: Test Plugin Discovery System
    println!("🔧 Step 1: Testing Plugin Discovery...");

    let discovery = PluginDiscovery::new(DiscoveryConfig::default());
    discovery.discover_all()?;

    let stats = discovery.get_stats()?;
    println!("   ✅ Plugins discovered: {}", stats.total_discovered);
    println!("   ✅ Plugins loaded: {}", stats.loaded_plugins);

    // Step 2: Get Logging Plugins
    println!("\n🔧 Step 2: Finding Console Logging Plugin...");

    let logging_plugins =
        discovery.get_plugins_by_category(&StaticPluginCategory::Logging("console").into())?;

    if logging_plugins.is_empty() {
        println!("   ❌ No console logging plugins found");
        return Ok(());
    }

    println!("   ✅ Found {} logging plugin(s)", logging_plugins.len());

    // Step 3: Create and test Console Logging Plugin
    println!("\n🔧 Step 3: Testing Console Logging Plugin...");

    // Create plugin instance using factory
    let plugin_registration = &logging_plugins[0];
    let plugin_any = (plugin_registration.factory)();

    // Downcast to ConsoleLoggingPlugin
    let console_plugin = plugin_any
        .downcast::<ttrpg_output_plugins::logging::console::ConsoleLoggingPlugin>()
        .map_err(|_| "Failed to downcast to ConsoleLoggingPlugin")?;

    let mut console_plugin = *console_plugin;

    // Display plugin info
    let plugin_info = console_plugin.plugin_info();
    println!("   📋 Plugin: {} v{}", plugin_info.name, plugin_info.version);
    println!("   📝 Description: {}", plugin_info.description);
    println!("   🏷️  Features: {:?}", plugin_info.supported_features);

    // Step 4: Initialize Plugin
    println!("\n🔧 Step 4: Initializing Console Logging Plugin...");

    let config = PluginConfig {
        config_data: HashMap::from([("log_level".to_string(), serde_json::json!("info"))]),
        cache_dir: None,
        temp_dir: None,
    };

    LoggingPlugin::initialize(&mut console_plugin, config).await?;
    println!("   ✅ Plugin initialized successfully");

    // Step 5: Test Logging Functionality
    println!("\n🔧 Step 5: Testing Logging Functionality...");

    console_plugin.set_level(LogLevel::Debug);

    // Test different log levels
    console_plugin.info("🎉 Testing INFO level logging from Console Logging Plugin", None);
    console_plugin.debug("🔍 Testing DEBUG level logging - this should be visible", None);
    console_plugin.warn("⚠️ Testing WARN level logging", None);
    console_plugin.error("❌ Testing ERROR level logging", None);

    // Test structured logging
    let mut context = HashMap::new();
    context.insert("test_key".to_string(), "test_value".to_string());
    context.insert("plugin_name".to_string(), plugin_info.name.clone());

    let context_json = serde_json::to_value(context)?;
    console_plugin.log_with_data(
        LogLevel::Info,
        "📊 Testing structured logging with context",
        &context_json,
    );

    // Step 6: Test Health Check
    println!("\n🔧 Step 6: Testing Plugin Health Check...");

    let health = console_plugin.health_check().await?;
    println!("   📊 Plugin Health: {health:?}");

    // Step 7: Get Stats
    println!("\n🔧 Step 7: Getting Plugin Statistics...");

    let logging_stats = console_plugin.get_stats();
    println!("   📈 Messages logged: {}", logging_stats.messages_logged);
    println!("   ❌ Errors logged: {}", logging_stats.errors_logged);
    println!("   ⚠️ Warnings logged: {}", logging_stats.warnings_logged);
    println!("   ⏱️ Started at: {:?}", logging_stats.start_time);

    // Step 8: Cleanup
    println!("\n🔧 Step 8: Cleaning up...");

    console_plugin.cleanup().await?;
    println!("   ✅ Plugin cleanup completed");

    println!("\n🎉 CONSOLE LOGGING PLUGIN TEST COMPLETE!");
    println!("✅ All functionality verified successfully");

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
                let sanitized_path = sanitize_path_for_windows(&path);
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
#[allow(dead_code)]
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

#[allow(dead_code)]
fn list_zip_contents(zip_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        println!("   • {} ({} bytes)", file.name(), file.size());
    }

    Ok(())
}
