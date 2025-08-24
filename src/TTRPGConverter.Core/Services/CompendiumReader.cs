
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Caching.Memory;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Logging.Abstractions;
using TTRPGConverter.Core.Entities;

/// <summary>
/// Represents a compendium item from any database format
/// </summary>
public record CompendiumItem
{
    public required string Id { get; init; }
    public required string Name { get; init; }
    public required string Type { get; init; }
    public required JsonDocument Data { get; init; }
    public required string SourceFormat { get; init; }
    public required string SourceFile { get; init; }
    public string? Description { get; init; }
    public string? Source { get; init; }
    public string? Rarity { get; init; }
    public DateTime LoadedAt { get; init; } = DateTime.UtcNow;
}

/// <summary>
/// Abstract interface for reading compendium databases
/// </summary>
public interface ICompendiumReader
{
    string FormatName { get; }
    bool SupportsFormat(string packPath);
    Dictionary<string, CompendiumItem> LoadPack(string packPath);
    Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName);
}

/// <summary>
/// NeDB compendium reader - handles JSON Lines format (.db files)
/// </summary>
public class NeDBCompendiumReader : ICompendiumReader
{
    private readonly ILogger<NeDBCompendiumReader> _logger;
    private readonly JsonSerializerOptions _jsonOptions;

    public string FormatName => "NeDB (Legacy Foundry)";

    public NeDBCompendiumReader(ILogger<NeDBCompendiumReader> logger)
    {
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            AllowTrailingCommas = true,
            ReadCommentHandling = JsonCommentHandling.Skip
        };
    }

    public bool SupportsFormat(string packPath)
    {
        return File.Exists(packPath) && Path.GetExtension(packPath).Equals(".db", StringComparison.OrdinalIgnoreCase);
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName)
    {
        return LoadPack(packPath);
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath)
    {
        if (!SupportsFormat(packPath))
            throw new ArgumentException($"Unsupported NeDB format: {packPath}");

        var items = new Dictionary<string, CompendiumItem>();
        
        try
        {
            var lines = File.ReadAllLines(packPath);
            
            for (int lineNum = 0; lineNum < lines.Length; lineNum++)
            {
                var line = lines[lineNum].Trim();
                if (string.IsNullOrEmpty(line))
                    continue;

                try
                {
                    var jsonDoc = JsonDocument.Parse(line, new JsonDocumentOptions 
                    { 
                        AllowTrailingCommas = true,
                        CommentHandling = JsonCommentHandling.Skip
                    });

                    var root = jsonDoc.RootElement;
                    
                    var itemId = root.TryGetProperty("_id", out var idProp) ? 
                        idProp.GetString() ?? $"item_{lineNum}" : $"item_{lineNum}";
                    
                    var name = root.TryGetProperty("name", out var nameProp) ? 
                        nameProp.GetString() ?? "Unknown" : "Unknown";
                    
                    var type = root.TryGetProperty("type", out var typeProp) ? 
                        typeProp.GetString() ?? "unknown" : "unknown";

                    items[itemId] = new CompendiumItem
                    {
                        Id = itemId,
                        Name = name,
                        Type = type,
                        Data = jsonDoc,
                        SourceFormat = "nedb",
                        SourceFile = packPath
                    };
                }
                catch (JsonException ex)
                {
                    _logger.LogWarning("Invalid JSON in {File} line {LineNum}: {Error}", 
                        packPath, lineNum + 1, ex.Message);
                    continue;
                }
            }

            _logger.LogInformation("Loaded {Count} items from NeDB pack: {Pack}", 
                items.Count, Path.GetFileName(packPath));
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Failed to load NeDB pack: {Pack}", packPath);
            throw;
        }

        return items;
    }

    public void Dispose()
    {
        GC.SuppressFinalize(this);
    }
}



/// <summary>
/// FoundryCLI bridge for LevelDB conversion - uses Node.js interop
/// </summary>
public class FoundryCLIBridge : ICompendiumReader
{
    private readonly ILogger<FoundryCLIBridge> _logger;
    private readonly string _foundryCliPath;

    public string FormatName => "FoundryCLI Bridge (Node.js)";

    public FoundryCLIBridge(ILogger<FoundryCLIBridge> logger, string foundryCliPath = "fvtt")
    {
        _logger = logger;
        _foundryCliPath = foundryCliPath;
    }

    public bool SupportsFormat(string packPath)
    {
        // Can handle both NeDB and LevelDB through conversion
        return Directory.Exists(packPath) || 
               (File.Exists(packPath) && Path.GetExtension(packPath) == ".db");
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath)
    {
        // Simulate CLI bridge - not implemented in this version
        _logger.LogWarning("FoundryCLI bridge not implemented - using empty dataset");
        return new Dictionary<string, CompendiumItem>();
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName)
    {
        throw new NotImplementedException();
    }

    public void Dispose()
    {
        GC.SuppressFinalize(this);
    }
}

/// <summary>
/// Factory and manager for compendium readers with unified entity cache and deduplication
/// </summary>
public class CompendiumManager
{
    private static readonly string[] TARGET_SYSTEMS = { "dnd5e", "pf2e", "pf1" };
    private const string CACHE_NOT_LOADED_MESSAGE = "Unified cache not loaded. Call LoadUnifiedCacheAsync first.";
    
    private readonly ILogger<CompendiumManager> _logger;
    private readonly IMemoryCache _cache;
    private readonly bool _verbose;
    private readonly Dictionary<string, string> _detectedSystems = new(); // Track detected systems by source
    private readonly Dictionary<string, string> _packTypes = new(); // Track pack types by pack name
    
    // Unified entity cache: entityType -> entityName -> best CompendiumItem
    private readonly Dictionary<string, Dictionary<string, CompendiumItem>> _unifiedEntityCache;
    private readonly Dictionary<string, Dictionary<string, CompendiumItem>> _packCache;
    private bool _globalCacheLoaded = false;

    public CompendiumManager(ILogger<CompendiumManager> logger, IMemoryCache cache, bool verbose = false)
    {
        _logger = logger;
        _cache = cache;
        _verbose = verbose;
        _unifiedEntityCache = new Dictionary<string, Dictionary<string, CompendiumItem>>();
        _packCache = new Dictionary<string, Dictionary<string, CompendiumItem>>();
    }

