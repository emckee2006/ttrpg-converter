using System;
using Spectre.Console;
using TTRPGConverter.Core;

namespace TTRPGConverter.CLI.Services;

public class ProgressService : IProgress<ProgressUpdate>
{
    private ProgressTask? _overallTask;
    private ProgressTask? _detailTask;

    public void Start(Action<IProgress<ProgressUpdate>> action)
    {
        AnsiConsole.Progress()
            .Columns(new ProgressColumn[]
            {
                new TaskDescriptionColumn(),
                new ProgressBarColumn(),
                new PercentageColumn(),
                new RemainingTimeColumn(),
                new SpinnerColumn(),
            })
            .Start(ctx =>
            {
                _overallTask = ctx.AddTask("[green]Overall Progress[/]");
                _detailTask = ctx.AddTask("[blue]Details[/]");
                action(this); // Pass the service itself as the IProgress<T> implementation
            });
    }

    public void Report(ProgressUpdate value)
    {
        if (_overallTask != null)
        {
            if (value.OverallMaxValue.HasValue) _overallTask.MaxValue = value.OverallMaxValue.Value;
            if (value.OverallIncrement.HasValue) _overallTask.Increment(value.OverallIncrement.Value);
            if (!string.IsNullOrEmpty(value.OverallDescription)) _overallTask.Description = value.OverallDescription;
        }
        if (_detailTask != null)
        {
            if (value.DetailMaxValue.HasValue) _detailTask.MaxValue = value.DetailMaxValue.Value;
            if (value.DetailIncrement.HasValue) _detailTask.Increment(value.DetailIncrement.Value);
            if (!string.IsNullOrEmpty(value.DetailDescription)) _detailTask.Description = value.DetailDescription;
        }
    }
}
