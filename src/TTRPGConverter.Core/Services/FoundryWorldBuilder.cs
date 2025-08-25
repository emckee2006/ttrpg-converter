using System.Text.Json;
using TTRPGConverter.Core.Models.Roll20;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using Roll20Path = TTRPGConverter.Core.Models.Roll20.Path;

namespace TTRPGConverter.Core.Services;

/// <summary>
/// Service for building Foundry VTT world packages from Roll20 campaign data
/// Now uses proper NeDB database format and parallel asset processing
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

    /// <summary>
    /// Create a complete Foundry world from Roll20 campaign data
    /// </summary>
    /// <param name="campaign">Loaded Roll20 campaign</param>
    /// <param name="outputPath">Directory to create Foundry world</param>
    /// <param name="system">Target Foundry system (dnd5e, pf2e, etc.)</param>
    /// <param name="worldName">Name for the Foundry world</param>
    /// <returns>Created world information</returns>
    public async Task<FoundryWorld> CreateWorldAsync(Roll20Campaign campaign, string outputPath, string system, string worldName)
    {
        _logger.LogInformation("🏗️ Creating Foundry {System} world: {Name} → {OutputPath}", system, worldName, outputPath);

        // Create world directory structure
        var worldDir = System.IO.Path.Combine(outputPath, SanitizeFileName(worldName));
        CreateWorldDirectories(worldDir);

        // Generate world.json
        var worldInfo = new FoundryWorldInfo
        {
            Id = SanitizeFileName(worldName).ToLowerInvariant(),
            Title = worldName,
            Description = $"Converted from Roll20 campaign with {campaign.Campaign.Characters?.Count ?? 0} characters and {campaign.Campaign.Pages?.Count ?? 0} scenes",
            System = system,
            CoreVersion = "12",
            SystemVersion = GetSystemVersion(system),
            CreatedAt = DateTimeOffset.UtcNow,
            LastPlayed = DateTimeOffset.UtcNow
        };

        var worldJsonPath = System.IO.Path.Combine(worldDir, "world.json");
        await File.WriteAllTextAsync(worldJsonPath, JsonSerializer.Serialize(worldInfo, _jsonOptions));
        _logger.LogDebug("Created world.json: {Path}", worldJsonPath);

        var world = new FoundryWorld
        {
            WorldPath = worldDir,
            WorldInfo = worldInfo,
            Campaign = campaign,
            System = system
        };

        // Create world structure with proper NeDB databases and parallel asset processing
        await CreateActorsAsync(world);
        await CreateScenesAsync(world);
        await CreateJournalEntriesAsync(world);
        await ProcessAssetsAsync(world);

        _logger.LogInformation("✅ Foundry world created successfully: {Path}", worldDir);
        return world;
    }

    private void CreateWorldDirectories(string worldDir)
    {
        var directories = new[]
        {
            worldDir,
            System.IO.Path.Combine(worldDir, "data"),
            System.IO.Path.Combine(worldDir, "data", "actors"),
            System.IO.Path.Combine(worldDir, "data", "scenes"), 
            System.IO.Path.Combine(worldDir, "data", "items"),
            System.IO.Path.Combine(worldDir, "data", "journal"),
            System.IO.Path.Combine(worldDir, "data", "playlists"),
            System.IO.Path.Combine(worldDir, "data", "tables"),
            System.IO.Path.Combine(worldDir, "assets"),
            System.IO.Path.Combine(worldDir, "assets", "maps"),
            System.IO.Path.Combine(worldDir, "assets", "tokens"),
            System.IO.Path.Combine(worldDir, "assets", "audio"),
            System.IO.Path.Combine(worldDir, "assets", "handouts")
        };

        foreach (var dir in directories)
        {
            Directory.CreateDirectory(dir);
        }

        _logger.LogDebug("Created Foundry world directory structure");
    }

    private async Task CreateActorsAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Characters == null) return;

        _logger.LogInformation("🎭 Converting {Count} characters to Foundry actors", world.Campaign.Campaign.Characters.Count);

        // Create NeDB database for actors
        using var actorsDb = _databaseFactory.CreateWriter(FoundryDatabaseFactory.DatabaseFormat.NeDB);
        var actorsDbPath = System.IO.Path.Combine(world.WorldPath, "data", "actors.db");
        await actorsDb.InitializeAsync(actorsDbPath);

        // Convert characters to Foundry actors
        var actors = new List<Dnd5eActor>();
        var actorId = 1;
        
        foreach (var character in world.Campaign.Campaign.Characters)
        {
            var actor = ConvertCharacterToActor(character, $"actor{actorId:D4}");
            actors.Add(actor);
            actorId++;
        }

        // Bulk insert all actors into database - convert to JSON objects for NeDB
        var actorJsons = actors.Select(a => JsonSerializer.Serialize(a, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase })).ToList();
        
        foreach (var (actor, json) in actors.Zip(actorJsons))
        {
            await File.AppendAllTextAsync(actorsDbPath, json + Environment.NewLine);
        }
        
        _logger.LogInformation("✅ Created {Count} actors in NeDB format: {Path}", 
            actors.Count, actorsDbPath);
    }

    private async Task CreateScenesAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Pages == null) return;

        _logger.LogInformation("🗺️ Converting {Count} pages to Foundry scenes", world.Campaign.Campaign.Pages.Count);

        // Create NeDB database for scenes
        using var scenesDb = _databaseFactory.CreateWriter(FoundryDatabaseFactory.DatabaseFormat.NeDB);
        var scenesDbPath = System.IO.Path.Combine(world.WorldPath, "data", "scenes.db");
        await scenesDb.InitializeAsync(scenesDbPath);

        // Convert pages to Foundry scenes
        var scenes = new List<CoreScene>();
        var sceneId = 1;
        
        foreach (var page in world.Campaign.Campaign.Pages)
        {
            var scene = ConvertPageToScene(page, $"scene{sceneId:D4}");
            scenes.Add(scene);
            sceneId++;
        }

        // Bulk insert all scenes into database - convert to JSON objects for NeDB
        var sceneJsons = scenes.Select(s => JsonSerializer.Serialize(s, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase })).ToList();
        
        foreach (var (scene, json) in scenes.Zip(sceneJsons))
        {
            await File.AppendAllTextAsync(scenesDbPath, json + Environment.NewLine);
        }
        
        _logger.LogInformation("✅ Created {Count} scenes in NeDB format: {Path}", 
            scenes.Count, scenesDbPath);
    }

    private async Task CreateJournalEntriesAsync(FoundryWorld world)
    {
        if (world.Campaign.Campaign.Handouts == null) return;

        _logger.LogInformation("📑 Converting {Count} handouts to journal entries", world.Campaign.Campaign.Handouts.Count);

        // Create NeDB database for journal entries
        using var journalDb = _databaseFactory.CreateWriter(FoundryDatabaseFactory.DatabaseFormat.NeDB);
        var journalDbPath = System.IO.Path.Combine(world.WorldPath, "data", "journal.db");
        await journalDb.InitializeAsync(journalDbPath);

        // Convert handouts to Foundry journal entries
        var journals = new List<CoreJournal>();
        var journalId = 1;
        
        foreach (var handout in world.Campaign.Campaign.Handouts)
        {
            var journal = ConvertHandoutToJournal(handout, $"journal{journalId:D4}");
            journals.Add(journal);
            journalId++;
        }

        // Bulk insert all journal entries into database - convert to JSON objects for NeDB
        var journalJsons = journals.Select(j => JsonSerializer.Serialize(j, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase })).ToList();
        
        foreach (var (journal, json) in journals.Zip(journalJsons))
        {
            await File.AppendAllTextAsync(journalDbPath, json + Environment.NewLine);
        }
        
        _logger.LogInformation("✅ Created {Count} journal entries in NeDB format: {Path}", 
            journals.Count, journalDbPath);
    }

    private async Task ProcessAssetsAsync(FoundryWorld world)
    {
        _logger.LogInformation("🎨 Processing campaign assets with parallel processing");

        // Extract asset requests from all entities
        var assetRequests = ExtractAssetRequests(world.Campaign);
        
        if (!assetRequests.Any())
        {
            _logger.LogInformation("No assets found to process");
            return;
        }

        // Process assets from campaign data
        var campaignZipPath = "campaign.zip"; // TODO: Get actual path from campaign service
        var result = await _assetProcessor.ProcessAssetsAsync(assetRequests, campaignZipPath);
        
        _logger.LogInformation("✅ Asset processing complete: {Processed} processed, {Errors} errors in {Time:F2}s",
            result.ProcessedAssets.Count, result.Errors.Count, result.ProcessingTime.TotalSeconds);
            
        // Log any errors
        foreach (var error in result.Errors)
        {
            _logger.LogWarning("Asset processing error for {Url}: {Error}", 
                error.Request.Roll20Url, error.Error.Message);
        }
    }
    
    private List<AssetRequest> ExtractAssetRequests(Roll20Campaign campaign)
    {
        var requests = new List<AssetRequest>();
        var assetsDir = "assets";
        
        // Extract character/actor assets
        if (campaign.Campaign.Characters != null)
        {
            foreach (var character in campaign.Campaign.Characters)
            {
                if (character.Avatar != null)
                {
                    requests.Add(new AssetRequest
                    {
                        Roll20Url = character.Avatar.ToString(),
                        DestinationPath = System.IO.Path.Combine(assetsDir, "actors", $"{character.Name}_avatar.png"),
                        AssetType = "actors",
                        EntityId = character.Id,
                        EntityType = "character"
                    });
                }
                
                // TODO: Extract token image URL from token data
                var tokenImageUrl = ""; // Extract from token properties
                requests.Add(new AssetRequest
                {
                    Roll20Url = tokenImageUrl,
                    DestinationPath = System.IO.Path.Combine(assetsDir, "actors", $"{character.Name}_token.png"),
                    AssetType = "actors",
                    EntityId = character.Id,
                    EntityType = "character"
                });
            }
        }
        
        // Extract page/scene assets
        if (campaign.Campaign.Pages != null)
        {
            foreach (var page in campaign.Campaign.Pages)
            {
                // TODO: Extract thumbnail from page data
                // if (!string.IsNullOrEmpty(page.ThumbnailUrl))
                // {
                //     var thumbnailUri = await _assetProcessor.ProcessAssetAsync(page.ThumbnailUrl, campaignZipPath);
                //     sceneData.Thumb = thumbnailUri.ToString();
                // }
                // TODO: Extract actual background image from page data
                requests.Add(new AssetRequest
                {
                    Roll20Url = "", // TODO: Get actual background image URL
                    DestinationPath = System.IO.Path.Combine(assetsDir, "scenes", $"{page.Name}_background.jpg"),
                    AssetType = "tiles",
                    EntityId = page.Id,
                    EntityType = "page"
                });
            }
        }
        
        // Extract handout/journal assets
        if (campaign.Campaign.Handouts != null)
        {
            foreach (var handout in campaign.Campaign.Handouts)
            {
                if (!string.IsNullOrEmpty(handout.Avatar))
                {
                    requests.Add(new AssetRequest
                    {
                        Roll20Url = handout.Avatar,
                        DestinationPath = System.IO.Path.Combine(assetsDir, "journal", $"{handout.Name}_image.png"),
                        AssetType = "cards",
                        EntityId = handout.Id,
                        EntityType = "handout"
                    });
                }
            }
        }
        
        return requests;
    }

    private Dnd5eActor ConvertCharacterToActor(Character character, string id)
    {
        return new Dnd5eActor
        {
            _id = id,
            Name = character.Name ?? "Unknown Character",
            Type = Dnd5eactortype.Character,
            // TODO: Use Roll20ToFoundryMapper for full conversion
            System = new SystemModel()
        };
    }

    private CoreScene ConvertPageToScene(Page page, string id)
    {
        return new CoreScene
        {
            _id = id,
            Name = page.Name ?? "Unknown Scene",
            // TODO: Complete scene conversion with background, grid, tokens
        };
    }

    private CoreJournal ConvertHandoutToJournal(Handout handout, string id)
    {
        return new CoreJournal
        {
            _id = id,
            Name = handout.Name ?? "Unknown Journal",
            // TODO: Complete journal conversion with content and pages
        };
    }

    private static string SanitizeFileName(string fileName)
    {
        var invalid = System.IO.Path.GetInvalidFileNameChars();
        return string.Join("_", fileName.Split(invalid, StringSplitOptions.RemoveEmptyEntries));
    }

    private static string GetSystemVersion(string system)
    {
        return system switch
        {
            "dnd5e" => "3.3.1",
            "pf2e" => "6.5.3", 
            "pf1e" => "10.7.2",
            _ => "1.0.0"
        };
    }
}

/// <summary>
/// Foundry world information for world.json
/// </summary>
public class FoundryWorldInfo
{
    public required string Id { get; init; }
    public required string Title { get; init; }
    public required string Description { get; init; }
    public required string System { get; init; }
    public required string CoreVersion { get; init; }
    public required string SystemVersion { get; init; }
    public required DateTimeOffset CreatedAt { get; init; }
    public required DateTimeOffset LastPlayed { get; init; }
}

/// <summary>
/// Container for a created Foundry world
/// </summary>
public class FoundryWorld
{
    public required string WorldPath { get; init; }
    public required FoundryWorldInfo WorldInfo { get; init; }
    public required Roll20Campaign Campaign { get; init; }
    public required string System { get; init; }
}
