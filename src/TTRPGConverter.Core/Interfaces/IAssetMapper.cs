namespace TTRPGConverter.Core.Interfaces;

/// <summary>
/// Interface for mapping assets (images, audio, etc.) between platforms
/// </summary>
public interface IAssetMapper
{
    /// <summary>
    /// Map an asset URL from source to target platform format
    /// </summary>
    /// <param name="sourceAssetUrl">Original asset URL</param>
    /// <param name="assetType">Type of asset (image, audio, etc.)</param>
    /// <returns>Mapped asset URL for target platform</returns>
    string MapAsset(string sourceAssetUrl, AssetType assetType);
    
    /// <summary>
    /// Check if an asset exists and is accessible
    /// </summary>
    /// <param name="assetUrl">Asset URL to check</param>
    /// <returns>True if asset exists</returns>
    Task<bool> AssetExistsAsync(string assetUrl);
    
    /// <summary>
    /// Download and process an asset for the target platform
    /// </summary>
    /// <param name="sourceAssetUrl">Source asset URL</param>
    /// <param name="targetPath">Target file path</param>
    /// <param name="assetType">Type of asset</param>
    /// <returns>Processed asset information</returns>
    Task<AssetInfo> ProcessAssetAsync(string sourceAssetUrl, string targetPath, AssetType assetType);
}

/// <summary>
/// Types of assets in TTRPG platforms
/// </summary>
public enum AssetType
{
    Image,
    Audio,
    Video,
    Document,
    Token,
    Map,
    Portrait,
    Icon
}

/// <summary>
/// Information about a processed asset
/// </summary>
public record AssetInfo(
    string OriginalUrl,
    string ProcessedPath,
    AssetType Type,
    long SizeBytes,
    string? MimeType = null,
    Dictionary<string, object>? Metadata = null
);
