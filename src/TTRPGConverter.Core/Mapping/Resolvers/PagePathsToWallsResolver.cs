using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Collections.Generic;

namespace TTRPGConverter.Core.Mapping.Resolvers;

/// <summary>
/// A custom AutoMapper resolver to convert Roll20 Paths to Foundry Walls.
/// </summary>
public class PagePathsToWallsResolver : IValueResolver<Page, CoreScene, IList<Wall>>
{
    public IList<Wall> Resolve(Page source, CoreScene destination, IList<Wall> destMember, ResolutionContext context)
    {
        var walls = new List<Wall>();
        if (source.Paths == null) return walls;

        foreach (var path in source.Paths)
        {
            // Add logic here to determine if a path is a wall and map it.
        }

        return walls;
    }
}
