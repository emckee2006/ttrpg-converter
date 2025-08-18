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
///Ability score with modifiers
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Ability Score",
///  "description": "Ability score with modifiers",
///  "type": "object",
///  "properties": {
///    "bonus": {
///      "description": "Additional bonus",
///      "type": "number"
///    },
///    "mod": {
///      "description": "Calculated modifier",
///      "type": "number"
///    },
///    "proficient": {
///      "description": "Proficiency multiplier (0, 0.5, 1, 2)",
///      "type": "number"
///    },
///    "save": {
///      "description": "Calculated save bonus",
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AbilityScore {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bonus: ::std::option::Option<f64>,
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proficient: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&AbilityScore> for AbilityScore {
    fn from(value: &AbilityScore) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AbilityScore {
    fn default() -> Self {
        Self {
            bonus: Default::default(),
            mod_: Default::default(),
            proficient: Default::default(),
            save: Default::default(),
            value: Default::default(),
        }
    }
}
impl AbilityScore {
    pub fn builder() -> builder::AbilityScore {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AbilityScore {
        bonus: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        proficient: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        save: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AbilityScore {
        fn default() -> Self {
            Self {
                bonus: Ok(Default::default()),
                mod_: Ok(Default::default()),
                proficient: Ok(Default::default()),
                save: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl AbilityScore {
        pub fn bonus<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.bonus = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for bonus: {}", e)
                });
            self
        }
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
            self
        }
        pub fn proficient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.proficient = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proficient: {}", e)
                });
            self
        }
        pub fn save<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.save = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for save: {}", e));
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
    impl ::std::convert::TryFrom<AbilityScore> for super::AbilityScore {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AbilityScore,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bonus: value.bonus?,
                mod_: value.mod_?,
                proficient: value.proficient?,
                save: value.save?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::AbilityScore> for AbilityScore {
        fn from(value: super::AbilityScore) -> Self {
            Self {
                bonus: Ok(value.bonus),
                mod_: Ok(value.mod_),
                proficient: Ok(value.proficient),
                save: Ok(value.save),
                value: Ok(value.value),
            }
        }
    }
}
