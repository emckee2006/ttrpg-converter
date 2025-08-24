using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Caching.Memory;
using System.Diagnostics;
using TTRPGConverter.Core.Schemas;
using TTRPGConverter.Core.Services;
using TTRPGConverter.CLI.Commands;

Console.WriteLine("TTRPGConverter CLI");

if (args.Length == 0)
{
    ShowUsage();
    return 1;
}

var command = args[0].ToLowerInvariant();
var commandArgs = args.Skip(1).ToArray();

// Handle commands
return command switch
{
    "generate" => await RunGenerateCommand(args[1..]),
    "test-campaign" => await RunTestCampaignCommand(args[1..]),
    "test-compendium" => await RunTestCompendiumCommand(args[1..]),
    "convert-campaign" => await RunConvertCampaignCommand(args[1..]),
    // "convert" => await ConvertCommand.RunAsync(args[1..]), // Temporarily disabled for model generation
    // "convert-world" => await ConvertWorldCommandRunner.RunAsync(args[1..]), // Temporarily disabled for model generation
    _ => ShowUsage()
};

static int ShowUsage()
{
    Console.WriteLine();
    Console.WriteLine("Usage: TTRPGConverter <command> [options]");
    Console.WriteLine();
    Console.WriteLine("Commands:");
    Console.WriteLine("  generate         Generate C# models from JSON schemas");
    Console.WriteLine("  test-campaign    Test Roll20 campaign ZIP loading and parsing");
    Console.WriteLine("  test-compendium  Test Foundry VTT compendium reading (NeDB/LevelDB via CLI/PlutroniumJSON)");
    Console.WriteLine("  convert-campaign Convert Roll20 campaign ZIP to Foundry VTT world");
    Console.WriteLine("  convert          Convert between TTRPG formats");
    Console.WriteLine("  convert-world    Convert Roll20 campaign ZIP to Foundry world");
    Console.WriteLine("  help             Show this help message");
    Console.WriteLine();
    Console.WriteLine("Test-Compendium Options:");
    Console.WriteLine("  --dry-run, -n         Show commands that would be executed without running them");
    Console.WriteLine("  --verbose, -v         Enable verbose logging (shows debug output)");
    Console.WriteLine("  --module <name>       Test only the specified module or system");
    Console.WriteLine("                        Example: --module statblock-library");
    Console.WriteLine("  --foundry-base-dir    Override Foundry data directory path");
    Console.WriteLine();
    Console.WriteLine("Generate Options:");
    Console.WriteLine("  --schema-path <path>    Path to schema directory (default: schemas)");
    Console.WriteLine("  --output-path <path>    Output directory (default: src/TTRPGConverter.Core/Models)");
    Console.WriteLine("  --namespace <ns>        Base namespace (default: TTRPGConverter.Core.Models)");
    Console.WriteLine();
    Console.WriteLine("Convert Usage:");
    Console.WriteLine("  convert <input-file> <output-file> <source-format> <target-format>");
    Console.WriteLine();
    Console.WriteLine("Examples:");
    Console.WriteLine("  TTRPGConverter generate --schema-path schemas --output-path Models");
    Console.WriteLine("  TTRPGConverter convert character.json foundry-char.json roll20 foundry-pf2e");
    Console.WriteLine("  TTRPGConverter convert-world campaign.zip ./foundry-worlds dnd5e");
    return 0;
}

