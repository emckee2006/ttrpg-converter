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
///X/Y coordinate position
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Position",
///  "description": "X/Y coordinate position",
///  "type": "object",
///  "properties": {
///    "x": {
///      "type": "number"
///    },
///    "y": {
///      "type": "number"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Position {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub x: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub y: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Position> for Position {
    fn from(value: &Position) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Position {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl Position {
    pub fn builder() -> builder::Position {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Position {
        x: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        y: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl Position {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Position> for super::Position {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Position,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { x: value.x?, y: value.y? })
        }
    }
    impl ::std::convert::From<super::Position> for Position {
        fn from(value: super::Position) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
}
