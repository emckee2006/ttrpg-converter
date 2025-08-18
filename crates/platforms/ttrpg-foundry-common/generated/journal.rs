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
///Schema for Foundry VTT journal entry documents (system-agnostic)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Journal Entry",
///  "description": "Schema for Foundry VTT journal entry documents (system-agnostic)",
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
///      "properties": {
///        "pages": {
///          "description": "Journal pages",
///          "type": "array",
///          "items": {
///            "title": "Foundry Journal Page",
///            "description": "Individual journal entry page",
///            "type": "object",
///            "required": [
///              "_id",
///              "name",
///              "type"
///            ],
///            "properties": {
///              "_id": {
///                "type": "string",
///                "pattern": "^[a-zA-Z0-9]{16}$"
///              },
///              "flags": {
///                "type": "object"
///              },
///              "image": {
///                "type": "object",
///                "properties": {
///                  "caption": {
///                    "description": "Image caption text",
///                    "type": "string"
///                  }
///                }
///              },
///              "name": {
///                "type": "string",
///                "minLength": 1
///              },
///              "ownership": {
///                "title": "Foundry Document Ownership",
///                "description": "Document ownership permissions by user role",
///                "type": "object",
///                "properties": {
///                  "default": {
///                    "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///                    "type": "integer",
///                    "enum": [
///                      0,
///                      1,
///                      2,
///                      3
///                    ]
///                  }
///                },
///                "patternProperties": {
///                  "^(PLAYER|TRUSTED|ASSISTANT|GAMEMASTER)$": {
///                    "description": "Role-based permission level",
///                    "type": "integer",
///                    "enum": [
///                      0,
///                      1,
///                      2,
///                      3
///                    ]
///                  },
///                  "^[a-zA-Z0-9]{16}$": {
///                    "description": "User-specific permission level",
///                    "type": "integer",
///                    "enum": [
///                      0,
///                      1,
///                      2,
///                      3
///                    ]
///                  }
///                },
///                "additionalProperties": false,
///                "$schema": "https://json-schema.org/draft-07/schema#"
///              },
///              "sort": {
///                "type": "number"
///              },
///              "src": {
///                "description": "Source URL for PDF/external content",
///                "type": "string",
///                "format": "uri"
///              },
///              "text": {
///                "type": "object",
///                "properties": {
///                  "content": {
///                    "description": "Rich text HTML content",
///                    "type": "string"
///                  },
///                  "format": {
///                    "description": "Text format: 1=HTML, 2=Markdown",
///                    "type": "integer",
///                    "enum": [
///                      1,
///                      2
///                    ]
///                  },
///                  "markdown": {
///                    "description": "Markdown source content",
///                    "type": "string"
///                  }
///                }
///              },
///              "title": {
///                "type": "object",
///                "properties": {
///                  "level": {
///                    "description": "Heading level (1-6)",
///                    "type": "integer",
///                    "maximum": 6.0,
///                    "minimum": 1.0
///                  },
///                  "show": {
///                    "description": "Whether to show page title",
///                    "type": "boolean"
///                  }
///                }
///              },
///              "type": {
///                "description": "Page content type",
///                "type": "string",
///                "enum": [
///                  "text",
///                  "image",
///                  "pdf",
///                  "video"
///                ]
///              },
///              "video": {
///                "type": "object",
///                "properties": {
///                  "autoplay": {
///                    "description": "Whether to autoplay video",
///                    "type": "boolean"
///                  },
///                  "height": {
///                    "description": "Video display height",
///                    "type": "number",
///                    "minimum": 1.0
///                  },
///                  "loop": {
///                    "description": "Whether to loop video playback",
///                    "type": "boolean"
///                  },
///                  "timestamp": {
///                    "description": "Start timestamp in seconds",
///                    "type": "number",
///                    "minimum": 0.0
///                  },
///                  "volume": {
///                    "description": "Video volume (0-1)",
///                    "type": "number",
///                    "maximum": 1.0,
///                    "minimum": 0.0
///                  },
///                  "width": {
///                    "description": "Video display width",
///                    "type": "number",
///                    "minimum": 1.0
///                  }
///                }
///              }
///            },
///            "$schema": "https://json-schema.org/draft-07/schema#"
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
pub struct FoundryJournalEntry {
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryJournalEntryFolder>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryJournalEntryId,
    ///Document name
    pub name: FoundryJournalEntryName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    ///Journal pages
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub pages: ::std::vec::Vec<FoundryJournalPage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
}
impl ::std::convert::From<&FoundryJournalEntry> for FoundryJournalEntry {
    fn from(value: &FoundryJournalEntry) -> Self {
        value.clone()
    }
}
impl FoundryJournalEntry {
    pub fn builder() -> builder::FoundryJournalEntry {
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
pub struct FoundryJournalEntryFolder(::std::string::String);
impl ::std::ops::Deref for FoundryJournalEntryFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalEntryFolder> for ::std::string::String {
    fn from(value: FoundryJournalEntryFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalEntryFolder> for FoundryJournalEntryFolder {
    fn from(value: &FoundryJournalEntryFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryJournalEntryFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryJournalEntryFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalEntryFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalEntryFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalEntryFolder {
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
pub struct FoundryJournalEntryId(::std::string::String);
impl ::std::ops::Deref for FoundryJournalEntryId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalEntryId> for ::std::string::String {
    fn from(value: FoundryJournalEntryId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalEntryId> for FoundryJournalEntryId {
    fn from(value: &FoundryJournalEntryId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryJournalEntryId {
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
impl ::std::convert::TryFrom<&str> for FoundryJournalEntryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalEntryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalEntryId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalEntryId {
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
pub struct FoundryJournalEntryName(::std::string::String);
impl ::std::ops::Deref for FoundryJournalEntryName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalEntryName> for ::std::string::String {
    fn from(value: FoundryJournalEntryName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalEntryName> for FoundryJournalEntryName {
    fn from(value: &FoundryJournalEntryName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryJournalEntryName {
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
impl ::std::convert::TryFrom<&str> for FoundryJournalEntryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalEntryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalEntryName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalEntryName {
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
///Individual journal entry page
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Journal Page",
///  "description": "Individual journal entry page",
///  "type": "object",
///  "required": [
///    "_id",
///    "name",
///    "type"
///  ],
///  "properties": {
///    "_id": {
///      "type": "string",
///      "pattern": "^[a-zA-Z0-9]{16}$"
///    },
///    "flags": {
///      "type": "object"
///    },
///    "image": {
///      "type": "object",
///      "properties": {
///        "caption": {
///          "description": "Image caption text",
///          "type": "string"
///        }
///      }
///    },
///    "name": {
///      "type": "string",
///      "minLength": 1
///    },
///    "ownership": {
///      "title": "Foundry Document Ownership",
///      "description": "Document ownership permissions by user role",
///      "type": "object",
///      "properties": {
///        "default": {
///          "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///          "type": "integer",
///          "enum": [
///            0,
///            1,
///            2,
///            3
///          ]
///        }
///      },
///      "patternProperties": {
///        "^(PLAYER|TRUSTED|ASSISTANT|GAMEMASTER)$": {
///          "description": "Role-based permission level",
///          "type": "integer",
///          "enum": [
///            0,
///            1,
///            2,
///            3
///          ]
///        },
///        "^[a-zA-Z0-9]{16}$": {
///          "description": "User-specific permission level",
///          "type": "integer",
///          "enum": [
///            0,
///            1,
///            2,
///            3
///          ]
///        }
///      },
///      "additionalProperties": false,
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "sort": {
///      "type": "number"
///    },
///    "src": {
///      "description": "Source URL for PDF/external content",
///      "type": "string",
///      "format": "uri"
///    },
///    "text": {
///      "type": "object",
///      "properties": {
///        "content": {
///          "description": "Rich text HTML content",
///          "type": "string"
///        },
///        "format": {
///          "description": "Text format: 1=HTML, 2=Markdown",
///          "type": "integer",
///          "enum": [
///            1,
///            2
///          ]
///        },
///        "markdown": {
///          "description": "Markdown source content",
///          "type": "string"
///        }
///      }
///    },
///    "title": {
///      "type": "object",
///      "properties": {
///        "level": {
///          "description": "Heading level (1-6)",
///          "type": "integer",
///          "maximum": 6.0,
///          "minimum": 1.0
///        },
///        "show": {
///          "description": "Whether to show page title",
///          "type": "boolean"
///        }
///      }
///    },
///    "type": {
///      "description": "Page content type",
///      "type": "string",
///      "enum": [
///        "text",
///        "image",
///        "pdf",
///        "video"
///      ]
///    },
///    "video": {
///      "type": "object",
///      "properties": {
///        "autoplay": {
///          "description": "Whether to autoplay video",
///          "type": "boolean"
///        },
///        "height": {
///          "description": "Video display height",
///          "type": "number",
///          "minimum": 1.0
///        },
///        "loop": {
///          "description": "Whether to loop video playback",
///          "type": "boolean"
///        },
///        "timestamp": {
///          "description": "Start timestamp in seconds",
///          "type": "number",
///          "minimum": 0.0
///        },
///        "volume": {
///          "description": "Video volume (0-1)",
///          "type": "number",
///          "maximum": 1.0,
///          "minimum": 0.0
///        },
///        "width": {
///          "description": "Video display width",
///          "type": "number",
///          "minimum": 1.0
///        }
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryJournalPage {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(rename = "_id")]
    pub id: FoundryJournalPageId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub image: ::std::option::Option<FoundryJournalPageImage>,
    pub name: FoundryJournalPageName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    ///Source URL for PDF/external content
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub src: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<FoundryJournalPageText>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<FoundryJournalPageTitle>,
    ///Page content type
    #[serde(rename = "type")]
    pub type_: FoundryJournalPageType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub video: ::std::option::Option<FoundryJournalPageVideo>,
}
impl ::std::convert::From<&FoundryJournalPage> for FoundryJournalPage {
    fn from(value: &FoundryJournalPage) -> Self {
        value.clone()
    }
}
impl FoundryJournalPage {
    pub fn builder() -> builder::FoundryJournalPage {
        Default::default()
    }
}
///`FoundryJournalPageId`
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
pub struct FoundryJournalPageId(::std::string::String);
impl ::std::ops::Deref for FoundryJournalPageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalPageId> for ::std::string::String {
    fn from(value: FoundryJournalPageId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalPageId> for FoundryJournalPageId {
    fn from(value: &FoundryJournalPageId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryJournalPageId {
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
impl ::std::convert::TryFrom<&str> for FoundryJournalPageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalPageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalPageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalPageId {
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
///`FoundryJournalPageImage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "caption": {
///      "description": "Image caption text",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryJournalPageImage {
    ///Image caption text
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub caption: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryJournalPageImage> for FoundryJournalPageImage {
    fn from(value: &FoundryJournalPageImage) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryJournalPageImage {
    fn default() -> Self {
        Self {
            caption: Default::default(),
        }
    }
}
impl FoundryJournalPageImage {
    pub fn builder() -> builder::FoundryJournalPageImage {
        Default::default()
    }
}
///`FoundryJournalPageName`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryJournalPageName(::std::string::String);
impl ::std::ops::Deref for FoundryJournalPageName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalPageName> for ::std::string::String {
    fn from(value: FoundryJournalPageName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalPageName> for FoundryJournalPageName {
    fn from(value: &FoundryJournalPageName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryJournalPageName {
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
impl ::std::convert::TryFrom<&str> for FoundryJournalPageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalPageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalPageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalPageName {
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
///`FoundryJournalPageText`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "content": {
///      "description": "Rich text HTML content",
///      "type": "string"
///    },
///    "format": {
///      "description": "Text format: 1=HTML, 2=Markdown",
///      "type": "integer",
///      "enum": [
///        1,
///        2
///      ]
///    },
///    "markdown": {
///      "description": "Markdown source content",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryJournalPageText {
    ///Rich text HTML content
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub content: ::std::option::Option<::std::string::String>,
    ///Text format: 1=HTML, 2=Markdown
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub format: ::std::option::Option<FoundryJournalPageTextFormat>,
    ///Markdown source content
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub markdown: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryJournalPageText> for FoundryJournalPageText {
    fn from(value: &FoundryJournalPageText) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryJournalPageText {
    fn default() -> Self {
        Self {
            content: Default::default(),
            format: Default::default(),
            markdown: Default::default(),
        }
    }
}
impl FoundryJournalPageText {
    pub fn builder() -> builder::FoundryJournalPageText {
        Default::default()
    }
}
///Text format: 1=HTML, 2=Markdown
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Text format: 1=HTML, 2=Markdown",
///  "type": "integer",
///  "enum": [
///    1,
///    2
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FoundryJournalPageTextFormat(i64);
impl ::std::ops::Deref for FoundryJournalPageTextFormat {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<FoundryJournalPageTextFormat> for i64 {
    fn from(value: FoundryJournalPageTextFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryJournalPageTextFormat>
for FoundryJournalPageTextFormat {
    fn from(value: &FoundryJournalPageTextFormat) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for FoundryJournalPageTextFormat {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![1_i64, 2_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryJournalPageTextFormat {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`FoundryJournalPageTitle`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "level": {
///      "description": "Heading level (1-6)",
///      "type": "integer",
///      "maximum": 6.0,
///      "minimum": 1.0
///    },
///    "show": {
///      "description": "Whether to show page title",
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryJournalPageTitle {
    ///Heading level (1-6)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<::std::num::NonZeroU64>,
    ///Whether to show page title
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub show: ::std::option::Option<bool>,
}
impl ::std::convert::From<&FoundryJournalPageTitle> for FoundryJournalPageTitle {
    fn from(value: &FoundryJournalPageTitle) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryJournalPageTitle {
    fn default() -> Self {
        Self {
            level: Default::default(),
            show: Default::default(),
        }
    }
}
impl FoundryJournalPageTitle {
    pub fn builder() -> builder::FoundryJournalPageTitle {
        Default::default()
    }
}
///Page content type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Page content type",
///  "type": "string",
///  "enum": [
///    "text",
///    "image",
///    "pdf",
///    "video"
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
pub enum FoundryJournalPageType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "video")]
    Video,
}
impl ::std::convert::From<&Self> for FoundryJournalPageType {
    fn from(value: &FoundryJournalPageType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryJournalPageType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => f.write_str("text"),
            Self::Image => f.write_str("image"),
            Self::Pdf => f.write_str("pdf"),
            Self::Video => f.write_str("video"),
        }
    }
}
impl ::std::str::FromStr for FoundryJournalPageType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            "pdf" => Ok(Self::Pdf),
            "video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryJournalPageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryJournalPageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryJournalPageType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryJournalPageVideo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "autoplay": {
///      "description": "Whether to autoplay video",
///      "type": "boolean"
///    },
///    "height": {
///      "description": "Video display height",
///      "type": "number",
///      "minimum": 1.0
///    },
///    "loop": {
///      "description": "Whether to loop video playback",
///      "type": "boolean"
///    },
///    "timestamp": {
///      "description": "Start timestamp in seconds",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "volume": {
///      "description": "Video volume (0-1)",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0
///    },
///    "width": {
///      "description": "Video display width",
///      "type": "number",
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryJournalPageVideo {
    ///Whether to autoplay video
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub autoplay: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    ///Whether to loop video playback
    #[serde(
        rename = "loop",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub loop_: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryJournalPageVideo> for FoundryJournalPageVideo {
    fn from(value: &FoundryJournalPageVideo) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryJournalPageVideo {
    fn default() -> Self {
        Self {
            autoplay: Default::default(),
            height: Default::default(),
            loop_: Default::default(),
            timestamp: Default::default(),
            volume: Default::default(),
            width: Default::default(),
        }
    }
}
impl FoundryJournalPageVideo {
    pub fn builder() -> builder::FoundryJournalPageVideo {
        Default::default()
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
    pub struct FoundryJournalEntry {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalEntryFolder>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryJournalEntryId, ::std::string::String>,
        name: ::std::result::Result<
            super::FoundryJournalEntryName,
            ::std::string::String,
        >,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        pages: ::std::result::Result<
            ::std::vec::Vec<super::FoundryJournalPage>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryJournalEntry {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                pages: Ok(Default::default()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalEntry {
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
                ::std::option::Option<super::FoundryJournalEntryFolder>,
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
            T: ::std::convert::TryInto<super::FoundryJournalEntryId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryJournalEntryName>,
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
        pub fn pages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FoundryJournalPage>>,
            T::Error: ::std::fmt::Display,
        {
            self.pages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pages: {}", e)
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
    impl ::std::convert::TryFrom<FoundryJournalEntry> for super::FoundryJournalEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                folder: value.folder?,
                id: value.id?,
                name: value.name?,
                ownership: value.ownership?,
                pages: value.pages?,
                sort: value.sort?,
                stats: value.stats?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryJournalEntry> for FoundryJournalEntry {
        fn from(value: super::FoundryJournalEntry) -> Self {
            Self {
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                id: Ok(value.id),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                pages: Ok(value.pages),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryJournalPage {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryJournalPageId, ::std::string::String>,
        image: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalPageImage>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::FoundryJournalPageName,
            ::std::string::String,
        >,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        src: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalPageText>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalPageTitle>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::FoundryJournalPageType,
            ::std::string::String,
        >,
        video: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalPageVideo>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryJournalPage {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                image: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                sort: Ok(Default::default()),
                src: Ok(Default::default()),
                text: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                video: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalPage {
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryJournalPageId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryJournalPageImage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for image: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryJournalPageName>,
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
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryJournalPageText>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryJournalPageTitle>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryJournalPageType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn video<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryJournalPageVideo>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.video = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for video: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryJournalPage> for super::FoundryJournalPage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalPage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                id: value.id?,
                image: value.image?,
                name: value.name?,
                ownership: value.ownership?,
                sort: value.sort?,
                src: value.src?,
                text: value.text?,
                title: value.title?,
                type_: value.type_?,
                video: value.video?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryJournalPage> for FoundryJournalPage {
        fn from(value: super::FoundryJournalPage) -> Self {
            Self {
                flags: Ok(value.flags),
                id: Ok(value.id),
                image: Ok(value.image),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                sort: Ok(value.sort),
                src: Ok(value.src),
                text: Ok(value.text),
                title: Ok(value.title),
                type_: Ok(value.type_),
                video: Ok(value.video),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryJournalPageImage {
        caption: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryJournalPageImage {
        fn default() -> Self {
            Self {
                caption: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalPageImage {
        pub fn caption<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.caption = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for caption: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryJournalPageImage>
    for super::FoundryJournalPageImage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalPageImage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { caption: value.caption? })
        }
    }
    impl ::std::convert::From<super::FoundryJournalPageImage>
    for FoundryJournalPageImage {
        fn from(value: super::FoundryJournalPageImage) -> Self {
            Self { caption: Ok(value.caption) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryJournalPageText {
        content: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        format: ::std::result::Result<
            ::std::option::Option<super::FoundryJournalPageTextFormat>,
            ::std::string::String,
        >,
        markdown: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryJournalPageText {
        fn default() -> Self {
            Self {
                content: Ok(Default::default()),
                format: Ok(Default::default()),
                markdown: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalPageText {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for content: {}", e)
                });
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryJournalPageTextFormat>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for format: {}", e)
                });
            self
        }
        pub fn markdown<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.markdown = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for markdown: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryJournalPageText>
    for super::FoundryJournalPageText {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalPageText,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                content: value.content?,
                format: value.format?,
                markdown: value.markdown?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryJournalPageText> for FoundryJournalPageText {
        fn from(value: super::FoundryJournalPageText) -> Self {
            Self {
                content: Ok(value.content),
                format: Ok(value.format),
                markdown: Ok(value.markdown),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryJournalPageTitle {
        level: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        show: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryJournalPageTitle {
        fn default() -> Self {
            Self {
                level: Ok(Default::default()),
                show: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalPageTitle {
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for level: {}", e)
                });
            self
        }
        pub fn show<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.show = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for show: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryJournalPageTitle>
    for super::FoundryJournalPageTitle {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalPageTitle,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                level: value.level?,
                show: value.show?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryJournalPageTitle>
    for FoundryJournalPageTitle {
        fn from(value: super::FoundryJournalPageTitle) -> Self {
            Self {
                level: Ok(value.level),
                show: Ok(value.show),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryJournalPageVideo {
        autoplay: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        loop_: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        timestamp: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryJournalPageVideo {
        fn default() -> Self {
            Self {
                autoplay: Ok(Default::default()),
                height: Ok(Default::default()),
                loop_: Ok(Default::default()),
                timestamp: Ok(Default::default()),
                volume: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl FoundryJournalPageVideo {
        pub fn autoplay<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.autoplay = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for autoplay: {}", e)
                });
            self
        }
        pub fn height<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.height = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for height: {}", e)
                });
            self
        }
        pub fn loop_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.loop_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for loop_: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
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
        pub fn width<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.width = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for width: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryJournalPageVideo>
    for super::FoundryJournalPageVideo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryJournalPageVideo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                autoplay: value.autoplay?,
                height: value.height?,
                loop_: value.loop_?,
                timestamp: value.timestamp?,
                volume: value.volume?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryJournalPageVideo>
    for FoundryJournalPageVideo {
        fn from(value: super::FoundryJournalPageVideo) -> Self {
            Self {
                autoplay: Ok(value.autoplay),
                height: Ok(value.height),
                loop_: Ok(value.loop_),
                timestamp: Ok(value.timestamp),
                volume: Ok(value.volume),
                width: Ok(value.width),
            }
        }
    }
}
