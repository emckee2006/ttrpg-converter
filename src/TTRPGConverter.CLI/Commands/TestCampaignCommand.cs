using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Services;

namespace TTRPGConverter.CLI.Commands;

/// <summary>
/// Test command for Roll20 campaign loading and parsing
/// </summary>
public class TestCampaignCommand
{
    private readonly ILogger<TestCampaignCommand> _logger;
    private readonly Roll20CampaignService _campaignService;

    public TestCampaignCommand(ILogger<TestCampaignCommand> logger, Roll20CampaignService campaignService)
    {
        _logger = logger;
        _campaignService = campaignService;
    }

    public async Task ExecuteAsync(string zipPath)
    {
        try
        {
            _logger.LogInformation("🎲 Testing Roll20 campaign loading: {ZipPath}", zipPath);

            // Load campaign
            var campaign = await _campaignService.LoadCampaignAsync(zipPath);
            
            _logger.LogInformation("✅ Campaign loaded successfully:");
            _logger.LogInformation("   🎭 Characters: {Count}", campaign.Campaign.Characters?.Count ?? 0);
            _logger.LogInformation("   📄 Pages: {Count}", campaign.Campaign.Pages?.Count ?? 0);
            _logger.LogInformation("   📑 Handouts: {Count}", campaign.Campaign.Handouts?.Count ?? 0);
            _logger.LogInformation("   🎵 Audio: {Count}", campaign.Campaign.Jukebox?.Count ?? 0);

            // Test character processing
            var characters = _campaignService.GetCharacters(campaign);
            _logger.LogInformation("📊 Character Analysis:");
            
            foreach (var character in characters.Take(3)) // Show first 3 characters
            {
                _logger.LogInformation("   🎭 {Name}:", character.Character.Name);
                _logger.LogInformation("      - Attributes: {Count}", character.Attributes.Count);
                _logger.LogInformation("      - Abilities: {Count}", character.Abilities.Count);
                
                // Show sample attributes
                foreach (var attr in character.Attributes.Take(5))
                {
                    _logger.LogInformation("      - {Name}: {Value}", attr.Key, attr.Value);
                }
            }

            // Test asset discovery
            var assets = _campaignService.GetAssetPaths(campaign);
            _logger.LogInformation("🗂️ Asset Discovery:");
            _logger.LogInformation("   🖼️ Images: {Count}", assets["images"].Count);
            _logger.LogInformation("   🗺️ Maps: {Count}", assets["maps"].Count);
            _logger.LogInformation("   🎭 Tokens: {Count}", assets["tokens"].Count);
            _logger.LogInformation("   🎵 Audio: {Count}", assets["audio"].Count);
            _logger.LogInformation("   📄 Handouts: {Count}", assets["handouts"].Count);

            // Show sample asset paths
            foreach (var (type, paths) in assets.Where(kv => kv.Value.Count > 0))
            {
                _logger.LogInformation("   {Type} samples:", type);
                foreach (var path in paths.Take(2))
                {
                    _logger.LogInformation("     - {Path}", Path.GetFileName(path));
                }
            }

            _logger.LogInformation("🎉 Campaign test completed successfully!");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Campaign test failed: {Message}", ex.Message);
            throw;
        }
    }
}
