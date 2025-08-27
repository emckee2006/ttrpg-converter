using Microsoft.EntityFrameworkCore;
using System.Text.Json;
using TTRPGConverter.Core;
using TTRPGConverter.Core.Compendium;

namespace TTRPGConverter.Infrastructure;

public class TTRPGConverterContext : DbContext
{
    public DbSet<CompendiumItem> CompendiumItems { get; set; } = null!;
    public DbSet<AssetMapping> AssetMappings { get; set; } = null!;

    public TTRPGConverterContext(DbContextOptions<TTRPGConverterContext> options)
        : base(options)
    {
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);

        // == CompendiumItem Configuration ==
        var compendiumItem = modelBuilder.Entity<CompendiumItem>();
        compendiumItem.ToTable("CompendiumItems");
        compendiumItem.HasKey(ci => ci.Id);
        compendiumItem.Property(ci => ci.Name).IsRequired();
        compendiumItem.Property(ci => ci.Type).IsRequired();
        compendiumItem.Property(ci => ci.System);
        compendiumItem.Property(ci => ci.IsPrimary);
        compendiumItem.HasIndex(ci => new { ci.Name, ci.Type, ci.System });
        compendiumItem.Property(ci => ci.Data)
            .HasConversion(
                d => d.RootElement.GetRawText(),
                s => JsonDocument.Parse(s, new JsonDocumentOptions { AllowTrailingCommas = true })
            );

        // == AssetMapping Configuration ==
        var assetMapping = modelBuilder.Entity<AssetMapping>();
        assetMapping.ToTable("AssetMappings");
        assetMapping.HasKey(am => am.SourceUrl);
    }
}
