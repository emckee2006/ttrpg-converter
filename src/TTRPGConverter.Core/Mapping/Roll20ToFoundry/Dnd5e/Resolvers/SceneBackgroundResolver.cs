using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Resolvers;

/// <summary>
/// Resolves the path for a Scene's background image from a Roll20 Page's background_image.
/// In the future, this will use the IAssetMapper service.
/// </summary>
public class SceneBackgroundResolver : IValueResolver<Page, CoreScene, Uri?>
{
    public Uri? Resolve(Page source, CoreScene destination, Uri? destMember, ResolutionContext context)
    {
        // The Roll20 export does not seem to contain the background image URL directly in the Page object.
        // This logic will need to be updated once the source of the background image is identified.
        // For now, we return null.
        return null;
    }
}
