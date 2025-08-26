using System.CommandLine;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using TTRPGConverter.CLI.Commands;
using TTRPGConverter.Core.Mappers;
using TTRPGConverter.Infrastructure.Services.Assets;
using TTRPGConverter.Infrastructure.Services.Compendium;
using TTRPGConverter.Infrastructure.Services.Foundry;
using TTRPGConverter.Infrastructure.Services.Roll20;

public class Program
{
    public static async Task<int> Main(string[] args)
    {
        var serviceProvider = ConfigureServices();
        var rootCommand = BuildRootCommand(serviceProvider);

        // This is the correct pattern based on your example: Parse, then Invoke.
        var parseResult = rootCommand.Parse(args);
        return await parseResult.InvokeAsync();
    }

    private static IServiceProvider ConfigureServices()
    {
        var services = new ServiceCollection();

        services.AddLogging(configure =>
        {
            configure.AddConsole();
            configure.SetMinimumLevel(LogLevel.Information);
        });

        services.AddAutoMapper(typeof(Roll20ToFoundryMapper).Assembly);

        // Infrastructure Services
        services.AddSingleton<Roll20CampaignService>();
        services.AddSingleton<FoundryWorldBuilder>();
        services.AddSingleton<FoundryDatabaseFactory>();
        services.AddSingleton<ParallelAssetProcessor>();
        services.AddSingleton<IAssetMapper, AssetMapper>();
        services.AddSingleton<ICompendiumManager>(provider =>
            new RavenDbCompendiumManager(
                provider.GetRequiredService<ILogger<RavenDbCompendiumManager>>(),
                "compendium.ravendb"
            ));
        services.AddSingleton<CompendiumCacheBuilder>();

        // Core Services
        services.AddSingleton<Roll20ToFoundryMapper>();

        // CLI Commands (as services that contain the logic)
        services.AddTransient<ConvertCampaignCommand>();
        services.AddTransient<UpdateCompendiumCommand>();
        services.AddTransient<TestCampaignCommand>();
        services.AddTransient<TestCompendiumCommand>();

        return services.BuildServiceProvider();
    }

    private static RootCommand BuildRootCommand(IServiceProvider serviceProvider)
    {
        var rootCommand = new RootCommand("TTRPG Converter CLI");

        // == Convert Campaign Command ==
        var sourceArg = new Argument<string>("sourceZip") { Description = "Path to the Roll20 campaign ZIP file" };
        var outputArg = new Argument<string>("outputDir") { Description = "Directory to create the Foundry world in" };
        var worldNameOpt = new Option<string>("--worldName") { Description = "Name for the new Foundry world" };
        var systemOpt = new Option<string>("--system")
            { Description = "The target Foundry VTT system (e.g., dnd5e, pf2e)" };

        var convertCampaignCommand =
            new Command("convert-campaign", "Convert a Roll20 campaign ZIP to a Foundry VTT world");
        convertCampaignCommand.Arguments.Add(sourceArg);
        convertCampaignCommand.Arguments.Add(outputArg);
        convertCampaignCommand.Options.Add(worldNameOpt);
        convertCampaignCommand.Options.Add(systemOpt);

        convertCampaignCommand.SetAction(async parseResult =>
        {
            var handler = serviceProvider.GetRequiredService<ConvertCampaignCommand>();
            var sourceZip = parseResult.GetValue(sourceArg);
            var outputDir = parseResult.GetValue(outputArg);
            var worldName = parseResult.GetValue(worldNameOpt) ?? "My Converted World";
            var system = parseResult.GetValue(systemOpt) ?? "dnd5e";
            await handler.HandleCommandAsync(sourceZip!, outputDir!, worldName, system);
        });
        rootCommand.Subcommands.Add(convertCampaignCommand);

        // == Update Compendium Command ==
        var outputOpt = new Option<string>("--output") { Description = "Path to save the compendium database file" };
        var foundryPathOpt =
            new Option<string>("--foundry-data-path") { Description = "Override Foundry data directory path"};

    var updateCompendiumCommand =
            new Command("update-compendium", "Create or update the compendium database cache");
        updateCompendiumCommand.Options.Add(outputOpt);
        updateCompendiumCommand.Options.Add(foundryPathOpt);

        updateCompendiumCommand.SetAction(async parseResult =>
        {
            var handler = serviceProvider.GetRequiredService<UpdateCompendiumCommand>();
            var output = parseResult.GetValue(outputOpt);
            var foundryDataPath = parseResult.GetValue(foundryPathOpt);
            await handler.ExecuteAsync(output, foundryDataPath);
        });
        rootCommand.Subcommands.Add(updateCompendiumCommand);

        // == Test Campaign Command ==
        var zipPathArg = new Argument<string>("sourceZip") { Description = "Path to the Roll20 campaign ZIP file" };

        var testCampaignCommand = new Command("test-campaign", "Test Roll20 campaign loading and parsing");
        testCampaignCommand.Arguments.Add(zipPathArg);

        testCampaignCommand.SetAction(async parseResult =>
        {
            var handler = serviceProvider.GetRequiredService<TestCampaignCommand>();
            var zipPath = parseResult.GetValue(zipPathArg);
            await handler.ExecuteAsync(zipPath!);
        });
        rootCommand.Subcommands.Add(testCampaignCommand);
        
        // == Test Compendium Command ==
        var moduleOpt = new Option<string>("--module") { Description = "Test only the specified module or system"};
        var foundryPathOptCompendium = new Option<string>("--foundry-data-path") { Description = "Override Foundry data directory path"};
        var verboseOpt = new Option<bool>( "--verbose", "-v" ) { Description = "Enable verbose logging"};

        var testCompendiumCommand = new Command("test-compendium", "Test Foundry VTT compendium reading");
        testCompendiumCommand.Options.Add(moduleOpt);
        testCompendiumCommand.Options.Add(foundryPathOptCompendium);
        testCompendiumCommand.Options.Add(verboseOpt);

        testCompendiumCommand.SetAction(async parseResult =>
        {
            var handler = serviceProvider.GetRequiredService<TestCompendiumCommand>();
            await handler.ExecuteAsync(
                parseResult.GetValue(moduleOpt),
                parseResult.GetValue(foundryPathOptCompendium),
                parseResult.GetValue(verboseOpt)
            );
        });
        rootCommand.Subcommands.Add(testCompendiumCommand);


        return rootCommand;
    }
}