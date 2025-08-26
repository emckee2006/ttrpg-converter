using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using TTRPGConverter.Core.Models.Roll20;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;
using AutoMapper;

namespace TTRPGConverter.Core.Mappers;

public class Roll20ToFoundryMapper
{
    private readonly IMapper _mapper;

    public Roll20ToFoundryMapper(IMapper mapper)
    {
        _mapper = mapper;
    }

    public SystemModel CreateDnd5ESystemData(Character roll20Character)
    {
        var attributes = ParseRoll20Attributes(roll20Character.Attribs);
        var isNpc = IsNpc(roll20Character);

        return new SystemModel
        {
            Abilities = CreateAbilities(attributes),
            Attributes = CreateAttributes(attributes, isNpc),
            Details = CreateDetails(roll20Character, attributes),
            Skills = CreateSkills(attributes),
            Currency = CreateCurrency(attributes),
            Spells = CreateSpells(attributes),
            Traits = CreateTraits(attributes),
            Bonuses = CreateBonuses(attributes),
            Resources = CreateResources(attributes)
        };
    }

    public List<Dnd5eItem> CreateItems(IList<TTRPGConverter.Core.Models.Roll20.Attribute>? attribs)
    {
        var inventory = GroupRepeatingAttributes("repeating_inventory", attribs);
        var attacks = GroupRepeatingAttributes("repeating_attack", attribs);

        var allItems = new List<Dnd5eItem>();

        foreach (var itemData in inventory.Values)
        {
            allItems.Add(_mapper.Map<Dnd5eItem>(itemData));
        }

        foreach (var attackData in attacks.Values)
        {
            allItems.Add(_mapper.Map<Dnd5eItem>(attackData));
        }

        for (int i = 0; i <= 9; i++)
        {
            var spellLevelKey = i == 0 ? "cantrip" : $"spell-{i}";
            var spells = GroupRepeatingAttributes($"repeating_{spellLevelKey}", attribs);
            foreach (var spellData in spells.Values)
            {
                allItems.Add(_mapper.Map<Dnd5eItem>(spellData));
            }
        }

        return allItems;
    }

    private Dictionary<string, Dictionary<string, string>> GroupRepeatingAttributes(string sectionPrefix, IList<TTRPGConverter.Core.Models.Roll20.Attribute>? attribs)
    {
        var repeatingSection = new Dictionary<string, Dictionary<string, string>>();
        if (attribs == null) return repeatingSection;

        string pattern = $@"^{sectionPrefix}_([^_]+)_(.+)s*$";

        var sectionAttribs = attribs.Where(a => a.Name != null && a.Name.StartsWith(sectionPrefix));

        foreach (var attr in sectionAttribs)
        {
            var match = Regex.Match(attr.Name!, pattern);
            if (match.Success)
            {
                string rowId = match.Groups[1].Value;
                string attrName = match.Groups[2].Value;

                if (!repeatingSection.ContainsKey(rowId))
                {
                    repeatingSection[rowId] = new Dictionary<string, string>();
                }
                repeatingSection[rowId][attrName] = attr.Current ?? "";
            }
        }

        return repeatingSection;
    }

    private Dictionary<string, (string current, string max, string id)> ParseRoll20Attributes(IList<TTRPGConverter.Core.Models.Roll20.Attribute>? attribs)
    {
        var result = new Dictionary<string, (string, string, string)>();
        if (attribs == null) return result;
        foreach (var attr in attribs)
        {
            if (!string.IsNullOrEmpty(attr.Name))
            {
                result[attr.Name.ToLower()] = (attr.Current ?? "0", attr.Max ?? "0", attr.Id ?? "");
            }
        }
        return result;
    }

    private bool IsNpc(Character character)
    {
        if (character.Attribs == null) return false;
        return character.Attribs.Any(a => a.Name?.StartsWith("npc_") == true);
    }

    private Abilities CreateAbilities(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Abilities
        {
            Str = CreateAbility("strength", attributes),
            Dex = CreateAbility("dexterity", attributes),
            Con = CreateAbility("constitution", attributes),
            Int = CreateAbility("intelligence", attributes),
            Wis = CreateAbility("wisdom", attributes),
            Cha = CreateAbility("charisma", attributes)
        };
    }

    private CommonAbilityScore CreateAbility(string abilityName, Dictionary<string, (string current, string max, string id)> attributes)
    {
        var score = GetAttributeInt(abilityName, 10, attributes);
        var proficient = GetAttributeInt($"{abilityName}_save_prof", 0, attributes) > 0 ? 1 : 0;
        return new CommonAbilityScore
        {
            Value = score,
            Proficient = proficient
        };
    }

    private Attributes CreateAttributes(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        return new Attributes
        {
            Ac = new Ac { Value = GetAttributeInt(isNpc ? "npc_ac" : "ac", 10, attributes) },
            Hp = new CommonResource { Value = GetAttributeInt("hp", 10, attributes), Max = GetAttributeInt("hp_max", 10, attributes) },
            Init = new Init { Bonus = GetAttributeInt("initiative_bonus", 0, attributes) },
            Speed = new Speed { Value = GetAttributeInt(isNpc ? "npc_speed" : "speed", 30, attributes), Special = GetAttribute("speed_special", "", attributes).current }
        };
    }

    private Details CreateDetails(Character character, Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Details
        {
            Biography = new Biography { Value = character.Bio ?? "", Public = character.Bio ?? "" },
            Alignment = GetAttribute("alignment", "", attributes).current,
            Race = GetAttribute("race", "", attributes).current
        };
    }

