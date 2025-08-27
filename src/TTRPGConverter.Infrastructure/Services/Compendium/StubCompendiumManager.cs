using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using TTRPGConverter.Core.Compendium;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public class StubCompendiumManager : ICompendiumManager
{
    public Task<CompendiumItem?> FindEntityAsync(string entityType, string entityName, string? system = null)
    {
        return Task.FromResult<CompendiumItem?>(null);
    }

    public Task<IEnumerable<CompendiumItem>> FindAllCandidatesAsync(string entityType, string entityName, string? system = null)
    {
        return Task.FromResult(Enumerable.Empty<CompendiumItem>());
    }

    public Task<IEnumerable<CompendiumItem>> GetEntitiesByTypeAsync(string entityType)
    {
        return Task.FromResult(Enumerable.Empty<CompendiumItem>());
    }
}
