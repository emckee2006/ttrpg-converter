using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Compendium;
using TTRPGConverter.Infrastructure.Services.Compendium.CompendiumReaders;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

/// <summary>
/// Responsible for the one-time process of building the compendium cache from source files.
/// </summary>
public class CompendiumCacheBuilder : IDisposable
{
    private static readonly string[] TARGET_SYSTEMS = { "dnd5e", "pf2e", "pf1" };
    private readonly ILogger<CompendiumCacheBuilder> _logger;
    private readonly ILoggerFactory _loggerFactory;
    private readonly bool _verbose;
    private readonly CollisionLogger? _collisionLogger;
    private readonly Dictionary<string, Dictionary<string, CompendiumItem>> _inMemoryCache = new();
    private readonly Dictionary<string, string> _detectedSystems = new();
    private readonly Dictionary<string, string> _packTypes = new();

    public CompendiumCacheBuilder(ILoggerFactory loggerFactory, bool verbose = false, string? collisionLogPath = null)
    {
        _loggerFactory = loggerFactory;
        _logger = _loggerFactory.CreateLogger<CompendiumCacheBuilder>();
        _verbose = verbose;
        _collisionLogger = !string.IsNullOrEmpty(collisionLogPath) ? new CollisionLogger(collisionLogPath) : null;
    }

    public Task LoadUnifiedCacheAsync(IEnumerable<string> compendiumPaths)
    {
        var allLoadedPacks = new List<(CompendiumPackInfo pack, string source, int priority, Dictionary<string, CompendiumItem> packData)>();
        var sourceIndex = 0;

        foreach (var basePath in compendiumPaths)
        {
            var sourceName = Path.GetFileName(basePath);
            if (string.IsNullOrEmpty(sourceName)) continue;

            var packsDir = Path.Combine(basePath, "packs");
            if (!Directory.Exists(packsDir)) continue;

            var packsByName = DiscoverPacksByName(packsDir);
            var moduleSystem = GetDetectedSystem(sourceName);

            foreach (var packName in packsByName.Keys)
            {
                var packPaths = packsByName[packName];
                var levelDbPath = packPaths.FirstOrDefault(p => Directory.Exists(p) && DetectFormat(p) == CompendiumFormat.LevelDB);
                var neDbPath = packPaths.FirstOrDefault(p => File.Exists(p) && Path.GetExtension(p) == ".db");
                var selectedPath = levelDbPath ?? neDbPath;

                if (selectedPath != null)
                {
                    var format = DetectFormat(selectedPath);
                    var packInfo = new CompendiumPackInfo { Name = packName, Path = selectedPath, Format = format };
                    try
                    {
                        var packSpecificSystem = GetDetectedSystem(packName);
                        var finalSystemName = packSpecificSystem ?? moduleSystem;

                        var packData = format == CompendiumFormat.LevelDB
                            ? LoadLevelDBPack(selectedPath, sourceName, _packTypes.GetValueOrDefault(packName), finalSystemName)
                            : GetReader(packInfo.Path).LoadPack(packInfo.Path, sourceName);

                        if (finalSystemName != null)
                        {
                            foreach(var item in packData.Values) { item.System = finalSystemName; }
                        }

                        allLoadedPacks.Add((packInfo, sourceName, sourceIndex, packData));
                    }
                    catch (Exception ex)
                    {
                        _logger.LogError(ex, "❌ Failed to load pack '{PackName}' from {Source}: {Message}", packInfo.Name, sourceName, ex.Message);
                    }
                }
            }
            sourceIndex++;
        }

        var entityConflicts = new Dictionary<string, List<(string source, CompendiumItem item)>>();
        foreach (var (_, source, _, packData) in allLoadedPacks)
        {
            foreach (var (_, item) in packData)
            {
                var entityKey = $"{item.System}:{item.Type}:{item.Name}".ToLowerInvariant();
                if (!_inMemoryCache.ContainsKey(item.Type))
                {
                    _inMemoryCache[item.Type] = new Dictionary<string, CompendiumItem>();
                }

                var existing = _inMemoryCache[item.Type].GetValueOrDefault(item.Name);
                if (existing == null)
                {
                    _inMemoryCache[item.Type][item.Name] = item;
                }
                else
                {
                    if (!entityConflicts.TryGetValue(entityKey, out var value))
                    {
                        value = [(existing.SourceFile ?? "unknown", existing)];
                        entityConflicts[entityKey] = value;
                    }

                    value.Add((source, item));

                    var (winner, losers) = ChooseWinningEntity(value);
                    _inMemoryCache[item.Type][item.Name] = winner.item;

                    foreach (var loser in losers)
                    {
                        _collisionLogger?.LogCollision(winner.item, loser.item);
                    }
                }
            }
        }
        return Task.CompletedTask;
    }