    /// <summary>
    /// Load unified compendium cache from multiple sources with entity-level deduplication
    /// </summary>
    public Task LoadUnifiedCacheAsync(IEnumerable<string> compendiumPaths, bool isDryRun = false, CancellationToken cancellationToken = default)
    {
        if (_globalCacheLoaded) return Task.CompletedTask;
        
        if (isDryRun)
        {
            Console.WriteLine("=== DRY RUN MODE: Commands that would be executed ===\n");
            PrintDryRunCommands(compendiumPaths);
            Console.WriteLine("=== End of dry run commands ===");
            return Task.CompletedTask;
        }

        _logger.LogInformation("🔄 Loading unified compendium cache from {Count} sources", compendiumPaths.Count());

        var allLoadedPacks = new List<(CompendiumPackInfo pack, string source, int priority, Dictionary<string, CompendiumItem> packData)>();
        var sourceIndex = 0;

        // Process each source and load packs immediately
        foreach (var basePath in compendiumPaths)
        {
            var sourceName = Path.GetFileName(basePath);
            
            var packsDir = Path.Combine(basePath, "packs");
            _logger.LogInformation("🔍 Processing source: {Source} at {PacksDir}", sourceName, packsDir);
            
            if (!Directory.Exists(packsDir)) 
            {
                _logger.LogWarning("❌ Packs directory not found: {PacksDir}", packsDir);
                continue;
            }

            var packsByName = DiscoverPacksByName(packsDir);
            _logger.LogInformation("📦 Found {Count} pack types in {Source}", packsByName.Count, sourceName);
            
            // Load each pack immediately after finding it
            foreach (var packName in packsByName.Keys)
            {
                var packPaths = packsByName[packName];
                var levelDbPath = packPaths.FirstOrDefault(p => Directory.Exists(p) && DetectFormat(p) == CompendiumFormat.LevelDB);
                var neDbPath = packPaths.FirstOrDefault(p => File.Exists(p) && Path.GetExtension(p) == ".db");
                var selectedPath = levelDbPath ?? neDbPath;
                
                if (selectedPath != null)
                {
                    var format = DetectFormat(selectedPath);
                    var packInfo = new CompendiumPackInfo 
                    { 
                        Name = packName, 
                        Path = selectedPath, 
                        Format = format 
                    };
                    
                    _logger.LogInformation("✅ Found pack '{PackName}' ({Format}) from {Source}: {Path}", 
                        packName, format, sourceName, selectedPath);
                    
                    // Load immediately
                    try
                    {
                        if (format == CompendiumFormat.LevelDB)
                        {
                            // Extract filesystem pack name for CLI - CLI expects directory name, not manifest name
                            var filesystemPackName = Path.GetFileName(selectedPath);
                            _packTypes.TryGetValue(packName, out var packType);
                            var packData = LoadLevelDBPack(filesystemPackName, sourceName, packType);
                            _logger.LogInformation("📄 Loaded {Count} items from pack '{PackName}'", 
                                packData.Count, packInfo.Name);
                                
                            allLoadedPacks.Add((packInfo, sourceName, sourceIndex, packData));
                        }
                        else
                        {
                            var reader = GetReader(packInfo.Path);
                            var packData = reader.LoadPack(packInfo.Path);
                            
                            _logger.LogInformation("📄 Loaded {Count} items from pack '{PackName}'", 
                                packData.Count, packInfo.Name);
                                
                            allLoadedPacks.Add((packInfo, sourceName, sourceIndex, packData));
                        }
                    }
                    catch (Exception ex)
                    {
                        _logger.LogError(ex, "❌ Failed to load pack '{PackName}' from {Source}: {Message}", 
                            packInfo.Name, sourceName, ex.Message);
                        allLoadedPacks.Add((packInfo, sourceName, sourceIndex, new Dictionary<string, CompendiumItem>()));
                    }
                }
                else
                {
                    _logger.LogWarning("⚠️ No valid format found for pack '{PackName}' in {Source}", 
                        packName, sourceName);
                }
            }
            sourceIndex++;
        }

        // All packs already loaded during discovery
        var loadedPacks = allLoadedPacks.ToArray();

        // Build unified entity cache with deduplication
        var entityConflicts = new Dictionary<string, List<(string source, CompendiumItem item)>>();

        foreach (var (pack, source, priority, packData) in loadedPacks)
        {
            foreach (var (itemId, item) in packData)
            {
                var entityKey = $"{item.Type}:{item.Name}".ToLowerInvariant();
                
                // Initialize entity type cache if needed
                if (!_unifiedEntityCache.ContainsKey(item.Type))
                {
                    _unifiedEntityCache[item.Type] = new Dictionary<string, CompendiumItem>();
                }

                var existing = _unifiedEntityCache[item.Type].ContainsKey(item.Name) ? 
                    _unifiedEntityCache[item.Type][item.Name] : null;

                if (existing == null)
                {
                    // First occurrence - add to cache
                    _unifiedEntityCache[item.Type][item.Name] = item;
                }
                else
                {
                    // Conflict detected - apply priority rules
                    if (!entityConflicts.ContainsKey(entityKey))
                    {
                        entityConflicts[entityKey] = new List<(string, CompendiumItem)>
                        {
                            (existing.SourceFile ?? "unknown", existing)
                        };
                    }
                    entityConflicts[entityKey].Add((source, item));

                    // Priority rules: System (dnd5e) > Official modules > Third-party
                    var winner = ChooseWinningEntity(entityConflicts[entityKey]);
                    _unifiedEntityCache[item.Type][item.Name] = winner.item;
                }
            }
        }

        var totalEntities = _unifiedEntityCache.Values.Sum(dict => dict.Count);
        if (entityConflicts.Count > 0)
        {
            _logger.LogDebug("Entity conflicts resolved: {Conflicts}", 
                string.Join(", ", entityConflicts.Keys.Take(5)));
        }
        
        _globalCacheLoaded = true;
        _logger.LogInformation("✅ Unified cache loaded: {Entities} entities, {Conflicts} conflicts resolved", 
            _unifiedEntityCache.Values.Sum(d => d.Count), entityConflicts.Count);
            
        return Task.CompletedTask;
    }
    
    /// <summary>
    /// Gets the detected game system for a given source (module/system name)
    /// </summary>
    public string? GetDetectedSystem(string sourceName)
    {
        _detectedSystems.TryGetValue(sourceName, out var system);
        if (_verbose)
        {
            _logger.LogDebug("🔍 GetDetectedSystem({SourceName}) → {System} (Available: {Available})", 
                sourceName, system ?? "null", string.Join(", ", _detectedSystems.Keys));
        }
        return system;
    }
    
    /// <summary>
    /// Gets all detected systems for iteration
    /// </summary>
    public IEnumerable<KeyValuePair<string, string>> GetAllDetectedSystems()
    {
        return _detectedSystems;
    }

