using Microsoft.Extensions.Logging;
using TTRPGConverter.Infrastructure.Services.Assets;

namespace TTRPGConverter.Processing.Conversion;

/// <summary>
/// Holds the context and settings for a single conversion job.
/// </summary>
public class ConversionContext
{
    public string SourcePlatform { get; }
    public string TargetPlatform { get; }
    public IReadOnlyDictionary<string, object> Settings { get; }
    public IAssetMapper AssetMapper { get; }
    public ILogger Logger { get; }

    public ConversionContext(
        string sourcePlatform,
        string targetPlatform,
        IAssetMapper assetMapper,
        ILogger logger,
        Dictionary<string, object>? settings = null)
    {
        SourcePlatform = sourcePlatform ?? throw new ArgumentNullException(nameof(sourcePlatform));
        TargetPlatform = targetPlatform ?? throw new ArgumentNullException(nameof(targetPlatform));
        AssetMapper = assetMapper ?? throw new ArgumentNullException(nameof(assetMapper));
        Logger = logger ?? throw new ArgumentNullException(nameof(logger));
        Settings = settings ?? new Dictionary<string, object>();
    }
}
