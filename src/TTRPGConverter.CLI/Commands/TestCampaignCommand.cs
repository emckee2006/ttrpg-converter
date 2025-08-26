using Microsoft.Extensions.Logging;
using System;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using TTRPGConverter.Infrastructure.Services.Roll20;

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

            _logger.LogInformation("🎉 Campaign test completed successfully!");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Campaign test failed: {Message}", ex.Message);
            throw;
        }
    }
}
