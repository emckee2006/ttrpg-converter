using System.Collections.Concurrent;
using System.IO.Compression;
using System.IO.Hashing;
using System.Text;
using System.Threading.Channels;
using Microsoft.Extensions.Caching.Memory;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;
using Polly;

namespace TTRPGConverter.Infrastructure.Services.Assets;

/// <summary>
/// Configuration for asset processing
/// </summary>
public class AssetProcessingOptions
{
    public int MaxConcurrentDownloads { get; set; } = Environment.ProcessorCount * 2;
    public int MaxConcurrentExtractions { get; set; } = Environment.ProcessorCount;
    public TimeSpan HttpTimeout { get; set; } = TimeSpan.FromMinutes(5);
    public int RetryAttempts { get; set; } = 3;
    public bool EnableDeduplication { get; set; } = true;
    public string CacheDirectory { get; set; } = "cache/assets";
}

/// <summary>
/// Represents an asset processing request
/// </summary>
public record AssetRequest
{
    public required string Roll20Url { get; init; }
    public required string DestinationPath { get; init; }
    public required string AssetType { get; init; } // "actors", "tiles", "cards", "audio"
    public string? EntityId { get; init; }
    public string? EntityType { get; init; }
    public Dictionary<string, object> Metadata { get; init; } = new();
}

/// <summary>
/// Result of processing an asset
/// </summary>
public record ProcessedAsset
{
    public required string OriginalUrl { get; init; }
    public required string LocalPath { get; init; }
    public required string ContentType { get; init; }
    public long FileSize { get; init; }
    public AssetSource Source { get; init; }
    public string? UrlHash { get; init; }
    public DateTime ProcessedAt { get; init; } = DateTime.UtcNow;
}

/// <summary>
/// Source of the processed asset
/// </summary>
public enum AssetSource
{
    ZipExtraction,
    WebDownload,
    DiskCache,
    MemoryCache
}

/// <summary>
/// Error during asset processing
/// </summary>
public record AssetProcessingError
{
    public required AssetRequest Request { get; init; }
    public required Exception Error { get; init; }
}

/// <summary>
/// Complete result of parallel asset processing
/// </summary>
public record AssetProcessingResult
{
    public required List<ProcessedAsset> ProcessedAssets { get; init; }
    public required List<AssetProcessingError> Errors { get; init; }
    public required Dictionary<string, string> DeduplicationMap { get; init; }
    public TimeSpan ProcessingTime { get; init; }
}

/// <summary>
/// High-performance parallel asset processor with caching and deduplication
/// </summary>
public class ParallelAssetProcessor
{
    private readonly HttpClient _httpClient;
    private readonly IMemoryCache _memoryCache;
    private readonly ILogger<ParallelAssetProcessor> _logger;
    private readonly AssetProcessingOptions _options;
    
    // Thread-safe collections for tracking
    private readonly ConcurrentDictionary<string, string> _urlHashToPath = new();
    private readonly ConcurrentDictionary<string, SemaphoreSlim> _downloadSemaphores = new();
    
    // Semaphores for controlling concurrency
    private readonly SemaphoreSlim _downloadSemaphore;
    private readonly SemaphoreSlim _extractionSemaphore;
    
    public ParallelAssetProcessor(
        IHttpClientFactory httpClientFactory,
        IMemoryCache memoryCache,
        ILogger<ParallelAssetProcessor> logger,
        IOptions<AssetProcessingOptions> options)
    {
        _httpClient = httpClientFactory.CreateClient("AssetDownloader");
        _memoryCache = memoryCache;
        _logger = logger;
        _options = options.Value;
        
        _downloadSemaphore = new SemaphoreSlim(_options.MaxConcurrentDownloads);
        _extractionSemaphore = new SemaphoreSlim(_options.MaxConcurrentExtractions);
        
        // Configure HTTP client
        _httpClient.Timeout = _options.HttpTimeout;
    }
    
