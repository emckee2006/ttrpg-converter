using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using TTRPGConverter.Core.Compendium;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public class SqliteCompendiumManager : ICompendiumManager
{
    private readonly TTRPGConverterContext _context;

    public SqliteCompendiumManager(TTRPGConverterContext context)
    {
        _context = context;
    }

    public async Task<CompendiumItem?> FindEntityAsync(string entityType, string entityName, string? system = null)
    {
        return await _context.CompendiumItems
            .AsNoTracking()
            .FirstOrDefaultAsync(ci => 
                ci.IsPrimary && 
                ci.Type == entityType && 
                ci.Name == entityName && 
                (system == null || ci.System == system));
    }

    public async Task<IEnumerable<CompendiumItem>> FindAllCandidatesAsync(string entityType, string entityName, string? system = null)
    {
        return await _context.CompendiumItems
            .AsNoTracking()
            .Where(ci => 
                ci.Type == entityType && 
                ci.Name == entityName && 
                (system == null || ci.System == system))
            .ToListAsync();
    }

    public async Task<IEnumerable<CompendiumItem>> GetEntitiesByTypeAsync(string entityType)
    {
        return await _context.CompendiumItems
            .AsNoTracking()
            .Where(ci => ci.Type == entityType && ci.IsPrimary)
            .ToListAsync();
    }
}
