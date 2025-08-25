using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Deck to a Foundry Cards document.
/// </summary>
public class Roll20DeckToFoundryCardsProfile : Profile
{
    public Roll20DeckToFoundryCardsProfile()
    {
        CreateMap<Deck, CoreCards>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Description, opt => opt.MapFrom(src => "")) // Roll20 Decks don't have a description.
            .ForMember(dest => dest.Cards, opt => opt.MapFrom(src => src.Cards));
    }
}
