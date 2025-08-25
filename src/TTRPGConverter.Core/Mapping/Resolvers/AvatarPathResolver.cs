using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System;

namespace TTRPGConverter.Core.Mapping.Resolvers;

/// <summary>
/// Resolves the path for an Actor's image (avatar).
/// In the future, this will use the IAssetMapper service.
/// </summary>
public class AvatarPathResolver : IValueResolver<Character, Dnd5eActor, Uri?>
{
    public Uri? Resolve(Character source, Dnd5eActor destination, Uri? destMember, ResolutionContext context)
    {
        // source.Avatar is already a Uri?, so we can return it directly.
        // In a future step, this will be passed to the IAssetMapper to get the final local path.
        return source.Avatar;
    }
}
