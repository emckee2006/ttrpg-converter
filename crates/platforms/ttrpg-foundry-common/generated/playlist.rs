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
///Schema for Foundry VTT playlist documents
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry VTT Playlist",
///  "description": "Schema for Foundry VTT playlist documents",
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
///        "sounds"
///      ],
///      "properties": {
///        "description": {
///          "description": "Playlist description",
///          "type": "string"
///        },
///        "fade": {
///          "default": 0,
///          "type": "number",
///          "minimum": 0.0
///        },
///        "mode": {
///          "description": "Playback mode: -1=disabled, 0=sequential, 1=shuffle, 2=simultaneous",
///          "default": 0,
///          "type": "number",
///          "enum": [
///            -1,
///            0,
///            1,
///            2
///          ]
///        },
///        "playing": {
///          "default": false,
///          "type": "boolean"
///        },
///        "sounds": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "name",
///              "path"
///            ],
///            "properties": {
///              "_id": {
///                "type": "string",
///                "pattern": "^[a-zA-Z0-9]{16}$"
///              },
///              "description": {
///                "description": "Sound description",
///                "type": "string"
///              },
///              "fade": {
///                "default": 0,
///                "type": "number",
///                "minimum": 0.0
///              },
///              "name": {
///                "description": "Sound name",
///                "type": "string"
///              },
///              "path": {
///                "title": "Audio Reference",
///                "description": "Audio file URL or path",
///                "type": [
///                  "string",
///                  "null"
///                ],
///                "format": "uri",
///                "$schema": "https://json-schema.org/draft-07/schema#"
///              },
///              "pausedTime": {
///                "default": 0,
///                "type": "number",
///                "minimum": 0.0
///              },
///              "playing": {
///                "default": false,
///                "type": "boolean"
///              },
///              "repeat": {
///                "default": false,
///                "type": "boolean"
///              },
///              "sort": {
///                "default": 0,
///                "type": "number"
///              },
///              "streaming": {
///                "default": false,
///                "type": "boolean"
///              },
///              "volume": {
///                "default": 0.5,
///                "type": "number",
///                "maximum": 1.0,
///                "minimum": 0.0
///              }
///            }
///          }
///        },
///        "volume": {
///          "default": 0.5,
///          "type": "number",
///          "maximum": 1.0,
///          "minimum": 0.0
///        }
///      }
///    }
///  ],
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttPlaylist {
    ///Playlist description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fade: ::std::option::Option<f64>,
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryVttPlaylistFolder>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryVttPlaylistId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mode: ::std::option::Option<FoundryVttPlaylistMode>,
    ///Document name
    pub name: FoundryVttPlaylistName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    #[serde(default)]
    pub playing: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    pub sounds: ::std::vec::Vec<FoundryVttPlaylistSoundsItem>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryVttPlaylist> for FoundryVttPlaylist {
    fn from(value: &FoundryVttPlaylist) -> Self {
        value.clone()
    }
}
impl FoundryVttPlaylist {
    pub fn builder() -> builder::FoundryVttPlaylist {
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
pub struct FoundryVttPlaylistFolder(::std::string::String);
impl ::std::ops::Deref for FoundryVttPlaylistFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttPlaylistFolder> for ::std::string::String {
    fn from(value: FoundryVttPlaylistFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttPlaylistFolder> for FoundryVttPlaylistFolder {
    fn from(value: &FoundryVttPlaylistFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttPlaylistFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryVttPlaylistFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttPlaylistFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttPlaylistFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttPlaylistFolder {
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
pub struct FoundryVttPlaylistId(::std::string::String);
impl ::std::ops::Deref for FoundryVttPlaylistId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttPlaylistId> for ::std::string::String {
    fn from(value: FoundryVttPlaylistId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttPlaylistId> for FoundryVttPlaylistId {
    fn from(value: &FoundryVttPlaylistId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttPlaylistId {
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
impl ::std::convert::TryFrom<&str> for FoundryVttPlaylistId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttPlaylistId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttPlaylistId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttPlaylistId {
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
///`FoundryVttPlaylistMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Playback mode: -1=disabled, 0=sequential, 1=shuffle, 2=simultaneous",
///  "default": 0,
///  "type": "number",
///  "enum": [
///    -1,
///    0,
///    1,
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FoundryVttPlaylistMode(f64);
impl ::std::ops::Deref for FoundryVttPlaylistMode {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttPlaylistMode> for f64 {
    fn from(value: FoundryVttPlaylistMode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttPlaylistMode> for FoundryVttPlaylistMode {
    fn from(value: &FoundryVttPlaylistMode) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<f64> for FoundryVttPlaylistMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: f64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![-1_f64, 0_f64, 1_f64, 2_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttPlaylistMode {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
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
pub struct FoundryVttPlaylistName(::std::string::String);
impl ::std::ops::Deref for FoundryVttPlaylistName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttPlaylistName> for ::std::string::String {
    fn from(value: FoundryVttPlaylistName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttPlaylistName> for FoundryVttPlaylistName {
    fn from(value: &FoundryVttPlaylistName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttPlaylistName {
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
impl ::std::convert::TryFrom<&str> for FoundryVttPlaylistName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttPlaylistName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttPlaylistName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttPlaylistName {
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
///`FoundryVttPlaylistSoundsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "path"
///  ],
///  "properties": {
///    "_id": {
///      "type": "string",
///      "pattern": "^[a-zA-Z0-9]{16}$"
///    },
///    "description": {
///      "description": "Sound description",
///      "type": "string"
///    },
///    "fade": {
///      "default": 0,
///      "type": "number",
///      "minimum": 0.0
///    },
///    "name": {
///      "description": "Sound name",
///      "type": "string"
///    },
///    "path": {
///      "title": "Audio Reference",
///      "description": "Audio file URL or path",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uri",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "pausedTime": {
///      "default": 0,
///      "type": "number",
///      "minimum": 0.0
///    },
///    "playing": {
///      "default": false,
///      "type": "boolean"
///    },
///    "repeat": {
///      "default": false,
///      "type": "boolean"
///    },
///    "sort": {
///      "default": 0,
///      "type": "number"
///    },
///    "streaming": {
///      "default": false,
///      "type": "boolean"
///    },
///    "volume": {
///      "default": 0.5,
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttPlaylistSoundsItem {
    ///Sound description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fade: ::std::option::Option<f64>,
    #[serde(
        rename = "_id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<FoundryVttPlaylistSoundsItemId>,
    ///Sound name
    pub name: ::std::string::String,
    ///Audio file URL or path
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "pausedTime",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub paused_time: ::std::option::Option<f64>,
    #[serde(default)]
    pub playing: bool,
    #[serde(default)]
    pub repeat: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(default)]
    pub streaming: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryVttPlaylistSoundsItem>
for FoundryVttPlaylistSoundsItem {
    fn from(value: &FoundryVttPlaylistSoundsItem) -> Self {
        value.clone()
    }
}
impl FoundryVttPlaylistSoundsItem {
    pub fn builder() -> builder::FoundryVttPlaylistSoundsItem {
        Default::default()
    }
}
///`FoundryVttPlaylistSoundsItemId`
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
pub struct FoundryVttPlaylistSoundsItemId(::std::string::String);
impl ::std::ops::Deref for FoundryVttPlaylistSoundsItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttPlaylistSoundsItemId> for ::std::string::String {
    fn from(value: FoundryVttPlaylistSoundsItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttPlaylistSoundsItemId>
for FoundryVttPlaylistSoundsItemId {
    fn from(value: &FoundryVttPlaylistSoundsItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttPlaylistSoundsItemId {
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
impl ::std::convert::TryFrom<&str> for FoundryVttPlaylistSoundsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttPlaylistSoundsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttPlaylistSoundsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttPlaylistSoundsItemId {
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
    pub struct FoundryVttPlaylist {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fade: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryVttPlaylistFolder>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryVttPlaylistId, ::std::string::String>,
        mode: ::std::result::Result<
            ::std::option::Option<super::FoundryVttPlaylistMode>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::FoundryVttPlaylistName,
            ::std::string::String,
        >,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        playing: ::std::result::Result<bool, ::std::string::String>,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sounds: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttPlaylistSoundsItem>,
            ::std::string::String,
        >,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
        volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttPlaylist {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                fade: Ok(Default::default()),
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                mode: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                playing: Ok(Default::default()),
                sort: Ok(Default::default()),
                sounds: Err("no value supplied for sounds".to_string()),
                stats: Ok(Default::default()),
                volume: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttPlaylist {
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
        pub fn fade<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fade = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fade: {}", e));
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
                ::std::option::Option<super::FoundryVttPlaylistFolder>,
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttPlaylistId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn mode<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttPlaylistMode>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.mode = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mode: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttPlaylistName>,
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
        pub fn playing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.playing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playing: {}", e)
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
        pub fn sounds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttPlaylistSoundsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.sounds = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for sounds: {}", e)
                });
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
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for volume: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttPlaylist> for super::FoundryVttPlaylist {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttPlaylist,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                fade: value.fade?,
                flags: value.flags?,
                folder: value.folder?,
                id: value.id?,
                mode: value.mode?,
                name: value.name?,
                ownership: value.ownership?,
                playing: value.playing?,
                sort: value.sort?,
                sounds: value.sounds?,
                stats: value.stats?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttPlaylist> for FoundryVttPlaylist {
        fn from(value: super::FoundryVttPlaylist) -> Self {
            Self {
                description: Ok(value.description),
                fade: Ok(value.fade),
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                id: Ok(value.id),
                mode: Ok(value.mode),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                playing: Ok(value.playing),
                sort: Ok(value.sort),
                sounds: Ok(value.sounds),
                stats: Ok(value.stats),
                volume: Ok(value.volume),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttPlaylistSoundsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fade: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<super::FoundryVttPlaylistSoundsItemId>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        paused_time: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        playing: ::std::result::Result<bool, ::std::string::String>,
        repeat: ::std::result::Result<bool, ::std::string::String>,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        streaming: ::std::result::Result<bool, ::std::string::String>,
        volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttPlaylistSoundsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                fade: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                path: Err("no value supplied for path".to_string()),
                paused_time: Ok(Default::default()),
                playing: Ok(Default::default()),
                repeat: Ok(Default::default()),
                sort: Ok(Default::default()),
                streaming: Ok(Default::default()),
                volume: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttPlaylistSoundsItem {
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
        pub fn fade<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fade = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fade: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttPlaylistSoundsItemId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn paused_time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.paused_time = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for paused_time: {}", e)
                });
            self
        }
        pub fn playing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.playing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playing: {}", e)
                });
            self
        }
        pub fn repeat<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.repeat = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for repeat: {}", e)
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
        pub fn streaming<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.streaming = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for streaming: {}", e)
                });
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for volume: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttPlaylistSoundsItem>
    for super::FoundryVttPlaylistSoundsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttPlaylistSoundsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                fade: value.fade?,
                id: value.id?,
                name: value.name?,
                path: value.path?,
                paused_time: value.paused_time?,
                playing: value.playing?,
                repeat: value.repeat?,
                sort: value.sort?,
                streaming: value.streaming?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttPlaylistSoundsItem>
    for FoundryVttPlaylistSoundsItem {
        fn from(value: super::FoundryVttPlaylistSoundsItem) -> Self {
            Self {
                description: Ok(value.description),
                fade: Ok(value.fade),
                id: Ok(value.id),
                name: Ok(value.name),
                path: Ok(value.path),
                paused_time: Ok(value.paused_time),
                playing: Ok(value.playing),
                repeat: Ok(value.repeat),
                sort: Ok(value.sort),
                streaming: Ok(value.streaming),
                volume: Ok(value.volume),
            }
        }
    }
}
