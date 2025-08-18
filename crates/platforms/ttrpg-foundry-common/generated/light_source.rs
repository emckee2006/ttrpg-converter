#[allow(unused_imports)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::uninlined_format_args)]
use serde::{Deserialize, Serialize};
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///6-digit hexadecimal color code
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Color Hex",
///  "description": "6-digit hexadecimal color code",
///  "type": "string",
///  "pattern": "^#[0-9a-fA-F]{6}$",
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ColorHex(::std::string::String);
impl ::std::ops::Deref for ColorHex {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ColorHex> for ::std::string::String {
    fn from(value: ColorHex) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ColorHex> for ColorHex {
    fn from(value: &ColorHex) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ColorHex {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^#[0-9a-fA-F]{6}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#[0-9a-fA-F]{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ColorHex {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///Light emission properties
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Light Source",
///  "description": "Light emission properties",
///  "type": "object",
///  "properties": {
///    "angle": {
///      "description": "Light angle in degrees",
///      "type": "number",
///      "maximum": 360.0,
///      "minimum": 0.0
///    },
///    "animation": {
///      "type": "object",
///      "properties": {
///        "intensity": {
///          "type": "number",
///          "maximum": 10.0,
///          "minimum": 1.0
///        },
///        "speed": {
///          "type": "number",
///          "maximum": 10.0,
///          "minimum": 1.0
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "none",
///            "pulse",
///            "chroma",
///            "wave",
///            "fog",
///            "sunburst",
///            "dome",
///            "emanation",
///            "hexa",
///            "ghost",
///            "energy",
///            "roiling",
///            "hole"
///          ]
///        }
///      }
///    },
///    "bright": {
///      "description": "Bright light radius",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "color": {
///      "title": "Color Hex",
///      "description": "6-digit hexadecimal color code",
///      "type": "string",
///      "pattern": "^#[0-9a-fA-F]{6}$",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "dim": {
///      "description": "Dim light radius",
///      "type": "number",
///      "minimum": 0.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LightSource {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub angle: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub animation: ::std::option::Option<LightSourceAnimation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bright: ::std::option::Option<f64>,
    ///6-digit hexadecimal color code
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<ColorHex>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dim: ::std::option::Option<f64>,
}
impl ::std::convert::From<&LightSource> for LightSource {
    fn from(value: &LightSource) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LightSource {
    fn default() -> Self {
        Self {
            angle: Default::default(),
            animation: Default::default(),
            bright: Default::default(),
            color: Default::default(),
            dim: Default::default(),
        }
    }
}
impl LightSource {
    pub fn builder() -> builder::LightSource {
        Default::default()
    }
}
///`LightSourceAnimation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "intensity": {
///      "type": "number",
///      "maximum": 10.0,
///      "minimum": 1.0
///    },
///    "speed": {
///      "type": "number",
///      "maximum": 10.0,
///      "minimum": 1.0
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "none",
///        "pulse",
///        "chroma",
///        "wave",
///        "fog",
///        "sunburst",
///        "dome",
///        "emanation",
///        "hexa",
///        "ghost",
///        "energy",
///        "roiling",
///        "hole"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LightSourceAnimation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intensity: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<LightSourceAnimationType>,
}
impl ::std::convert::From<&LightSourceAnimation> for LightSourceAnimation {
    fn from(value: &LightSourceAnimation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for LightSourceAnimation {
    fn default() -> Self {
        Self {
            intensity: Default::default(),
            speed: Default::default(),
            type_: Default::default(),
        }
    }
}
impl LightSourceAnimation {
    pub fn builder() -> builder::LightSourceAnimation {
        Default::default()
    }
}
///`LightSourceAnimationType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "none",
///    "pulse",
///    "chroma",
///    "wave",
///    "fog",
///    "sunburst",
///    "dome",
///    "emanation",
///    "hexa",
///    "ghost",
///    "energy",
///    "roiling",
///    "hole"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum LightSourceAnimationType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "pulse")]
    Pulse,
    #[serde(rename = "chroma")]
    Chroma,
    #[serde(rename = "wave")]
    Wave,
    #[serde(rename = "fog")]
    Fog,
    #[serde(rename = "sunburst")]
    Sunburst,
    #[serde(rename = "dome")]
    Dome,
    #[serde(rename = "emanation")]
    Emanation,
    #[serde(rename = "hexa")]
    Hexa,
    #[serde(rename = "ghost")]
    Ghost,
    #[serde(rename = "energy")]
    Energy,
    #[serde(rename = "roiling")]
    Roiling,
    #[serde(rename = "hole")]
    Hole,
}
impl ::std::convert::From<&Self> for LightSourceAnimationType {
    fn from(value: &LightSourceAnimationType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LightSourceAnimationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("none"),
            Self::Pulse => f.write_str("pulse"),
            Self::Chroma => f.write_str("chroma"),
            Self::Wave => f.write_str("wave"),
            Self::Fog => f.write_str("fog"),
            Self::Sunburst => f.write_str("sunburst"),
            Self::Dome => f.write_str("dome"),
            Self::Emanation => f.write_str("emanation"),
            Self::Hexa => f.write_str("hexa"),
            Self::Ghost => f.write_str("ghost"),
            Self::Energy => f.write_str("energy"),
            Self::Roiling => f.write_str("roiling"),
            Self::Hole => f.write_str("hole"),
        }
    }
}
impl ::std::str::FromStr for LightSourceAnimationType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "none" => Ok(Self::None),
            "pulse" => Ok(Self::Pulse),
            "chroma" => Ok(Self::Chroma),
            "wave" => Ok(Self::Wave),
            "fog" => Ok(Self::Fog),
            "sunburst" => Ok(Self::Sunburst),
            "dome" => Ok(Self::Dome),
            "emanation" => Ok(Self::Emanation),
            "hexa" => Ok(Self::Hexa),
            "ghost" => Ok(Self::Ghost),
            "energy" => Ok(Self::Energy),
            "roiling" => Ok(Self::Roiling),
            "hole" => Ok(Self::Hole),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LightSourceAnimationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LightSourceAnimationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LightSourceAnimationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct LightSource {
        angle: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        animation: ::std::result::Result<
            ::std::option::Option<super::LightSourceAnimation>,
            ::std::string::String,
        >,
        bright: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        color: ::std::result::Result<
            ::std::option::Option<super::ColorHex>,
            ::std::string::String,
        >,
        dim: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for LightSource {
        fn default() -> Self {
            Self {
                angle: Ok(Default::default()),
                animation: Ok(Default::default()),
                bright: Ok(Default::default()),
                color: Ok(Default::default()),
                dim: Ok(Default::default()),
            }
        }
    }
    impl LightSource {
        pub fn angle<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.angle = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for angle: {}", e)
                });
            self
        }
        pub fn animation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LightSourceAnimation>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.animation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for animation: {}", e)
                });
            self
        }
        pub fn bright<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bright = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bright: {}", e)
                });
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ColorHex>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn dim<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dim = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dim: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<LightSource> for super::LightSource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LightSource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                angle: value.angle?,
                animation: value.animation?,
                bright: value.bright?,
                color: value.color?,
                dim: value.dim?,
            })
        }
    }
    impl ::std::convert::From<super::LightSource> for LightSource {
        fn from(value: super::LightSource) -> Self {
            Self {
                angle: Ok(value.angle),
                animation: Ok(value.animation),
                bright: Ok(value.bright),
                color: Ok(value.color),
                dim: Ok(value.dim),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LightSourceAnimation {
        intensity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        speed: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::LightSourceAnimationType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for LightSourceAnimation {
        fn default() -> Self {
            Self {
                intensity: Ok(Default::default()),
                speed: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl LightSourceAnimation {
        pub fn intensity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.intensity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intensity: {}", e)
                });
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::LightSourceAnimationType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LightSourceAnimation> for super::LightSourceAnimation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LightSourceAnimation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                intensity: value.intensity?,
                speed: value.speed?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::LightSourceAnimation> for LightSourceAnimation {
        fn from(value: super::LightSourceAnimation) -> Self {
            Self {
                intensity: Ok(value.intensity),
                speed: Ok(value.speed),
                type_: Ok(value.type_),
            }
        }
    }
}
