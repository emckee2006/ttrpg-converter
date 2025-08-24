namespace TTRPGConverter.Core.Interfaces;

/// <summary>
/// Base interface for conversion services between TTRPG platforms
/// </summary>
public interface IConversionService
{
    /// <summary>
    /// Source platform identifier
    /// </summary>
    string SourcePlatform { get; }
    
    /// <summary>
    /// Target platform identifier  
    /// </summary>
    string TargetPlatform { get; }
    
    /// <summary>
    /// Check if this service can handle the given conversion
    /// </summary>
    /// <param name="sourcePlatform">Source platform</param>
    /// <param name="targetPlatform">Target platform</param>
    /// <returns>True if conversion is supported</returns>
    bool CanConvert(string sourcePlatform, string targetPlatform);
}

/// <summary>
/// Interface for campaign-level conversion services
/// </summary>
public interface ICampaignConversionService : IConversionService
{
    /// <summary>
    /// Convert a complete campaign from source to target platform
    /// </summary>
    /// <param name="sourceCampaignPath">Path to source campaign file/directory</param>
    /// <param name="targetPath">Target output path</param>
    /// <param name="context">Conversion context</param>
    /// <returns>Conversion result with metadata</returns>
    Task<ConversionResult> ConvertCampaignAsync(string sourceCampaignPath, string targetPath, IConversionContext context);
}

/// <summary>
/// Interface for character-level conversion services
/// </summary>
public interface ICharacterConversionService : IConversionService
{
    /// <summary>
    /// Convert a character from source to target platform
    /// </summary>
    /// <param name="sourceCharacter">Source character data</param>
    /// <param name="context">Conversion context</param>
    /// <returns>Converted character</returns>
    Task<TTarget> ConvertCharacterAsync<TSource, TTarget>(TSource sourceCharacter, IConversionContext context) 
        where TSource : IEntity 
        where TTarget : IEntity;
}

/// <summary>
/// Result of a conversion operation
/// </summary>
public record ConversionResult(
    bool Success,
    string? TargetPath = null,
    List<string>? Warnings = null,
    List<string>? Errors = null,
    Dictionary<string, object>? Metadata = null
);
