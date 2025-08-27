# M6: GUI Application Development

**Timeline**: 1.5 weeks  
**Status**: 🔴 Blocked (requires M2, M2.5, M3)  
**Priority**: High - User Experience

## Overview

Develop a modern, cross-platform GUI application using Avalonia UI for intuitive TTRPG campaign conversion. Leverages two-stage architecture for enhanced progress visualization, error isolation, and conversion pipeline management.

## Key Deliverables

### Week 1: Core GUI Framework & Two-Stage Integration
- 🔲 Avalonia UI project setup and configuration
- 🔲 Main window layout with modern design principles
- 🔲 File selection and drag-and-drop interface
- 🔲 **Two-stage progress visualization** (Platform → System conversion)
- 🔲 **Stage isolation controls** (run platform-only or full conversion)

### Week 1.5: Advanced Features & Polish
- 🔲 Integration with M2, M2.5, M3 conversion engines
- 🔲 **Error isolation by stage** (platform vs system conversion errors)
- 🔲 Batch processing queue management
- 🔲 **Conversion preview** (intermediate Foundry output before system conversion)
- 🔲 Output file validation and previewrences
- 🔲 Results visualization and export options

## GUI Architecture

### Main Application
```csharp
public partial class MainWindow : Window
{
    private readonly IConversionService _converter;
    private readonly ISettingsService _settings;
    
    public MainWindow(IConversionService converter, ISettingsService settings)
    {
        InitializeComponent();
        _converter = converter;
        _settings = settings;
        DataContext = new MainViewModel(converter, settings);
    }
}
```

### Conversion Wizard
```csharp
public class ConversionWizardViewModel : ViewModelBase
{
    public ObservableCollection<ConversionPipeline> AvailablePipelines { get; set; }
    public ConversionPipeline SelectedPipeline { get; set; }
    public string InputPath { get; set; }
    public string OutputPath { get; set; }
    public bool IsConverting { get; set; }
    public double ProgressPercentage { get; set; }
    
    public async Task StartConversionAsync()
    {
        var progress = new Progress<ConversionProgress>(UpdateProgress);
        var result = await _converter.ConvertAsync(
            SelectedPipeline, 
            InputPath, 
            OutputPath, 
            progress);
        
        ShowResults(result);
    }
}
```

## User Interface Design

### Main Window Layout
```xml
<Grid>
    <Grid.RowDefinitions>
        <RowDefinition Height="Auto" />   <!-- Menu bar -->
        <RowDefinition Height="*" />      <!-- Main content -->
        <RowDefinition Height="200" />    <!-- Log panel -->
    </Grid.RowDefinitions>
    
    <!-- Conversion wizard in center -->
    <UserControl Grid.Row="1" Content="{Binding CurrentView}" />
    
    <!-- Real-time logging panel -->
    <ScrollViewer Grid.Row="2">
        <TextBlock Text="{Binding LogOutput}" />
    </ScrollViewer>
</Grid>
```

### Pipeline Selection
```xml
<ComboBox ItemsSource="{Binding AvailablePipelines}"
          SelectedItem="{Binding SelectedPipeline}">
    <ComboBox.ItemTemplate>
        <DataTemplate>
            <StackPanel>
                <TextBlock Text="{Binding DisplayName}" FontWeight="Bold" />
                <TextBlock Text="{Binding Description}" FontSize="10" />
            </StackPanel>
        </DataTemplate>
    </ComboBox.ItemTemplate>
</ComboBox>
```

## Conversion Pipelines

### Pipeline Definitions
```csharp
public static class ConversionPipelines
{
    public static readonly ConversionPipeline Roll205eToPf2e = new()
    {
        Id = "roll20-5e-to-foundry-pf2e",
        DisplayName = "Roll20 D&D 5e → Foundry PF2e",
        Description = "Convert Roll20 D&D 5e campaign to Foundry Pathfinder 2e",
        InputType = InputType.Roll20Zip,
        OutputType = OutputType.FoundryWorld,
        RequiresSystemConversion = true
    };
    
    public static readonly ConversionPipeline FoundryPf1eToPf2e = new()
    {
        Id = "foundry-pf1e-to-pf2e", 
        DisplayName = "Foundry PF1e → PF2e",
        Description = "Convert Foundry Pathfinder 1e to Pathfinder 2e",
        InputType = InputType.FoundryWorld,
        OutputType = OutputType.FoundryWorld,
        RequiresSystemConversion = true
    };
    
    public static readonly ConversionPipeline FoundryEnhancement = new()
    {
        Id = "foundry-enhancement",
        DisplayName = "Foundry World Enhancement", 
        Description = "Optimize assets and match homebrew to official content",
        InputType = InputType.FoundryWorld,
        OutputType = OutputType.FoundryWorld,
        RequiresSystemConversion = false
    };
}
```

