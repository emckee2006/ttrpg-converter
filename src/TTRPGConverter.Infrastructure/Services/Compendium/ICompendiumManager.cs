namespace TTRPGConverter.Infrastructure.Services.Compendium;

/// <summary>
/// Defines the contract for a service that provides access to compendium data.
/// </summary>
public interface ICompendiumManager
{
    /// <summary>
    /// Finds the single best ("primary") compendium item that matches the specified type and name.
    /// </summary>
    /// <param name="entityType">The type of the entity to find (e.g., "spell", "npc").</param>
    /// <param name="entityName">The name of the entity to find.</param>
    /// <returns>The primary CompendiumItem if found; otherwise, null.</returns>
    CompendiumItem? FindEntity(string entityType, string entityName);

    /// <summary>
    /// Finds all possible compendium items (both primary and secondary) that match the specified type and name.
    /// </summary>
    /// <param name="entityType">The type of the entity to find.</param>
    /// <param name="entityName">The name of the entity to find.</param>
    /// <returns>An enumerable collection of all matching CompendiumItem candidates.</returns>
    IEnumerable<CompendiumItem> FindAllCandidates(string entityType, string entityName);

    /// <summary>
    /// Gets all compendium items of a specific type.
    /// </summary>
    /// <param name="entityType">The type of entities to retrieve.</param>
    /// <returns>An enumerable collection of all items of the specified type.</returns>
    IEnumerable<CompendiumItem> GetEntitiesByType(string entityType);
}
