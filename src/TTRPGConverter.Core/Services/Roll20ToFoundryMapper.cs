using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using TTRPGConverter.Core.Models.Roll20;
using TTRPGConverter.Core.Models.Foundry.Dnd5e;
using Microsoft.Extensions.Logging;

namespace TTRPGConverter.Core.Services;

/// <summary>
/// Maps Roll20 character data to Foundry D&D 5E system format
/// Based on R20Converter Python patterns
/// </summary>
public class Roll20ToFoundryMapper
{
    private readonly ILogger<Roll20ToFoundryMapper> _logger;

    public Roll20ToFoundryMapper(ILogger<Roll20ToFoundryMapper> logger)
    {
        _logger = logger;
    }

    /// <summary>
    /// Create complete Foundry D&D 5E system data from Roll20 character
    /// </summary>
    public dynamic CreateDnd5eSystemData(Character roll20Character)
    {
        var attributes = ParseRoll20Attributes(roll20Character.Attribs);
        var isNpc = IsNPC(roll20Character);

        return new
        {
            abilities = CreateAbilities(attributes),
            attributes = CreateAttributes(attributes, isNpc),
            details = CreateDetails(roll20Character, attributes, isNpc),
            skills = CreateSkills(attributes),
            traits = CreateTraits(attributes, isNpc),
            currency = CreateCurrency(attributes),
            spells = CreateSpells(attributes),
            resources = CreateResources(attributes),
            bonuses = CreateBonuses(attributes)
        };
    }

    /// <summary>
    /// Parse Roll20 attributes into lookup dictionary
    /// </summary>
    private Dictionary<string, (string current, string max, string id)> ParseRoll20Attributes(IList<TTRPGConverter.Core.Models.Roll20.Attribute>? attribs)
    {
        var result = new Dictionary<string, (string, string, string)>();
        
        if (attribs == null) return result;

        foreach (var attr in attribs)
        {
            if (!string.IsNullOrEmpty(attr.Name))
            {
                result[attr.Name.ToLower()] = (
                    attr.Current?.ToString() ?? "0",
                    attr.Max?.ToString() ?? "0", 
                    attr.Id ?? ""
                );
            }
        }
        
        return result;
    }

    /// <summary>
    /// Check if character is NPC based on Roll20 patterns
    /// </summary>
    private bool IsNPC(Character character)
    {
        // NPCs typically have npc_ prefixed attributes
        return character.Attribs?.Any(a => a.Name?.StartsWith("npc_") == true) == true;
    }

    /// <summary>
    /// Create D&D 5E ability scores from Roll20 data
    /// </summary>
    private dynamic CreateAbilities(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new
        {
            str = CreateAbility("strength", attributes),
            dex = CreateAbility("dexterity", attributes),
            con = CreateAbility("constitution", attributes),
            @int = CreateAbility("intelligence", attributes),
            wis = CreateAbility("wisdom", attributes),
            cha = CreateAbility("charisma", attributes)
        };
    }

    /// <summary>
    /// Create single ability score with modifier
    /// </summary>
    private dynamic CreateAbility(string abilityName, Dictionary<string, (string current, string max, string id)> attributes)
    {
        var score = GetAttributeInt(abilityName, 10, attributes);
        var modifier = (int)Math.Floor((score - 10) / 2.0);
        var proficient = GetAttributeInt($"{abilityName}_save_prof", 0, attributes);
        var saveBonus = GetAttributeInt($"{abilityName}_save_bonus", 0, attributes);

        return new
        {
            value = score,
            proficient = proficient > 0 ? 1 : 0,
            bonuses = new
            {
                check = "",
                save = saveBonus > 0 ? saveBonus.ToString() : ""
            },
            mod = modifier,
            save = modifier + (proficient > 0 ? GetProficiencyBonus(attributes) : 0) + saveBonus,
            checkBonus = 0,
            saveBonus = saveBonus,
            dc = 8 + modifier + GetProficiencyBonus(attributes)
        };
    }

