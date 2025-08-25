using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using System.Diagnostics;
using TTRPGConverter.CLI.Commands;
using TTRPGConverter.Core.Interfaces;
using TTRPGConverter.Core.Services;

internal class Program
{
    private static async Task<int> Main(string[] args)
    {
        Console.WriteLine("TTRPGConverter CLI");

        if (args.Length == 0)
        {
            ShowUsage();
            return 1;
        }

        var command = args[0].ToLowerInvariant();

        // Handle commands
        return await (command switch
        {
            "generate" => RunGenerateCommand(args[1..]),
            "test-campaign" => RunTestCampaignCommand(args[1..]),
            "test-compendium" => RunTestCompendiumCommand(args[1..]),
            "update-compendium" => RunUpdateCompendiumCommand(args[1..]),
            "convert-campaign" => RunConvertCampaignCommand(args[1..]),
            _ => Task.FromResult(ShowUsage())
        });

        static int ShowUsage()
        {
            Console.WriteLine();
            Console.WriteLine("Usage: TTRPGConverter <command> [options]");
            Console.WriteLine();
            Console.WriteLine("Commands:");
            Console.WriteLine("  generate           Generate C# models from JSON schemas");
            Console.WriteLine("  test-campaign      Test Roll20 campaign ZIP loading and parsing");
            Console.WriteLine("  test-compendium    Test Foundry VTT compendium reading");
            Console.WriteLine("  update-compendium  Create or update the compendium database cache");
            Console.WriteLine("  convert-campaign   Convert Roll20 campaign ZIP to Foundry VTT world");
            Console.WriteLine("  help               Show this help message");
            Console.WriteLine();
            Console.WriteLine("Update-Compendium Options:");
            Console.WriteLine("  --output <path>         Path to save the compendium database file (default: compendium.ravendb)");
            Console.WriteLine("  --foundry-base-dir    Override Foundry data directory path");
            Console.WriteLine();
            Console.WriteLine("Test-Compendium Options:");
            Console.WriteLine("  --dry-run, -n         Show commands that would be executed without running them");
            Console.WriteLine("  --verbose, -v         Enable verbose logging (shows debug output)");
            Console.WriteLine("  --module <name>       Test only the specified module or system");
            Console.WriteLine("  --foundry-base-dir    Override Foundry data directory path");
            Console.WriteLine();
            Console.WriteLine("Generate Options:");
            Console.WriteLine("  --schema-path <path>    Path to schema directory (default: schemas)");
            Console.WriteLine("  --output-path <path>    Output directory (default: src/TTRPGConverter.Core/Models)");
            Console.WriteLine("  --namespace <ns>        Base namespace (default: TTRPGConverter.Core.Models)");
            Console.WriteLine();
            return 0;
        }

        static async Task<int> RunUpdateCompendiumCommand(string[] args)
        {
            string? outputPath = null;
            string? foundryBaseDirOverride = null;

            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--output" && i + 1 < args.Length)
                {
                    outputPath = args[i + 1];
                }
                else if (args[i] == "--foundry-base-dir" && i + 1 < args.Length)
                {
                    foundryBaseDirOverride = args[i + 1];
                }
            }

            var builder = Host.CreateApplicationBuilder();
            builder.Services.AddLogging(configure =>
                configure.AddConsole().SetMinimumLevel(LogLevel.Information));

            builder.Services.AddSingleton(provider =>
            {
                var logger = provider.GetRequiredService<ILogger<CompendiumManager>>();
                return new CompendiumManager(logger, true, "collisions.csv");
            });
            builder.Services.AddSingleton<UpdateCompendiumCommand>();

            var host = builder.Build();
            var updateCommand = host.Services.GetRequiredService<UpdateCompendiumCommand>();

            var foundryDataPath = !string.IsNullOrEmpty(foundryBaseDirOverride)
                ? foundryBaseDirOverride
                : GetFoundryDataPath();

            return await updateCommand.ExecuteAsync(outputPath, foundryDataPath);
        }

        static async Task<int> RunConvertCampaignCommand(string[] args)
        {
            if (args.Length < 2)
            {
                Console.WriteLine("Error: Campaign ZIP and output directory required");
                Console.WriteLine("Usage: TTRPGConverter convert-campaign <zip-path> <output-dir> [--system dnd5e] [--name WorldName]");
                return 1;
            }

            var zipPath = args[0];
            var outputDir = args[1];
            var system = "dnd5e";
            string? worldName = null;
            var verbose = args.Contains("--verbose") || args.Contains("-v");
            var cachePath = "compendium.ravendb";

            for (int i = 2; i < args.Length; i++)
            {
                if (args[i] == "--system" && i + 1 < args.Length)
                    system = args[i + 1];
                else if (args[i] == "--name" && i + 1 < args.Length)
                    worldName = args[i + 1];
                else if (args[i] == "--cache-path" && i + 1 < args.Length)
                    cachePath = args[i + 1];
            }

            if (!File.Exists(zipPath))
            {
                Console.WriteLine($"Error: File not found: {zipPath}");
                return 1;
            }

            var builder = Host.CreateApplicationBuilder();
            var logLevel = verbose ? LogLevel.Debug : LogLevel.Information;
            builder.Services.AddLogging(configure => configure.AddConsole().SetMinimumLevel(logLevel));

            builder.Services.AddAutoMapper(typeof(Program));
            builder.Services.AddSingleton<Roll20CampaignService>();
            builder.Services.AddSingleton<FoundryWorldBuilder>();
            builder.Services.AddSingleton<ConvertCampaignCommand>();
            builder.Services.AddSingleton<Roll20ToFoundryMapper>();

            builder.Services.AddSingleton<ICompendiumManager, StubCompendiumManager>();

            var host = builder.Build();
            var convertCommand = host.Services.GetRequiredService<ConvertCampaignCommand>();

            await convertCommand.ExecuteAsync(zipPath, outputDir, system, worldName);
            return 0;
        }

