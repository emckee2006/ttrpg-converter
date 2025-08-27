using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Data.Core;
using Avalonia.Data.Core.Plugins;
using System.Linq;
using Avalonia.Markup.Xaml;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using TTRPGConverter.GUI.ViewModels;
using TTRPGConverter.GUI.Views;
using TTRPGConverter.Infrastructure;
using TTRPGConverter.Infrastructure.Services.Compendium;

namespace TTRPGConverter.GUI;

public partial class App : Application
{
    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public override void OnFrameworkInitializationCompleted()
    {
        var services = new ServiceCollection();
        ConfigureServices(services);
        var serviceProvider = services.BuildServiceProvider();

        if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
        {
            DisableAvaloniaDataAnnotationValidation();
            desktop.MainWindow = new MainWindow
            {
                DataContext = serviceProvider.GetRequiredService<MainWindowViewModel>(),
            };
        }

        base.OnFrameworkInitializationCompleted();
    }

    private void ConfigureServices(IServiceCollection services)
    {
        services.AddLogging(configure =>
        {
            // In a real app, you'd configure this to write to a file or a logging service.
            // For now, we'll just use the debug console.
            configure.AddDebug();
            configure.SetMinimumLevel(LogLevel.Information);
        });

        // Add the DbContextFactory for our SQLite database
        services.AddDbContextFactory<TTRPGConverterContext>();

        // Add our application's services
        services.AddTransient<FoundryModuleService>();
        services.AddTransient<CompendiumCacheBuilder>();

        // Add our ViewModels
        services.AddTransient<MainWindowViewModel>();
    }

    private void DisableAvaloniaDataAnnotationValidation()
    {
        var dataValidationPluginsToRemove =
            BindingPlugins.DataValidators.OfType<DataAnnotationsValidationPlugin>().ToArray();

        foreach (var plugin in dataValidationPluginsToRemove)
        {
            BindingPlugins.DataValidators.Remove(plugin);
        }
    }
}
