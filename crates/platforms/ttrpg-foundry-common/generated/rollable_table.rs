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
///Dice formula (e.g., '1d20+5', '2d6', '10')
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Dice Formula",
///  "description": "Dice formula (e.g., '1d20+5', '2d6', '10')",
///  "type": "string",
///  "pattern": "^\\d*d\\d+([+-]\\d+)?$|^\\d+$",
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DiceFormula(::std::string::String);
impl ::std::ops::Deref for DiceFormula {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DiceFormula> for ::std::string::String {
    fn from(value: DiceFormula) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DiceFormula> for DiceFormula {
    fn from(value: &DiceFormula) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DiceFormula {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d*d\\d+([+-]\\d+)?$|^\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d*d\\d+([+-]\\d+)?$|^\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DiceFormula {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DiceFormula {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DiceFormula {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DiceFormula {
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
///Document ownership permissions by user role
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Document Ownership",
///  "description": "Document ownership permissions by user role",
///  "type": "object",
///  "properties": {
///    "default": {
///      "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///      "type": "integer",
///      "enum": [
///        0,
///        1,
///        2,
///        3
///      ]
///    }
///  },
///  "patternProperties": {
///    "^(PLAYER|TRUSTED|ASSISTANT|GAMEMASTER)$": {
///      "description": "Role-based permission level",
///      "type": "integer",
///      "enum": [
///        0,
///        1,
///        2,
///        3
///      ]
///    },
///    "^[a-zA-Z0-9]{16}$": {
///      "description": "User-specific permission level",
///      "type": "integer",
///      "enum": [
///        0,
///        1,
///        2,
///        3
///      ]
///    }
///  },
///  "additionalProperties": false,
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FoundryDocumentOwnership {
    ///Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default: ::std::option::Option<FoundryDocumentOwnershipDefault>,
}
impl ::std::convert::From<&FoundryDocumentOwnership> for FoundryDocumentOwnership {
    fn from(value: &FoundryDocumentOwnership) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryDocumentOwnership {
    fn default() -> Self {
        Self {
            default: Default::default(),
        }
    }
}
impl FoundryDocumentOwnership {
    pub fn builder() -> builder::FoundryDocumentOwnership {
        Default::default()
    }
}
///Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///  "type": "integer",
///  "enum": [
///    0,
///    1,
///    2,
///    3
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FoundryDocumentOwnershipDefault(i64);
impl ::std::ops::Deref for FoundryDocumentOwnershipDefault {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<FoundryDocumentOwnershipDefault> for i64 {
    fn from(value: FoundryDocumentOwnershipDefault) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryDocumentOwnershipDefault>
for FoundryDocumentOwnershipDefault {
    fn from(value: &FoundryDocumentOwnershipDefault) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for FoundryDocumentOwnershipDefault {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![0_i64, 1_i64, 2_i64, 3_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryDocumentOwnershipDefault {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///Document creation/modification metadata
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Document Statistics",
///  "description": "Document creation/modification metadata",
///  "type": "object",
///  "properties": {
///    "coreVersion": {
///      "description": "Foundry core version",
///      "type": "string"
///    },
///    "createdTime": {
///      "description": "Creation timestamp (Unix milliseconds)",
///      "type": "number"
///    },
///    "lastModifiedBy": {
///      "description": "User ID who last modified",
///      "type": "string",
///      "pattern": "^[a-zA-Z0-9]{16}$"
///    },
///    "modifiedTime": {
///      "description": "Last modification timestamp (Unix milliseconds)",
///      "type": "number"
///    },
///    "systemId": {
///      "description": "Game system ID",
///      "type": "string"
///    },
///    "systemVersion": {
///      "description": "Game system version",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryDocumentStatistics {
    ///Foundry core version
    #[serde(
        rename = "coreVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub core_version: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "createdTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_time: ::std::option::Option<f64>,
    ///User ID who last modified
    #[serde(
        rename = "lastModifiedBy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_modified_by: ::std::option::Option<FoundryDocumentStatisticsLastModifiedBy>,
    #[serde(
        rename = "modifiedTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_time: ::std::option::Option<f64>,
    ///Game system ID
    #[serde(
        rename = "systemId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_id: ::std::option::Option<::std::string::String>,
    ///Game system version
    #[serde(
        rename = "systemVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryDocumentStatistics> for FoundryDocumentStatistics {
    fn from(value: &FoundryDocumentStatistics) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryDocumentStatistics {
    fn default() -> Self {
        Self {
            core_version: Default::default(),
            created_time: Default::default(),
            last_modified_by: Default::default(),
            modified_time: Default::default(),
            system_id: Default::default(),
            system_version: Default::default(),
        }
    }
}
impl FoundryDocumentStatistics {
    pub fn builder() -> builder::FoundryDocumentStatistics {
        Default::default()
    }
}
///User ID who last modified
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "User ID who last modified",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryDocumentStatisticsLastModifiedBy(::std::string::String);
impl ::std::ops::Deref for FoundryDocumentStatisticsLastModifiedBy {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryDocumentStatisticsLastModifiedBy>
for ::std::string::String {
    fn from(value: FoundryDocumentStatisticsLastModifiedBy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryDocumentStatisticsLastModifiedBy>
for FoundryDocumentStatisticsLastModifiedBy {
    fn from(value: &FoundryDocumentStatisticsLastModifiedBy) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryDocumentStatisticsLastModifiedBy {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9]{16}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9]{16}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryDocumentStatisticsLastModifiedBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryDocumentStatisticsLastModifiedBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryDocumentStatisticsLastModifiedBy {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryDocumentStatisticsLastModifiedBy {
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
///Schema for Foundry VTT rollable table documents
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry VTT RollTable",
///  "description": "Schema for Foundry VTT rollable table documents",
///  "allOf": [
///    {
///      "title": "Foundry Base Document",
///      "description": "Common properties shared by all Foundry VTT documents",
///      "type": "object",
///      "required": [
///        "_id",
///        "name"
///      ],
///      "properties": {
///        "_id": {
///          "description": "Unique 16-character document ID",
///          "type": "string",
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "_stats": {
///          "title": "Foundry Document Statistics",
///          "description": "Document creation/modification metadata",
///          "type": "object",
///          "properties": {
///            "coreVersion": {
///              "description": "Foundry core version",
///              "type": "string"
///            },
///            "createdTime": {
///              "description": "Creation timestamp (Unix milliseconds)",
///              "type": "number"
///            },
///            "lastModifiedBy": {
///              "description": "User ID who last modified",
///              "type": "string",
///              "pattern": "^[a-zA-Z0-9]{16}$"
///            },
///            "modifiedTime": {
///              "description": "Last modification timestamp (Unix milliseconds)",
///              "type": "number"
///            },
///            "systemId": {
///              "description": "Game system ID",
///              "type": "string"
///            },
///            "systemVersion": {
///              "description": "Game system version",
///              "type": "string"
///            }
///          },
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "flags": {
///          "description": "Module/system-specific flags",
///          "type": "object",
///          "additionalProperties": true
///        },
///        "folder": {
///          "description": "Parent folder ID, null if in root",
///          "type": [
///            "string",
///            "null"
///          ],
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "name": {
///          "description": "Document name",
///          "type": "string",
///          "minLength": 1
///        },
///        "ownership": {
///          "title": "Foundry Document Ownership",
///          "description": "Document ownership permissions by user role",
///          "type": "object",
///          "properties": {
///            "default": {
///              "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///              "type": "integer",
///              "enum": [
///                0,
///                1,
///                2,
///                3
///              ]
///            }
///          },
///          "patternProperties": {
///            "^(PLAYER|TRUSTED|ASSISTANT|GAMEMASTER)$": {
///              "description": "Role-based permission level",
///              "type": "integer",
///              "enum": [
///                0,
///                1,
///                2,
///                3
///              ]
///            },
///            "^[a-zA-Z0-9]{16}$": {
///              "description": "User-specific permission level",
///              "type": "integer",
///              "enum": [
///                0,
///                1,
///                2,
///                3
///              ]
///            }
///          },
///          "additionalProperties": false,
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "sort": {
///          "description": "Sort order for display",
///          "type": "number"
///        }
///      },
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    {
///      "type": "object",
///      "required": [
///        "name",
///        "results"
///      ],
///      "properties": {
///        "description": {
///          "description": "Table description",
///          "type": "string"
///        },
///        "displayRoll": {
///          "description": "Whether to display the dice roll",
///          "default": true,
///          "type": "boolean"
///        },
///        "formula": {
///          "title": "Dice Formula",
///          "description": "Dice formula (e.g., '1d20+5', '2d6', '10')",
///          "type": "string",
///          "pattern": "^\\d*d\\d+([+-]\\d+)?$|^\\d+$",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "replacement": {
///          "description": "Whether results can be rolled multiple times",
///          "default": true,
///          "type": "boolean"
///        },
///        "results": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "text",
///              "weight"
///            ],
///            "properties": {
///              "_id": {
///                "type": "string",
///                "pattern": "^[a-zA-Z0-9]{16}$"
///              },
///              "collection": {
///                "description": "Document collection name",
///                "type": "string"
///              },
///              "drawn": {
///                "description": "Whether this result has been drawn",
///                "default": false,
///                "type": "boolean"
///              },
///              "img": {
///                "title": "Image Reference",
///                "description": "Image file URL or path",
///                "type": [
///                  "string",
///                  "null"
///                ],
///                "format": "uri",
///                "$schema": "https://json-schema.org/draft-07/schema#"
///              },
///              "range": {
///                "description": "Dice range [low, high]",
///                "type": "array",
///                "items": {
///                  "type": "number"
///                },
///                "maxItems": 2,
///                "minItems": 2
///              },
///              "resultId": {
///                "description": "Referenced document ID",
///                "type": "string"
///              },
///              "text": {
///                "description": "Result text",
///                "type": "string"
///              },
///              "type": {
///                "description": "Result type: 0=text, 1=document, 2=compendium",
///                "default": 0,
///                "type": "number",
///                "enum": [
///                  0,
///                  1,
///                  2
///                ]
///              },
///              "weight": {
///                "default": 1,
///                "type": "number",
///                "minimum": 1.0
///              }
///            }
///          }
///        }
///      }
///    }
///  ],
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttRollTable {
    ///Table description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///Whether to display the dice roll
    #[serde(rename = "displayRoll", default = "defaults::default_bool::<true>")]
    pub display_roll: bool,
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryVttRollTableFolder>,
    ///Dice formula (e.g., '1d20+5', '2d6', '10')
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub formula: ::std::option::Option<DiceFormula>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryVttRollTableId,
    ///Document name
    pub name: FoundryVttRollTableName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    ///Whether results can be rolled multiple times
    #[serde(default = "defaults::default_bool::<true>")]
    pub replacement: bool,
    pub results: ::std::vec::Vec<FoundryVttRollTableResultsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
}
impl ::std::convert::From<&FoundryVttRollTable> for FoundryVttRollTable {
    fn from(value: &FoundryVttRollTable) -> Self {
        value.clone()
    }
}
impl FoundryVttRollTable {
    pub fn builder() -> builder::FoundryVttRollTable {
        Default::default()
    }
}
///Parent folder ID, null if in root
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parent folder ID, null if in root",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttRollTableFolder(::std::string::String);
impl ::std::ops::Deref for FoundryVttRollTableFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttRollTableFolder> for ::std::string::String {
    fn from(value: FoundryVttRollTableFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttRollTableFolder> for FoundryVttRollTableFolder {
    fn from(value: &FoundryVttRollTableFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttRollTableFolder {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9]{16}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9]{16}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttRollTableFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttRollTableFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttRollTableFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttRollTableFolder {
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
///Unique 16-character document ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique 16-character document ID",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttRollTableId(::std::string::String);
impl ::std::ops::Deref for FoundryVttRollTableId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttRollTableId> for ::std::string::String {
    fn from(value: FoundryVttRollTableId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttRollTableId> for FoundryVttRollTableId {
    fn from(value: &FoundryVttRollTableId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttRollTableId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9]{16}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9]{16}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttRollTableId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttRollTableId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttRollTableId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttRollTableId {
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
///Document name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Document name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttRollTableName(::std::string::String);
impl ::std::ops::Deref for FoundryVttRollTableName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttRollTableName> for ::std::string::String {
    fn from(value: FoundryVttRollTableName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttRollTableName> for FoundryVttRollTableName {
    fn from(value: &FoundryVttRollTableName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttRollTableName {
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
impl ::std::convert::TryFrom<&str> for FoundryVttRollTableName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttRollTableName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttRollTableName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttRollTableName {
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
///`FoundryVttRollTableResultsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "text",
///    "weight"
///  ],
///  "properties": {
///    "_id": {
///      "type": "string",
///      "pattern": "^[a-zA-Z0-9]{16}$"
///    },
///    "collection": {
///      "description": "Document collection name",
///      "type": "string"
///    },
///    "drawn": {
///      "description": "Whether this result has been drawn",
///      "default": false,
///      "type": "boolean"
///    },
///    "img": {
///      "title": "Image Reference",
///      "description": "Image file URL or path",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uri",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "range": {
///      "description": "Dice range [low, high]",
///      "type": "array",
///      "items": {
///        "type": "number"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "resultId": {
///      "description": "Referenced document ID",
///      "type": "string"
///    },
///    "text": {
///      "description": "Result text",
///      "type": "string"
///    },
///    "type": {
///      "description": "Result type: 0=text, 1=document, 2=compendium",
///      "default": 0,
///      "type": "number",
///      "enum": [
///        0,
///        1,
///        2
///      ]
///    },
///    "weight": {
///      "default": 1,
///      "type": "number",
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttRollTableResultsItem {
    ///Document collection name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub collection: ::std::option::Option<::std::string::String>,
    ///Whether this result has been drawn
    #[serde(default)]
    pub drawn: bool,
    #[serde(
        rename = "_id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<FoundryVttRollTableResultsItemId>,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    ///Dice range [low, high]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<[f64; 2usize]>,
    ///Referenced document ID
    #[serde(
        rename = "resultId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub result_id: ::std::option::Option<::std::string::String>,
    ///Result text
    pub text: ::std::string::String,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<FoundryVttRollTableResultsItemType>,
    pub weight: f64,
}
impl ::std::convert::From<&FoundryVttRollTableResultsItem>
for FoundryVttRollTableResultsItem {
    fn from(value: &FoundryVttRollTableResultsItem) -> Self {
        value.clone()
    }
}
impl FoundryVttRollTableResultsItem {
    pub fn builder() -> builder::FoundryVttRollTableResultsItem {
        Default::default()
    }
}
///`FoundryVttRollTableResultsItemId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttRollTableResultsItemId(::std::string::String);
impl ::std::ops::Deref for FoundryVttRollTableResultsItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttRollTableResultsItemId> for ::std::string::String {
    fn from(value: FoundryVttRollTableResultsItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttRollTableResultsItemId>
for FoundryVttRollTableResultsItemId {
    fn from(value: &FoundryVttRollTableResultsItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttRollTableResultsItemId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z0-9]{16}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9]{16}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttRollTableResultsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttRollTableResultsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttRollTableResultsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttRollTableResultsItemId {
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
///`FoundryVttRollTableResultsItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Result type: 0=text, 1=document, 2=compendium",
///  "default": 0,
///  "type": "number",
///  "enum": [
///    0,
///    1,
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FoundryVttRollTableResultsItemType(f64);
impl ::std::ops::Deref for FoundryVttRollTableResultsItemType {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttRollTableResultsItemType> for f64 {
    fn from(value: FoundryVttRollTableResultsItemType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttRollTableResultsItemType>
for FoundryVttRollTableResultsItemType {
    fn from(value: &FoundryVttRollTableResultsItemType) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<f64> for FoundryVttRollTableResultsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: f64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![0_f64, 1_f64, 2_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttRollTableResultsItemType {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct FoundryDocumentOwnership {
        default: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnershipDefault>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryDocumentOwnership {
        fn default() -> Self {
            Self {
                default: Ok(Default::default()),
            }
        }
    }
    impl FoundryDocumentOwnership {
        pub fn default<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryDocumentOwnershipDefault>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.default = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for default: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryDocumentOwnership>
    for super::FoundryDocumentOwnership {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryDocumentOwnership,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { default: value.default? })
        }
    }
    impl ::std::convert::From<super::FoundryDocumentOwnership>
    for FoundryDocumentOwnership {
        fn from(value: super::FoundryDocumentOwnership) -> Self {
            Self { default: Ok(value.default) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryDocumentStatistics {
        core_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created_time: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        last_modified_by: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatisticsLastModifiedBy>,
            ::std::string::String,
        >,
        modified_time: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        system_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryDocumentStatistics {
        fn default() -> Self {
            Self {
                core_version: Ok(Default::default()),
                created_time: Ok(Default::default()),
                last_modified_by: Ok(Default::default()),
                modified_time: Ok(Default::default()),
                system_id: Ok(Default::default()),
                system_version: Ok(Default::default()),
            }
        }
    }
    impl FoundryDocumentStatistics {
        pub fn core_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.core_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for core_version: {}", e)
                });
            self
        }
        pub fn created_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for created_time: {}", e)
                });
            self
        }
        pub fn last_modified_by<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryDocumentStatisticsLastModifiedBy>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.last_modified_by = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for last_modified_by: {}", e
                    )
                });
            self
        }
        pub fn modified_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for modified_time: {}", e)
                });
            self
        }
        pub fn system_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_id: {}", e)
                });
            self
        }
        pub fn system_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryDocumentStatistics>
    for super::FoundryDocumentStatistics {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryDocumentStatistics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                core_version: value.core_version?,
                created_time: value.created_time?,
                last_modified_by: value.last_modified_by?,
                modified_time: value.modified_time?,
                system_id: value.system_id?,
                system_version: value.system_version?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryDocumentStatistics>
    for FoundryDocumentStatistics {
        fn from(value: super::FoundryDocumentStatistics) -> Self {
            Self {
                core_version: Ok(value.core_version),
                created_time: Ok(value.created_time),
                last_modified_by: Ok(value.last_modified_by),
                modified_time: Ok(value.modified_time),
                system_id: Ok(value.system_id),
                system_version: Ok(value.system_version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttRollTable {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        display_roll: ::std::result::Result<bool, ::std::string::String>,
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryVttRollTableFolder>,
            ::std::string::String,
        >,
        formula: ::std::result::Result<
            ::std::option::Option<super::DiceFormula>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryVttRollTableId, ::std::string::String>,
        name: ::std::result::Result<
            super::FoundryVttRollTableName,
            ::std::string::String,
        >,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        replacement: ::std::result::Result<bool, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttRollTableResultsItem>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttRollTable {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                display_roll: Ok(super::defaults::default_bool::<true>()),
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                formula: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                replacement: Ok(super::defaults::default_bool::<true>()),
                results: Err("no value supplied for results".to_string()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttRollTable {
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
        pub fn display_roll<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.display_roll = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for display_roll: {}", e)
                });
            self
        }
        pub fn flags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.flags = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for flags: {}", e)
                });
            self
        }
        pub fn folder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttRollTableFolder>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.folder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for folder: {}", e)
                });
            self
        }
        pub fn formula<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DiceFormula>>,
            T::Error: ::std::fmt::Display,
        {
            self.formula = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for formula: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttRollTableId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttRollTableName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn ownership<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryDocumentOwnership>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ownership = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ownership: {}", e)
                });
            self
        }
        pub fn replacement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.replacement = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for replacement: {}", e)
                });
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttRollTableResultsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for results: {}", e)
                });
            self
        }
        pub fn sort<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sort = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sort: {}", e));
            self
        }
        pub fn stats<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryDocumentStatistics>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.stats = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for stats: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttRollTable> for super::FoundryVttRollTable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttRollTable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                display_roll: value.display_roll?,
                flags: value.flags?,
                folder: value.folder?,
                formula: value.formula?,
                id: value.id?,
                name: value.name?,
                ownership: value.ownership?,
                replacement: value.replacement?,
                results: value.results?,
                sort: value.sort?,
                stats: value.stats?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttRollTable> for FoundryVttRollTable {
        fn from(value: super::FoundryVttRollTable) -> Self {
            Self {
                description: Ok(value.description),
                display_roll: Ok(value.display_roll),
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                formula: Ok(value.formula),
                id: Ok(value.id),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                replacement: Ok(value.replacement),
                results: Ok(value.results),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttRollTableResultsItem {
        collection: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        drawn: ::std::result::Result<bool, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<super::FoundryVttRollTableResultsItemId>,
            ::std::string::String,
        >,
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        range: ::std::result::Result<
            ::std::option::Option<[f64; 2usize]>,
            ::std::string::String,
        >,
        result_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::FoundryVttRollTableResultsItemType>,
            ::std::string::String,
        >,
        weight: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttRollTableResultsItem {
        fn default() -> Self {
            Self {
                collection: Ok(Default::default()),
                drawn: Ok(Default::default()),
                id: Ok(Default::default()),
                img: Ok(Default::default()),
                range: Ok(Default::default()),
                result_id: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
                type_: Ok(Default::default()),
                weight: Err("no value supplied for weight".to_string()),
            }
        }
    }
    impl FoundryVttRollTableResultsItem {
        pub fn collection<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.collection = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for collection: {}", e)
                });
            self
        }
        pub fn drawn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.drawn = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for drawn: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttRollTableResultsItemId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn img<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.img = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for img: {}", e));
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<[f64; 2usize]>>,
            T::Error: ::std::fmt::Display,
        {
            self.range = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for range: {}", e)
                });
            self
        }
        pub fn result_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.result_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for result_id: {}", e)
                });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttRollTableResultsItemType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.weight = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for weight: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttRollTableResultsItem>
    for super::FoundryVttRollTableResultsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttRollTableResultsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                collection: value.collection?,
                drawn: value.drawn?,
                id: value.id?,
                img: value.img?,
                range: value.range?,
                result_id: value.result_id?,
                text: value.text?,
                type_: value.type_?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttRollTableResultsItem>
    for FoundryVttRollTableResultsItem {
        fn from(value: super::FoundryVttRollTableResultsItem) -> Self {
            Self {
                collection: Ok(value.collection),
                drawn: Ok(value.drawn),
                id: Ok(value.id),
                img: Ok(value.img),
                range: Ok(value.range),
                result_id: Ok(value.result_id),
                text: Ok(value.text),
                type_: Ok(value.type_),
                weight: Ok(value.weight),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
}