    /// <summary>
    /// Create D&D 5E attributes (HP, AC, etc.)
    /// </summary>
    private dynamic CreateAttributes(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        var result = new
        {
            ac = CreateArmorClass(attributes, isNpc),
            hp = CreateHitPoints(attributes, isNpc),
            init = CreateInitiative(attributes),
            prof = GetProficiencyBonus(attributes),
            movement = CreateMovement(attributes, isNpc),
            senses = CreateSenses(attributes),
            spellcasting = GetSpellcastingAbility(attributes),
            spelldc = GetAttributeInt(isNpc ? "npc_spelldc" : "spell_save_dc", 10, attributes),
            spellLevel = 0
        };

        return result;
    }

    /// <summary>
    /// Create armor class data
    /// </summary>
    private dynamic CreateArmorClass(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        var ac = GetAttributeInt(isNpc ? "npc_ac" : "ac", 10, attributes);
        
        return new
        {
            flat = ac,
            calc = "flat",
            formula = ""
        };
    }

    /// <summary>
    /// Create hit points data  
    /// </summary>
    private dynamic CreateHitPoints(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        int value, max;
        string formula = "";

        if (isNpc)
        {
            if (attributes.ContainsKey("hp"))
            {
                value = GetAttributeInt("hp", 10, attributes);
                max = value;
            }
            else
            {
                var hpBase = GetAttribute("npc_hpbase", "10", attributes);
                var parts = hpBase.current.Split(' ');
                value = max = int.TryParse(parts[0], out var parsed) ? parsed : 10;
            }
            formula = GetAttribute("npc_hpformula", "", attributes).current;
        }
        else
        {
            var hp = GetAttribute("hp", (current: "10", max: "10", id: ""), attributes);
            value = int.TryParse(hp.current, out var curr) ? curr : 10;
            max = int.TryParse(hp.max, out var mx) ? mx : 10;
        }

        return new
        {
            value = value,
            min = 0,
            max = max,
            temp = 0,
            tempmax = 0,
            formula = formula
        };
    }

    /// <summary>
    /// Create initiative data
    /// </summary>
    private dynamic CreateInitiative(Dictionary<string, (string current, string max, string id)> attributes)
    {
        var dexMod = GetAttributeInt("dexterity_mod", 0, attributes);
        var initBonus = GetAttributeInt("initiative_bonus", 0, attributes);
        var jackBonus = GetAttributeInt("jack_bonus", 0, attributes);
        var bonus = initBonus - dexMod - jackBonus;

        return new
        {
            value = bonus,
            bonus = bonus,
            mod = dexMod,
            prof = jackBonus != 0 ? 1 : 0,
            total = initBonus
        };
    }

    /// <summary>
    /// Create movement data
    /// </summary>
    private dynamic CreateMovement(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        var speedAttr = GetAttribute(isNpc ? "npc_speed" : "speed", "30 ft", attributes);
        var speed = speedAttr.current;

        // Parse speed string (e.g. "30 ft, fly 60 ft, swim 30 ft")
        var parts = speed.Replace("ft.", "ft").Split(',');
        var walkSpeed = 30;
        var special = "";

        if (parts.Length > 0)
        {
            var walkPart = parts[0].Trim();
            var match = System.Text.RegularExpressions.Regex.Match(walkPart, @"(\d+)");
            if (match.Success)
            {
                walkSpeed = int.Parse(match.Groups[1].Value);
            }
            
            if (parts.Length > 1)
            {
                special = string.Join(", ", parts.Skip(1)).Trim();
            }
        }

        var movement = new Dictionary<string, object>
        {
            ["walk"] = walkSpeed,
            ["units"] = "ft",
            ["hover"] = special.Contains("hover")
        };

        // Parse special movement types
        foreach (var movementType in new[] { "burrow", "climb", "fly", "swim" })
        {
            movement[movementType] = 0;
            var regex = new System.Text.RegularExpressions.Regex($@"{movementType}\s+(\d+)", System.Text.RegularExpressions.RegexOptions.IgnoreCase);
            var match = regex.Match(special);
            if (match.Success)
            {
                movement[movementType] = int.Parse(match.Groups[1].Value);
            }
        }

        return movement;
    }

    /// <summary>
    /// Create senses data
    /// </summary>
    private dynamic CreateSenses(Dictionary<string, (string current, string max, string id)> attributes)
    {
        return new
        {
            darkvision = GetAttributeInt("npc_darkvision", 0, attributes),
            blindsight = GetAttributeInt("npc_blindsight", 0, attributes),
            tremorsense = GetAttributeInt("npc_tremorsense", 0, attributes),
            truesight = GetAttributeInt("npc_truesight", 0, attributes),
            units = "ft",
            special = GetAttribute("npc_senses", "", attributes).current
        };
    }

