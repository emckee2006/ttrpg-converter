namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Concrete implementation of ICharacter for internal representation
/// </summary>
public class Character : ICharacter
{
    public string Name { get; set; } = string.Empty;
    public int Level { get; set; }
    public IReadOnlyList<CharacterClass> Classes { get; set; } = Array.Empty<CharacterClass>();
    public AbilityScores Abilities { get; set; } = new(10, 10, 10, 10, 10, 10);
    public IReadOnlyList<IItem> Items { get; set; } = Array.Empty<IItem>();
    public IReadOnlyList<ISpell> Spells { get; set; } = Array.Empty<ISpell>();
    public IDictionary<string, object> ExtensionData { get; set; } = new Dictionary<string, object>();
}
