using System.IO.Compression;
using System.Text.Json;
using TTRPGConverter.Core.Models.Roll20;
using Microsoft.Extensions.Logging;
using Roll20Path = TTRPGConverter.Core.Models.Roll20.Path;

namespace TTRPGConverter.Core.Services;

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

    /// <summary>
    /// Load a Roll20 campaign from a ZIP file
    /// </summary>
    /// <param name="zipFilePath">Path to the Roll20 campaign ZIP file</param>
    /// <returns>Loaded campaign data</returns>
    public async Task<Roll20Campaign> LoadCampaignAsync(string zipFilePath)
    {
        if (!File.Exists(zipFilePath))
            throw new FileNotFoundException($"Campaign ZIP file not found: {zipFilePath}");

        _logger.LogInformation("Reading Roll20 campaign from ZIP: {ZipPath}", zipFilePath);

        using var archive = ZipFile.OpenRead(zipFilePath);
        
        // Find campaign.json entry
        var campaignEntry = archive.Entries.FirstOrDefault(e => 
            e.Name.Equals("campaign.json", StringComparison.OrdinalIgnoreCase));

        if (campaignEntry == null)
            throw new InvalidOperationException($"campaign.json not found in ZIP: {zipFilePath}");

        _logger.LogDebug("Found campaign.json entry, size: {Size} bytes", campaignEntry.Length);

        // Read and deserialize campaign.json directly from ZIP
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

    /// <summary>
    /// Get all asset file paths from the ZIP archive
    /// </summary>
    /// <param name="campaign">Loaded campaign</param>
    /// <returns>Dictionary of asset paths by type</returns>
    public Dictionary<string, List<string>> GetAssetPaths(Roll20Campaign campaign)
    {
        var assets = new Dictionary<string, List<string>>
        {
            ["images"] = new(),
            ["audio"] = new(),
            ["maps"] = new(),
            ["tokens"] = new(),
            ["handouts"] = new()
        };

        using var archive = ZipFile.OpenRead(campaign.SourceZipPath);
        
        foreach (var entry in archive.Entries)
        {
            if (string.IsNullOrEmpty(entry.Name)) continue; // Skip directories
            
            var extension = System.IO.Path.GetExtension(entry.Name).ToLowerInvariant();
            var fileName = entry.Name.ToLowerInvariant();

            switch (extension)
            {
                case ".jpg" or ".jpeg" or ".png" or ".gif" or ".webp":
                    if (fileName.Contains("map") || fileName.Contains("battle"))
                        assets["maps"].Add(entry.FullName);
                    else if (fileName.Contains("token") || fileName.Contains("portrait"))
                        assets["tokens"].Add(entry.FullName);
                    else
                        assets["images"].Add(entry.FullName);
                    break;

                case ".mp3" or ".wav" or ".ogg" or ".m4a":
                    assets["audio"].Add(entry.FullName);
                    break;

                case ".pdf" or ".txt" or ".md":
                    assets["handouts"].Add(entry.FullName);
                    break;
            }
        }

        _logger.LogDebug("Found assets: {Images} images, {Maps} maps, {Tokens} tokens, {Audio} audio, {Handouts} handouts",
            assets["images"].Count, assets["maps"].Count, assets["tokens"].Count, 
            assets["audio"].Count, assets["handouts"].Count);

        return assets;
    }

    /// <summary>
    /// Get all characters from the campaign with their associated data
    /// </summary>
    /// <param name="campaign">Loaded campaign</param>
    /// <returns>List of enriched character data</returns>
    public List<CharacterInfo> GetCharacters(Roll20Campaign campaign)
    {
        var characters = new List<CharacterInfo>();

        if (campaign.Campaign.Characters == null)
            return characters;

        foreach (var character in campaign.Campaign.Characters)
        {
            var info = new CharacterInfo
            {
                Character = character,
                Attributes = character.Attribs?
                    .ToDictionary(a => a.Name ?? "", a => a.Current) ?? new(),
                Abilities = character.Abilities?.ToList() ?? new()
            };

            characters.Add(info);
        }

        _logger.LogDebug("Processed {Count} characters with attributes and abilities", characters.Count);
        return characters;
    }

    /// <summary>
    /// Extract an asset from the ZIP to a destination path
    /// </summary>
    /// <param name="campaign">Loaded campaign</param>
    /// <param name="assetPath">Path within ZIP</param>
    /// <param name="destinationPath">Local file path to extract to</param>
    public async Task ExtractAssetAsync(Roll20Campaign campaign, string assetPath, string destinationPath)
    {
        _logger.LogDebug("Extracting asset: {AssetPath} -> {DestinationPath}", assetPath, destinationPath);

        using var archive = ZipFile.OpenRead(campaign.SourceZipPath);
        
        var entry = archive.Entries.FirstOrDefault(e => 
            e.FullName.Equals(assetPath, StringComparison.OrdinalIgnoreCase));

        if (entry == null)
            throw new FileNotFoundException($"Asset not found in ZIP: {assetPath}");

        // Ensure destination directory exists
        var destinationDir = System.IO.Path.GetDirectoryName(destinationPath);
        if (!string.IsNullOrEmpty(destinationDir) && !Directory.Exists(destinationDir))
        {
            Directory.CreateDirectory(destinationDir);
            _logger.LogDebug("Created directory: {Directory}", destinationDir);
        }

        // Extract the asset
        using var sourceStream = entry.Open();
        using var destinationStream = File.Create(destinationPath);
        
        await sourceStream.CopyToAsync(destinationStream);
        
        _logger.LogDebug("Extracted asset: {AssetPath} ({Size} bytes)", assetPath, entry.Length);
    }

    /// <summary>
    /// Get asset stream directly from ZIP (for streaming scenarios)
    /// </summary>
    /// <param name="campaign">Loaded campaign</param>
    /// <param name="assetPath">Path within ZIP</param>
    /// <returns>Asset stream</returns>
    public Stream? GetAssetStream(Roll20Campaign campaign, string assetPath)
    {
        var archive = ZipFile.OpenRead(campaign.SourceZipPath);
        
        var entry = archive.Entries.FirstOrDefault(e => 
            e.FullName.Equals(assetPath, StringComparison.OrdinalIgnoreCase));

        return entry?.Open();
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

/// <summary>
/// Enriched character information with attributes and abilities
/// </summary>
public class CharacterInfo
{
    public required Character Character { get; init; }
    public required Dictionary<string, string?> Attributes { get; init; }
    public required IList<Ability> Abilities { get; init; }
}
