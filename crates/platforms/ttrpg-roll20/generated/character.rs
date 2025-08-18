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
///Character sheet attribute (e.g., strength, hp)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Attribute",
///  "description": "Character sheet attribute (e.g., strength, hp)",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "current": {
///      "description": "Current value as string",
///      "type": "string"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "max": {
///      "description": "Maximum value as string",
///      "type": "string"
///    },
///    "name": {
///      "description": "Attribute name (e.g., 'strength', 'hp')",
///      "type": "string",
///      "minLength": 1
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterAttribute {
    ///Current value as string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub current: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Roll20CharacterAttributeId>,
    ///Maximum value as string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<::std::string::String>,
    ///Attribute name (e.g., 'strength', 'hp')
    pub name: Roll20CharacterAttributeName,
}
impl ::std::convert::From<&Roll20CharacterAttribute> for Roll20CharacterAttribute {
    fn from(value: &Roll20CharacterAttribute) -> Self {
        value.clone()
    }
}
impl Roll20CharacterAttribute {
    pub fn builder() -> builder::Roll20CharacterAttribute {
        Default::default()
    }
}
///`Roll20CharacterAttributeId`
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
pub struct Roll20CharacterAttributeId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAttributeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAttributeId> for ::std::string::String {
    fn from(value: Roll20CharacterAttributeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAttributeId> for Roll20CharacterAttributeId {
    fn from(value: &Roll20CharacterAttributeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAttributeId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAttributeId {
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
///Attribute name (e.g., 'strength', 'hp')
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Attribute name (e.g., 'strength', 'hp')",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterAttributeName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAttributeName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAttributeName> for ::std::string::String {
    fn from(value: Roll20CharacterAttributeName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAttributeName>
for Roll20CharacterAttributeName {
    fn from(value: &Roll20CharacterAttributeName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAttributeName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAttributeName {
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
///Schema for Roll20 character sheet with attributes and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Sheet",
///  "description": "Schema for Roll20 character sheet with attributes and abilities",
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "abilities": {
///      "description": "Character sheet abilities/macros",
///      "type": "array",
///      "items": {
///        "title": "Roll20 Character Ability",
///        "description": "Character sheet ability/macro",
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "action": {
///            "description": "Macro commands/actions",
///            "type": "string"
///          },
///          "description": {
///            "description": "Ability description",
///            "type": "string"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "istokenaction": {
///            "description": "Whether this appears as token action",
///            "default": false,
///            "type": "boolean"
///          },
///          "name": {
///            "description": "Ability name",
///            "type": "string",
///            "minLength": 1
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "archived": {
///      "description": "Whether character is archived",
///      "default": false,
///      "type": "boolean"
///    },
///    "attribs": {
///      "description": "Character sheet attributes",
///      "type": "array",
///      "items": {
///        "title": "Roll20 Character Attribute",
///        "description": "Character sheet attribute (e.g., strength, hp)",
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "current": {
///            "description": "Current value as string",
///            "type": "string"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "max": {
///            "description": "Maximum value as string",
///            "type": "string"
///          },
///          "name": {
///            "description": "Attribute name (e.g., 'strength', 'hp')",
///            "type": "string",
///            "minLength": 1
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "avatar": {
///      "description": "Character avatar image URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "bio": {
///      "description": "Character biography/notes",
///      "type": "string"
///    },
///    "controlledby": {
///      "description": "Comma-separated player IDs who can control this character",
///      "type": "string"
///    },
///    "defaulttoken": {
///      "description": "Default token settings JSON",
///      "type": "string"
///    },
///    "gmnotes": {
///      "description": "GM-only notes",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique Roll20 character ID",
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "inplayerjournals": {
///      "description": "Comma-separated player IDs who can see this character",
///      "type": "string"
///    },
///    "name": {
///      "description": "Character name",
///      "type": "string",
///      "minLength": 1
///    },
///    "tags": {
///      "description": "Character tags",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterSheet {
    ///Character sheet abilities/macros
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub abilities: ::std::vec::Vec<Roll20CharacterAbility>,
    ///Whether character is archived
    #[serde(default)]
    pub archived: bool,
    ///Character sheet attributes
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attribs: ::std::vec::Vec<Roll20CharacterAttribute>,
    ///Character avatar image URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    ///Character biography/notes
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bio: ::std::option::Option<::std::string::String>,
    ///Comma-separated player IDs who can control this character
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub controlledby: ::std::option::Option<::std::string::String>,
    ///Default token settings JSON
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub defaulttoken: ::std::option::Option<::std::string::String>,
    ///GM-only notes
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gmnotes: ::std::option::Option<::std::string::String>,
    ///Unique Roll20 character ID
    pub id: Roll20CharacterSheetId,
    ///Comma-separated player IDs who can see this character
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inplayerjournals: ::std::option::Option<::std::string::String>,
    ///Character name
    pub name: Roll20CharacterSheetName,
    ///Character tags
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Roll20CharacterSheet> for Roll20CharacterSheet {
    fn from(value: &Roll20CharacterSheet) -> Self {
        value.clone()
    }
}
impl Roll20CharacterSheet {
    pub fn builder() -> builder::Roll20CharacterSheet {
        Default::default()
    }
}
///Unique Roll20 character ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique Roll20 character ID",
///  "type": "string",
///  "pattern": "^-[A-Za-z0-9_-]{19}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterSheetId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterSheetId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterSheetId> for ::std::string::String {
    fn from(value: Roll20CharacterSheetId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterSheetId> for Roll20CharacterSheetId {
    fn from(value: &Roll20CharacterSheetId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterSheetId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterSheetId {
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
///Character name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Character name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterSheetName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterSheetName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterSheetName> for ::std::string::String {
    fn from(value: Roll20CharacterSheetName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterSheetName> for Roll20CharacterSheetName {
    fn from(value: &Roll20CharacterSheetName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterSheetName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterSheetName {
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
    #[derive(Clone, Debug)]
    pub struct Roll20CharacterAttribute {
        current: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::Roll20CharacterAttributeId>,
            ::std::string::String,
        >,
        max: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::Roll20CharacterAttributeName,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterAttribute {
        fn default() -> Self {
            Self {
                current: Ok(Default::default()),
                id: Ok(Default::default()),
                max: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Roll20CharacterAttribute {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::Roll20CharacterAttributeId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Roll20CharacterAttributeName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterAttribute>
    for super::Roll20CharacterAttribute {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterAttribute,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                id: value.id?,
                max: value.max?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterAttribute>
    for Roll20CharacterAttribute {
        fn from(value: super::Roll20CharacterAttribute) -> Self {
            Self {
                current: Ok(value.current),
                id: Ok(value.id),
                max: Ok(value.max),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Roll20CharacterSheet {
        abilities: ::std::result::Result<
            ::std::vec::Vec<super::Roll20CharacterAbility>,
            ::std::string::String,
        >,
        archived: ::std::result::Result<bool, ::std::string::String>,
        attribs: ::std::result::Result<
            ::std::vec::Vec<super::Roll20CharacterAttribute>,
            ::std::string::String,
        >,
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        bio: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        controlledby: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        defaulttoken: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gmnotes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::Roll20CharacterSheetId, ::std::string::String>,
        inplayerjournals: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::Roll20CharacterSheetName,
            ::std::string::String,
        >,
        tags: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterSheet {
        fn default() -> Self {
            Self {
                abilities: Ok(Default::default()),
                archived: Ok(Default::default()),
                attribs: Ok(Default::default()),
                avatar: Ok(Default::default()),
                bio: Ok(Default::default()),
                controlledby: Ok(Default::default()),
                defaulttoken: Ok(Default::default()),
                gmnotes: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                inplayerjournals: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl Roll20CharacterSheet {
        pub fn abilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Roll20CharacterAbility>>,
            T::Error: ::std::fmt::Display,
        {
            self.abilities = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for abilities: {}", e)
                });
            self
        }
        pub fn archived<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.archived = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for archived: {}", e)
                });
            self
        }
        pub fn attribs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Roll20CharacterAttribute>>,
            T::Error: ::std::fmt::Display,
        {
            self.attribs = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attribs: {}", e)
                });
            self
        }
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
        pub fn bio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bio: {}", e));
            self
        }
        pub fn controlledby<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.controlledby = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for controlledby: {}", e)
                });
            self
        }
        pub fn defaulttoken<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.defaulttoken = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for defaulttoken: {}", e)
                });
            self
        }
        pub fn gmnotes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.gmnotes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for gmnotes: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Roll20CharacterSheetId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn inplayerjournals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.inplayerjournals = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for inplayerjournals: {}", e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Roll20CharacterSheetName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterSheet> for super::Roll20CharacterSheet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterSheet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abilities: value.abilities?,
                archived: value.archived?,
                attribs: value.attribs?,
                avatar: value.avatar?,
                bio: value.bio?,
                controlledby: value.controlledby?,
                defaulttoken: value.defaulttoken?,
                gmnotes: value.gmnotes?,
                id: value.id?,
                inplayerjournals: value.inplayerjournals?,
                name: value.name?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterSheet> for Roll20CharacterSheet {
        fn from(value: super::Roll20CharacterSheet) -> Self {
            Self {
                abilities: Ok(value.abilities),
                archived: Ok(value.archived),
                attribs: Ok(value.attribs),
                avatar: Ok(value.avatar),
                bio: Ok(value.bio),
                controlledby: Ok(value.controlledby),
                defaulttoken: Ok(value.defaulttoken),
                gmnotes: Ok(value.gmnotes),
                id: Ok(value.id),
                inplayerjournals: Ok(value.inplayerjournals),
                name: Ok(value.name),
                tags: Ok(value.tags),
            }
        }
    }
}