### Progress Monitoring
```csharp
public class ConversionProgress
{
    public string CurrentStep { get; set; }
    public double Percentage { get; set; }
    public TimeSpan Elapsed { get; set; }
    public TimeSpan? EstimatedRemaining { get; set; }
    public List<string> Warnings { get; set; } = new();
    public List<string> Errors { get; set; } = new();
}
```

## File Handling

### Drag and Drop
```csharp
public class FileDropHandler
{
    public void HandleDrop(DragEventArgs e)
    {
        if (e.Data.Contains(DataFormats.Files))
        {
            var files = e.Data.GetFiles();
            var file = files?.FirstOrDefault();
            
            if (file != null)
            {
                var extension = Path.GetExtension(file.Path.LocalPath);
                var pipeline = DetectPipelineFromFile(extension);
                
                ViewModel.SelectedPipeline = pipeline;
                ViewModel.InputPath = file.Path.LocalPath;
            }
        }
    }
}
```

### File Type Detection
```csharp
public static ConversionPipeline DetectPipelineFromFile(string extension)
{
    return extension.ToLowerInvariant() switch
    {
        ".zip" => ConversionPipelines.Roll205eToPf2e, // Default for ZIP
        ".json" when IsFoundryWorld(_) => ConversionPipelines.FoundryEnhancement,
        _ => null
    };
}
```

## Settings Management

### Application Settings
```csharp
public class AppSettings
{
    public string DefaultOutputDirectory { get; set; }
    public bool EnableAssetOptimization { get; set; } = true;
    public bool EnableCompendiumMatching { get; set; } = true;
    public LogLevel LogLevel { get; set; } = LogLevel.Information;
    public Theme ApplicationTheme { get; set; } = Theme.System;
}
```

### Settings UI
```xml
<TabControl>
    <TabItem Header="General">
        <StackPanel>
            <CheckBox Content="Enable asset optimization" 
                      IsChecked="{Binding Settings.EnableAssetOptimization}" />
            <CheckBox Content="Enable compendium matching"
                      IsChecked="{Binding Settings.EnableCompendiumMatching}" />
        </StackPanel>
    </TabItem>
    
    <TabItem Header="Paths">
        <StackPanel>
            <TextBox Text="{Binding Settings.DefaultOutputDirectory}" />
            <Button Command="{Binding BrowseOutputDirectoryCommand}"
                    Content="Browse..." />
        </StackPanel>
    </TabItem>
</TabControl>
```

## Error Handling and Logging

### Real-time Log Display
```csharp
public class LogViewModel : ViewModelBase, ILogEventSink
{
    public string LogOutput { get; set; }
    
    public void Emit(LogEvent logEvent)
    {
        var message = logEvent.RenderMessage();
        var timestamp = logEvent.Timestamp.ToString("HH:mm:ss");
        var level = logEvent.Level.ToString().ToUpper();
        
        var formatted = $"[{timestamp}] {level}: {message}\n";
        
        Dispatcher.UIThread.Post(() =>
        {
            LogOutput += formatted;
            NotifyPropertyChanged(nameof(LogOutput));
        });
    }
}
```

### Error Visualization
- Color-coded log messages (errors in red, warnings in orange)
- Error summary panel with actionable suggestions
- Export log functionality for troubleshooting
- Automatic error reporting option

## Success Criteria

- [ ] Intuitive wizard-style interface requiring minimal user input
- [ ] Real-time progress updates with accurate time estimates
- [ ] All conversion pipelines accessible through GUI
- [ ] Drag-and-drop functionality working smoothly
- [ ] Error messages clear and actionable
- [ ] Application starts in <2 seconds
- [ ] Cross-platform compatibility (Windows, macOS, Linux)

## Cross-Platform Considerations

- **Avalonia UI** - Native cross-platform support
- **File dialogs** - Platform-appropriate file browsers
- **Path handling** - Cross-platform path management
- **Fonts and themes** - System-appropriate styling

## Next Milestone

→ M7: Pathbuilder Integration (optional, 1 week)
