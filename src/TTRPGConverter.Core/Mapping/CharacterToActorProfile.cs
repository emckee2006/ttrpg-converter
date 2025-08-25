using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Mapping.Resolvers;
using System.Linq;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20Character to a Foundry Dnd5eActor.
/// </summary>
public class CharacterToActorProfile : Profile
{
    public CharacterToActorProfile()
    {
        CreateMap<Character, Dnd5eActor>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Img, opt => opt.MapFrom<AvatarPathResolver>())
            .ForMember(dest => dest.System, opt => opt.MapFrom<CharacterDataResolver>())
            .ForMember(dest => dest.Type, opt => opt.MapFrom(src => 
                (src.Attribs != null && src.Attribs.Any(a => a.Name != null && a.Name.StartsWith("npc_"))) 
                ? Dnd5eactortype.Npc 
                : Dnd5eactortype.Character));
    }
}
