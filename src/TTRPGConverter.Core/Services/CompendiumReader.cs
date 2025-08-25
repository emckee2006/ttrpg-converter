using System.Diagnostics;
using System.Text;
using System.Text.Json;
using Microsoft.Extensions.Logging;
using Raven.Client.Documents;
using Raven.Embedded;
using TTRPGConverter.Core.Entities;
using TTRPGConverter.Core.Interfaces;
using TTRPGConverter.Core.Services.CompendiumReaders;

namespace TTRPGConverter.Core.Services
{
    public class CompendiumManager : IDisposable
    {
        private static readonly string[] TARGET_SYSTEMS = ["dnd5e", "pf2e", "pf1"];
        private readonly ILogger<CompendiumManager> _logger;
        private readonly bool _verbose;
        private readonly CollisionLogger? _collisionLogger;
        private readonly IDocumentStore? _store;
        private readonly Dictionary<string, Dictionary<string, CompendiumItem>> _inMemoryCache = new();
        private readonly Dictionary<string, string> _detectedSystems = new();
        private readonly Dictionary<string, string> _packTypes = new();
        private readonly bool _isBuilderMode;

        public CompendiumManager(ILogger<CompendiumManager> logger, string cachePath, bool verbose = false)
        {
            _logger = logger;
            _verbose = verbose;
            if (Directory.Exists(cachePath))
            {
                try
                {
                    var serverOptions = new ServerOptions { DataDirectory = cachePath, ServerUrl = "http://127.0.0.1:8081" };
                    EmbeddedServer.Instance.StartServer(serverOptions);
                    _store = EmbeddedServer.Instance.GetDocumentStore(new DatabaseOptions("Compendium"));
                    new CompendiumItem_Index().Execute(_store);
                }
                catch (Exception ex)
                {
                    _logger.LogError(ex, "❌ Failed to connect to RavenDB cache at {Path}", cachePath);
                }
            }
            else
            {
                _logger.LogWarning("Compendium cache not found at {Path}. Run 'update-compendium' to create it.", cachePath);
            }
        }

        public CompendiumManager(ILogger<CompendiumManager> logger, bool verbose = false, string? collisionLogPath = null)
        {
            _logger = logger;
            _verbose = verbose;
            _collisionLogger = !string.IsNullOrEmpty(collisionLogPath) ? new CollisionLogger(collisionLogPath) : null;
            _isBuilderMode = true;
        }

        public Task LoadUnifiedCacheAsync(IEnumerable<string> compendiumPaths)
        {
            if (!_isBuilderMode) {
                _logger.LogWarning("LoadUnifiedCacheAsync should only be called in Builder Mode.");
                return Task.CompletedTask;
            }

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
                                ? LoadLevelDBPack(Path.GetFileName(selectedPath), sourceName, _packTypes.GetValueOrDefault(packName), finalSystemName)
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

        public CompendiumItem? FindEntity(string entityType, string entityName)
        {
            if (_store == null) return null;
            using var session = _store.OpenSession();
            return session.Query<CompendiumItem, CompendiumItem_Index>().FirstOrDefault(x => x.Type == entityType && x.Name == entityName && x.IsPrimary);
        }

        public IEnumerable<CompendiumItem> FindAllCandidates(string entityType, string entityName)
        {
            if (_store == null) return [];
            using var session = _store.OpenSession();
            return session.Query<CompendiumItem, CompendiumItem_Index>().Where(x => x.Type == entityType && x.Name == entityName);
        }

        public IEnumerable<CompendiumItem> GetEntitiesByType(string entityType)
        {
            if (_store == null) return [];
            using var session = _store.OpenSession();
            return session.Query<CompendiumItem, CompendiumItem_Index>().Where(x => x.Type == entityType);
        }

        public Dictionary<string, CompendiumItem> GetAllEntities()
        {
            if (_isBuilderMode)
            {
                return _inMemoryCache.SelectMany(kvp => kvp.Value).ToDictionary(kvp => kvp.Key, kvp => kvp.Value);
            }
            if (_store == null) return new Dictionary<string, CompendiumItem>();
            using var session = _store.OpenSession();
            using var stream = session.Advanced.Stream<CompendiumItem>("CompendiumItems/");
            var results = new Dictionary<string, CompendiumItem>();
            while (stream.MoveNext()) { if (stream.Current.Id != null) results[stream.Current.Id] = stream.Current.Document; }
            return results;
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
                        if (!packsByName.ContainsKey(pack.Name)) packsByName[pack.Name] = [];
                        packsByName[pack.Name].Add(packPath);
                        _packTypes[pack.Name] = pack.Type;
                    }
                    else if (pack.Path.EndsWith(".db"))
                    {
                        var levelDbPath = packPath.Substring(0, packPath.Length - 3);
                        if (Directory.Exists(levelDbPath))
                        {
                            if (!packsByName.ContainsKey(pack.Name)) packsByName[pack.Name] = [];
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
                    if (!packsByName.ContainsKey(packName)) packsByName[packName] = [];
                    packsByName[packName].Add(entry);
                }
            }
            return packsByName;
        }

