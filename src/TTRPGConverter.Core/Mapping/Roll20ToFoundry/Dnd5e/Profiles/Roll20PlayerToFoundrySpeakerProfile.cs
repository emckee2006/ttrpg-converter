using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Player to a Foundry ChatMessage Speaker.
/// </summary>
public class Roll20PlayerToFoundrySpeakerProfile : Profile
{
    public Roll20PlayerToFoundrySpeakerProfile()
    {
        // This is a lightweight mapping to an intermediate object, not a full Foundry User.
        CreateMap<Player, Speaker>()
            .ForMember(dest => dest.Actor, opt => opt.MapFrom(src => src.Id)) // Using Player ID as a stand-in for Actor ID
            .ForMember(dest => dest.Alias, opt => opt.MapFrom(src => src.Displayname));
    }
}
