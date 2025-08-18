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
///Resource with current/maximum values
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Resource",
///  "description": "Resource with current/maximum values",
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "temp": {
///      "description": "Temporary bonus",
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "minimum": 0.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Resource> for Resource {
    fn from(value: &Resource) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Resource {
    fn default() -> Self {
        Self {
            max: Default::default(),
            temp: Default::default(),
            value: Default::default(),
        }
    }
}
impl Resource {
    pub fn builder() -> builder::Resource {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Resource {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        temp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Resource {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                temp: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl Resource {
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
        pub fn temp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temp: {}", e));
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
    impl ::std::convert::TryFrom<Resource> for super::Resource {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Resource,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                temp: value.temp?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::Resource> for Resource {
        fn from(value: super::Resource) -> Self {
            Self {
                max: Ok(value.max),
                temp: Ok(value.temp),
                value: Ok(value.value),
            }
        }
    }
}