    /// <summary>
    /// Process assets in parallel with ZIP-first strategy and deduplication
    /// </summary>
    public async Task<AssetProcessingResult> ProcessAssetsAsync(
        IEnumerable<AssetRequest> requests,
        string? campaignZipPath = null,
        CancellationToken cancellationToken = default)
    {
        var startTime = DateTime.UtcNow;
        var requestList = requests.ToList();
        
        _logger.LogInformation("🎨 Starting parallel processing of {Count} assets", requestList.Count);
        
        // Thread-safe collections for results
        var processedAssets = new ConcurrentBag<ProcessedAsset>();
        var errors = new ConcurrentBag<AssetProcessingError>();
        
        // Create processing channel for better control
        var channelOptions = new BoundedChannelOptions(1000)
        {
            SingleReader = false,
            SingleWriter = false,
            AllowSynchronousContinuations = false,
            FullMode = BoundedChannelFullMode.Wait
        };
        
        var channel = Channel.CreateBounded<AssetRequest>(channelOptions);
        var writer = channel.Writer;
        var reader = channel.Reader;
        
        // Start consumer tasks
        var consumerTasks = Enumerable.Range(0, _options.MaxConcurrentDownloads)
            .Select(_ => ProcessAssetChannelAsync(reader, campaignZipPath, processedAssets, errors, cancellationToken))
            .ToArray();
        
        // Producer: Queue all requests
        var producerTask = Task.Run(async () =>
        {
            try
            {
                foreach (var request in requestList)
                {
                    await writer.WriteAsync(request, cancellationToken);
                }
            }
            finally
            {
                writer.Complete();
            }
        }, cancellationToken);
        
        // Wait for completion
        await producerTask;
        await Task.WhenAll(consumerTasks);
        
        var processingTime = DateTime.UtcNow - startTime;
        
        _logger.LogInformation("✅ Completed asset processing: {Processed} processed, {Errors} errors in {Time:F2}s", 
            processedAssets.Count, errors.Count, processingTime.TotalSeconds);
        
        return new AssetProcessingResult
        {
            ProcessedAssets = processedAssets.ToList(),
            Errors = errors.ToList(),
            DeduplicationMap = new Dictionary<string, string>(_urlHashToPath),
            ProcessingTime = processingTime
        };
    }
    
    /// <summary>
    /// Consumer task that processes assets from the channel
    /// </summary>
    private async Task ProcessAssetChannelAsync(
        ChannelReader<AssetRequest> reader,
        string? campaignZipPath,
        ConcurrentBag<ProcessedAsset> processedAssets,
        ConcurrentBag<AssetProcessingError> errors,
        CancellationToken cancellationToken)
    {
        await foreach (var request in reader.ReadAllAsync(cancellationToken))
        {
            try
            {
                var result = await ProcessSingleAssetAsync(request, campaignZipPath, cancellationToken);
                if (result != null)
                {
                    processedAssets.Add(result);
                }
            }
            catch (Exception ex)
            {
                errors.Add(new AssetProcessingError { Request = request, Error = ex });
                _logger.LogError(ex, "Failed to process asset: {Url}", request.Roll20Url);
            }
        }
    }
    
    /// <summary>
    /// Process a single asset with ZIP-first strategy
    /// </summary>
    private async Task<ProcessedAsset?> ProcessSingleAssetAsync(
        AssetRequest request,
        string? campaignZipPath,
        CancellationToken cancellationToken)
    {
        if (string.IsNullOrEmpty(request.Roll20Url))
            return null;
        
        // Generate URL hash for deduplication
        var urlHash = GenerateUrlHash(request.Roll20Url);
        var cacheKey = $"asset_{urlHash}";
        
        // Check memory cache first
        if (_memoryCache.TryGetValue(cacheKey, out ProcessedAsset? cachedAsset))
        {
            _logger.LogDebug("Memory cache hit: {Url}", request.Roll20Url);
            return cachedAsset;
        }
        
        // Check if we've already processed this URL (deduplication)
        if (_urlHashToPath.TryGetValue(urlHash, out var existingPath) && File.Exists(existingPath))
        {
            var deduplicatedAsset = CreateAssetFromExistingFile(request, existingPath, AssetSource.DiskCache, urlHash);
            _memoryCache.Set(cacheKey, deduplicatedAsset, TimeSpan.FromHours(1));
            return deduplicatedAsset;
        }
        
        // PHASE 1: Try ZIP extraction first
        if (!string.IsNullOrEmpty(campaignZipPath))
        {
            var zipResult = await TryExtractFromZipAsync(request, campaignZipPath, urlHash, cancellationToken);
            if (zipResult != null)
            {
                _memoryCache.Set(cacheKey, zipResult, TimeSpan.FromHours(1));
                return zipResult;
            }
        }
        
        // PHASE 2: Download from web with deduplication protection
        var downloadResult = await DownloadAssetAsync(request, urlHash, cancellationToken);
        if (downloadResult != null)
        {
            _memoryCache.Set(cacheKey, downloadResult, TimeSpan.FromHours(1));
        }
        
        return downloadResult;
    }
    
