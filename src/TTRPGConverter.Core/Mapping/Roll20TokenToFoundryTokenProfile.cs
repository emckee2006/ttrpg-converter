using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Graphic (token) to a Foundry Token.
/// </summary>
public class Roll20TokenToFoundryTokenProfile : Profile
{
    public Roll20TokenToFoundryTokenProfile()
    {
        CreateMap<Graphic, Token>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.X, opt => opt.MapFrom(src => src.Left))
            .ForMember(dest => dest.Y, opt => opt.MapFrom(src => src.Top))
            .ForMember(dest => dest.Width, opt => opt.MapFrom(src => src.Width))
            .ForMember(dest => dest.Height, opt => opt.MapFrom(src => src.Height))
            .ForMember(dest => dest.Rotation, opt => opt.MapFrom(src => src.Rotation))
            .ForMember(dest => dest.ActorId, opt => opt.MapFrom(src => src.Represents))
            .ForMember(dest => dest.Vision, opt => opt.MapFrom(src => src.LightHasSight))
            .ForMember(dest => dest.SightAngle, opt => opt.MapFrom(src => src.LightLosAngle))
            .ForMember(dest => dest.Light, opt => opt.MapFrom(src => new Light
            {
                Bright = src.LightRadius,
                Dim = src.LightDimradius,
                Angle = src.LightAngle,
                Color = src.LightColor
            }))
            .ForMember(dest => dest.Img, opt => opt.MapFrom(src => CreateUriFromString(src.Imgsrc)));
    }

    private Uri? CreateUriFromString(string? url)
    {
        if (!string.IsNullOrEmpty(url) && Uri.TryCreate(url, UriKind.Absolute, out var uri))
        {
            return uri;
        }
        return null;
    }
}