    private void PrintDryRunCommands(IEnumerable<string> compendiumPaths)
    {
        // Process each source and discover packs
        foreach (var basePath in compendiumPaths)
        {
            var sourceName = Path.GetFileName(basePath);
            
            var packsDir = Path.Combine(basePath, "packs");
            
            if (!Directory.Exists(packsDir)) 
            {
                Console.WriteLine($"❌ Packs directory not found: {packsDir}");
                continue;
            }

            // Discover pack files  
            var packsByName = DiscoverPacksByName(packsDir);
            
            // For each discovered pack, show what command would be executed
            foreach (var packName in packsByName.Keys)
            {
                var packPaths = packsByName[packName];
                var levelDbPath = packPaths.FirstOrDefault(p => Directory.Exists(p) && DetectFormat(p) == CompendiumFormat.LevelDB);
                
                if (levelDbPath != null)
                {
                    // This pack would use LevelDB loading via CLI
                    var (packageId, packageType) = DeterminePackageContext(sourceName);
                    
                    // Find the root project directory for script path
                    var currentDir = Directory.GetCurrentDirectory();
                    var rootDir = currentDir;
                    while (rootDir != null && !File.Exists(Path.Combine(rootDir, "package.json")))
                    {
                        rootDir = Directory.GetParent(rootDir)?.FullName;
                    }
                    
                    var scriptPath = rootDir != null ? Path.Combine(rootDir, "foundry-extract.ps1") : "foundry-extract.ps1";
                    
                    Console.WriteLine($"For pack: {packName} (from {sourceName})");
                    Console.WriteLine($@"  Command: pwsh.exe -ExecutionPolicy Bypass -NoProfile -NonInteractive -File ""{scriptPath}"" -PackName ""{packName}"" -PackageId ""{packageId}"" -PackageType ""{packageType}"" ");
                    Console.WriteLine("  Will extract to random temp directory");
                    Console.WriteLine();
                }
            }
        }
    }

    /// <summary>
    /// Choose winning entity based on source priority
    /// </summary>
    private (string source, CompendiumItem item) ChooseWinningEntity(List<(string source, CompendiumItem item)> candidates)
    {
        // Priority order: dnd5e system > official D&D modules > other modules > third-party
        var priorityOrder = new[]
        {
            "dnd5e",                        // Core system
            "dnd-players-handbook",         // Official WotC
            "dnd-dungeon-masters-guide", 
            "dnd-monsters-manual",
            "dnd-",                         // Other official D&D modules
            ""
        };

        foreach (var priority in priorityOrder)
        {
            var match = candidates.FirstOrDefault(c => c.source.StartsWith(priority, StringComparison.OrdinalIgnoreCase));
            if (match != default)
            {
                return match;
            }
        }

        // Fallback to first candidate
        return candidates.First();
    }

    /// <summary>
    /// Discover all pack paths grouped by pack name
    /// </summary>
    private Dictionary<string, List<string>> DiscoverPacksByName(string packsDir)
    {
        var packsByName = new Dictionary<string, List<string>>();
        
        // Try to read manifest file to get official pack definitions
        var manifestPacks = TryReadManifestPacks(packsDir);
        
        if (manifestPacks.Any())
        {
            _logger.LogInformation("📋 Using manifest-defined packs ({Count} found)", manifestPacks.Count);
            
            foreach (var pack in manifestPacks)
            {
                var packPath = Path.Combine(Path.GetDirectoryName(packsDir)!, pack.Path);
                
                // Check if the manifest path exists as-is
                if (Directory.Exists(packPath) || File.Exists(packPath))
                {
                    if (!packsByName.ContainsKey(pack.Name))
                        packsByName[pack.Name] = new List<string>();
                    packsByName[pack.Name].Add(packPath);
                    _packTypes[pack.Name] = pack.Type;
                    
                    _logger.LogDebug("✅ Found manifest pack: {Name} ({Type}) at {Path}", 
                        pack.Name, pack.Type, packPath);
                }
                // Handle case where manifest lists .db file but actual format is LevelDB directory
                else if (pack.Path.EndsWith(".db"))
                {
                    var levelDbPath = packPath.Substring(0, packPath.Length - 3); // Remove .db extension
                    if (Directory.Exists(levelDbPath))
                    {
                        if (!packsByName.ContainsKey(pack.Name))
                            packsByName[pack.Name] = new List<string>();
                        packsByName[pack.Name].Add(levelDbPath);
                        _packTypes[pack.Name] = pack.Type;
                        
                        _logger.LogDebug("✅ Found manifest pack (LevelDB format): {Name} ({Type}) at {Path}", 
                            pack.Name, pack.Type, levelDbPath);
                    }
                    else
                    {
                        _logger.LogWarning("❌ Manifest pack not found: {Name} at {Path} (also checked LevelDB path)", 
                            pack.Name, packPath);
                    }
                }
                else
                {
                    _logger.LogWarning("❌ Manifest pack not found: {Name} at {Path}", 
                        pack.Name, packPath);
                }
            }
        }
        else
        {
            _logger.LogInformation("📁 No manifest found, using filesystem discovery");
            
            // Fallback to filesystem scanning
            foreach (var entry in Directory.GetFileSystemEntries(packsDir))
            {
                string packName;
                
                if (Directory.Exists(entry))
                {
                    packName = Path.GetFileName(entry);
                }
                else if (Path.GetExtension(entry) == ".db")
                {
                    packName = Path.GetFileNameWithoutExtension(entry);
                }
                else
                {
                    continue;
                }
                
                if (!packsByName.ContainsKey(packName))
                    packsByName[packName] = new List<string>();
                packsByName[packName].Add(entry);
            }
        }
        
        return packsByName;
    }

