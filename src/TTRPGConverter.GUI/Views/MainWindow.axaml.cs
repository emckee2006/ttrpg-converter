using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Platform.Storage;
using Avalonia.Threading;
using Avalonia.VisualTree;
using System.Collections.Specialized;
using System.ComponentModel;
using TTRPGConverter.GUI.ViewModels;

namespace TTRPGConverter.GUI.Views;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();

        DataContextChanged += (sender, args) =>
        {
            if (DataContext is MainWindowViewModel viewModel)
            {
                viewModel.PropertyChanged += ViewModel_PropertyChanged;
                viewModel.BuildResults.CollectionChanged += BuildResults_CollectionChanged;
            }
        };
    }

    private void ViewModel_PropertyChanged(object? sender, PropertyChangedEventArgs e)
    {
        if (e.PropertyName == nameof(MainWindowViewModel.LogText))
        {
            LogTextBox.CaretIndex = LogTextBox.Text?.Length ?? 0;
        }
    }

    private void BuildResults_CollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        if (e.Action == NotifyCollectionChangedAction.Add)
        {
            Dispatcher.UIThread.Post(() =>
            {
                var scrollViewer = ResultsTreeView.FindDescendantOfType<ScrollViewer>();
                if (scrollViewer != null)
                {
                    scrollViewer.Offset = new Vector(0, scrollViewer.Extent.Height);
                }
            }, DispatcherPriority.Background);
        }
    }

    private async void UpdateCompendiumButton_Click(object? sender, RoutedEventArgs e)
    {
        if (DataContext is not MainWindowViewModel viewModel) return;

        var topLevel = GetTopLevel(this);
        if (topLevel == null) return;

        var file = await topLevel.StorageProvider.SaveFilePickerAsync(new FilePickerSaveOptions
        {
            Title = "Save Compendium Database",
            SuggestedFileName = "compendium.db",
            DefaultExtension = "db",
            FileTypeChoices = new[] { new FilePickerFileType("SQLite Database") { Patterns = new[] { "*.db" } } }
        });

        if (file is not null)
        {
            var localPath = file.TryGetLocalPath();
            if (localPath is not null)
            {
                await viewModel.StartUpdateCompendiumAsync(localPath);
            }
        }
    }
}
