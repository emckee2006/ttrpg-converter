namespace TTRPGConverter.Infrastructure.Services.Compendium;

/// <summary>
/// Abstract interface for reading compendium databases
/// </summary>
public interface ICompendiumReader
{
    string FormatName { get; }
    bool SupportsFormat(string packPath);
    Dictionary<string, CompendiumItem> LoadPack(string packPath);
    Dictionary<string, CompendiumItem> LoadPack(string packPath, string sourceName);
}