    /// <summary>
    /// Attempts to read pack definitions from system.json or module.json manifest
    /// </summary>
    private List<ManifestPack> TryReadManifestPacks(string packsDir)
    {
        var baseDir = Path.GetDirectoryName(packsDir);
        if (baseDir == null) return new List<ManifestPack>();
        
        // Try system.json first, then module.json
        var manifestFiles = new[] { "system.json", "module.json" };
        
        foreach (var manifestFile in manifestFiles)
        {
            var manifestPath = Path.Combine(baseDir, manifestFile);
            if (!File.Exists(manifestPath)) continue;
            
            try
            {
                var jsonContent = File.ReadAllText(manifestPath);
                var manifest = JsonSerializer.Deserialize<FoundryManifest>(jsonContent, new JsonSerializerOptions 
                { 
                    PropertyNameCaseInsensitive = true 
                });
                
                if (manifest?.Packs != null)
                {
                    var allPacks = manifest.Packs.Count;
                    
                    // Detect system from manifest relationships
                    var detectedSystem = DetectSystemFromManifest(manifest);
                    if (!string.IsNullOrEmpty(detectedSystem))
                    {
                        _logger.LogInformation("🎯 Detected system from manifest: {System}", detectedSystem);
                        // Store the detected system for this source directory
                        var sourceKey = Path.GetFileName(baseDir) ?? baseDir;
                        _detectedSystems[sourceKey] = detectedSystem;
                        _logger.LogDebug("📌 Stored system detection: {SourceKey} → {System}", sourceKey, detectedSystem);
                    }
                    else
                    {
                        _logger.LogDebug("❓ No system detected from manifest for {BaseDir}", baseDir);
                    }
                    
                    // Filter packs by system if this is a multi-system module
                    var relevantPacks = FilterPacksBySystem(manifest.Packs, manifestFile);
                    
                    // Store system information per pack for multi-system modules
                    foreach (var pack in relevantPacks)
                    {
                        if (_verbose)
                        {
                            _logger.LogDebug("📦 Processing pack: {PackName}, System: {System}", pack.Name, pack.System ?? "null");
                        }
                        
                        if (!string.IsNullOrEmpty(pack.System))
                        {
                            var packSystem = pack.System switch
                            {
                                "pf1" => "Pathfinder 1e",
                                "pf2e" => "Pathfinder 2e",
                                "dnd5e" => "D&D 5e",
                                _ => pack.System
                            };
                            _detectedSystems[pack.Name] = packSystem;
                            
                            // Also store under filesystem-derived pack name for lookup compatibility
                            // e.g., "features-dnd5e" (manifest) maps to "feats-dnd5e" (filesystem)
                            var filesystemPackName = Path.GetFileName(pack.Path);
                            if (!string.IsNullOrEmpty(filesystemPackName) && filesystemPackName != pack.Name)
                            {
                                _detectedSystems[filesystemPackName] = packSystem;
                                if (_verbose)
                                {
                                    _logger.LogDebug("Also stored pack system: {FilesystemPackName} → {System} (alias for {ManifestPackName})", 
                                        filesystemPackName, packSystem, pack.Name);
                                }
                            }
                            if (_verbose)
                            {
                                _logger.LogDebug(" Stored pack system: {PackName} → {System}", pack.Name, packSystem);
                            }
                        }
                    }
                    
                    _logger.LogInformation("📋 Read manifest: {ManifestFile} with {Total} packs ({Relevant} for current systems)", 
                        manifestFile, allPacks, relevantPacks.Count);
                    
                    return relevantPacks;
                }
            }
            catch (Exception ex)
            {
                _logger.LogWarning("⚠️ Failed to parse {ManifestFile}: {Error}", manifestFile, ex.Message);
            }
        }
        
        return new List<ManifestPack>();
    }

    /// <summary>
    /// Detects the game system from manifest relationships
    /// </summary>
    private string? DetectSystemFromManifest(FoundryManifest manifest)
    {
        // Check relationships.systems for system compatibility
        var systems = manifest.Relationships?.Systems?.Select(s => s.Id).ToList();
        
        if (systems != null && systems.Any())
        {
            if (_verbose)
            {
                _logger.LogDebug("📋 Manifest systems: {Systems}", string.Join(", ", systems));
            }
            
            // Handle multi-system modules
            if (systems.Count > 1)
            {
                if (_verbose)
                {
                    _logger.LogDebug("🔄 Multi-system module detected: {Systems}", string.Join(", ", systems));
                }
                // For multi-system modules, we'll rely on per-pack system detection
                return "Multi-System";
            }
            
            // Single system module
            var systemId = systems.First();
            return systemId switch
            {
                "pf1" => "Pathfinder 1e",
                "pf2e" => "Pathfinder 2e", 
                "dnd5e" => "D&D 5e",
                _ => systemId // Return the raw system ID if we don't have a mapping
            };
        }

        // Fallback: Check title for system hints
        var title = manifest.Title?.ToLowerInvariant();
        if (!string.IsNullOrEmpty(title))
        {
            if (title.Contains("pathfinder 1") || title.Contains("pf1"))
                return "Pathfinder 1e";
            if (title.Contains("pathfinder 2") || title.Contains("pf2e"))
                return "Pathfinder 2e";
            if (title.Contains("d&d 5e") || title.Contains("dnd5e"))
                return "D&D 5e";
        }

        return null;
    }

    /// <summary>
    /// Filters packs to only include those relevant to supported systems (D&D 5e, PF2e, PF1e) or system-agnostic packs
    /// </summary>
    private List<ManifestPack> FilterPacksBySystem(List<ManifestPack> packs, string manifestFile)
    {
        var relevantPacks = new List<ManifestPack>();
        
        foreach (var pack in packs)
        {
            var isRelevant = false;
            var systemInfo = "";
            
            // Check if pack is for any of our target systems
            if (!string.IsNullOrEmpty(pack.System))
            {
                if (TARGET_SYSTEMS.Contains(pack.System, StringComparer.OrdinalIgnoreCase))
                {
                    isRelevant = true;
                    systemInfo = $" (system: {pack.System})";
                }
            }
            // Check if pack supports multiple systems including any of our targets
            else if (pack.Systems != null && pack.Systems.Any(s => TARGET_SYSTEMS.Contains(s, StringComparer.OrdinalIgnoreCase)))
            {
                isRelevant = true;
                systemInfo = $" (systems: {string.Join(", ", pack.Systems)})";
            }
            // If no system specified, assume it's system-agnostic (for system.json) or check manifest type
            else if (string.IsNullOrEmpty(pack.System) && pack.Systems == null)
            {
                // For system.json, include all packs
                // For module.json, only include if no system filtering is present
                isRelevant = manifestFile == "system.json" || !HasAnySystemSpecificPacks(packs);
                systemInfo = isRelevant ? " (system-agnostic)" : " (skipped - no system match)";
            }
            
            if (isRelevant)
            {
                relevantPacks.Add(pack);
                _logger.LogDebug("✅ Including pack: {Name} ({Type}){SystemInfo}", 
                    pack.Name, pack.Type, systemInfo);
            }
            else
            {
                _logger.LogDebug("⏭️ Skipping pack: {Name} ({Type}){SystemInfo}", 
                    pack.Name, pack.Type, systemInfo);
            }
        }
        
        return relevantPacks;
    }

    /// <summary>
    /// Checks if any packs in the list have system-specific definitions
    /// </summary>
    private bool HasAnySystemSpecificPacks(List<ManifestPack> packs)
    {
        return packs.Any(p => !string.IsNullOrEmpty(p.System) || (p.Systems != null && p.Systems.Any()));
    }

    /// <summary>
    /// Represents a FoundryVTT system.json or module.json manifest file
    /// </summary>
    public class FoundryManifest
    {
        public List<ManifestPack>? Packs { get; set; }
        public string? Id { get; set; }
        public string? Title { get; set; }
        public string? Version { get; set; }
        public ManifestRelationships? Relationships { get; set; }
    }

    /// <summary>
    /// Represents relationships in a FoundryVTT manifest
    /// </summary>
    public class ManifestRelationships
    {
        public List<ManifestSystemRelationship>? Systems { get; set; }
        public List<ManifestModuleRelationship>? Modules { get; set; }
    }

    /// <summary>
    /// Represents a system relationship in a FoundryVTT manifest
    /// </summary>
    public class ManifestSystemRelationship
    {
        public string Id { get; set; } = string.Empty;
        public string Type { get; set; } = string.Empty;
        public string? Manifest { get; set; }
        public ManifestCompatibility? Compatibility { get; set; }
    }

