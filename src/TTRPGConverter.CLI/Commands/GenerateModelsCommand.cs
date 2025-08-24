using TTRPGConverter.Core.Schemas;
using Microsoft.Extensions.Logging;

namespace TTRPGConverter.CLI.Commands;

/// <summary>
/// CLI command to generate C# models from JSON schemas
/// </summary>
public class GenerateModelsCommand
{
    private readonly ILogger<GenerateModelsCommand> _logger;
    private readonly SchemaGenerator _schemaGenerator;

    public GenerateModelsCommand(ILogger<GenerateModelsCommand> logger, SchemaGenerator schemaGenerator)
    {
        _logger = logger;
        _schemaGenerator = schemaGenerator;
    }

    public async Task ExecuteAsync(string schemaPath, string outputPath, string baseNamespace)
    {
        _logger.LogInformation("Starting unified model generation from schemas in: {SchemaPath}", schemaPath);
        _logger.LogInformation("Output path: {OutputPath}", outputPath);
        _logger.LogInformation("Base namespace: {BaseNamespace}", baseNamespace);

        if (!Directory.Exists(schemaPath))
        {
            _logger.LogError("Schema path does not exist: {SchemaPath}", schemaPath);
            return;
        }

        Directory.CreateDirectory(outputPath);

        // Generate models for each platform using unified schemas
        await GeneratePlatformModels(schemaPath, outputPath, baseNamespace);

        _logger.LogInformation("Unified model generation completed");
    }

