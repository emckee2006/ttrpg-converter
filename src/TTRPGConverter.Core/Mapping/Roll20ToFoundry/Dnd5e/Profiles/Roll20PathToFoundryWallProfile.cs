using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Collections.Generic;
using System.Linq;
using Roll20Path = TTRPGConverter.Core.Models.Roll20.Path;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Path to a Foundry Wall.
/// </summary>
public class Roll20PathToFoundryWallProfile : Profile
{
    public Roll20PathToFoundryWallProfile()
    {
        CreateMap<Roll20Path, Wall>()
            .ForMember(dest => dest._id, opt => opt.MapFrom(src => src.Id))
            // The core of the conversion is mapping the Roll20 path data to Foundry's coordinate array.
            // Roll20 path data is a string like "M 10 10 L 20 20". We need to parse it.
            .ForMember(dest => dest.C, opt => opt.MapFrom(src => ParsePathCoordinates(src.PathData)));
    }

    private IList<double> ParsePathCoordinates(string? pathData)
    {
        var coordinates = new List<double>();
        if (string.IsNullOrEmpty(pathData)) return coordinates;

        var parts = pathData.Split(' ');
        foreach (var part in parts)
        {
            if (double.TryParse(part, out var num))
            {
                coordinates.Add(num);
            }
        }
        return coordinates;
    }
}