    public Dictionary<string, CompendiumItem> GetAllEntities()
    {
        return _inMemoryCache.SelectMany(kvp => kvp.Value).ToDictionary(kvp => kvp.Key, kvp => kvp.Value);
    }

    public string? GetDetectedSystem(string sourceName) => _detectedSystems.GetValueOrDefault(sourceName);
    public IEnumerable<KeyValuePair<string, string>> GetAllDetectedSystems() => _detectedSystems;

    private ((string source, CompendiumItem item) winner, List<(string source, CompendiumItem item)> losers) ChooseWinningEntity(List<(string source, CompendiumItem item)> candidates)
    {
        var priorityOrder = new[] { "dnd5e", "dnd-players-handbook", "dnd-dungeon-masters-guide", "dnd-monsters-manual", "dnd-", "" };
        (string source, CompendiumItem item) winner = default;
        foreach (var priority in priorityOrder)
        {
            var match = candidates.FirstOrDefault(c => c.source.StartsWith(priority, StringComparison.OrdinalIgnoreCase));
            if (match != default) { winner = match; break; }
        }
        winner = winner == default ? candidates.First() : winner;
        var losers = candidates.Where(c => c != winner).ToList();
        return (winner, losers);
    }

    private Dictionary<string, List<string>> DiscoverPacksByName(string packsDir)
    {
        var packsByName = new Dictionary<string, List<string>>();
        var manifestPacks = TryReadManifestPacks(packsDir);
        if (manifestPacks.Any())
        {
            foreach (var pack in manifestPacks)
            {
                var packPath = Path.Combine(Path.GetDirectoryName(packsDir)!, pack.Path);
                if (Directory.Exists(packPath) || File.Exists(packPath))
                {
                    if (!packsByName.ContainsKey(pack.Name)) packsByName[pack.Name] = new List<string>();
                    packsByName[pack.Name].Add(packPath);
                    _packTypes[pack.Name] = pack.Type;
                }
                else if (pack.Path.EndsWith(".db"))
                {
                    var levelDbPath = packPath.Substring(0, packPath.Length - 3);
                    if (Directory.Exists(levelDbPath))
                    {
                        if (!packsByName.ContainsKey(pack.Name)) packsByName[pack.Name] = new List<string>();
                        packsByName[pack.Name].Add(levelDbPath);
                        _packTypes[pack.Name] = pack.Type;
                    }
                }
            }
        }
        else
        {
            foreach (var entry in Directory.GetFileSystemEntries(packsDir))
            {
                string packName;
                if (Directory.Exists(entry)) packName = Path.GetFileName(entry);
                else if (Path.GetExtension(entry) == ".db") packName = Path.GetFileNameWithoutExtension(entry);
                else continue;
                if (!packsByName.ContainsKey(packName)) packsByName[packName] = new List<string>();
                packsByName[packName].Add(entry);
            }
        }
        return packsByName;
    }

