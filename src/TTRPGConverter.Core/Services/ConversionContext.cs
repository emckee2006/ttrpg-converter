using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Interfaces;

namespace TTRPGConverter.Core.Services;

/// <summary>
/// Default implementation of conversion context
/// </summary>
public class ConversionContext : IConversionContext
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
        Settings = settings?.AsReadOnly() ?? new Dictionary<string, object>().AsReadOnly();
    }
}
