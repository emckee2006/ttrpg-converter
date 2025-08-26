using System.Text.Json;
using Microsoft.Extensions.Logging;

namespace TTRPGConverter.Infrastructure.Services.Compendium.CompendiumReaders;

/// <summary>
/// NeDB compendium reader - handles JSON Lines format (.db files)
/// </summary>
public class NeDbCompendiumReader : ICompendiumReader, IDisposable
{
    private readonly ILogger<NeDbCompendiumReader> _logger;

    public string FormatName => "NeDB (Legacy Foundry)";

    public NeDbCompendiumReader(ILogger<NeDbCompendiumReader> logger)
    {
        _logger = logger;
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
            
            for (var lineNum = 0; lineNum < lines.Length; lineNum++)
            {
                var line = lines[lineNum].Trim();
                if (string.IsNullOrEmpty(line))
                {
                    continue;
                }

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