    /// <summary>
    /// Represents a module relationship in a FoundryVTT manifest
    /// </summary>
    public class ManifestModuleRelationship
    {
        public string Id { get; set; } = string.Empty;
        public string Type { get; set; } = string.Empty;
        public string? Manifest { get; set; }
    }

    /// <summary>
    /// Represents compatibility information in a FoundryVTT manifest
    /// </summary>
    public class ManifestCompatibility
    {
        public string? Minimum { get; set; }
        public string? Verified { get; set; }
    }

    /// <summary>
    /// Represents a compendium pack definition from a FoundryVTT manifest
    /// </summary>
    public class ManifestPack
    {
        public string Name { get; set; } = string.Empty;
        public string Label { get; set; } = string.Empty;
        public string Path { get; set; } = string.Empty;
        public string Type { get; set; } = string.Empty;
        public string? System { get; set; }
        public List<string>? Systems { get; set; }
    }

    /// <summary>
    /// Load Plutonium directory with JSON files
    /// </summary>
    private Task<Dictionary<string, CompendiumItem>> LoadPlutroniumDirectoryAsync(string dataPath)
    {
        var items = new Dictionary<string, CompendiumItem>();
        
        if (!Directory.Exists(dataPath))
            return Task.FromResult(items);
        
        var reader = new PlutroniumJSONReader(new NullLogger<PlutroniumJSONReader>());
        
        // Load root JSON files
        var rootItems = reader.LoadPack(dataPath);
        foreach (var item in rootItems.Values)
        {
            var key = $"{item.Type}:{item.Name}".ToLowerInvariant();
            if (!items.ContainsKey(key))
            {
                items[key] = item;
            }
        }
        
        // Load subdirectories
        foreach (var subDir in Directory.GetDirectories(dataPath))
        {
            try
            {
                var subItems = reader.LoadPack(subDir);
                foreach (var item in subItems.Values)
                {
                    var key = $"{item.Type}:{item.Name}".ToLowerInvariant();
                    if (!items.ContainsKey(key))
                    {
                        items[key] = item;
                    }
                }
            }
            catch (Exception ex)
            {
                _logger.LogWarning(ex, "Failed to load Plutonium subdirectory: {SubDir}", subDir);
            }
        }
        
        _logger.LogDebug("Loaded {Count} items from Plutonium data directory", items.Count);
        return Task.FromResult(items);
    }
    
    /// <summary>
    /// Get appropriate reader for pack format with fallback chain
    /// </summary>
    public ICompendiumReader GetReader(string packPath)
    {
        // Priority order: NeDB -> Plutonium JSON -> FoundryCLI Bridge
        // Note: LevelDB is handled separately via CLI bridge
        var readers = new ICompendiumReader[]
        {
            new NeDBCompendiumReader(new NullLogger<NeDBCompendiumReader>()),
            new PlutroniumJSONReader(new NullLogger<PlutroniumJSONReader>()),
            new FoundryCLIBridge(new NullLogger<FoundryCLIBridge>())
        };

        foreach (var reader in readers)
        {
            _logger.LogDebug("🔍 Testing {ReaderType} for {Pack} - Supports: {Supports}", 
                reader.FormatName, packPath, reader.SupportsFormat(packPath));
                
            if (reader.SupportsFormat(packPath))
            {
                _logger.LogDebug("Selected {ReaderType} for {Pack}", reader.FormatName, packPath);
                _logger.LogError("🎯 RETURNING READER TYPE: {ActualType}", reader.GetType().Name);
                return reader;
            }
        }

        throw new NotSupportedException($"No compendium reader supports format: {packPath}");
    }

    /// <summary>
    /// Find entity in unified cache with fuzzy matching
    /// </summary>
    public CompendiumItem? FindEntity(string entityType, string entityName, double threshold = 0.8)
    {
        if (!_globalCacheLoaded)
        {
            throw new InvalidOperationException("Unified cache not loaded. Call LoadUnifiedCacheAsync first.");
        }

        if (!_unifiedEntityCache.ContainsKey(entityType))
        {
            return null;
        }

        var entityCache = _unifiedEntityCache[entityType];

        // Direct match first
        if (entityCache.TryGetValue(entityName, out var exactMatch))
        {
            return exactMatch;
        }

        // Case-insensitive match
        var caseInsensitive = entityCache.FirstOrDefault(kvp => 
            string.Equals(kvp.Key, entityName, StringComparison.OrdinalIgnoreCase));
        if (caseInsensitive.Key != null)
        {
            return caseInsensitive.Value;
        }

        // Fuzzy matching (simplified Levenshtein-based)
        var bestMatch = entityCache
            .Select(kvp => new { Item = kvp.Value, Similarity = CalculateSimilarity(entityName, kvp.Key) })
            .Where(x => x.Similarity >= threshold)
            .OrderByDescending(x => x.Similarity)
            .FirstOrDefault();

        return bestMatch?.Item;
    }

    /// <summary>
    /// Get all entities of a specific type from unified cache
    /// </summary>
    public IEnumerable<CompendiumItem> GetEntitiesByType(string entityType)
    {
        if (!_globalCacheLoaded)
        {
            throw new InvalidOperationException("Unified cache not loaded. Call LoadUnifiedCacheAsync first.");
        }

        return _unifiedEntityCache.TryGetValue(entityType, out var cache) ? 
            cache.Values : 
            Enumerable.Empty<CompendiumItem>();
    }

    /// <summary>
    /// Get all entities from unified cache
    /// </summary>
    public Dictionary<string, CompendiumItem> GetAllEntities()
    {
        if (!_globalCacheLoaded)
        {
            throw new InvalidOperationException("Unified cache not loaded. Call LoadUnifiedCacheAsync first.");
        }

        var allEntities = new Dictionary<string, CompendiumItem>();
        foreach (var typeCache in _unifiedEntityCache.Values)
        {
            foreach (var item in typeCache.Values)
            {
                var key = $"{item.Type}:{item.Name}".ToLowerInvariant();
                allEntities[key] = item;
            }
        }
        return allEntities;
    }

    /// <summary>
    /// Search entities across all types
    /// </summary>
    public IEnumerable<CompendiumItem> SearchEntities(string searchTerm, string? entityType = null)
    {
        if (!_globalCacheLoaded)
        {
            throw new InvalidOperationException("Unified cache not loaded. Call LoadUnifiedCacheAsync first.");
        }

        var searchLower = searchTerm.ToLowerInvariant();
        var results = new List<CompendiumItem>();

        var allEntityCaches = _unifiedEntityCache.Values.ToArray();

        var cachesToSearch = entityType != null && _unifiedEntityCache.ContainsKey(entityType) ?
            new[] { _unifiedEntityCache[entityType] } :
            allEntityCaches;

        foreach (var cache in cachesToSearch)
        {
            var matches = cache.Values.Where(item =>
                item.Name.ToLowerInvariant().Contains(searchLower) ||
                CalculateSimilarity(searchTerm, item.Name) > 0.6);
            
            results.AddRange(matches);
        }

        return results.OrderBy(item => item.Type).ThenBy(item => item.Name);
    }

