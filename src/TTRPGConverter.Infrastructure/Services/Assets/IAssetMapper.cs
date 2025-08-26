namespace TTRPGConverter.Infrastructure.Services.Assets;

/// <summary>
/// Interface for handling the mapping and processing of external assets (images, audio, etc.)
/// </summary>
public interface IAssetMapper
{
    /// <summary>
    /// Takes a source URL for an asset and returns the final, local path in the Foundry world.
    /// This method encapsulates all logic for checking local ZIPs, downloading, de-duplicating, and optimizing.
    /// </summary>
    /// <param name="sourceUrl">The original URL of the asset from the Roll20 campaign.</param>
    /// <param name="campaignZipPath">The local path to the campaign ZIP file, if available.</param>
    /// <returns>The final, local path for the asset within the Foundry world's data structure.</returns>
    Task<string?> GetAssetPathAsync(string sourceUrl, string? campaignZipPath = null);

    /// <summary>
    /// A specialized method for processing scene backgrounds, which may require cropping or padding.
    /// </summary>
    /// <param name="sourceUrl">The original URL of the background image.</param>
    /// <param name="targetWidth">The target width of the final scene in pixels.</param>
    /// <param name="targetHeight">The target height of the final scene in pixels.</param>
    /// <param name="campaignZipPath">The local path to the campaign ZIP file, if available.</param>
    /// <returns>The final, local path for the processed background image.</returns>
    Task<string?> ProcessSceneBackgroundAsync(string sourceUrl, int targetWidth, int targetHeight, string? campaignZipPath = null);
}
