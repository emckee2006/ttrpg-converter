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
///Extracted card definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Card",
///  "description": "Extracted card definition",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "avatar": {
///      "description": "Card face image URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "name": {
///      "description": "Card name",
///      "type": "string",
///      "minLength": 1
///    },
///    "tooltip": {
///      "description": "Card tooltip text",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Card {
    ///Card face image URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<CardId>,
    ///Card name
    pub name: CardName,
    ///Card tooltip text
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tooltip: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Card> for Card {
    fn from(value: &Card) -> Self {
        value.clone()
    }
}
impl Card {
    pub fn builder() -> builder::Card {
        Default::default()
    }
}
///`CardId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^-[A-Za-z0-9_-]{19}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CardId(::std::string::String);
impl ::std::ops::Deref for CardId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CardId> for ::std::string::String {
    fn from(value: CardId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CardId> for CardId {
    fn from(value: &CardId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CardId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^-[A-Za-z0-9_-]{19}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^-[A-Za-z0-9_-]{19}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CardId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CardId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CardId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CardId {
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
///Card name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Card name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CardName(::std::string::String);
impl ::std::ops::Deref for CardName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CardName> for ::std::string::String {
    fn from(value: CardName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CardName> for CardName {
    fn from(value: &CardName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CardName {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CardName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CardName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CardName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CardName {
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
    pub struct Card {
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::CardId>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::CardName, ::std::string::String>,
        tooltip: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Card {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                tooltip: Ok(Default::default()),
            }
        }
    }
    impl Card {
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CardId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CardName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn tooltip<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tooltip = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tooltip: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Card> for super::Card {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Card,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                id: value.id?,
                name: value.name?,
                tooltip: value.tooltip?,
            })
        }
    }
    impl ::std::convert::From<super::Card> for Card {
        fn from(value: super::Card) -> Self {
            Self {
                avatar: Ok(value.avatar),
                id: Ok(value.id),
                name: Ok(value.name),
                tooltip: Ok(value.tooltip),
            }
        }
    }
}
