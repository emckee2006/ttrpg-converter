using AutoMapper;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using System.Collections.Generic;
using FoundryRange = TTRPGConverter.Core.Models.Foundry.Dnd5e.Range;

namespace TTRPGConverter.Core.Mapping;

/// <summary>
/// Defines the AutoMapper profile for converting data from a Roll20 repeating section
/// into a Foundry Dnd5eItem.
/// </summary>
public class Roll20ItemProfile : Profile
{
    public Roll20ItemProfile()
    {
        CreateMap<Dictionary<string, string>, Dnd5eItem>()
            .ForMember(dest => dest.Name, opt => opt.MapFrom(src => 
                src.GetValueOrDefault("itemname", 
                src.GetValueOrDefault("atkname", 
                src.GetValueOrDefault("spellname", "Unknown Item")))))

            .ForMember(dest => dest.System, opt => opt.MapFrom(src => MapItemSystem(src)))

            .AfterMap((src, dest) => 
            {
                if (src.ContainsKey("atkname")) { dest.Type = Dnd5eitemtype.Weapon; }
                else if (src.ContainsKey("spellname")) { dest.Type = Dnd5eitemtype.Spell; }
                else { dest.Type = Dnd5eitemtype.Loot; }
            });
    }

    private SystemModel1 MapItemSystem(Dictionary<string, string> src)
    {
        var system = new SystemModel1
        {
            Description = new Description { Value = src.GetValueOrDefault("itemcontent", src.GetValueOrDefault("spelldescription", "")) },
            Quantity = double.TryParse(src.GetValueOrDefault("itemcount", "1"), out var qty) ? qty : 1,
            Weight = double.TryParse(src.GetValueOrDefault("itemweight", "0"), out var wt) ? wt : 0
        };

        if (src.ContainsKey("atkname")) // Weapon
        {
            system.Activation = new Activation { Type = Activationtype.Action, Cost = 1 };
            var damageParts = new List<IList<string>>();
            var damageFormula = src.GetValueOrDefault("dmgbase", "");
            if (!string.IsNullOrEmpty(damageFormula)) { damageParts.Add(new List<string> { damageFormula, src.GetValueOrDefault("dmgtype", "") }); }
            system.Damage = new Damage { Parts = damageParts };
            system.Range = new FoundryRange { Value = double.TryParse(src.GetValueOrDefault("atkrange", "5"), out var r) ? r : 5, Units = "ft" };
        }
        else if (src.ContainsKey("spellname")) // Spell
        {
            system.Level = int.TryParse(src.GetValueOrDefault("spelllevel", "0"), out var lvl) ? lvl : 0;
            system.School = src.GetValueOrDefault("school", "");
            system.Activation = new Activation { Type = GetActivationType(src.GetValueOrDefault("castingtime", "")) };
            system.Duration = new Duration { Value = src.GetValueOrDefault("duration", "") };
            system.Target = new Target { Value = double.TryParse(src.GetValueOrDefault("target", ""), out var t) ? t : 0 };
            system.Range = new FoundryRange { Value = double.TryParse(src.GetValueOrDefault("range", ""), out var r) ? r : 0 };
        }

        return system;
    }

    private Activationtype GetActivationType(string castingTime)
    {
        return castingTime.ToLower() switch
        {
            "1 action" => Activationtype.Action,
            "1 bonus action" => Activationtype.Bonus,
            "1 reaction" => Activationtype.Reaction,
            "1 minute" => Activationtype.Minute,
            "1 hour" => Activationtype.Hour,
            _ => Activationtype.Action
        };
    }
}
