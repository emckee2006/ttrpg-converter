using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Macro to a Foundry Macro.
/// </summary>
public class Roll20MacroToFoundryMacroProfile : Profile
{
    public Roll20MacroToFoundryMacroProfile()
    {
        CreateMap<Macro, CoreMacro>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Command, opt => opt.MapFrom(src => src.Action))
            .ForMember(dest => dest.Type, opt => opt.MapFrom(src => Coremacrotype.Chat)) // Default to chat macro
            .ForMember(dest => dest.Scope, opt => opt.MapFrom(src => 
                src.Istokenaction == true ? Coremacroscope.Actor : Coremacroscope.Global));
    }
}
