using AutoMapper;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Collections.Generic;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 light object to a Foundry light source.
/// </summary>
public class Roll20LightToFoundryLightProfile : Profile
{
    public Roll20LightToFoundryLightProfile()
    {
        // This assumes the source is a Dictionary<string, object> representing a single light.
        CreateMap<Dictionary<string, object>, CommonLightSource>()
            .ForMember(dest => dest.Bright, opt => opt.MapFrom(src => src.GetValueOrDefault("light_radius", 0.0)))
            .ForMember(dest => dest.Dim, opt => opt.MapFrom(src => src.GetValueOrDefault("light_dimradius", 0.0)))
            .ForMember(dest => dest.Angle, opt => opt.MapFrom(src => src.GetValueOrDefault("light_angle", 360.0)))
            .ForMember(dest => dest.Color, opt => opt.MapFrom(src => src.GetValueOrDefault("light_color", "#FFFFFF").ToString()));
    }
}
