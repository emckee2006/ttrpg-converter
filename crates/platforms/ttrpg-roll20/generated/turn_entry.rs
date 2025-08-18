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
///Extracted turn_entry definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Turn Entry",
///  "description": "Extracted turn_entry definition",
///  "type": "object",
///  "properties": {
///    "_pageid": {
///      "type": "string"
///    },
///    "custom": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "pr": {
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TurnEntry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_pageid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pageid: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pr: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TurnEntry> for TurnEntry {
    fn from(value: &TurnEntry) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TurnEntry {
    fn default() -> Self {
        Self {
            custom: Default::default(),
            id: Default::default(),
            pageid: Default::default(),
            pr: Default::default(),
        }
    }
}
impl TurnEntry {
    pub fn builder() -> builder::TurnEntry {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct TurnEntry {
        custom: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pageid: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TurnEntry {
        fn default() -> Self {
            Self {
                custom: Ok(Default::default()),
                id: Ok(Default::default()),
                pageid: Ok(Default::default()),
                pr: Ok(Default::default()),
            }
        }
    }
    impl TurnEntry {
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn pageid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pageid = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pageid: {}", e)
                });
            self
        }
        pub fn pr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pr: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TurnEntry> for super::TurnEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TurnEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom: value.custom?,
                id: value.id?,
                pageid: value.pageid?,
                pr: value.pr?,
            })
        }
    }
    impl ::std::convert::From<super::TurnEntry> for TurnEntry {
        fn from(value: super::TurnEntry) -> Self {
            Self {
                custom: Ok(value.custom),
                id: Ok(value.id),
                pageid: Ok(value.pageid),
                pr: Ok(value.pr),
            }
        }
    }
}
