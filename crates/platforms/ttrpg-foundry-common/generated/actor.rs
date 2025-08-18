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
///Schema for Foundry VTT Pathfinder 2e actor documents
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry PF2e Actor",
///  "description": "Schema for Foundry VTT Pathfinder 2e actor documents",
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
///        "items": {
///          "description": "Owned items",
///          "type": "array",
///          "items": {
///            "type": "string",
///            "pattern": "^[a-zA-Z0-9]{16}$"
///          }
///        },
///        "system": {
///          "description": "PF2e-specific actor data",
///          "type": "object",
///          "properties": {
///            "abilities": {
///              "type": "object",
///              "properties": {
///                "cha": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                },
///                "con": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                },
///                "dex": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                },
///                "int": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                },
///                "str": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                },
///                "wis": {
///                  "type": "object",
///                  "properties": {
///                    "mod": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number",
///                      "maximum": 30.0,
///                      "minimum": 1.0
///                    }
///                  }
///                }
///              }
///            },
///            "attributes": {
///              "type": "object",
///              "properties": {
///                "ac": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                },
///                "classDC": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                },
///                "hp": {
///                  "type": "object",
///                  "properties": {
///                    "max": {
///                      "type": "number",
///                      "minimum": 1.0
///                    },
///                    "temp": {
///                      "type": "number",
///                      "minimum": 0.0
///                    },
///                    "value": {
///                      "type": "number",
///                      "minimum": 0.0
///                    }
///                  }
///                },
///                "perception": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                },
///                "speed": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "number",
///                      "minimum": 0.0
///                    }
///                  }
///                }
///              }
///            },
///            "currency": {
///              "type": "object",
///              "properties": {
///                "cp": {
///                  "type": "number",
///                  "minimum": 0.0
///                },
///                "gp": {
///                  "type": "number",
///                  "minimum": 0.0
///                },
///                "pp": {
///                  "type": "number",
///                  "minimum": 0.0
///                },
///                "sp": {
///                  "type": "number",
///                  "minimum": 0.0
///                }
///              }
///            },
///            "details": {
///              "type": "object",
///              "properties": {
///                "alignment": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "ancestry": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "background": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "biography": {
///                  "type": "object",
///                  "properties": {
///                    "public": {
///                      "title": "HTML String",
///                      "description": "Rich text content with HTML markup",
///                      "type": "string",
///                      "$schema": "https://json-schema.org/draft-07/schema#"
///                    },
///                    "value": {
///                      "title": "HTML String",
///                      "description": "Rich text content with HTML markup",
///                      "type": "string",
///                      "$schema": "https://json-schema.org/draft-07/schema#"
///                    }
///                  }
///                },
///                "class": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "heritage": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string"
///                    }
///                  }
///                },
///                "keyability": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string",
///                      "enum": [
///                        "str",
///                        "dex",
///                        "con",
///                        "int",
///                        "wis",
///                        "cha"
///                      ]
///                    }
///                  }
///                },
///                "level": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "number",
///                      "maximum": 20.0,
///                      "minimum": 1.0
///                    }
///                  }
///                }
///              }
///            },
///            "saves": {
///              "type": "object",
///              "properties": {
///                "fortitude": {
///                  "type": "object",
///                  "properties": {
///                    "proficient": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                },
///                "reflex": {
///                  "type": "object",
///                  "properties": {
///                    "proficient": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                },
///                "will": {
///                  "type": "object",
///                  "properties": {
///                    "proficient": {
///                      "type": "number"
///                    },
///                    "value": {
///                      "type": "number"
///                    }
///                  }
///                }
///              }
///            },
///            "traits": {
///              "type": "object",
///              "properties": {
///                "languages": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "array",
///                      "items": {
///                        "type": "string"
///                      }
///                    }
///                  }
///                },
///                "senses": {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                "size": {
///                  "type": "object",
///                  "properties": {
///                    "value": {
///                      "type": "string",
///                      "enum": [
///                        "tiny",
///                        "sm",
///                        "med",
///                        "lg",
///                        "huge",
///                        "grg"
///                      ]
///                    }
///                  }
///                }
///              }
///            }
///          }
///        },
///        "type": {
///          "description": "PF2e actor type",
///          "type": "string",
///          "enum": [
///            "character",
///            "npc",
///            "hazard",
///            "loot",
///            "familiar",
///            "vehicle"
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
pub struct FoundryPf2eActor {
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundryPf2eActorFolder>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundryPf2eActorId,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    ///Owned items
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub items: ::std::vec::Vec<FoundryPf2eActorItemsItem>,
    ///Document name
    pub name: FoundryPf2eActorName,
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
    pub system: FoundryPf2eActorSystem,
    ///PF2e actor type
    #[serde(rename = "type")]
    pub type_: FoundryPf2eActorType,
}
impl ::std::convert::From<&FoundryPf2eActor> for FoundryPf2eActor {
    fn from(value: &FoundryPf2eActor) -> Self {
        value.clone()
    }
}
impl FoundryPf2eActor {
    pub fn builder() -> builder::FoundryPf2eActor {
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
pub struct FoundryPf2eActorFolder(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eActorFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eActorFolder> for ::std::string::String {
    fn from(value: FoundryPf2eActorFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eActorFolder> for FoundryPf2eActorFolder {
    fn from(value: &FoundryPf2eActorFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eActorFolder {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eActorFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eActorFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eActorFolder {
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
pub struct FoundryPf2eActorId(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eActorId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eActorId> for ::std::string::String {
    fn from(value: FoundryPf2eActorId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eActorId> for FoundryPf2eActorId {
    fn from(value: &FoundryPf2eActorId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eActorId {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eActorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eActorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eActorId {
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
///`FoundryPf2eActorItemsItem`
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
pub struct FoundryPf2eActorItemsItem(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eActorItemsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eActorItemsItem> for ::std::string::String {
    fn from(value: FoundryPf2eActorItemsItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eActorItemsItem> for FoundryPf2eActorItemsItem {
    fn from(value: &FoundryPf2eActorItemsItem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eActorItemsItem {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorItemsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eActorItemsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eActorItemsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eActorItemsItem {
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
pub struct FoundryPf2eActorName(::std::string::String);
impl ::std::ops::Deref for FoundryPf2eActorName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryPf2eActorName> for ::std::string::String {
    fn from(value: FoundryPf2eActorName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryPf2eActorName> for FoundryPf2eActorName {
    fn from(value: &FoundryPf2eActorName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryPf2eActorName {
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
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eActorName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eActorName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryPf2eActorName {
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
///PF2e-specific actor data
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "PF2e-specific actor data",
///  "type": "object",
///  "properties": {
///    "abilities": {
///      "type": "object",
///      "properties": {
///        "cha": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        },
///        "con": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        },
///        "dex": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        },
///        "int": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        },
///        "str": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        },
///        "wis": {
///          "type": "object",
///          "properties": {
///            "mod": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number",
///              "maximum": 30.0,
///              "minimum": 1.0
///            }
///          }
///        }
///      }
///    },
///    "attributes": {
///      "type": "object",
///      "properties": {
///        "ac": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "number"
///            }
///          }
///        },
///        "classDC": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "number"
///            }
///          }
///        },
///        "hp": {
///          "type": "object",
///          "properties": {
///            "max": {
///              "type": "number",
///              "minimum": 1.0
///            },
///            "temp": {
///              "type": "number",
///              "minimum": 0.0
///            },
///            "value": {
///              "type": "number",
///              "minimum": 0.0
///            }
///          }
///        },
///        "perception": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "number"
///            }
///          }
///        },
///        "speed": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "number",
///              "minimum": 0.0
///            }
///          }
///        }
///      }
///    },
///    "currency": {
///      "type": "object",
///      "properties": {
///        "cp": {
///          "type": "number",
///          "minimum": 0.0
///        },
///        "gp": {
///          "type": "number",
///          "minimum": 0.0
///        },
///        "pp": {
///          "type": "number",
///          "minimum": 0.0
///        },
///        "sp": {
///          "type": "number",
///          "minimum": 0.0
///        }
///      }
///    },
///    "details": {
///      "type": "object",
///      "properties": {
///        "alignment": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string"
///            }
///          }
///        },
///        "ancestry": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string"
///            }
///          }
///        },
///        "background": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string"
///            }
///          }
///        },
///        "biography": {
///          "type": "object",
///          "properties": {
///            "public": {
///              "title": "HTML String",
///              "description": "Rich text content with HTML markup",
///              "type": "string",
///              "$schema": "https://json-schema.org/draft-07/schema#"
///            },
///            "value": {
///              "title": "HTML String",
///              "description": "Rich text content with HTML markup",
///              "type": "string",
///              "$schema": "https://json-schema.org/draft-07/schema#"
///            }
///          }
///        },
///        "class": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string"
///            }
///          }
///        },
///        "heritage": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string"
///            }
///          }
///        },
///        "keyability": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string",
///              "enum": [
///                "str",
///                "dex",
///                "con",
///                "int",
///                "wis",
///                "cha"
///              ]
///            }
///          }
///        },
///        "level": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "number",
///              "maximum": 20.0,
///              "minimum": 1.0
///            }
///          }
///        }
///      }
///    },
///    "saves": {
///      "type": "object",
///      "properties": {
///        "fortitude": {
///          "type": "object",
///          "properties": {
///            "proficient": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number"
///            }
///          }
///        },
///        "reflex": {
///          "type": "object",
///          "properties": {
///            "proficient": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number"
///            }
///          }
///        },
///        "will": {
///          "type": "object",
///          "properties": {
///            "proficient": {
///              "type": "number"
///            },
///            "value": {
///              "type": "number"
///            }
///          }
///        }
///      }
///    },
///    "traits": {
///      "type": "object",
///      "properties": {
///        "languages": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            }
///          }
///        },
///        "senses": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        "size": {
///          "type": "object",
///          "properties": {
///            "value": {
///              "type": "string",
///              "enum": [
///                "tiny",
///                "sm",
///                "med",
///                "lg",
///                "huge",
///                "grg"
///              ]
///            }
///          }
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub abilities: ::std::option::Option<FoundryPf2eActorSystemAbilities>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub attributes: ::std::option::Option<FoundryPf2eActorSystemAttributes>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<FoundryPf2eActorSystemCurrency>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<FoundryPf2eActorSystemDetails>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub saves: ::std::option::Option<FoundryPf2eActorSystemSaves>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub traits: ::std::option::Option<FoundryPf2eActorSystemTraits>,
}
impl ::std::convert::From<&FoundryPf2eActorSystem> for FoundryPf2eActorSystem {
    fn from(value: &FoundryPf2eActorSystem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystem {
    fn default() -> Self {
        Self {
            abilities: Default::default(),
            attributes: Default::default(),
            currency: Default::default(),
            details: Default::default(),
            saves: Default::default(),
            traits: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystem {
    pub fn builder() -> builder::FoundryPf2eActorSystem {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilities`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "cha": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    },
///    "con": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    },
///    "dex": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    },
///    "int": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    },
///    "str": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    },
///    "wis": {
///      "type": "object",
///      "properties": {
///        "mod": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number",
///          "maximum": 30.0,
///          "minimum": 1.0
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilities {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cha: ::std::option::Option<FoundryPf2eActorSystemAbilitiesCha>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub con: ::std::option::Option<FoundryPf2eActorSystemAbilitiesCon>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dex: ::std::option::Option<FoundryPf2eActorSystemAbilitiesDex>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub int: ::std::option::Option<FoundryPf2eActorSystemAbilitiesInt>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub str: ::std::option::Option<FoundryPf2eActorSystemAbilitiesStr>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wis: ::std::option::Option<FoundryPf2eActorSystemAbilitiesWis>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilities>
for FoundryPf2eActorSystemAbilities {
    fn from(value: &FoundryPf2eActorSystemAbilities) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilities {
    fn default() -> Self {
        Self {
            cha: Default::default(),
            con: Default::default(),
            dex: Default::default(),
            int: Default::default(),
            str: Default::default(),
            wis: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilities {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilities {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesCha`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesCha {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesCha>
for FoundryPf2eActorSystemAbilitiesCha {
    fn from(value: &FoundryPf2eActorSystemAbilitiesCha) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesCha {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesCha {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesCha {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesCon`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesCon {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesCon>
for FoundryPf2eActorSystemAbilitiesCon {
    fn from(value: &FoundryPf2eActorSystemAbilitiesCon) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesCon {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesCon {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesCon {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesDex`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesDex {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesDex>
for FoundryPf2eActorSystemAbilitiesDex {
    fn from(value: &FoundryPf2eActorSystemAbilitiesDex) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesDex {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesDex {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesDex {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesInt`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesInt {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesInt>
for FoundryPf2eActorSystemAbilitiesInt {
    fn from(value: &FoundryPf2eActorSystemAbilitiesInt) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesInt {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesInt {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesInt {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesStr`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesStr {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesStr>
for FoundryPf2eActorSystemAbilitiesStr {
    fn from(value: &FoundryPf2eActorSystemAbilitiesStr) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesStr {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesStr {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesStr {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAbilitiesWis`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "mod": {
///      "type": "number"
///    },
///    "value": {
///      "type": "number",
///      "maximum": 30.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAbilitiesWis {
    #[serde(
        rename = "mod",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mod_: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAbilitiesWis>
for FoundryPf2eActorSystemAbilitiesWis {
    fn from(value: &FoundryPf2eActorSystemAbilitiesWis) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesWis {
    fn default() -> Self {
        Self {
            mod_: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAbilitiesWis {
    pub fn builder() -> builder::FoundryPf2eActorSystemAbilitiesWis {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributes`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "ac": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "classDC": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "hp": {
///      "type": "object",
///      "properties": {
///        "max": {
///          "type": "number",
///          "minimum": 1.0
///        },
///        "temp": {
///          "type": "number",
///          "minimum": 0.0
///        },
///        "value": {
///          "type": "number",
///          "minimum": 0.0
///        }
///      }
///    },
///    "perception": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "speed": {
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
pub struct FoundryPf2eActorSystemAttributes {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ac: ::std::option::Option<FoundryPf2eActorSystemAttributesAc>,
    #[serde(
        rename = "classDC",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub class_dc: ::std::option::Option<FoundryPf2eActorSystemAttributesClassDc>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hp: ::std::option::Option<FoundryPf2eActorSystemAttributesHp>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perception: ::std::option::Option<FoundryPf2eActorSystemAttributesPerception>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub speed: ::std::option::Option<FoundryPf2eActorSystemAttributesSpeed>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributes>
for FoundryPf2eActorSystemAttributes {
    fn from(value: &FoundryPf2eActorSystemAttributes) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributes {
    fn default() -> Self {
        Self {
            ac: Default::default(),
            class_dc: Default::default(),
            hp: Default::default(),
            perception: Default::default(),
            speed: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAttributes {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributes {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributesAc`
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
pub struct FoundryPf2eActorSystemAttributesAc {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributesAc>
for FoundryPf2eActorSystemAttributesAc {
    fn from(value: &FoundryPf2eActorSystemAttributesAc) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributesAc {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemAttributesAc {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributesAc {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributesClassDc`
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
pub struct FoundryPf2eActorSystemAttributesClassDc {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributesClassDc>
for FoundryPf2eActorSystemAttributesClassDc {
    fn from(value: &FoundryPf2eActorSystemAttributesClassDc) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributesClassDc {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemAttributesClassDc {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributesClassDc {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributesHp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number",
///      "minimum": 1.0
///    },
///    "temp": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "value": {
///      "type": "number",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemAttributesHp {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub temp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributesHp>
for FoundryPf2eActorSystemAttributesHp {
    fn from(value: &FoundryPf2eActorSystemAttributesHp) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributesHp {
    fn default() -> Self {
        Self {
            max: Default::default(),
            temp: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemAttributesHp {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributesHp {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributesPerception`
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
pub struct FoundryPf2eActorSystemAttributesPerception {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributesPerception>
for FoundryPf2eActorSystemAttributesPerception {
    fn from(value: &FoundryPf2eActorSystemAttributesPerception) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributesPerception {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemAttributesPerception {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributesPerception {
        Default::default()
    }
}
///`FoundryPf2eActorSystemAttributesSpeed`
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
pub struct FoundryPf2eActorSystemAttributesSpeed {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemAttributesSpeed>
for FoundryPf2eActorSystemAttributesSpeed {
    fn from(value: &FoundryPf2eActorSystemAttributesSpeed) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemAttributesSpeed {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemAttributesSpeed {
    pub fn builder() -> builder::FoundryPf2eActorSystemAttributesSpeed {
        Default::default()
    }
}
///`FoundryPf2eActorSystemCurrency`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "cp": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "gp": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "pp": {
///      "type": "number",
///      "minimum": 0.0
///    },
///    "sp": {
///      "type": "number",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemCurrency {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sp: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemCurrency>
for FoundryPf2eActorSystemCurrency {
    fn from(value: &FoundryPf2eActorSystemCurrency) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemCurrency {
    fn default() -> Self {
        Self {
            cp: Default::default(),
            gp: Default::default(),
            pp: Default::default(),
            sp: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemCurrency {
    pub fn builder() -> builder::FoundryPf2eActorSystemCurrency {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alignment": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "ancestry": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "background": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "biography": {
///      "type": "object",
///      "properties": {
///        "public": {
///          "title": "HTML String",
///          "description": "Rich text content with HTML markup",
///          "type": "string",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "value": {
///          "title": "HTML String",
///          "description": "Rich text content with HTML markup",
///          "type": "string",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        }
///      }
///    },
///    "class": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "heritage": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string"
///        }
///      }
///    },
///    "keyability": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string",
///          "enum": [
///            "str",
///            "dex",
///            "con",
///            "int",
///            "wis",
///            "cha"
///          ]
///        }
///      }
///    },
///    "level": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "number",
///          "maximum": 20.0,
///          "minimum": 1.0
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemDetails {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alignment: ::std::option::Option<FoundryPf2eActorSystemDetailsAlignment>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ancestry: ::std::option::Option<FoundryPf2eActorSystemDetailsAncestry>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<FoundryPf2eActorSystemDetailsBackground>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub biography: ::std::option::Option<FoundryPf2eActorSystemDetailsBiography>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub class: ::std::option::Option<FoundryPf2eActorSystemDetailsClass>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub heritage: ::std::option::Option<FoundryPf2eActorSystemDetailsHeritage>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keyability: ::std::option::Option<FoundryPf2eActorSystemDetailsKeyability>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub level: ::std::option::Option<FoundryPf2eActorSystemDetailsLevel>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetails>
for FoundryPf2eActorSystemDetails {
    fn from(value: &FoundryPf2eActorSystemDetails) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetails {
    fn default() -> Self {
        Self {
            alignment: Default::default(),
            ancestry: Default::default(),
            background: Default::default(),
            biography: Default::default(),
            class: Default::default(),
            heritage: Default::default(),
            keyability: Default::default(),
            level: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemDetails {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetails {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsAlignment`
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
pub struct FoundryPf2eActorSystemDetailsAlignment {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsAlignment>
for FoundryPf2eActorSystemDetailsAlignment {
    fn from(value: &FoundryPf2eActorSystemDetailsAlignment) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsAlignment {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsAlignment {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsAlignment {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsAncestry`
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
pub struct FoundryPf2eActorSystemDetailsAncestry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsAncestry>
for FoundryPf2eActorSystemDetailsAncestry {
    fn from(value: &FoundryPf2eActorSystemDetailsAncestry) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsAncestry {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsAncestry {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsAncestry {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsBackground`
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
pub struct FoundryPf2eActorSystemDetailsBackground {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsBackground>
for FoundryPf2eActorSystemDetailsBackground {
    fn from(value: &FoundryPf2eActorSystemDetailsBackground) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsBackground {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsBackground {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsBackground {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsBiography`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "public": {
///      "title": "HTML String",
///      "description": "Rich text content with HTML markup",
///      "type": "string",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
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
pub struct FoundryPf2eActorSystemDetailsBiography {
    ///Rich text content with HTML markup
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub public: ::std::option::Option<::std::string::String>,
    ///Rich text content with HTML markup
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsBiography>
for FoundryPf2eActorSystemDetailsBiography {
    fn from(value: &FoundryPf2eActorSystemDetailsBiography) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsBiography {
    fn default() -> Self {
        Self {
            public: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemDetailsBiography {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsBiography {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsClass`
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
pub struct FoundryPf2eActorSystemDetailsClass {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsClass>
for FoundryPf2eActorSystemDetailsClass {
    fn from(value: &FoundryPf2eActorSystemDetailsClass) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsClass {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsClass {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsClass {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsHeritage`
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
pub struct FoundryPf2eActorSystemDetailsHeritage {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsHeritage>
for FoundryPf2eActorSystemDetailsHeritage {
    fn from(value: &FoundryPf2eActorSystemDetailsHeritage) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsHeritage {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsHeritage {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsHeritage {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsKeyability`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string",
///      "enum": [
///        "str",
///        "dex",
///        "con",
///        "int",
///        "wis",
///        "cha"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemDetailsKeyability {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<FoundryPf2eActorSystemDetailsKeyabilityValue>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsKeyability>
for FoundryPf2eActorSystemDetailsKeyability {
    fn from(value: &FoundryPf2eActorSystemDetailsKeyability) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsKeyability {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsKeyability {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsKeyability {
        Default::default()
    }
}
///`FoundryPf2eActorSystemDetailsKeyabilityValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "str",
///    "dex",
///    "con",
///    "int",
///    "wis",
///    "cha"
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
pub enum FoundryPf2eActorSystemDetailsKeyabilityValue {
    #[serde(rename = "str")]
    Str,
    #[serde(rename = "dex")]
    Dex,
    #[serde(rename = "con")]
    Con,
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "wis")]
    Wis,
    #[serde(rename = "cha")]
    Cha,
}
impl ::std::convert::From<&Self> for FoundryPf2eActorSystemDetailsKeyabilityValue {
    fn from(value: &FoundryPf2eActorSystemDetailsKeyabilityValue) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eActorSystemDetailsKeyabilityValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Str => f.write_str("str"),
            Self::Dex => f.write_str("dex"),
            Self::Con => f.write_str("con"),
            Self::Int => f.write_str("int"),
            Self::Wis => f.write_str("wis"),
            Self::Cha => f.write_str("cha"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eActorSystemDetailsKeyabilityValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "str" => Ok(Self::Str),
            "dex" => Ok(Self::Dex),
            "con" => Ok(Self::Con),
            "int" => Ok(Self::Int),
            "wis" => Ok(Self::Wis),
            "cha" => Ok(Self::Cha),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorSystemDetailsKeyabilityValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryPf2eActorSystemDetailsKeyabilityValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryPf2eActorSystemDetailsKeyabilityValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryPf2eActorSystemDetailsLevel`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "number",
///      "maximum": 20.0,
///      "minimum": 1.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemDetailsLevel {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemDetailsLevel>
for FoundryPf2eActorSystemDetailsLevel {
    fn from(value: &FoundryPf2eActorSystemDetailsLevel) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemDetailsLevel {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemDetailsLevel {
    pub fn builder() -> builder::FoundryPf2eActorSystemDetailsLevel {
        Default::default()
    }
}
///`FoundryPf2eActorSystemSaves`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "fortitude": {
///      "type": "object",
///      "properties": {
///        "proficient": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "reflex": {
///      "type": "object",
///      "properties": {
///        "proficient": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number"
///        }
///      }
///    },
///    "will": {
///      "type": "object",
///      "properties": {
///        "proficient": {
///          "type": "number"
///        },
///        "value": {
///          "type": "number"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemSaves {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fortitude: ::std::option::Option<FoundryPf2eActorSystemSavesFortitude>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reflex: ::std::option::Option<FoundryPf2eActorSystemSavesReflex>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub will: ::std::option::Option<FoundryPf2eActorSystemSavesWill>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemSaves> for FoundryPf2eActorSystemSaves {
    fn from(value: &FoundryPf2eActorSystemSaves) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemSaves {
    fn default() -> Self {
        Self {
            fortitude: Default::default(),
            reflex: Default::default(),
            will: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemSaves {
    pub fn builder() -> builder::FoundryPf2eActorSystemSaves {
        Default::default()
    }
}
///`FoundryPf2eActorSystemSavesFortitude`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "proficient": {
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
pub struct FoundryPf2eActorSystemSavesFortitude {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proficient: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemSavesFortitude>
for FoundryPf2eActorSystemSavesFortitude {
    fn from(value: &FoundryPf2eActorSystemSavesFortitude) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemSavesFortitude {
    fn default() -> Self {
        Self {
            proficient: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemSavesFortitude {
    pub fn builder() -> builder::FoundryPf2eActorSystemSavesFortitude {
        Default::default()
    }
}
///`FoundryPf2eActorSystemSavesReflex`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "proficient": {
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
pub struct FoundryPf2eActorSystemSavesReflex {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proficient: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemSavesReflex>
for FoundryPf2eActorSystemSavesReflex {
    fn from(value: &FoundryPf2eActorSystemSavesReflex) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemSavesReflex {
    fn default() -> Self {
        Self {
            proficient: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemSavesReflex {
    pub fn builder() -> builder::FoundryPf2eActorSystemSavesReflex {
        Default::default()
    }
}
///`FoundryPf2eActorSystemSavesWill`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "proficient": {
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
pub struct FoundryPf2eActorSystemSavesWill {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub proficient: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemSavesWill>
for FoundryPf2eActorSystemSavesWill {
    fn from(value: &FoundryPf2eActorSystemSavesWill) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemSavesWill {
    fn default() -> Self {
        Self {
            proficient: Default::default(),
            value: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemSavesWill {
    pub fn builder() -> builder::FoundryPf2eActorSystemSavesWill {
        Default::default()
    }
}
///`FoundryPf2eActorSystemTraits`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "languages": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "senses": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "size": {
///      "type": "object",
///      "properties": {
///        "value": {
///          "type": "string",
///          "enum": [
///            "tiny",
///            "sm",
///            "med",
///            "lg",
///            "huge",
///            "grg"
///          ]
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemTraits {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub languages: ::std::option::Option<FoundryPf2eActorSystemTraitsLanguages>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub senses: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<FoundryPf2eActorSystemTraitsSize>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemTraits>
for FoundryPf2eActorSystemTraits {
    fn from(value: &FoundryPf2eActorSystemTraits) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemTraits {
    fn default() -> Self {
        Self {
            languages: Default::default(),
            senses: Default::default(),
            size: Default::default(),
        }
    }
}
impl FoundryPf2eActorSystemTraits {
    pub fn builder() -> builder::FoundryPf2eActorSystemTraits {
        Default::default()
    }
}
///`FoundryPf2eActorSystemTraitsLanguages`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
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
pub struct FoundryPf2eActorSystemTraitsLanguages {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub value: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemTraitsLanguages>
for FoundryPf2eActorSystemTraitsLanguages {
    fn from(value: &FoundryPf2eActorSystemTraitsLanguages) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemTraitsLanguages {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemTraitsLanguages {
    pub fn builder() -> builder::FoundryPf2eActorSystemTraitsLanguages {
        Default::default()
    }
}
///`FoundryPf2eActorSystemTraitsSize`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "value": {
///      "type": "string",
///      "enum": [
///        "tiny",
///        "sm",
///        "med",
///        "lg",
///        "huge",
///        "grg"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryPf2eActorSystemTraitsSize {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub value: ::std::option::Option<FoundryPf2eActorSystemTraitsSizeValue>,
}
impl ::std::convert::From<&FoundryPf2eActorSystemTraitsSize>
for FoundryPf2eActorSystemTraitsSize {
    fn from(value: &FoundryPf2eActorSystemTraitsSize) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryPf2eActorSystemTraitsSize {
    fn default() -> Self {
        Self { value: Default::default() }
    }
}
impl FoundryPf2eActorSystemTraitsSize {
    pub fn builder() -> builder::FoundryPf2eActorSystemTraitsSize {
        Default::default()
    }
}
///`FoundryPf2eActorSystemTraitsSizeValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "tiny",
///    "sm",
///    "med",
///    "lg",
///    "huge",
///    "grg"
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
pub enum FoundryPf2eActorSystemTraitsSizeValue {
    #[serde(rename = "tiny")]
    Tiny,
    #[serde(rename = "sm")]
    Sm,
    #[serde(rename = "med")]
    Med,
    #[serde(rename = "lg")]
    Lg,
    #[serde(rename = "huge")]
    Huge,
    #[serde(rename = "grg")]
    Grg,
}
impl ::std::convert::From<&Self> for FoundryPf2eActorSystemTraitsSizeValue {
    fn from(value: &FoundryPf2eActorSystemTraitsSizeValue) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eActorSystemTraitsSizeValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tiny => f.write_str("tiny"),
            Self::Sm => f.write_str("sm"),
            Self::Med => f.write_str("med"),
            Self::Lg => f.write_str("lg"),
            Self::Huge => f.write_str("huge"),
            Self::Grg => f.write_str("grg"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eActorSystemTraitsSizeValue {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tiny" => Ok(Self::Tiny),
            "sm" => Ok(Self::Sm),
            "med" => Ok(Self::Med),
            "lg" => Ok(Self::Lg),
            "huge" => Ok(Self::Huge),
            "grg" => Ok(Self::Grg),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorSystemTraitsSizeValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryPf2eActorSystemTraitsSizeValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryPf2eActorSystemTraitsSizeValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///PF2e actor type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "PF2e actor type",
///  "type": "string",
///  "enum": [
///    "character",
///    "npc",
///    "hazard",
///    "loot",
///    "familiar",
///    "vehicle"
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
pub enum FoundryPf2eActorType {
    #[serde(rename = "character")]
    Character,
    #[serde(rename = "npc")]
    Npc,
    #[serde(rename = "hazard")]
    Hazard,
    #[serde(rename = "loot")]
    Loot,
    #[serde(rename = "familiar")]
    Familiar,
    #[serde(rename = "vehicle")]
    Vehicle,
}
impl ::std::convert::From<&Self> for FoundryPf2eActorType {
    fn from(value: &FoundryPf2eActorType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryPf2eActorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Character => f.write_str("character"),
            Self::Npc => f.write_str("npc"),
            Self::Hazard => f.write_str("hazard"),
            Self::Loot => f.write_str("loot"),
            Self::Familiar => f.write_str("familiar"),
            Self::Vehicle => f.write_str("vehicle"),
        }
    }
}
impl ::std::str::FromStr for FoundryPf2eActorType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "character" => Ok(Self::Character),
            "npc" => Ok(Self::Npc),
            "hazard" => Ok(Self::Hazard),
            "loot" => Ok(Self::Loot),
            "familiar" => Ok(Self::Familiar),
            "vehicle" => Ok(Self::Vehicle),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryPf2eActorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryPf2eActorType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryPf2eActorType {
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
    pub struct FoundryPf2eActor {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorFolder>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryPf2eActorId, ::std::string::String>,
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        items: ::std::result::Result<
            ::std::vec::Vec<super::FoundryPf2eActorItemsItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::FoundryPf2eActorName, ::std::string::String>,
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
            super::FoundryPf2eActorSystem,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::FoundryPf2eActorType, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActor {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                folder: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                img: Ok(Default::default()),
                items: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
                system: Err("no value supplied for system".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryPf2eActor {
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
                ::std::option::Option<super::FoundryPf2eActorFolder>,
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
            T: ::std::convert::TryInto<super::FoundryPf2eActorId>,
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
        pub fn items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryPf2eActorItemsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.items = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for items: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryPf2eActorName>,
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
            T: ::std::convert::TryInto<super::FoundryPf2eActorSystem>,
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
            T: ::std::convert::TryInto<super::FoundryPf2eActorType>,
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
    impl ::std::convert::TryFrom<FoundryPf2eActor> for super::FoundryPf2eActor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                folder: value.folder?,
                id: value.id?,
                img: value.img?,
                items: value.items?,
                name: value.name?,
                ownership: value.ownership?,
                sort: value.sort?,
                stats: value.stats?,
                system: value.system?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActor> for FoundryPf2eActor {
        fn from(value: super::FoundryPf2eActor) -> Self {
            Self {
                flags: Ok(value.flags),
                folder: Ok(value.folder),
                id: Ok(value.id),
                img: Ok(value.img),
                items: Ok(value.items),
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
    pub struct FoundryPf2eActorSystem {
        abilities: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilities>,
            ::std::string::String,
        >,
        attributes: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributes>,
            ::std::string::String,
        >,
        currency: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemCurrency>,
            ::std::string::String,
        >,
        details: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetails>,
            ::std::string::String,
        >,
        saves: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemSaves>,
            ::std::string::String,
        >,
        traits: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemTraits>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystem {
        fn default() -> Self {
            Self {
                abilities: Ok(Default::default()),
                attributes: Ok(Default::default()),
                currency: Ok(Default::default()),
                details: Ok(Default::default()),
                saves: Ok(Default::default()),
                traits: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystem {
        pub fn abilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilities>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.abilities = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for abilities: {}", e)
                });
            self
        }
        pub fn attributes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributes>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.attributes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attributes: {}", e)
                });
            self
        }
        pub fn currency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemCurrency>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.currency = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for currency: {}", e)
                });
            self
        }
        pub fn details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetails>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.details = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for details: {}", e)
                });
            self
        }
        pub fn saves<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemSaves>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.saves = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for saves: {}", e)
                });
            self
        }
        pub fn traits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemTraits>,
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystem>
    for super::FoundryPf2eActorSystem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abilities: value.abilities?,
                attributes: value.attributes?,
                currency: value.currency?,
                details: value.details?,
                saves: value.saves?,
                traits: value.traits?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystem> for FoundryPf2eActorSystem {
        fn from(value: super::FoundryPf2eActorSystem) -> Self {
            Self {
                abilities: Ok(value.abilities),
                attributes: Ok(value.attributes),
                currency: Ok(value.currency),
                details: Ok(value.details),
                saves: Ok(value.saves),
                traits: Ok(value.traits),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilities {
        cha: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesCha>,
            ::std::string::String,
        >,
        con: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesCon>,
            ::std::string::String,
        >,
        dex: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesDex>,
            ::std::string::String,
        >,
        int: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesInt>,
            ::std::string::String,
        >,
        str: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesStr>,
            ::std::string::String,
        >,
        wis: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesWis>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilities {
        fn default() -> Self {
            Self {
                cha: Ok(Default::default()),
                con: Ok(Default::default()),
                dex: Ok(Default::default()),
                int: Ok(Default::default()),
                str: Ok(Default::default()),
                wis: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilities {
        pub fn cha<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesCha>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cha = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cha: {}", e));
            self
        }
        pub fn con<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesCon>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.con = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for con: {}", e));
            self
        }
        pub fn dex<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesDex>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.dex = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dex: {}", e));
            self
        }
        pub fn int<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesInt>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.int = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for int: {}", e));
            self
        }
        pub fn str<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesStr>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.str = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for str: {}", e));
            self
        }
        pub fn wis<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAbilitiesWis>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.wis = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for wis: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilities>
    for super::FoundryPf2eActorSystemAbilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cha: value.cha?,
                con: value.con?,
                dex: value.dex?,
                int: value.int?,
                str: value.str?,
                wis: value.wis?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilities>
    for FoundryPf2eActorSystemAbilities {
        fn from(value: super::FoundryPf2eActorSystemAbilities) -> Self {
            Self {
                cha: Ok(value.cha),
                con: Ok(value.con),
                dex: Ok(value.dex),
                int: Ok(value.int),
                str: Ok(value.str),
                wis: Ok(value.wis),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesCha {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesCha {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesCha {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesCha>
    for super::FoundryPf2eActorSystemAbilitiesCha {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesCha,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesCha>
    for FoundryPf2eActorSystemAbilitiesCha {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesCha) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesCon {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesCon {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesCon {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesCon>
    for super::FoundryPf2eActorSystemAbilitiesCon {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesCon,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesCon>
    for FoundryPf2eActorSystemAbilitiesCon {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesCon) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesDex {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesDex {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesDex {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesDex>
    for super::FoundryPf2eActorSystemAbilitiesDex {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesDex,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesDex>
    for FoundryPf2eActorSystemAbilitiesDex {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesDex) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesInt {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesInt {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesInt {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesInt>
    for super::FoundryPf2eActorSystemAbilitiesInt {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesInt,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesInt>
    for FoundryPf2eActorSystemAbilitiesInt {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesInt) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesStr {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesStr {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesStr {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesStr>
    for super::FoundryPf2eActorSystemAbilitiesStr {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesStr,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesStr>
    for FoundryPf2eActorSystemAbilitiesStr {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesStr) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAbilitiesWis {
        mod_: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAbilitiesWis {
        fn default() -> Self {
            Self {
                mod_: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAbilitiesWis {
        pub fn mod_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.mod_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mod_: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAbilitiesWis>
    for super::FoundryPf2eActorSystemAbilitiesWis {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAbilitiesWis,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mod_: value.mod_?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAbilitiesWis>
    for FoundryPf2eActorSystemAbilitiesWis {
        fn from(value: super::FoundryPf2eActorSystemAbilitiesWis) -> Self {
            Self {
                mod_: Ok(value.mod_),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributes {
        ac: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributesAc>,
            ::std::string::String,
        >,
        class_dc: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributesClassDc>,
            ::std::string::String,
        >,
        hp: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributesHp>,
            ::std::string::String,
        >,
        perception: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributesPerception>,
            ::std::string::String,
        >,
        speed: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemAttributesSpeed>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributes {
        fn default() -> Self {
            Self {
                ac: Ok(Default::default()),
                class_dc: Ok(Default::default()),
                hp: Ok(Default::default()),
                perception: Ok(Default::default()),
                speed: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributes {
        pub fn ac<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributesAc>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ac = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ac: {}", e));
            self
        }
        pub fn class_dc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributesClassDc>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.class_dc = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for class_dc: {}", e)
                });
            self
        }
        pub fn hp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributesHp>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.hp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hp: {}", e));
            self
        }
        pub fn perception<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributesPerception>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.perception = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for perception: {}", e)
                });
            self
        }
        pub fn speed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemAttributesSpeed>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.speed = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributes>
    for super::FoundryPf2eActorSystemAttributes {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributes,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ac: value.ac?,
                class_dc: value.class_dc?,
                hp: value.hp?,
                perception: value.perception?,
                speed: value.speed?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributes>
    for FoundryPf2eActorSystemAttributes {
        fn from(value: super::FoundryPf2eActorSystemAttributes) -> Self {
            Self {
                ac: Ok(value.ac),
                class_dc: Ok(value.class_dc),
                hp: Ok(value.hp),
                perception: Ok(value.perception),
                speed: Ok(value.speed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributesAc {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributesAc {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributesAc {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributesAc>
    for super::FoundryPf2eActorSystemAttributesAc {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributesAc,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributesAc>
    for FoundryPf2eActorSystemAttributesAc {
        fn from(value: super::FoundryPf2eActorSystemAttributesAc) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributesClassDc {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributesClassDc {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributesClassDc {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributesClassDc>
    for super::FoundryPf2eActorSystemAttributesClassDc {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributesClassDc,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributesClassDc>
    for FoundryPf2eActorSystemAttributesClassDc {
        fn from(value: super::FoundryPf2eActorSystemAttributesClassDc) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributesHp {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        temp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributesHp {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                temp: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributesHp {
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
        pub fn temp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.temp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for temp: {}", e));
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributesHp>
    for super::FoundryPf2eActorSystemAttributesHp {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributesHp,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                temp: value.temp?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributesHp>
    for FoundryPf2eActorSystemAttributesHp {
        fn from(value: super::FoundryPf2eActorSystemAttributesHp) -> Self {
            Self {
                max: Ok(value.max),
                temp: Ok(value.temp),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributesPerception {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributesPerception {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributesPerception {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributesPerception>
    for super::FoundryPf2eActorSystemAttributesPerception {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributesPerception,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributesPerception>
    for FoundryPf2eActorSystemAttributesPerception {
        fn from(value: super::FoundryPf2eActorSystemAttributesPerception) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemAttributesSpeed {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemAttributesSpeed {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemAttributesSpeed {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemAttributesSpeed>
    for super::FoundryPf2eActorSystemAttributesSpeed {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemAttributesSpeed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemAttributesSpeed>
    for FoundryPf2eActorSystemAttributesSpeed {
        fn from(value: super::FoundryPf2eActorSystemAttributesSpeed) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemCurrency {
        cp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        gp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        pp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemCurrency {
        fn default() -> Self {
            Self {
                cp: Ok(Default::default()),
                gp: Ok(Default::default()),
                pp: Ok(Default::default()),
                sp: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemCurrency {
        pub fn cp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cp: {}", e));
            self
        }
        pub fn gp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gp: {}", e));
            self
        }
        pub fn pp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pp: {}", e));
            self
        }
        pub fn sp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemCurrency>
    for super::FoundryPf2eActorSystemCurrency {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemCurrency,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cp: value.cp?,
                gp: value.gp?,
                pp: value.pp?,
                sp: value.sp?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemCurrency>
    for FoundryPf2eActorSystemCurrency {
        fn from(value: super::FoundryPf2eActorSystemCurrency) -> Self {
            Self {
                cp: Ok(value.cp),
                gp: Ok(value.gp),
                pp: Ok(value.pp),
                sp: Ok(value.sp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetails {
        alignment: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsAlignment>,
            ::std::string::String,
        >,
        ancestry: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsAncestry>,
            ::std::string::String,
        >,
        background: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsBackground>,
            ::std::string::String,
        >,
        biography: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsBiography>,
            ::std::string::String,
        >,
        class: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsClass>,
            ::std::string::String,
        >,
        heritage: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsHeritage>,
            ::std::string::String,
        >,
        keyability: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsKeyability>,
            ::std::string::String,
        >,
        level: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsLevel>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetails {
        fn default() -> Self {
            Self {
                alignment: Ok(Default::default()),
                ancestry: Ok(Default::default()),
                background: Ok(Default::default()),
                biography: Ok(Default::default()),
                class: Ok(Default::default()),
                heritage: Ok(Default::default()),
                keyability: Ok(Default::default()),
                level: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetails {
        pub fn alignment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsAlignment>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.alignment = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alignment: {}", e)
                });
            self
        }
        pub fn ancestry<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsAncestry>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.ancestry = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ancestry: {}", e)
                });
            self
        }
        pub fn background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsBackground>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.background = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for background: {}", e)
                });
            self
        }
        pub fn biography<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsBiography>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.biography = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for biography: {}", e)
                });
            self
        }
        pub fn class<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsClass>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.class = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for class: {}", e)
                });
            self
        }
        pub fn heritage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsHeritage>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.heritage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for heritage: {}", e)
                });
            self
        }
        pub fn keyability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsKeyability>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.keyability = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for keyability: {}", e)
                });
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemDetailsLevel>,
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
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetails>
    for super::FoundryPf2eActorSystemDetails {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alignment: value.alignment?,
                ancestry: value.ancestry?,
                background: value.background?,
                biography: value.biography?,
                class: value.class?,
                heritage: value.heritage?,
                keyability: value.keyability?,
                level: value.level?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetails>
    for FoundryPf2eActorSystemDetails {
        fn from(value: super::FoundryPf2eActorSystemDetails) -> Self {
            Self {
                alignment: Ok(value.alignment),
                ancestry: Ok(value.ancestry),
                background: Ok(value.background),
                biography: Ok(value.biography),
                class: Ok(value.class),
                heritage: Ok(value.heritage),
                keyability: Ok(value.keyability),
                level: Ok(value.level),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsAlignment {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsAlignment {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsAlignment {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsAlignment>
    for super::FoundryPf2eActorSystemDetailsAlignment {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsAlignment,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsAlignment>
    for FoundryPf2eActorSystemDetailsAlignment {
        fn from(value: super::FoundryPf2eActorSystemDetailsAlignment) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsAncestry {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsAncestry {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsAncestry {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsAncestry>
    for super::FoundryPf2eActorSystemDetailsAncestry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsAncestry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsAncestry>
    for FoundryPf2eActorSystemDetailsAncestry {
        fn from(value: super::FoundryPf2eActorSystemDetailsAncestry) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsBackground {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsBackground {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsBackground {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsBackground>
    for super::FoundryPf2eActorSystemDetailsBackground {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsBackground,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsBackground>
    for FoundryPf2eActorSystemDetailsBackground {
        fn from(value: super::FoundryPf2eActorSystemDetailsBackground) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsBiography {
        public: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsBiography {
        fn default() -> Self {
            Self {
                public: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsBiography {
        pub fn public<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.public = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for public: {}", e)
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsBiography>
    for super::FoundryPf2eActorSystemDetailsBiography {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsBiography,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                public: value.public?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsBiography>
    for FoundryPf2eActorSystemDetailsBiography {
        fn from(value: super::FoundryPf2eActorSystemDetailsBiography) -> Self {
            Self {
                public: Ok(value.public),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsClass {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsClass {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsClass {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsClass>
    for super::FoundryPf2eActorSystemDetailsClass {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsClass,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsClass>
    for FoundryPf2eActorSystemDetailsClass {
        fn from(value: super::FoundryPf2eActorSystemDetailsClass) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsHeritage {
        value: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsHeritage {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsHeritage {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsHeritage>
    for super::FoundryPf2eActorSystemDetailsHeritage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsHeritage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsHeritage>
    for FoundryPf2eActorSystemDetailsHeritage {
        fn from(value: super::FoundryPf2eActorSystemDetailsHeritage) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsKeyability {
        value: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemDetailsKeyabilityValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsKeyability {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsKeyability {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FoundryPf2eActorSystemDetailsKeyabilityValue,
                >,
            >,
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsKeyability>
    for super::FoundryPf2eActorSystemDetailsKeyability {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsKeyability,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsKeyability>
    for FoundryPf2eActorSystemDetailsKeyability {
        fn from(value: super::FoundryPf2eActorSystemDetailsKeyability) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemDetailsLevel {
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemDetailsLevel {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemDetailsLevel {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemDetailsLevel>
    for super::FoundryPf2eActorSystemDetailsLevel {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemDetailsLevel,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemDetailsLevel>
    for FoundryPf2eActorSystemDetailsLevel {
        fn from(value: super::FoundryPf2eActorSystemDetailsLevel) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemSaves {
        fortitude: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemSavesFortitude>,
            ::std::string::String,
        >,
        reflex: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemSavesReflex>,
            ::std::string::String,
        >,
        will: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemSavesWill>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemSaves {
        fn default() -> Self {
            Self {
                fortitude: Ok(Default::default()),
                reflex: Ok(Default::default()),
                will: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemSaves {
        pub fn fortitude<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemSavesFortitude>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.fortitude = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fortitude: {}", e)
                });
            self
        }
        pub fn reflex<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemSavesReflex>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reflex = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reflex: {}", e)
                });
            self
        }
        pub fn will<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemSavesWill>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.will = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for will: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemSaves>
    for super::FoundryPf2eActorSystemSaves {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemSaves,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fortitude: value.fortitude?,
                reflex: value.reflex?,
                will: value.will?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemSaves>
    for FoundryPf2eActorSystemSaves {
        fn from(value: super::FoundryPf2eActorSystemSaves) -> Self {
            Self {
                fortitude: Ok(value.fortitude),
                reflex: Ok(value.reflex),
                will: Ok(value.will),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemSavesFortitude {
        proficient: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemSavesFortitude {
        fn default() -> Self {
            Self {
                proficient: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemSavesFortitude {
        pub fn proficient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.proficient = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proficient: {}", e)
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemSavesFortitude>
    for super::FoundryPf2eActorSystemSavesFortitude {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemSavesFortitude,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                proficient: value.proficient?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemSavesFortitude>
    for FoundryPf2eActorSystemSavesFortitude {
        fn from(value: super::FoundryPf2eActorSystemSavesFortitude) -> Self {
            Self {
                proficient: Ok(value.proficient),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemSavesReflex {
        proficient: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemSavesReflex {
        fn default() -> Self {
            Self {
                proficient: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemSavesReflex {
        pub fn proficient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.proficient = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proficient: {}", e)
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemSavesReflex>
    for super::FoundryPf2eActorSystemSavesReflex {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemSavesReflex,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                proficient: value.proficient?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemSavesReflex>
    for FoundryPf2eActorSystemSavesReflex {
        fn from(value: super::FoundryPf2eActorSystemSavesReflex) -> Self {
            Self {
                proficient: Ok(value.proficient),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemSavesWill {
        proficient: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemSavesWill {
        fn default() -> Self {
            Self {
                proficient: Ok(Default::default()),
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemSavesWill {
        pub fn proficient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.proficient = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for proficient: {}", e)
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemSavesWill>
    for super::FoundryPf2eActorSystemSavesWill {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemSavesWill,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                proficient: value.proficient?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemSavesWill>
    for FoundryPf2eActorSystemSavesWill {
        fn from(value: super::FoundryPf2eActorSystemSavesWill) -> Self {
            Self {
                proficient: Ok(value.proficient),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemTraits {
        languages: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemTraitsLanguages>,
            ::std::string::String,
        >,
        senses: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        size: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemTraitsSize>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemTraits {
        fn default() -> Self {
            Self {
                languages: Ok(Default::default()),
                senses: Ok(Default::default()),
                size: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemTraits {
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemTraitsLanguages>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.languages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for languages: {}", e)
                });
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemTraitsSize>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemTraits>
    for super::FoundryPf2eActorSystemTraits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemTraits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                languages: value.languages?,
                senses: value.senses?,
                size: value.size?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemTraits>
    for FoundryPf2eActorSystemTraits {
        fn from(value: super::FoundryPf2eActorSystemTraits) -> Self {
            Self {
                languages: Ok(value.languages),
                senses: Ok(value.senses),
                size: Ok(value.size),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemTraitsLanguages {
        value: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemTraitsLanguages {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemTraitsLanguages {
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemTraitsLanguages>
    for super::FoundryPf2eActorSystemTraitsLanguages {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemTraitsLanguages,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemTraitsLanguages>
    for FoundryPf2eActorSystemTraitsLanguages {
        fn from(value: super::FoundryPf2eActorSystemTraitsLanguages) -> Self {
            Self { value: Ok(value.value) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryPf2eActorSystemTraitsSize {
        value: ::std::result::Result<
            ::std::option::Option<super::FoundryPf2eActorSystemTraitsSizeValue>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryPf2eActorSystemTraitsSize {
        fn default() -> Self {
            Self {
                value: Ok(Default::default()),
            }
        }
    }
    impl FoundryPf2eActorSystemTraitsSize {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryPf2eActorSystemTraitsSizeValue>,
            >,
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
    impl ::std::convert::TryFrom<FoundryPf2eActorSystemTraitsSize>
    for super::FoundryPf2eActorSystemTraitsSize {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryPf2eActorSystemTraitsSize,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { value: value.value? })
        }
    }
    impl ::std::convert::From<super::FoundryPf2eActorSystemTraitsSize>
    for FoundryPf2eActorSystemTraitsSize {
        fn from(value: super::FoundryPf2eActorSystemTraitsSize) -> Self {
            Self { value: Ok(value.value) }
        }
    }
}
