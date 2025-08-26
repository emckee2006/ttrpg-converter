using System.Text.Json;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

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
    public bool IsPrimary { get; set; } = false;
    public string? System { get; set; }
}
