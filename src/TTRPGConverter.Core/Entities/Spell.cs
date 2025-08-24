namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Concrete implementation of ISpell for internal representation
/// </summary>
public class Spell : ISpell
{
    public string Name { get; set; } = string.Empty;
    public int Level { get; set; }
    public string? School { get; set; }
    public string? Description { get; set; }
    public string? CastingTime { get; set; }
    public string? Range { get; set; }
    public string? Duration { get; set; }
    public bool RequiresConcentration { get; set; }
    public SpellComponents Components { get; set; } = new();
    public IDictionary<string, object> ExtensionData { get; set; } = new Dictionary<string, object>();
}
