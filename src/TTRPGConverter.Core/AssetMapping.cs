using System.ComponentModel.DataAnnotations;

namespace TTRPGConverter.Core;

/// <summary>
/// Represents a mapping between a remote asset URL and its local, cached path.
/// </summary>
public record AssetMapping
{
    [Key]
    public required string SourceUrl { get; init; }

    public required string LocalPath { get; set; }
}
