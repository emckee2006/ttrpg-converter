namespace TTRPGConverter.Core;

/// <summary>
/// A flexible container for reporting different kinds of progress from the backend services to the UI.
/// </summary>
public record BuildProgressReport
{
    /// <summary>
    /// Contains a generic progress update for progress bars and status text.
    /// Will be null if this report contains a specific pack result.
    /// </summary>
    public ProgressUpdate? ProgressUpdate { get; init; }

    /// <summary>
    /// Contains the result of a single compendium pack being processed.
    /// Will be null if this report is just a generic progress update.
    /// </summary>
    public PackProcessingResult? PackResult { get; init; }
}
