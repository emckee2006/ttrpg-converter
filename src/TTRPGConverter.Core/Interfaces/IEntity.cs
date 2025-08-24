using Microsoft.Extensions.Logging;

namespace TTRPGConverter.Core.Interfaces;

/// <summary>
/// Base interface for all TTRPG entities
/// </summary>
public interface IEntity
{
    /// <summary>
    /// Unique identifier for the entity
    /// </summary>
    string Id { get; }
    
    /// <summary>
    /// Human-readable name of the entity
    /// </summary>
    string Name { get; }
}

/// <summary>
/// Interface for entities that can be converted between platforms
/// </summary>
/// <typeparam name="TTarget">Target platform entity type</typeparam>
public interface IConvertible<out TTarget> where TTarget : IEntity
{
    /// <summary>
    /// Convert this entity to the target platform format
    /// </summary>
    /// <param name="context">Conversion context with settings and mappings</param>
    /// <returns>Converted entity</returns>
    TTarget ConvertTo(IConversionContext? context = null);
}

/// <summary>
/// Interface for bidirectional conversion between platforms
/// </summary>
/// <typeparam name="TSource">Source platform entity type</typeparam>
/// <typeparam name="TTarget">Target platform entity type</typeparam>
public interface IBidirectionalConvertible<in TSource, out TTarget> 
    where TSource : IEntity 
    where TTarget : IEntity
{
    /// <summary>
    /// Convert from source to target platform format
    /// </summary>
    /// <param name="source">Source entity</param>
    /// <param name="context">Conversion context</param>
    /// <returns>Converted target entity</returns>
    TTarget ConvertFrom(TSource source, IConversionContext? context = null);
}

/// <summary>
/// Context information for entity conversion operations
/// </summary>
public interface IConversionContext
{
    /// <summary>
    /// Source platform identifier (e.g., "roll20", "foundry", "pathbuilder")
    /// </summary>
    string SourcePlatform { get; }
    
    /// <summary>
    /// Target platform identifier
    /// </summary>
    string TargetPlatform { get; }
    
    /// <summary>
    /// Conversion settings and options
    /// </summary>
    IReadOnlyDictionary<string, object> Settings { get; }
    
    /// <summary>
    /// Asset mapping for images, audio, etc.
    /// </summary>
    IAssetMapper AssetMapper { get; }
    
    /// <summary>
    /// Logger for conversion operations
    /// </summary>
    ILogger Logger { get; }
}

/// <summary>
/// Interface for entities with ownership/permission information
/// </summary>
public interface IOwnable
{
    /// <summary>
    /// Ownership information (user IDs and permission levels)
    /// </summary>
    Dictionary<string, int>? Ownership { get; }
}

/// <summary>
/// Interface for entities with folder organization
/// </summary>
public interface IFolderable
{
    /// <summary>
    /// Folder ID for organization
    /// </summary>
    string? Folder { get; }
}

/// <summary>
/// Interface for entities with sorting capability
/// </summary>
public interface ISortable
{
    /// <summary>
    /// Sort order value
    /// </summary>
    int Sort { get; }
}

/// <summary>
/// Interface for entities with image assets
/// </summary>
public interface IImageAsset
{
    /// <summary>
    /// Path to the entity's image
    /// </summary>
    string? Img { get; }
}
