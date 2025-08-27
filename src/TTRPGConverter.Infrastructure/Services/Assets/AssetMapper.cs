using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core;

namespace TTRPGConverter.Infrastructure.Services.Assets;

public class AssetMapper : IAssetMapper
{
    private readonly ILogger<AssetMapper> _logger;
    private readonly ParallelAssetProcessor _assetProcessor;
    private readonly IDbContextFactory<TTRPGConverterContext> _contextFactory;

    public AssetMapper(
        ILogger<AssetMapper> logger, 
        ParallelAssetProcessor assetProcessor, 
        IDbContextFactory<TTRPGConverterContext> contextFactory)
    {
        _logger = logger;
        _assetProcessor = assetProcessor;
        _contextFactory = contextFactory;
    }

    public async Task<string?> GetAssetPathAsync(string sourceUrl, string? campaignZipPath = null)
    {
        if (string.IsNullOrEmpty(sourceUrl))
        {
            return null;
        }

        await using var context = await _contextFactory.CreateDbContextAsync();

        var cachedMapping = await context.AssetMappings.FindAsync(sourceUrl);
        if (cachedMapping != null)
        {
            _logger.LogDebug("Asset found in cache: {Url}", sourceUrl);
            return cachedMapping.LocalPath;
        }

        _logger.LogDebug("Asset not in cache, processing: {Url}", sourceUrl);
        var request = new AssetRequest
        {
            Roll20Url = sourceUrl,
            DestinationPath = "assets/" + System.IO.Path.GetFileName(new System.Uri(sourceUrl).AbsolutePath),
            EntityId = "unknown",
            EntityType = "unknown",
            AssetType = "unknown"
        };

        var result = await _assetProcessor.ProcessAssetsAsync(new List<AssetRequest> { request }, campaignZipPath);

        if (result.Errors.Any())
        {
            // Correctly access the Exception object's Message property
            _logger.LogWarning("Failed to process asset: {Url}. Error: {Error}", sourceUrl, result.Errors.First().Error.Message);
            return null;
        }

        var processedAsset = result.ProcessedAssets.FirstOrDefault();
        if (processedAsset == null)
        {
            _logger.LogWarning("Asset was processed but no final path was returned: {Url}", sourceUrl);
            return null;
        }

        var newMapping = new AssetMapping
        {
            SourceUrl = sourceUrl,
            LocalPath = processedAsset.FinalPath
        };
        context.AssetMappings.Add(newMapping);
        await context.SaveChangesAsync();

        _logger.LogInformation("Cached new asset: {Url} -> {Path}", sourceUrl, processedAsset.FinalPath);
        return processedAsset.FinalPath;
    }

    public Task<string?> ProcessSceneBackgroundAsync(string sourceUrl, int targetWidth, int targetHeight, string? campaignZipPath = null)
    {
        _logger.LogInformation("Scene background processing not yet implemented. Using standard asset mapping for now.");
        return GetAssetPathAsync(sourceUrl, campaignZipPath);
    }
}
