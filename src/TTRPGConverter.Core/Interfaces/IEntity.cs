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
    /// <returns>Converted entity</returns>
    TTarget ConvertTo();
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