    private Skills CreateSkills(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Skills
        {
            Acr = CreateSkill("acrobatics", attributes),
            Ani = CreateSkill("animal_handling", attributes),
            Arc = CreateSkill("arcana", attributes),
            Ath = CreateSkill("athletics", attributes),
            Dec = CreateSkill("deception", attributes),
            His = CreateSkill("history", attributes),
            Ins = CreateSkill("insight", attributes),
            Itm = CreateSkill("intimidation", attributes),
            Inv = CreateSkill("investigation", attributes),
            Med = CreateSkill("medicine", attributes),
            Nat = CreateSkill("nature", attributes),
            Prc = CreateSkill("perception", attributes),
            Prf = CreateSkill("performance", attributes),
            Per = CreateSkill("persuasion", attributes),
            Rel = CreateSkill("religion", attributes),
            Slt = CreateSkill("sleight_of_hand", attributes),
            Ste = CreateSkill("stealth", attributes),
            Sur = CreateSkill("survival", attributes)
        };
    }

    private Dnd5eSkill CreateSkill(string skillName, Dictionary<string, (string current, string max, string id)> attributes)
    {
        var proficient = GetAttributeInt($"{skillName}_prof", 0, attributes);
        var bonus = GetAttributeInt($"{skillName}_bonus", 0, attributes);
        return new Dnd5eSkill
        {
            Value = proficient,
            Proficient = proficient,
            Bonuses = new SkillBonuses { Check = bonus > 0 ? bonus.ToString() : "" }
        };
    }

    private CommonCurrency CreateCurrency(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new CommonCurrency
        {
            Cp = GetAttributeInt("cp", 0, attributes),
            Sp = GetAttributeInt("sp", 0, attributes),
            Ep = GetAttributeInt("ep", 0, attributes),
            Gp = GetAttributeInt("gp", 0, attributes),
            Pp = GetAttributeInt("pp", 0, attributes)
        };
    }

    private Spells CreateSpells(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Spells
        {
            Spell1 = CreateSpellSlot(1, attributes),
            Spell2 = CreateSpellSlot(2, attributes),
            Spell3 = CreateSpellSlot(3, attributes),
            Spell4 = CreateSpellSlot(4, attributes),
            Spell5 = CreateSpellSlot(5, attributes),
            Spell6 = CreateSpellSlot(6, attributes),
            Spell7 = CreateSpellSlot(7, attributes),
            Spell8 = CreateSpellSlot(8, attributes),
            Spell9 = CreateSpellSlot(9, attributes),
            Pact = CreateSpellSlot(0, attributes, "pact")
        };
    }

    private Dnd5eSpellSlot CreateSpellSlot(int level, Dictionary<string, (string current, string max, string id)> attributes, string prefix = "lvl")
    {
        var key = level > 0 ? $"{prefix}{level}" : prefix;
        var max = GetAttributeInt($"{key}_spell_slots_total", 0, attributes);
        var expended = GetAttributeInt($"{key}_spell_slots_expended", 0, attributes);
        return new Dnd5eSpellSlot
        {
            Max = max,
            Value = max - expended
        };
    }

    private Traits CreateTraits(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Traits
        {
            Di = new Trait { Value = GetAttribute("npc_damage_immunities", "", attributes).current.Split(',').ToList() },
            Dr = new Trait { Value = GetAttribute("npc_damage_resistances", "", attributes).current.Split(',').ToList() },
            Dv = new Trait { Value = GetAttribute("npc_damage_vulnerabilities", "", attributes).current.Split(',').ToList() },
            Ci = new Trait { Value = GetAttribute("npc_condition_immunities", "", attributes).current.Split(',').ToList() },
            Languages = new Trait { Value = GetAttribute("languages", "", attributes).current.Split(',').ToList() }
        };
    }

    private GlobalBonuses CreateBonuses(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new GlobalBonuses
        {
            Mwak = new AttackBonus { Attack = GetAttribute("global_melee_attack_bonus", "", attributes).current, Damage = GetAttribute("global_melee_damage_bonus", "", attributes).current },
            Rwak = new AttackBonus { Attack = GetAttribute("global_ranged_attack_bonus", "", attributes).current, Damage = GetAttribute("global_ranged_damage_bonus", "", attributes).current },
            Msak = new AttackBonus { Attack = GetAttribute("global_spell_attack_bonus", "", attributes).current, Damage = GetAttribute("global_spell_damage_bonus", "", attributes).current },
            Rsak = new AttackBonus { Attack = GetAttribute("global_spell_attack_bonus", "", attributes).current, Damage = GetAttribute("global_spell_damage_bonus", "", attributes).current }
        };
    }

    private Resources CreateResources(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new Resources
        {
            Primary = new CommonResource { Value = GetAttributeInt("resource_1", 0, attributes), Max = GetAttributeInt("resource_1_max", 0, attributes) },
            Secondary = new CommonResource { Value = GetAttributeInt("resource_2", 0, attributes), Max = GetAttributeInt("resource_2_max", 0, attributes) },
            Tertiary = new CommonResource { Value = GetAttributeInt("resource_3", 0, attributes), Max = GetAttributeInt("resource_3_max", 0, attributes) }
        };
    }

    private (string current, string max, string id) GetAttribute(string key, string defaultValue, Dictionary<string, (string current, string max, string id)> attributes) => attributes.TryGetValue(key.ToLower(), out var value) ? value : (defaultValue, defaultValue, "");
    private int GetAttributeInt(string key, int defaultValue, Dictionary<string, (string current, string max, string id)> attributes) => int.TryParse(GetAttribute(key, defaultValue.ToString(), attributes).current, out var result) ? result : defaultValue;
}
