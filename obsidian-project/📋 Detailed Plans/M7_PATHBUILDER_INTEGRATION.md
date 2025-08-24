# M7: Pathbuilder Integration

**Timeline**: 1 week  
**Status**: 🔴 Blocked (requires M2, M6)  
**Priority**: Medium - External Integration Feature

## Overview

Integrate Pathbuilder 2e character import functionality using simplified platform-only conversion. Since Pathbuilder exports PF2e data and we only need Foundry PF2e output, no system conversion is required - only platform format translation.

## Key Deliverables

### Week 1: Streamlined Integration
- 🔲 Pathbuilder 2e API client implementation  
- 🔲 Character export format analysis and parsing
- 🔲 **Platform-only conversion** (Pathbuilder PF2e → Foundry PF2e)
- 🔲 Authentication and rate limiting handling
- 🔲 Character data deserialization to PathbuilderCharacter model
- 🔲 GUI integration for character import workflow
- 🔲 Batch character import from Pathbuilder URLs/files
- 🔲 Batch character import from multiple URLs
- 🔲 Campaign-level organization and metadata
- 🔲 GUI integration with existing interface
- 🔲 Export options and validation

## API Integration

### Pathbuilder Service
```csharp
public class PathbuilderService
{
    private readonly HttpClient _httpClient;
    private readonly ILogger<PathbuilderService> _logger;
    
    public async Task<PathbuilderCharacter> ImportCharacterAsync(int characterId)
    {
        var url = $"https://pathbuilder2e.com/json.php?id={characterId}";
        var response = await _httpClient.GetAsync(url);
        
        if (!response.IsSuccessStatusCode)
        {
            throw new PathbuilderException($"Character {characterId} not found or not public");
        }
        
        var json = await response.Content.ReadAsStringAsync();
        return JsonSerializer.Deserialize<PathbuilderCharacter>(json);
    }
    
    public async Task<List<PathbuilderCharacter>> ImportCampaignAsync(List<int> characterIds)
    {
        var tasks = characterIds.Select(ImportCharacterAsync);
        return (await Task.WhenAll(tasks)).ToList();
    }
}
```

### URL Parsing
```csharp
public class PathbuilderUrlParser
{
    public int? ExtractCharacterId(string url)
    {
        // https://pathbuilder2e.com/launch.html?build=12345
        // https://pathbuilder2e.com/json.php?id=12345
        
        var patterns = new[]
        {
            @"build=(\d+)",
            @"id=(\d+)"
        };
        
        foreach (var pattern in patterns)
        {
            var match = Regex.Match(url, pattern);
            if (match.Success && int.TryParse(match.Groups[1].Value, out var id))
            {
                return id;
            }
        }
        
        return null;
    }
}
```

## Character Conversion

### Pathbuilder → Foundry Mapping
```csharp
public class PathbuilderToFoundryConverter
{
    public FoundryPf2eActor ConvertCharacter(PathbuilderCharacter pbCharacter)
    {
        return new FoundryPf2eActor
        {
            Name = pbCharacter.Name,
            System = new FoundryPf2eSystemData
            {
                Details = new FoundryPf2eDetails
                {
                    Class = ConvertClass(pbCharacter.Class),
                    Ancestry = ConvertAncestry(pbCharacter.Ancestry),
                    Background = ConvertBackground(pbCharacter.Background),
                    Level = pbCharacter.Level
                },
                Abilities = ConvertAbilities(pbCharacter.Abilities),
                Skills = ConvertSkills(pbCharacter.Skills)
            },
            Items = ConvertFeatsAndEquipment(pbCharacter)
        };
    }
}
```

### Feat System Translation
```csharp
private List<FoundryItem> ConvertFeats(List<PathbuilderFeat> pbFeats)
{
    return pbFeats.Select(feat => new FoundryItem
    {
        Name = feat.Name,
        Type = "feat",
        System = new FoundryFeatSystemData
        {
            FeatType = DetermineFeatType(feat.Category),
            Prerequisites = feat.Prerequisites,
            Description = feat.Description,
            Level = feat.Level
        }
    }).ToList();
}
```

## Batch Processing

### Campaign Import
```csharp
public class PathbuilderCampaignImporter
{
    public async Task<FoundryWorld> ImportCampaignAsync(
        List<string> characterUrls, 
        string campaignName)
    {
        var characterIds = characterUrls
            .Select(_urlParser.ExtractCharacterId)
            .Where(id => id.HasValue)
            .Select(id => id.Value)
            .ToList();
        
        var characters = await _pathbuilderService.ImportCampaignAsync(characterIds);
        var foundryActors = characters.Select(_converter.ConvertCharacter).ToList();
        
        return new FoundryWorld
        {
            Title = campaignName,
            System = "pf2e",
            Actors = foundryActors,
            Scenes = new[] { CreateDefaultScene() },
            JournalPages = new[] { CreateWelcomeJournal(campaignName) }
        };
    }
}
```

