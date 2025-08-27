using System.Collections.Concurrent;
using System.Diagnostics;
using System.Text.Json;
using System.Threading.Channels;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core;
using TTRPGConverter.Core.Compendium;
using TTRPGConverter.Infrastructure.Services.Compendium.CompendiumReaders;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public record CompendiumWorkItem(string PackPath, string PackName, string SourceName, CompendiumFormat Format, string? SystemName, string? PackType);

public class CompendiumCacheBuilder : IDisposable
{
    private readonly ILoggerFactory _loggerFactory;
    private readonly ILogger<CompendiumCacheBuilder> _logger;
    private readonly int _maxDegreeOfParallelism;
    private static readonly SemaphoreSlim _cliSemaphore = new(1, 1);

    public CompendiumCacheBuilder(ILoggerFactory loggerFactory, int maxDegreeOfParallelism = 4)
    {
        _loggerFactory = loggerFactory;
        _logger = loggerFactory.CreateLogger<CompendiumCacheBuilder>();
        _maxDegreeOfParallelism = maxDegreeOfParallelism;
    }

    public async Task BuildCacheAsync(IEnumerable<string> compendiumPaths, string outputPath, IProgress<BuildProgressReport>? progress = null)
    {
        var connectionString = $"Data Source={outputPath}";
        var options = new DbContextOptionsBuilder<TTRPGConverterContext>()
            .UseSqlite(connectionString)
            .Options;

        if (File.Exists(outputPath))
        {
            var connection = new SqliteConnection(connectionString);
            SqliteConnection.ClearPool(connection);
            File.Delete(outputPath);
        }
        
        await using var context = new TTRPGConverterContext(options);
        await context.Database.MigrateAsync();

        var channel = Channel.CreateUnbounded<CompendiumWorkItem>();

        var producer = ProduceWorkItems(compendiumPaths, channel.Writer, progress);
        var consumers = ConsumeWorkItems(channel.Reader, options, progress);

        await producer;
        await Task.WhenAll(consumers);

        await FinalizeCache(options, progress);
    }

    private Task ProduceWorkItems(IEnumerable<string> compendiumPaths, ChannelWriter<CompendiumWorkItem> writer, IProgress<BuildProgressReport>? progress)
    {
        return Task.Run(async () =>
        {
            var detectedSystems = new Dictionary<string, string>();
            var packTypes = new Dictionary<string, string>();
            var allPacks = new List<CompendiumWorkItem>();

            progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallDescription: "Discovering all compendium packs...") });
            foreach (var basePath in compendiumPaths)
            {
                var packsDir = Path.Combine(basePath, "packs");
                if (!Directory.Exists(packsDir)) continue;

                var manifestPacks = TryReadManifestPacks(packsDir, detectedSystems, packTypes);
                var packsByName = DiscoverPacksByName(packsDir, manifestPacks);
                var moduleSystem = detectedSystems.GetValueOrDefault(Path.GetFileName(basePath));

                allPacks.AddRange(from packName in packsByName.Keys 
                                  let packPath = packsByName[packName].First() 
                                  let format = DetectFormat(packPath) 
                                  where format != CompendiumFormat.Unknown 
                                  let packSystem = detectedSystems.GetValueOrDefault(packName) ?? moduleSystem 
                                  let packType = packTypes.GetValueOrDefault(packName) 
                                  select new CompendiumWorkItem(packPath, packName, Path.GetFileName(basePath), format, packSystem, packType));
            }

            progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallMaxValue: allPacks.Count, OverallDescription: "Processing compendium packs...") });

            foreach (var workItem in allPacks)
            {
                if (workItem.Format == CompendiumFormat.LevelDB)
                {
                    await _cliSemaphore.WaitAsync();
                    try
                    {
                        var extractedJsonDir = RunFoundryCliBridge(workItem.PackName, workItem.SourceName);
                        if (!string.IsNullOrEmpty(extractedJsonDir))
                        {
                            await writer.WriteAsync(workItem with { PackPath = extractedJsonDir });
                        }
                        else
                        {
                            progress?.Report(new BuildProgressReport { PackResult = new PackProcessingResult { PackName = workItem.PackName, ModuleName = workItem.SourceName, IsSuccess = false, ErrorMessage = "PowerShell script failed to extract pack.", ItemsProcessed = 0 } });
                            progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallIncrement: 1) });
                        }
                    }
                    finally
                    {
                        _cliSemaphore.Release();
                    }
                }
                else
                {
                    await writer.WriteAsync(workItem);
                }
            }
            writer.Complete();
        });
    }

    private List<Task> ConsumeWorkItems(ChannelReader<CompendiumWorkItem> reader, DbContextOptions<TTRPGConverterContext> dbOptions, IProgress<BuildProgressReport>? progress)
    {
        var consumers = new List<Task>();
        for (int i = 0; i < _maxDegreeOfParallelism; i++)
        {
            consumers.Add(Task.Run(async () =>
            {
                await using var context = new TTRPGConverterContext(dbOptions);
                var neDbReader = new NeDbCompendiumReader(_loggerFactory.CreateLogger<NeDbCompendiumReader>());

                await foreach (var workItem in reader.ReadAllAsync())
                {
                    progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallIncrement: 1, DetailDescription: $"Processing: {workItem.PackName}") });
                    try
                    {
                        var items = new Dictionary<string, CompendiumItem>();
                        bool isLevelDb = workItem.Format == CompendiumFormat.LevelDB;

                        if (isLevelDb)
                        {
                            items = ProcessExtractedJson(workItem.PackPath, workItem.SourceName, workItem.SystemName, workItem.PackType);
                            Directory.Delete(workItem.PackPath, true);
                        }
                        else // NeDB
                        {
                            items = neDbReader.LoadPack(workItem.PackPath, workItem.SourceName);
                        }

                        var itemsProcessed = items.Count;
                        if (itemsProcessed > 0)
                        {
                            var newItems = items.Values.Select(item => item with { Id = Guid.NewGuid().ToString(), System = workItem.SystemName, IsPrimary = false });
                            context.CompendiumItems.AddRange(newItems);
                            await context.SaveChangesAsync();
                            progress?.Report(new BuildProgressReport { PackResult = new PackProcessingResult { PackName = workItem.PackName, ModuleName = workItem.SourceName, IsSuccess = true, ItemsProcessed = itemsProcessed } });
                        }
                        else
                        {
                            if (isLevelDb)
                            {
                                progress?.Report(new BuildProgressReport { PackResult = new PackProcessingResult { PackName = workItem.PackName, ModuleName = workItem.SourceName, IsSuccess = false, ErrorMessage = "Extraction resulted in 0 items.", ItemsProcessed = 0 } });
                            }
                            else
                            {
                                progress?.Report(new BuildProgressReport { PackResult = new PackProcessingResult { PackName = workItem.PackName, ModuleName = workItem.SourceName, IsSuccess = true, ItemsProcessed = 0 } });
                            }
                        }
                    }
                    catch (Exception ex)
                    {
                        _logger.LogError(ex, "Error processing pack {PackName}", workItem.PackName);
                        progress?.Report(new BuildProgressReport { PackResult = new PackProcessingResult { PackName = workItem.PackName, ModuleName = workItem.SourceName, IsSuccess = false, ErrorMessage = ex.Message, ItemsProcessed = 0 } });
                    }
                }
            }));
        }
        return consumers;
    }
    
    private Dictionary<string, CompendiumItem> ProcessExtractedJson(string jsonDir, string sourceName, string? systemName, string? packType)
    {
        var items = new Dictionary<string, CompendiumItem>();
        var files = Directory.GetFiles(jsonDir, "*.json");
        _logger.LogInformation("Found {FileCount} JSON files to process in {Directory}", files.Length, jsonDir);

        foreach (var filePath in files)
        {
            try
            {
                var jsonContent = File.ReadAllText(filePath);
                var jsonDoc = JsonDocument.Parse(jsonContent);
                var name = jsonDoc.RootElement.TryGetProperty("name", out var nameProp) ? nameProp.GetString() ?? Path.GetFileNameWithoutExtension(filePath) : Path.GetFileNameWithoutExtension(filePath);
                var type = jsonDoc.RootElement.TryGetProperty("type", out var typeProp) ? typeProp.GetString() : null;

                var item = new CompendiumItem
                {
                    Id = Path.GetFileNameWithoutExtension(filePath),
                    Name = name,
                    Type = type ?? packType ?? "unknown",
                    Data = jsonDoc,
                    SourceFormat = "LevelDB-CLI",
                    SourceFile = sourceName,
                    System = systemName,
                    IsPrimary = false
                };
                items[item.Id] = item;
            } catch (Exception ex) { _logger.LogError(ex, "Error processing JSON file: {FilePath}", filePath); }
        }
        return items;
    }

    private async Task FinalizeCache(DbContextOptions<TTRPGConverterContext> dbOptions, IProgress<BuildProgressReport>? progress)
    {
        progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallDescription: "Finalizing cache...", DetailDescription: "Resolving conflicts...") });
        await using var context = new TTRPGConverterContext(dbOptions);

        var groupedItems = await context.CompendiumItems
            .AsNoTracking()
            .GroupBy(i => new { i.Name, i.Type, i.System })
            .ToListAsync();

        progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(DetailMaxValue: groupedItems.Count) });

        foreach (var group in groupedItems)
        {
            progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(DetailIncrement: 1) });
            var candidates = group.ToList();
            if (candidates.Count > 1)
            {
                var winner = candidates.First();
                await context.CompendiumItems
                    .Where(i => i.Id == winner.Id)
                    .ExecuteUpdateAsync(s => s.SetProperty(p => p.IsPrimary, true));
            }
            else
            {
                await context.CompendiumItems
                    .Where(i => i.Id == candidates.First().Id)
                    .ExecuteUpdateAsync(s => s.SetProperty(p => p.IsPrimary, true));
            }
        }
        progress?.Report(new BuildProgressReport { ProgressUpdate = new ProgressUpdate(OverallDescription: "✅ Compendium cache successfully created!") });
    }

    private string? RunFoundryCliBridge(string packName, string sourceName)
    {
        var rootDir = GetProjectRoot();
        if (rootDir == null) throw new Exception("Could not find project root directory.");

        var scriptPath = Path.Combine(rootDir, "src", "TTRPGConverter.Infrastructure", "Tools", "foundry-extract.ps1");
        if (!File.Exists(scriptPath)) throw new Exception($"PowerShell script not found: {scriptPath}");

        var (packageId, packageType) = DeterminePackageContext(sourceName);
        var startInfo = new ProcessStartInfo
        {
            FileName = "pwsh.exe",
            Arguments = $"-ExecutionPolicy Bypass -NoProfile -NonInteractive -File \"{scriptPath}\" -PackName \"{packName}\" -PackageId \"{packageId}\" -PackageType \"{packageType}\"",
            WorkingDirectory = rootDir,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true
        };

        using var process = Process.Start(startInfo);
        if (process == null) throw new Exception("Failed to start PowerShell process.");

        var output = process.StandardOutput.ReadToEnd();
        var error = process.StandardError.ReadToEnd();
        process.WaitForExit();

        if (process.ExitCode != 0)
        {
            _logger.LogError("PowerShell script failed for pack '{PackName}' with exit code {ExitCode}. Error: {Error}", packName, process.ExitCode, error);
            return null;
        }

        return output.Trim();
    }

    private Dictionary<string, List<string>> DiscoverPacksByName(string packsDir, List<ManifestPack> manifestPacks)
    {
        var packsByName = new Dictionary<string, List<string>>();
        if (manifestPacks.Any())
        {
            foreach (var pack in manifestPacks)
            {
                var packPath = Path.Combine(Path.GetDirectoryName(packsDir)!, pack.Path);
                if (Directory.Exists(packPath) || File.Exists(packPath))
                {
                    if (!packsByName.ContainsKey(pack.Name)) packsByName[pack.Name] = new List<string>();
                    packsByName[pack.Name].Add(packPath);
                }
            }
        }
        else
        {
            foreach (var entry in Directory.GetFileSystemEntries(packsDir))
            {
                string packName = Path.GetFileNameWithoutExtension(entry)!;
                if (!packsByName.ContainsKey(packName)) packsByName[packName] = new List<string>();
                packsByName[packName].Add(entry);
            }
        }
        return packsByName;
    }

    private List<ManifestPack> TryReadManifestPacks(string packsDir, Dictionary<string, string> detectedSystems, Dictionary<string, string> packTypes)
    {
        var baseDir = Path.GetDirectoryName(packsDir);
        if (baseDir == null) return [];
        var manifestFile = File.Exists(Path.Combine(baseDir, "system.json")) ? "system.json" : "module.json";
        var manifestPath = Path.Combine(baseDir, manifestFile);

        if (!File.Exists(manifestPath)) return [];

        try
        {
            var jsonContent = File.ReadAllText(manifestPath);
            var manifest = JsonSerializer.Deserialize<FoundryManifest>(jsonContent, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
            if (manifest?.Packs == null) return [];

            var sourceKey = Path.GetFileName(baseDir);
            if (sourceKey != null)
            {
                detectedSystems[sourceKey] = manifest.Relationships?.Systems?.FirstOrDefault()?.Id ?? manifest.Title ?? sourceKey;
            }

            foreach (var pack in manifest.Packs)
            {
                packTypes[pack.Name] = pack.Type;
                if (!string.IsNullOrEmpty(pack.System)) detectedSystems[pack.Name] = pack.System;
            }
            return manifest.Packs;
        }
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "⚠️ Failed to parse {ManifestFile}", manifestFile);
            return [];
        }
    }

    private CompendiumFormat DetectFormat(string path)
    {
        if (Directory.Exists(path) && new[] { "CURRENT", "LOCK" }.All(f => File.Exists(Path.Combine(path, f)))) return CompendiumFormat.LevelDB;
        if (File.Exists(path) && path.EndsWith(".db")) return CompendiumFormat.NeDB;
        return CompendiumFormat.Unknown;
    }
    
    private string? GetProjectRoot()
    {
        var currentDir = Directory.GetCurrentDirectory();
        var root = new DirectoryInfo(currentDir);
        while (root != null && !root.GetFiles("*.sln").Any())
        {
            root = root.Parent;
        }
        return root?.FullName;
    }

    private (string, string) DeterminePackageContext(string sourceName)
    {
        if (sourceName.Equals("dnd5e", StringComparison.OrdinalIgnoreCase)) return ("dnd5e", "System");
        if (sourceName.Equals("pf2e", StringComparison.OrdinalIgnoreCase)) return ("pf2e", "System");
        if (sourceName.Equals("pf1", StringComparison.OrdinalIgnoreCase)) return ("pf1", "System");
        return (sourceName, "Module");
    }

    public void Dispose() { }
}

public class FoundryManifest { public List<ManifestPack>? Packs { get; set; } public string? Title { get; set; } public ManifestRelationships? Relationships { get; set; } }
public class ManifestPack { public string Name { get; set; } = ""; public string Label { get; set; } = ""; public string Path { get; set; } = ""; public string Type { get; set; } = ""; public string? System { get; set; } public List<string>? Systems { get; set; } }
public class ManifestRelationships { public List<ManifestSystemRelationship>? Systems { get; set; } }
public class ManifestSystemRelationship { public string Id { get; set; } = ""; }
public enum CompendiumFormat { Unknown, NeDB, LevelDB, PlutoJSON }
