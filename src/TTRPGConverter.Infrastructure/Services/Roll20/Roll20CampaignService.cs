using System;
using System.Collections.Generic;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Models.Roll20;

namespace TTRPGConverter.Infrastructure.Services.Roll20;

/// <summary>
/// Service for loading and parsing Roll20 campaign ZIP files
/// </summary>
public class Roll20CampaignService
{
    private readonly ILogger<Roll20CampaignService> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public Roll20CampaignService(ILogger<Roll20CampaignService> logger)
    {
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNameCaseInsensitive = true,
            AllowTrailingCommas = true,
            ReadCommentHandling = JsonCommentHandling.Skip
        };
    }

    public async Task<Roll20Campaign> LoadCampaignAsync(string zipFilePath)
    {
        if (!File.Exists(zipFilePath))
            throw new FileNotFoundException($"Campaign ZIP file not found: {zipFilePath}");

        _logger.LogInformation("Reading Roll20 campaign from ZIP: {ZipPath}", zipFilePath);

        using var archive = ZipFile.OpenRead(zipFilePath);
        
        var campaignEntry = archive.Entries.FirstOrDefault(e => 
            e.Name.Equals("campaign.json", StringComparison.OrdinalIgnoreCase));

        if (campaignEntry == null)
            throw new InvalidOperationException($"campaign.json not found in ZIP: {zipFilePath}");

        _logger.LogDebug("Found campaign.json entry, size: {Size} bytes", campaignEntry.Length);

        using var stream = campaignEntry.Open();
        using var reader = new StreamReader(stream);
        var campaignJson = await reader.ReadToEndAsync();

        var campaign = JsonSerializer.Deserialize<Campaign>(campaignJson, _jsonOptions);
        if (campaign == null)
            throw new InvalidOperationException($"Failed to deserialize campaign.json from: {zipFilePath}");

        _logger.LogInformation("Loaded Roll20 campaign with {Characters} characters, {Pages} pages", 
            campaign.Characters?.Count ?? 0, campaign.Pages?.Count ?? 0);

        return new Roll20Campaign
        {
            SourceZipPath = zipFilePath,
            Campaign = campaign,
            LoadedAt = DateTimeOffset.UtcNow
        };
    }
}

/// <summary>
/// Container for a loaded Roll20 campaign with metadata
/// </summary>
public class Roll20Campaign
{
    public required string SourceZipPath { get; init; }
    public required Campaign Campaign { get; init; }
    public required DateTimeOffset LoadedAt { get; init; }
}