static async Task<int> RunTestCompendiumCommand(string[] args)
{
    // Parse command line arguments
    var dryRun = args.Contains("--dry-run") || args.Contains("-n");
    var verbose = args.Contains("--verbose") || args.Contains("-v");
    
    // Parse --module argument
    string? specificModule = null;
    string? foundryBaseDirOverride = null;
    for (int i = 0; i < args.Length - 1; i++)
    {
        if (args[i] == "--module")
        {
            specificModule = args[i + 1];
        }
        else if (args[i] == "--foundry-base-dir")
        {
            foundryBaseDirOverride = args[i + 1];
        }
    }
    
    // Configure logging based on verbose flag
    var logLevel = verbose ? LogLevel.Debug : LogLevel.Warning;
    using var loggerFactory = LoggerFactory.Create(builder =>
    {
        builder.AddConsole().SetMinimumLevel(logLevel);
    });
    var logger = loggerFactory.CreateLogger<CompendiumManager>();
    var cache = new MemoryCache(new MemoryCacheOptions());

    if (dryRun)
    {
        Console.WriteLine("🔍 DRY RUN: Testing CompendiumReader command generation (no actual execution)...\n");
    }
    else
    {
        Console.WriteLine("🔍 Testing CompendiumReader with D&D 5e priority deduplication...\n");
    }

    var compendiumManager = new CompendiumManager(logger, cache, verbose);
    
    // Get Foundry data path from multiple sources in priority order:
    // 1. Command line override
    // 2. Environment variable FOUNDRY_DATA_PATH
    // 3. FoundryVTT CLI configuration file
    // 4. Default system location
    var foundryDataPath = !string.IsNullOrEmpty(foundryBaseDirOverride) 
        ? foundryBaseDirOverride 
        : GetFoundryDataPath();

    Console.WriteLine($"📁 Using Foundry data path: {foundryDataPath}");
    
    if (!Directory.Exists(foundryDataPath))
    {
        Console.WriteLine($"❌ Foundry data directory not found: {foundryDataPath}");
        Console.WriteLine("💡 Options to fix this:");
        Console.WriteLine("   - Set FOUNDRY_DATA_PATH environment variable");
        Console.WriteLine("   - Configure FoundryVTT CLI: fvtt configure set dataPath <path>");
        Console.WriteLine("   - Ensure FoundryVTT is installed in default location");
        return 1; // Exit with error code
    }

    // One-time FoundryVTT CLI configuration setup
    await EnsureFoundryCliConfigured();

    // Multi-system priority order paths (D&D 5e, PF2e, PF1e)
    var basePaths = new[]
    {
        // D&D 5e official content (highest priority)
        Path.Combine(foundryDataPath, "modules", "dnd-dungeon-masters-guide"),
        Path.Combine(foundryDataPath, "modules", "dnd-players-handbook"), 
        Path.Combine(foundryDataPath, "systems", "dnd5e"),
        
        // PF2e official content
        Path.Combine(foundryDataPath, "systems", "pf2e"),
        
        // PF1e official content and recommended modules
        Path.Combine(foundryDataPath, "systems", "pf1"),
        Path.Combine(foundryDataPath, "modules", "pf-content"),
        Path.Combine(foundryDataPath, "modules", "pf1-statblock-converter"),
        Path.Combine(foundryDataPath, "modules", "statblock-library"),
    };
    
    // If specific module requested, filter to just that module/system
    if (!string.IsNullOrEmpty(specificModule))
    {
        var moduleDir = Path.Combine(foundryDataPath, "modules", specificModule);
        var systemDir = Path.Combine(foundryDataPath, "systems", specificModule);
        
        if (Directory.Exists(moduleDir))
        {
            basePaths = new[] { moduleDir };
            Console.WriteLine($"🎯 Testing specific module: {specificModule}");
        }
        else if (Directory.Exists(systemDir))
        {
            basePaths = new[] { systemDir };
            Console.WriteLine($"🎯 Testing specific system: {specificModule}");
        }
        else
        {
            Console.WriteLine($"❌ Module/system not found: {specificModule}");
            Console.WriteLine($"   Checked: {moduleDir}");
            Console.WriteLine($"   Checked: {systemDir}");
            return 1;
        }
    }
    
    // Discover Battlezoo and other multi-system modules dynamically
    var modulesDir = Path.Combine(foundryDataPath, "modules");
    var battlezooModules = new List<string>();
    var otherModules = new List<string>();
    
    // Only do dynamic discovery if no specific module requested
    if (string.IsNullOrEmpty(specificModule) && Directory.Exists(modulesDir))
    {
        foreach (var moduleDir in Directory.GetDirectories(modulesDir))
        {
            var moduleName = Path.GetFileName(moduleDir);
            
            // Check for Battlezoo modules
            if (moduleName.StartsWith("battlezoo", StringComparison.OrdinalIgnoreCase))
            {
                battlezooModules.Add(moduleDir);
            }
            // Add other known multi-system modules
            else if (moduleName.Equals("plutonium", StringComparison.OrdinalIgnoreCase))
            {
                otherModules.Add(moduleDir);
            }
        }
    }
    
    // Combine all paths in priority order
    var compendiumPaths = basePaths
        .Concat(battlezooModules.OrderBy(p => Path.GetFileName(p))) // Alphabetical order for Battlezoo
        .Concat(otherModules)
        .ToArray();

    Console.WriteLine($"🔍 Discovered {battlezooModules.Count} Battlezoo modules:");
    foreach (var module in battlezooModules)
    {
        Console.WriteLine($"   - {Path.GetFileName(module)}");
    }
    Console.WriteLine();

    try
    {
        Console.WriteLine("📚 Loading unified compendium cache with multi-system priority order:");
        Console.WriteLine("   1. D&D 5e Official Content (DMG, PHB, System)");
        Console.WriteLine("   2. PF2e Official System");
        Console.WriteLine("   3. PF1e Official System + Recommended Modules");
        Console.WriteLine("   4. Multi-System Modules (Battlezoo, etc.)");
        Console.WriteLine("   5. Third-Party Content (Plutonium)");
        Console.WriteLine("   Format priority within each source: LevelDB (via CLI) > NeDB");
        Console.WriteLine("   System filtering: Includes D&D 5e, PF2e, and PF1e packs only\n");

        await compendiumManager.LoadUnifiedCacheAsync(compendiumPaths, isDryRun: dryRun);
        var unifiedCache = compendiumManager.GetAllEntities();
        
        Console.WriteLine($"✅ Successfully loaded {unifiedCache.Count} unique entities!");
        Console.WriteLine();
        
        // Show statistics by system type
        var systemStats = unifiedCache.Values
            .GroupBy(x => DetermineSystemFromSource(x.SourceFile, compendiumManager))
            .OrderByDescending(g => g.Count());
            
        Console.WriteLine("🎲 Content by game system:");
        foreach (var systemGroup in systemStats)
        {
            Console.WriteLine($"   {systemGroup.Key}: {systemGroup.Count()} items");
            
            // Show top entity types for this system
            var topTypes = systemGroup
                .GroupBy(x => x.Type)
                .OrderByDescending(g => g.Count())
                .Take(3);
            
            foreach (var typeGroup in topTypes)
            {
                Console.WriteLine($"      {typeGroup.Key}: {typeGroup.Count()}");
            }
        }
        Console.WriteLine();
        
        // Show overall statistics by entity type
        var entityStats = unifiedCache.Values
            .GroupBy(x => x.Type)
            .OrderByDescending(g => g.Count())
            .Take(10);
            
        Console.WriteLine("📊 Top entity types (all systems):");
        foreach (var typeGroup in entityStats)
        {
            Console.WriteLine($"   {typeGroup.Key}: {typeGroup.Count()} items");
        }
        Console.WriteLine();
        
        // Show sample items from different sources
        var sourceStats = unifiedCache.Values
            .GroupBy(x => x.SourceFormat)
            .OrderByDescending(g => g.Count());
            
        Console.WriteLine("🗂️ Items by source format:");
        foreach (var formatGroup in sourceStats)
        {
            var sampleItem = formatGroup.First();
            Console.WriteLine($"   {formatGroup.Key}: {formatGroup.Count()} items");
            Console.WriteLine($"      Sample: {sampleItem.Name} ({sampleItem.Type})");
        }
        Console.WriteLine();
        
        // Test entity lookup
        Console.WriteLine("🔍 Testing entity lookup:");
        var testEntities = new[] { "Fireball", "Fighter", "Longsword", "Goblin" };
        
        foreach (var entityName in testEntities)
        {
            var found = compendiumManager.FindEntity("spell", entityName, 0.8) ??
                       compendiumManager.FindEntity("class", entityName, 0.8) ??
                       compendiumManager.FindEntity("weapon", entityName, 0.8) ??
                       compendiumManager.FindEntity("npc", entityName, 0.8);
                       
            if (found != null)
            {
                Console.WriteLine($"   ✅ Found '{entityName}': {found.Name} ({found.Type}) from {found.SourceFormat}");
            }
            else
            {
                Console.WriteLine($"   ❌ '{entityName}' not found");
            }
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"❌ Error loading unified cache: {ex.Message}");
    }
    
    Console.WriteLine("✅ CompendiumReader testing complete!");
    Console.WriteLine("This validates our abstraction layer works with real Foundry VTT data.");
    return 0;
}

