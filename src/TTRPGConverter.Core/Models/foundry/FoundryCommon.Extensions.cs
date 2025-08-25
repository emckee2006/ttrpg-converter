using System.Collections.Generic;
using System.Text.Json.Serialization;
using System;

namespace TTRPGConverter.Core.Models.Foundry.Dnd5e;

#pragma warning disable // Disable all warnings

// Using partial classes to extend the auto-generated models from FoundryDnd5e.cs

public partial class CoreJournal
{
    [JsonPropertyName("img")]
    public Uri? Img { get; set; }
}

public partial class CoreScene
{
    [JsonPropertyName("tokens")]
    public IList<Token>? Tokens { get; set; }

    [JsonPropertyName("walls")]
    public IList<Wall>? Walls { get; set; }

    [JsonPropertyName("lights")]
    public IList<Light>? Lights { get; set; }
}

public partial class Token : CommonBaseDocument
{
    [JsonPropertyName("x")]
    public double X { get; set; }

    [JsonPropertyName("y")]
    public double Y { get; set; }

    [JsonPropertyName("width")]
    public double Width { get; set; }

    [JsonPropertyName("height")]
    public double Height { get; set; }

    [JsonPropertyName("rotation")]
    public double Rotation { get; set; }

    [JsonPropertyName("actorId")]
    public string? ActorId { get; set; }

    [JsonPropertyName("img")]
    public Uri? Img { get; set; }

    [JsonPropertyName("vision")]
    public bool Vision { get; set; }

    [JsonPropertyName("sightAngle")]
    public double SightAngle { get; set; }

    [JsonPropertyName("light")]
    public Light? Light { get; set; }
}

public partial class Wall : CommonBaseDocument
{
    [JsonPropertyName("c")]
    public IList<double>? C { get; set; }

    [JsonPropertyName("door")]
    public int? Door { get; set; }
}

public partial class Light
{
    [JsonPropertyName("bright")]
    public double? Bright { get; set; }

    [JsonPropertyName("dim")]
    public double? Dim { get; set; }

    [JsonPropertyName("angle")]
    public double? Angle { get; set; }

    [JsonPropertyName("color")]
    public string? Color { get; set; }
}

public partial class ChatMessage : CommonBaseDocument
{
    [JsonPropertyName("speaker")]
    public Speaker? Speaker { get; set; }

    [JsonPropertyName("content")]
    public string? Content { get; set; }

    [JsonPropertyName("roll")]
    public DiceRoll? Roll { get; set; }
}

public partial class Speaker
{
    [JsonPropertyName("actor")]
    public string? Actor { get; set; }

    [JsonPropertyName("alias")]
    public string? Alias { get; set; }
}

public partial class DiceRoll
{
    [JsonPropertyName("formula")]
    public string? Formula { get; set; }

    [JsonPropertyName("result")]
    public string? Result { get; set; }
}
