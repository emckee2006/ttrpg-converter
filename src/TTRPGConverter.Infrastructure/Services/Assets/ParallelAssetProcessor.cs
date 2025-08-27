using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.IO.Compression;
using System.Linq;
using System.Net.Http;
using System.Threading.Channels;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core;

namespace TTRPGConverter.Infrastructure.Services.Assets;

public class ParallelAssetProcessor
{
    private readonly ILogger<ParallelAssetProcessor> _logger;
    private readonly IHttpClientFactory _httpClientFactory;
    private readonly int _maxDegreeOfParallelism;

    public ParallelAssetProcessor(ILogger<ParallelAssetProcessor> logger, IHttpClientFactory httpClientFactory, int maxDegreeOfParallelism = 16)
    {
        _logger = logger;
        _httpClientFactory = httpClientFactory;
        _maxDegreeOfParallelism = maxDegreeOfParallelism;
    }

    public async Task<AssetProcessingResult> ProcessAssetsAsync(IReadOnlyList<AssetRequest> requests, string? campaignZipPath)
    {
        var stopwatch = Stopwatch.StartNew();
        var channel = Channel.CreateUnbounded<AssetRequest>();
        var processedAssets = new List<ProcessedAsset>();
        var errors = new List<AssetProcessingError>();

        // Producer task
        var producer = Task.Run(async () =>
        {
            foreach (var request in requests)
            {
                await channel.Writer.WriteAsync(request);
            }
            channel.Writer.Complete();
        });

        // Consumer tasks
        var consumers = Enumerable.Range(0, _maxDegreeOfParallelism)
            .Select(_ => Task.Run(async () =>
            {
                while (await channel.Reader.WaitToReadAsync())
                {
                    if (channel.Reader.TryRead(out var request))
                    {
                        try
                        {
                            var finalPath = await ProcessSingleAssetAsync(request, campaignZipPath);
                            if (finalPath != null)
                            {
                                lock (processedAssets)
                                {
                                    processedAssets.Add(new ProcessedAsset { Request = request, FinalPath = finalPath });
                                }
                            }
                        }
                        catch (Exception ex)
                        {
                            lock (errors)
                            {
                                // Correctly create the AssetProcessingError with the Exception object
                                errors.Add(new AssetProcessingError { Request = request, Error = ex });
                            }
                        }
                    }
                }
            }))
            .ToList();

        await producer;
        await Task.WhenAll(consumers);

        stopwatch.Stop();
        return new AssetProcessingResult
        {
            ProcessedAssets = processedAssets,
            Errors = errors,
            ProcessingTime = stopwatch.Elapsed
        };
    }

    private async Task<string?> ProcessSingleAssetAsync(AssetRequest request, string? campaignZipPath)
    {
        // Ensure the destination directory exists
        var directory = Path.GetDirectoryName(request.DestinationPath);
        if (!string.IsNullOrEmpty(directory)) {
             Directory.CreateDirectory(directory);
        }

        // 1. Try to extract from ZIP first
        if (!string.IsNullOrEmpty(campaignZipPath) && File.Exists(campaignZipPath))
        {
            if (TryExtractFromZip(request, campaignZipPath))
            {
                _logger.LogDebug("Extracted asset from ZIP: {Url}", request.Roll20Url);
                return request.DestinationPath;
            }
        }

        // 2. Fallback to network download
        if (Uri.TryCreate(request.Roll20Url, UriKind.Absolute, out _))
        {
            return await DownloadAssetAsync(request);
        }

        _logger.LogWarning("Could not process asset (invalid URL and not in ZIP): {Url}", request.Roll20Url);
        return null;
    }

    private bool TryExtractFromZip(AssetRequest request, string campaignZipPath)
    {
        try
        {
            using var archive = ZipFile.OpenRead(campaignZipPath);
            // Roll20 URLs are often percent-encoded and have a prefix like "https://s3.amazonaws.com/files.d20.io/images/"
            // We need to find the corresponding file in the zip, which is usually just the image name.
            var imageName = Path.GetFileName(new Uri(request.Roll20Url).AbsolutePath);

            var entry = archive.Entries.FirstOrDefault(e => e.Name.Equals(imageName, StringComparison.OrdinalIgnoreCase));

            if (entry != null)
            {
                entry.ExtractToFile(request.DestinationPath, overwrite: true);
                return true;
            }
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to extract asset from ZIP: {Url}", request.Roll20Url);
        }
        return false;
    }

    private async Task<string?> DownloadAssetAsync(AssetRequest request)
    {
        try
        {
            var client = _httpClientFactory.CreateClient();
            using var response = await client.GetAsync(request.Roll20Url, HttpCompletionOption.ResponseHeadersRead);
            response.EnsureSuccessStatusCode();

            using var stream = await response.Content.ReadAsStreamAsync();
            using var fileStream = new FileStream(request.DestinationPath, FileMode.Create, FileAccess.Write, FileShare.None);
            await stream.CopyToAsync(fileStream);
            _logger.LogDebug("Downloaded asset: {Url}", request.Roll20Url);
            return request.DestinationPath;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to download asset: {Url}", request.Roll20Url);
            return null;
        }
    }
}