        private List<ManifestPack> TryReadManifestPacks(string packsDir)
        {
            var baseDir = Path.GetDirectoryName(packsDir);
            if (baseDir == null) return [];
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
                            var sourceKey = Path.GetFileName(baseDir);
                            _detectedSystems[sourceKey] = detectedSystem;
                        }
                        foreach (var pack in manifest.Packs.Where(p => !string.IsNullOrEmpty(p.System)))
                        {
                            _detectedSystems[pack.Name] = pack.System ?? "unknown";
                        }
                        return FilterPacksBySystem(manifest.Packs, manifestFile);
                    }
                }
                catch (Exception ex) { _logger.LogWarning(ex, "⚠️ Failed to parse {ManifestFile}", manifestFile); }
            }
            return [];
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
            if (packPath.EndsWith(".db")) return new NeDbCompendiumReader(new Logger<NeDbCompendiumReader>(new LoggerFactory()));
            if (Directory.Exists(packPath)) { 
                var plutoReader = new PlutoniumJsonReader(new Logger<PlutoniumJsonReader>(new LoggerFactory()));
                if (plutoReader.SupportsFormat(packPath)) return plutoReader;
            }
            throw new NotSupportedException($"No compendium reader supports format: {packPath}");
        }

        private Dictionary<string, CompendiumItem> LoadLevelDBPack(string packName, string sourceName, string? packType, string? systemName)
        {
            try
            {
                return LoadLevelDBViaFoundryCLI(packName, sourceName, _verbose, packType, systemName);
            }
            catch (Exception ex)
            {
                _logger.LogDebug(ex, "LevelDB CLI bridge failed for {PackName}. This is expected and will be skipped.", packName);
                return new Dictionary<string, CompendiumItem>();
            }
        }

        private Dictionary<string, CompendiumItem> LoadLevelDBViaFoundryCLI(string packName, string sourceName, bool verbose, string? packType, string? systemName)
        {
            var currentDir = Directory.GetCurrentDirectory();
            var rootDir = currentDir;
            while (rootDir != null && !File.Exists(Path.Combine(rootDir, "package.json"))) { rootDir = Directory.GetParent(rootDir)?.FullName; }
            if (rootDir == null) throw new Exception($"Could not find root project directory with package.json from: {currentDir}");
            var scriptPath = Path.Combine(rootDir, "foundry-extract.ps1");
            if (!File.Exists(scriptPath)) throw new Exception($"Foundry CLI PowerShell script not found: {scriptPath}");

            var (packageId, cliPackageType) = DeterminePackageContext(sourceName);
            var debugFlag = verbose ? "-ShowDebug" : "";
            var startInfo = new ProcessStartInfo
            {
                FileName = "pwsh.exe",
                Arguments = $@"-ExecutionPolicy Bypass -NoProfile -NonInteractive -File ""{scriptPath}"" -PackName ""{packName}"" -PackageId ""{packageId}"" -PackageType ""{cliPackageType}"" {debugFlag}",
                WorkingDirectory = rootDir,
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
                CreateNoWindow = true
            };

            using var process = Process.Start(startInfo) ?? throw new Exception("Failed to start PowerShell process");
            var outputBuilder = new StringBuilder();
            var errorBuilder = new StringBuilder();
            var outputTcs = new TaskCompletionSource<bool>();
            var errorTcs = new TaskCompletionSource<bool>();

            process.OutputDataReceived += (sender, e) => { 
                if (e.Data != null) outputBuilder.AppendLine(e.Data); else outputTcs.SetResult(true); 
            };
            process.ErrorDataReceived += (sender, e) => { 
                if (e.Data != null) errorBuilder.AppendLine(e.Data); else errorTcs.SetResult(true); 
            };

            process.BeginOutputReadLine();
            process.BeginErrorReadLine();

            if (process.WaitForExit(90000) && outputTcs.Task.Wait(1000) && errorTcs.Task.Wait(1000))
            {
                var output = outputBuilder.ToString().Trim();
                var error = errorBuilder.ToString().Trim();

                if (verbose && !string.IsNullOrEmpty(output)) _logger.LogDebug("PowerShell stdout: {Output}", output);
                if (!string.IsNullOrEmpty(error)) _logger.LogWarning("PowerShell stderr: {Error}", error);

                if (process.ExitCode != 0) throw new Exception($"PowerShell process exited with code {process.ExitCode}. Error: {error}");

                var outputDirectory = output.Split(['\r', '\n'], StringSplitOptions.RemoveEmptyEntries).LastOrDefault(Directory.Exists);
                if (string.IsNullOrEmpty(outputDirectory)) throw new Exception($"Output directory not found or invalid in script output: {output}");
                outputDirectory = outputDirectory.Trim();

                var items = new Dictionary<string, CompendiumItem>();
                foreach (var filePath in Directory.GetFiles(outputDirectory, "*.json"))
                {
                    var fileName = Path.GetFileNameWithoutExtension(filePath);
                    var jsonContent = File.ReadAllText(filePath);
                    var jsonDoc = JsonDocument.Parse(jsonContent);
                    var name = jsonDoc.RootElement.TryGetProperty("name", out var nameProp) ? nameProp.GetString() ?? fileName : fileName;
                    var type = jsonDoc.RootElement.TryGetProperty("type", out var typeProp) ? typeProp.GetString() : null;

                    if (string.IsNullOrEmpty(type) || type == "unknown")
                    {
                        type = packType switch
                        {
                            "Actor" => "npc",
                            "Item" => "item",
                            "JournalEntry" => "journal",
                            "RollTable" => "rolltable",
                            "Scene" => "scene",
                            _ => packType?.ToLowerInvariant() ?? "unknown"
                        };
                    }

                    items[fileName] = new CompendiumItem { Id = fileName, Name = name, Type = type, Data = jsonDoc, SourceFormat = "LevelDB-CLI", SourceFile = packName, IsPrimary = true, System = systemName ?? "unknown" };
                }
                Directory.Delete(outputDirectory, true);
                return items;
            }
            else
            {
                try { process.Kill(); } catch { }
                throw new Exception("Foundry CLI PowerShell process timed out or streams did not close.");
            }
        }

        private static (string packageId, string packageType) DeterminePackageContext(string sourceName)
        {
            if (sourceName.Equals("dnd5e", StringComparison.OrdinalIgnoreCase)) return ("dnd5e", "System");
            if (sourceName.Equals("pf2e", StringComparison.OrdinalIgnoreCase)) return ("pf2e", "System");
            if (sourceName.Equals("pf1", StringComparison.OrdinalIgnoreCase)) return ("pf1", "System");
            if (sourceName.Contains("dungeon-masters-guide")) return ("dnd-dungeon-masters-guide", "Module");
            if (sourceName.Contains("players-handbook")) return ("dnd-players-handbook", "Module");
            return (sourceName, "Module");
        }

        private CompendiumFormat DetectFormat(string path)
        {
            if (Directory.Exists(path)) {
                if (new[] { "CURRENT", "LOCK" }.All(f => File.Exists(Path.Combine(path, f)))) return CompendiumFormat.LevelDB;
                if (new PlutoniumJsonReader(new Logger<PlutoniumJsonReader>(new LoggerFactory())).SupportsFormat(path)) return CompendiumFormat.PlutoJSON;
            }
            if (File.Exists(path) && path.EndsWith(".db")) return CompendiumFormat.NeDB;
            return CompendiumFormat.Unknown;
        }

        public void Dispose() { _store?.Dispose(); _collisionLogger?.Dispose(); GC.SuppressFinalize(this); }
    }

    public class FoundryManifest { public List<ManifestPack>? Packs { get; set; } public string? Title { get; set; } public ManifestRelationships? Relationships { get; set; } }
    public class ManifestPack { public string Name { get; set; } = ""; public string Label { get; set; } = ""; public string Path { get; set; } = ""; public string Type { get; set; } = ""; public string? System { get; set; } public List<string>? Systems { get; set; } }
    public class ManifestRelationships { public List<ManifestSystemRelationship>? Systems { get; set; } }
    public class ManifestSystemRelationship { public string Id { get; set; } = ""; }
    public enum CompendiumFormat { Unknown, NeDB, LevelDB, PlutoJSON }
    public class CompendiumPackInfo { public string Name { get; set; } = ""; public string Path { get; set; } = ""; public CompendiumFormat Format { get; set; } }
}
