using AutoMapper;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Mappers;

namespace TTRPGConverter.Core.Mapping.Roll20ToFoundry.Dnd5e.Resolvers;

/// <summary>
/// A custom AutoMapper resolver to handle the complex mapping of a Roll20Character's
/// attributes and bio to the Foundry Dnd5eActor's system data property.
/// </summary>
public class CharacterDataResolver : IValueResolver<Character, Dnd5eActor, SystemModel>
{
    private readonly Roll20ToFoundryMapper _roll20Mapper;

    public CharacterDataResolver(Roll20ToFoundryMapper roll20Mapper)
    {
        _roll20Mapper = roll20Mapper;
    }

    public SystemModel Resolve(Character source, Dnd5eActor destination, SystemModel destMember, ResolutionContext context)
    {
        // Delegate the entire complex conversion to the dedicated mapper service.
        return _roll20Mapper.CreateDnd5ESystemData(source);
    }
}