/// <summary>
/// Determines the game system from the source file path and manifest information
/// </summary>
static string DetermineSystemFromSource(string sourceFile, CompendiumManager? compendiumManager = null)
{
    if (string.IsNullOrEmpty(sourceFile))
        return "Unknown";
        
    // First, try to get system from manifest detection if available
    if (compendiumManager != null)
    {
        // Special handling for statblock-library module (packs start with "sb-")
        if (sourceFile.StartsWith("sb-", StringComparison.OrdinalIgnoreCase))
        {
            var statblockSystem = compendiumManager.GetDetectedSystem("statblock-library");
            if (!string.IsNullOrEmpty(statblockSystem))
            {
                return statblockSystem;
            }
        }
        
        // Direct lookup by source name
        var sourceName = Path.GetFileName(sourceFile);
        var detectedSystem = compendiumManager.GetDetectedSystem(sourceName);
        if (!string.IsNullOrEmpty(detectedSystem))
        {
            return detectedSystem;
        }
        
        // Try to match against all stored systems
        foreach (var kvp in compendiumManager.GetAllDetectedSystems())
        {
            var moduleKey = kvp.Key;
            var systemName = kvp.Value;
            
            // Check if the source file is related to this module
            if (sourceFile.Contains(moduleKey, StringComparison.OrdinalIgnoreCase))
            {
                return systemName;
            }
        }
    }
        
    var sourceLower = sourceFile.ToLowerInvariant();
    
    // Check for PF2e system
    if (sourceLower.Contains("pf2e") || sourceLower.Contains("pathfinder-2e") || 
        sourceLower.Contains("pathfinder2e"))
        return "Pathfinder 2e";
        
    // Check for PF1e system  
    if (sourceLower.Contains("pf1") || sourceLower.Contains("pathfinder") ||
        sourceLower.Contains("pf-content") || sourceLower.Contains("pf1-statblock"))
        return "Pathfinder 1e";
        
    // Check for D&D 5e system (default for dnd5e, system, and most modules)
    if (sourceLower.Contains("dnd5e") || sourceLower.Contains("system") ||
        sourceLower.Contains("dungeon-masters-guide") || sourceLower.Contains("players-handbook") ||
        sourceLower.Contains("monsters-of-the-multiverse") || sourceLower.Contains("basic-rules") ||
        sourceLower.Contains("xanathars") || sourceLower.Contains("tashas") || sourceLower.Contains("volos"))
        return "D&D 5e";
        
    // Default to D&D 5e for unknown sources (most FoundryVTT content is D&D 5e)
    return "D&D 5e";
}

