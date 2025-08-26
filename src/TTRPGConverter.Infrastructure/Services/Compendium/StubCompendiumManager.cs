namespace TTRPGConverter.Infrastructure.Services.Compendium;

/// <summary>
/// A temporary, stub implementation of the ICompendiumManager for development purposes.
/// It returns no data, allowing other components to be built without a live database connection.
/// </summary>
public class StubCompendiumManager : ICompendiumManager
{
    public CompendiumItem? FindEntity(string entityType, string entityName)
    {
        // Always return null to simulate a cache miss.
        return null;
    }

    public IEnumerable<CompendiumItem> FindAllCandidates(string entityType, string entityName)
    {
        // Always return an empty list.
        return [];
    }

    public IEnumerable<CompendiumItem> GetEntitiesByType(string entityType)
    {
        // Always return an empty list.
        return [];
    }
}
