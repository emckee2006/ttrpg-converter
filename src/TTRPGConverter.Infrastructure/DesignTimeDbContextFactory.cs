using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Design;

namespace TTRPGConverter.Infrastructure;

/// <summary>
/// This factory is used by the EF Core design-time tools (e.g., for creating migrations).
/// It provides a way for the tools to create an instance of the DbContext without needing
/// to rely on the application's dependency injection container.
/// </summary>
public class DesignTimeDbContextFactory : IDesignTimeDbContextFactory<TTRPGConverterContext>
{
    public TTRPGConverterContext CreateDbContext(string[] args)
    {
        var optionsBuilder = new DbContextOptionsBuilder<TTRPGConverterContext>();
        // Use a default connection string for design-time operations.
        // This will be the name of the database file created by the update-compendium command.
        optionsBuilder.UseSqlite("Data Source=compendium.db");

        return new TTRPGConverterContext(optionsBuilder.Options);
    }
}
