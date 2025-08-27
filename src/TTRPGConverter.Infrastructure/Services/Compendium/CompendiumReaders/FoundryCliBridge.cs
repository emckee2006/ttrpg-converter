using System.Diagnostics;
using System.Text;
using System.Text.Json;
using Microsoft.Extensions.Logging;
using TTRPGConverter.Core.Compendium;

namespace TTRPGConverter.Infrastructure.Services.Compendium.CompendiumReaders;

/// <summary>
/// A compendium reader that uses a PowerShell script to interface with the Foundry VTT CLI 
/// to unpack LevelDB (.db) compendium packs into JSON.
/// </summary>
public class FoundryCliBridge : ICompendiumReader, IDisposable
{
    private readonly ILogger<FoundryCliBridge> _logger;
    private readonly bool _verbose;

    public string FormatName => "Foundry LevelDB (via CLI)";

    public FoundryCliBridge(ILogger<FoundryCliBridge> logger, bool verbose = false)
    {
        _logger = logger;
        _verbose = verbose;
    }

    public bool SupportsFormat(string packPath)
    {
        // This bridge is specifically for LevelDB directories.
        return Directory.Exists(packPath) && new[] { "CURRENT", "LOCK" }.All(f => File.Exists(Path.Combine(packPath, f)));
    }

    // This is the new method that accepts the full context.
    public Dictionary<string, CompendiumItem> LoadPackWithContext(string packPath, string sourceName, string? packType, string? systemName)
    {
        var packName = Path.GetFileName(packPath);
        return LoadLevelDBViaFoundryCLI(packName, sourceName, _verbose, packType, systemName);
    }

    // These methods satisfy the ICompendiumReader interface but lack the full context.
    public Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName)
    {
        return LoadPackWithContext(packPath, sourceName, null, null);
    }

    public Dictionary<string, CompendiumItem> LoadPack(string packPath)
    {
        return LoadPack(packPath, Path.GetDirectoryName(packPath) ?? "unknown");
    }

    private Dictionary<string, CompendiumItem> LoadLevelDBViaFoundryCLI(string packName, string sourceName, bool verbose, string? packType, string? systemName)
    {
        var currentDir = Directory.GetCurrentDirectory();
        var rootDir = currentDir;
        while (rootDir != null && !File.Exists(Path.Combine(rootDir, "package.json"))) { rootDir = Directory.GetParent(rootDir)?.FullName; }
        if (rootDir == null) throw new Exception($"Could not find root project directory with package.json from: {currentDir}");
        
        var scriptPath = Path.Combine(rootDir, "src", "TTRPGConverter.Infrastructure", "Tools", "foundry-extract.ps1");
        if (!File.Exists(scriptPath)) throw new Exception($"Foundry CLI PowerShell script not found: {scriptPath}");

        var (packageId, cliPackageType) = DeterminePackageContext(sourceName);
        var debugFlag = verbose ? "-ShowDebug" : "";
        var startInfo = new ProcessStartInfo
        {
            FileName = "pwsh.exe",
            Arguments = $@"-ExecutionPolicy Bypass -NoProfile -NonInteractive -File ""{scriptPath}"" -PackName ""{packName}"" -PackageId ""{packageId}"" -PackageType ""{cliPackageType}"" {debugFlag}",
            WorkingDirectory = rootDir,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true
        };

        using var process = Process.Start(startInfo);
        if (process == null) throw new Exception("Failed to start PowerShell process");

        var outputBuilder = new StringBuilder();
        var errorBuilder = new StringBuilder();
        var outputTcs = new TaskCompletionSource<bool>();
        var errorTcs = new TaskCompletionSource<bool>();

        process.OutputDataReceived += (sender, e) => { 
            if (e.Data != null) outputBuilder.AppendLine(e.Data); else outputTcs.SetResult(true); 
        };
        process.ErrorDataReceived += (sender, e) => { 
            if (e.Data != null) errorBuilder.AppendLine(e.Data); else errorTcs.SetResult(true); 
        };

        process.BeginOutputReadLine();
        process.BeginErrorReadLine();

        if (process.WaitForExit(90000) && outputTcs.Task.Wait(1000) && errorTcs.Task.Wait(1000))
        {
            var output = outputBuilder.ToString().Trim();
            var error = errorBuilder.ToString().Trim();

            if (verbose && !string.IsNullOrEmpty(output)) _logger.LogDebug("PowerShell stdout: {Output}", output);
            if (!string.IsNullOrEmpty(error)) _logger.LogWarning("PowerShell stderr: {Error}", error);

            if (process.ExitCode != 0) throw new Exception($"PowerShell process exited with code {process.ExitCode}. Error: {error}");

            var outputDirectory = output.Split(new[] { '\r', '\n' }, StringSplitOptions.RemoveEmptyEntries).LastOrDefault(Directory.Exists);
            if (string.IsNullOrEmpty(outputDirectory)) throw new Exception($"Output directory not found or invalid in script output: {output}");
            outputDirectory = outputDirectory.Trim();

            var items = new Dictionary<string, CompendiumItem>();
            foreach (var filePath in Directory.GetFiles(outputDirectory, "*.json"))
            {
                var fileName = Path.GetFileNameWithoutExtension(filePath);
                var jsonContent = File.ReadAllText(filePath);
                var jsonDoc = JsonDocument.Parse(jsonContent);
                var name = jsonDoc.RootElement.TryGetProperty("name", out var nameProp) ? nameProp.GetString() ?? fileName : fileName;
                var itemType = jsonDoc.RootElement.TryGetProperty("type", out var typeProp) ? typeProp.GetString() : null;

                if (string.IsNullOrEmpty(itemType) || itemType.Equals("unknown", StringComparison.OrdinalIgnoreCase))
                {
                    itemType = packType switch
                    {
                        "Actor" => "npc",
                        "Item" => "item",
                        "JournalEntry" => "journal",
                        "RollTable" => "rolltable",
                        "Scene" => "scene",
                        _ => packType?.ToLowerInvariant() ?? "unknown"
                    };
                }

                items[fileName] = new CompendiumItem { Id = fileName, Name = name, Type = itemType, Data = jsonDoc, SourceFormat = "LevelDB-CLI", SourceFile = packName, IsPrimary = true, System = systemName ?? "unknown" };
            }
            Directory.Delete(outputDirectory, true);
            return items;
        }
        else
        {
            try { process.Kill(); } catch { }
            throw new Exception("Foundry CLI PowerShell process timed out or streams did not close.");
        }
    }

    private static (string packageId, string packageType) DeterminePackageContext(string sourceName)
    {
        if (sourceName.Equals("dnd5e", StringComparison.OrdinalIgnoreCase)) return ("dnd5e", "System");
        if (sourceName.Equals("pf2e", StringComparison.OrdinalIgnoreCase)) return ("pf2e", "System");
        if (sourceName.Equals("pf1", StringComparison.OrdinalIgnoreCase)) return ("pf1", "System");
        if (sourceName.Contains("dungeon-masters-guide")) return ("dnd-dungeon-masters-guide", "Module");
        if (sourceName.Contains("players-handbook")) return ("dnd-players-handbook", "Module");
        return (sourceName, "Module");
    }

    public void Dispose()
    {
        GC.SuppressFinalize(this);
    }
}