    private List<ManifestPack> TryReadManifestPacks(string packsDir)
    {
        var baseDir = Path.GetDirectoryName(packsDir);
        if (baseDir == null) return new List<ManifestPack>();
        var manifestFiles = new[] { "system.json", "module.json" };
        foreach (var manifestFile in manifestFiles)
        {
            var manifestPath = Path.Combine(baseDir, manifestFile);
            if (!File.Exists(manifestPath)) continue;
            try
            {
                var jsonContent = File.ReadAllText(manifestPath);
                var manifest = JsonSerializer.Deserialize<FoundryManifest>(jsonContent, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
                if (manifest?.Packs != null)
                {
                    var detectedSystem = DetectSystemFromManifest(manifest);
                    if (!string.IsNullOrEmpty(detectedSystem))
                    {
                        var sourceKey = Path.GetFileName(baseDir) ?? baseDir;
                        _detectedSystems[sourceKey] = detectedSystem;
                    }
                    foreach (var pack in manifest.Packs.Where(p => !string.IsNullOrEmpty(p.System)))
                    {
                        _detectedSystems[pack.Name] = pack.System!;
                    }
                    return FilterPacksBySystem(manifest.Packs, manifestFile);
                }
            }
            catch (Exception ex) { _logger.LogWarning(ex, "⚠️ Failed to parse {ManifestFile}", manifestFile); }
        }
        return new List<ManifestPack>();
    }

    private string? DetectSystemFromManifest(FoundryManifest manifest)
    {
        var systems = manifest.Relationships?.Systems?.Select(s => s.Id).ToList();
        if (systems != null && systems.Any())
        {
            if (systems.Count > 1) return "Multi-System";
            var systemId = systems.First();
            return systemId switch
            {
                "pf1" => "Pathfinder 1e",
                "pf2e" => "Pathfinder 2e",
                "dnd5e" => "D&D 5e",
                _ => systemId
            };
        }
        var title = manifest.Title?.ToLowerInvariant();
        if (!string.IsNullOrEmpty(title))
        {
            if (title.Contains("pathfinder 1") || title.Contains("pf1")) return "Pathfinder 1e";
            if (title.Contains("pathfinder 2") || title.Contains("pf2e")) return "Pathfinder 2e";
            if (title.Contains("d&d 5e") || title.Contains("dnd5e")) return "D&D 5e";
        }
        return null;
    }

    private List<ManifestPack> FilterPacksBySystem(List<ManifestPack> packs, string manifestFile)
    {
        return packs.Where(pack =>
        {
            if (!string.IsNullOrEmpty(pack.System)) return TARGET_SYSTEMS.Contains(pack.System, StringComparer.OrdinalIgnoreCase);
            if (pack.Systems != null && pack.Systems.Any(s => TARGET_SYSTEMS.Contains(s, StringComparer.OrdinalIgnoreCase))) return true;
            if (string.IsNullOrEmpty(pack.System) && pack.Systems == null) return manifestFile == "system.json" || !HasAnySystemSpecificPacks(packs);
            return false;
        }).ToList();
    }

    private bool HasAnySystemSpecificPacks(List<ManifestPack> packs) => packs.Any(p => !string.IsNullOrEmpty(p.System) || (p.Systems != null && p.Systems.Any()));

    private ICompendiumReader GetReader(string packPath)
    {
        if (packPath.EndsWith(".db")) return new NeDbCompendiumReader(_loggerFactory.CreateLogger<NeDbCompendiumReader>());
        if (Directory.Exists(packPath)) { 
            var plutoReader = new PlutoniumJsonReader(_loggerFactory.CreateLogger<PlutoniumJsonReader>());
            if (plutoReader.SupportsFormat(packPath)) return plutoReader;
        }
        throw new NotSupportedException($"No compendium reader supports format: {packPath}");
    }

    private Dictionary<string, CompendiumItem> LoadLevelDBPack(string packPath, string sourceName, string? packType, string? systemName)
    {
        try
        {
            var bridge = new FoundryCliBridge(_loggerFactory.CreateLogger<FoundryCliBridge>(), _verbose);
            return bridge.LoadPackWithContext(packPath, sourceName, packType, systemName);
        }
        catch (Exception ex)
        {
            _logger.LogDebug(ex, "LevelDB CLI bridge failed for {PackName}. This is expected and will be skipped.", Path.GetFileName(packPath));
            return new Dictionary<string, CompendiumItem>();
        }
    }

    private CompendiumFormat DetectFormat(string path)
    {
        if (Directory.Exists(path)) {
            if (new[] { "CURRENT", "LOCK" }.All(f => File.Exists(Path.Combine(path, f)))) return CompendiumFormat.LevelDB;
            if (new PlutoniumJsonReader(_loggerFactory.CreateLogger<PlutoniumJsonReader>()).SupportsFormat(path)) return CompendiumFormat.PlutoJSON;
        }
        if (File.Exists(path) && path.EndsWith(".db")) return CompendiumFormat.NeDB;
        return CompendiumFormat.Unknown;
    }

    public void Dispose() { _collisionLogger?.Dispose(); GC.SuppressFinalize(this); }
}

public class FoundryManifest { public List<ManifestPack>? Packs { get; set; } public string? Title { get; set; } public ManifestRelationships? Relationships { get; set; } }
public class ManifestPack { public string Name { get; set; } = ""; public string Label { get; set; } = ""; public string Path { get; set; } = ""; public string Type { get; set; } = ""; public string? System { get; set; } public List<string>? Systems { get; set; } }
public class ManifestRelationships { public List<ManifestSystemRelationship>? Systems { get; set; } }
public class ManifestSystemRelationship { public string Id { get; set; } = ""; }
public enum CompendiumFormat { Unknown, NeDB, LevelDB, PlutoJSON }
public class CompendiumPackInfo { public string Name { get; set; } = ""; public string Path { get; set; } = ""; public CompendiumFormat Format { get; set; } }
