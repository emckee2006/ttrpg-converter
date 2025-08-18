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
///Character sheet ability/macro
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Ability",
///  "description": "Character sheet ability/macro",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "action": {
///      "description": "Macro commands/actions",
///      "type": "string"
///    },
///    "description": {
///      "description": "Ability description",
///      "type": "string"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "istokenaction": {
///      "description": "Whether this appears as token action",
///      "default": false,
///      "type": "boolean"
///    },
///    "name": {
///      "description": "Ability name",
///      "type": "string",
///      "minLength": 1
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterAbility {
    ///Macro commands/actions
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<::std::string::String>,
    ///Ability description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Roll20CharacterAbilityId>,
    ///Whether this appears as token action
    #[serde(default)]
    pub istokenaction: bool,
    ///Ability name
    pub name: Roll20CharacterAbilityName,
}
impl ::std::convert::From<&Roll20CharacterAbility> for Roll20CharacterAbility {
    fn from(value: &Roll20CharacterAbility) -> Self {
        value.clone()
    }
}
impl Roll20CharacterAbility {
    pub fn builder() -> builder::Roll20CharacterAbility {
        Default::default()
    }
}
///`Roll20CharacterAbilityId`
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
pub struct Roll20CharacterAbilityId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAbilityId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAbilityId> for ::std::string::String {
    fn from(value: Roll20CharacterAbilityId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAbilityId> for Roll20CharacterAbilityId {
    fn from(value: &Roll20CharacterAbilityId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAbilityId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAbilityId {
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
///Ability name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Ability name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterAbilityName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAbilityName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAbilityName> for ::std::string::String {
    fn from(value: Roll20CharacterAbilityName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAbilityName> for Roll20CharacterAbilityName {
    fn from(value: &Roll20CharacterAbilityName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAbilityName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAbilityName {
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
    pub struct Roll20CharacterAbility {
        action: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::Roll20CharacterAbilityId>,
            ::std::string::String,
        >,
        istokenaction: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<
            super::Roll20CharacterAbilityName,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterAbility {
        fn default() -> Self {
            Self {
                action: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                istokenaction: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Roll20CharacterAbility {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for action: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::Roll20CharacterAbilityId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn istokenaction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.istokenaction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for istokenaction: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Roll20CharacterAbilityName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterAbility>
    for super::Roll20CharacterAbility {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterAbility,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                description: value.description?,
                id: value.id?,
                istokenaction: value.istokenaction?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterAbility> for Roll20CharacterAbility {
        fn from(value: super::Roll20CharacterAbility) -> Self {
            Self {
                action: Ok(value.action),
                description: Ok(value.description),
                id: Ok(value.id),
                istokenaction: Ok(value.istokenaction),
                name: Ok(value.name),
            }
        }
    }
}
