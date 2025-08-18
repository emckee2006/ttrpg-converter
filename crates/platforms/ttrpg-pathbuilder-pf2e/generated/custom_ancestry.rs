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
}
