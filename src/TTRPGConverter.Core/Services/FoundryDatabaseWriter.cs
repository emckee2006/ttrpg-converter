using System.Text.Json;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.DependencyInjection;

namespace TTRPGConverter.Core.Services;

/// <summary>
/// Interface for writing Foundry VTT database files in proper format
/// </summary>
public interface IFoundryDatabaseWriter : IDisposable
{
    string DatabasePath { get; }
    Task<bool> InitializeAsync(string dbPath);
    Task<string> InsertAsync<T>(T entity) where T : IFoundryEntity;
    Task<List<string>> InsertManyAsync<T>(IEnumerable<T> entities) where T : IFoundryEntity;
    Task<bool> OptimizeAsync();
}

/// <summary>
/// Base interface for all Foundry entities
/// </summary>
public interface IFoundryEntity
{
    string Id { get; set; }
    string Name { get; set; }
}

/// <summary>
/// NeDB-compatible database writer (JSON Lines format)
/// Creates .db files that Foundry VTT can read natively
/// </summary>
public class NeDBDatabaseWriter : IFoundryDatabaseWriter
{
    private readonly SemaphoreSlim _writeLock = new(1, 1);
    private readonly JsonSerializerOptions _jsonOptions;
    private readonly ILogger<NeDBDatabaseWriter> _logger;

    public string DatabasePath { get; private set; } = string.Empty;

    public NeDBDatabaseWriter(ILogger<NeDBDatabaseWriter> logger)
    {
        _logger = logger;
        _jsonOptions = new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            WriteIndented = false, // NeDB requires single-line JSON
            DefaultIgnoreCondition = System.Text.Json.Serialization.JsonIgnoreCondition.WhenWritingNull
        };
    }

    public async Task<bool> InitializeAsync(string dbPath)
    {
        DatabasePath = dbPath;
        var directory = Path.GetDirectoryName(dbPath);
        if (!string.IsNullOrEmpty(directory))
        {
            Directory.CreateDirectory(directory);
        }

        // Create empty database file if it doesn't exist
        if (!File.Exists(dbPath))
        {
            await File.WriteAllTextAsync(dbPath, string.Empty);
        }

        _logger.LogInformation("Initialized NeDB database: {Path}", dbPath);
        return true;
    }

    public async Task<string> InsertAsync<T>(T entity) where T : IFoundryEntity
    {
        await _writeLock.WaitAsync();
        try
        {
            var json = JsonSerializer.Serialize(entity, _jsonOptions);
            await File.AppendAllTextAsync(DatabasePath, json + Environment.NewLine);
            
            _logger.LogDebug("Inserted entity {Id} into {Path}", entity.Id, DatabasePath);
            return entity.Id;
        }
        finally
        {
            _writeLock.Release();
        }
    }

    public async Task<List<string>> InsertManyAsync<T>(IEnumerable<T> entities) where T : IFoundryEntity
    {
        await _writeLock.WaitAsync();
        try
        {
            var lines = new List<string>();
            var ids = new List<string>();

            foreach (var entity in entities)
            {
                var json = JsonSerializer.Serialize(entity, _jsonOptions);
                lines.Add(json);
                ids.Add(entity.Id);
            }

            if (lines.Any())
            {
                await File.AppendAllLinesAsync(DatabasePath, lines);
                _logger.LogInformation("Bulk inserted {Count} entities into {Path}", 
                    lines.Count, DatabasePath);
            }

            return ids;
        }
        finally
        {
            _writeLock.Release();
        }
    }

    public async Task<bool> OptimizeAsync()
    {
        // NeDB doesn't require optimization, but we can log completion
        _logger.LogInformation("NeDB optimization complete for {Path}", DatabasePath);
        return await Task.FromResult(true);
    }

    public void Dispose()
    {
        _writeLock?.Dispose();
        GC.SuppressFinalize(this);
    }
}

/// <summary>
/// Factory for creating appropriate database writers
/// </summary>
public class FoundryDatabaseFactory
{
    public enum DatabaseFormat
    {
        NeDB,      // JSON Lines format - maximum Foundry compatibility
        LiteDB     // Modern embedded database - future enhancement
    }

    private readonly IServiceProvider _serviceProvider;

    public FoundryDatabaseFactory(IServiceProvider serviceProvider)
    {
        _serviceProvider = serviceProvider;
    }

    public IFoundryDatabaseWriter CreateWriter(DatabaseFormat format)
    {
        return format switch
        {
            DatabaseFormat.NeDB => _serviceProvider.GetRequiredService<NeDBDatabaseWriter>(),
            DatabaseFormat.LiteDB => throw new NotImplementedException("LiteDB support coming in future release"),
            _ => throw new ArgumentException($"Unsupported database format: {format}")
        };
    }
}
