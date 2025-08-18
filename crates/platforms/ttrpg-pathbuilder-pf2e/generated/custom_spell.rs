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
///Extracted custom_spell definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Spell",
///  "description": "Extracted custom_spell definition",
///  "type": "object",
///  "required": [
///    "databaseID",
///    "description",
///    "id",
///    "level",
///    "name",
///    "src",
///    "timestamp",
///    "traits"
///  ],
///  "properties": {
///    "area": {
///      "description": "Area of effect",
///      "type": "string"
///    },
///    "cast": {
///      "description": "Casting time and actions",
///      "type": "string"
///    },
///    "components": {
///      "description": "Spell components (somatic, verbal, material)",
///      "type": "string"
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Spell description and effects",
///      "type": "string",
///      "minLength": 1
///    },
///    "duration": {
///      "description": "Spell duration",
///      "type": "string"
///    },
///    "heightened": {
///      "description": "Heightened spell effects",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique spell ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "level": {
///      "description": "Spell level (0=cantrip)",
///      "type": "integer",
///      "maximum": 10.0,
///      "minimum": 0.0
///    },
///    "name": {
///      "description": "Spell name",
///      "type": "string",
///      "minLength": 1
///    },
///    "range": {
///      "description": "Spell range",
///      "type": "string"
///    },
///    "savingThrow": {
///      "description": "Required saving throw",
///      "type": "string"
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "targets": {
///      "description": "Valid targets",
///      "type": "string"
///    },
///    "timestamp": {
///      "description": "Creation timestamp",
///      "type": "string",
///      "pattern": "^\\d+$"
///    },
///    "traditions": {
///      "description": "Magical traditions",
///      "type": "array",
///      "items": {
///        "type": "string",
///        "enum": [
///          "Arcane",
///          "Divine",
///          "Occult",
///          "Primal"
///        ]
///      }
///    },
///    "traits": {
///      "description": "Spell traits (comma-separated)",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomSpell {
    ///Area of effect
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub area: ::std::option::Option<::std::string::String>,
    ///Casting time and actions
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cast: ::std::option::Option<::std::string::String>,
    ///Spell components (somatic, verbal, material)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub components: ::std::option::Option<::std::string::String>,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Spell description and effects
    pub description: CustomSpellDescription,
    ///Spell duration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<::std::string::String>,
    ///Heightened spell effects
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub heightened: ::std::option::Option<::std::string::String>,
    ///Unique spell ID (UUID)
    pub id: CustomSpellId,
    ///Spell level (0=cantrip)
    pub level: i64,
    ///Spell name
    pub name: CustomSpellName,
    ///Spell range
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<::std::string::String>,
    ///Required saving throw
    #[serde(
        rename = "savingThrow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub saving_throw: ::std::option::Option<::std::string::String>,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Valid targets
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub targets: ::std::option::Option<::std::string::String>,
    ///Creation timestamp
    pub timestamp: CustomSpellTimestamp,
    ///Magical traditions
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub traditions: ::std::vec::Vec<CustomSpellTraditionsItem>,
    ///Spell traits (comma-separated)
    pub traits: ::std::string::String,
}
impl ::std::convert::From<&CustomSpell> for CustomSpell {
    fn from(value: &CustomSpell) -> Self {
        value.clone()
    }
}
impl CustomSpell {
    pub fn builder() -> builder::CustomSpell {
        Default::default()
    }
}
///Spell description and effects
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Spell description and effects",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomSpellDescription(::std::string::String);
impl ::std::ops::Deref for CustomSpellDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomSpellDescription> for ::std::string::String {
    fn from(value: CustomSpellDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSpellDescription> for CustomSpellDescription {
    fn from(value: &CustomSpellDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomSpellDescription {
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
impl ::std::convert::TryFrom<&str> for CustomSpellDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomSpellDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomSpellDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomSpellDescription {
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
///Unique spell ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique spell ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomSpellId(::std::string::String);
impl ::std::ops::Deref for CustomSpellId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomSpellId> for ::std::string::String {
    fn from(value: CustomSpellId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSpellId> for CustomSpellId {
    fn from(value: &CustomSpellId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomSpellId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CustomSpellId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomSpellId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomSpellId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomSpellId {
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
///Spell name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Spell name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomSpellName(::std::string::String);
impl ::std::ops::Deref for CustomSpellName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomSpellName> for ::std::string::String {
    fn from(value: CustomSpellName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSpellName> for CustomSpellName {
    fn from(value: &CustomSpellName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomSpellName {
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
impl ::std::convert::TryFrom<&str> for CustomSpellName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomSpellName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomSpellName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomSpellName {
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
///Creation timestamp
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Creation timestamp",
///  "type": "string",
///  "pattern": "^\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomSpellTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomSpellTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomSpellTimestamp> for ::std::string::String {
    fn from(value: CustomSpellTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomSpellTimestamp> for CustomSpellTimestamp {
    fn from(value: &CustomSpellTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomSpellTimestamp {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CustomSpellTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomSpellTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomSpellTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomSpellTimestamp {
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
///`CustomSpellTraditionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Arcane",
///    "Divine",
///    "Occult",
///    "Primal"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CustomSpellTraditionsItem {
    Arcane,
    Divine,
    Occult,
    Primal,
}
impl ::std::convert::From<&Self> for CustomSpellTraditionsItem {
    fn from(value: &CustomSpellTraditionsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomSpellTraditionsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Arcane => f.write_str("Arcane"),
            Self::Divine => f.write_str("Divine"),
            Self::Occult => f.write_str("Occult"),
            Self::Primal => f.write_str("Primal"),
        }
    }
}
impl ::std::str::FromStr for CustomSpellTraditionsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Arcane" => Ok(Self::Arcane),
            "Divine" => Ok(Self::Divine),
            "Occult" => Ok(Self::Occult),
            "Primal" => Ok(Self::Primal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomSpellTraditionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomSpellTraditionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomSpellTraditionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CustomSpell {
        area: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cast: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        components: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomSpellDescription,
            ::std::string::String,
        >,
        duration: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        heightened: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomSpellId, ::std::string::String>,
        level: ::std::result::Result<i64, ::std::string::String>,
        name: ::std::result::Result<super::CustomSpellName, ::std::string::String>,
        range: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        saving_throw: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        targets: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        timestamp: ::std::result::Result<
            super::CustomSpellTimestamp,
            ::std::string::String,
        >,
        traditions: ::std::result::Result<
            ::std::vec::Vec<super::CustomSpellTraditionsItem>,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CustomSpell {
        fn default() -> Self {
            Self {
                area: Ok(Default::default()),
                cast: Ok(Default::default()),
                components: Ok(Default::default()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                duration: Ok(Default::default()),
                heightened: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                level: Err("no value supplied for level".to_string()),
                name: Err("no value supplied for name".to_string()),
                range: Ok(Default::default()),
                saving_throw: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                targets: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traditions: Ok(Default::default()),
                traits: Err("no value supplied for traits".to_string()),
            }
        }
    }
    impl CustomSpell {
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn cast<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cast = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cast: {}", e));
            self
        }
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.components = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for components: {}", e)
                });
            self
        }
        pub fn database_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.database_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for database_id: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomSpellDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn duration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for duration: {}", e)
                });
            self
        }
        pub fn heightened<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.heightened = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for heightened: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomSpellId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for level: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomSpellName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for range: {}", e)
                });
            self
        }
        pub fn saving_throw<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.saving_throw = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for saving_throw: {}", e)
                });
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn targets<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.targets = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for targets: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomSpellTimestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
        pub fn traditions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::CustomSpellTraditionsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.traditions = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for traditions: {}", e)
                });
            self
        }
        pub fn traits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.traits = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for traits: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomSpell> for super::CustomSpell {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomSpell,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                area: value.area?,
                cast: value.cast?,
                components: value.components?,
                database_id: value.database_id?,
                description: value.description?,
                duration: value.duration?,
                heightened: value.heightened?,
                id: value.id?,
                level: value.level?,
                name: value.name?,
                range: value.range?,
                saving_throw: value.saving_throw?,
                src: value.src?,
                targets: value.targets?,
                timestamp: value.timestamp?,
                traditions: value.traditions?,
                traits: value.traits?,
            })
        }
    }
    impl ::std::convert::From<super::CustomSpell> for CustomSpell {
        fn from(value: super::CustomSpell) -> Self {
            Self {
                area: Ok(value.area),
                cast: Ok(value.cast),
                components: Ok(value.components),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                duration: Ok(value.duration),
                heightened: Ok(value.heightened),
                id: Ok(value.id),
                level: Ok(value.level),
                name: Ok(value.name),
                range: Ok(value.range),
                saving_throw: Ok(value.saving_throw),
                src: Ok(value.src),
                targets: Ok(value.targets),
                timestamp: Ok(value.timestamp),
                traditions: Ok(value.traditions),
                traits: Ok(value.traits),
            }
        }
    }
}
