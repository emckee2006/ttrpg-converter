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
///Width and height dimensions
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Dimensions",
///  "description": "Width and height dimensions",
///  "type": "object",
///  "properties": {
///    "height": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "width": {
///      "type": "number",
///      "minimum": 0.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Dimensions {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Dimensions> for Dimensions {
    fn from(value: &Dimensions) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Dimensions {
    fn default() -> Self {
        Self {
            height: Default::default(),
            width: Default::default(),
        }
    }
}
impl Dimensions {
    pub fn builder() -> builder::Dimensions {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Dimensions {
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Dimensions {
        fn default() -> Self {
            Self {
                height: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl Dimensions {
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Dimensions> for super::Dimensions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Dimensions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                height: value.height?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::Dimensions> for Dimensions {
        fn from(value: super::Dimensions) -> Self {
            Self {
                height: Ok(value.height),
                width: Ok(value.width),
            }
        }
    }
}
