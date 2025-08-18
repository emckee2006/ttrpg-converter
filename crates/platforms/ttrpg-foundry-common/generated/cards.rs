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
///Schema for Foundry VTT card deck documents
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry VTT Cards",
///  "description": "Schema for Foundry VTT card deck documents",
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
///        "type"
///      ],
///      "properties": {
///        "cards": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "name",
///              "type"
///            ],
///            "properties": {
///              "_id": {
///                "type": "string",
///                "pattern": "^[a-zA-Z0-9]{16}$"
///              },
///              "description": {
///                "description": "Card description",
///                "type": "string"
///              },
///              "drawn": {
///                "default": false,
///                "type": "boolean"
///              },
///              "face": {
///                "description": "Current face index",
///                "type": "number",
///                "minimum": 0.0
///              },
///              "faces": {
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "properties": {
///                    "img": {
///                      "title": "Image Reference",
///                      "description": "Image file URL or path",
///                      "type": [
///                        "string",
///                        "null"
///                      ],
///                      "format": "uri",
///                      "$schema": "https://json-schema.org/draft-07/schema#"
///                    },
///                    "name": {
///                      "type": "string"
///                    },
///                    "text": {
///                      "type": "string"
///                    }
///                  }
///                }
///              },
///              "height": {
///                "default": 1,
///                "type": "number",
///                "minimum": 1.0
///              },
///              "name": {
///                "description": "Card name",
///                "type": "string"
///              },
///              "origin": {
///                "description": "Origin deck ID",
///                "type": "string"
///              },
///              "sort": {
///                "default": 0,
///                "type": "number"
///              },
///              "suit": {
///                "description": "Card suit",
///                "type": "string"
///              },
///              "type": {
///                "default": "base",
///                "type": "string",
///                "enum": [
///                  "base",
///                  "preset"
///                ]
///              },
///              "value": {
///                "description": "Card value",
///                "type": "string"
///              },
///              "width": {
///                "default": 1,
///                "type": "number",
///                "minimum": 1.0
///              }
///            }
///          }
///        },
///        "description": {
///          "description": "Cards description",
///          "type": "string"
///        },
///        "displayCount": {
///          "default": false,
///          "type": "boolean"
///        },
///        "preset": {
///          "description": "Preset card configuration",
///          "type": "string",
///          "enum": [
///            "",
///            "poker",
///            "tarot"
///          ]
///        },
///        "shuffle": {
///          "default": true,
///          "type": "boolean"
///        },
///        "type": {
///          "description": "Type of card collection",
///          "type": "string",
///          "enum": [
///            "deck",
///            "pile",
///            "hand"
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
pub struct FoundryVttCards {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub cards: ::std::vec::Vec<FoundryVttCardsCardsItem>,
    ///Cards description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayCount", default)]
    pub display_count: bool,
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryVttCardsFolder>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryVttCardsId,
    ///Document name
    pub name: FoundryVttCardsName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    ///Preset card configuration
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub preset: ::std::option::Option<FoundryVttCardsPreset>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub shuffle: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
    ///Type of card collection
    #[serde(rename = "type")]
    pub type_: FoundryVttCardsType,
}
impl ::std::convert::From<&FoundryVttCards> for FoundryVttCards {
    fn from(value: &FoundryVttCards) -> Self {
        value.clone()
    }
}
impl FoundryVttCards {
    pub fn builder() -> builder::FoundryVttCards {
        Default::default()
    }
}
///`FoundryVttCardsCardsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "type"
///  ],
///  "properties": {
///    "_id": {
///      "type": "string",
///      "pattern": "^[a-zA-Z0-9]{16}$"
///    },
///    "description": {
///      "description": "Card description",
///      "type": "string"
///    },
///    "drawn": {
///      "default": false,
///      "type": "boolean"
///    },
///    "face": {
///      "description": "Current face index",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "faces": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "img": {
///            "title": "Image Reference",
///            "description": "Image file URL or path",
///            "type": [
///              "string",
///              "null"
///            ],
///            "format": "uri",
///            "$schema": "https://json-schema.org/draft-07/schema#"
///          },
///          "name": {
///            "type": "string"
///          },
///          "text": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "height": {
///      "default": 1,
///      "type": "number",
///      "minimum": 1.0
///    },
///    "name": {
///      "description": "Card name",
///      "type": "string"
///    },
///    "origin": {
///      "description": "Origin deck ID",
///      "type": "string"
///    },
///    "sort": {
///      "default": 0,
///      "type": "number"
///    },
///    "suit": {
///      "description": "Card suit",
///      "type": "string"
///    },
///    "type": {
///      "default": "base",
///      "type": "string",
///      "enum": [
///        "base",
///        "preset"
///      ]
///    },
///    "value": {
///      "description": "Card value",
///      "type": "string"
///    },
///    "width": {
///      "default": 1,
///      "type": "number",
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttCardsCardsItem {
    ///Card description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub drawn: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub face: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub faces: ::std::vec::Vec<FoundryVttCardsCardsItemFacesItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    #[serde(
        rename = "_id",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub id: ::std::option::Option<FoundryVttCardsCardsItemId>,
    ///Card name
    pub name: ::std::string::String,
    ///Origin deck ID
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub origin: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    ///Card suit
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub suit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FoundryVttCardsCardsItemType,
    ///Card value
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryVttCardsCardsItem> for FoundryVttCardsCardsItem {
    fn from(value: &FoundryVttCardsCardsItem) -> Self {
        value.clone()
    }
}
impl FoundryVttCardsCardsItem {
    pub fn builder() -> builder::FoundryVttCardsCardsItem {
        Default::default()
    }
}
///`FoundryVttCardsCardsItemFacesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
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
///    "name": {
///      "type": "string"
///    },
///    "text": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttCardsCardsItemFacesItem {
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryVttCardsCardsItemFacesItem>
for FoundryVttCardsCardsItemFacesItem {
    fn from(value: &FoundryVttCardsCardsItemFacesItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryVttCardsCardsItemFacesItem {
    fn default() -> Self {
        Self {
            img: Default::default(),
            name: Default::default(),
            text: Default::default(),
        }
    }
}
impl FoundryVttCardsCardsItemFacesItem {
    pub fn builder() -> builder::FoundryVttCardsCardsItemFacesItem {
        Default::default()
    }
}
///`FoundryVttCardsCardsItemId`
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
pub struct FoundryVttCardsCardsItemId(::std::string::String);
impl ::std::ops::Deref for FoundryVttCardsCardsItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttCardsCardsItemId> for ::std::string::String {
    fn from(value: FoundryVttCardsCardsItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttCardsCardsItemId> for FoundryVttCardsCardsItemId {
    fn from(value: &FoundryVttCardsCardsItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttCardsCardsItemId {
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
impl ::std::convert::TryFrom<&str> for FoundryVttCardsCardsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsCardsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsCardsItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttCardsCardsItemId {
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
///`FoundryVttCardsCardsItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "default": "base",
///  "type": "string",
///  "enum": [
///    "base",
///    "preset"
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
pub enum FoundryVttCardsCardsItemType {
    #[serde(rename = "base")]
    Base,
    #[serde(rename = "preset")]
    Preset,
}
impl ::std::convert::From<&Self> for FoundryVttCardsCardsItemType {
    fn from(value: &FoundryVttCardsCardsItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttCardsCardsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Base => f.write_str("base"),
            Self::Preset => f.write_str("preset"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttCardsCardsItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "base" => Ok(Self::Base),
            "preset" => Ok(Self::Preset),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttCardsCardsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsCardsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsCardsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for FoundryVttCardsCardsItemType {
    fn default() -> Self {
        FoundryVttCardsCardsItemType::Base
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
pub struct FoundryVttCardsFolder(::std::string::String);
impl ::std::ops::Deref for FoundryVttCardsFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttCardsFolder> for ::std::string::String {
    fn from(value: FoundryVttCardsFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttCardsFolder> for FoundryVttCardsFolder {
    fn from(value: &FoundryVttCardsFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttCardsFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryVttCardsFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttCardsFolder {
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
pub struct FoundryVttCardsId(::std::string::String);
impl ::std::ops::Deref for FoundryVttCardsId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttCardsId> for ::std::string::String {
    fn from(value: FoundryVttCardsId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttCardsId> for FoundryVttCardsId {
    fn from(value: &FoundryVttCardsId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttCardsId {
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
impl ::std::convert::TryFrom<&str> for FoundryVttCardsId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttCardsId {
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
pub struct FoundryVttCardsName(::std::string::String);
impl ::std::ops::Deref for FoundryVttCardsName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttCardsName> for ::std::string::String {
    fn from(value: FoundryVttCardsName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttCardsName> for FoundryVttCardsName {
    fn from(value: &FoundryVttCardsName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttCardsName {
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
impl ::std::convert::TryFrom<&str> for FoundryVttCardsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttCardsName {
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
///Preset card configuration
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Preset card configuration",
///  "type": "string",
///  "enum": [
///    "",
///    "poker",
///    "tarot"
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
pub enum FoundryVttCardsPreset {
    #[serde(rename = "")]
    X,
    #[serde(rename = "poker")]
    Poker,
    #[serde(rename = "tarot")]
    Tarot,
}
impl ::std::convert::From<&Self> for FoundryVttCardsPreset {
    fn from(value: &FoundryVttCardsPreset) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttCardsPreset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str(""),
            Self::Poker => f.write_str("poker"),
            Self::Tarot => f.write_str("tarot"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttCardsPreset {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "" => Ok(Self::X),
            "poker" => Ok(Self::Poker),
            "tarot" => Ok(Self::Tarot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttCardsPreset {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsPreset {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsPreset {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Type of card collection
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Type of card collection",
///  "type": "string",
///  "enum": [
///    "deck",
///    "pile",
///    "hand"
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
pub enum FoundryVttCardsType {
    #[serde(rename = "deck")]
    Deck,
    #[serde(rename = "pile")]
    Pile,
    #[serde(rename = "hand")]
    Hand,
}
impl ::std::convert::From<&Self> for FoundryVttCardsType {
    fn from(value: &FoundryVttCardsType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttCardsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Deck => f.write_str("deck"),
            Self::Pile => f.write_str("pile"),
            Self::Hand => f.write_str("hand"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttCardsType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "deck" => Ok(Self::Deck),
            "pile" => Ok(Self::Pile),
            "hand" => Ok(Self::Hand),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttCardsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttCardsType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttCardsType {
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
    pub struct FoundryVttCards {
        cards: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttCardsCardsItem>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        display_count: ::std::result::Result<bool, ::std::string::String>,
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryVttCardsFolder>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryVttCardsId, ::std::string::String>,
        name: ::std::result::Result<super::FoundryVttCardsName, ::std::string::String>,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        preset: ::std::result::Result<
            ::std::option::Option<super::FoundryVttCardsPreset>,
            ::std::string::String,
        >,
        shuffle: ::std::result::Result<bool, ::std::string::String>,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FoundryVttCardsType, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttCards {
        fn default() -> Self {
            Self {
                cards: Ok(Default::default()),
                description: Ok(Default::default()),
                display_count: Ok(Default::default()),
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                preset: Ok(Default::default()),
                shuffle: Ok(super::defaults::default_bool::<true>()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryVttCards {
        pub fn cards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FoundryVttCardsCardsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.cards = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cards: {}", e)
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
        pub fn display_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.display_count = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for display_count: {}", e)
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
                ::std::option::Option<super::FoundryVttCardsFolder>,
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
            T: ::std::convert::TryInto<super::FoundryVttCardsId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttCardsName>,
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
        pub fn preset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttCardsPreset>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.preset = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for preset: {}", e)
                });
            self
        }
        pub fn shuffle<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.shuffle = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shuffle: {}", e)
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
            T: ::std::convert::TryInto<super::FoundryVttCardsType>,
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
    impl ::std::convert::TryFrom<FoundryVttCards> for super::FoundryVttCards {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttCards,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cards: value.cards?,
                description: value.description?,
                display_count: value.display_count?,
                flags: value.flags?,
                folder: value.folder?,
                id: value.id?,
                name: value.name?,
                ownership: value.ownership?,
                preset: value.preset?,
                shuffle: value.shuffle?,
                sort: value.sort?,
                stats: value.stats?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttCards> for FoundryVttCards {
        fn from(value: super::FoundryVttCards) -> Self {
            Self {
                cards: Ok(value.cards),
                description: Ok(value.description),
                display_count: Ok(value.display_count),
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                id: Ok(value.id),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                preset: Ok(value.preset),
                shuffle: Ok(value.shuffle),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttCardsCardsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        drawn: ::std::result::Result<bool, ::std::string::String>,
        face: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        faces: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttCardsCardsItemFacesItem>,
            ::std::string::String,
        >,
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<super::FoundryVttCardsCardsItemId>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        origin: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        suit: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::FoundryVttCardsCardsItemType,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttCardsCardsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                drawn: Ok(Default::default()),
                face: Ok(Default::default()),
                faces: Ok(Default::default()),
                height: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                origin: Ok(Default::default()),
                sort: Ok(Default::default()),
                suit: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                value: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttCardsCardsItem {
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
        pub fn face<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.face = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for face: {}", e));
            self
        }
        pub fn faces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttCardsCardsItemFacesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.faces = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for faces: {}", e)
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttCardsCardsItemId>,
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
        pub fn origin<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.origin = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for origin: {}", e)
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
        pub fn suit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.suit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suit: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttCardsCardsItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
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
    impl ::std::convert::TryFrom<FoundryVttCardsCardsItem>
    for super::FoundryVttCardsCardsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttCardsCardsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                drawn: value.drawn?,
                face: value.face?,
                faces: value.faces?,
                height: value.height?,
                id: value.id?,
                name: value.name?,
                origin: value.origin?,
                sort: value.sort?,
                suit: value.suit?,
                type_: value.type_?,
                value: value.value?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttCardsCardsItem>
    for FoundryVttCardsCardsItem {
        fn from(value: super::FoundryVttCardsCardsItem) -> Self {
            Self {
                description: Ok(value.description),
                drawn: Ok(value.drawn),
                face: Ok(value.face),
                faces: Ok(value.faces),
                height: Ok(value.height),
                id: Ok(value.id),
                name: Ok(value.name),
                origin: Ok(value.origin),
                sort: Ok(value.sort),
                suit: Ok(value.suit),
                type_: Ok(value.type_),
                value: Ok(value.value),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttCardsCardsItemFacesItem {
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttCardsCardsItemFacesItem {
        fn default() -> Self {
            Self {
                img: Ok(Default::default()),
                name: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttCardsCardsItemFacesItem {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttCardsCardsItemFacesItem>
    for super::FoundryVttCardsCardsItemFacesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttCardsCardsItemFacesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                img: value.img?,
                name: value.name?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttCardsCardsItemFacesItem>
    for FoundryVttCardsCardsItemFacesItem {
        fn from(value: super::FoundryVttCardsCardsItemFacesItem) -> Self {
            Self {
                img: Ok(value.img),
                name: Ok(value.name),
                text: Ok(value.text),
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