    private async Task GeneratePlatformModels(string schemaPath, string outputPath, string baseNamespace)
    {
        var platforms = Directory.GetDirectories(schemaPath);
        
        foreach (var platformDir in platforms)
        {
            var platformName = Path.GetFileName(platformDir);
            var platformNamespace = $"{baseNamespace}.{FormatNamespacePart(platformName)}";
            var outputFile = Path.Combine(outputPath, platformName, $"{FormatNamespacePart(platformName)}Models.cs");
            
            _logger.LogInformation("Processing {Platform} platform", platformName);
            
            try
            {
                await GenerateForPlatform(platformDir, platformName, platformNamespace, outputFile);
                _logger.LogInformation("✅ Generated: {OutputFile}", Path.GetRelativePath(outputPath, outputFile));
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "❌ Failed to generate models for {Platform}", platformName);
            }
        }
    }

    private async Task GenerateForPlatform(string platformDir, string platformName, string platformNamespace, string outputFile)
    {
        string schemaFile;
        
        if (platformName == "roll20")
        {
            // Create unified schema for Roll20 on-the-fly
            schemaFile = await CreateRoll20UnifiedSchema(platformDir);
        }
        else if (platformName == "foundry")
        {
            // Generate unified schemas for each Foundry system
            await GenerateFoundrySystemModels(platformDir, platformNamespace, outputFile);
            return;
        }
        else if (platformName == "pathbuilder")
        {
            // Pathbuilder uses all individual schemas in directory
            var schemaFiles = Directory.GetFiles(platformDir, "*.json");
            if (schemaFiles.Length == 0)
            {
                throw new FileNotFoundException($"No schema files found in {platformDir}");
            }
            
            // For now, process first schema file (they should be unified)
            schemaFile = schemaFiles[0];
        }
        else
        {
            throw new NotSupportedException($"Platform {platformName} not supported");
        }

        Directory.CreateDirectory(Path.GetDirectoryName(outputFile)!);
        
        var generatedCode = await _schemaGenerator.GenerateFromSchemaFileAsync(schemaFile, platformNamespace);
        await _schemaGenerator.SaveGeneratedCodeAsync(generatedCode, outputFile);
    }

    private async Task GenerateFoundrySystemModels(string foundryDir, string baseNamespace, string outputFile)
    {
        var systemsDir = Path.Combine(foundryDir, "systems");
        if (!Directory.Exists(systemsDir))
        {
            throw new DirectoryNotFoundException($"Foundry systems directory not found: {systemsDir}");
        }

        var systems = Directory.GetDirectories(systemsDir);
        var outputDir = Path.GetDirectoryName(outputFile)!;
        
        foreach (var systemDir in systems)
        {
            var systemName = Path.GetFileName(systemDir);
            var systemNamespace = $"{baseNamespace}.{FormatNamespacePart(systemName)}";
            var systemOutputFile = Path.Combine(outputDir, $"Foundry{FormatNamespacePart(systemName)}.cs");
            
            _logger.LogInformation("  📦 Generating Foundry {System} models", systemName);
            
            // Create unified schema for this system on-the-fly
            var unifiedSchemaFile = await CreateFoundryUnifiedSchema(foundryDir, systemName);
            
            Directory.CreateDirectory(Path.GetDirectoryName(systemOutputFile)!);
            
            var generatedCode = await _schemaGenerator.GenerateFromSchemaFileAsync(unifiedSchemaFile, systemNamespace);
            await _schemaGenerator.SaveGeneratedCodeAsync(generatedCode, systemOutputFile);
            
            _logger.LogInformation("  ✅ Generated: Foundry{System}.cs", FormatNamespacePart(systemName));
        }
    }

    private async Task<string> CreateFoundryUnifiedSchema(string foundryDir, string systemName)
    {
        var tempDir = Path.Combine(Path.GetTempPath(), "ttrpg-converter-schemas");
        Directory.CreateDirectory(tempDir);
        
        var unifiedSchemaPath = Path.Combine(tempDir, $"foundry-{systemName}-unified.json");
        
        // Combine common, core, and system schemas
        var commonDir = Path.Combine(foundryDir, "common");
        var coreDir = Path.Combine(foundryDir, "core");
        var systemDir = Path.Combine(foundryDir, "systems", systemName);
        
        var allSchemas = new Dictionary<string, object>();
        var definitions = new Dictionary<string, object>();
        
        // Read and combine all schemas, building definitions
        await CollectSchemasWithDefinitions(allSchemas, definitions, commonDir, "common");
        await CollectSchemasWithDefinitions(allSchemas, definitions, coreDir, "core");
        await CollectSchemasWithDefinitions(allSchemas, definitions, systemDir, systemName);
        
        var unifiedSchema = new
        {
            schema = "https://json-schema.org/draft-07/schema#",
            id = $"https://ttrpg-converter.com/schemas/foundry/{systemName}/unified.json",
            title = $"Foundry {systemName.ToUpper()} Unified Schema",
            type = "object",
            definitions = definitions,
            properties = allSchemas
        };
        
        var json = System.Text.Json.JsonSerializer.Serialize(unifiedSchema, new System.Text.Json.JsonSerializerOptions 
        { 
            WriteIndented = true,
            PropertyNamingPolicy = System.Text.Json.JsonNamingPolicy.CamelCase
        });
        
        await File.WriteAllTextAsync(unifiedSchemaPath, json);
        _logger.LogDebug("Created unified schema: {Path}", unifiedSchemaPath);
        
        return unifiedSchemaPath;
    }

    private async Task<string> CreateRoll20UnifiedSchema(string roll20Dir)
    {
        var tempDir = Path.Combine(Path.GetTempPath(), "ttrpg-converter-schemas");
        Directory.CreateDirectory(tempDir);
        
        var unifiedSchemaPath = Path.Combine(tempDir, "roll20-unified.json");
        
        var allSchemas = new Dictionary<string, object>();
        var definitions = new Dictionary<string, object>();
        
        // Read and combine all Roll20 schemas
        await CollectSchemasWithDefinitions(allSchemas, definitions, roll20Dir, "roll20");
        
        var unifiedSchema = new
        {
            schema = "https://json-schema.org/draft-07/schema#",
            id = "https://ttrpg-converter.com/schemas/roll20/unified.json",
            title = "Roll20 Unified Schema",
            description = "Combined schema for all Roll20 entity types",
            type = "object",
            definitions = definitions,
            properties = allSchemas
        };
        
        var json = System.Text.Json.JsonSerializer.Serialize(unifiedSchema, new System.Text.Json.JsonSerializerOptions 
        { 
            WriteIndented = true,
            PropertyNamingPolicy = System.Text.Json.JsonNamingPolicy.CamelCase
        });
        
        await File.WriteAllTextAsync(unifiedSchemaPath, json);
        _logger.LogDebug("Created unified Roll20 schema: {Path}", unifiedSchemaPath);
        
        return unifiedSchemaPath;
    }

    private async Task CollectSchemasWithDefinitions(Dictionary<string, object> properties, Dictionary<string, object> definitions, string directory, string prefix)
    {
        if (!Directory.Exists(directory)) return;
        
        var schemaFiles = Directory.GetFiles(directory, "*.json");
        var schemaContents = new Dictionary<string, System.Text.Json.JsonElement>();
        
        // First pass: Load all raw schemas
        foreach (var file in schemaFiles)
        {
            var fileName = Path.GetFileNameWithoutExtension(file);
            
            // Skip existing unified.json to avoid recursion
            if (fileName == "unified") continue;
            
            var content = await File.ReadAllTextAsync(file);
            var rawSchema = System.Text.Json.JsonSerializer.Deserialize<System.Text.Json.JsonElement>(content);
            
            schemaContents[fileName] = rawSchema;
        }
        
        // Second pass: Process schemas and convert external refs to internal refs
        foreach (var (fileName, rawSchema) in schemaContents)
        {
            // Convert external $ref to internal #/definitions/ refs
            var processedSchema = ConvertExternalReferences(rawSchema, schemaContents, prefix);
            
            // Convert to dictionary and clean metadata
            var schema = System.Text.Json.JsonSerializer.Deserialize<Dictionary<string, object>>(
                System.Text.Json.JsonSerializer.Serialize(processedSchema));
            
            if (schema != null)
            {
                schema.Remove("$schema");
                schema.Remove("$id");
                
                // Add to definitions for internal referencing
                var definitionName = prefix == "roll20" ? NormalizeFileName(fileName) : $"{prefix}_{NormalizeFileName(fileName)}";
                definitions[definitionName] = schema;
                
                // Also add main schemas as top-level properties
                if (IsMainSchema(fileName))
                {
                    var propertyName = prefix == "roll20" ? fileName : $"{prefix}_{fileName}";
                    properties[propertyName] = new Dictionary<string, object>
                    {
                        ["$ref"] = $"#/definitions/{definitionName}"
                    };
                }
            }
        }
    }

    private bool IsMainSchema(string fileName)
    {
        // These are typically the main entity schemas we want as top-level properties
        return fileName is "actor" or "item" or "scene" or "journal" or "macro" or "cards" or "world" or 
               "character" or "handout" or "page" or "ability" or "attribute" or "deck" or "card" or 
               "table" or "player" or "campaign" or "turn-entry" or "pdf" or "audio-track";
    }

    private static string NormalizeFileName(string fileName)
    {
        // Replace hyphens with underscores for consistent definition names
        return fileName.Replace("-", "_");
    }

    private System.Text.Json.JsonElement ConvertExternalReferences(System.Text.Json.JsonElement element, Dictionary<string, System.Text.Json.JsonElement> allSchemas, string prefix)
    {
        switch (element.ValueKind)
        {
            case System.Text.Json.JsonValueKind.Object:
                var obj = new Dictionary<string, System.Text.Json.JsonElement>();
                foreach (var prop in element.EnumerateObject())
                {
                    if (prop.Name == "$ref" && prop.Value.ValueKind == System.Text.Json.JsonValueKind.String)
                    {
                        var refValue = prop.Value.GetString();
                        if (refValue != null)
                        {
                            // Convert external file references to internal definitions
                            if (refValue.StartsWith("./") || refValue.StartsWith("../"))
                            {
                                var refFileName = Path.GetFileNameWithoutExtension(refValue);
                                
                                // Determine the correct definition name based on the reference path
                                string definitionName;
                                if (refValue.Contains("../common/"))
                                {
                                    definitionName = $"common_{NormalizeFileName(refFileName)}";
                                }
                                else if (refValue.Contains("../core/"))
                                {
                                    definitionName = $"core_{NormalizeFileName(refFileName)}";
                                }
                                else if (refValue.StartsWith("./"))
                                {
                                    // Same directory reference
                                    definitionName = prefix == "roll20" ? NormalizeFileName(refFileName) : $"{prefix}_{NormalizeFileName(refFileName)}";
                                }
                                else
                                {
                                    // Default case
                                    definitionName = prefix == "roll20" ? NormalizeFileName(refFileName) : $"{prefix}_{NormalizeFileName(refFileName)}";
                                }
                                
                                obj[prop.Name] = System.Text.Json.JsonSerializer.SerializeToElement($"#/definitions/{definitionName}");
                            }
                            else
                            {
                                // Keep other references as-is
                                obj[prop.Name] = prop.Value;
                            }
                        }
                    }
                    else
                    {
                        obj[prop.Name] = ConvertExternalReferences(prop.Value, allSchemas, prefix);
                    }
                }
                return System.Text.Json.JsonSerializer.SerializeToElement(obj);
                
            case System.Text.Json.JsonValueKind.Array:
                var arr = new List<System.Text.Json.JsonElement>();
                foreach (var item in element.EnumerateArray())
                {
                    arr.Add(ConvertExternalReferences(item, allSchemas, prefix));
                }
                return System.Text.Json.JsonSerializer.SerializeToElement(arr);
                
            default:
                return element;
        }
    }

    private static string FormatNamespacePart(string part)
    {
        return string.Join("", part.Split('-').Select(p => 
            p.Length > 0 ? char.ToUpper(p[0]) + p[1..].ToLower() : p));
    }

    private static string FormatClassName(string fileName)
    {
        return FormatNamespacePart(fileName);
    }
}
