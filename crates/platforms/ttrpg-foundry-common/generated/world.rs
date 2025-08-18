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
///Author information for Foundry VTT content
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Author",
///  "description": "Author information for Foundry VTT content",
///  "type": "object",
///  "properties": {
///    "email": {
///      "description": "Author email",
///      "type": "string",
///      "format": "email"
///    },
///    "name": {
///      "description": "Author name",
///      "type": "string"
///    },
///    "url": {
///      "description": "Author website",
///      "type": "string",
///      "format": "uri"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryAuthor {
    ///Author email
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    ///Author name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Author website
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryAuthor> for FoundryAuthor {
    fn from(value: &FoundryAuthor) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryAuthor {
    fn default() -> Self {
        Self {
            email: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl FoundryAuthor {
    pub fn builder() -> builder::FoundryAuthor {
        Default::default()
    }
}
///Compendium pack configuration
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Compendium Pack",
///  "description": "Compendium pack configuration",
///  "type": "object",
///  "required": [
///    "label",
///    "name",
///    "type"
///  ],
///  "properties": {
///    "flags": {
///      "description": "Pack-specific flags",
///      "type": "object"
///    },
///    "label": {
///      "description": "Pack display name",
///      "type": "string"
///    },
///    "name": {
///      "description": "Pack identifier",
///      "type": "string"
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
///    "path": {
///      "description": "Relative path to pack database file",
///      "type": "string"
///    },
///    "system": {
///      "description": "Game system identifier (for system packs)",
///      "type": "string"
///    },
///    "type": {
///      "description": "Document type stored in this pack",
///      "type": "string",
///      "enum": [
///        "Actor",
///        "Item",
///        "JournalEntry",
///        "Macro",
///        "Playlist",
///        "RollTable",
///        "Scene"
///      ]
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryCompendiumPack {
    ///Pack-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Pack display name
    pub label: ::std::string::String,
    ///Pack identifier
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    ///Relative path to pack database file
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    ///Game system identifier (for system packs)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system: ::std::option::Option<::std::string::String>,
    ///Document type stored in this pack
    #[serde(rename = "type")]
    pub type_: FoundryCompendiumPackType,
}
impl ::std::convert::From<&FoundryCompendiumPack> for FoundryCompendiumPack {
    fn from(value: &FoundryCompendiumPack) -> Self {
        value.clone()
    }
}
impl FoundryCompendiumPack {
    pub fn builder() -> builder::FoundryCompendiumPack {
        Default::default()
    }
}
///Document type stored in this pack
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Document type stored in this pack",
///  "type": "string",
///  "enum": [
///    "Actor",
///    "Item",
///    "JournalEntry",
///    "Macro",
///    "Playlist",
///    "RollTable",
///    "Scene"
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
pub enum FoundryCompendiumPackType {
    Actor,
    Item,
    JournalEntry,
    Macro,
    Playlist,
    RollTable,
    Scene,
}
impl ::std::convert::From<&Self> for FoundryCompendiumPackType {
    fn from(value: &FoundryCompendiumPackType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryCompendiumPackType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Actor => f.write_str("Actor"),
            Self::Item => f.write_str("Item"),
            Self::JournalEntry => f.write_str("JournalEntry"),
            Self::Macro => f.write_str("Macro"),
            Self::Playlist => f.write_str("Playlist"),
            Self::RollTable => f.write_str("RollTable"),
            Self::Scene => f.write_str("Scene"),
        }
    }
}
impl ::std::str::FromStr for FoundryCompendiumPackType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Actor" => Ok(Self::Actor),
            "Item" => Ok(Self::Item),
            "JournalEntry" => Ok(Self::JournalEntry),
            "Macro" => Ok(Self::Macro),
            "Playlist" => Ok(Self::Playlist),
            "RollTable" => Ok(Self::RollTable),
            "Scene" => Ok(Self::Scene),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryCompendiumPackType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryCompendiumPackType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryCompendiumPackType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
///Schema for Foundry VTT world.json configuration file
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry VTT World",
///  "description": "Schema for Foundry VTT world.json configuration file",
///  "type": "object",
///  "required": [
///    "coreVersion",
///    "id",
///    "system",
///    "title"
///  ],
///  "properties": {
///    "authors": {
///      "description": "World authors",
///      "type": "array",
///      "items": {
///        "title": "Foundry Author",
///        "description": "Author information for Foundry VTT content",
///        "type": "object",
///        "properties": {
///          "email": {
///            "description": "Author email",
///            "type": "string",
///            "format": "email"
///          },
///          "name": {
///            "description": "Author name",
///            "type": "string"
///          },
///          "url": {
///            "description": "Author website",
///            "type": "string",
///            "format": "uri"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "compatibility": {
///      "description": "Foundry version compatibility",
///      "type": "object",
///      "properties": {
///        "maximum": {
///          "description": "Maximum supported Foundry version",
///          "type": "string",
///          "pattern": "^\\d+(\\.\\d+)?$"
///        },
///        "minimum": {
///          "description": "Minimum supported Foundry version",
///          "type": "string",
///          "pattern": "^\\d+(\\.\\d+)?$"
///        },
///        "verified": {
///          "description": "Last verified Foundry version",
///          "type": "string",
///          "pattern": "^\\d+(\\.\\d+)?$"
///        }
///      }
///    },
///    "compatibleCoreVersion": {
///      "description": "Legacy compatibility field",
///      "type": "string"
///    },
///    "coreVersion": {
///      "description": "Foundry core version (e.g., 12.331)",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+(\\.\\d+)?$"
///    },
///    "description": {
///      "description": "World description",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique world identifier (lowercase, alphanumeric, hyphens, underscores)",
///      "type": "string",
///      "minLength": 1,
///      "pattern": "^[a-z0-9-_]+$"
///    },
///    "joinTheme": {
///      "description": "Join screen theme",
///      "default": "default",
///      "type": "string"
///    },
///    "lastPlayed": {
///      "description": "ISO date string of last play session",
///      "type": "string"
///    },
///    "minimumCoreVersion": {
///      "description": "Legacy compatibility field",
///      "type": "string"
///    },
///    "packs": {
///      "description": "Compendium packs in this world",
///      "type": "array",
///      "items": {
///        "title": "Foundry Compendium Pack",
///        "description": "Compendium pack configuration",
///        "type": "object",
///        "required": [
///          "label",
///          "name",
///          "type"
///        ],
///        "properties": {
///          "flags": {
///            "description": "Pack-specific flags",
///            "type": "object"
///          },
///          "label": {
///            "description": "Pack display name",
///            "type": "string"
///          },
///          "name": {
///            "description": "Pack identifier",
///            "type": "string"
///          },
///          "ownership": {
///            "title": "Foundry Document Ownership",
///            "description": "Document ownership permissions by user role",
///            "type": "object",
///            "properties": {
///              "default": {
///                "description": "Default permission level (0=None, 1=Limited, 2=Observer, 3=Owner)",
///                "type": "integer",
///                "enum": [
///                  0,
///                  1,
///                  2,
///                  3
///                ]
///              }
///            },
///            "patternProperties": {
///              "^(PLAYER|TRUSTED|ASSISTANT|GAMEMASTER)$": {
///                "description": "Role-based permission level",
///                "type": "integer",
///                "enum": [
///                  0,
///                  1,
///                  2,
///                  3
///                ]
///              },
///              "^[a-zA-Z0-9]{16}$": {
///                "description": "User-specific permission level",
///                "type": "integer",
///                "enum": [
///                  0,
///                  1,
///                  2,
///                  3
///                ]
///              }
///            },
///            "additionalProperties": false,
///            "$schema": "https://json-schema.org/draft-07/schema#"
///          },
///          "path": {
///            "description": "Relative path to pack database file",
///            "type": "string"
///          },
///          "system": {
///            "description": "Game system identifier (for system packs)",
///            "type": "string"
///          },
///          "type": {
///            "description": "Document type stored in this pack",
///            "type": "string",
///            "enum": [
///              "Actor",
///              "Item",
///              "JournalEntry",
///              "Macro",
///              "Playlist",
///              "RollTable",
///              "Scene"
///            ]
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "playtime": {
///      "description": "Total playtime in seconds",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "system": {
///      "description": "Game system ID (e.g., dnd5e, pf2e, ose)",
///      "type": "string",
///      "minLength": 1
///    },
///    "systemVersion": {
///      "description": "Game system version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+(\\.\\d+)?$"
///    },
///    "title": {
///      "description": "Display name of the world",
///      "type": "string",
///      "minLength": 1
///    },
///    "version": {
///      "description": "World version",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttWorld {
    ///World authors
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub authors: ::std::vec::Vec<FoundryAuthor>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<FoundryVttWorldCompatibility>,
    ///Legacy compatibility field
    #[serde(
        rename = "compatibleCoreVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub compatible_core_version: ::std::option::Option<::std::string::String>,
    ///Foundry core version (e.g., 12.331)
    #[serde(rename = "coreVersion")]
    pub core_version: FoundryVttWorldCoreVersion,
    ///World description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///Unique world identifier (lowercase, alphanumeric, hyphens, underscores)
    pub id: FoundryVttWorldId,
    ///Join screen theme
    #[serde(rename = "joinTheme", default = "defaults::foundry_vtt_world_join_theme")]
    pub join_theme: ::std::string::String,
    ///ISO date string of last play session
    #[serde(
        rename = "lastPlayed",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_played: ::std::option::Option<::std::string::String>,
    ///Legacy compatibility field
    #[serde(
        rename = "minimumCoreVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub minimum_core_version: ::std::option::Option<::std::string::String>,
    ///Compendium packs in this world
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub packs: ::std::vec::Vec<FoundryCompendiumPack>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playtime: ::std::option::Option<f64>,
    ///Game system ID (e.g., dnd5e, pf2e, ose)
    pub system: FoundryVttWorldSystem,
    ///Game system version
    #[serde(
        rename = "systemVersion",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub system_version: ::std::option::Option<FoundryVttWorldSystemVersion>,
    ///Display name of the world
    pub title: FoundryVttWorldTitle,
    ///World version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryVttWorld> for FoundryVttWorld {
    fn from(value: &FoundryVttWorld) -> Self {
        value.clone()
    }
}
impl FoundryVttWorld {
    pub fn builder() -> builder::FoundryVttWorld {
        Default::default()
    }
}
///Foundry version compatibility
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Foundry version compatibility",
///  "type": "object",
///  "properties": {
///    "maximum": {
///      "description": "Maximum supported Foundry version",
///      "type": "string",
///      "pattern": "^\\d+(\\.\\d+)?$"
///    },
///    "minimum": {
///      "description": "Minimum supported Foundry version",
///      "type": "string",
///      "pattern": "^\\d+(\\.\\d+)?$"
///    },
///    "verified": {
///      "description": "Last verified Foundry version",
///      "type": "string",
///      "pattern": "^\\d+(\\.\\d+)?$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttWorldCompatibility {
    ///Maximum supported Foundry version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<FoundryVttWorldCompatibilityMaximum>,
    ///Minimum supported Foundry version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<FoundryVttWorldCompatibilityMinimum>,
    ///Last verified Foundry version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub verified: ::std::option::Option<FoundryVttWorldCompatibilityVerified>,
}
impl ::std::convert::From<&FoundryVttWorldCompatibility>
for FoundryVttWorldCompatibility {
    fn from(value: &FoundryVttWorldCompatibility) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryVttWorldCompatibility {
    fn default() -> Self {
        Self {
            maximum: Default::default(),
            minimum: Default::default(),
            verified: Default::default(),
        }
    }
}
impl FoundryVttWorldCompatibility {
    pub fn builder() -> builder::FoundryVttWorldCompatibility {
        Default::default()
    }
}
///Maximum supported Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Maximum supported Foundry version",
///  "type": "string",
///  "pattern": "^\\d+(\\.\\d+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldCompatibilityMaximum(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldCompatibilityMaximum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldCompatibilityMaximum>
for ::std::string::String {
    fn from(value: FoundryVttWorldCompatibilityMaximum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldCompatibilityMaximum>
for FoundryVttWorldCompatibilityMaximum {
    fn from(value: &FoundryVttWorldCompatibilityMaximum) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldCompatibilityMaximum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+(\\.\\d+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttWorldCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttWorldCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldCompatibilityMaximum {
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
///Minimum supported Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Minimum supported Foundry version",
///  "type": "string",
///  "pattern": "^\\d+(\\.\\d+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldCompatibilityMinimum(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldCompatibilityMinimum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldCompatibilityMinimum>
for ::std::string::String {
    fn from(value: FoundryVttWorldCompatibilityMinimum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldCompatibilityMinimum>
for FoundryVttWorldCompatibilityMinimum {
    fn from(value: &FoundryVttWorldCompatibilityMinimum) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldCompatibilityMinimum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+(\\.\\d+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttWorldCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttWorldCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldCompatibilityMinimum {
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
///Last verified Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Last verified Foundry version",
///  "type": "string",
///  "pattern": "^\\d+(\\.\\d+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldCompatibilityVerified(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldCompatibilityVerified {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldCompatibilityVerified>
for ::std::string::String {
    fn from(value: FoundryVttWorldCompatibilityVerified) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldCompatibilityVerified>
for FoundryVttWorldCompatibilityVerified {
    fn from(value: &FoundryVttWorldCompatibilityVerified) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldCompatibilityVerified {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+(\\.\\d+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttWorldCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttWorldCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldCompatibilityVerified {
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
///Foundry core version (e.g., 12.331)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Foundry core version (e.g., 12.331)",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+(\\.\\d+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldCoreVersion(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldCoreVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldCoreVersion> for ::std::string::String {
    fn from(value: FoundryVttWorldCoreVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldCoreVersion> for FoundryVttWorldCoreVersion {
    fn from(value: &FoundryVttWorldCoreVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldCoreVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+(\\.\\d+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldCoreVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttWorldCoreVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttWorldCoreVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldCoreVersion {
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
///Unique world identifier (lowercase, alphanumeric, hyphens, underscores)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique world identifier (lowercase, alphanumeric, hyphens, underscores)",
///  "type": "string",
///  "minLength": 1,
///  "pattern": "^[a-z0-9-_]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldId(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldId> for ::std::string::String {
    fn from(value: FoundryVttWorldId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldId> for FoundryVttWorldId {
    fn from(value: &FoundryVttWorldId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9-_]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9-_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttWorldId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttWorldId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldId {
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
///Game system ID (e.g., dnd5e, pf2e, ose)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Game system ID (e.g., dnd5e, pf2e, ose)",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldSystem(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldSystem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldSystem> for ::std::string::String {
    fn from(value: FoundryVttWorldSystem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldSystem> for FoundryVttWorldSystem {
    fn from(value: &FoundryVttWorldSystem) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldSystem {
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
impl ::std::convert::TryFrom<&str> for FoundryVttWorldSystem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttWorldSystem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttWorldSystem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldSystem {
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
///Game system version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Game system version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+(\\.\\d+)?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldSystemVersion(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldSystemVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldSystemVersion> for ::std::string::String {
    fn from(value: FoundryVttWorldSystemVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldSystemVersion>
for FoundryVttWorldSystemVersion {
    fn from(value: &FoundryVttWorldSystemVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldSystemVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+(\\.\\d+)?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+(\\.\\d+)?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttWorldSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttWorldSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttWorldSystemVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldSystemVersion {
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
///Display name of the world
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Display name of the world",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttWorldTitle(::std::string::String);
impl ::std::ops::Deref for FoundryVttWorldTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttWorldTitle> for ::std::string::String {
    fn from(value: FoundryVttWorldTitle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttWorldTitle> for FoundryVttWorldTitle {
    fn from(value: &FoundryVttWorldTitle) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttWorldTitle {
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
impl ::std::convert::TryFrom<&str> for FoundryVttWorldTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttWorldTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttWorldTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttWorldTitle {
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
    pub struct FoundryAuthor {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryAuthor {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl FoundryAuthor {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for email: {}", e)
                });
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
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryAuthor> for super::FoundryAuthor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryAuthor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryAuthor> for FoundryAuthor {
        fn from(value: super::FoundryAuthor) -> Self {
            Self {
                email: Ok(value.email),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryCompendiumPack {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::FoundryCompendiumPackType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryCompendiumPack {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                label: Err("no value supplied for label".to_string()),
                name: Err("no value supplied for name".to_string()),
                ownership: Ok(Default::default()),
                path: Ok(Default::default()),
                system: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryCompendiumPack {
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
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for label: {}", e)
                });
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
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
            T: ::std::convert::TryInto<super::FoundryCompendiumPackType>,
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
    impl ::std::convert::TryFrom<FoundryCompendiumPack>
    for super::FoundryCompendiumPack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryCompendiumPack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                label: value.label?,
                name: value.name?,
                ownership: value.ownership?,
                path: value.path?,
                system: value.system?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryCompendiumPack> for FoundryCompendiumPack {
        fn from(value: super::FoundryCompendiumPack) -> Self {
            Self {
                flags: Ok(value.flags),
                label: Ok(value.label),
                name: Ok(value.name),
                ownership: Ok(value.ownership),
                path: Ok(value.path),
                system: Ok(value.system),
                type_: Ok(value.type_),
            }
        }
    }
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
    pub struct FoundryVttWorld {
        authors: ::std::result::Result<
            ::std::vec::Vec<super::FoundryAuthor>,
            ::std::string::String,
        >,
        compatibility: ::std::result::Result<
            ::std::option::Option<super::FoundryVttWorldCompatibility>,
            ::std::string::String,
        >,
        compatible_core_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        core_version: ::std::result::Result<
            super::FoundryVttWorldCoreVersion,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::FoundryVttWorldId, ::std::string::String>,
        join_theme: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_played: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        minimum_core_version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        packs: ::std::result::Result<
            ::std::vec::Vec<super::FoundryCompendiumPack>,
            ::std::string::String,
        >,
        playtime: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            super::FoundryVttWorldSystem,
            ::std::string::String,
        >,
        system_version: ::std::result::Result<
            ::std::option::Option<super::FoundryVttWorldSystemVersion>,
            ::std::string::String,
        >,
        title: ::std::result::Result<super::FoundryVttWorldTitle, ::std::string::String>,
        version: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttWorld {
        fn default() -> Self {
            Self {
                authors: Ok(Default::default()),
                compatibility: Ok(Default::default()),
                compatible_core_version: Ok(Default::default()),
                core_version: Err("no value supplied for core_version".to_string()),
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                join_theme: Ok(super::defaults::foundry_vtt_world_join_theme()),
                last_played: Ok(Default::default()),
                minimum_core_version: Ok(Default::default()),
                packs: Ok(Default::default()),
                playtime: Ok(Default::default()),
                system: Err("no value supplied for system".to_string()),
                system_version: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                version: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttWorld {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FoundryAuthor>>,
            T::Error: ::std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for authors: {}", e)
                });
            self
        }
        pub fn compatibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttWorldCompatibility>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.compatibility = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for compatibility: {}", e)
                });
            self
        }
        pub fn compatible_core_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.compatible_core_version = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for compatible_core_version: {}",
                        e
                    )
                });
            self
        }
        pub fn core_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttWorldCoreVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.core_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for core_version: {}", e)
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttWorldId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn join_theme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.join_theme = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for join_theme: {}", e)
                });
            self
        }
        pub fn last_played<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_played = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for last_played: {}", e)
                });
            self
        }
        pub fn minimum_core_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum_core_version = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for minimum_core_version: {}", e
                    )
                });
            self
        }
        pub fn packs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::FoundryCompendiumPack>>,
            T::Error: ::std::fmt::Display,
        {
            self.packs = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for packs: {}", e)
                });
            self
        }
        pub fn playtime<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.playtime = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playtime: {}", e)
                });
            self
        }
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttWorldSystem>,
            T::Error: ::std::fmt::Display,
        {
            self.system = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system: {}", e)
                });
            self
        }
        pub fn system_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttWorldSystemVersion>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.system_version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_version: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttWorldTitle>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttWorld> for super::FoundryVttWorld {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttWorld,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authors: value.authors?,
                compatibility: value.compatibility?,
                compatible_core_version: value.compatible_core_version?,
                core_version: value.core_version?,
                description: value.description?,
                id: value.id?,
                join_theme: value.join_theme?,
                last_played: value.last_played?,
                minimum_core_version: value.minimum_core_version?,
                packs: value.packs?,
                playtime: value.playtime?,
                system: value.system?,
                system_version: value.system_version?,
                title: value.title?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttWorld> for FoundryVttWorld {
        fn from(value: super::FoundryVttWorld) -> Self {
            Self {
                authors: Ok(value.authors),
                compatibility: Ok(value.compatibility),
                compatible_core_version: Ok(value.compatible_core_version),
                core_version: Ok(value.core_version),
                description: Ok(value.description),
                id: Ok(value.id),
                join_theme: Ok(value.join_theme),
                last_played: Ok(value.last_played),
                minimum_core_version: Ok(value.minimum_core_version),
                packs: Ok(value.packs),
                playtime: Ok(value.playtime),
                system: Ok(value.system),
                system_version: Ok(value.system_version),
                title: Ok(value.title),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttWorldCompatibility {
        maximum: ::std::result::Result<
            ::std::option::Option<super::FoundryVttWorldCompatibilityMaximum>,
            ::std::string::String,
        >,
        minimum: ::std::result::Result<
            ::std::option::Option<super::FoundryVttWorldCompatibilityMinimum>,
            ::std::string::String,
        >,
        verified: ::std::result::Result<
            ::std::option::Option<super::FoundryVttWorldCompatibilityVerified>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttWorldCompatibility {
        fn default() -> Self {
            Self {
                maximum: Ok(Default::default()),
                minimum: Ok(Default::default()),
                verified: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttWorldCompatibility {
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttWorldCompatibilityMaximum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for maximum: {}", e)
                });
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttWorldCompatibilityMinimum>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for minimum: {}", e)
                });
            self
        }
        pub fn verified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttWorldCompatibilityVerified>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.verified = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for verified: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttWorldCompatibility>
    for super::FoundryVttWorldCompatibility {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttWorldCompatibility,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                maximum: value.maximum?,
                minimum: value.minimum?,
                verified: value.verified?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttWorldCompatibility>
    for FoundryVttWorldCompatibility {
        fn from(value: super::FoundryVttWorldCompatibility) -> Self {
            Self {
                maximum: Ok(value.maximum),
                minimum: Ok(value.minimum),
                verified: Ok(value.verified),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn foundry_vtt_world_join_theme() -> ::std::string::String {
        "default".to_string()
    }
}
