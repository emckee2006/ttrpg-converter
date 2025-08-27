using System.Text;
using TTRPGConverter.Core.Compendium;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public class CollisionLogger : IDisposable
{
    private readonly StreamWriter _writer;
    private bool _headerWritten;

    public CollisionLogger(string filePath)
    {
        var stream = new FileStream(filePath, FileMode.Create, FileAccess.Write, FileShare.Read);
        _writer = new StreamWriter(stream, Encoding.UTF8);
    }

    public void LogCollision(CompendiumItem winner, CompendiumItem loser)
    {
        if (!_headerWritten)
        {
            _writer.WriteLine("WinnerSourceFile,WinnerName,WinnerType,LoserSourceFile,LoserName,LoserType");
            _headerWritten = true;
        }

        var winnerLine = $"\"{winner.SourceFile}\",\"{winner.Name}\",\"{winner.Type}\"";
        var loserLine = $"\"{loser.SourceFile}\",\"{loser.Name}\",\"{loser.Type}\"";
        _writer.WriteLine($"{winnerLine},{loserLine}");
    }

    public void Dispose()
    {
        _writer.Dispose();
        GC.SuppressFinalize(this);
    }
}
