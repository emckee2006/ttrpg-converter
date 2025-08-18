using NJsonSchema;
using NJsonSchema.CodeGeneration.CSharp;
using System.Text.Json;

namespace TTRPGConverter.Core.Schemas;

/// <summary>
/// Generates C# classes from JSON schemas for TTRPG platform data structures
/// </summary>
public class SchemaGenerator
{
    private readonly CSharpGeneratorSettings _settings;

    public SchemaGenerator()
    {
        _settings = new CSharpGeneratorSettings
        {
            Namespace = "TTRPGConverter.Core.Models",
            GenerateDataAnnotations = true,
            JsonLibrary = CSharpJsonLibrary.SystemTextJson,
            GenerateNullableReferenceTypes = true,
            GenerateOptionalPropertiesAsNullable = true,
            ArrayType = "System.Collections.Generic.List",
            DictionaryType = "System.Collections.Generic.Dictionary",
            DateTimeType = "System.DateTimeOffset"
        };
    }

    /// <summary>
    /// Generate C# model classes from a JSON schema file
    /// </summary>
    /// <param name="schemaPath">Path to JSON schema file</param>
    /// <param name="outputNamespace">Target namespace for generated classes</param>
    /// <returns>Generated C# code</returns>
    public async Task<string> GenerateFromSchemaFileAsync(string schemaPath, string outputNamespace)
    {
        if (!File.Exists(schemaPath))
            throw new FileNotFoundException($"Schema file not found: {schemaPath}");

        var schemaJson = await File.ReadAllTextAsync(schemaPath);
        return await GenerateFromSchemaJsonAsync(schemaJson, outputNamespace, schemaPath);
    }

    /// <summary>
    /// Generate C# model classes from JSON schema string
    /// </summary>
    /// <param name="schemaJson">JSON schema as string</param>
    /// <param name="outputNamespace">Target namespace for generated classes</param>
    /// <param name="schemaFilePath">Path to JSON schema file (optional)</param>
    /// <returns>Generated C# code</returns>
    public async Task<string> GenerateFromSchemaJsonAsync(string schemaJson, string outputNamespace, string? schemaFilePath = null)
    {
        JsonSchema schema;
        
        if (!string.IsNullOrEmpty(schemaFilePath))
        {
            // Use file path for proper $ref resolution
            schema = await JsonSchema.FromFileAsync(schemaFilePath);
        }
        else
        {
            // Fallback to JSON string
            schema = await JsonSchema.FromJsonAsync(schemaJson);
        }
        
        var settings = new CSharpGeneratorSettings
        {
            Namespace = outputNamespace,
            JsonLibrary = CSharpJsonLibrary.SystemTextJson,
            GenerateNullableReferenceTypes = true,
            GenerateDataAnnotations = true,
            ClassStyle = CSharpClassStyle.Poco,
            GenerateJsonMethods = false,
            GenerateOptionalPropertiesAsNullable = true,
            ArrayType = "System.Collections.Generic.List",
            DictionaryType = "System.Collections.Generic.Dictionary",
            DateTimeType = "System.DateTimeOffset"
        };

        var generator = new CSharpGenerator(schema, settings);
        return generator.GenerateFile();
    }

    /// <summary>
    /// Generate C# model classes from sample JSON data
    /// </summary>
    /// <param name="sampleJson">Sample JSON data</param>
    /// <param name="className">Name for the root class</param>
    /// <param name="outputNamespace">Target namespace for generated classes</param>
    /// <returns>Generated C# code</returns>
    public async Task<string> GenerateFromSampleJsonAsync(string sampleJson, string className, string outputNamespace)
    {
        var schema = JsonSchema.FromSampleJson(sampleJson);
        schema.Title = className;
        
        var settings = new CSharpGeneratorSettings
        {
            Namespace = outputNamespace,
            GenerateDataAnnotations = _settings.GenerateDataAnnotations,
            JsonLibrary = _settings.JsonLibrary,
            GenerateNullableReferenceTypes = _settings.GenerateNullableReferenceTypes,
            GenerateOptionalPropertiesAsNullable = _settings.GenerateOptionalPropertiesAsNullable,
            ArrayType = _settings.ArrayType,
            DictionaryType = _settings.DictionaryType,
            DateTimeType = _settings.DateTimeType
        };
        
        var generator = new CSharpGenerator(schema, settings);
        return generator.GenerateFile();
    }

    /// <summary>
    /// Save generated code to file
    /// </summary>
    /// <param name="generatedCode">Generated C# code</param>
    /// <param name="outputPath">Path to save the generated file</param>
    public async Task SaveGeneratedCodeAsync(string generatedCode, string outputPath)
    {
        var directory = Path.GetDirectoryName(outputPath);
        if (!string.IsNullOrEmpty(directory) && !Directory.Exists(directory))
        {
            Directory.CreateDirectory(directory);
        }

        await File.WriteAllTextAsync(outputPath, generatedCode);
    }
}
