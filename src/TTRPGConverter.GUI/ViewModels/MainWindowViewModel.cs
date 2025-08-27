using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using Avalonia.Threading;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core;
using TTRPGConverter.Infrastructure;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.GUI.ViewModels;

public partial class ModuleSummaryViewModel : ViewModelBase
{
    [ObservableProperty]
    private string _moduleName;

    [ObservableProperty]
    private int _successCount;

    [ObservableProperty]
    private int _failCount;

    public ObservableCollection<PackProcessingResult> ProcessedPacks { get; } = new();

    public ModuleSummaryViewModel(string moduleName)
    {
        _moduleName = moduleName;
    }

    public void AddResult(PackProcessingResult result)
    {
        ProcessedPacks.Add(result);
        if (result.IsSuccess)
        {
            SuccessCount++;
        }
        else
        {
            FailCount++;
        }
    }
}

public partial class MainWindowViewModel : ViewModelBase
{
    private readonly CompendiumCacheBuilder _compendiumBuilder;
    private readonly FoundryModuleService _moduleService;
    private readonly IDbContextFactory<TTRPGConverterContext> _contextFactory;
    private readonly ILogger<MainWindowViewModel> _logger;

    [ObservableProperty]
    private string _logText = "Welcome to the TTRPG Converter!\n";

    [ObservableProperty]
    [NotifyPropertyChangedFor(nameof(IsProgressBarIndeterminate))]
    private double _progressValue;

    [ObservableProperty]
    private string _progressText = "Ready";

    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(TestCompendiumCommand))]
    private bool _isBusy;

    [ObservableProperty]
    [NotifyCanExecuteChangedFor(nameof(TestCompendiumCommand))]
    private string? _selectedModule;

    public IReadOnlyList<string> AvailableModules { get; }
    public ObservableCollection<ModuleSummaryViewModel> BuildResults { get; } = new();

    public bool IsProgressBarIndeterminate => ProgressValue == 0 && IsBusy;

    public MainWindowViewModel(
        CompendiumCacheBuilder compendiumBuilder, 
        FoundryModuleService moduleService,
        IDbContextFactory<TTRPGConverterContext> contextFactory,
        ILogger<MainWindowViewModel> logger)
    {
        _compendiumBuilder = compendiumBuilder;
        _moduleService = moduleService;
        _contextFactory = contextFactory;
        _logger = logger;
        AvailableModules = _moduleService.GetDiscoverableModuleNames().ToList();
        _selectedModule = AvailableModules.FirstOrDefault();
    }

    public async Task StartUpdateCompendiumAsync(string dbPath)
    {
        await RunCompendiumOperation(null, dbPath);
    }

    [RelayCommand(CanExecute = nameof(CanExecuteTestOperation))]
    private async Task TestCompendiumAsync()
    {
        var dbPath = Path.Combine(Path.GetTempPath(), $"test_{SelectedModule}.db");
        await RunCompendiumOperation(SelectedModule, dbPath);
    }

    private bool CanExecuteTestOperation() => !IsBusy && !string.IsNullOrEmpty(SelectedModule);

    private async Task RunCompendiumOperation(string? moduleName, string dbPath)
    {
        IsBusy = true;
        BuildResults.Clear();
        var logBuilder = new StringBuilder();
        LogText = string.Empty;
        ProgressValue = 0;
        ProgressText = "Starting...";

        double maxValue = 100.0;
        double currentValue = 0.0;

        var progress = new Progress<BuildProgressReport>(report =>
        {
            // Use the dispatcher to ensure UI updates happen on the main thread.
            Dispatcher.UIThread.Post(() =>
            {
                if (report.ProgressUpdate != null)
                {
                    var update = report.ProgressUpdate;
                    if (update.OverallMaxValue.HasValue) maxValue = update.OverallMaxValue.Value;
                    if (update.OverallIncrement.HasValue) currentValue += update.OverallIncrement.Value;
                    if (maxValue > 0) ProgressValue = (currentValue / maxValue) * 100;

                    var description = update.DetailDescription ?? update.OverallDescription;
                    if (!string.IsNullOrEmpty(description))
                    {
                        ProgressText = description;
                        logBuilder.AppendLine(description);
                        LogText = logBuilder.ToString();
                    }
                }

                if (report.PackResult != null)
                {
                    var result = report.PackResult;
                    var moduleSummary = BuildResults.FirstOrDefault(m => m.ModuleName == result.ModuleName);
                    if (moduleSummary == null)
                    {
                        moduleSummary = new ModuleSummaryViewModel(result.ModuleName);
                        BuildResults.Add(moduleSummary);
                    }
                    moduleSummary.AddResult(result);
                }
            });
        });

        try
        {
            await Task.Run(async () =>
            {
                var compendiumPaths = _moduleService.DiscoverCompendiumPaths(moduleName);
                await _compendiumBuilder.BuildCacheAsync(compendiumPaths, dbPath, progress);
            });

            await ReportOnCacheContents(dbPath, logBuilder);
            ProgressText = "Operation completed successfully!";
        }
        catch (Exception ex)
        {
            var errorMessage = $"An error occurred: {ex.Message}";
            ProgressText = errorMessage;
            logBuilder.AppendLine(errorMessage);
            LogText = logBuilder.ToString();
        }
        finally
        {
            IsBusy = false;
            ProgressValue = 100;
            if (moduleName != null && File.Exists(dbPath))
            {
                try { File.Delete(dbPath); } catch (Exception ex) { _logger.LogWarning(ex, "Failed to delete temporary test database."); }
            }
        }
    }

    private async Task ReportOnCacheContents(string dbPath, StringBuilder logBuilder)
    {
        var options = new DbContextOptionsBuilder<TTRPGConverterContext>()
            .UseSqlite($"Data Source={dbPath}")
            .Options;

        await using var context = new TTRPGConverterContext(options);

        var primaryCount = await context.CompendiumItems.CountAsync(i => i.IsPrimary);
        logBuilder.AppendLine($"\n✅ Successfully loaded {primaryCount} unique entities!\n");

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

        logBuilder.AppendLine("🎲 Content by game system:");
        foreach (var systemGroup in systemGroups)
        {
            logBuilder.AppendLine($"   {systemGroup.Key}: {systemGroup.Sum(s => s.Count)} items");
            foreach (var typeGroup in systemGroup.OrderByDescending(s => s.Count))
            {
                logBuilder.AppendLine($"      {typeGroup.Type}: {typeGroup.Count}");
            }
        }
        logBuilder.AppendLine();
        LogText = logBuilder.ToString();
    }
}
