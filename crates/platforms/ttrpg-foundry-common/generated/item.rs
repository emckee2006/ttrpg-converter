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
///Schema for Foundry VTT Pathfinder 2e item documents
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry PF2e Item",
///  "description": "Schema for Foundry VTT Pathfinder 2e item documents",
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
///        "system",
///        "type"
///      ],
///      "properties": {
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
///        "system": {
///          "description": "PF2e-specific item data",
///          "type": "object",
///          "properties": {
///            "activation": {
///              "type": "object",
///              "properties": {
///                "condition": {
///                  "type": "string"
///                },
///                "cost": {
///                  "type": "number"
///                },
///                "type": {
///                  "type": "string"
///                }
///              }
///            },
///            "area": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "armor": {
///              "type": "object",
///              "properties": {
///                "check": {
///                  "type": "number"
///                },
///                "dex": {
///                  "type": "number"
///                },
///                "speed": {
///                  "type": "number"
///                },
///                "strength": {
///                  "type": "number"
///                },
///                "value": {
///                  "type": "number"
///                }
///              }
///            },
///            "broken": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "boolean"
///                }
///              }
///            },
///            "bulk": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "number"
///                }
///              }
///            },
///            "damage": {
///              "type": "object",
///              "properties": {
///                "damageType": {
///                  "type": "string"
///                },
///                "dice": {
///                  "type": "number"
///                },
///                "die": {
///                  "type": "string"
///                }
///              }
///            },
///            "description": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "title": "HTML String",
///                  "description": "Rich text content with HTML markup",
///                  "type": "string",
///                  "$schema": "https://json-schema.org/draft-07/schema#"
///                }
///              }
///            },
///            "duration": {
///              "type": "object",
///              "properties": {
///                "units": {
///                  "type": "string"
///                },
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "equipped": {
///              "type": "object",
///              "properties": {
///                "carryType": {
///                  "type": "string",
///                  "enum": [
///                    "held",
///                    "worn",
///                    "stowed"
///                  ]
///                },
///                "handsHeld": {
///                  "type": "number",
///                  "enum": [
///                    0,
///                    1,
///                    2
///                  ]
///                },
///                "invested": {
///                  "type": "boolean"
///                }
///              }
///            },
///            "hardness": {
///              "type": "number"
///            },
///            "hp": {
///              "type": "object",
///              "properties": {
///                "max": {
///                  "type": "number"
///                },
///                "value": {
///                  "type": "number"
///                }
///              }
///            },
///            "level": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "number",
///                  "maximum": 25.0,
///                  "minimum": 0.0
///                }
///              }
///            },
///            "price": {
///              "type": "object",
///              "properties": {
///                "denomination": {
///                  "type": "string",
///                  "enum": [
///                    "pp",
///                    "gp",
///                    "sp",
///                    "cp"
///                  ]
///                },
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "quantity": {
///              "type": "number",
///              "minimum": 0.0
///            },
///            "range": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "number"
///                }
///              }
///            },
///            "reload": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "save": {
///              "type": "object",
///              "properties": {
///                "basic": {
///                  "type": "boolean"
///                },
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "target": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "time": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "traits": {
///              "type": "object",
///              "properties": {
///                "rarity": {
///                  "type": "string",
///                  "enum": [
///                    "common",
///                    "uncommon",
///                    "rare",
///                    "unique"
///                  ]
///                },
///                "value": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                }
///              }
///            },
///            "usage": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "string"
///                }
///              }
///            },
///            "weight": {
///              "type": "object",
///              "properties": {
///                "value": {
///                  "type": "number",
///                  "minimum": 0.0
///                }
///              }
///            }
///          }
///        },
///        "type": {
///          "description": "PF2e item type",
///          "type": "string",
///          "enum": [
///            "weapon",
///            "armor",
///            "equipment",
///            "consumable",
///            "treasure",
///            "backpack",
///            "kit",
///            "ancestry",
///            "background",
///            "class",
///            "feat",
///            "spell",
///            "action",
///            "lore"
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
pub struct FoundryPf2eItem {
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryPf2eItemFolder>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryPf2eItemId,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    ///Document name
    pub name: FoundryPf2eItemName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
    pub system: FoundryPf2eItemSystem,
    ///PF2e item type
    #[serde(rename = "type")]
    pub type_: FoundryPf2eItemType,
}
impl ::std::convert::From<&FoundryPf2eItem> for FoundryPf2eItem {
    fn from(value: &FoundryPf2eItem) -> Self {
        value.clone()
    }
}
impl FoundryPf2eItem {
    pub fn builder() -> builder::FoundryPf2eItem {
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
pub struct FoundryPf2eItemFolder(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eItemFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eItemFolder> for ::std::string::String {
    fn from(value: FoundryPf2eItemFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eItemFolder> for FoundryPf2eItemFolder {
    fn from(value: &FoundryPf2eItemFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eItemFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eItemFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eItemFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eItemFolder {
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
pub struct FoundryPf2eItemId(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eItemId> for ::std::string::String {
    fn from(value: FoundryPf2eItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eItemId> for FoundryPf2eItemId {
    fn from(value: &FoundryPf2eItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eItemId {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eItemId {
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
pub struct FoundryPf2eItemName(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eItemName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eItemName> for ::std::string::String {
    fn from(value: FoundryPf2eItemName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eItemName> for FoundryPf2eItemName {
    fn from(value: &FoundryPf2eItemName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eItemName {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eItemName {
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
///PF2e-specific item data
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "PF2e-specific item data",
///  "type": "object",
///  "properties": {
///    "activation": {
///      "type": "object",
///      "properties": {
///        "condition": {
///          "type": "string"
///        },
///        "cost": {
///          "type": "number"
///        },
///        "type": {
///          "type": "string"
///        }
///      }
///    },
///    "area": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "armor": {
///      "type": "object",
///      "properties": {
///        "check": {
///          "type": "number"
///        },
///        "dex": {
///          "type": "number"
///        },
///        "speed": {
///          "type": "number"
///        },
///        "strength": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "broken": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "boolean"
///        }
///      }
///    },
///    "bulk": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "damage": {
///      "type": "object",
///      "properties": {
///        "damageType": {
///          "type": "string"
///        },
///        "dice": {
///          "type": "number"
///        },
///        "die": {
///          "type": "string"
///        }
///      }
///    },
///    "description": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "title": "HTML String",
///          "description": "Rich text content with HTML markup",
///          "type": "string",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        }
///      }
///    },
///    "duration": {
///      "type": "object",
///      "properties": {
///        "units": {
///          "type": "string"
///        },
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "equipped": {
///      "type": "object",
///      "properties": {
///        "carryType": {
///          "type": "string",
///          "enum": [
///            "held",
///            "worn",
///            "stowed"
///          ]
///        },
///        "handsHeld": {
///          "type": "number",
///          "enum": [
///            0,
///            1,
///            2
///          ]
///        },
///        "invested": {
///          "type": "boolean"
///        }
///      }
///    },
///    "hardness": {
///      "type": "number"
///    },
///    "hp": {
///      "type": "object",
///      "properties": {
///        "max": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "level": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number",
///          "maximum": 25.0,
///          "minimum": 0.0
///        }
///      }
///    },
///    "price": {
///      "type": "object",
///      "properties": {
///        "denomination": {
///          "type": "string",
///          "enum": [
///            "pp",
///            "gp",
///            "sp",
///            "cp"
///          ]
///        },
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "quantity": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "range": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "reload": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "save": {
///      "type": "object",
///      "properties": {
///        "basic": {
///          "type": "boolean"
///        },
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "target": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "time": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "traits": {
///      "type": "object",
///      "properties": {
///        "rarity": {
///          "type": "string",
///          "enum": [
///            "common",
///            "uncommon",
///            "rare",
///            "unique"
///          ]
///        },
///        "value": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "usage": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "weight": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number",
///          "minimum": 0.0
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub activation: ::std::option::Option<FoundryPf2eItemSystemActivation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub area: ::std::option::Option<FoundryPf2eItemSystemArea>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub armor: ::std::option::Option<FoundryPf2eItemSystemArmor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub broken: ::std::option::Option<FoundryPf2eItemSystemBroken>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bulk: ::std::option::Option<FoundryPf2eItemSystemBulk>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub damage: ::std::option::Option<FoundryPf2eItemSystemDamage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<FoundryPf2eItemSystemDescription>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration: ::std::option::Option<FoundryPf2eItemSystemDuration>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub equipped: ::std::option::Option<FoundryPf2eItemSystemEquipped>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hardness: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hp: ::std::option::Option<FoundryPf2eItemSystemHp>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<FoundryPf2eItemSystemLevel>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<FoundryPf2eItemSystemPrice>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quantity: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<FoundryPf2eItemSystemRange>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reload: ::std::option::Option<FoundryPf2eItemSystemReload>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save: ::std::option::Option<FoundryPf2eItemSystemSave>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target: ::std::option::Option<FoundryPf2eItemSystemTarget>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub time: ::std::option::Option<FoundryPf2eItemSystemTime>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub traits: ::std::option::Option<FoundryPf2eItemSystemTraits>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<FoundryPf2eItemSystemUsage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<FoundryPf2eItemSystemWeight>,
}
impl ::std::convert::From<&FoundryPf2eItemSystem> for FoundryPf2eItemSystem {
    fn from(value: &FoundryPf2eItemSystem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystem {
    fn default() -> Self {
        Self {
            activation: Default::default(),
            area: Default::default(),
            armor: Default::default(),
            broken: Default::default(),
            bulk: Default::default(),
            damage: Default::default(),
            description: Default::default(),
            duration: Default::default(),
            equipped: Default::default(),
            hardness: Default::default(),
            hp: Default::default(),
            level: Default::default(),
            price: Default::default(),
            quantity: Default::default(),
            range: Default::default(),
            reload: Default::default(),
            save: Default::default(),
            target: Default::default(),
            time: Default::default(),
            traits: Default::default(),
            usage: Default::default(),
            weight: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystem {
    pub fn builder() -> builder::FoundryPf2eItemSystem {
        Default::default()
    }
}
///`FoundryPf2eItemSystemActivation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "condition": {
///      "type": "string"
///    },
///    "cost": {
///      "type": "number"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemActivation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cost: ::std::option::Option<f64>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemActivation>
for FoundryPf2eItemSystemActivation {
    fn from(value: &FoundryPf2eItemSystemActivation) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemActivation {
    fn default() -> Self {
        Self {
            condition: Default::default(),
            cost: Default::default(),
            type_: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemActivation {
    pub fn builder() -> builder::FoundryPf2eItemSystemActivation {
        Default::default()
    }
}
///`FoundryPf2eItemSystemArea`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemArea {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemArea> for FoundryPf2eItemSystemArea {
    fn from(value: &FoundryPf2eItemSystemArea) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemArea {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemArea {
    pub fn builder() -> builder::FoundryPf2eItemSystemArea {
        Default::default()
    }
}
///`FoundryPf2eItemSystemArmor`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "check": {
///      "type": "number"
///    },
///    "dex": {
///      "type": "number"
///    },
///    "speed": {
///      "type": "number"
///    },
///    "strength": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemArmor {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub check: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dex: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub strength: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemArmor> for FoundryPf2eItemSystemArmor {
    fn from(value: &FoundryPf2eItemSystemArmor) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemArmor {
    fn default() -> Self {
        Self {
            check: Default::default(),
            dex: Default::default(),
            speed: Default::default(),
            strength: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemArmor {
    pub fn builder() -> builder::FoundryPf2eItemSystemArmor {
        Default::default()
    }
}
///`FoundryPf2eItemSystemBroken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemBroken {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<bool>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemBroken> for FoundryPf2eItemSystemBroken {
    fn from(value: &FoundryPf2eItemSystemBroken) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemBroken {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemBroken {
    pub fn builder() -> builder::FoundryPf2eItemSystemBroken {
        Default::default()
    }
}
///`FoundryPf2eItemSystemBulk`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "number"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemBulk {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemBulk> for FoundryPf2eItemSystemBulk {
    fn from(value: &FoundryPf2eItemSystemBulk) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemBulk {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemBulk {
    pub fn builder() -> builder::FoundryPf2eItemSystemBulk {
        Default::default()
    }
}
///`FoundryPf2eItemSystemDamage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "damageType": {
///      "type": "string"
///    },
///    "dice": {
///      "type": "number"
///    },
///    "die": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemDamage {
    #[serde(
        rename = "damageType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub damage_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dice: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub die: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemDamage> for FoundryPf2eItemSystemDamage {
    fn from(value: &FoundryPf2eItemSystemDamage) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemDamage {
    fn default() -> Self {
        Self {
            damage_type: Default::default(),
            dice: Default::default(),
            die: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemDamage {
    pub fn builder() -> builder::FoundryPf2eItemSystemDamage {
        Default::default()
    }
}
///`FoundryPf2eItemSystemDescription`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "title": "HTML String",
///      "description": "Rich text content with HTML markup",
///      "type": "string",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemDescription {
    ///Rich text content with HTML markup
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemDescription>
for FoundryPf2eItemSystemDescription {
    fn from(value: &FoundryPf2eItemSystemDescription) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemDescription {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemDescription {
    pub fn builder() -> builder::FoundryPf2eItemSystemDescription {
        Default::default()
    }
}
///`FoundryPf2eItemSystemDuration`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "units": {
///      "type": "string"
///    },
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemDuration {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemDuration>
for FoundryPf2eItemSystemDuration {
    fn from(value: &FoundryPf2eItemSystemDuration) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemDuration {
    fn default() -> Self {
        Self {
            units: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemDuration {
    pub fn builder() -> builder::FoundryPf2eItemSystemDuration {
        Default::default()
    }
}
///`FoundryPf2eItemSystemEquipped`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "carryType": {
///      "type": "string",
///      "enum": [
///        "held",
///        "worn",
///        "stowed"
///      ]
///    },
///    "handsHeld": {
///      "type": "number",
///      "enum": [
///        0,
///        1,
///        2
///      ]
///    },
///    "invested": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemEquipped {
    #[serde(
        rename = "carryType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub carry_type: ::std::option::Option<FoundryPf2eItemSystemEquippedCarryType>,
    #[serde(
        rename = "handsHeld",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hands_held: ::std::option::Option<FoundryPf2eItemSystemEquippedHandsHeld>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invested: ::std::option::Option<bool>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemEquipped>
for FoundryPf2eItemSystemEquipped {
    fn from(value: &FoundryPf2eItemSystemEquipped) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemEquipped {
    fn default() -> Self {
        Self {
            carry_type: Default::default(),
            hands_held: Default::default(),
            invested: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemEquipped {
    pub fn builder() -> builder::FoundryPf2eItemSystemEquipped {
        Default::default()
    }
}
///`FoundryPf2eItemSystemEquippedCarryType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "held",
///    "worn",
///    "stowed"
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
pub enum FoundryPf2eItemSystemEquippedCarryType {
    #[serde(rename = "held")]
    Held,
    #[serde(rename = "worn")]
    Worn,
    #[serde(rename = "stowed")]
    Stowed,
}
impl ::std::convert::From<&Self> for FoundryPf2eItemSystemEquippedCarryType {
    fn from(value: &FoundryPf2eItemSystemEquippedCarryType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eItemSystemEquippedCarryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Held => f.write_str("held"),
            Self::Worn => f.write_str("worn"),
            Self::Stowed => f.write_str("stowed"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eItemSystemEquippedCarryType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "held" => Ok(Self::Held),
            "worn" => Ok(Self::Worn),
            "stowed" => Ok(Self::Stowed),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemSystemEquippedCarryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryPf2eItemSystemEquippedCarryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryPf2eItemSystemEquippedCarryType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryPf2eItemSystemEquippedHandsHeld`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
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
pub struct FoundryPf2eItemSystemEquippedHandsHeld(f64);
impl ::std::ops::Deref for FoundryPf2eItemSystemEquippedHandsHeld {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eItemSystemEquippedHandsHeld> for f64 {
    fn from(value: FoundryPf2eItemSystemEquippedHandsHeld) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eItemSystemEquippedHandsHeld>
for FoundryPf2eItemSystemEquippedHandsHeld {
    fn from(value: &FoundryPf2eItemSystemEquippedHandsHeld) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<f64> for FoundryPf2eItemSystemEquippedHandsHeld {
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
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eItemSystemEquippedHandsHeld {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
    }
}
///`FoundryPf2eItemSystemHp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemHp {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemHp> for FoundryPf2eItemSystemHp {
    fn from(value: &FoundryPf2eItemSystemHp) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemHp {
    fn default() -> Self {
        Self {
            max: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemHp {
    pub fn builder() -> builder::FoundryPf2eItemSystemHp {
        Default::default()
    }
}
///`FoundryPf2eItemSystemLevel`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "number",
///      "maximum": 25.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemLevel {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemLevel> for FoundryPf2eItemSystemLevel {
    fn from(value: &FoundryPf2eItemSystemLevel) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemLevel {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemLevel {
    pub fn builder() -> builder::FoundryPf2eItemSystemLevel {
        Default::default()
    }
}
///`FoundryPf2eItemSystemPrice`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "denomination": {
///      "type": "string",
///      "enum": [
///        "pp",
///        "gp",
///        "sp",
///        "cp"
///      ]
///    },
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemPrice {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub denomination: ::std::option::Option<FoundryPf2eItemSystemPriceDenomination>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemPrice> for FoundryPf2eItemSystemPrice {
    fn from(value: &FoundryPf2eItemSystemPrice) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemPrice {
    fn default() -> Self {
        Self {
            denomination: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemPrice {
    pub fn builder() -> builder::FoundryPf2eItemSystemPrice {
        Default::default()
    }
}
///`FoundryPf2eItemSystemPriceDenomination`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "pp",
///    "gp",
///    "sp",
///    "cp"
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
pub enum FoundryPf2eItemSystemPriceDenomination {
    #[serde(rename = "pp")]
    Pp,
    #[serde(rename = "gp")]
    Gp,
    #[serde(rename = "sp")]
    Sp,
    #[serde(rename = "cp")]
    Cp,
}
impl ::std::convert::From<&Self> for FoundryPf2eItemSystemPriceDenomination {
    fn from(value: &FoundryPf2eItemSystemPriceDenomination) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eItemSystemPriceDenomination {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pp => f.write_str("pp"),
            Self::Gp => f.write_str("gp"),
            Self::Sp => f.write_str("sp"),
            Self::Cp => f.write_str("cp"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eItemSystemPriceDenomination {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "pp" => Ok(Self::Pp),
            "gp" => Ok(Self::Gp),
            "sp" => Ok(Self::Sp),
            "cp" => Ok(Self::Cp),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemSystemPriceDenomination {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryPf2eItemSystemPriceDenomination {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryPf2eItemSystemPriceDenomination {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryPf2eItemSystemRange`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "number"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemRange {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemRange> for FoundryPf2eItemSystemRange {
    fn from(value: &FoundryPf2eItemSystemRange) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemRange {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemRange {
    pub fn builder() -> builder::FoundryPf2eItemSystemRange {
        Default::default()
    }
}
///`FoundryPf2eItemSystemReload`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemReload {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemReload> for FoundryPf2eItemSystemReload {
    fn from(value: &FoundryPf2eItemSystemReload) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemReload {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemReload {
    pub fn builder() -> builder::FoundryPf2eItemSystemReload {
        Default::default()
    }
}
///`FoundryPf2eItemSystemSave`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "basic": {
///      "type": "boolean"
///    },
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemSave {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub basic: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemSave> for FoundryPf2eItemSystemSave {
    fn from(value: &FoundryPf2eItemSystemSave) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemSave {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemSave {
    pub fn builder() -> builder::FoundryPf2eItemSystemSave {
        Default::default()
    }
}
///`FoundryPf2eItemSystemTarget`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemTarget {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemTarget> for FoundryPf2eItemSystemTarget {
    fn from(value: &FoundryPf2eItemSystemTarget) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemTarget {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemTarget {
    pub fn builder() -> builder::FoundryPf2eItemSystemTarget {
        Default::default()
    }
}
///`FoundryPf2eItemSystemTime`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemTime {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemTime> for FoundryPf2eItemSystemTime {
    fn from(value: &FoundryPf2eItemSystemTime) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemTime {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemTime {
    pub fn builder() -> builder::FoundryPf2eItemSystemTime {
        Default::default()
    }
}
///`FoundryPf2eItemSystemTraits`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "rarity": {
///      "type": "string",
///      "enum": [
///        "common",
///        "uncommon",
///        "rare",
///        "unique"
///      ]
///    },
///    "value": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemTraits {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rarity: ::std::option::Option<FoundryPf2eItemSystemTraitsRarity>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub value: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemTraits> for FoundryPf2eItemSystemTraits {
    fn from(value: &FoundryPf2eItemSystemTraits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemTraits {
    fn default() -> Self {
        Self {
            rarity: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eItemSystemTraits {
    pub fn builder() -> builder::FoundryPf2eItemSystemTraits {
        Default::default()
    }
}
///`FoundryPf2eItemSystemTraitsRarity`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "common",
///    "uncommon",
///    "rare",
///    "unique"
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
pub enum FoundryPf2eItemSystemTraitsRarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "unique")]
    Unique,
}
impl ::std::convert::From<&Self> for FoundryPf2eItemSystemTraitsRarity {
    fn from(value: &FoundryPf2eItemSystemTraitsRarity) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eItemSystemTraitsRarity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Common => f.write_str("common"),
            Self::Uncommon => f.write_str("uncommon"),
            Self::Rare => f.write_str("rare"),
            Self::Unique => f.write_str("unique"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eItemSystemTraitsRarity {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "common" => Ok(Self::Common),
            "uncommon" => Ok(Self::Uncommon),
            "rare" => Ok(Self::Rare),
            "unique" => Ok(Self::Unique),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemSystemTraitsRarity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryPf2eItemSystemTraitsRarity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryPf2eItemSystemTraitsRarity {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryPf2eItemSystemUsage`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemUsage {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemUsage> for FoundryPf2eItemSystemUsage {
    fn from(value: &FoundryPf2eItemSystemUsage) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemUsage {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemUsage {
    pub fn builder() -> builder::FoundryPf2eItemSystemUsage {
        Default::default()
    }
}
///`FoundryPf2eItemSystemWeight`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "number",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eItemSystemWeight {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eItemSystemWeight> for FoundryPf2eItemSystemWeight {
    fn from(value: &FoundryPf2eItemSystemWeight) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eItemSystemWeight {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eItemSystemWeight {
    pub fn builder() -> builder::FoundryPf2eItemSystemWeight {
        Default::default()
    }
}
///PF2e item type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "PF2e item type",
///  "type": "string",
///  "enum": [
///    "weapon",
///    "armor",
///    "equipment",
///    "consumable",
///    "treasure",
///    "backpack",
///    "kit",
///    "ancestry",
///    "background",
///    "class",
///    "feat",
///    "spell",
///    "action",
///    "lore"
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
pub enum FoundryPf2eItemType {
    #[serde(rename = "weapon")]
    Weapon,
    #[serde(rename = "armor")]
    Armor,
    #[serde(rename = "equipment")]
    Equipment,
    #[serde(rename = "consumable")]
    Consumable,
    #[serde(rename = "treasure")]
    Treasure,
    #[serde(rename = "backpack")]
    Backpack,
    #[serde(rename = "kit")]
    Kit,
    #[serde(rename = "ancestry")]
    Ancestry,
    #[serde(rename = "background")]
    Background,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "feat")]
    Feat,
    #[serde(rename = "spell")]
    Spell,
    #[serde(rename = "action")]
    Action,
    #[serde(rename = "lore")]
    Lore,
}
impl ::std::convert::From<&Self> for FoundryPf2eItemType {
    fn from(value: &FoundryPf2eItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Weapon => f.write_str("weapon"),
            Self::Armor => f.write_str("armor"),
            Self::Equipment => f.write_str("equipment"),
            Self::Consumable => f.write_str("consumable"),
            Self::Treasure => f.write_str("treasure"),
            Self::Backpack => f.write_str("backpack"),
            Self::Kit => f.write_str("kit"),
            Self::Ancestry => f.write_str("ancestry"),
            Self::Background => f.write_str("background"),
            Self::Class => f.write_str("class"),
            Self::Feat => f.write_str("feat"),
            Self::Spell => f.write_str("spell"),
            Self::Action => f.write_str("action"),
            Self::Lore => f.write_str("lore"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "weapon" => Ok(Self::Weapon),
            "armor" => Ok(Self::Armor),
            "equipment" => Ok(Self::Equipment),
            "consumable" => Ok(Self::Consumable),
            "treasure" => Ok(Self::Treasure),
            "backpack" => Ok(Self::Backpack),
            "kit" => Ok(Self::Kit),
            "ancestry" => Ok(Self::Ancestry),
            "background" => Ok(Self::Background),
            "class" => Ok(Self::Class),
            "feat" => Ok(Self::Feat),
            "spell" => Ok(Self::Spell),
            "action" => Ok(Self::Action),
            "lore" => Ok(Self::Lore),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eItemType {
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
    pub struct FoundryPf2eItem {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemFolder>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryPf2eItemId, ::std::string::String>,
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::FoundryPf2eItemName, ::std::string::String>,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            super::FoundryPf2eItemSystem,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FoundryPf2eItemType, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItem {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                img: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
                system: Err("no value supplied for system".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryPf2eItem {
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
                ::std::option::Option<super::FoundryPf2eItemFolder>,
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
            T: ::std::convert::TryInto<super::FoundryPf2eItemId>,
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
            T: ::std::convert::TryInto<super::FoundryPf2eItemName>,
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
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryPf2eItemSystem>,
            T::Error: ::std::fmt::Display,
        {
            self.system = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryPf2eItemType>,
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
    impl ::std::convert::TryFrom<FoundryPf2eItem> for super::FoundryPf2eItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                folder: value.folder?,
                id: value.id?,
                img: value.img?,
                name: value.name?,
                ownership: value.ownership?,
                sort: value.sort?,
                stats: value.stats?,
                system: value.system?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItem> for FoundryPf2eItem {
        fn from(value: super::FoundryPf2eItem) -> Self {
            Self {
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                id: Ok(value.id),
                img: Ok(value.img),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
                system: Ok(value.system),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystem {
        activation: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemActivation>,
            ::std::string::String,
        >,
        area: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemArea>,
            ::std::string::String,
        >,
        armor: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemArmor>,
            ::std::string::String,
        >,
        broken: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemBroken>,
            ::std::string::String,
        >,
        bulk: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemBulk>,
            ::std::string::String,
        >,
        damage: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemDamage>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemDescription>,
            ::std::string::String,
        >,
        duration: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemDuration>,
            ::std::string::String,
        >,
        equipped: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemEquipped>,
            ::std::string::String,
        >,
        hardness: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        hp: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemHp>,
            ::std::string::String,
        >,
        level: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemLevel>,
            ::std::string::String,
        >,
        price: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemPrice>,
            ::std::string::String,
        >,
        quantity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        range: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemRange>,
            ::std::string::String,
        >,
        reload: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemReload>,
            ::std::string::String,
        >,
        save: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemSave>,
            ::std::string::String,
        >,
        target: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemTarget>,
            ::std::string::String,
        >,
        time: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemTime>,
            ::std::string::String,
        >,
        traits: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemTraits>,
            ::std::string::String,
        >,
        usage: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemUsage>,
            ::std::string::String,
        >,
        weight: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemWeight>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystem {
        fn default() -> Self {
            Self {
                activation: Ok(Default::default()),
                area: Ok(Default::default()),
                armor: Ok(Default::default()),
                broken: Ok(Default::default()),
                bulk: Ok(Default::default()),
                damage: Ok(Default::default()),
                description: Ok(Default::default()),
                duration: Ok(Default::default()),
                equipped: Ok(Default::default()),
                hardness: Ok(Default::default()),
                hp: Ok(Default::default()),
                level: Ok(Default::default()),
                price: Ok(Default::default()),
                quantity: Ok(Default::default()),
                range: Ok(Default::default()),
                reload: Ok(Default::default()),
                save: Ok(Default::default()),
                target: Ok(Default::default()),
                time: Ok(Default::default()),
                traits: Ok(Default::default()),
                usage: Ok(Default::default()),
                weight: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystem {
        pub fn activation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemActivation>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.activation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for activation: {}", e)
                });
            self
        }
        pub fn area<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemArea>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.area = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for area: {}", e));
            self
        }
        pub fn armor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemArmor>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.armor = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for armor: {}", e)
                });
            self
        }
        pub fn broken<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemBroken>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.broken = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for broken: {}", e)
                });
            self
        }
        pub fn bulk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemBulk>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.bulk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bulk: {}", e));
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemDamage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for damage: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemDescription>,
            >,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemDuration>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.duration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for duration: {}", e)
                });
            self
        }
        pub fn equipped<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemEquipped>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.equipped = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for equipped: {}", e)
                });
            self
        }
        pub fn hardness<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.hardness = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hardness: {}", e)
                });
            self
        }
        pub fn hp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemHp>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemLevel>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for level: {}", e)
                });
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemPrice>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for price: {}", e)
                });
            self
        }
        pub fn quantity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.quantity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for quantity: {}", e)
                });
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemRange>,
            >,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemReload>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reload: {}", e)
                });
            self
        }
        pub fn save<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemSave>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.save = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for save: {}", e));
            self
        }
        pub fn target<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemTarget>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.target = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for target: {}", e)
                });
            self
        }
        pub fn time<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemTime>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.time = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for time: {}", e));
            self
        }
        pub fn traits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemTraits>,
            >,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemUsage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for usage: {}", e)
                });
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemWeight>,
            >,
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
    impl ::std::convert::TryFrom<FoundryPf2eItemSystem>
    for super::FoundryPf2eItemSystem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                activation: value.activation?,
                area: value.area?,
                armor: value.armor?,
                broken: value.broken?,
                bulk: value.bulk?,
                damage: value.damage?,
                description: value.description?,
                duration: value.duration?,
                equipped: value.equipped?,
                hardness: value.hardness?,
                hp: value.hp?,
                level: value.level?,
                price: value.price?,
                quantity: value.quantity?,
                range: value.range?,
                reload: value.reload?,
                save: value.save?,
                target: value.target?,
                time: value.time?,
                traits: value.traits?,
                usage: value.usage?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystem> for FoundryPf2eItemSystem {
        fn from(value: super::FoundryPf2eItemSystem) -> Self {
            Self {
                activation: Ok(value.activation),
                area: Ok(value.area),
                armor: Ok(value.armor),
                broken: Ok(value.broken),
                bulk: Ok(value.bulk),
                damage: Ok(value.damage),
                description: Ok(value.description),
                duration: Ok(value.duration),
                equipped: Ok(value.equipped),
                hardness: Ok(value.hardness),
                hp: Ok(value.hp),
                level: Ok(value.level),
                price: Ok(value.price),
                quantity: Ok(value.quantity),
                range: Ok(value.range),
                reload: Ok(value.reload),
                save: Ok(value.save),
                target: Ok(value.target),
                time: Ok(value.time),
                traits: Ok(value.traits),
                usage: Ok(value.usage),
                weight: Ok(value.weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemActivation {
        condition: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cost: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemActivation {
        fn default() -> Self {
            Self {
                condition: Ok(Default::default()),
                cost: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemActivation {
        pub fn condition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.condition = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for condition: {}", e)
                });
            self
        }
        pub fn cost<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cost = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cost: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemActivation>
    for super::FoundryPf2eItemSystemActivation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemActivation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                condition: value.condition?,
                cost: value.cost?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemActivation>
    for FoundryPf2eItemSystemActivation {
        fn from(value: super::FoundryPf2eItemSystemActivation) -> Self {
            Self {
                condition: Ok(value.condition),
                cost: Ok(value.cost),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemArea {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemArea {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemArea {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemArea>
    for super::FoundryPf2eItemSystemArea {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemArea,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemArea>
    for FoundryPf2eItemSystemArea {
        fn from(value: super::FoundryPf2eItemSystemArea) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemArmor {
        check: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        dex: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        speed: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        strength: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemArmor {
        fn default() -> Self {
            Self {
                check: Ok(Default::default()),
                dex: Ok(Default::default()),
                speed: Ok(Default::default()),
                strength: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemArmor {
        pub fn check<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.check = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for check: {}", e)
                });
            self
        }
        pub fn dex<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dex: {}", e));
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed: {}", e)
                });
            self
        }
        pub fn strength<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.strength = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for strength: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemArmor>
    for super::FoundryPf2eItemSystemArmor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemArmor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                check: value.check?,
                dex: value.dex?,
                speed: value.speed?,
                strength: value.strength?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemArmor>
    for FoundryPf2eItemSystemArmor {
        fn from(value: super::FoundryPf2eItemSystemArmor) -> Self {
            Self {
                check: Ok(value.check),
                dex: Ok(value.dex),
                speed: Ok(value.speed),
                strength: Ok(value.strength),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemBroken {
        value: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemBroken {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemBroken {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemBroken>
    for super::FoundryPf2eItemSystemBroken {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemBroken,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemBroken>
    for FoundryPf2eItemSystemBroken {
        fn from(value: super::FoundryPf2eItemSystemBroken) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemBulk {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemBulk {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemBulk {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemBulk>
    for super::FoundryPf2eItemSystemBulk {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemBulk,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemBulk>
    for FoundryPf2eItemSystemBulk {
        fn from(value: super::FoundryPf2eItemSystemBulk) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemDamage {
        damage_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        dice: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        die: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemDamage {
        fn default() -> Self {
            Self {
                damage_type: Ok(Default::default()),
                dice: Ok(Default::default()),
                die: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemDamage {
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
        pub fn dice<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dice = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dice: {}", e));
            self
        }
        pub fn die<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.die = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for die: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemDamage>
    for super::FoundryPf2eItemSystemDamage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemDamage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                damage_type: value.damage_type?,
                dice: value.dice?,
                die: value.die?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemDamage>
    for FoundryPf2eItemSystemDamage {
        fn from(value: super::FoundryPf2eItemSystemDamage) -> Self {
            Self {
                damage_type: Ok(value.damage_type),
                dice: Ok(value.dice),
                die: Ok(value.die),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemDescription {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemDescription {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemDescription {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemDescription>
    for super::FoundryPf2eItemSystemDescription {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemDescription,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemDescription>
    for FoundryPf2eItemSystemDescription {
        fn from(value: super::FoundryPf2eItemSystemDescription) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemDuration {
        units: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemDuration {
        fn default() -> Self {
            Self {
                units: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemDuration {
        pub fn units<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.units = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for units: {}", e)
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemDuration>
    for super::FoundryPf2eItemSystemDuration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemDuration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                units: value.units?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemDuration>
    for FoundryPf2eItemSystemDuration {
        fn from(value: super::FoundryPf2eItemSystemDuration) -> Self {
            Self {
                units: Ok(value.units),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemEquipped {
        carry_type: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemEquippedCarryType>,
            ::std::string::String,
        >,
        hands_held: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemEquippedHandsHeld>,
            ::std::string::String,
        >,
        invested: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemEquipped {
        fn default() -> Self {
            Self {
                carry_type: Ok(Default::default()),
                hands_held: Ok(Default::default()),
                invested: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemEquipped {
        pub fn carry_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemEquippedCarryType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.carry_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for carry_type: {}", e)
                });
            self
        }
        pub fn hands_held<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemEquippedHandsHeld>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hands_held = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hands_held: {}", e)
                });
            self
        }
        pub fn invested<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.invested = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for invested: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemEquipped>
    for super::FoundryPf2eItemSystemEquipped {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemEquipped,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                carry_type: value.carry_type?,
                hands_held: value.hands_held?,
                invested: value.invested?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemEquipped>
    for FoundryPf2eItemSystemEquipped {
        fn from(value: super::FoundryPf2eItemSystemEquipped) -> Self {
            Self {
                carry_type: Ok(value.carry_type),
                hands_held: Ok(value.hands_held),
                invested: Ok(value.invested),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemHp {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemHp {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemHp {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemHp>
    for super::FoundryPf2eItemSystemHp {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemHp,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemHp>
    for FoundryPf2eItemSystemHp {
        fn from(value: super::FoundryPf2eItemSystemHp) -> Self {
            Self {
                max: Ok(value.max),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemLevel {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemLevel {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemLevel {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemLevel>
    for super::FoundryPf2eItemSystemLevel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemLevel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemLevel>
    for FoundryPf2eItemSystemLevel {
        fn from(value: super::FoundryPf2eItemSystemLevel) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemPrice {
        denomination: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemPriceDenomination>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemPrice {
        fn default() -> Self {
            Self {
                denomination: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemPrice {
        pub fn denomination<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemPriceDenomination>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.denomination = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for denomination: {}", e)
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemPrice>
    for super::FoundryPf2eItemSystemPrice {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemPrice,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                denomination: value.denomination?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemPrice>
    for FoundryPf2eItemSystemPrice {
        fn from(value: super::FoundryPf2eItemSystemPrice) -> Self {
            Self {
                denomination: Ok(value.denomination),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemRange {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemRange {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemRange {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemRange>
    for super::FoundryPf2eItemSystemRange {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemRange,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemRange>
    for FoundryPf2eItemSystemRange {
        fn from(value: super::FoundryPf2eItemSystemRange) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemReload {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemReload {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemReload {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemReload>
    for super::FoundryPf2eItemSystemReload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemReload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemReload>
    for FoundryPf2eItemSystemReload {
        fn from(value: super::FoundryPf2eItemSystemReload) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemSave {
        basic: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemSave {
        fn default() -> Self {
            Self {
                basic: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemSave {
        pub fn basic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.basic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for basic: {}", e)
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemSave>
    for super::FoundryPf2eItemSystemSave {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemSave,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                basic: value.basic?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemSave>
    for FoundryPf2eItemSystemSave {
        fn from(value: super::FoundryPf2eItemSystemSave) -> Self {
            Self {
                basic: Ok(value.basic),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemTarget {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemTarget {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemTarget {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemTarget>
    for super::FoundryPf2eItemSystemTarget {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemTarget,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemTarget>
    for FoundryPf2eItemSystemTarget {
        fn from(value: super::FoundryPf2eItemSystemTarget) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemTime {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemTime {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemTime {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemTime>
    for super::FoundryPf2eItemSystemTime {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemTime,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemTime>
    for FoundryPf2eItemSystemTime {
        fn from(value: super::FoundryPf2eItemSystemTime) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemTraits {
        rarity: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eItemSystemTraitsRarity>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemTraits {
        fn default() -> Self {
            Self {
                rarity: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemTraits {
        pub fn rarity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eItemSystemTraitsRarity>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.rarity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for rarity: {}", e)
                });
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemTraits>
    for super::FoundryPf2eItemSystemTraits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemTraits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rarity: value.rarity?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemTraits>
    for FoundryPf2eItemSystemTraits {
        fn from(value: super::FoundryPf2eItemSystemTraits) -> Self {
            Self {
                rarity: Ok(value.rarity),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemUsage {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemUsage {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemUsage {
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemUsage>
    for super::FoundryPf2eItemSystemUsage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemUsage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemUsage>
    for FoundryPf2eItemSystemUsage {
        fn from(value: super::FoundryPf2eItemSystemUsage) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eItemSystemWeight {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eItemSystemWeight {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eItemSystemWeight {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eItemSystemWeight>
    for super::FoundryPf2eItemSystemWeight {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eItemSystemWeight,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eItemSystemWeight>
    for FoundryPf2eItemSystemWeight {
        fn from(value: super::FoundryPf2eItemSystemWeight) -> Self {
            Self { value: Ok(value.value) }
        }
    }
}
