// using TTRPGConverter.Core.Models.Foundry.Common; // Temporarily commented for model generation

namespace TTRPGConverter.Core.Interfaces;

/// <summary>
/// Base interface for all Foundry VTT documents
/// </summary>
public interface IFoundryDocument
{
    /// <summary>
    /// Document unique identifier
    /// </summary>
    string Id { get; set; }
    
    /// <summary>
    /// Document name
    /// </summary>
    string Name { get; set; }
    
    /// <summary>
    /// Document image/icon
    /// </summary>
    string? Img { get; set; }
    
    /// <summary>
    /// Parent folder identifier
    /// </summary>
    string? Folder { get; set; }
    
    /// <summary>
    /// Sort order
    /// </summary>
    int Sort { get; set; }
    
    /// <summary>
    /// Document ownership permissions
    /// </summary>
    object? Ownership { get; set; } // Temporarily changed from Ownership type
    
    /// <summary>
    /// Module/system flags
    /// </summary>
    Dictionary<string, object>? Flags { get; set; }
    
    /// <summary>
    /// Document statistics
    /// </summary>
    object? Stats { get; set; } // Temporarily changed from DocumentStats type
}

/// <summary>
/// Interface for Foundry VTT actors (characters, NPCs, etc.)
/// </summary>
public interface IFoundryActor : IFoundryDocument
{
    /// <summary>
    /// Actor type (character, npc, vehicle, etc.)
    /// </summary>
    string Type { get; set; }
    
    /// <summary>
    /// System-specific data
    /// </summary>
    object? System { get; set; }
    
    /// <summary>
    /// Actor's items
    /// </summary>
    List<IFoundryItem>? Items { get; set; }
    
    /// <summary>
    /// Active effects
    /// </summary>
    List<object>? Effects { get; set; }
}

/// <summary>
/// Interface for Foundry VTT items (equipment, spells, features, etc.)
/// </summary>
public interface IFoundryItem : IFoundryDocument
{
    /// <summary>
    /// Item type (weapon, armor, spell, etc.)
    /// </summary>
    string Type { get; set; }
    
    /// <summary>
    /// System-specific data
    /// </summary>
    object? System { get; set; }
    
    /// <summary>
    /// Active effects
    /// </summary>
    List<object>? Effects { get; set; }
}

/// <summary>
/// Interface for Foundry VTT scenes (maps/battlefields)
/// </summary>
public interface IFoundryScene : IFoundryDocument
{
    /// <summary>
    /// Scene width in pixels
    /// </summary>
    int Width { get; set; }
    
    /// <summary>
    /// Scene height in pixels
    /// </summary>
    int Height { get; set; }
    
    /// <summary>
    /// Background image configuration
    /// </summary>
    object? Background { get; set; }
    
    /// <summary>
    /// Grid configuration
    /// </summary>
    object? Grid { get; set; }
    
    /// <summary>
    /// Tokens on the scene
    /// </summary>
    List<object>? Tokens { get; set; }
    
    /// <summary>
    /// Walls and barriers
    /// </summary>
    List<object>? Walls { get; set; }
    
    /// <summary>
    /// Light sources
    /// </summary>
    List<object>? Lights { get; set; }
    
    /// <summary>
    /// Ambient sounds
    /// </summary>
    List<object>? Sounds { get; set; }
}

/// <summary>
/// Interface for Foundry VTT journal entries
/// </summary>
public interface IFoundryJournal : IFoundryDocument
{
    /// <summary>
    /// Journal pages
    /// </summary>
    List<object>? Pages { get; set; }
}

/// <summary>
/// Interface for Foundry VTT macros
/// </summary>
public interface IFoundryMacro : IFoundryDocument
{
    /// <summary>
    /// Macro type (script, chat)
    /// </summary>
    string Type { get; set; }
    
    /// <summary>
    /// Macro scope (global, actors, actor)
    /// </summary>
    string Scope { get; set; }
    
    /// <summary>
    /// Macro command/script
    /// </summary>
    string Command { get; set; }
}
