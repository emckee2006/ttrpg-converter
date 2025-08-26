using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Resolvers;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Page to a Foundry Scene.
/// </summary>
public class PageToSceneProfile : Profile
{
    public PageToSceneProfile()
    {
        CreateMap<Page, CoreScene>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            // Convert Roll20 grid units to Foundry pixels (assuming 70 pixels per unit).
            .ForMember(dest => dest.Width, opt => opt.MapFrom(src => src.Width * 70))
            .ForMember(dest => dest.Height, opt => opt.MapFrom(src => src.Height * 70))
            .ForMember(dest => dest.Grid, opt => opt.MapFrom(src => new Grid
            {
                Distance = src.ScaleNumber,
                Units = src.ScaleUnits,
                Color = src.Gridcolor,
                Alpha = src.GridAlpha,
                Size = 70 // Default grid size
            }))
            .ForMember(dest => dest.Img, opt => opt.MapFrom<SceneBackgroundResolver>())
            .ForMember(dest => dest.BackgroundColor, opt => opt.MapFrom(src => src.Background_color))
            .ForMember(dest => dest.GlobalLight, opt => opt.MapFrom(src => src.Lightglobalillum))
            .ForMember(dest => dest.Darkness, opt => opt.MapFrom(src => src.Fog_opacity))
            .ForMember(dest => dest.Tokens, opt => opt.MapFrom(src => src.Graphics))
            .ForMember(dest => dest.Walls, opt => opt.MapFrom(src => src.Paths));
    }
}
