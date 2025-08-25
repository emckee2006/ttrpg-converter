using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 AudioTrack to a Foundry PlaylistSound.
/// </summary>
public class Roll20AudioTrackToFoundryPlaylistSoundProfile : Profile
{
    public Roll20AudioTrackToFoundryPlaylistSoundProfile()
    {
        CreateMap<AudioTrack, Sounds>()
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Title))
            .ForMember(dest => dest.Repeat, opt => opt.MapFrom(src => src.Loop))
            .ForMember(dest => dest.Volume, opt => opt.MapFrom(src => src.Volume))
            // The Path will be handled by a dedicated AssetResolver in the future.
            .ForMember(dest => dest.Path, opt => opt.Ignore());
    }
}
