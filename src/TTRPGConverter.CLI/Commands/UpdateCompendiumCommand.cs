using System;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using Spectre.Console;
using TTRPGConverter.Core;
using TTRPGConverter.Core.Compendium;
using TTRPGConverter.Infrastructure;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.CLI.Commands;

public class UpdateCompendiumCommand
{
    private readonly ILogger<UpdateCompendiumCommand> _logger;
    private readonly CompendiumCacheBuilder _compendiumBuilder;
    private readonly FoundryModuleService _moduleService;

    public UpdateCompendiumCommand(
        ILogger<UpdateCompendiumCommand> logger, 
        CompendiumCacheBuilder compendiumBuilder, 
        FoundryModuleService moduleService)
    {
        _logger = logger;
        _compendiumBuilder = compendiumBuilder;
        _moduleService = moduleService;
    }

    public async Task<int> ExecuteAsync(string? outputPath, string? foundryDataPath)
    {
        var compendiumPaths = _moduleService.DiscoverCompendiumPaths();
        var dbPath = outputPath ?? "compendium.db";

        try
        {
            await AnsiConsole.Progress()
                .Columns(new ProgressColumn[]
                {
                    new TaskDescriptionColumn(),
                    new ProgressBarColumn(),
                    new PercentageColumn(),
                    new RemainingTimeColumn(),
                    new SpinnerColumn(),
                })
                .StartAsync(async ctx =>
                {
                    var overallTask = ctx.AddTask("[green]Building Compendium...[/]");

                    var progress = new Progress<BuildProgressReport>(report =>
                    {
                        if (report.ProgressUpdate != null)
                        {
                            var update = report.ProgressUpdate;
                            if (update.OverallMaxValue.HasValue) overallTask.MaxValue = update.OverallMaxValue.Value;
                            if (update.OverallIncrement.HasValue) overallTask.Increment(update.OverallIncrement.Value);
                            if (!string.IsNullOrEmpty(update.DetailDescription)) overallTask.Description = $"[green]{update.DetailDescription}[/]";
                        }

                        if (report.PackResult != null)
                        {
                            var result = report.PackResult;
                            if (result.IsSuccess)
                            {
                                AnsiConsole.MarkupLine($"  [green]✓[/] {result.ModuleName} -> {result.PackName} ({result.ItemsProcessed} items)");
                            }
                            else
                            {
                                AnsiConsole.MarkupLine($"  [red]✗[/] {result.ModuleName} -> {result.PackName}: {result.ErrorMessage}");
                            }
                        }
                    });

                    await _compendiumBuilder.BuildCacheAsync(compendiumPaths, dbPath, progress);
                });

            await ReportOnCacheContents(dbPath);
            return 0;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "❌ Failed to create compendium cache.");
            AnsiConsole.WriteException(ex);
            return 1;
        }
    }

    private async Task ReportOnCacheContents(string dbPath)
    {
        var options = new DbContextOptionsBuilder<TTRPGConverterContext>()
            .UseSqlite($"Data Source={dbPath}")
            .Options;

        await using var context = new TTRPGConverterContext(options);

        var primaryCount = await context.CompendiumItems.CountAsync(i => i.IsPrimary);
        AnsiConsole.MarkupLine($"\n[green]✅ Successfully loaded {primaryCount} unique entities![/]\n");

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

        AnsiConsole.MarkupLine("[yellow]🎲 Content by game system:[/]");
        foreach (var systemGroup in systemGroups)
        {
            AnsiConsole.MarkupLine($"   [bold]{systemGroup.Key}[/]: {systemGroup.Sum(s => s.Count)} items");
            foreach (var typeGroup in systemGroup.OrderByDescending(s => s.Count))
            {
                AnsiConsole.MarkupLine($"      [dim]{typeGroup.Type}[/]: {typeGroup.Count}");
            }
        }
        AnsiConsole.WriteLine();
    }
}