    /// <summary>
    /// Simple similarity calculation (Jaro-Winkler approximation)
    /// </summary>
    private double CalculateSimilarity(string s1, string s2)
    {
        if (string.IsNullOrEmpty(s1) || string.IsNullOrEmpty(s2))
            return 0;

        if (s1 == s2) return 1.0;

        var longer = s1.Length > s2.Length ? s1 : s2;
        var shorter = s1.Length > s2.Length ? s2 : s1;

        if (longer.Length == 0) return 1.0;

        // Simple edit distance approximation
        var editDistance = CalculateEditDistance(shorter, longer);
        return (longer.Length - editDistance) / (double)longer.Length;
    }

    private int CalculateEditDistance(string s1, string s2)
    {
        var matrix = new int[s1.Length + 1, s2.Length + 1];

        for (int i = 0; i <= s1.Length; i++)
            matrix[i, 0] = i;
        for (int j = 0; j <= s2.Length; j++)
            matrix[0, j] = j;

        for (int i = 1; i <= s1.Length; i++)
        {
            for (int j = 1; j <= s2.Length; j++)
            {
                var cost = s1[i - 1] == s2[j - 1] ? 0 : 1;
                matrix[i, j] = Math.Min(
                    Math.Min(matrix[i - 1, j] + 1, matrix[i, j - 1] + 1),
                    matrix[i - 1, j - 1] + cost);
            }
        }

        return matrix[s1.Length, s2.Length];
    }

    /// <summary>
    /// Detect compendium format from directory or file path
    /// </summary>
    public CompendiumFormat DetectFormat(string path)
    {
        if (Directory.Exists(path))
        {
            // Check for LevelDB format (CURRENT, LOCK, LOG files)
            var levelDbFiles = new[] { "CURRENT", "LOCK" };
            if (levelDbFiles.All(file => File.Exists(Path.Combine(path, file))))
            {
                return CompendiumFormat.LevelDB;
            }
            
            // Check for Plutonium JSON format
            var plutoniumJsonReader = new PlutroniumJSONReader(new NullLogger<PlutroniumJSONReader>());
            if (plutoniumJsonReader.SupportsFormat(path))
            {
                return CompendiumFormat.PlutoJSON;
            }
        }
        else if (File.Exists(path) && path.EndsWith(".db"))
        {
            return CompendiumFormat.NeDB;
        }
        
        return CompendiumFormat.Unknown;
    }

    private Dictionary<string, CompendiumItem> LoadLevelDBPack(string packName, string sourceName = "", string? packType = null)
    {
        _logger.LogInformation("Loading LevelDB pack via Foundry CLI bridge: {PackName}", packName);
        
        try
        {
            return LoadLevelDBViaFoundryCLI(packName, sourceName, _verbose, packType);
        }
        catch (Exception ex)
        {
            _logger.LogDebug("LevelDB CLI bridge failed for {PackName}. This is expected and will be skipped.", packName);
            _logger.LogTrace(ex, "LevelDB CLI error details for {PackName}", packName);
            return new Dictionary<string, CompendiumItem>();
        }
    }