    /// <summary>
    /// Create character details
    /// </summary>
    private dynamic CreateDetails(Character character, Dictionary<string, (string current, string max, string id)> attributes, bool isNpc)
    {
        return new
        {
            race = GetAttribute("race", "", attributes).current,
            @class = GetAttribute("class", "", attributes).current,
            level = GetAttributeInt("level", 1, attributes),
            background = GetAttribute("background", "", attributes).current,
            alignment = GetAttribute("alignment", "", attributes).current,
            biography = new
            {
                value = character.Bio ?? "",
                @public = ""
            }
        };
    }

    /// <summary>
    /// Create skills data (simplified)
    /// </summary>
    private dynamic CreateSkills(Dictionary<string, (string current, string max, string id)> attributes)
    {
        // D&D 5E skills mapping - simplified version
        var skills = new Dictionary<string, object>();
        var skillNames = new[]
        {
            "acr", "ani", "arc", "ath", "dec", "his", "ins", "inti", "inv", "med",
            "nat", "prc", "prf", "per", "rel", "slt", "ste", "sur"
        };

        foreach (var skill in skillNames)
        {
            var proficient = GetAttributeInt($"{skill}_prof", 0, attributes);
            var bonus = GetAttributeInt($"{skill}_bonus", 0, attributes);
            
            skills[skill] = new
            {
                value = proficient,
                proficient = proficient > 0 ? 1 : 0,
                bonuses = new { check = bonus > 0 ? bonus.ToString() : "" }
            };
        }

        return skills;
    }

    /// <summary>
    /// Create traits, currency, spells, resources, bonuses (simplified)
    /// </summary>
    private dynamic CreateTraits(Dictionary<string, (string current, string max, string id)> attributes, bool isNpc) => new { };
    private dynamic CreateCurrency(Dictionary<string, (string current, string max, string id)> attributes) => new { cp = 0, sp = 0, ep = 0, gp = 0, pp = 0 };
    private dynamic CreateSpells(Dictionary<string, (string current, string max, string id)> attributes) => new { };
    private dynamic CreateResources(Dictionary<string, (string current, string max, string id)> attributes) => new { };
    private dynamic CreateBonuses(Dictionary<string, (string current, string max, string id)> attributes) => new { };

    /// <summary>
    /// Helper methods
    /// </summary>
    private (string current, string max, string id) GetAttribute(string key, string defaultValue, Dictionary<string, (string current, string max, string id)> attributes)
    {
        return GetAttribute(key, (current: defaultValue, max: defaultValue, id: ""), attributes);
    }

    private (string current, string max, string id) GetAttribute(string key, (string current, string max, string id) defaultValue, Dictionary<string, (string current, string max, string id)> attributes)
    {
        return attributes.TryGetValue(key.ToLower(), out var value) ? value : defaultValue;
    }

    private int GetAttributeInt(string key, int defaultValue, Dictionary<string, (string current, string max, string id)> attributes)
    {
        var attr = GetAttribute(key, defaultValue.ToString(), attributes);
        return int.TryParse(attr.current, out var result) ? result : defaultValue;
    }

    private int GetProficiencyBonus(Dictionary<string, (string current, string max, string id)> attributes)
    {
        var level = GetAttributeInt("level", 1, attributes);
        return (int)Math.Ceiling(level / 4.0) + 1; // D&D 5E proficiency bonus
    }

    private string GetSpellcastingAbility(Dictionary<string, (string current, string max, string id)> attributes)
    {
        // Check common spellcasting ability attributes
        var spellAbility = GetAttribute("spellcasting_ability", "", attributes).current;
        if (!string.IsNullOrEmpty(spellAbility))
            return spellAbility.ToLower();

        // Default based on class
        var characterClass = GetAttribute("class", "", attributes).current.ToLower();
        return characterClass switch
        {
            "wizard" => "int",
            "cleric" or "druid" or "ranger" => "wis", 
            "bard" or "paladin" or "sorcerer" or "warlock" => "cha",
            _ => "int"
        };
    }
}
