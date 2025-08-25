using AutoMapper;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Services;
using System.Collections.Generic;

namespace TTRPGConverter.Core.Mapping.Resolvers;

/// <summary>
/// A custom AutoMapper resolver to handle the creation of Foundry Items from a Roll20Character.
/// </summary>
public class CharacterItemsResolver : IValueResolver<Character, Dnd5eActor, IList<Dnd5eItem>>
{
    private readonly Roll20ToFoundryMapper _roll20Mapper;

    public CharacterItemsResolver(Roll20ToFoundryMapper roll20Mapper)
    {
        _roll20Mapper = roll20Mapper;
    }

    public IList<Dnd5eItem> Resolve(Character source, Dnd5eActor destination, IList<Dnd5eItem> destMember, ResolutionContext context)
    {
        // Delegate the item creation to the dedicated mapper service.
        return _roll20Mapper.CreateItems(source.Attribs);
    }
}