    /// <summary>
    /// Try to extract asset from campaign ZIP
    /// </summary>
    private async Task<ProcessedAsset?> TryExtractFromZipAsync(
        AssetRequest request,
        string zipPath,
        string urlHash,
        CancellationToken cancellationToken)
    {
        if (!File.Exists(zipPath))
            return null;
        
        await _extractionSemaphore.WaitAsync(cancellationToken);
        try
        {
            using var archive = ZipFile.OpenRead(zipPath);
            
            // Try to find the asset in the ZIP
            // Roll20 URLs often have the filename at the end
            var fileName = ExtractFileNameFromUrl(request.Roll20Url);
            if (string.IsNullOrEmpty(fileName))
                return null;
            
            var entry = archive.Entries.FirstOrDefault(e => 
                e.Name.Equals(fileName, StringComparison.OrdinalIgnoreCase) ||
                e.FullName.EndsWith(fileName, StringComparison.OrdinalIgnoreCase));
            
            if (entry == null)
                return null;
            
            // Create destination path with deduplication
            var destinationPath = GetDeduplicatedPath(request.DestinationPath, urlHash);
            Directory.CreateDirectory(Path.GetDirectoryName(destinationPath)!);
            
            // Extract file
            using var entryStream = entry.Open();
            using var fileStream = File.Create(destinationPath);
            await entryStream.CopyToAsync(fileStream, cancellationToken);
            
            var result = new ProcessedAsset
            {
                OriginalUrl = request.Roll20Url,
                LocalPath = destinationPath,
                ContentType = GetContentTypeFromExtension(destinationPath),
                FileSize = entry.Length,
                Source = AssetSource.ZipExtraction,
                UrlHash = urlHash
            };
            
            _urlHashToPath[urlHash] = destinationPath;
            _logger.LogDebug("Extracted from ZIP: {Url} → {Path}", request.Roll20Url, destinationPath);
            
            return result;
        }
        catch (Exception ex)
        {
            _logger.LogDebug(ex, "Failed to extract from ZIP: {Url}", request.Roll20Url);
            return null;
        }
        finally
        {
            _extractionSemaphore.Release();
        }
    }
    
