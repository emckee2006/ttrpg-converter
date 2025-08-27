using System.Text.Json;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Infrastructure.Services.Assets;
using TTRPGConverter.Infrastructure.Services.Roll20;

namespace TTRPGConverter.Infrastructure.Services.Foundry;

/// <summary>
/// Service for building Foundry VTT world packages from Roll20 campaign data
/// </summary>
public class FoundryWorldBuilder
{
    private readonly ILogger<FoundryWorldBuilder> _logger;
    private readonly Roll20CampaignService _campaignService;
    private readonly FoundryDatabaseFactory _databaseFactory;
    private readonly ParallelAssetProcessor _assetProcessor;
    private readonly JsonSerializerOptions _jsonOptions;

    public FoundryWorldBuilder(
        ILogger<FoundryWorldBuilder> logger,
        Roll20CampaignService campaignService,
        FoundryDatabaseFactory databaseFactory,
        ParallelAssetProcessor assetProcessor)
    {
        _logger = logger;
        _campaignService = campaignService;
        _databaseFactory = databaseFactory;
        _assetProcessor = assetProcessor;
        _jsonOptions = new JsonSerializerOptions
        {
            WriteIndented = false, // NeDB requires single-line JSON
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull
        };
    }

    public async Task<FoundryWorld> CreateWorldAsync(Roll20Campaign campaign, string outputPath, string system, string worldName)
    {
        _logger.LogInformation("🏗️ Creating Foundry {System} world: {Name} → {OutputPath}", system, worldName, outputPath);

        var worldDir = Path.Combine(outputPath, SanitizeFileName(worldName));
        CreateWorldDirectories(worldDir);

        var worldInfo = new FoundryWorldInfo
        {
            Id = SanitizeFileName(worldName).ToLowerInvariant(),
            Title = worldName,
            Description = $"Converted from Roll20 campaign with {campaign.Campaign.Characters?.Count ?? 0} characters and {campaign.Campaign.Pages?.Count ?? 0} scenes",
            System = system,
            CoreVersion = "12",
            SystemVersion = GetSystemVersion(system),
            CreatedAt = System.DateTimeOffset.UtcNow,
            LastPlayed = System.DateTimeOffset.UtcNow
        };

        var worldJsonPath = Path.Combine(worldDir, "world.json");
        await File.WriteAllTextAsync(worldJsonPath, JsonSerializer.Serialize(worldInfo, _jsonOptions));

        var world = new FoundryWorld
        {
            WorldPath = worldDir,
            WorldInfo = worldInfo,
            Campaign = campaign,
            System = system
        };

        await CreateActorsAsync(world);
        await CreateScenesAsync(world);
        await CreateJournalEntriesAsync(world);
        await ProcessAssetsAsync(world);

        _logger.LogInformation("✅ Foundry world created successfully: {Path}", worldDir);
        return world;
    }

    private void CreateWorldDirectories(string worldDir)
    {
        var subDirs = new[] { "data", "assets", "data/actors", "data/scenes", "data/items", "data/journal", "data/playlists", "data/tables", "assets/maps", "assets/tokens", "assets/audio", "assets/handouts" };
        Directory.CreateDirectory(worldDir);
        foreach (var subDir in subDirs) { Directory.CreateDirectory(Path.Combine(worldDir, subDir)); }
    }

    private async Task CreateActorsAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Characters == null) return;
        var actors = new List<Dnd5eActor>();
        // TODO: Use actual mapper
        await WriteDbFile(world, "actors", actors);
    }

    private async Task CreateScenesAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Pages == null) return;
        var scenes = new List<CoreScene>();
        // TODO: Use actual mapper
        await WriteDbFile(world, "scenes", scenes);
    }

    private async Task CreateJournalEntriesAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Handouts == null) return;
        var journals = new List<CoreJournal>();
        // TODO: Use actual mapper
        await WriteDbFile(world, "journal", journals);
    }

    private async Task WriteDbFile<T>(FoundryWorld world, string dbName, IEnumerable<T> items)
    {
        var dbPath = Path.Combine(world.WorldPath, "data", $"{dbName}.db");
        using var dbWriter = _databaseFactory.CreateWriter(FoundryDatabaseFactory.DatabaseFormat.NeDB);
        await dbWriter.InitializeAsync(dbPath);
        foreach (var item in items)
        {
            var json = JsonSerializer.Serialize(item, _jsonOptions);
            await File.AppendAllTextAsync(dbPath, json + System.Environment.NewLine);
        }
        _logger.LogInformation("✅ Created {Count} {DbName} in NeDB format: {Path}", items.Count(), dbName, dbPath);
    }

    private async Task ProcessAssetsAsync(FoundryWorld world)
    {
        var assetRequests = ExtractAssetRequests(world.Campaign);
        if (!assetRequests.Any()) return;

        var result = await _assetProcessor.ProcessAssetsAsync(assetRequests, world.Campaign.SourceZipPath);
        _logger.LogInformation("✅ Asset processing complete: {Processed} processed, {Errors} errors in {Time:F2}s", result.ProcessedAssets.Count, result.Errors.Count, result.ProcessingTime.TotalSeconds);
        // Correctly access the Exception object's Message property
        foreach (var error in result.Errors) { _logger.LogWarning("Asset error for {Url}: {Error}", error.Request.Roll20Url, error.Error.Message); }
    }
    
    private List<AssetRequest> ExtractAssetRequests(Roll20Campaign campaign)
    {
        var requests = new List<AssetRequest>();
        // TODO: Implement asset extraction logic
        return requests;
    }

    private static string SanitizeFileName(string fileName) => string.Join("_", fileName.Split(Path.GetInvalidFileNameChars()));

    private static string GetSystemVersion(string system) => system switch
    {
        "dnd5e" => "3.3.1",
        "pf2e" => "6.5.3", 
        "pf1e" => "10.7.2",
        _ => "1.0.0"
    };
}

public class FoundryWorldInfo
{
    public required string Id { get; init; }
    public required string Title { get; init; }
    public required string Description { get; init; }
    public required string System { get; init; }
    public required string CoreVersion { get; init; }
    public required string SystemVersion { get; init; }
    public required System.DateTimeOffset CreatedAt { get; init; }
    public required System.DateTimeOffset LastPlayed { get; init; }
}

public class FoundryWorld
{
    public required string WorldPath { get; init; }
    public required FoundryWorldInfo WorldInfo { get; init; }
    public required Roll20Campaign Campaign { get; init; }
    public required string System { get; init; }
}

// Placeholder for a proper factory implementation
public class FoundryDatabaseFactory
{
    public enum DatabaseFormat { NeDB }
    public IDatabaseWriter CreateWriter(DatabaseFormat format) => new NeDbWriter();
}

public interface IDatabaseWriter : System.IDisposable
{
    Task InitializeAsync(string path);
}

public class NeDbWriter : IDatabaseWriter
{
    public Task InitializeAsync(string path) { File.WriteAllText(path, ""); return Task.CompletedTask; }
    public void Dispose() { }
}
