using Raven.Client.Documents.Indexes;

namespace TTRPGConverter.Infrastructure.Services.Compendium;

public class CompendiumItem_Index : AbstractIndexCreationTask<CompendiumItem>
{
    public CompendiumItem_Index()
    {
        Map = items => from item in items
                       select new
                       {
                           item.Type,
                           item.Name
                       };
    }
}
