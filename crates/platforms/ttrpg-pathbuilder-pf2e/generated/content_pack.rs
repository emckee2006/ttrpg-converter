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
///Extracted custom_ancestry definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Ancestry",
///  "description": "Extracted custom_ancestry definition",
///  "type": "object",
///  "required": [
///    "databaseID",
///    "description",
///    "hp",
///    "id",
///    "name",
///    "size",
///    "speed",
///    "src",
///    "timestamp",
///    "traits"
///  ],
///  "properties": {
///    "boost_ref_1": {
///      "description": "First ability boost reference",
///      "type": "string",
///      "enum": [
///        "0",
///        "1",
///        "2",
///        "3",
///        "4",
///        "5",
///        "FREE"
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
///        "5",
///        "FREE"
///      ]
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Ancestry description",
///      "type": "string",
///      "minLength": 1
///    },
///    "flaw_ref": {
///      "description": "Ability flaw reference (empty if none)",
///      "type": "string",
///      "enum": [
///        "0",
///        "1",
///        "2",
///        "3",
///        "4",
///        "5",
///        ""
///      ]
///    },
///    "hp": {
///      "description": "Base hit points",
///      "type": "integer",
///      "maximum": 12.0,
///      "minimum": 4.0
///    },
///    "id": {
///      "description": "Unique ancestry ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "languages": {
///      "description": "Starting languages",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "name": {
///      "description": "Ancestry name",
///      "type": "string",
///      "minLength": 1
///    },
///    "senses": {
///      "description": "Special senses",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "size": {
///      "description": "Size category: 1=Small, 2=Medium, 3=Large, 4=Huge",
///      "type": "integer",
///      "enum": [
///        1,
///        2,
///        3,
///        4
///      ]
///    },
///    "speed": {
///      "description": "Base speed in feet",
///      "type": "integer",
///      "minimum": 5.0
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
///      "description": "Ancestry traits (comma-separated)",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomAncestry {
    ///First ability boost reference
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boost_ref_1: ::std::option::Option<CustomAncestryBoostRef1>,
    ///Second ability boost reference
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub boost_ref_2: ::std::option::Option<CustomAncestryBoostRef2>,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Ancestry description
    pub description: CustomAncestryDescription,
    ///Ability flaw reference (empty if none)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub flaw_ref: ::std::option::Option<CustomAncestryFlawRef>,
    ///Base hit points
    pub hp: i64,
    ///Unique ancestry ID (UUID)
    pub id: CustomAncestryId,
    ///Starting languages
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<::std::string::String>,
    ///Ancestry name
    pub name: CustomAncestryName,
    ///Special senses
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub senses: ::std::vec::Vec<::std::string::String>,
    ///Size category: 1=Small, 2=Medium, 3=Large, 4=Huge
    pub size: CustomAncestrySize,
    ///Base speed in feet
    pub speed: i64,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Creation timestamp
    pub timestamp: CustomAncestryTimestamp,
    ///Ancestry traits (comma-separated)
    pub traits: ::std::string::String,
}
impl ::std::convert::From<&CustomAncestry> for CustomAncestry {
    fn from(value: &CustomAncestry) -> Self {
        value.clone()
    }
}
impl CustomAncestry {
    pub fn builder() -> builder::CustomAncestry {
        Default::default()
    }
}
///First ability boost reference
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "First ability boost reference",
///  "type": "string",
///  "enum": [
///    "0",
///    "1",
///    "2",
///    "3",
///    "4",
///    "5",
///    "FREE"
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
pub enum CustomAncestryBoostRef1 {
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
    #[serde(rename = "FREE")]
    Free,
}
impl ::std::convert::From<&Self> for CustomAncestryBoostRef1 {
    fn from(value: &CustomAncestryBoostRef1) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomAncestryBoostRef1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X0 => f.write_str("0"),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::X4 => f.write_str("4"),
            Self::X5 => f.write_str("5"),
            Self::Free => f.write_str("FREE"),
        }
    }
}
impl ::std::str::FromStr for CustomAncestryBoostRef1 {
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
            "FREE" => Ok(Self::Free),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomAncestryBoostRef1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryBoostRef1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryBoostRef1 {
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
///    "5",
///    "FREE"
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
pub enum CustomAncestryBoostRef2 {
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
    #[serde(rename = "FREE")]
    Free,
}
impl ::std::convert::From<&Self> for CustomAncestryBoostRef2 {
    fn from(value: &CustomAncestryBoostRef2) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomAncestryBoostRef2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X0 => f.write_str("0"),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::X4 => f.write_str("4"),
            Self::X5 => f.write_str("5"),
            Self::Free => f.write_str("FREE"),
        }
    }
}
impl ::std::str::FromStr for CustomAncestryBoostRef2 {
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
            "FREE" => Ok(Self::Free),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomAncestryBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryBoostRef2 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Ancestry description
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Ancestry description",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomAncestryDescription(::std::string::String);
impl ::std::ops::Deref for CustomAncestryDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomAncestryDescription> for ::std::string::String {
    fn from(value: CustomAncestryDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomAncestryDescription> for CustomAncestryDescription {
    fn from(value: &CustomAncestryDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomAncestryDescription {
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
impl ::std::convert::TryFrom<&str> for CustomAncestryDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomAncestryDescription {
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
///Ability flaw reference (empty if none)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Ability flaw reference (empty if none)",
///  "type": "string",
///  "enum": [
///    "0",
///    "1",
///    "2",
///    "3",
///    "4",
///    "5",
///    ""
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
pub enum CustomAncestryFlawRef {
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
    #[serde(rename = "")]
    X,
}
impl ::std::convert::From<&Self> for CustomAncestryFlawRef {
    fn from(value: &CustomAncestryFlawRef) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomAncestryFlawRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X0 => f.write_str("0"),
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X3 => f.write_str("3"),
            Self::X4 => f.write_str("4"),
            Self::X5 => f.write_str("5"),
            Self::X => f.write_str(""),
        }
    }
}
impl ::std::str::FromStr for CustomAncestryFlawRef {
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
            "" => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomAncestryFlawRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryFlawRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryFlawRef {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Unique ancestry ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique ancestry ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomAncestryId(::std::string::String);
impl ::std::ops::Deref for CustomAncestryId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomAncestryId> for ::std::string::String {
    fn from(value: CustomAncestryId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomAncestryId> for CustomAncestryId {
    fn from(value: &CustomAncestryId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomAncestryId {
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
impl ::std::convert::TryFrom<&str> for CustomAncestryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomAncestryId {
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
///Ancestry name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Ancestry name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomAncestryName(::std::string::String);
impl ::std::ops::Deref for CustomAncestryName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomAncestryName> for ::std::string::String {
    fn from(value: CustomAncestryName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomAncestryName> for CustomAncestryName {
    fn from(value: &CustomAncestryName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomAncestryName {
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
impl ::std::convert::TryFrom<&str> for CustomAncestryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomAncestryName {
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
///Size category: 1=Small, 2=Medium, 3=Large, 4=Huge
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Size category: 1=Small, 2=Medium, 3=Large, 4=Huge",
///  "type": "integer",
///  "enum": [
///    1,
///    2,
///    3,
///    4
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CustomAncestrySize(i64);
impl ::std::ops::Deref for CustomAncestrySize {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<CustomAncestrySize> for i64 {
    fn from(value: CustomAncestrySize) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomAncestrySize> for CustomAncestrySize {
    fn from(value: &CustomAncestrySize) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for CustomAncestrySize {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64, 3_i64, 4_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomAncestrySize {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
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
pub struct CustomAncestryTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomAncestryTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomAncestryTimestamp> for ::std::string::String {
    fn from(value: CustomAncestryTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomAncestryTimestamp> for CustomAncestryTimestamp {
    fn from(value: &CustomAncestryTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomAncestryTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomAncestryTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomAncestryTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomAncestryTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomAncestryTimestamp {
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
///Extracted custom_heritage definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Heritage",
///  "description": "Extracted custom_heritage definition",
///  "type": "object",
///  "required": [
///    "ancestryID",
///    "databaseID",
///    "description",
///    "id",
///    "name",
///    "src",
///    "timestamp"
///  ],
///  "properties": {
///    "ancestryID": {
///      "description": "Parent ancestry ID",
///      "type": "string"
///    },
///    "databaseID": {
///      "type": "integer"
///    },
///    "description": {
///      "description": "Heritage description and abilities",
///      "type": "string",
///      "minLength": 1
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "name": {
///      "description": "Heritage name",
///      "type": "string",
///      "minLength": 1
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "string",
///      "pattern": "^\\d+$"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomHeritage {
    ///Parent ancestry ID
    #[serde(rename = "ancestryID")]
    pub ancestry_id: ::std::string::String,
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Heritage description and abilities
    pub description: CustomHeritageDescription,
    pub id: CustomHeritageId,
    ///Heritage name
    pub name: CustomHeritageName,
    ///Source book/pack
    pub src: ::std::string::String,
    pub timestamp: CustomHeritageTimestamp,
}
impl ::std::convert::From<&CustomHeritage> for CustomHeritage {
    fn from(value: &CustomHeritage) -> Self {
        value.clone()
    }
}
impl CustomHeritage {
    pub fn builder() -> builder::CustomHeritage {
        Default::default()
    }
}
///Heritage description and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Heritage description and abilities",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageDescription(::std::string::String);
impl ::std::ops::Deref for CustomHeritageDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageDescription> for ::std::string::String {
    fn from(value: CustomHeritageDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageDescription> for CustomHeritageDescription {
    fn from(value: &CustomHeritageDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageDescription {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageDescription {
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
///`CustomHeritageId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageId(::std::string::String);
impl ::std::ops::Deref for CustomHeritageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageId> for ::std::string::String {
    fn from(value: CustomHeritageId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageId> for CustomHeritageId {
    fn from(value: &CustomHeritageId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageId {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageId {
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
///Heritage name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Heritage name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageName(::std::string::String);
impl ::std::ops::Deref for CustomHeritageName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageName> for ::std::string::String {
    fn from(value: CustomHeritageName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageName> for CustomHeritageName {
    fn from(value: &CustomHeritageName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageName {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageName {
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
///`CustomHeritageTimestamp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomHeritageTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageTimestamp> for ::std::string::String {
    fn from(value: CustomHeritageTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageTimestamp> for CustomHeritageTimestamp {
    fn from(value: &CustomHeritageTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageTimestamp {
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
///Extracted custom_item definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Item",
///  "description": "Extracted custom_item definition",
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
///    "ac": {
///      "description": "Armor AC bonus",
///      "type": "integer"
///    },
///    "activate": {
///      "description": "Activation method",
///      "type": "string"
///    },
///    "armorGroup": {
///      "description": "Armor group classification",
///      "type": "string"
///    },
///    "bulk": {
///      "description": "Item bulk (L, 1, 2, etc.)",
///      "type": "string"
///    },
///    "category": {
///      "description": "Item category (weapon, armor, consumable, etc.)",
///      "type": "string"
///    },
///    "checkPenalty": {
///      "description": "Armor check penalty",
///      "type": "integer"
///    },
///    "damage": {
///      "description": "Weapon damage dice",
///      "type": "string"
///    },
///    "damageType": {
///      "description": "Damage type (piercing, slashing, etc.)",
///      "type": "string"
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Item description and abilities",
///      "type": "string",
///      "minLength": 1
///    },
///    "dexCap": {
///      "description": "Maximum Dex bonus for armor",
///      "type": "integer"
///    },
///    "hands": {
///      "description": "Number of hands required",
///      "type": "string",
///      "enum": [
///        "1",
///        "2",
///        "1+"
///      ]
///    },
///    "id": {
///      "description": "Unique item ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "level": {
///      "description": "Item level",
///      "type": "integer",
///      "maximum": 25.0,
///      "minimum": 0.0
///    },
///    "name": {
///      "description": "Item name",
///      "type": "string",
///      "minLength": 1
///    },
///    "price": {
///      "description": "Item price in gold/silver/copper",
///      "type": "string"
///    },
///    "range": {
///      "description": "Weapon range increment",
///      "type": "string"
///    },
///    "reload": {
///      "description": "Reload time for ranged weapons",
///      "type": "string"
///    },
///    "speedPenalty": {
///      "description": "Speed penalty from armor",
///      "type": "integer"
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "strength": {
///      "description": "Required Strength score",
///      "type": "integer"
///    },
///    "timestamp": {
///      "description": "Creation timestamp",
///      "type": "string",
///      "pattern": "^\\d+$"
///    },
///    "traits": {
///      "description": "Item traits (comma-separated)",
///      "type": "string"
///    },
///    "usage": {
///      "description": "Item usage/activation",
///      "type": "string"
///    },
///    "weaponGroup": {
///      "description": "Weapon group classification",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomItem {
    ///Armor AC bonus
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ac: ::std::option::Option<i64>,
    ///Activation method
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub activate: ::std::option::Option<::std::string::String>,
    ///Armor group classification
    #[serde(
        rename = "armorGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub armor_group: ::std::option::Option<::std::string::String>,
    ///Item bulk (L, 1, 2, etc.)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bulk: ::std::option::Option<::std::string::String>,
    ///Item category (weapon, armor, consumable, etc.)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    ///Armor check penalty
    #[serde(
        rename = "checkPenalty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub check_penalty: ::std::option::Option<i64>,
    ///Weapon damage dice
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub damage: ::std::option::Option<::std::string::String>,
    ///Damage type (piercing, slashing, etc.)
    #[serde(
        rename = "damageType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub damage_type: ::std::option::Option<::std::string::String>,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Item description and abilities
    pub description: CustomItemDescription,
    ///Maximum Dex bonus for armor
    #[serde(
        rename = "dexCap",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dex_cap: ::std::option::Option<i64>,
    ///Number of hands required
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hands: ::std::option::Option<CustomItemHands>,
    ///Unique item ID (UUID)
    pub id: CustomItemId,
    ///Item level
    pub level: i64,
    ///Item name
    pub name: CustomItemName,
    ///Item price in gold/silver/copper
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::std::string::String>,
    ///Weapon range increment
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<::std::string::String>,
    ///Reload time for ranged weapons
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reload: ::std::option::Option<::std::string::String>,
    ///Speed penalty from armor
    #[serde(
        rename = "speedPenalty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_penalty: ::std::option::Option<i64>,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Required Strength score
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub strength: ::std::option::Option<i64>,
    ///Creation timestamp
    pub timestamp: CustomItemTimestamp,
    ///Item traits (comma-separated)
    pub traits: ::std::string::String,
    ///Item usage/activation
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<::std::string::String>,
    ///Weapon group classification
    #[serde(
        rename = "weaponGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weapon_group: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CustomItem> for CustomItem {
    fn from(value: &CustomItem) -> Self {
        value.clone()
    }
}
impl CustomItem {
    pub fn builder() -> builder::CustomItem {
        Default::default()
    }
}
///Item description and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Item description and abilities",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemDescription(::std::string::String);
impl ::std::ops::Deref for CustomItemDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemDescription> for ::std::string::String {
    fn from(value: CustomItemDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemDescription> for CustomItemDescription {
    fn from(value: &CustomItemDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemDescription {
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
impl ::std::convert::TryFrom<&str> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemDescription {
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
///Number of hands required
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Number of hands required",
///  "type": "string",
///  "enum": [
///    "1",
///    "2",
///    "1+"
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
pub enum CustomItemHands {
    #[serde(rename = "1")]
    X1,
    #[serde(rename = "2")]
    X2,
    #[serde(rename = "1+")]
    X1x,
}
impl ::std::convert::From<&Self> for CustomItemHands {
    fn from(value: &CustomItemHands) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomItemHands {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X1x => f.write_str("1+"),
        }
    }
}
impl ::std::str::FromStr for CustomItemHands {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1" => Ok(Self::X1),
            "2" => Ok(Self::X2),
            "1+" => Ok(Self::X1x),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Unique item ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique item ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemId(::std::string::String);
impl ::std::ops::Deref for CustomItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemId> for ::std::string::String {
    fn from(value: CustomItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemId> for CustomItemId {
    fn from(value: &CustomItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemId {
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
impl ::std::convert::TryFrom<&str> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemId {
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
///Item name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Item name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemName(::std::string::String);
impl ::std::ops::Deref for CustomItemName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemName> for ::std::string::String {
    fn from(value: CustomItemName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemName> for CustomItemName {
    fn from(value: &CustomItemName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemName {
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
impl ::std::convert::TryFrom<&str> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemName {
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
pub struct CustomItemTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomItemTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemTimestamp> for ::std::string::String {
    fn from(value: CustomItemTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemTimestamp> for CustomItemTimestamp {
    fn from(value: &CustomItemTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemTimestamp {
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
///Schema for Pathbuilder 2e custom content pack JSON files
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Pathbuilder Custom Content Pack",
///  "description": "Schema for Pathbuilder 2e custom content pack JSON files",
///  "type": "object",
///  "required": [
///    "customPackID",
///    "customPackName"
///  ],
///  "properties": {
///    "customPackID": {
///      "description": "Unique pack identifier (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "customPackName": {
///      "description": "Display name for the content pack",
///      "type": "string",
///      "minLength": 1
///    },
///    "listCustomAncestries": {
///      "description": "Custom ancestries in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Ancestry",
///        "description": "Extracted custom_ancestry definition",
///        "type": "object",
///        "required": [
///          "databaseID",
///          "description",
///          "hp",
///          "id",
///          "name",
///          "size",
///          "speed",
///          "src",
///          "timestamp",
///          "traits"
///        ],
///        "properties": {
///          "boost_ref_1": {
///            "description": "First ability boost reference",
///            "type": "string",
///            "enum": [
///              "0",
///              "1",
///              "2",
///              "3",
///              "4",
///              "5",
///              "FREE"
///            ]
///          },
///          "boost_ref_2": {
///            "description": "Second ability boost reference",
///            "type": "string",
///            "enum": [
///              "0",
///              "1",
///              "2",
///              "3",
///              "4",
///              "5",
///              "FREE"
///            ]
///          },
///          "databaseID": {
///            "description": "Database identifier",
///            "type": "integer"
///          },
///          "description": {
///            "description": "Ancestry description",
///            "type": "string",
///            "minLength": 1
///          },
///          "flaw_ref": {
///            "description": "Ability flaw reference (empty if none)",
///            "type": "string",
///            "enum": [
///              "0",
///              "1",
///              "2",
///              "3",
///              "4",
///              "5",
///              ""
///            ]
///          },
///          "hp": {
///            "description": "Base hit points",
///            "type": "integer",
///            "maximum": 12.0,
///            "minimum": 4.0
///          },
///          "id": {
///            "description": "Unique ancestry ID (UUID)",
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "languages": {
///            "description": "Starting languages",
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "name": {
///            "description": "Ancestry name",
///            "type": "string",
///            "minLength": 1
///          },
///          "senses": {
///            "description": "Special senses",
///            "type": "array",
///            "items": {
///              "type": "string"
///            }
///          },
///          "size": {
///            "description": "Size category: 1=Small, 2=Medium, 3=Large, 4=Huge",
///            "type": "integer",
///            "enum": [
///              1,
///              2,
///              3,
///              4
///            ]
///          },
///          "speed": {
///            "description": "Base speed in feet",
///            "type": "integer",
///            "minimum": 5.0
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "timestamp": {
///            "description": "Creation timestamp",
///            "type": "string",
///            "pattern": "^\\d+$"
///          },
///          "traits": {
///            "description": "Ancestry traits (comma-separated)",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "listCustomArchetypes": {
///      "description": "Custom archetypes in this pack",
///      "type": "array"
///    },
///    "listCustomBackgrounds": {
///      "description": "Custom backgrounds in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Background",
///        "description": "Extracted custom_background definition",
///        "type": "object",
///        "required": [
///          "boost_ref_1",
///          "boost_ref_2",
///          "databaseID",
///          "description",
///          "id",
///          "name",
///          "skill",
///          "src",
///          "timestamp",
///          "traits"
///        ],
///        "properties": {
///          "boost_ref_1": {
///            "description": "First ability boost reference (0=Str, 1=Dex, 2=Con, 3=Int, 4=Wis, 5=Cha)",
///            "type": "string",
///            "enum": [
///              "0",
///              "1",
///              "2",
///              "3",
///              "4",
///              "5"
///            ]
///          },
///          "boost_ref_2": {
///            "description": "Second ability boost reference",
///            "type": "string",
///            "enum": [
///              "0",
///              "1",
///              "2",
///              "3",
///              "4",
///              "5"
///            ]
///          },
///          "databaseID": {
///            "description": "Database identifier",
///            "type": "integer"
///          },
///          "description": {
///            "description": "Background description",
///            "type": "string",
///            "minLength": 1
///          },
///          "freeFeatDetail": {
///            "description": "Additional details for free feat",
///            "type": "string"
///          },
///          "freeFeatID": {
///            "description": "Free feat granted by background",
///            "type": "string"
///          },
///          "id": {
///            "description": "Unique background ID (UUID)",
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "lore": {
///            "description": "Lore skill granted",
///            "type": "string"
///          },
///          "name": {
///            "description": "Background name",
///            "type": "string",
///            "minLength": 1
///          },
///          "skill": {
///            "description": "Skill proficiency granted",
///            "type": "string"
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "timestamp": {
///            "description": "Creation timestamp",
///            "type": "string",
///            "pattern": "^\\d+$"
///          },
///          "traits": {
///            "description": "Background traits (e.g., '3rd Party')",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "listCustomClasses": {
///      "description": "Custom classes in this pack",
///      "type": "array"
///    },
///    "listCustomFeats": {
///      "description": "Custom feats in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Feat",
///        "description": "Extracted custom_feat definition",
///        "type": "object",
///        "required": [
///          "databaseID",
///          "description",
///          "id",
///          "level",
///          "name",
///          "src",
///          "timestamp",
///          "traits"
///        ],
///        "properties": {
///          "actionType": {
///            "description": "Action type (empty=passive, 1-3=actions, R=reaction, F=free)",
///            "type": "string",
///            "enum": [
///              "",
///              "1",
///              "2",
///              "3",
///              "R",
///              "F"
///            ]
///          },
///          "databaseID": {
///            "description": "Database identifier",
///            "type": "integer"
///          },
///          "description": {
///            "description": "Feat description and effects",
///            "type": "string",
///            "minLength": 1
///          },
///          "frequency": {
///            "description": "Usage frequency",
///            "type": "string"
///          },
///          "id": {
///            "description": "Unique feat ID (UUID)",
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "level": {
///            "description": "Required level for feat",
///            "type": "integer",
///            "maximum": 20.0,
///            "minimum": 1.0
///          },
///          "name": {
///            "description": "Feat name",
///            "type": "string",
///            "minLength": 1
///          },
///          "prerequisites": {
///            "description": "Feat prerequisites",
///            "type": "string"
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "timestamp": {
///            "description": "Creation timestamp",
///            "type": "string",
///            "pattern": "^\\d+$"
///          },
///          "traits": {
///            "description": "Feat traits (comma-separated)",
///            "type": "string"
///          },
///          "trigger": {
///            "description": "Trigger condition",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "listCustomHeritages": {
///      "description": "Custom heritages in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Heritage",
///        "description": "Extracted custom_heritage definition",
///        "type": "object",
///        "required": [
///          "ancestryID",
///          "databaseID",
///          "description",
///          "id",
///          "name",
///          "src",
///          "timestamp"
///        ],
///        "properties": {
///          "ancestryID": {
///            "description": "Parent ancestry ID",
///            "type": "string"
///          },
///          "databaseID": {
///            "type": "integer"
///          },
///          "description": {
///            "description": "Heritage description and abilities",
///            "type": "string",
///            "minLength": 1
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "name": {
///            "description": "Heritage name",
///            "type": "string",
///            "minLength": 1
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "timestamp": {
///            "type": "string",
///            "pattern": "^\\d+$"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "listCustomItems": {
///      "description": "Custom items in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Item",
///        "description": "Extracted custom_item definition",
///        "type": "object",
///        "required": [
///          "databaseID",
///          "description",
///          "id",
///          "level",
///          "name",
///          "src",
///          "timestamp",
///          "traits"
///        ],
///        "properties": {
///          "ac": {
///            "description": "Armor AC bonus",
///            "type": "integer"
///          },
///          "activate": {
///            "description": "Activation method",
///            "type": "string"
///          },
///          "armorGroup": {
///            "description": "Armor group classification",
///            "type": "string"
///          },
///          "bulk": {
///            "description": "Item bulk (L, 1, 2, etc.)",
///            "type": "string"
///          },
///          "category": {
///            "description": "Item category (weapon, armor, consumable, etc.)",
///            "type": "string"
///          },
///          "checkPenalty": {
///            "description": "Armor check penalty",
///            "type": "integer"
///          },
///          "damage": {
///            "description": "Weapon damage dice",
///            "type": "string"
///          },
///          "damageType": {
///            "description": "Damage type (piercing, slashing, etc.)",
///            "type": "string"
///          },
///          "databaseID": {
///            "description": "Database identifier",
///            "type": "integer"
///          },
///          "description": {
///            "description": "Item description and abilities",
///            "type": "string",
///            "minLength": 1
///          },
///          "dexCap": {
///            "description": "Maximum Dex bonus for armor",
///            "type": "integer"
///          },
///          "hands": {
///            "description": "Number of hands required",
///            "type": "string",
///            "enum": [
///              "1",
///              "2",
///              "1+"
///            ]
///          },
///          "id": {
///            "description": "Unique item ID (UUID)",
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "level": {
///            "description": "Item level",
///            "type": "integer",
///            "maximum": 25.0,
///            "minimum": 0.0
///          },
///          "name": {
///            "description": "Item name",
///            "type": "string",
///            "minLength": 1
///          },
///          "price": {
///            "description": "Item price in gold/silver/copper",
///            "type": "string"
///          },
///          "range": {
///            "description": "Weapon range increment",
///            "type": "string"
///          },
///          "reload": {
///            "description": "Reload time for ranged weapons",
///            "type": "string"
///          },
///          "speedPenalty": {
///            "description": "Speed penalty from armor",
///            "type": "integer"
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "strength": {
///            "description": "Required Strength score",
///            "type": "integer"
///          },
///          "timestamp": {
///            "description": "Creation timestamp",
///            "type": "string",
///            "pattern": "^\\d+$"
///          },
///          "traits": {
///            "description": "Item traits (comma-separated)",
///            "type": "string"
///          },
///          "usage": {
///            "description": "Item usage/activation",
///            "type": "string"
///          },
///          "weaponGroup": {
///            "description": "Weapon group classification",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "listCustomSpells": {
///      "description": "Custom spells in this pack",
///      "type": "array",
///      "items": {
///        "title": "Custom Spell",
///        "description": "Extracted custom_spell definition",
///        "type": "object",
///        "required": [
///          "databaseID",
///          "description",
///          "id",
///          "level",
///          "name",
///          "src",
///          "timestamp",
///          "traits"
///        ],
///        "properties": {
///          "area": {
///            "description": "Area of effect",
///            "type": "string"
///          },
///          "cast": {
///            "description": "Casting time and actions",
///            "type": "string"
///          },
///          "components": {
///            "description": "Spell components (somatic, verbal, material)",
///            "type": "string"
///          },
///          "databaseID": {
///            "description": "Database identifier",
///            "type": "integer"
///          },
///          "description": {
///            "description": "Spell description and effects",
///            "type": "string",
///            "minLength": 1
///          },
///          "duration": {
///            "description": "Spell duration",
///            "type": "string"
///          },
///          "heightened": {
///            "description": "Heightened spell effects",
///            "type": "string"
///          },
///          "id": {
///            "description": "Unique spell ID (UUID)",
///            "type": "string",
///            "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///          },
///          "level": {
///            "description": "Spell level (0=cantrip)",
///            "type": "integer",
///            "maximum": 10.0,
///            "minimum": 0.0
///          },
///          "name": {
///            "description": "Spell name",
///            "type": "string",
///            "minLength": 1
///          },
///          "range": {
///            "description": "Spell range",
///            "type": "string"
///          },
///          "savingThrow": {
///            "description": "Required saving throw",
///            "type": "string"
///          },
///          "src": {
///            "description": "Source book/pack",
///            "type": "string"
///          },
///          "targets": {
///            "description": "Valid targets",
///            "type": "string"
///          },
///          "timestamp": {
///            "description": "Creation timestamp",
///            "type": "string",
///            "pattern": "^\\d+$"
///          },
///          "traditions": {
///            "description": "Magical traditions",
///            "type": "array",
///            "items": {
///              "type": "string",
///              "enum": [
///                "Arcane",
///                "Divine",
///                "Occult",
///                "Primal"
///              ]
///            }
///          },
///          "traits": {
///            "description": "Spell traits (comma-separated)",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PathbuilderCustomContentPack {
    ///Unique pack identifier (UUID)
    #[serde(rename = "customPackID")]
    pub custom_pack_id: PathbuilderCustomContentPackCustomPackId,
    ///Display name for the content pack
    #[serde(rename = "customPackName")]
    pub custom_pack_name: PathbuilderCustomContentPackCustomPackName,
    ///Custom ancestries in this pack
    #[serde(
        rename = "listCustomAncestries",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_ancestries: ::std::vec::Vec<CustomAncestry>,
    ///Custom archetypes in this pack
    #[serde(
        rename = "listCustomArchetypes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_archetypes: ::std::vec::Vec<::serde_json::Value>,
    ///Custom backgrounds in this pack
    #[serde(
        rename = "listCustomBackgrounds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_backgrounds: ::std::vec::Vec<CustomBackground>,
    ///Custom classes in this pack
    #[serde(
        rename = "listCustomClasses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_classes: ::std::vec::Vec<::serde_json::Value>,
    ///Custom feats in this pack
    #[serde(
        rename = "listCustomFeats",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_feats: ::std::vec::Vec<CustomFeat>,
    ///Custom heritages in this pack
    #[serde(
        rename = "listCustomHeritages",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_heritages: ::std::vec::Vec<CustomHeritage>,
    ///Custom items in this pack
    #[serde(
        rename = "listCustomItems",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_items: ::std::vec::Vec<CustomItem>,
    ///Custom spells in this pack
    #[serde(
        rename = "listCustomSpells",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub list_custom_spells: ::std::vec::Vec<CustomSpell>,
}
impl ::std::convert::From<&PathbuilderCustomContentPack>
for PathbuilderCustomContentPack {
    fn from(value: &PathbuilderCustomContentPack) -> Self {
        value.clone()
    }
}
impl PathbuilderCustomContentPack {
    pub fn builder() -> builder::PathbuilderCustomContentPack {
        Default::default()
    }
}
///Unique pack identifier (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique pack identifier (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PathbuilderCustomContentPackCustomPackId(::std::string::String);
impl ::std::ops::Deref for PathbuilderCustomContentPackCustomPackId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PathbuilderCustomContentPackCustomPackId>
for ::std::string::String {
    fn from(value: PathbuilderCustomContentPackCustomPackId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PathbuilderCustomContentPackCustomPackId>
for PathbuilderCustomContentPackCustomPackId {
    fn from(value: &PathbuilderCustomContentPackCustomPackId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PathbuilderCustomContentPackCustomPackId {
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
impl ::std::convert::TryFrom<&str> for PathbuilderCustomContentPackCustomPackId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PathbuilderCustomContentPackCustomPackId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PathbuilderCustomContentPackCustomPackId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PathbuilderCustomContentPackCustomPackId {
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
///Display name for the content pack
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Display name for the content pack",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PathbuilderCustomContentPackCustomPackName(::std::string::String);
impl ::std::ops::Deref for PathbuilderCustomContentPackCustomPackName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PathbuilderCustomContentPackCustomPackName>
for ::std::string::String {
    fn from(value: PathbuilderCustomContentPackCustomPackName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PathbuilderCustomContentPackCustomPackName>
for PathbuilderCustomContentPackCustomPackName {
    fn from(value: &PathbuilderCustomContentPackCustomPackName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PathbuilderCustomContentPackCustomPackName {
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
impl ::std::convert::TryFrom<&str> for PathbuilderCustomContentPackCustomPackName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PathbuilderCustomContentPackCustomPackName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PathbuilderCustomContentPackCustomPackName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PathbuilderCustomContentPackCustomPackName {
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
    pub struct CustomAncestry {
        boost_ref_1: ::std::result::Result<
            ::std::option::Option<super::CustomAncestryBoostRef1>,
            ::std::string::String,
        >,
        boost_ref_2: ::std::result::Result<
            ::std::option::Option<super::CustomAncestryBoostRef2>,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomAncestryDescription,
            ::std::string::String,
        >,
        flaw_ref: ::std::result::Result<
            ::std::option::Option<super::CustomAncestryFlawRef>,
            ::std::string::String,
        >,
        hp: ::std::result::Result<i64, ::std::string::String>,
        id: ::std::result::Result<super::CustomAncestryId, ::std::string::String>,
        languages: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::CustomAncestryName, ::std::string::String>,
        senses: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        size: ::std::result::Result<super::CustomAncestrySize, ::std::string::String>,
        speed: ::std::result::Result<i64, ::std::string::String>,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<
            super::CustomAncestryTimestamp,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CustomAncestry {
        fn default() -> Self {
            Self {
                boost_ref_1: Ok(Default::default()),
                boost_ref_2: Ok(Default::default()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                flaw_ref: Ok(Default::default()),
                hp: Err("no value supplied for hp".to_string()),
                id: Err("no value supplied for id".to_string()),
                languages: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                senses: Ok(Default::default()),
                size: Err("no value supplied for size".to_string()),
                speed: Err("no value supplied for speed".to_string()),
                src: Err("no value supplied for src".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traits: Err("no value supplied for traits".to_string()),
            }
        }
    }
    impl CustomAncestry {
        pub fn boost_ref_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomAncestryBoostRef1>,
            >,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomAncestryBoostRef2>,
            >,
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
            T: ::std::convert::TryInto<super::CustomAncestryDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn flaw_ref<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::CustomAncestryFlawRef>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.flaw_ref = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for flaw_ref: {}", e)
                });
            self
        }
        pub fn hp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomAncestryId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for languages: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomAncestryName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn senses<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.senses = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for senses: {}", e)
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomAncestrySize>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed: {}", e)
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
            T: ::std::convert::TryInto<super::CustomAncestryTimestamp>,
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
    impl ::std::convert::TryFrom<CustomAncestry> for super::CustomAncestry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomAncestry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                boost_ref_1: value.boost_ref_1?,
                boost_ref_2: value.boost_ref_2?,
                database_id: value.database_id?,
                description: value.description?,
                flaw_ref: value.flaw_ref?,
                hp: value.hp?,
                id: value.id?,
                languages: value.languages?,
                name: value.name?,
                senses: value.senses?,
                size: value.size?,
                speed: value.speed?,
                src: value.src?,
                timestamp: value.timestamp?,
                traits: value.traits?,
            })
        }
    }
    impl ::std::convert::From<super::CustomAncestry> for CustomAncestry {
        fn from(value: super::CustomAncestry) -> Self {
            Self {
                boost_ref_1: Ok(value.boost_ref_1),
                boost_ref_2: Ok(value.boost_ref_2),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                flaw_ref: Ok(value.flaw_ref),
                hp: Ok(value.hp),
                id: Ok(value.id),
                languages: Ok(value.languages),
                name: Ok(value.name),
                senses: Ok(value.senses),
                size: Ok(value.size),
                speed: Ok(value.speed),
                src: Ok(value.src),
                timestamp: Ok(value.timestamp),
                traits: Ok(value.traits),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct CustomHeritage {
        ancestry_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomHeritageDescription,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomHeritageId, ::std::string::String>,
        name: ::std::result::Result<super::CustomHeritageName, ::std::string::String>,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<
            super::CustomHeritageTimestamp,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomHeritage {
        fn default() -> Self {
            Self {
                ancestry_id: Err("no value supplied for ancestry_id".to_string()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                src: Err("no value supplied for src".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl CustomHeritage {
        pub fn ancestry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ancestry_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ancestry_id: {}", e)
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
            T: ::std::convert::TryInto<super::CustomHeritageDescription>,
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
            T: ::std::convert::TryInto<super::CustomHeritageId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomHeritageName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
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
            T: ::std::convert::TryInto<super::CustomHeritageTimestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomHeritage> for super::CustomHeritage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomHeritage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ancestry_id: value.ancestry_id?,
                database_id: value.database_id?,
                description: value.description?,
                id: value.id?,
                name: value.name?,
                src: value.src?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::CustomHeritage> for CustomHeritage {
        fn from(value: super::CustomHeritage) -> Self {
            Self {
                ancestry_id: Ok(value.ancestry_id),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                src: Ok(value.src),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CustomItem {
        ac: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        activate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        armor_group: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        bulk: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        category: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        check_penalty: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        damage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        damage_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomItemDescription,
            ::std::string::String,
        >,
        dex_cap: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        hands: ::std::result::Result<
            ::std::option::Option<super::CustomItemHands>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomItemId, ::std::string::String>,
        level: ::std::result::Result<i64, ::std::string::String>,
        name: ::std::result::Result<super::CustomItemName, ::std::string::String>,
        price: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        range: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reload: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        speed_penalty: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        strength: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        timestamp: ::std::result::Result<
            super::CustomItemTimestamp,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        weapon_group: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomItem {
        fn default() -> Self {
            Self {
                ac: Ok(Default::default()),
                activate: Ok(Default::default()),
                armor_group: Ok(Default::default()),
                bulk: Ok(Default::default()),
                category: Ok(Default::default()),
                check_penalty: Ok(Default::default()),
                damage: Ok(Default::default()),
                damage_type: Ok(Default::default()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                dex_cap: Ok(Default::default()),
                hands: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                level: Err("no value supplied for level".to_string()),
                name: Err("no value supplied for name".to_string()),
                price: Ok(Default::default()),
                range: Ok(Default::default()),
                reload: Ok(Default::default()),
                speed_penalty: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                strength: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traits: Err("no value supplied for traits".to_string()),
                usage: Ok(Default::default()),
                weapon_group: Ok(Default::default()),
            }
        }
    }
    impl CustomItem {
        pub fn ac<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ac = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ac: {}", e));
            self
        }
        pub fn activate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.activate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for activate: {}", e)
                });
            self
        }
        pub fn armor_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.armor_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for armor_group: {}", e)
                });
            self
        }
        pub fn bulk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bulk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bulk: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn check_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.check_penalty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for check_penalty: {}", e)
                });
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for damage: {}", e)
                });
            self
        }
        pub fn damage_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.damage_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for damage_type: {}", e)
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
            T: ::std::convert::TryInto<super::CustomItemDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn dex_cap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dex_cap = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dex_cap: {}", e)
                });
            self
        }
        pub fn hands<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CustomItemHands>>,
            T::Error: ::std::fmt::Display,
        {
            self.hands = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hands: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemId>,
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
            T: ::std::convert::TryInto<super::CustomItemName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for price: {}", e)
                });
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
        pub fn reload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reload: {}", e)
                });
            self
        }
        pub fn speed_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed_penalty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed_penalty: {}", e)
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
        pub fn strength<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.strength = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for strength: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemTimestamp>,
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
        pub fn usage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for usage: {}", e)
                });
            self
        }
        pub fn weapon_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.weapon_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for weapon_group: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomItem> for super::CustomItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ac: value.ac?,
                activate: value.activate?,
                armor_group: value.armor_group?,
                bulk: value.bulk?,
                category: value.category?,
                check_penalty: value.check_penalty?,
                damage: value.damage?,
                damage_type: value.damage_type?,
                database_id: value.database_id?,
                description: value.description?,
                dex_cap: value.dex_cap?,
                hands: value.hands?,
                id: value.id?,
                level: value.level?,
                name: value.name?,
                price: value.price?,
                range: value.range?,
                reload: value.reload?,
                speed_penalty: value.speed_penalty?,
                src: value.src?,
                strength: value.strength?,
                timestamp: value.timestamp?,
                traits: value.traits?,
                usage: value.usage?,
                weapon_group: value.weapon_group?,
            })
        }
    }
    impl ::std::convert::From<super::CustomItem> for CustomItem {
        fn from(value: super::CustomItem) -> Self {
            Self {
                ac: Ok(value.ac),
                activate: Ok(value.activate),
                armor_group: Ok(value.armor_group),
                bulk: Ok(value.bulk),
                category: Ok(value.category),
                check_penalty: Ok(value.check_penalty),
                damage: Ok(value.damage),
                damage_type: Ok(value.damage_type),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                dex_cap: Ok(value.dex_cap),
                hands: Ok(value.hands),
                id: Ok(value.id),
                level: Ok(value.level),
                name: Ok(value.name),
                price: Ok(value.price),
                range: Ok(value.range),
                reload: Ok(value.reload),
                speed_penalty: Ok(value.speed_penalty),
                src: Ok(value.src),
                strength: Ok(value.strength),
                timestamp: Ok(value.timestamp),
                traits: Ok(value.traits),
                usage: Ok(value.usage),
                weapon_group: Ok(value.weapon_group),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct PathbuilderCustomContentPack {
        custom_pack_id: ::std::result::Result<
            super::PathbuilderCustomContentPackCustomPackId,
            ::std::string::String,
        >,
        custom_pack_name: ::std::result::Result<
            super::PathbuilderCustomContentPackCustomPackName,
            ::std::string::String,
        >,
        list_custom_ancestries: ::std::result::Result<
            ::std::vec::Vec<super::CustomAncestry>,
            ::std::string::String,
        >,
        list_custom_archetypes: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        list_custom_backgrounds: ::std::result::Result<
            ::std::vec::Vec<super::CustomBackground>,
            ::std::string::String,
        >,
        list_custom_classes: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        list_custom_feats: ::std::result::Result<
            ::std::vec::Vec<super::CustomFeat>,
            ::std::string::String,
        >,
        list_custom_heritages: ::std::result::Result<
            ::std::vec::Vec<super::CustomHeritage>,
            ::std::string::String,
        >,
        list_custom_items: ::std::result::Result<
            ::std::vec::Vec<super::CustomItem>,
            ::std::string::String,
        >,
        list_custom_spells: ::std::result::Result<
            ::std::vec::Vec<super::CustomSpell>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PathbuilderCustomContentPack {
        fn default() -> Self {
            Self {
                custom_pack_id: Err("no value supplied for custom_pack_id".to_string()),
                custom_pack_name: Err(
                    "no value supplied for custom_pack_name".to_string(),
                ),
                list_custom_ancestries: Ok(Default::default()),
                list_custom_archetypes: Ok(Default::default()),
                list_custom_backgrounds: Ok(Default::default()),
                list_custom_classes: Ok(Default::default()),
                list_custom_feats: Ok(Default::default()),
                list_custom_heritages: Ok(Default::default()),
                list_custom_items: Ok(Default::default()),
                list_custom_spells: Ok(Default::default()),
            }
        }
    }
    impl PathbuilderCustomContentPack {
        pub fn custom_pack_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PathbuilderCustomContentPackCustomPackId>,
            T::Error: ::std::fmt::Display,
        {
            self.custom_pack_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom_pack_id: {}", e)
                });
            self
        }
        pub fn custom_pack_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::PathbuilderCustomContentPackCustomPackName,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.custom_pack_name = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for custom_pack_name: {}", e
                    )
                });
            self
        }
        pub fn list_custom_ancestries<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomAncestry>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_ancestries = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_ancestries: {}",
                        e
                    )
                });
            self
        }
        pub fn list_custom_archetypes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_archetypes = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_archetypes: {}",
                        e
                    )
                });
            self
        }
        pub fn list_custom_backgrounds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomBackground>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_backgrounds = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_backgrounds: {}",
                        e
                    )
                });
            self
        }
        pub fn list_custom_classes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_classes = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_classes: {}", e
                    )
                });
            self
        }
        pub fn list_custom_feats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomFeat>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_feats = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_feats: {}", e
                    )
                });
            self
        }
        pub fn list_custom_heritages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomHeritage>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_heritages = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_heritages: {}",
                        e
                    )
                });
            self
        }
        pub fn list_custom_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_items = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_items: {}", e
                    )
                });
            self
        }
        pub fn list_custom_spells<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CustomSpell>>,
            T::Error: ::std::fmt::Display,
        {
            self.list_custom_spells = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for list_custom_spells: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<PathbuilderCustomContentPack>
    for super::PathbuilderCustomContentPack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PathbuilderCustomContentPack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom_pack_id: value.custom_pack_id?,
                custom_pack_name: value.custom_pack_name?,
                list_custom_ancestries: value.list_custom_ancestries?,
                list_custom_archetypes: value.list_custom_archetypes?,
                list_custom_backgrounds: value.list_custom_backgrounds?,
                list_custom_classes: value.list_custom_classes?,
                list_custom_feats: value.list_custom_feats?,
                list_custom_heritages: value.list_custom_heritages?,
                list_custom_items: value.list_custom_items?,
                list_custom_spells: value.list_custom_spells?,
            })
        }
    }
    impl ::std::convert::From<super::PathbuilderCustomContentPack>
    for PathbuilderCustomContentPack {
        fn from(value: super::PathbuilderCustomContentPack) -> Self {
            Self {
                custom_pack_id: Ok(value.custom_pack_id),
                custom_pack_name: Ok(value.custom_pack_name),
                list_custom_ancestries: Ok(value.list_custom_ancestries),
                list_custom_archetypes: Ok(value.list_custom_archetypes),
                list_custom_backgrounds: Ok(value.list_custom_backgrounds),
                list_custom_classes: Ok(value.list_custom_classes),
                list_custom_feats: Ok(value.list_custom_feats),
                list_custom_heritages: Ok(value.list_custom_heritages),
                list_custom_items: Ok(value.list_custom_items),
                list_custom_spells: Ok(value.list_custom_spells),
            }
        }
    }
}
