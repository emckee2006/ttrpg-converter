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
///Numeric value with min/max range
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Numeric Range",
///  "description": "Numeric value with min/max range",
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number"
///    },
///    "min": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NumericRange {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&NumericRange> for NumericRange {
    fn from(value: &NumericRange) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for NumericRange {
    fn default() -> Self {
        Self {
            max: Default::default(),
            min: Default::default(),
            value: Default::default(),
        }
    }
}
impl NumericRange {
    pub fn builder() -> builder::NumericRange {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct NumericRange {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for NumericRange {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                min: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl NumericRange {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NumericRange> for super::NumericRange {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NumericRange,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                min: value.min?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::NumericRange> for NumericRange {
        fn from(value: super::NumericRange) -> Self {
            Self {
                max: Ok(value.max),
                min: Ok(value.min),
                value: Ok(value.value),
            }
        }
    }
}
