using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 TableItem to a Foundry RollableTable Result.
/// </summary>
public class Roll20TableItemToFoundryResultProfile : Profile
{
    public Roll20TableItemToFoundryResultProfile()
    {
        CreateMap<TableItem, Results>()
            .ForMember(dest => dest.Text, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Weight, opt => opt.MapFrom(src => src.Weight))
            .ForMember(dest => dest.Img, opt => opt.MapFrom(src => src.Avatar));
    }
}
