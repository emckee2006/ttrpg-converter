namespace TTRPGConverter.Core.Interfaces;

/// <summary>
/// Interface for processing campaign assets (images, audio, etc.)
/// </summary>
public interface IAssetProcessor
{
    /// <summary>
    /// Process and copy assets from source to destination
    /// </summary>
    Task<string> ProcessAssetAsync(string assetPath, string destinationPath);
    
    /// <summary>
    /// Get relative path for asset in destination structure
    /// </summary>
    string GetAssetPath(string originalPath);
}