/// <summary>
/// Ensures FoundryVTT CLI is configured with the correct install path (one-time setup)
/// </summary>
static async Task EnsureFoundryCliConfigured()
{
    try
    {
        var fvttPath = ".\\node_modules\\.bin\\fvtt.cmd";
        
        if (!File.Exists(fvttPath))
        {
            Console.WriteLine("⚠️  FoundryVTT CLI not found - pack extraction may not work");
            return;
        }

        // Set FoundryVTT install path (only needs to be done once)
        var processInfo = new ProcessStartInfo
        {
            FileName = fvttPath,
            Arguments = "configure set installPath \"C:/Program Files/Foundry Virtual Tabletop\"",
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };

        using var process = Process.Start(processInfo);
        if (process != null)
        {
            await process.WaitForExitAsync();
            if (process.ExitCode == 0)
            {
                Console.WriteLine("✅ FoundryVTT CLI configured successfully");
            }
            else
            {
                Console.WriteLine("⚠️  FoundryVTT CLI configuration may have failed");
            }
        }
    }
    catch (Exception ex)
    {
        Console.WriteLine($"⚠️  Could not configure FoundryVTT CLI: {ex.Message}");
    }
}


static async Task<int> RunGenerateCommand(string[] args)
{
    var builder = Host.CreateApplicationBuilder();
    builder.Services.AddLogging(configure => 
        configure.AddConsole().SetMinimumLevel(LogLevel.Debug));
    builder.Services.AddSingleton<SchemaGenerator>();
    builder.Services.AddScoped<GenerateModelsCommand>();
    builder.Services.AddScoped<TestCampaignCommand>();
    builder.Services.AddScoped<ConvertCampaignCommand>();
    var host = builder.Build();
    var generateModelsCommand = host.Services.GetRequiredService<GenerateModelsCommand>();

    // Parse named arguments
    var schemaPath = "schemas";
    var outputPath = "src/TTRPGConverter.Core/Models";
    var baseNamespace = "TTRPGConverter.Core.Models";

    for (int i = 0; i < args.Length; i++)
    {
        if (args[i] == "--schema-path" && i + 1 < args.Length)
            schemaPath = args[i + 1];
        else if (args[i] == "--output-path" && i + 1 < args.Length)
            outputPath = args[i + 1];
        else if (args[i] == "--namespace" && i + 1 < args.Length)
            baseNamespace = args[i + 1];
    }

    await generateModelsCommand.ExecuteAsync(schemaPath, outputPath, baseNamespace);
    return 0;
}

