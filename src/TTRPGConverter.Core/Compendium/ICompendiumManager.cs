using System.Collections.Generic;
using System.Threading.Tasks;

namespace TTRPGConverter.Core.Compendium;

public interface ICompendiumManager
{
    Task<CompendiumItem?> FindEntityAsync(string entityType, string entityName, string? system = null);

    Task<IEnumerable<CompendiumItem>> FindAllCandidatesAsync(string entityType, string entityName, string? system = null);

    Task<IEnumerable<CompendiumItem>> GetEntitiesByTypeAsync(string entityType);
}
