# M4: Enhancement Pipeline

**Timeline**: 2 weeks  
**Status**: 🔴 Blocked (requires M2-M3)  
**Priority**: High - Quality Improvements

## Overview

Implement enhancement pipeline for existing Foundry worlds, focusing on compendium matching to replace homebrew content with official entries, and comprehensive asset optimization.

## Key Deliverables

### Week 1: Compendium Matching
- 🔲 Foundry compendium data loading and indexing
- 🔲 Fuzzy matching algorithms for homebrew content
- 🔲 Official content replacement with confidence scoring
- 🔲 Manual review interface for uncertain matches

### Week 2: Asset Optimization  
- 🔲 Professional image processing with ImageSharp
- 🔲 WebP conversion and multi-resolution generation
- 🔲 Audio format standardization and compression
- 🔲 Broken link detection and repair suggestions

## Enhancement Architecture

### Compendium Matcher
```csharp
public class CompendiumMatcher
{
    private readonly IFuzzyMatcher _fuzzyMatcher;
    private readonly ICompendiumService _compendium;
    
    public async Task<MatchResult> FindOfficialContentAsync(
        FoundryItem homebrewItem, 
        float confidenceThreshold = 0.8f)
    {
        var candidates = await _compendium.SearchAsync(homebrewItem.Name);
        var bestMatch = _fuzzyMatcher.FindBestMatch(homebrewItem, candidates);
        
        return new MatchResult(bestMatch, bestMatch.Confidence);
    }
}
```

### Asset Optimizer
```csharp
public class AssetOptimizer
{
    public async Task<OptimizedAsset> OptimizeImageAsync(string assetPath)
    {
        using var image = await Image.LoadAsync(assetPath);
        
        // Generate multiple resolutions
        var thumbnail = image.Clone(x => x.Resize(150, 150));
        var medium = image.Clone(x => x.Resize(512, 512));
        
        // Convert to WebP with quality settings
        var webpOptions = new WebpEncoder { Quality = 85 };
        
        return new OptimizedAsset
        {
            Thumbnail = await SaveAsWebP(thumbnail, webpOptions),
            Medium = await SaveAsWebP(medium, webpOptions),
            Original = await SaveAsWebP(image, webpOptions)
        };
    }
}
```

## Compendium Integration

### Data Sources
```csharp
public interface ICompendiumService
{
    Task<List<CompendiumItem>> LoadOfficialSpellsAsync();
    Task<List<CompendiumItem>> LoadOfficialItemsAsync();
    Task<List<CompendiumItem>> LoadOfficialMonstersAsync();
    Task<List<CompendiumItem>> SearchAsync(string query);
}
```

### Matching Algorithms
- **Exact Name Match** - Direct string comparison
- **Fuzzy String Match** - Levenshtein distance with threshold
- **Semantic Similarity** - Description and mechanics comparison
- **Statistical Properties** - Damage, AC, HP ranges for monsters

### Confidence Scoring
```csharp
public class MatchConfidence
{
    public float NameSimilarity { get; set; }     // 0.0 - 1.0
    public float DescriptionMatch { get; set; }   // 0.0 - 1.0  
    public float MechanicsMatch { get; set; }     // 0.0 - 1.0
    public float StatisticsMatch { get; set; }    // 0.0 - 1.0
    
    public float OverallConfidence => 
        (NameSimilarity * 0.4f) + 
        (DescriptionMatch * 0.2f) + 
        (MechanicsMatch * 0.3f) + 
        (StatisticsMatch * 0.1f);
}
```

## Asset Processing Pipeline

### Image Optimization
- **Format Conversion**: PNG/JPG → WebP
- **Resolution Generation**: Thumbnail (150px), Medium (512px), Full
- **Quality Optimization**: 85% WebP quality for good compression
- **Metadata Preservation**: Copyright and attribution info

### Audio Processing  
- **Format Standardization**: Various → OGG Vorbis
- **Bitrate Optimization**: Adaptive bitrate based on content
- **Volume Normalization**: Consistent audio levels
- **Silence Detection**: Trim unnecessary silence

### Performance Optimization
```csharp
public class AssetProcessor
{
    public async Task<ProcessingResults> ProcessAllAssetsAsync(
        FoundryWorld world, 
        IProgress<ProcessingProgress> progress)
    {
        var tasks = world.Assets.Select(async asset =>
        {
            try
            {
                return await OptimizeAssetAsync(asset);
            }
            catch (Exception ex)
            {
                _logger.LogWarning("Asset processing failed: {Asset}", asset.Path);
                return new AssetResult { Success = false, Error = ex.Message };
            }
        });
        
        var results = await Task.WhenAll(tasks);
        return new ProcessingResults(results);
    }
}
```

## Quality Assurance

### Validation Tests
- Asset integrity after processing
- Compendium match accuracy validation
- Performance regression testing
- Memory usage monitoring

### Manual Review Interface
```csharp
public class EnhancementReview
{
    public List<CompendiumMatch> UncertainMatches { get; set; }
    public List<AssetIssue> BrokenAssets { get; set; }
    public List<QualityWarning> QualityIssues { get; set; }
}
```

## Success Criteria

- [ ] >90% of homebrew spells/items matched to official content
- [ ] Asset sizes reduced by >50% with minimal quality loss
- [ ] All images converted to WebP format successfully
- [ ] Broken asset links detected and reported
- [ ] Processing time <5 minutes for typical campaign
- [ ] Enhanced world loads 25% faster in Foundry

## Integration Points

- **Input**: Any Foundry world file
- **Output**: Enhanced world with official content and optimized assets
- **Dependencies**: Official Foundry compendium packs
- **Performance**: Parallel processing for asset optimization

## Next Milestone

→ M5: Platform Conversion (Roll20 same-system → Foundry) (1.5 weeks)
