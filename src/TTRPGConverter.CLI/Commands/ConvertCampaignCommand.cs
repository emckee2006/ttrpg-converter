using TTRPGConverter.Core.Services;
using Microsoft.Extensions.Logging;

namespace TTRPGConverter.CLI.Commands;

/// <summary>
/// Command to convert Roll20 campaign ZIP to Foundry VTT world
/// </summary>
public class ConvertCampaignCommand
{
    private readonly ILogger<ConvertCampaignCommand> _logger;
    private readonly Roll20CampaignService _campaignService;
    private readonly FoundryWorldBuilder _worldBuilder;

    public ConvertCampaignCommand(
        ILogger<ConvertCampaignCommand> logger,
        Roll20CampaignService campaignService,
        FoundryWorldBuilder worldBuilder)
    {
        _logger = logger;
        _campaignService = campaignService;
        _worldBuilder = worldBuilder;
    }

    public async Task ExecuteAsync(string campaignZipPath, string outputDir, string system, string? worldName)
    {
        try
        {
            _logger.LogInformation("🚀 Starting Roll20→Foundry conversion");
            _logger.LogInformation("📦 Campaign ZIP: {CampaignPath}", campaignZipPath);
            _logger.LogInformation("📁 Output Directory: {OutputDir}", outputDir);
            _logger.LogInformation("🎲 Target System: {System}", system);

            // Validate inputs
            if (!File.Exists(campaignZipPath))
            {
                _logger.LogError("❌ Campaign ZIP file not found: {Path}", campaignZipPath);
                return;
            }

            if (!Directory.Exists(outputDir))
            {
                _logger.LogInformation("📁 Creating output directory: {Dir}", outputDir);
                Directory.CreateDirectory(outputDir);
            }

            // Generate world name if not provided
            if (string.IsNullOrEmpty(worldName))
            {
                worldName = Path.GetFileNameWithoutExtension(campaignZipPath);
                _logger.LogInformation("🏷️ Using world name: {Name}", worldName);
            }

            // Load Roll20 campaign
            _logger.LogInformation("📖 Loading Roll20 campaign...");
            var campaign = await _campaignService.LoadCampaignAsync(campaignZipPath);
            
            if (campaign.Campaign == null)
            {
                _logger.LogError("❌ Failed to load campaign data");
                return;
            }

            _logger.LogInformation("✅ Campaign loaded successfully");
            _logger.LogInformation("👥 Characters: {Count}", campaign.Campaign.Characters?.Count ?? 0);
            _logger.LogInformation("🗺️ Pages: {Count}", campaign.Campaign.Pages?.Count ?? 0);
            _logger.LogInformation("📄 Handouts: {Count}", campaign.Campaign.Handouts?.Count ?? 0);

            // Convert to Foundry world
            _logger.LogInformation("🏗️ Building Foundry world...");
            var world = await _worldBuilder.CreateWorldAsync(campaign, outputDir, system, worldName);

            _logger.LogInformation("🎉 Conversion completed successfully!");
            _logger.LogInformation("📍 Foundry World Path: {Path}", world.WorldPath);
            _logger.LogInformation("💡 To use:");
            _logger.LogInformation("   1. Copy world folder to Foundry Data/worlds/");
            _logger.LogInformation("   2. Restart Foundry VTT");
            _logger.LogInformation("   3. Select '{Name}' from world list", worldName);

            // Show conversion summary  
            await ShowConversionSummary(world);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Conversion failed");
        }
    }

    private Task ShowConversionSummary(FoundryWorld world)
    {
        try
        {
            _logger.LogInformation("📊 CONVERSION SUMMARY");
            _logger.LogInformation("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

            // Count created files
            var worldPath = world.WorldPath;
            var actorFiles = Directory.GetFiles(Path.Combine(worldPath, "data", "actors"), "*.json").Length;
            var sceneFiles = Directory.GetFiles(Path.Combine(worldPath, "data", "scenes"), "*.json").Length;
            var journalFiles = Directory.GetFiles(Path.Combine(worldPath, "data", "journal"), "*.json").Length;

            _logger.LogInformation("🎭 Actors Created: {Count}", actorFiles);
            _logger.LogInformation("🗺️ Scenes Created: {Count}", sceneFiles);
            _logger.LogInformation("📑 Journal Entries: {Count}", journalFiles);

            // Count assets
            var assetDirs = new[] { "maps", "tokens", "audio" };
            var totalAssets = 0;
            foreach (var assetDir in assetDirs)
            {
                var assetPath = Path.Combine(worldPath, "assets", assetDir);
                if (Directory.Exists(assetPath))
                {
                    var count = Directory.GetFiles(assetPath).Length;
                    totalAssets += count;
                    _logger.LogInformation("📦 {AssetType}: {Count}", assetDir.ToTitleCase(), count);
                }
            }

            _logger.LogInformation("📊 Total Assets: {Count}", totalAssets);

            // Show world info
            var worldJsonPath = Path.Combine(worldPath, "world.json");
            if (File.Exists(worldJsonPath))
            {
                var worldSize = new DirectoryInfo(worldPath).GetDirectorySize();
                _logger.LogInformation("💾 World Size: {Size:F1} MB", worldSize / (1024.0 * 1024.0));
            }

            _logger.LogInformation("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Could not generate conversion summary");
        }
        
        return Task.CompletedTask;
    }
}

/// <summary>
/// Extension methods for ConvertCampaignCommand
/// </summary>
public static class ConvertCampaignExtensions
{
    public static string ToTitleCase(this string input)
    {
        if (string.IsNullOrEmpty(input)) return input;
        return char.ToUpper(input[0]) + input[1..].ToLower();
    }

    public static long GetDirectorySize(this DirectoryInfo directory)
    {
        return directory.GetFiles("*", SearchOption.AllDirectories).Sum(file => file.Length);
    }
}
