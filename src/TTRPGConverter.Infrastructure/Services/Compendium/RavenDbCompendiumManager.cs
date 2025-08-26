using Microsoft.Extensions.Logging;
using Raven.Client.Documents;
using Raven.Embedded;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

/// <summary>
/// Runtime implementation of ICompendiumManager that connects to and queries a pre-built RavenDB cache.
/// </summary>
public class RavenDbCompendiumManager : ICompendiumManager, IDisposable
{
    private readonly ILogger<RavenDbCompendiumManager> _logger;
    private readonly IDocumentStore? _store;

    public RavenDbCompendiumManager(ILogger<RavenDbCompendiumManager> logger, string cachePath)
    {
        _logger = logger;
        if (Directory.Exists(cachePath))
        {
            try
            {
                var serverOptions = new ServerOptions { DataDirectory = cachePath, ServerUrl = "http://127.0.0.1:8081" };
                EmbeddedServer.Instance.StartServer(serverOptions);
                _store = EmbeddedServer.Instance.GetDocumentStore(new DatabaseOptions("Compendium"));
                new CompendiumItem_Index().Execute(_store); // Ensure index exists
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "❌ Failed to connect to RavenDB cache at {Path}", cachePath);
            }
        }
        else
        {
            _logger.LogWarning("Compendium cache not found at {Path}. Run 'update-compendium' to create it.", cachePath);
        }
    }

    public CompendiumItem? FindEntity(string entityType, string entityName)
    {
        if (_store == null) return null;
        using var session = _store.OpenSession();
        return session.Query<CompendiumItem, CompendiumItem_Index>().FirstOrDefault(x => x.Type == entityType && x.Name == entityName && x.IsPrimary);
    }

    public IEnumerable<CompendiumItem> FindAllCandidates(string entityType, string entityName)
    {
        if (_store == null) return Enumerable.Empty<CompendiumItem>();
        using var session = _store.OpenSession();
        return session.Query<CompendiumItem, CompendiumItem_Index>().Where(x => x.Type == entityType && x.Name == entityName).ToList();
    }

    public IEnumerable<CompendiumItem> GetEntitiesByType(string entityType)
    {
        if (_store == null) return Enumerable.Empty<CompendiumItem>();
        using var session = _store.OpenSession();
        return session.Query<CompendiumItem, CompendiumItem_Index>().Where(x => x.Type == entityType).ToList();
    }

    public void Dispose()
    {
        _store?.Dispose();
        GC.SuppressFinalize(this);
    }
}
