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
///Extracted deck definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Deck",
///  "description": "Extracted deck definition",
///  "type": "object",
///  "properties": {
///    "avatar": {
///      "type": "string"
///    },
///    "cards": {
///      "type": "array"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "playerscandraw": {
///      "type": "boolean"
///    },
///    "shown": {
///      "type": "boolean"
///    },
///    "showplayers": {
///      "type": "boolean"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Deck {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub cards: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playerscandraw: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shown: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub showplayers: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Deck> for Deck {
    fn from(value: &Deck) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Deck {
    fn default() -> Self {
        Self {
            avatar: Default::default(),
            cards: Default::default(),
            id: Default::default(),
            name: Default::default(),
            playerscandraw: Default::default(),
            shown: Default::default(),
            showplayers: Default::default(),
        }
    }
}
impl Deck {
    pub fn builder() -> builder::Deck {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Deck {
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cards: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        playerscandraw: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        shown: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        showplayers: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Deck {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                cards: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                playerscandraw: Ok(Default::default()),
                shown: Ok(Default::default()),
                showplayers: Ok(Default::default()),
            }
        }
    }
    impl Deck {
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.avatar = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for avatar: {}", e)
                });
            self
        }
        pub fn cards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.cards = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cards: {}", e)
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
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn playerscandraw<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.playerscandraw = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playerscandraw: {}", e)
                });
            self
        }
        pub fn shown<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.shown = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shown: {}", e)
                });
            self
        }
        pub fn showplayers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.showplayers = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for showplayers: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Deck> for super::Deck {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Deck,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                cards: value.cards?,
                id: value.id?,
                name: value.name?,
                playerscandraw: value.playerscandraw?,
                shown: value.shown?,
                showplayers: value.showplayers?,
            })
        }
    }
    impl ::std::convert::From<super::Deck> for Deck {
        fn from(value: super::Deck) -> Self {
            Self {
                avatar: Ok(value.avatar),
                cards: Ok(value.cards),
                id: Ok(value.id),
                name: Ok(value.name),
                playerscandraw: Ok(value.playerscandraw),
                shown: Ok(value.shown),
                showplayers: Ok(value.showplayers),
            }
        }
    }
}
