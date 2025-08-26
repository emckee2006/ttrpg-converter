using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Mappers;
using TTRPGConverter.Infrastructure.Services.Foundry;
using TTRPGConverter.Infrastructure.Services.Roll20;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.CLI.Commands;

public class ConvertCampaignCommand
{
    private readonly ILogger<ConvertCampaignCommand> _logger;
    private readonly Roll20CampaignService _campaignService;
    private readonly FoundryWorldBuilder _worldBuilder;
    private readonly ICompendiumManager _compendiumManager;
    private readonly Roll20ToFoundryMapper _mapper;

    public ConvertCampaignCommand(
        ILogger<ConvertCampaignCommand> logger,
        Roll20CampaignService campaignService,
        FoundryWorldBuilder worldBuilder,
        ICompendiumManager compendiumManager,
        Roll20ToFoundryMapper mapper)
    {
        _logger = logger;
        _campaignService = campaignService;
        _worldBuilder = worldBuilder;
        _compendiumManager = compendiumManager;
        _mapper = mapper;
    }

    public async Task HandleCommandAsync(string sourceZip, string outputDir, string worldName, string system)
    {
        _logger.LogInformation("Starting campaign conversion...");
        _logger.LogInformation("Source ZIP: {SourceZip}", sourceZip);
        _logger.LogInformation("Output Directory: {OutputDir}", outputDir);
        _logger.LogInformation("World Name: {WorldName}", worldName);
        _logger.LogInformation("Target System: {System}", system);

        try
        {
            // 1. Load the Roll20 Campaign
            var campaign = await _campaignService.LoadCampaignAsync(sourceZip);
            if (campaign == null)
            {
                _logger.LogError("Failed to load campaign from {SourceZip}", sourceZip);
                return;
            }

            // 2. Create the Foundry World
            var world = await _worldBuilder.CreateWorldAsync(campaign, outputDir, system, worldName);
            if (world == null)
            {
                _logger.LogError("Failed to create Foundry world.");
                return;
            }

            _logger.LogInformation("Conversion complete! Foundry world created at: {WorldPath}", world.WorldPath);
        }
        catch (Exception ex)
        {
            _logger.LogCritical(ex, "A critical error occurred during the conversion process.");
        }
    }
}
