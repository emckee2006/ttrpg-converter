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
///Schema for Foundry VTT macro documents (system-agnostic)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Macro",
///  "description": "Schema for Foundry VTT macro documents (system-agnostic)",
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
///        "command",
///        "type"
///      ],
///      "properties": {
///        "author": {
///          "description": "Author user ID",
///          "type": "string",
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "command": {
///          "description": "Macro command/script content",
///          "type": "string"
///        },
///        "hotbar": {
///          "description": "Whether macro appears in hotbar",
///          "type": "boolean"
///        },
///        "img": {
///          "title": "Image Reference",
///          "description": "Image file URL or path",
///          "type": [
///            "string",
///            "null"
///          ],
///          "format": "uri",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "scope": {
///          "description": "Macro execution scope",
///          "type": "string",
///          "enum": [
///            "global",
///            "actors",
///            "actor"
///          ]
///        },
///        "type": {
///          "description": "Macro execution type",
///          "type": "string",
///          "enum": [
///            "script",
///            "chat"
///          ]
///        }
///      }
///    }
///  ],
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryMacro {
    ///Author user ID
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub author: ::std::option::Option<FoundryMacroAuthor>,
    ///Macro command/script content
    pub command: ::std::string::String,
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryMacroFolder>,
    ///Whether macro appears in hotbar
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hotbar: ::std::option::Option<bool>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryMacroId,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    ///Document name
    pub name: FoundryMacroName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    ///Macro execution scope
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<FoundryMacroScope>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
    ///Macro execution type
    #[serde(rename = "type")]
    pub type_: FoundryMacroType,
}
impl ::std::convert::From<&FoundryMacro> for FoundryMacro {
    fn from(value: &FoundryMacro) -> Self {
        value.clone()
    }
}
impl FoundryMacro {
    pub fn builder() -> builder::FoundryMacro {
        Default::default()
    }
}
///Author user ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Author user ID",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryMacroAuthor(::std::string::String);
impl ::std::ops::Deref for FoundryMacroAuthor {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryMacroAuthor> for ::std::string::String {
    fn from(value: FoundryMacroAuthor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryMacroAuthor> for FoundryMacroAuthor {
    fn from(value: &FoundryMacroAuthor) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryMacroAuthor {
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
impl ::std::convert::TryFrom<&str> for FoundryMacroAuthor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroAuthor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroAuthor {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryMacroAuthor {
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
pub struct FoundryMacroFolder(::std::string::String);
impl ::std::ops::Deref for FoundryMacroFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryMacroFolder> for ::std::string::String {
    fn from(value: FoundryMacroFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryMacroFolder> for FoundryMacroFolder {
    fn from(value: &FoundryMacroFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryMacroFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryMacroFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryMacroFolder {
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
pub struct FoundryMacroId(::std::string::String);
impl ::std::ops::Deref for FoundryMacroId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryMacroId> for ::std::string::String {
    fn from(value: FoundryMacroId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryMacroId> for FoundryMacroId {
    fn from(value: &FoundryMacroId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryMacroId {
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
impl ::std::convert::TryFrom<&str> for FoundryMacroId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryMacroId {
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
pub struct FoundryMacroName(::std::string::String);
impl ::std::ops::Deref for FoundryMacroName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryMacroName> for ::std::string::String {
    fn from(value: FoundryMacroName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryMacroName> for FoundryMacroName {
    fn from(value: &FoundryMacroName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryMacroName {
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
impl ::std::convert::TryFrom<&str> for FoundryMacroName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryMacroName {
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
///Macro execution scope
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Macro execution scope",
///  "type": "string",
///  "enum": [
///    "global",
///    "actors",
///    "actor"
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
pub enum FoundryMacroScope {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "actors")]
    Actors,
    #[serde(rename = "actor")]
    Actor,
}
impl ::std::convert::From<&Self> for FoundryMacroScope {
    fn from(value: &FoundryMacroScope) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryMacroScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Global => f.write_str("global"),
            Self::Actors => f.write_str("actors"),
            Self::Actor => f.write_str("actor"),
        }
    }
}
impl ::std::str::FromStr for FoundryMacroScope {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "global" => Ok(Self::Global),
            "actors" => Ok(Self::Actors),
            "actor" => Ok(Self::Actor),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryMacroScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Macro execution type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Macro execution type",
///  "type": "string",
///  "enum": [
///    "script",
///    "chat"
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
pub enum FoundryMacroType {
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "chat")]
    Chat,
}
impl ::std::convert::From<&Self> for FoundryMacroType {
    fn from(value: &FoundryMacroType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryMacroType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Script => f.write_str("script"),
            Self::Chat => f.write_str("chat"),
        }
    }
}
impl ::std::str::FromStr for FoundryMacroType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "script" => Ok(Self::Script),
            "chat" => Ok(Self::Chat),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryMacroType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryMacroType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryMacroType {
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
    pub struct FoundryMacro {
        author: ::std::result::Result<
            ::std::option::Option<super::FoundryMacroAuthor>,
            ::std::string::String,
        >,
        command: ::std::result::Result<::std::string::String, ::std::string::String>,
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryMacroFolder>,
            ::std::string::String,
        >,
        hotbar: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryMacroId, ::std::string::String>,
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::FoundryMacroName, ::std::string::String>,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        scope: ::std::result::Result<
            ::std::option::Option<super::FoundryMacroScope>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FoundryMacroType, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryMacro {
        fn default() -> Self {
            Self {
                author: Ok(Default::default()),
                command: Err("no value supplied for command".to_string()),
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                hotbar: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                img: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                scope: Ok(Default::default()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryMacro {
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FoundryMacroAuthor>>,
            T::Error: ::std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for author: {}", e)
                });
            self
        }
        pub fn command<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.command = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for command: {}", e)
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
            T: ::std::convert::TryInto<::std::option::Option<super::FoundryMacroFolder>>,
            T::Error: ::std::fmt::Display,
        {
            self.folder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for folder: {}", e)
                });
            self
        }
        pub fn hotbar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.hotbar = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hotbar: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryMacroId>,
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
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryMacroName>,
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
        pub fn scope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FoundryMacroScope>>,
            T::Error: ::std::fmt::Display,
        {
            self.scope = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scope: {}", e)
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
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryMacroType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryMacro> for super::FoundryMacro {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryMacro,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                author: value.author?,
                command: value.command?,
                flags: value.flags?,
                folder: value.folder?,
                hotbar: value.hotbar?,
                id: value.id?,
                img: value.img?,
                name: value.name?,
                ownership: value.ownership?,
                scope: value.scope?,
                sort: value.sort?,
                stats: value.stats?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryMacro> for FoundryMacro {
        fn from(value: super::FoundryMacro) -> Self {
            Self {
                author: Ok(value.author),
                command: Ok(value.command),
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                hotbar: Ok(value.hotbar),
                id: Ok(value.id),
                img: Ok(value.img),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                scope: Ok(value.scope),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
                type_: Ok(value.type_),
            }
        }
    }
}
