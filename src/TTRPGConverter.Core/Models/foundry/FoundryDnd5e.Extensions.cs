using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace TTRPGConverter.Core.Models.Foundry.Dnd5e;

#pragma warning disable // Disable all warnings

// This file contains custom extensions to the auto-generated Dnd5e models.
// Only classes and properties that do NOT exist in the generated file should be here.

public partial class SystemModel
{
    [JsonPropertyName("skills")]
    public Skills? Skills { get; set; }

    [JsonPropertyName("traits")]
    public Traits? Traits { get; set; }

    [JsonPropertyName("bonuses")]
    public GlobalBonuses? Bonuses { get; set; }

    [JsonPropertyName("resources")]
    public Resources? Resources { get; set; }
}

public partial class SystemModel1 // For Items
{
    [JsonPropertyName("level")]
    public int? Level { get; set; }

    [JsonPropertyName("school")]
    public string? School { get; set; }
}

// --- Supporting Classes for Extensions ---

public partial class Dnd5eSkill
{
    [JsonPropertyName("value")] public double? Value { get; set; }
    [JsonPropertyName("proficient")] public double? Proficient { get; set; }
    [JsonPropertyName("bonuses")] public SkillBonuses? Bonuses { get; set; }
}

public partial class SkillBonuses
{
    [JsonPropertyName("check")] public string? Check { get; set; } = "";
}

public partial class Skills
{
    [JsonPropertyName("acr")] public Dnd5eSkill? Acr { get; set; }
    [JsonPropertyName("ani")] public Dnd5eSkill? Ani { get; set; }
    [JsonPropertyName("arc")] public Dnd5eSkill? Arc { get; set; }
    [JsonPropertyName("ath")] public Dnd5eSkill? Ath { get; set; }
    [JsonPropertyName("dec")] public Dnd5eSkill? Dec { get; set; }
    [JsonPropertyName("his")] public Dnd5eSkill? His { get; set; }
    [JsonPropertyName("ins")] public Dnd5eSkill? Ins { get; set; }
    [JsonPropertyName("itm")] public Dnd5eSkill? Itm { get; set; }
    [JsonPropertyName("inv")] public Dnd5eSkill? Inv { get; set; }
    [JsonPropertyName("med")] public Dnd5eSkill? Med { get; set; }
    [JsonPropertyName("nat")] public Dnd5eSkill? Nat { get; set; }
    [JsonPropertyName("prc")] public Dnd5eSkill? Prc { get; set; }
    [JsonPropertyName("prf")] public Dnd5eSkill? Prf { get; set; }
    [JsonPropertyName("per")] public Dnd5eSkill? Per { get; set; }
    [JsonPropertyName("rel")] public Dnd5eSkill? Rel { get; set; }
    [JsonPropertyName("slt")] public Dnd5eSkill? Slt { get; set; }
    [JsonPropertyName("ste")] public Dnd5eSkill? Ste { get; set; }
    [JsonPropertyName("sur")] public Dnd5eSkill? Sur { get; set; }
}

public partial class Trait
{
    [JsonPropertyName("value")] public IList<string> Value { get; set; } = new List<string>();
    [JsonPropertyName("custom")] public string? Custom { get; set; } = "";
}

public partial class Traits
{
    [JsonPropertyName("di")] public Trait? Di { get; set; }
    [JsonPropertyName("dr")] public Trait? Dr { get; set; }
    [JsonPropertyName("dv")] public Trait? Dv { get; set; }
    [JsonPropertyName("ci")] public Trait? Ci { get; set; }
    [JsonPropertyName("languages")] public Trait? Languages { get; set; }
}

public partial class AttackBonus
{
    [JsonPropertyName("attack")] public string? Attack { get; set; } = "";
    [JsonPropertyName("damage")] public string? Damage { get; set; } = "";
}

public partial class GlobalBonuses
{
    [JsonPropertyName("mwak")] public AttackBonus? Mwak { get; set; }
    [JsonPropertyName("rwak")] public AttackBonus? Rwak { get; set; }
    [JsonPropertyName("msak")] public AttackBonus? Msak { get; set; }
    [JsonPropertyName("rsak")] public AttackBonus? Rsak { get; set; }
}

public partial class Resources
{
    [JsonPropertyName("primary")] public CommonResource? Primary { get; set; }
    [JsonPropertyName("secondary")] public CommonResource? Secondary { get; set; }
    [JsonPropertyName("tertiary")] public CommonResource? Tertiary { get; set; }
}
