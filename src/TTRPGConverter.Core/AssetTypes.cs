using System;
using System.Collections.Generic;

namespace TTRPGConverter.Core;

public record AssetRequest
{
    public required string Roll20Url { get; init; }
    public required string DestinationPath { get; init; }
    public required string EntityId { get; init; }
    public required string EntityType { get; init; }
    public required string AssetType { get; init; } // e.g., "token", "avatar", "handout"
}

public record ProcessedAsset
{
    public required AssetRequest Request { get; init; }
    public required string FinalPath { get; init; }
}

public record AssetProcessingError
{
    public required AssetRequest Request { get; init; }
    public required Exception Error { get; init; }
}

public record AssetProcessingResult
{
    public required IReadOnlyList<ProcessedAsset> ProcessedAssets { get; init; }
    public required IReadOnlyList<AssetProcessingError> Errors { get; init; }
    public required TimeSpan ProcessingTime { get; init; }
}
