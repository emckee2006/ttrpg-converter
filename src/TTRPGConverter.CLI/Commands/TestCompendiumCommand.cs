using System;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Tasks;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Compendium;
using TTRPGConverter.Infrastructure;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.CLI.Commands;

public partial class TestCompendiumCommand
{
    private readonly ILogger<TestCompendiumCommand> _logger;
    private readonly CompendiumCacheBuilder _compendiumBuilder;
    private readonly FoundryModuleService _moduleService;

    public TestCompendiumCommand(
        ILogger<TestCompendiumCommand> logger, 
        CompendiumCacheBuilder compendiumBuilder,
        FoundryModuleService moduleService)
    {
        _logger = logger;
        _compendiumBuilder = compendiumBuilder;
        _moduleService = moduleService;
    }

    public async Task<int> ExecuteAsync(string? specificModule, string? foundryBaseDirOverride, bool verbose)
    {
        _logger.LogInformation("🧪 Testing Compendium Loading...");

        var tempDbPath = Path.Combine(Path.GetTempPath(), "TTRPGConverter_TestCache_" + Path.GetRandomFileName());

        try
        {
            // Use the new centralized service to discover paths
            var compendiumPaths = _moduleService.DiscoverCompendiumPaths(specificModule);
            if (!compendiumPaths.Any())
            {
                _logger.LogWarning("No compendium paths found to test.");
                return 1;
            }

            await _compendiumBuilder.BuildCacheAsync(compendiumPaths, tempDbPath);

            await ReportOnCacheContents(tempDbPath);

            return 0;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Compendium test failed.");
            return 1;
        }
        finally
        {
            var connection = new SqliteConnection($"Data Source={tempDbPath}");
            SqliteConnection.ClearPool(connection);

            if (File.Exists(tempDbPath)) { File.Delete(tempDbPath); }
        }
    }

    private async Task ReportOnCacheContents(string dbPath)
    {
        var options = new DbContextOptionsBuilder<TTRPGConverterContext>()
            .UseSqlite($"Data Source={dbPath}")
            .Options;

        await using var context = new TTRPGConverterContext(options);

        var primaryCount = await context.CompendiumItems.CountAsync(i => i.IsPrimary);

        Console.WriteLine($"\n✅ Successfully loaded {primaryCount} unique entities!\n");

        var stats = await context.CompendiumItems
            .Where(i => i.IsPrimary)
            .GroupBy(x => new { System = x.System ?? "No System", x.Type })
            .Select(g => new {
                System = g.Key.System,
                Type = g.Key.Type,
                Count = g.Count()
            })
            .ToListAsync();

        var systemGroups = stats.GroupBy(s => s.System).OrderByDescending(g => g.Sum(s => s.Count));

        Console.WriteLine("🎲 Content by game system:");
        foreach (var systemGroup in systemGroups)
        {
            Console.WriteLine($"   {systemGroup.Key}: {systemGroup.Sum(s => s.Count)} items");
            foreach (var typeGroup in systemGroup.OrderByDescending(s => s.Count))
            {
                Console.WriteLine($"      {typeGroup.Type}: {typeGroup.Count}");
            }
        }
        Console.WriteLine();
    }
}
