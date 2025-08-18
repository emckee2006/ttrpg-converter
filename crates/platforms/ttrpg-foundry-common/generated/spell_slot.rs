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
///D&D 5e spell slot with current/max values
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "D&D 5e Spell Slot",
///  "description": "D&D 5e spell slot with current/max values",
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "override": {
///      "type": [
///        "number",
///        "null"
///      ]
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
pub struct DD5eSpellSlot {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(
        rename = "override",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub override_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&DD5eSpellSlot> for DD5eSpellSlot {
    fn from(value: &DD5eSpellSlot) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DD5eSpellSlot {
    fn default() -> Self {
        Self {
            max: Default::default(),
            override_: Default::default(),
            value: Default::default(),
        }
    }
}
impl DD5eSpellSlot {
    pub fn builder() -> builder::DD5eSpellSlot {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct DD5eSpellSlot {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        override_: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for DD5eSpellSlot {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                override_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl DD5eSpellSlot {
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
        pub fn override_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.override_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for override_: {}", e)
                });
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
    impl ::std::convert::TryFrom<DD5eSpellSlot> for super::DD5eSpellSlot {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DD5eSpellSlot,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                override_: value.override_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::DD5eSpellSlot> for DD5eSpellSlot {
        fn from(value: super::DD5eSpellSlot) -> Self {
            Self {
                max: Ok(value.max),
                override_: Ok(value.override_),
                value: Ok(value.value),
            }
        }
    }
}