static async Task<int> RunTestCampaignCommand(string[] args)
{
    if (args.Length == 0)
    {
        Console.WriteLine("Error: ZIP file path required");
        Console.WriteLine("Usage: TTRPGConverter test-campaign <zip-path>");
        return 1;
    }

    var zipPath = args[0];
    if (!File.Exists(zipPath))
    {
        Console.WriteLine($"Error: File not found: {zipPath}");
        return 1;
    }

    var builder = Host.CreateApplicationBuilder();
    builder.Services.AddLogging(configure => configure.AddConsole().SetMinimumLevel(LogLevel.Information));
    builder.Services.AddSingleton<Roll20CampaignService>();
    builder.Services.AddSingleton<TestCampaignCommand>();

    var host = builder.Build();
    var testCommand = host.Services.GetRequiredService<TestCampaignCommand>();

    await testCommand.ExecuteAsync(zipPath);
    return 0;
}

static async Task<int> RunConvertCampaignCommand(string[] args)
{
    if (args.Length < 2)
    {
        Console.WriteLine("Error: Campaign ZIP and output directory required");
        Console.WriteLine("Usage: TTRPGConverter convert-campaign <zip-path> <output-dir> [--system dnd5e] [--name WorldName]");
        return 1;
    }

    var zipPath = args[0];
    var outputDir = args[1];
    var system = "dnd5e";
    string? worldName = null;

    // Parse options
    for (int i = 2; i < args.Length; i++)
    {
        if (args[i] == "--system" && i + 1 < args.Length)
            system = args[i + 1];
        else if (args[i] == "--name" && i + 1 < args.Length)
            worldName = args[i + 1];
    }

    if (!File.Exists(zipPath))
    {
        Console.WriteLine($"Error: File not found: {zipPath}");
        return 1;
    }

    var builder = Host.CreateApplicationBuilder();
    builder.Services.AddLogging(configure => configure.AddConsole().SetMinimumLevel(LogLevel.Information));
    builder.Services.AddSingleton<Roll20CampaignService>();
    builder.Services.AddSingleton<FoundryWorldBuilder>();
    builder.Services.AddSingleton<ConvertCampaignCommand>();

    var host = builder.Build();
    var convertCommand = host.Services.GetRequiredService<ConvertCampaignCommand>();

    await convertCommand.ExecuteAsync(zipPath, outputDir, system, worldName);
    return 0;
}

