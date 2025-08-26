using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Card to a Foundry Card.
/// </summary>
public class Roll20CardToFoundryCardProfile : Profile
{
    public Roll20CardToFoundryCardProfile()
    {
        CreateMap<Card, Cards>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            // The Faces property will require a custom resolver to map the avatar.
            .ForMember(dest => dest.Faces, opt => opt.Ignore());
    }
}
