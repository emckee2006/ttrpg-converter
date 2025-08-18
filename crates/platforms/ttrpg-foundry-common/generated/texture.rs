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
///Texture/image with additional properties
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Texture",
///  "description": "Texture/image with additional properties",
///  "type": "object",
///  "properties": {
///    "offsetX": {
///      "description": "Horizontal offset",
///      "type": "number"
///    },
///    "offsetY": {
///      "description": "Vertical offset",
///      "type": "number"
///    },
///    "rotation": {
///      "description": "Rotation in degrees",
///      "type": "number"
///    },
///    "scaleX": {
///      "description": "Horizontal scale factor",
///      "type": "number"
///    },
///    "scaleY": {
///      "description": "Vertical scale factor",
///      "type": "number"
///    },
///    "src": {
///      "title": "Image Reference",
///      "description": "Image file URL or path",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uri",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "tint": {
///      "description": "Hex color tint",
///      "type": [
///        "string",
///        "null"
///      ],
///      "pattern": "^#[0-9a-fA-F]{6}$"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Texture {
    #[serde(
        rename = "offsetX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offset_x: ::std::option::Option<f64>,
    #[serde(
        rename = "offsetY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub offset_y: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rotation: ::std::option::Option<f64>,
    #[serde(
        rename = "scaleX",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scale_x: ::std::option::Option<f64>,
    #[serde(
        rename = "scaleY",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scale_y: ::std::option::Option<f64>,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<::std::string::String>,
    ///Hex color tint
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tint: ::std::option::Option<TextureTint>,
}
impl ::std::convert::From<&Texture> for Texture {
    fn from(value: &Texture) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Texture {
    fn default() -> Self {
        Self {
            offset_x: Default::default(),
            offset_y: Default::default(),
            rotation: Default::default(),
            scale_x: Default::default(),
            scale_y: Default::default(),
            src: Default::default(),
            tint: Default::default(),
        }
    }
}
impl Texture {
    pub fn builder() -> builder::Texture {
        Default::default()
    }
}
///Hex color tint
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Hex color tint",
///  "type": "string",
///  "pattern": "^#[0-9a-fA-F]{6}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TextureTint(::std::string::String);
impl ::std::ops::Deref for TextureTint {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TextureTint> for ::std::string::String {
    fn from(value: TextureTint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TextureTint> for TextureTint {
    fn from(value: &TextureTint) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TextureTint {
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
impl ::std::convert::TryFrom<&str> for TextureTint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TextureTint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TextureTint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TextureTint {
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
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Texture {
        offset_x: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        offset_y: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        rotation: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        scale_x: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        scale_y: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        src: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tint: ::std::result::Result<
            ::std::option::Option<super::TextureTint>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Texture {
        fn default() -> Self {
            Self {
                offset_x: Ok(Default::default()),
                offset_y: Ok(Default::default()),
                rotation: Ok(Default::default()),
                scale_x: Ok(Default::default()),
                scale_y: Ok(Default::default()),
                src: Ok(Default::default()),
                tint: Ok(Default::default()),
            }
        }
    }
    impl Texture {
        pub fn offset_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.offset_x = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for offset_x: {}", e)
                });
            self
        }
        pub fn offset_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.offset_y = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for offset_y: {}", e)
                });
            self
        }
        pub fn rotation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.rotation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for rotation: {}", e)
                });
            self
        }
        pub fn scale_x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scale_x = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scale_x: {}", e)
                });
            self
        }
        pub fn scale_y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.scale_y = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scale_y: {}", e)
                });
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn tint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TextureTint>>,
            T::Error: ::std::fmt::Display,
        {
            self.tint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tint: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Texture> for super::Texture {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Texture,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                offset_x: value.offset_x?,
                offset_y: value.offset_y?,
                rotation: value.rotation?,
                scale_x: value.scale_x?,
                scale_y: value.scale_y?,
                src: value.src?,
                tint: value.tint?,
            })
        }
    }
    impl ::std::convert::From<super::Texture> for Texture {
        fn from(value: super::Texture) -> Self {
            Self {
                offset_x: Ok(value.offset_x),
                offset_y: Ok(value.offset_y),
                rotation: Ok(value.rotation),
                scale_x: Ok(value.scale_x),
                scale_y: Ok(value.scale_y),
                src: Ok(value.src),
                tint: Ok(value.tint),
            }
        }
    }
}
