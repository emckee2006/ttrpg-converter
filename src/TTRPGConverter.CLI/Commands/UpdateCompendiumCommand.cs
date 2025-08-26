using Microsoft.Extensions.Logging;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Raven.Client.Documents;
using Raven.Embedded;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.CLI.Commands;

public class UpdateCompendiumCommand
{
    private readonly ILogger<UpdateCompendiumCommand> _logger;
    private readonly CompendiumCacheBuilder _compendiumBuilder;

    public UpdateCompendiumCommand(ILogger<UpdateCompendiumCommand> logger, CompendiumCacheBuilder compendiumBuilder)
    {
        _logger = logger;
        _compendiumBuilder = compendiumBuilder;
    }

    public async Task<int> ExecuteAsync(string? outputPath, string? foundryDataPath)
    {
        _logger.LogInformation("Starting compendium update process...");

        if (string.IsNullOrEmpty(foundryDataPath) || !Directory.Exists(foundryDataPath))
        {
            _logger.LogError("❌ Foundry data path is invalid or not provided: {Path}", foundryDataPath);
            return 1;
        }

        var compendiumPaths = DiscoverCompendiumPaths(foundryDataPath);
        _logger.LogInformation("Discovered {Count} potential compendium sources.", compendiumPaths.Count());

        await _compendiumBuilder.LoadUnifiedCacheAsync(compendiumPaths);
        var allItems = _compendiumBuilder.GetAllEntities().Values;

        if (!allItems.Any())
        {
            _logger.LogWarning("No compendium items were found. The cache will be empty.");
            return 1;
        }

        var dbPath = outputPath ?? "compendium.ravendb";
        _logger.LogInformation("Writing {Count} items to RavenDB cache at {Path}...", allItems.Count, dbPath);

        try
        {
            var serverOptions = new ServerOptions
            {
                DataDirectory = dbPath,
                ServerUrl = "http://127.0.0.1:8080"
            };
            EmbeddedServer.Instance.StartServer(serverOptions);

            using (var store = EmbeddedServer.Instance.GetDocumentStore(new DatabaseOptions("Compendium")))
            {
                new CompendiumItem_Index().Execute(store);

                using (var bulkInsert = store.BulkInsert())
                {
                    foreach (var item in allItems)
                    {
                        var docId = $"{item.Type.ToLowerInvariant()}/{item.Name.Replace(' ', '-')}";
                        bulkInsert.Store(item, docId);
                    }
                }
            }

            _logger.LogInformation("✅ Compendium cache successfully created at {Path}", dbPath);
            return 0;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Failed to create compendium cache.");
            return 1;
        }
    }

    private IEnumerable<string> DiscoverCompendiumPaths(string foundryDataPath)
    {
        var basePaths = new[]
        {
            Path.Combine(foundryDataPath, "modules", "dnd-dungeon-masters-guide"),
            Path.Combine(foundryDataPath, "modules", "dnd-players-handbook"),
            Path.Combine(foundryDataPath, "systems", "dnd5e"),
            Path.Combine(foundryDataPath, "systems", "pf2e"),
            Path.Combine(foundryDataPath, "systems", "pf1"),
            Path.Combine(foundryDataPath, "modules", "pf-content"),
            Path.Combine(foundryDataPath, "modules", "pf1-statblock-converter"),
            Path.Combine(foundryDataPath, "modules", "statblock-library"),
        };

        var modulesDir = Path.Combine(foundryDataPath, "modules");
        var battlezooModules = new List<string>();
        var otherModules = new List<string>();

        if (Directory.Exists(modulesDir))
        {
            foreach (var moduleDir in Directory.GetDirectories(modulesDir))
            {
                var moduleName = Path.GetFileName(moduleDir);
                if (moduleName.StartsWith("battlezoo", StringComparison.OrdinalIgnoreCase))
                {
                    battlezooModules.Add(moduleDir);
                }
                else if (moduleName.Equals("plutonium", StringComparison.OrdinalIgnoreCase))
                {
                    otherModules.Add(moduleDir);
                }
            }
        }

        return basePaths
            .Concat(battlezooModules.OrderBy(p => Path.GetFileName(p)))
            .Concat(otherModules)
            .Where(Directory.Exists);
    }
}
