namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Internal representation of a TTRPG spell, format-agnostic
/// </summary>
public interface ISpell
{
    /// <summary>
    /// Spell's display name
    /// </summary>
    string Name { get; }
    
    /// <summary>
    /// Spell level (0-9 for most systems)
    /// </summary>
    int Level { get; }
    
    /// <summary>
    /// Magic school (evocation, enchantment, etc.)
    /// </summary>
    string? School { get; }
    
    /// <summary>
    /// Spell description/effect
    /// </summary>
    string? Description { get; }
    
    /// <summary>
    /// Casting time (action, bonus action, reaction, etc.)
    /// </summary>
    string? CastingTime { get; }
    
    /// <summary>
    /// Spell range
    /// </summary>
    string? Range { get; }
    
    /// <summary>
    /// Spell duration
    /// </summary>
    string? Duration { get; }
    
    /// <summary>
    /// Whether spell requires concentration
    /// </summary>
    bool RequiresConcentration { get; }
    
    /// <summary>
    /// Spell components (verbal, somatic, material)
    /// </summary>
    SpellComponents Components { get; }
    
    /// <summary>
    /// Format-specific data preservation
    /// </summary>
    IDictionary<string, object> ExtensionData { get; }
}

/// <summary>
/// Spell component requirements
/// </summary>
public record SpellComponents(
    bool Verbal = false,
    bool Somatic = false, 
    bool Material = false,
    string? MaterialDescription = null
);
