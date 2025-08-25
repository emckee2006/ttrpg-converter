using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System;

namespace TTRPGConverter.Core.Mapping.Resolvers;

/// <summary>
/// Resolves the path for a JournalEntry's main image from a Roll20 Handout's avatar.
/// In the future, this will use the IAssetMapper service.
/// </summary>
public class HandoutImageResolver : IValueResolver<Handout, CoreJournal, Uri?>
{
    public Uri? Resolve(Handout source, CoreJournal destination, Uri? destMember, ResolutionContext context)
    {
        if (!string.IsNullOrEmpty(source.Avatar) && Uri.TryCreate(source.Avatar, UriKind.Absolute, out var uri))
        {
            // In a future step, this will be passed to the IAssetMapper to get the final local path.
            return uri;
        }
        return null;
    }
}
