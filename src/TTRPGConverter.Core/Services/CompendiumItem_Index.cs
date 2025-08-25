using Raven.Client.Documents.Indexes;
using System.Linq;
using TTRPGConverter.Core.Entities;

namespace TTRPGConverter.Core.Services;

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
