using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using Microsoft.Extensions.Logging;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public class FoundryModuleService
{
    private readonly ILogger<FoundryModuleService> _logger;

    private static readonly HashSet<string> SystemWhitelist = new() { "dnd5e", "pf2e", "pf1" };
    private static readonly HashSet<string> ModuleWhitelist = new() 
    {
        "dnd-dungeon-masters-guide",
        "dnd-players-handbook",
        "pf1-statblock-converter",
        "statblock-library",
        "pf-content"
    };

    public FoundryModuleService(ILogger<FoundryModuleService> logger)
    {
        _logger = logger;
    }

    public IEnumerable<string> GetDiscoverableModuleNames()
    {
        var foundryDataPath = GetFoundryDataPath();
        if (!Directory.Exists(foundryDataPath)) yield break;

        var systemsDir = Path.Combine(foundryDataPath, "systems");
        if (Directory.Exists(systemsDir))
        {
            foreach (var dir in Directory.GetDirectories(systemsDir))
            {
                yield return Path.GetFileName(dir);
            }
        }

        var modulesDir = Path.Combine(foundryDataPath, "modules");
        if (Directory.Exists(modulesDir))
        {
            foreach (var dir in Directory.GetDirectories(modulesDir))
            {
                yield return Path.GetFileName(dir);
            }
        }
    }

    public IEnumerable<string> DiscoverCompendiumPaths(string? specificModule = null)
    {
        var foundryDataPath = GetFoundryDataPath();
        if (!Directory.Exists(foundryDataPath))
        {
            _logger.LogError("❌ Foundry data path is invalid or not found: {Path}", foundryDataPath);
            return Enumerable.Empty<string>();
        }

        if (!string.IsNullOrEmpty(specificModule))
        {
            var moduleDir = Path.Combine(foundryDataPath, "modules", specificModule);
            if (Directory.Exists(moduleDir)) return new[] { moduleDir };

            var systemDir = Path.Combine(foundryDataPath, "systems", specificModule);
            if (Directory.Exists(systemDir)) return new[] { systemDir };

            _logger.LogError("❌ Specified module or system not found: {Module}", specificModule);
            return Enumerable.Empty<string>();
        }

        var discoveredPaths = new List<string>();

        var systemsDir = Path.Combine(foundryDataPath, "systems");
        if (Directory.Exists(systemsDir))
        {
            discoveredPaths.AddRange(Directory.GetDirectories(systemsDir).Where(dir => SystemWhitelist.Contains(Path.GetFileName(dir))));
        }

        var modulesDir = Path.Combine(foundryDataPath, "modules");
        if (Directory.Exists(modulesDir))
        {
            discoveredPaths.AddRange(Directory.GetDirectories(modulesDir).Where(dir => 
                ModuleWhitelist.Contains(Path.GetFileName(dir)) || 
                Path.GetFileName(dir).Contains("battlezoo", StringComparison.OrdinalIgnoreCase)));
        }

        _logger.LogInformation("Discovered {Count} compendium paths to process.", discoveredPaths.Count);
        return discoveredPaths;
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
        catch (Exception ex)
        {
            _logger.LogWarning(ex, "Failed to read FoundryVTT config file.");
        }
        return null;
    }
}