    private Dictionary<string, CompendiumItem> LoadLevelDBViaFoundryCLI(string packName, string sourceName = "", bool verbose = false, string? packType = null)
    {
        _logger.LogInformation("Extracting LevelDB pack '{PackName}' using Foundry CLI", packName);

        // Use PowerShell script for direct fvtt execution
        var currentDir = Directory.GetCurrentDirectory();
        
        // Find the root project directory (where package.json is located)
        var rootDir = currentDir;
        while (rootDir != null && !File.Exists(Path.Combine(rootDir, "package.json")))
        {
            rootDir = Directory.GetParent(rootDir)?.FullName;
        }
        
        if (rootDir == null)
        {
            throw new Exception($"Could not find root project directory with package.json from: {currentDir}");
        }
        
        var scriptPath = Path.Combine(rootDir, "foundry-extract.ps1");
        
        if (!File.Exists(scriptPath))
        {
            throw new Exception($"Foundry CLI PowerShell script not found: {scriptPath}");
        }

        // Determine package context based on source name
        var (packageId, packageType) = DeterminePackageContext(sourceName);

        var debugFlag = verbose ? "-ShowDebug" : "";
        var startInfo = new ProcessStartInfo
        {
            FileName = "pwsh.exe",  // Use PowerShell Core like manual execution
            Arguments = $@"-ExecutionPolicy Bypass -NoProfile -NonInteractive -File ""{scriptPath}"" -PackName ""{packName}"" -PackageId ""{packageId}"" -PackageType ""{packageType}"" {debugFlag}",
            WorkingDirectory = rootDir,  // Set working directory to root project directory
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true
        };

        using var process = Process.Start(startInfo);
        if (process == null)
            throw new Exception("Failed to start PowerShell process");

        // Read output and error streams separately to avoid conflicts
        var outputBuilder = new StringBuilder();
        var errorBuilder = new StringBuilder();
        
        // Set up async reading for both streams
        process.OutputDataReceived += (sender, e) => {
            if (e.Data != null) outputBuilder.AppendLine(e.Data);
        };
        process.ErrorDataReceived += (sender, e) => {
            if (e.Data != null) errorBuilder.AppendLine(e.Data);
        };

        // Start async reading
        process.BeginOutputReadLine();
        process.BeginErrorReadLine();

        var timeout = 90000; // 90 second timeout
        var completed = process.WaitForExit(timeout);
        
        if (!completed)
        {
            _logger.LogWarning("PowerShell process timed out after {Timeout}ms, killing process", timeout);
            try { process.Kill(); } catch { }
            throw new Exception($"Foundry CLI PowerShell process timed out after {timeout/1000} seconds");
        }

        var output = outputBuilder.ToString().Trim();
        var error = errorBuilder.ToString().Trim();

        if (!string.IsNullOrEmpty(error))
            _logger.LogDebug("Foundry CLI stderr: {Error}", error);
            
        // Debug: Log what we got from stdout
        _logger.LogDebug("Foundry CLI stdout: '{Output}' (Length: {Length})", output, output.Length);

        if (process.ExitCode != 0)
        {
            // Extract just the main error message, not the full PowerShell stack trace
            var cleanError = error;
            if (!string.IsNullOrEmpty(error))
            {
                var lines = error.Split('\n', StringSplitOptions.RemoveEmptyEntries);
                cleanError = lines.FirstOrDefault(l => l.Contains("failed:"))?.Split("failed:")[1]?.Trim() ?? 
                           lines.LastOrDefault()?.Trim() ?? 
                           "Unknown error";
            }
            throw new Exception($"PowerShell process exited with code {process.ExitCode}. Error: {cleanError}");
        }

        if (string.IsNullOrWhiteSpace(output))
            throw new Exception("No output received from PowerShell script");

        // Extract the directory path from the last line of output
        // The output contains fvtt command output followed by the directory path
        var outputLines = output.Split('\n', StringSplitOptions.RemoveEmptyEntries);
        var outputDirectory = outputLines.LastOrDefault()?.Trim();
        
        if (string.IsNullOrEmpty(outputDirectory))
        {
            throw new Exception("No directory path found in PowerShell output");
        }
        
        _logger.LogDebug("Extracted directory path: '{Directory}'", outputDirectory);
        
        if (!Directory.Exists(outputDirectory))
        {
            throw new Exception($"Output directory not found or invalid: {outputDirectory}");
        }

        var jsonFiles = Directory.GetFiles(outputDirectory, "*.json");
        _logger.LogInformation("Extraction successful: {FileCount} files in {Directory}", jsonFiles.Length, outputDirectory);

        // Read JSON files directly from the extracted directory
        var items = new Dictionary<string, CompendiumItem>();

        foreach (var filePath in jsonFiles)
        {
            try
            {
                var fileName = Path.GetFileNameWithoutExtension(filePath);
                var jsonContent = File.ReadAllText(filePath);
                
                // Parse the JSON content
                var jsonDoc = JsonDocument.Parse(jsonContent);
                var jsonElement = jsonDoc.RootElement;
                
                // Extract name and type from the JSON object
                var name = jsonElement.TryGetProperty("name", out var nameProp) ? nameProp.GetString() ?? fileName : fileName;
                var type = jsonElement.TryGetProperty("type", out var typeProp) ? typeProp.GetString() ?? "unknown" : "unknown";
                
                // Enhanced type detection for common FoundryVTT data types
                if (type == "unknown" || string.IsNullOrEmpty(type))
                {
                    if (!string.IsNullOrEmpty(packType))
                    {
                        type = packType switch
                        {
                            "Actor" => "npc",
                            "Item" => "item",
                            "JournalEntry" => "journal",
                            "RollTable" => "rolltable",
                            "Scene" => "scene",
                            _ => packType.ToLowerInvariant()
                        };
                    }
                    else if (packName.Contains("journal", StringComparison.OrdinalIgnoreCase))
                    {
                        type = "journal";
                    }
                    else if (packName.Contains("scene", StringComparison.OrdinalIgnoreCase))
                    {
                        type = "scene";
                    }
                    else if (packName.Contains("rolltable", StringComparison.OrdinalIgnoreCase) || 
                             packName.Contains("table", StringComparison.OrdinalIgnoreCase))
                    {
                        type = "rolltable";
                    }
                    else if (jsonElement.TryGetProperty("data", out var dataProp))
                    {
                        // Check for specific D&D 5e/PF2e type indicators in data structure
                        if (dataProp.TryGetProperty("type", out var dataTypeProp))
                        {
                            type = dataTypeProp.GetString() ?? type;
                        }
                    }
                }
                
                var item = new CompendiumItem
                {
                    Id = fileName,
                    Name = name,
                    Type = type,
                    Data = jsonDoc,
                    SourceFormat = "LevelDB-CLI",
                    SourceFile = packName
                };
                
                items[fileName] = item;
                _logger.LogDebug("Successfully parsed file: {FileName} (name: {Name}, type: {Type})", fileName, name, type);
            }
            catch (Exception ex)
            {
                _logger.LogWarning("Failed to parse JSON file {FilePath}: {Error}", filePath, ex.Message);
            }
        }

        // Clean up the extracted files directory
        try
        {
            Directory.Delete(outputDirectory, true);
            _logger.LogDebug("Cleaned up temporary directory: {Directory}", outputDirectory);
        }
        catch (Exception ex)
        {
            _logger.LogWarning("Failed to clean up temporary directory {Directory}: {Error}", outputDirectory, ex.Message);
        }

        _logger.LogInformation("Successfully extracted {Count} items from pack '{PackName}' via Foundry CLI", 
            items.Count, packName);
            
        return items;
    }

    private static (string packageId, string packageType) DeterminePackageContext(string sourceName)
    {
        // Determine correct package context based on source name (directory name from basePath)
        // sourceName comes from Path.GetFileName(basePath) in the calling method
        
        // System packages
        if (sourceName.Equals("dnd5e", StringComparison.OrdinalIgnoreCase))
            return ("dnd5e", "System");
        if (sourceName.Equals("pf2e", StringComparison.OrdinalIgnoreCase))
            return ("pf2e", "System");
        if (sourceName.Equals("pf1", StringComparison.OrdinalIgnoreCase))
            return ("pf1", "System");
            
        // Module packages - for modules, sourceName is typically the module folder name
        // Note: These modules may not be installed, which will cause fvtt commands to fail
        // This is expected behavior and should be handled gracefully by the caller
        if (sourceName.Contains("dungeon-masters-guide"))
            return ("dnd-dungeon-masters-guide", "Module");
        if (sourceName.Contains("players-handbook"))
            return ("dnd-players-handbook", "Module");
        if (sourceName.Contains("battlezoo"))
            return (sourceName, "Module");
        if (sourceName.Contains("pf-content"))
            return ("pf-content", "Module");
        if (sourceName.Contains("plutonium"))
            return ("plutonium", "Module");
        if (sourceName.Contains("statblock"))
            return (sourceName, "Module");
            
        // Default: assume it's a module with the same name as the folder
        return (sourceName, "Module");
    }
    /// <summary>
    /// Compendium format types supported by the system
    /// </summary>
    public enum CompendiumFormat
    {
        Unknown,
        NeDB,
        LevelDB,        // Handled via CLI bridge
        PlutoJSON       // Plutonium's JSON file format
    }

    /// <summary>
    /// Information about a discovered compendium pack
    /// </summary>
    public class CompendiumPackInfo
    {
        public string Name { get; set; } = string.Empty;
        public string Path { get; set; } = string.Empty;
        public CompendiumFormat Format { get; set; }
        public string? BasePath { get; set; }
        public string? DbFile { get; set; }  // For NeDB format
    }

    public CompendiumItem? FindItem(
        Dictionary<string, Dictionary<string, CompendiumItem>> packs,
        string itemName,
        string? itemType = null)
    {
        // Direct name match first
        foreach (var (packName, packItems) in packs)
        {
            foreach (var (itemId, item) in packItems)
            {
                if (string.Equals(item.Name, itemName, StringComparison.OrdinalIgnoreCase))
                {
                    if (itemType == null || string.Equals(item.Type, itemType, StringComparison.OrdinalIgnoreCase))
                    {
                        return item;
                    }
                }
            }
        }

        // TODO: Implement fuzzy matching with FuzzySharp library
        // For now, return null if no exact match
        return null;
    }
}

