namespace TTRPGConverter.Core.Interfaces;

using System.Collections.Generic;
using TTRPGConverter.Core.Entities;

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