        static async Task<int> RunTestCompendiumCommand(string[] args)
        {
            var dryRun = args.Contains("--dry-run") || args.Contains("-n");
            var verbose = args.Contains("--verbose") || args.Contains("-v");
            string? specificModule = null;
            string? foundryBaseDirOverride = null;

            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--module" && i + 1 < args.Length) specificModule = args[i + 1];
                else if (args[i] == "--foundry-base-dir" && i + 1 < args.Length) foundryBaseDirOverride = args[i + 1];
            }

            var logLevel = verbose ? LogLevel.Debug : LogLevel.Information;
            using var loggerFactory = LoggerFactory.Create(builder => builder.AddConsole().SetMinimumLevel(logLevel));
            var logger = loggerFactory.CreateLogger<CompendiumManager>();

            var compendiumManager = new CompendiumManager(logger, verbose, "collisions.csv");
            var foundryDataPath = !string.IsNullOrEmpty(foundryBaseDirOverride) ? foundryBaseDirOverride : GetFoundryDataPath();

            Console.WriteLine($"📁 Using Foundry data path: {foundryDataPath}");
            if (!Directory.Exists(foundryDataPath))
            {
                Console.WriteLine($"❌ Foundry data directory not found: {foundryDataPath}");
                return 1;
            }

            await EnsureFoundryCliConfigured();
            var compendiumPaths = DiscoverCompendiumPaths(foundryDataPath, specificModule);

            await compendiumManager.LoadUnifiedCacheAsync(compendiumPaths);
            var unifiedCache = compendiumManager.GetAllEntities();

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

        static IEnumerable<string> DiscoverCompendiumPaths(string foundryDataPath, string? specificModule = null)
        {
            var basePaths = new[]
            {
        Path.Combine(foundryDataPath, "systems", "dnd5e"),
        Path.Combine(foundryDataPath, "systems", "pf2e"),
        Path.Combine(foundryDataPath, "systems", "pf1"),
        Path.Combine(foundryDataPath, "modules", "dnd-dungeon-masters-guide"),
        Path.Combine(foundryDataPath, "modules", "dnd-players-handbook"),
        Path.Combine(foundryDataPath, "modules", "pf-content"),
        Path.Combine(foundryDataPath, "modules", "pf1-statblock-converter"),
        Path.Combine(foundryDataPath, "modules", "statblock-library"),
    };

            if (!string.IsNullOrEmpty(specificModule))
            {
                var moduleDir = Path.Combine(foundryDataPath, "modules", specificModule);
                if (Directory.Exists(moduleDir)) return new[] { moduleDir };
                var systemDir = Path.Combine(foundryDataPath, "systems", specificModule);
                if (Directory.Exists(systemDir)) return new[] { systemDir };
                Console.WriteLine($"❌ Module/system not found: {specificModule}");
                return Enumerable.Empty<string>();
            }

            var modulesDir = Path.Combine(foundryDataPath, "modules");
            var battlezooModules = new List<string>();
            var otherModules = new List<string>();

            if (Directory.Exists(modulesDir))
            {
                foreach (var moduleDir in Directory.GetDirectories(modulesDir))
                {
                    var moduleName = Path.GetFileName(moduleDir);
                    if (moduleName.StartsWith("battlezoo", StringComparison.OrdinalIgnoreCase)) battlezooModules.Add(moduleDir);
                    else if (moduleName.Equals("plutonium", StringComparison.OrdinalIgnoreCase)) otherModules.Add(moduleDir);
                }
            }

            return basePaths.Concat(battlezooModules.OrderBy(p => Path.GetFileName(p))).Concat(otherModules).Where(Directory.Exists);
        }

        static string GetFoundryDataPath()
        {
            var envPath = Environment.GetEnvironmentVariable("FOUNDRY_DATA_PATH");
            if (!string.IsNullOrEmpty(envPath) && Directory.Exists(envPath)) return envPath;
            var configPath = TryGetFoundryConfigPath();
            if (!string.IsNullOrEmpty(configPath)) return configPath;
            return Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "FoundryVTT", "Data");
        }

        static string? TryGetFoundryConfigPath()
        {
            try
            {
                var configFile = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), ".fvttrc.yml");
                if (!File.Exists(configFile)) return null;
                var content = File.ReadAllText(configFile);
                var dataPath = System.Text.RegularExpressions.Regex.Match(content, @"dataPath:\s*""?([^""\n]+)""?").Groups[1].Value.Trim();
                if (!string.IsNullOrEmpty(dataPath)) return Path.Combine(dataPath, "Data");
            }
            catch { }
            return null;
        }

        static async Task EnsureFoundryCliConfigured()
        {
            try
            {
                var fvttPath = ".\\node_modules\\.bin\\fvtt.cmd";
                if (!File.Exists(fvttPath)) return;
                var processInfo = new ProcessStartInfo
                {
                    FileName = fvttPath,
                    Arguments = "configure set installPath \"C:/Program Files/Foundry Virtual Tabletop\"",
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    CreateNoWindow = true
                };
                using var process = Process.Start(processInfo);
                if (process != null) await process.WaitForExitAsync();
            }
            catch { }
        }

        static Task<int> RunGenerateCommand(string[] args) => Task.FromResult(0);
        static Task<int> RunTestCampaignCommand(string[] args) => Task.FromResult(0);
    }
}