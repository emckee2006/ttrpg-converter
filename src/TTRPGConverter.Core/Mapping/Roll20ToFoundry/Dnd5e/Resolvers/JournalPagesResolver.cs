using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Collections.Generic;
using FoundryText = TTRPGConverter.Core.Models.Foundry.Dnd5e.Text;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Resolvers;

/// <summary>
/// A custom AutoMapper resolver to convert Roll20 Handout notes and gmnotes
/// into a list of Foundry JournalEntry pages.
/// </summary>
public class JournalPagesResolver : IValueResolver<Handout, CoreJournal, IList<CoreJournalPage>?>
{
    public IList<CoreJournalPage>? Resolve(Handout source, CoreJournal destination, IList<CoreJournalPage>? destMember, ResolutionContext context)
    {
        var pages = new List<CoreJournalPage>();

        // Create a page for the public notes if they exist.
        if (!string.IsNullOrEmpty(source.Notes))
        {
            pages.Add(new CoreJournalPage
            {
                Name = source.Name ?? "Page 1",
                Type = Corejournalpagetype.Text,
                Text = new FoundryText { Content = source.Notes, Format = Textformat._1 },
                Ownership = new CommonOwnership { Default = Commonownershipdefault._3 } // Default Owner
            });
        }

        // Create a separate, GM-only page for the GM notes if they exist.
        if (!string.IsNullOrEmpty(source.Gmnotes))
        {
            pages.Add(new CoreJournalPage
            {
                Name = "GM Notes",
                Type = Corejournalpagetype.Text,
                Text = new FoundryText { Content = source.Gmnotes, Format = Textformat._1 },
                Ownership = new CommonOwnership { Default = Commonownershipdefault._2 } // Default Observer, GM is Owner
            });
        }

        return pages.Count > 0 ? pages : null;
    }
}
