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
        _logger.LogInformation("Starting model generation from schemas in: {SchemaPath}", schemaPath);
        _logger.LogInformation("Output path: {OutputPath}", outputPath);
        _logger.LogInformation("Base namespace: {BaseNamespace}", baseNamespace);

        if (!Directory.Exists(schemaPath))
        {
            _logger.LogError("Schema path does not exist: {SchemaPath}", schemaPath);
            return;
        }

        Directory.CreateDirectory(outputPath);

        var schemaFiles = Directory.GetFiles(schemaPath, "*.json", SearchOption.AllDirectories);
        _logger.LogInformation("Found {Count} schema files", schemaFiles.Length);

        foreach (var schemaFile in schemaFiles)
        {
            try
            {
                var relativePath = Path.GetRelativePath(schemaPath, schemaFile);
                var pathParts = relativePath.Split(Path.DirectorySeparatorChar);
                
                var namespaceParts = pathParts.Take(pathParts.Length - 1).ToArray();
                var fullNamespace = namespaceParts.Length > 0 
                    ? $"{baseNamespace}.{string.Join(".", namespaceParts.Select(FormatNamespacePart))}"
                    : baseNamespace;

                var fileName = Path.GetFileNameWithoutExtension(schemaFile);
                var outputDir = Path.Combine(outputPath, string.Join(Path.DirectorySeparatorChar.ToString(), namespaceParts));
                var outputFile = Path.Combine(outputDir, $"{FormatClassName(fileName)}.cs");

                _logger.LogDebug("Processing: {SchemaFile} -> {OutputFile}", relativePath, outputFile);

                var generatedCode = await _schemaGenerator.GenerateFromSchemaFileAsync(schemaFile, fullNamespace);
                await _schemaGenerator.SaveGeneratedCodeAsync(generatedCode, outputFile);
                
                _logger.LogInformation("Generated: {OutputFile}", Path.GetRelativePath(outputPath, outputFile));
            }
            catch (Exception ex)
            {
                _logger.LogWarning(ex, "Skipped {SchemaFile}: {Error}", Path.GetRelativePath(schemaPath, schemaFile), ex.Message);
            }
        }

        _logger.LogInformation("Model generation completed");
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
