using Microsoft.Extensions.Logging;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.CLI.Commands;

public class TestCompendiumCommand
{
    private readonly ILogger<TestCompendiumCommand> _logger;
    private readonly CompendiumCacheBuilder _compendiumBuilder;

    public TestCompendiumCommand(ILogger<TestCompendiumCommand> logger, CompendiumCacheBuilder compendiumBuilder)
    {
        _logger = logger;
        _compendiumBuilder = compendiumBuilder;
    }

    public async Task<int> ExecuteAsync(string? specificModule, string? foundryBaseDirOverride, bool verbose)
    {
        _logger.LogInformation("🧪 Testing Compendium Loading...");

        var foundryDataPath = !string.IsNullOrEmpty(foundryBaseDirOverride)
            ? foundryBaseDirOverride
            : GetFoundryDataPath();

        _logger.LogInformation("📁 Using Foundry data path: {Path}", foundryDataPath);
        if (!Directory.Exists(foundryDataPath))
        {
            _logger.LogError("❌ Foundry data directory not found: {Path}", foundryDataPath);
            return 1;
        }

        var compendiumPaths = DiscoverCompendiumPaths(foundryDataPath, specificModule);

        await _compendiumBuilder.LoadUnifiedCacheAsync(compendiumPaths);
        var unifiedCache = _compendiumBuilder.GetAllEntities();

        Console.WriteLine($"✅ Successfully loaded {unifiedCache.Count} unique entities!\n");

        var systemStats = unifiedCache.Values
            .GroupBy(x => x.System)
            .OrderByDescending(g => g.Count());

        Console.WriteLine("🎲 Content by game system:");
        foreach (var systemGroup in systemStats)
        {
            Console.WriteLine($"   {systemGroup.Key}: {systemGroup.Count()} items");
            var topTypes = systemGroup.GroupBy(x => x.Type).OrderByDescending(g => g.Count());
            foreach (var typeGroup in topTypes)
            {
                Console.WriteLine($"      {typeGroup.Key}: {typeGroup.Count()}");
            }
        }
        Console.WriteLine();

        return 0;
    }

    private IEnumerable<string> DiscoverCompendiumPaths(string foundryDataPath, string? specificModule = null)
    {
        var basePaths = new[]
        {
            Path.Combine(foundryDataPath, "systems", "dnd5e"),
            Path.Combine(foundryDataPath, "systems", "pf2e"),
            Path.Combine(foundryDataPath, "systems", "pf1"),
            Path.Combine(foundryDataPath, "modules", "dnd-dungeon-masters-guide"),
            Path.Combine(foundryDataPath, "modules", "dnd-players-handbook"),
        };

        if (!string.IsNullOrEmpty(specificModule))
        {
            var moduleDir = Path.Combine(foundryDataPath, "modules", specificModule);
            if (Directory.Exists(moduleDir)) return new[] { moduleDir };
            var systemDir = Path.Combine(foundryDataPath, "systems", specificModule);
            if (Directory.Exists(systemDir)) return new[] { systemDir };
            _logger.LogError("❌ Module/system not found: {Module}", specificModule);
            return Enumerable.Empty<string>();
        }

        var modulesDir = Path.Combine(foundryDataPath, "modules");
        var otherModules = new List<string>();
        if (Directory.Exists(modulesDir))
        {
            foreach (var moduleDir in Directory.GetDirectories(modulesDir))
            {
                var moduleName = Path.GetFileName(moduleDir);
                if (moduleName.StartsWith("battlezoo", StringComparison.OrdinalIgnoreCase) || moduleName.Equals("plutonium", StringComparison.OrdinalIgnoreCase))
                {
                    otherModules.Add(moduleDir);
                }
            }
        }

        return basePaths.Concat(otherModules).Where(Directory.Exists);
    }

    private string GetFoundryDataPath()
    {
        var envPath = Environment.GetEnvironmentVariable("FOUNDRY_DATA_PATH");
        if (!string.IsNullOrEmpty(envPath) && Directory.Exists(envPath)) return envPath;
        var configPath = TryGetFoundryConfigPath();
        if (!string.IsNullOrEmpty(configPath)) return configPath;
        return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "FoundryVTT", "Data");
    }

    private string? TryGetFoundryConfigPath()
    {
        try
        {
            var configFile = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), ".fvttrc.yml");
            if (!File.Exists(configFile)) return null;
            var content = File.ReadAllText(configFile);
            var dataPath = System.Text.RegularExpressions.Regex.Match(content, @"dataPath:\s*\""?([^\""\n]+)\""?").Groups[1].Value.Trim();
            if (!string.IsNullOrEmpty(dataPath)) return Path.Combine(dataPath, "Data");
        }
        catch { }
        return null;
    }
}
