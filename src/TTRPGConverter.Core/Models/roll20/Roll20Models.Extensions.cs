using System.Collections.Generic;
using System.Text.Json.Serialization;

namespace TTRPGConverter.Core.Models.Roll20;

#pragma warning disable // Disable all warnings

// Using partial classes to extend the auto-generated models from Roll20Models.cs

public partial class Page
{
    [JsonPropertyName("graphics")]
    public IList<Graphic>? Graphics { get; set; }

    [JsonPropertyName("paths")]
    public IList<Path>? Paths { get; set; }

    [JsonPropertyName("text")]
    public IList<Text>? Texts { get; set; }

    [JsonPropertyName("scale_number")]
    public double ScaleNumber { get; set; }

    [JsonPropertyName("scale_units")]
    public string? ScaleUnits { get; set; }

    [JsonPropertyName("gridcolor")]
    public string? Gridcolor { get; set; }

    [JsonPropertyName("grid_alpha")]
    public double GridAlpha { get; set; }
}

public partial class Graphic
{
    [JsonPropertyName("id")]
    public string? Id { get; set; }

    [JsonPropertyName("name")]
    public string? Name { get; set; }

    [JsonPropertyName("imgsrc")]
    public string? Imgsrc { get; set; }

    [JsonPropertyName("represents")]
    public string? Represents { get; set; }

    [JsonPropertyName("left")]
    public double Left { get; set; }

    [JsonPropertyName("top")]
    public double Top { get; set; }

    [JsonPropertyName("width")]
    public double Width { get; set; }

    [JsonPropertyName("height")]
    public double Height { get; set; }

    [JsonPropertyName("rotation")]
    public double Rotation { get; set; }

    [JsonPropertyName("light_radius")]
    public double LightRadius { get; set; }

    [JsonPropertyName("light_dimradius")]
    public double LightDimradius { get; set; }

    [JsonPropertyName("light_angle")]
    public double LightAngle { get; set; }

    [JsonPropertyName("light_color")]
    public string? LightColor { get; set; }

    [JsonPropertyName("light_hassight")]
    public bool LightHasSight { get; set; }

    [JsonPropertyName("light_losangle")]
    public double LightLosAngle { get; set; }
}

public partial class Path
{
    [JsonPropertyName("id")]
    public string? Id { get; set; }

    [JsonPropertyName("path")]
    public string? PathData { get; set; }

    [JsonPropertyName("stroke")]
    public string? Stroke { get; set; }
}

public partial class Text
{
    // Properties for text objects will be added here later.
}

public partial class Message
{
    [JsonPropertyName("player")]
    public Player? Player { get; set; }

    [JsonPropertyName("content")]
    public string? Content { get; set; }
}
