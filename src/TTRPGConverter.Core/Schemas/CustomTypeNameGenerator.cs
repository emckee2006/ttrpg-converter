using NJsonSchema;
using NJsonSchema.CodeGeneration;

namespace TTRPGConverter.Core.Schemas;

/// <summary>
/// Custom type name generator to avoid naming conflicts and ensure unique class names
/// </summary>
public class CustomTypeNameGenerator : ITypeNameGenerator
{
    private readonly Dictionary<string, int> _typeCounter = new();
    private readonly HashSet<string> _reservedNames = new()
    {
        "Json", "OwnershipDefault", "JsonType", "TextFormat", "System", "Object", "String"
    };

    public string Generate(JsonSchema schema, string? typeNameHint, IEnumerable<string> reservedTypeNames)
    {
        var baseName = typeNameHint ?? "GeneratedClass";
        
        // Clean up the name
        baseName = CleanTypeName(baseName);
        
        // Avoid conflicts with reserved names
        if (_reservedNames.Contains(baseName) || reservedTypeNames.Contains(baseName))
        {
            baseName = $"{baseName}Model";
        }
        
        // Ensure uniqueness
        if (_typeCounter.ContainsKey(baseName))
        {
            _typeCounter[baseName]++;
            return $"{baseName}{_typeCounter[baseName]}";
        }
        
        _typeCounter[baseName] = 0;
        return baseName;
    }

    private static string CleanTypeName(string name)
    {
        if (string.IsNullOrEmpty(name))
            return "GeneratedClass";

        // Remove invalid characters and make PascalCase
        var cleaned = string.Concat(name.Split(new char[] { '-', '_', ' ', '.', '/', '\\' }, StringSplitOptions.RemoveEmptyEntries)
            .Select(part => char.ToUpperInvariant(part[0]) + part.Substring(1).ToLowerInvariant()));

        // Ensure it starts with a letter
        if (!char.IsLetter(cleaned[0]))
            cleaned = "Class" + cleaned;

        return cleaned;
    }
}
