namespace TTRPGConverter.Core.Services.CompendiumReaders;

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Entities;
using TTRPGConverter.Core.Interfaces;

/// <summary>
/// Reads Plutonium JSON compendium format (D&D Beyond imported content)
/// </summary>
public class PlutoniumJsonReader : ICompendiumReader, IDisposable
{
    private readonly ILogger _logger;
    
    public string FormatName => "Plutonium JSON";
    
    public PlutoniumJsonReader(ILogger logger)
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
        return LoadPack(packPath);
    }

    private void ProcessPlutroniumFile(string jsonFilePath, Dictionary<string, CompendiumItem> items)
    {
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
        var entityType = "";
        
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
                if (descElement.ValueKind == JsonValueKind.Array)
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
