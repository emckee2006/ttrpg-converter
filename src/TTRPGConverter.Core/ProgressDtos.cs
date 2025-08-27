namespace TTRPGConverter.Core;

public record ProgressUpdate(
    double? OverallMaxValue = null,
    double? OverallIncrement = null,
    string? OverallDescription = null,
    double? DetailMaxValue = null,
    double? DetailIncrement = null,
    string? DetailDescription = null
);
