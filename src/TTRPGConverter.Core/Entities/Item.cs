namespace TTRPGConverter.Core.Entities;

/// <summary>
/// Concrete implementation of IItem for internal representation
/// </summary>
public class Item : IItem
{
    public string Name { get; set; } = string.Empty;
    public ItemType Type { get; set; }
    public string? Description { get; set; }
    public int Quantity { get; set; } = 1;
    public decimal? Weight { get; set; }
    public decimal? Value { get; set; }
    public bool IsEquipped { get; set; }
    public IDictionary<string, object> ExtensionData { get; set; } = new Dictionary<string, object>();
}
