using System.ComponentModel.DataAnnotations;

namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Internal representation of a TTRPG character, format-agnostic
/// </summary>
public interface ICharacter
{
    /// <summary>
    /// Character's display name
    /// </summary>
    string Name { get; }
    
    /// <summary>
    /// Character level (1-20 for most systems)
    /// </summary>
    int Level { get; }
    
    /// <summary>
    /// Character's class(es) and levels
    /// </summary>
    IReadOnlyList<CharacterClass> Classes { get; }
    
    /// <summary>
    /// Six core ability scores
    /// </summary>
    AbilityScores Abilities { get; }
    
    /// <summary>
    /// Character's equipment and items
    /// </summary>
    IReadOnlyList<IItem> Items { get; }
    
    /// <summary>
    /// Character's spells and spell-like abilities
    /// </summary>
    IReadOnlyList<ISpell> Spells { get; }
    
    /// <summary>
    /// Format-specific data preservation
    /// </summary>
    IDictionary<string, object> ExtensionData { get; }
}

/// <summary>
/// Character class and level information
/// </summary>
public record CharacterClass(string Name, int Level, string? Subclass = null);

/// <summary>
/// Six core RPG ability scores
/// </summary>
public record AbilityScores(
    int Strength,
    int Dexterity, 
    int Constitution,
    int Intelligence,
    int Wisdom,
    int Charisma
)
{
    /// <summary>
    /// Calculate ability modifier for a score
    /// </summary>
    public static int GetModifier(int score) => (score - 10) / 2;
    
    public int StrengthModifier => GetModifier(Strength);
    public int DexterityModifier => GetModifier(Dexterity);
    public int ConstitutionModifier => GetModifier(Constitution);
    public int IntelligenceModifier => GetModifier(Intelligence);
    public int WisdomModifier => GetModifier(Wisdom);
    public int CharismaModifier => GetModifier(Charisma);
}
