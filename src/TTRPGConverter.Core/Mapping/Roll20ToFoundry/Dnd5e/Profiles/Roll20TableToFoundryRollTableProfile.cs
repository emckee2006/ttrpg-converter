using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Table to a Foundry RollableTable.
/// </summary>
public class Roll20TableToFoundryRollTableProfile : Profile
{
    public Roll20TableToFoundryRollTableProfile()
    {
        CreateMap<Table, CoreRollableTable>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => src.Name))
            .ForMember(dest => dest.Description, opt => opt.MapFrom(src => "")) // Roll20 Tables don't have a description field.
            .ForMember(dest => dest.Results, opt => opt.MapFrom(src => src.Tableitems));
    }
}