### Rate Limiting
```csharp
public class RateLimitedPathbuilderService
{
    private readonly SemaphoreSlim _semaphore = new(5); // 5 concurrent requests
    private readonly Dictionary<DateTime, int> _requestCounts = new();
    
    public async Task<PathbuilderCharacter> ImportCharacterAsync(int characterId)
    {
        await _semaphore.WaitAsync();
        try
        {
            await EnforceRateLimit();
            return await _pathbuilderService.ImportCharacterAsync(characterId);
        }
        finally
        {
            _semaphore.Release();
        }
    }
    
    private async Task EnforceRateLimit()
    {
        // Pathbuilder allows ~60 requests/minute
        var now = DateTime.UtcNow;
        var minute = new DateTime(now.Year, now.Month, now.Day, now.Hour, now.Minute, 0);
        
        if (_requestCounts.TryGetValue(minute, out var count) && count >= 50)
        {
            await Task.Delay(TimeSpan.FromSeconds(60 - now.Second));
        }
        
        _requestCounts[minute] = count + 1;
    }
}
```

## GUI Integration

### Pathbuilder Import Tab
```xml
<TabItem Header="Pathbuilder Import">
    <StackPanel>
        <TextBlock Text="Import characters from Pathbuilder 2e" />
        
        <TextBox Watermark="Pathbuilder character URL or ID"
                 Text="{Binding PathbuilderUrl}" />
        
        <Button Content="Add Character" 
                Command="{Binding AddCharacterCommand}" />
        
        <ListBox ItemsSource="{Binding CharacterUrls}">
            <ListBox.ItemTemplate>
                <DataTemplate>
                    <Grid>
                        <TextBlock Text="{Binding}" />
                        <Button Content="Remove" 
                                Command="{Binding $parent.DataContext.RemoveCharacterCommand}"
                                CommandParameter="{Binding}" />
                    </Grid>
                </DataTemplate>
            </ListBox.ItemTemplate>
        </ListBox>
        
        <Button Content="Import Campaign"
                Command="{Binding ImportCampaignCommand}" />
    </StackPanel>
</TabItem>
```

### Progress Monitoring
```csharp
public class PathbuilderImportProgress
{
    public int TotalCharacters { get; set; }
    public int CompletedCharacters { get; set; }
    public string CurrentCharacter { get; set; }
    public List<string> FailedCharacters { get; set; } = new();
    
    public double ProgressPercentage => 
        TotalCharacters > 0 ? (double)CompletedCharacters / TotalCharacters * 100 : 0;
}
```

## Error Handling

### Common Issues
- **Character not found** - Invalid ID or private character
- **Rate limiting** - Too many requests in short time
- **Invalid JSON** - Pathbuilder API changes or corrupted data
- **Network issues** - Connectivity or timeout problems

### User-Friendly Messages
```csharp
public static class PathbuilderErrorMessages
{
    public static string GetUserMessage(PathbuilderException ex)
    {
        return ex.ErrorType switch
        {
            PathbuilderError.CharacterNotFound => 
                "Character not found. Make sure the character is public and the URL is correct.",
            PathbuilderError.RateLimited => 
                "Too many requests. Please wait a moment and try again.",
            PathbuilderError.InvalidUrl => 
                "Invalid Pathbuilder URL. Please check the format.",
            _ => "An unexpected error occurred while importing the character."
        };
    }
}
```

## Success Criteria

- [ ] Successfully import individual characters from Pathbuilder URLs
- [ ] Batch import multiple characters for full campaign
- [ ] All character data (feats, spells, equipment) converts accurately
- [ ] Rate limiting prevents API abuse
- [ ] Error handling provides clear user feedback
- [ ] Import process completes in <30 seconds for typical character
- [ ] Generated Foundry actors load correctly in PF2e system

## Optional Enhancements

- **Character portraits** - Import character art from Pathbuilder
- **Spell preparation** - Convert prepared spell lists
- **Custom content** - Handle homebrew feats and items
- **Version tracking** - Import character build history

## Next Steps

With M7 complete, the TTRPG Converter provides comprehensive conversion capabilities across all major platforms and use cases, with an estimated 13-week total development timeline.
