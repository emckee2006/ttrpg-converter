using System.Collections.Generic;

namespace TTRPGConverter.Core;

public record CompendiumBuildResult
{
    public required IReadOnlyList<PackProcessingResult> ProcessedPacks { get; init; }
}

public record PackProcessingResult
{
    public required string PackName { get; init; }
    public required string ModuleName { get; init; }
    public bool IsSuccess { get; init; }
    public string? ErrorMessage { get; init; }
    public int ItemsProcessed { get; init; }
}
