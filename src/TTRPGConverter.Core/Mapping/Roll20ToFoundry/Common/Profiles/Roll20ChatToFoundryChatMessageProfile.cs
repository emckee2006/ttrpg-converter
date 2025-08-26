using AutoMapper;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Text.RegularExpressions;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Common.Profiles;

/// <summary>
/// Defines the AutoMapper profile for converting a Roll20 Chat Message to a Foundry ChatMessage.
/// </summary>
public class Roll20ChatToFoundryChatMessageProfile : Profile
{
    public Roll20ChatToFoundryChatMessageProfile()
    {
        // This assumes the source is a Roll20 'message' object.
        CreateMap<Message, ChatMessage>()
            .ForMember(dest => dest.Speaker, opt => opt.MapFrom(src => src.Player))
            .ForMember(dest => dest.Content, opt => opt.MapFrom(src => src.Content))
            .ForMember(dest => dest.Roll, opt => opt.MapFrom(src => ParseRollTemplate(src.Content)));
    }

    private DiceRoll? ParseRollTemplate(string? content)
    {
        if (string.IsNullOrEmpty(content) || !content.Contains("rolltemplate"))
        {
            return null;
        }

        // Basic regex to find a dice formula and a result from a simple roll template.
        // This will need to be expanded to handle more complex templates.
        var formulaMatch = Regex.Match(content, @"roll_dice=([^&]+)");
        var resultMatch = Regex.Match(content, @"roll_total=(\d+)");

        if (formulaMatch.Success && resultMatch.Success)
        {
            return new DiceRoll
            {
                Formula = formulaMatch.Groups[1].Value,
                Result = resultMatch.Groups[1].Value
            };
        }

        return null;
    }
}
