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
///Extracted custom_feat definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Feat",
///  "description": "Extracted custom_feat definition",
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
///    "actionType": {
///      "description": "Action type (empty=passive, 1-3=actions, R=reaction, F=free)",
///      "type": "string",
///      "enum": [
///        "",
///        "1",
///        "2",
///        "3",
///        "R",
///        "F"
///      ]
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Feat description and effects",
///      "type": "string",
///      "minLength": 1
///    },
///    "frequency": {
///      "description": "Usage frequency",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique feat ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "level": {
///      "description": "Required level for feat",
///      "type": "integer",
///      "maximum": 20.0,
///      "minimum": 1.0
///    },
///    "name": {
///      "description": "Feat name",
///      "type": "string",
///      "minLength": 1
///    },
///    "prerequisites": {
///      "description": "Feat prerequisites",
///      "type": "string"
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "timestamp": {
///      "description": "Creation timestamp",
///      "type": "string",
///      "pattern": "^\\d+$"
///    },
///    "traits": {
///      "description": "Feat traits (comma-separated)",
///      "type": "string"
///    },
///    "trigger": {
///      "description": "Trigger condition",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomFeat {
    ///Action type (empty=passive, 1-3=actions, R=reaction, F=free)
    #[serde(
        rename = "actionType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub action_type: ::std::option::Option<CustomFeatActionType>,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Feat description and effects
    pub description: CustomFeatDescription,
    ///Usage frequency
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub frequency: ::std::option::Option<::std::string::String>,
    ///Unique feat ID (UUID)
    pub id: CustomFeatId,
    ///Required level for feat
    pub level: ::std::num::NonZeroU64,
    ///Feat name
    pub name: CustomFeatName,
    ///Feat prerequisites
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prerequisites: ::std::option::Option<::std::string::String>,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Creation timestamp
    pub timestamp: CustomFeatTimestamp,
    ///Feat traits (comma-separated)
    pub traits: ::std::string::String,
    ///Trigger condition
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CustomFeat> for CustomFeat {
    fn from(value: &CustomFeat) -> Self {
        value.clone()
    }
}
impl CustomFeat {
    pub fn builder() -> builder::CustomFeat {
        Default::default()
    }
}
///Action type (empty=passive, 1-3=actions, R=reaction, F=free)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Action type (empty=passive, 1-3=actions, R=reaction, F=free)",
///  "type": "string",
///  "enum": [
///    "",
///    "1",
///    "2",
///    "3",
///    "R",
///    "F"
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
pub enum CustomFeatActionType {
    #[serde(rename = "")]
    X,
    #[serde(rename = "1")]
    X1,
    #[serde(rename = "2")]
    X2,
    #[serde(rename = "3")]
    X3,
    R,
    F,
}
impl ::std::convert::From<&Self> for CustomFeatActionType {
    fn from(value: &CustomFeatActionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomFeatActionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str(""),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::R => f.write_str("R"),
            Self::F => f.write_str("F"),
        }
    }
}
impl ::std::str::FromStr for CustomFeatActionType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "" => Ok(Self::X),
            "1" => Ok(Self::X1),
            "2" => Ok(Self::X2),
            "3" => Ok(Self::X3),
            "R" => Ok(Self::R),
            "F" => Ok(Self::F),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomFeatActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomFeatActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomFeatActionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Feat description and effects
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Feat description and effects",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomFeatDescription(::std::string::String);
impl ::std::ops::Deref for CustomFeatDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomFeatDescription> for ::std::string::String {
    fn from(value: CustomFeatDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomFeatDescription> for CustomFeatDescription {
    fn from(value: &CustomFeatDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomFeatDescription {
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
impl ::std::convert::TryFrom<&str> for CustomFeatDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomFeatDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomFeatDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomFeatDescription {
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
///Unique feat ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique feat ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomFeatId(::std::string::String);
impl ::std::ops::Deref for CustomFeatId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomFeatId> for ::std::string::String {
    fn from(value: CustomFeatId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomFeatId> for CustomFeatId {
    fn from(value: &CustomFeatId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomFeatId {
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
impl ::std::convert::TryFrom<&str> for CustomFeatId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomFeatId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomFeatId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomFeatId {
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
///Feat name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Feat name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomFeatName(::std::string::String);
impl ::std::ops::Deref for CustomFeatName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomFeatName> for ::std::string::String {
    fn from(value: CustomFeatName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomFeatName> for CustomFeatName {
    fn from(value: &CustomFeatName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomFeatName {
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
impl ::std::convert::TryFrom<&str> for CustomFeatName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomFeatName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomFeatName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomFeatName {
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
pub struct CustomFeatTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomFeatTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomFeatTimestamp> for ::std::string::String {
    fn from(value: CustomFeatTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomFeatTimestamp> for CustomFeatTimestamp {
    fn from(value: &CustomFeatTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomFeatTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomFeatTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomFeatTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomFeatTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomFeatTimestamp {
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
    pub struct CustomFeat {
        action_type: ::std::result::Result<
            ::std::option::Option<super::CustomFeatActionType>,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomFeatDescription,
            ::std::string::String,
        >,
        frequency: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomFeatId, ::std::string::String>,
        level: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        name: ::std::result::Result<super::CustomFeatName, ::std::string::String>,
        prerequisites: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<
            super::CustomFeatTimestamp,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
        trigger: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomFeat {
        fn default() -> Self {
            Self {
                action_type: Ok(Default::default()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                frequency: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                level: Err("no value supplied for level".to_string()),
                name: Err("no value supplied for name".to_string()),
                prerequisites: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traits: Err("no value supplied for traits".to_string()),
                trigger: Ok(Default::default()),
            }
        }
    }
    impl CustomFeat {
        pub fn action_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomFeatActionType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.action_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for action_type: {}", e)
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
            T: ::std::convert::TryInto<super::CustomFeatDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn frequency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.frequency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for frequency: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomFeatId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
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
            T: ::std::convert::TryInto<super::CustomFeatName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn prerequisites<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.prerequisites = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for prerequisites: {}", e)
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
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomFeatTimestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
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
        pub fn trigger<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.trigger = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for trigger: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomFeat> for super::CustomFeat {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomFeat,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action_type: value.action_type?,
                database_id: value.database_id?,
                description: value.description?,
                frequency: value.frequency?,
                id: value.id?,
                level: value.level?,
                name: value.name?,
                prerequisites: value.prerequisites?,
                src: value.src?,
                timestamp: value.timestamp?,
                traits: value.traits?,
                trigger: value.trigger?,
            })
        }
    }
    impl ::std::convert::From<super::CustomFeat> for CustomFeat {
        fn from(value: super::CustomFeat) -> Self {
            Self {
                action_type: Ok(value.action_type),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                frequency: Ok(value.frequency),
                id: Ok(value.id),
                level: Ok(value.level),
                name: Ok(value.name),
                prerequisites: Ok(value.prerequisites),
                src: Ok(value.src),
                timestamp: Ok(value.timestamp),
                traits: Ok(value.traits),
                trigger: Ok(value.trigger),
            }
        }
    }
}