/// <summary>
/// Reads Plutonium JSON compendium format (D&D Beyond imported content)
/// </summary>
public class PlutroniumJSONReader : ICompendiumReader
{
    private readonly ILogger _logger;
    
    public string FormatName => "Plutonium JSON";
    
    public PlutroniumJSONReader(ILogger logger)
    {
        _logger = logger;
    }
    
    public bool SupportsFormat(string path)
    {
        return CanRead(path);
    }
    
    public bool CanRead(string path)
    {
        if (!Directory.Exists(path)) return false;
        
        // Look for typical Plutonium JSON files
        var plutoniumFiles = new[]
        {
            "backgrounds.json", "feats.json", "spells.json", "items.json",
            "races.json", "classes.json", "monsters.json", "conditions.json"
        };
        
        return plutoniumFiles.Any(file => File.Exists(Path.Combine(path, file)));
    }
    
    public Dictionary<string, CompendiumItem> LoadPack(string path)
    {
        var items = new Dictionary<string, CompendiumItem>();
        
        if (!Directory.Exists(path))
        {
            _logger.LogWarning("Plutonium directory not found: {Path}", path);
            return items;
        }
        
        // Process all JSON files in the directory and subdirectories
        var jsonFiles = Directory.GetFiles(path, "*.json", SearchOption.AllDirectories);
        
        foreach (var jsonFile in jsonFiles)
        {
            try
            {
                ProcessPlutroniumFile(jsonFile, items);
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Failed to process Plutonium file: {File}", jsonFile);
            }
        }
        
        _logger.LogInformation("Loaded {Count} items from Plutonium directory: {Path}", items.Count, path);
        return items;
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName)
    {
        throw new NotImplementedException();
    }

    private void ProcessPlutroniumFile(string jsonFilePath, Dictionary<string, CompendiumItem> items)
    {
        var fileName = Path.GetFileNameWithoutExtension(jsonFilePath);
        var jsonContent = File.ReadAllText(jsonFilePath);
        
        if (string.IsNullOrWhiteSpace(jsonContent))
        {
            return;
        }
        
        using var jsonDoc = JsonDocument.Parse(jsonContent);
        
        // Plutonium format: { "entityType": [ ...items ] }
        if (jsonDoc.RootElement.ValueKind != JsonValueKind.Object)
        {
            _logger.LogWarning("Expected object at root of Plutonium file: {File}", jsonFilePath);
            return;
        }
        
        // Find the first array property (should be the entity type)
        JsonElement arrayElement = default;
        string entityType = "";
        
        foreach (var property in jsonDoc.RootElement.EnumerateObject())
        {
            if (property.Value.ValueKind == JsonValueKind.Array)
            {
                arrayElement = property.Value;
                entityType = property.Name;
                break;
            }
        }
        
        if (arrayElement.ValueKind != JsonValueKind.Array)
        {
            _logger.LogDebug("No array property found in Plutonium file: {File}", jsonFilePath);
            return;
        }
        
        // Process each item in the array
        foreach (var itemElement in arrayElement.EnumerateArray())
        {
            try
            {
                var item = ParsePlutroniumItem(itemElement, entityType);
                if (item != null)
                {
                    items[item.Id] = item;
                }
            }
            catch (Exception ex)
            {
                _logger.LogWarning(ex, "Failed to parse item from {File}", jsonFilePath);
            }
        }
    }
    
    private CompendiumItem? ParsePlutroniumItem(JsonElement itemElement, string sourceType)
    {
        if (!itemElement.TryGetProperty("name", out var nameElement))
        {
            return null;
        }
        
        var name = nameElement.GetString();
        if (string.IsNullOrEmpty(name))
        {
            return null;
        }
        
        // Generate ID from name or use existing _id
        var id = itemElement.TryGetProperty("_id", out var idElement) && idElement.ValueKind == JsonValueKind.String
            ? idElement.GetString()!
            : GenerateId(name);
        
        // Determine type from source file or item data
        var type = DetermineItemType(itemElement, sourceType);
        
        // Extract common properties
        var description = ExtractDescription(itemElement);
        var rarity = ExtractRarity(itemElement);
        var source = ExtractSource(itemElement);
        
        var jsonDoc = JsonDocument.Parse(itemElement.GetRawText());
        return new CompendiumItem
        {
            Id = id,
            Name = name,
            Type = type,
            Description = description,
            Data = jsonDoc,
            Source = source,
            Rarity = rarity,
            SourceFormat = "PlutoJSON",
            SourceFile = "plutonium"
        };
    }
    
    private string DetermineItemType(JsonElement itemElement, string sourceType)
    {
        // Check for explicit type property
        if (itemElement.TryGetProperty("type", out var typeElement) && typeElement.ValueKind == JsonValueKind.String)
        {
            return typeElement.GetString() ?? sourceType;
        }
        
        // Infer from source file name
        return sourceType switch
        {
            "backgrounds" => "background",
            "feats" => "feat", 
            "spells" => "spell",
            "items" => "item",
            "races" => "race",
            "classes" => "class",
            "monsters" => "npc",
            "conditions" => "condition",
            _ => sourceType
        };
    }
    
    private string ExtractDescription(JsonElement itemElement)
    {
        // Try common description fields
        var descFields = new[] { "entries", "description", "desc", "text" };
        
        foreach (var field in descFields)
        {
            if (itemElement.TryGetProperty(field, out var descElement))
            {
                if (descElement.ValueKind == JsonValueKind.String)
                {
                    return descElement.GetString() ?? "";
                }
                else if (descElement.ValueKind == JsonValueKind.Array)
                {
                    // Join array entries with newlines
                    var entries = descElement.EnumerateArray()
                        .Where(e => e.ValueKind == JsonValueKind.String)
                        .Select(e => e.GetString())
                        .Where(s => !string.IsNullOrEmpty(s));
                    
                    return string.Join("\n", entries);
                }
            }
        }
        
        return "";
    }
    
    private string? ExtractRarity(JsonElement itemElement)
    {
        var rarityFields = new[] { "rarity", "tier" };
        
        foreach (var field in rarityFields)
        {
            if (itemElement.TryGetProperty(field, out var rarityElement) && rarityElement.ValueKind == JsonValueKind.String)
            {
                return rarityElement.GetString();
            }
        }
        
        return null;
    }
    
    private string ExtractSource(JsonElement itemElement)
    {
        if (itemElement.TryGetProperty("source", out var sourceElement) && sourceElement.ValueKind == JsonValueKind.String)
        {
            return sourceElement.GetString() ?? "Plutonium";
        }
        
        return "Plutonium";
    }
    
    private static string GenerateId(string name)
    {
        // Generate consistent ID from name
        return name.ToLowerInvariant()
            .Replace(" ", "")
            .Replace("'", "")
            .Replace("-", "")
            .Replace("(", "")
            .Replace(")", "");
    }
    
    public void Dispose()
    {
        // No resources to dispose
    }
}
