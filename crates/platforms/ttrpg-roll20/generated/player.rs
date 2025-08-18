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
///Extracted player definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Player",
///  "description": "Extracted player definition",
///  "type": "object",
///  "properties": {
///    "color": {
///      "type": "string"
///    },
///    "displayname": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "showname": {
///      "type": "boolean"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Player {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub displayname: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub showname: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Player> for Player {
    fn from(value: &Player) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Player {
    fn default() -> Self {
        Self {
            color: Default::default(),
            displayname: Default::default(),
            id: Default::default(),
            showname: Default::default(),
        }
    }
}
impl Player {
    pub fn builder() -> builder::Player {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Player {
        color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        displayname: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        showname: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Player {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                displayname: Ok(Default::default()),
                id: Ok(Default::default()),
                showname: Ok(Default::default()),
            }
        }
    }
    impl Player {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn displayname<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.displayname = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for displayname: {}", e)
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
        pub fn showname<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.showname = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for showname: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Player> for super::Player {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Player,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                displayname: value.displayname?,
                id: value.id?,
                showname: value.showname?,
            })
        }
    }
    impl ::std::convert::From<super::Player> for Player {
        fn from(value: super::Player) -> Self {
            Self {
                color: Ok(value.color),
                displayname: Ok(value.displayname),
                id: Ok(value.id),
                showname: Ok(value.showname),
            }
        }
    }
}