/// <summary>
/// Gets the FoundryVTT data path from multiple sources in priority order:
/// 1. FOUNDRY_DATA_PATH environment variable
/// 2. FoundryVTT CLI configuration file (.fvttrc.yml)
/// 3. Default system location
/// </summary>
static string GetFoundryDataPath()
{
    // 1. Check environment variable first
    var envPath = Environment.GetEnvironmentVariable("FOUNDRY_DATA_PATH");
    if (!string.IsNullOrEmpty(envPath) && Directory.Exists(envPath))
        return envPath;

    // 2. Try FoundryVTT CLI configuration
    var configPath = TryGetFoundryConfigPath();
    if (!string.IsNullOrEmpty(configPath))
        return configPath;

    // 3. Default system location
    return Path.Combine(
        Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), 
        "FoundryVTT", 
        "Data"
    );
}

/// <summary>
/// Attempts to read the dataPath from FoundryVTT CLI configuration file
/// </summary>
static string? TryGetFoundryConfigPath()
{
    try
    {
        var configFile = Path.Combine(
            Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), 
            ".fvttrc.yml"
        );
        
        // If config doesn't exist, try to initialize it
        if (!File.Exists(configFile))
        {
            Console.WriteLine("🔧 FoundryVTT CLI not configured yet, attempting initialization...");
            if (TryInitializeFoundryConfig())
            {
                Console.WriteLine("✅ FoundryVTT CLI configured successfully");
            }
            else
            {
                Console.WriteLine("❌ Could not initialize FoundryVTT CLI configuration");
                return null;
            }
        }

        if (!File.Exists(configFile))
            return null;

        var content = File.ReadAllText(configFile);
        var dataPath = ExtractDataPathFromYaml(content);
        
        if (!string.IsNullOrEmpty(dataPath))
        {
            var fullPath = Path.Combine(dataPath, "Data");
            return Directory.Exists(fullPath) ? fullPath : null;
        }
    }
    catch
    {
        // Ignore config file errors
    }
    
    return null;
}

/// <summary>
/// Attempts to initialize FoundryVTT CLI configuration
/// </summary>
static bool TryInitializeFoundryConfig()
{
    try
    {
        var fvttPath = Path.Combine(".", "node_modules", ".bin", "fvtt.cmd");
        if (!File.Exists(fvttPath))
        {
            Console.WriteLine($"❌ FoundryVTT CLI not found at: {fvttPath}");
            return false;
        }

        // Run basic configure command to create the config file
        var processInfo = new System.Diagnostics.ProcessStartInfo
        {
            FileName = fvttPath,
            Arguments = "configure",
            UseShellExecute = false,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            CreateNoWindow = true
        };

        using var process = System.Diagnostics.Process.Start(processInfo);
        if (process != null)
        {
            process.WaitForExit(10000); // 10 second timeout
            return process.ExitCode == 0;
        }
    }
    catch
    {
        // Ignore initialization errors
    }
    
    return false;
}

/// <summary>
/// Simple YAML parser to extract dataPath value
/// </summary>
static string? ExtractDataPathFromYaml(string yamlContent)
{
    foreach (var line in yamlContent.Split('\n'))
    {
        var trimmed = line.Trim();
        if (!trimmed.StartsWith("dataPath:", StringComparison.OrdinalIgnoreCase))
            continue;

        var value = trimmed.Substring("dataPath:".Length).Trim();
        
        // Remove quotes if present
        if (value.StartsWith('"') && value.EndsWith('"'))
            value = value.Substring(1, value.Length - 2);
            
        return string.IsNullOrEmpty(value) ? null : value;
    }
    
    return null;
}
