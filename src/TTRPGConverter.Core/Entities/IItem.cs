namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Internal representation of a TTRPG item, format-agnostic
/// </summary>
public interface IItem
{
    /// <summary>
    /// Item's display name
    /// </summary>
    string Name { get; }
    
    /// <summary>
    /// Item type (weapon, armor, consumable, etc.)
    /// </summary>
    ItemType Type { get; }
    
    /// <summary>
    /// Item description
    /// </summary>
    string? Description { get; }
    
    /// <summary>
    /// Item quantity
    /// </summary>
    int Quantity { get; }
    
    /// <summary>
    /// Item weight per unit
    /// </summary>
    decimal? Weight { get; }
    
    /// <summary>
    /// Item value/cost
    /// </summary>
    decimal? Value { get; }
    
    /// <summary>
    /// Whether item is equipped/attuned
    /// </summary>
    bool IsEquipped { get; }
    
    /// <summary>
    /// Format-specific data preservation
    /// </summary>
    IDictionary<string, object> ExtensionData { get; }
}

/// <summary>
/// Common item types across TTRPG systems
/// </summary>
public enum ItemType
{
    Weapon,
    Armor, 
    Shield,
    Consumable,
    Tool,
    Treasure,
    Ammunition,
    Equipment,
    MagicItem,
    Other
}
