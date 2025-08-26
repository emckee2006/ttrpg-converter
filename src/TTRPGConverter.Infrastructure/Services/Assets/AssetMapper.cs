using System.Threading.Tasks;

namespace TTRPGConverter.Infrastructure.Services.Assets;

public class AssetMapper : IAssetMapper
{
    public Task<string?> GetAssetPathAsync(string sourceUrl, string? campaignZipPath = null)
    {
        // Stub implementation
        return Task.FromResult<string?>(sourceUrl);
    }

    public Task<string?> ProcessSceneBackgroundAsync(string sourceUrl, int targetWidth, int targetHeight, string? campaignZipPath = null)
    {
        // Stub implementation
        return Task.FromResult<string?>(sourceUrl);
    }
}
