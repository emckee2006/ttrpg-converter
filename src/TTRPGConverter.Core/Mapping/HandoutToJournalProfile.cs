using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Mapping.Resolvers;
using System.Collections.Generic;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Handout to a Foundry JournalEntry.
/// </summary>
public class HandoutToJournalProfile : Profile
{
    public HandoutToJournalProfile()
    {
        CreateMap<Handout, CoreJournal>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Pages, opt => opt.MapFrom<JournalPagesResolver>())
            .ForMember(dest => dest.Img, opt => opt.MapFrom<HandoutImageResolver>());
    }
}
