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
///Extracted custom_background definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Background",
///  "description": "Extracted custom_background definition",
///  "type": "object",
///  "required": [
///    "boost_ref_1",
///    "boost_ref_2",
///    "databaseID",
///    "description",
///    "id",
///    "name",
///    "skill",
///    "src",
///    "timestamp",
///    "traits"
///  ],
///  "properties": {
///    "boost_ref_1": {
///      "description": "First ability boost reference (0=Str, 1=Dex, 2=Con, 3=Int, 4=Wis, 5=Cha)",
///      "type": "string",
///      "enum": [
///        "0",
///        "1",
///        "2",
///        "3",
///        "4",
///        "5"
///      ]
///    },
///    "boost_ref_2": {
///      "description": "Second ability boost reference",
///      "type": "string",
///      "enum": [
///        "0",
///        "1",
///        "2",
///        "3",
///        "4",
///        "5"
///      ]
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Background description",
///      "type": "string",
///      "minLength": 1
///    },
///    "freeFeatDetail": {
///      "description": "Additional details for free feat",
///      "type": "string"
///    },
///    "freeFeatID": {
///      "description": "Free feat granted by background",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique background ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "lore": {
///      "description": "Lore skill granted",
///      "type": "string"
///    },
///    "name": {
///      "description": "Background name",
///      "type": "string",
///      "minLength": 1
///    },
///    "skill": {
///      "description": "Skill proficiency granted",
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
///      "description": "Background traits (e.g., '3rd Party')",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomBackground {
    ///First ability boost reference (0=Str, 1=Dex, 2=Con, 3=Int, 4=Wis, 5=Cha)
    pub boost_ref_1: CustomBackgroundBoostRef1,
    ///Second ability boost reference
    pub boost_ref_2: CustomBackgroundBoostRef2,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Background description
    pub description: CustomBackgroundDescription,
    ///Additional details for free feat
    #[serde(
        rename = "freeFeatDetail",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub free_feat_detail: ::std::option::Option<::std::string::String>,
    ///Free feat granted by background
    #[serde(
        rename = "freeFeatID",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub free_feat_id: ::std::option::Option<::std::string::String>,
    ///Unique background ID (UUID)
    pub id: CustomBackgroundId,
    ///Lore skill granted
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lore: ::std::option::Option<::std::string::String>,
    ///Background name
    pub name: CustomBackgroundName,
    ///Skill proficiency granted
    pub skill: ::std::string::String,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Creation timestamp
    pub timestamp: CustomBackgroundTimestamp,
    ///Background traits (e.g., '3rd Party')
    pub traits: ::std::string::String,
}
impl ::std::convert::From<&CustomBackground> for CustomBackground {
    fn from(value: &CustomBackground) -> Self {
        value.clone()
    }
}
impl CustomBackground {
    pub fn builder() -> builder::CustomBackground {
        Default::default()
    }
}
///First ability boost reference (0=Str, 1=Dex, 2=Con, 3=Int, 4=Wis, 5=Cha)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "First ability boost reference (0=Str, 1=Dex, 2=Con, 3=Int, 4=Wis, 5=Cha)",
///  "type": "string",
///  "enum": [
///    "0",
///    "1",
///    "2",
///    "3",
///    "4",
///    "5"
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
pub enum CustomBackgroundBoostRef1 {
    #[serde(rename = "0")]
    X0,
    #[serde(rename = "1")]
    X1,
    #[serde(rename = "2")]
    X2,
    #[serde(rename = "3")]
    X3,
    #[serde(rename = "4")]
    X4,
    #[serde(rename = "5")]
    X5,
}
impl ::std::convert::From<&Self> for CustomBackgroundBoostRef1 {
    fn from(value: &CustomBackgroundBoostRef1) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomBackgroundBoostRef1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X0 => f.write_str("0"),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::X4 => f.write_str("4"),
            Self::X5 => f.write_str("5"),
        }
    }
}
impl ::std::str::FromStr for CustomBackgroundBoostRef1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "0" => Ok(Self::X0),
            "1" => Ok(Self::X1),
            "2" => Ok(Self::X2),
            "3" => Ok(Self::X3),
            "4" => Ok(Self::X4),
            "5" => Ok(Self::X5),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomBackgroundBoostRef1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundBoostRef1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundBoostRef1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Second ability boost reference
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Second ability boost reference",
///  "type": "string",
///  "enum": [
///    "0",
///    "1",
///    "2",
///    "3",
///    "4",
///    "5"
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
pub enum CustomBackgroundBoostRef2 {
    #[serde(rename = "0")]
    X0,
    #[serde(rename = "1")]
    X1,
    #[serde(rename = "2")]
    X2,
    #[serde(rename = "3")]
    X3,
    #[serde(rename = "4")]
    X4,
    #[serde(rename = "5")]
    X5,
}
impl ::std::convert::From<&Self> for CustomBackgroundBoostRef2 {
    fn from(value: &CustomBackgroundBoostRef2) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomBackgroundBoostRef2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X0 => f.write_str("0"),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::X4 => f.write_str("4"),
            Self::X5 => f.write_str("5"),
        }
    }
}
impl ::std::str::FromStr for CustomBackgroundBoostRef2 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "0" => Ok(Self::X0),
            "1" => Ok(Self::X1),
            "2" => Ok(Self::X2),
            "3" => Ok(Self::X3),
            "4" => Ok(Self::X4),
            "5" => Ok(Self::X5),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomBackgroundBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Background description
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Background description",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomBackgroundDescription(::std::string::String);
impl ::std::ops::Deref for CustomBackgroundDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomBackgroundDescription> for ::std::string::String {
    fn from(value: CustomBackgroundDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomBackgroundDescription> for CustomBackgroundDescription {
    fn from(value: &CustomBackgroundDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomBackgroundDescription {
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
impl ::std::convert::TryFrom<&str> for CustomBackgroundDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomBackgroundDescription {
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
///Unique background ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique background ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomBackgroundId(::std::string::String);
impl ::std::ops::Deref for CustomBackgroundId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomBackgroundId> for ::std::string::String {
    fn from(value: CustomBackgroundId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomBackgroundId> for CustomBackgroundId {
    fn from(value: &CustomBackgroundId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomBackgroundId {
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
impl ::std::convert::TryFrom<&str> for CustomBackgroundId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomBackgroundId {
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
///Background name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Background name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomBackgroundName(::std::string::String);
impl ::std::ops::Deref for CustomBackgroundName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomBackgroundName> for ::std::string::String {
    fn from(value: CustomBackgroundName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomBackgroundName> for CustomBackgroundName {
    fn from(value: &CustomBackgroundName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomBackgroundName {
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
impl ::std::convert::TryFrom<&str> for CustomBackgroundName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomBackgroundName {
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
pub struct CustomBackgroundTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomBackgroundTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomBackgroundTimestamp> for ::std::string::String {
    fn from(value: CustomBackgroundTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomBackgroundTimestamp> for CustomBackgroundTimestamp {
    fn from(value: &CustomBackgroundTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomBackgroundTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomBackgroundTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomBackgroundTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomBackgroundTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomBackgroundTimestamp {
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
    pub struct CustomBackground {
        boost_ref_1: ::std::result::Result<
            super::CustomBackgroundBoostRef1,
            ::std::string::String,
        >,
        boost_ref_2: ::std::result::Result<
            super::CustomBackgroundBoostRef2,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomBackgroundDescription,
            ::std::string::String,
        >,
        free_feat_detail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        free_feat_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomBackgroundId, ::std::string::String>,
        lore: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::CustomBackgroundName, ::std::string::String>,
        skill: ::std::result::Result<::std::string::String, ::std::string::String>,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<
            super::CustomBackgroundTimestamp,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CustomBackground {
        fn default() -> Self {
            Self {
                boost_ref_1: Err("no value supplied for boost_ref_1".to_string()),
                boost_ref_2: Err("no value supplied for boost_ref_2".to_string()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                free_feat_detail: Ok(Default::default()),
                free_feat_id: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                lore: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                skill: Err("no value supplied for skill".to_string()),
                src: Err("no value supplied for src".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traits: Err("no value supplied for traits".to_string()),
            }
        }
    }
    impl CustomBackground {
        pub fn boost_ref_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomBackgroundBoostRef1>,
            T::Error: ::std::fmt::Display,
        {
            self.boost_ref_1 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for boost_ref_1: {}", e)
                });
            self
        }
        pub fn boost_ref_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomBackgroundBoostRef2>,
            T::Error: ::std::fmt::Display,
        {
            self.boost_ref_2 = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for boost_ref_2: {}", e)
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
            T: ::std::convert::TryInto<super::CustomBackgroundDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn free_feat_detail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.free_feat_detail = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for free_feat_detail: {}", e
                    )
                });
            self
        }
        pub fn free_feat_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.free_feat_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for free_feat_id: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomBackgroundId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn lore<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.lore = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lore: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomBackgroundName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn skill<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.skill = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for skill: {}", e)
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
            T: ::std::convert::TryInto<super::CustomBackgroundTimestamp>,
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
    }
    impl ::std::convert::TryFrom<CustomBackground> for super::CustomBackground {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomBackground,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                boost_ref_1: value.boost_ref_1?,
                boost_ref_2: value.boost_ref_2?,
                database_id: value.database_id?,
                description: value.description?,
                free_feat_detail: value.free_feat_detail?,
                free_feat_id: value.free_feat_id?,
                id: value.id?,
                lore: value.lore?,
                name: value.name?,
                skill: value.skill?,
                src: value.src?,
                timestamp: value.timestamp?,
                traits: value.traits?,
            })
        }
    }
    impl ::std::convert::From<super::CustomBackground> for CustomBackground {
        fn from(value: super::CustomBackground) -> Self {
            Self {
                boost_ref_1: Ok(value.boost_ref_1),
                boost_ref_2: Ok(value.boost_ref_2),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                free_feat_detail: Ok(value.free_feat_detail),
                free_feat_id: Ok(value.free_feat_id),
                id: Ok(value.id),
                lore: Ok(value.lore),
                name: Ok(value.name),
                skill: Ok(value.skill),
                src: Ok(value.src),
                timestamp: Ok(value.timestamp),
                traits: Ok(value.traits),
            }
        }
    }
}