    /// <summary>
    /// Download asset from web with retry policies
    /// </summary>
    private async Task<ProcessedAsset?> DownloadAssetAsync(
        AssetRequest request,
        string urlHash,
        CancellationToken cancellationToken)
    {
        // Use per-URL semaphore to prevent duplicate downloads
        var urlSemaphore = _downloadSemaphores.GetOrAdd(urlHash, _ => new SemaphoreSlim(1, 1));
        
        await _downloadSemaphore.WaitAsync(cancellationToken);
        await urlSemaphore.WaitAsync(cancellationToken);
        
        try
        {
            // Double-check if file was created by another thread
            if (_urlHashToPath.TryGetValue(urlHash, out var existingPath) && File.Exists(existingPath))
            {
                return CreateAssetFromExistingFile(request, existingPath, AssetSource.DiskCache, urlHash);
            }
            
            var destinationPath = GetDeduplicatedPath(request.DestinationPath, urlHash);
            Directory.CreateDirectory(Path.GetDirectoryName(destinationPath)!);
            
            // Download with retry policy
            var retryPolicy = Policy
                .Handle<HttpRequestException>()
                .Or<TaskCanceledException>()
                .WaitAndRetryAsync(
                    _options.RetryAttempts,
                    retryAttempt => TimeSpan.FromSeconds(Math.Pow(2, retryAttempt)),
                    onRetry: (outcome, timespan, retryCount, context) =>
                    {
                        _logger.LogWarning("Retry {RetryCount} for {Url} in {Delay}ms",
                            retryCount, request.Roll20Url, timespan.TotalMilliseconds);
                    });
            
            using var response = await retryPolicy.ExecuteAsync(async () =>
            {
                var httpResponse = await _httpClient.GetAsync(request.Roll20Url, cancellationToken);
                httpResponse.EnsureSuccessStatusCode();
                return httpResponse;
            });
            
            using var fileStream = File.Create(destinationPath);
            await response.Content.CopyToAsync(fileStream, cancellationToken);
            
            var result = new ProcessedAsset
            {
                OriginalUrl = request.Roll20Url,
                LocalPath = destinationPath,
                ContentType = response.Content.Headers.ContentType?.MediaType ?? "application/octet-stream",
                FileSize = new FileInfo(destinationPath).Length,
                Source = AssetSource.WebDownload,
                UrlHash = urlHash
            };
            
            _urlHashToPath[urlHash] = destinationPath;
            _logger.LogDebug("Downloaded: {Url} → {Path}", request.Roll20Url, destinationPath);
            
            return result;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to download asset: {Url}", request.Roll20Url);
            throw;
        }
        finally
        {
            urlSemaphore.Release();
            _downloadSemaphore.Release();
        }
    }
    
    /// <summary>
    /// Generate hash from URL for deduplication
    /// </summary>
    private static string GenerateUrlHash(string url)
    {
        var hash = XxHash64.Hash(Encoding.UTF8.GetBytes(url));
        return Convert.ToHexString(hash)[..16]; // First 16 chars
    }
    
    /// <summary>
    /// Get deduplicated file path
    /// </summary>
    private string GetDeduplicatedPath(string originalPath, string urlHash)
    {
        if (!_options.EnableDeduplication)
            return originalPath;
        
        var directory = Path.GetDirectoryName(originalPath) ?? "";
        var extension = Path.GetExtension(originalPath);
        var deduplicatedFileName = $"{urlHash}{extension}";
        
        return Path.Combine(directory, deduplicatedFileName);
    }
    
    /// <summary>
    /// Extract filename from Roll20 URL
    /// </summary>
    private static string? ExtractFileNameFromUrl(string url)
    {
        try
        {
            var uri = new Uri(url);
            var fileName = Path.GetFileName(uri.LocalPath);
            return string.IsNullOrEmpty(fileName) ? null : fileName;
        }
        catch
        {
            return null;
        }
    }
    
    /// <summary>
    /// Create asset record from existing file
    /// </summary>
    private static ProcessedAsset CreateAssetFromExistingFile(
        AssetRequest request, 
        string filePath, 
        AssetSource source,
        string urlHash)
    {
        var fileInfo = new FileInfo(filePath);
        return new ProcessedAsset
        {
            OriginalUrl = request.Roll20Url,
            LocalPath = filePath,
            ContentType = GetContentTypeFromExtension(filePath),
            FileSize = fileInfo.Length,
            Source = source,
            UrlHash = urlHash
        };
    }
    
    /// <summary>
    /// Get content type from file extension
    /// </summary>
    private static string GetContentTypeFromExtension(string filePath)
    {
        return Path.GetExtension(filePath).ToLowerInvariant() switch
        {
            ".jpg" or ".jpeg" => "image/jpeg",
            ".png" => "image/png",
            ".gif" => "image/gif",
            ".webp" => "image/webp",
            ".mp3" => "audio/mpeg",
            ".wav" => "audio/wav",
            ".mp4" => "video/mp4",
            ".pdf" => "application/pdf",
            _ => "application/octet-stream"
        };
    }
    
    public void Dispose()
    {
        _downloadSemaphore?.Dispose();
        _extractionSemaphore?.Dispose();
        
        foreach (var semaphore in _downloadSemaphores.Values)
        {
            semaphore.Dispose();
        }
        
        _httpClient?.Dispose();
        GC.SuppressFinalize(this);
    }
}
