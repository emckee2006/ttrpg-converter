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
///6-digit hexadecimal color code
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Color Hex",
///  "description": "6-digit hexadecimal color code",
///  "type": "string",
///  "pattern": "^#[0-9a-fA-F]{6}$",
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ColorHex(::std::string::String);
impl ::std::ops::Deref for ColorHex {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ColorHex> for ::std::string::String {
    fn from(value: ColorHex) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ColorHex> for ColorHex {
    fn from(value: &ColorHex) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ColorHex {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^#[0-9a-fA-F]{6}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^#[0-9a-fA-F]{6}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ColorHex {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ColorHex {
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
///Schema for Foundry VTT scene documents (system-agnostic)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Scene",
///  "description": "Schema for Foundry VTT scene documents (system-agnostic)",
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
///        "active": {
///          "description": "Whether this is the active scene",
///          "type": "boolean"
///        },
///        "backgroundColor": {
///          "title": "Color Hex",
///          "description": "6-digit hexadecimal color code",
///          "type": "string",
///          "pattern": "^#[0-9a-fA-F]{6}$",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "darkness": {
///          "description": "Scene darkness level",
///          "type": "number",
///          "maximum": 1.0,
///          "minimum": 0.0
///        },
///        "fogExploration": {
///          "description": "Whether fog exploration is enabled",
///          "type": "boolean"
///        },
///        "fogReset": {
///          "description": "Timestamp of last fog reset",
///          "type": "number"
///        },
///        "foreground": {
///          "title": "Image Reference",
///          "description": "Image file URL or path",
///          "type": [
///            "string",
///            "null"
///          ],
///          "format": "uri",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "globalLight": {
///          "description": "Whether global illumination is enabled",
///          "type": "boolean"
///        },
///        "globalLightThreshold": {
///          "description": "Global light darkness threshold",
///          "type": "number",
///          "maximum": 1.0,
///          "minimum": 0.0
///        },
///        "grid": {
///          "type": "object",
///          "properties": {
///            "alpha": {
///              "description": "Grid opacity",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0
///            },
///            "color": {
///              "title": "Color Hex",
///              "description": "6-digit hexadecimal color code",
///              "type": "string",
///              "pattern": "^#[0-9a-fA-F]{6}$",
///              "$schema": "https://json-schema.org/draft-07/schema#"
///            },
///            "distance": {
///              "description": "Distance per grid unit",
///              "type": "number",
///              "minimum": 1.0
///            },
///            "size": {
///              "description": "Grid size in pixels",
///              "type": "number",
///              "minimum": 50.0
///            },
///            "type": {
///              "description": "Grid type: 0=None, 1=Square, 2=Hex(V), 3=Hex(H), 4=Gridless, 5=Isometric",
///              "type": "integer",
///              "enum": [
///                0,
///                1,
///                2,
///                3,
///                4,
///                5
///              ]
///            },
///            "units": {
///              "description": "Distance units (ft, m, etc.)",
///              "type": "string"
///            }
///          }
///        },
///        "height": {
///          "description": "Scene height in pixels",
///          "type": "number",
///          "minimum": 100.0
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
///        "initial": {
///          "title": "Position",
///          "description": "X/Y coordinate position",
///          "type": "object",
///          "properties": {
///            "x": {
///              "type": "number"
///            },
///            "y": {
///              "type": "number"
///            }
///          },
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "journal": {
///          "description": "Associated journal entry ID",
///          "type": [
///            "string",
///            "null"
///          ],
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "navOrder": {
///          "description": "Navigation bar sort order",
///          "type": "number"
///        },
///        "navigation": {
///          "description": "Whether scene appears in navigation bar",
///          "type": "boolean"
///        },
///        "padding": {
///          "description": "Scene boundary padding",
///          "type": "number",
///          "minimum": 0.0
///        },
///        "playlist": {
///          "description": "Scene playlist ID",
///          "type": [
///            "string",
///            "null"
///          ],
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "playlistSound": {
///          "description": "Background playlist sound ID",
///          "type": [
///            "string",
///            "null"
///          ],
///          "pattern": "^[a-zA-Z0-9]{16}$"
///        },
///        "thumb": {
///          "title": "Image Reference",
///          "description": "Image file URL or path",
///          "type": [
///            "string",
///            "null"
///          ],
///          "format": "uri",
///          "$schema": "https://json-schema.org/draft-07/schema#"
///        },
///        "tokenVision": {
///          "description": "Whether token vision is enabled",
///          "type": "boolean"
///        },
///        "weather": {
///          "description": "Weather conditions",
///          "type": "string"
///        },
///        "width": {
///          "description": "Scene width in pixels",
///          "type": "number",
///          "minimum": 100.0
///        }
///      }
///    }
///  ],
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryScene {
    ///Whether this is the active scene
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub active: ::std::option::Option<bool>,
    ///6-digit hexadecimal color code
    #[serde(
        rename = "backgroundColor",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub background_color: ::std::option::Option<ColorHex>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub darkness: ::std::option::Option<f64>,
    ///Module/system-specific flags
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Whether fog exploration is enabled
    #[serde(
        rename = "fogExploration",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fog_exploration: ::std::option::Option<bool>,
    #[serde(
        rename = "fogReset",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub fog_reset: ::std::option::Option<f64>,
    ///Parent folder ID, null if in root
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub folder: ::std::option::Option<FoundrySceneFolder>,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub foreground: ::std::option::Option<::std::string::String>,
    ///Whether global illumination is enabled
    #[serde(
        rename = "globalLight",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub global_light: ::std::option::Option<bool>,
    #[serde(
        rename = "globalLightThreshold",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub global_light_threshold: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grid: ::std::option::Option<FoundrySceneGrid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    ///Unique 16-character document ID
    #[serde(rename = "_id")]
    pub id: FoundrySceneId,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub img: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub initial: ::std::option::Option<Position>,
    ///Associated journal entry ID
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub journal: ::std::option::Option<FoundrySceneJournal>,
    ///Document name
    pub name: FoundrySceneName,
    #[serde(
        rename = "navOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub nav_order: ::std::option::Option<f64>,
    ///Whether scene appears in navigation bar
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub navigation: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ownership: ::std::option::Option<FoundryDocumentOwnership>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub padding: ::std::option::Option<f64>,
    ///Scene playlist ID
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playlist: ::std::option::Option<FoundryScenePlaylist>,
    ///Background playlist sound ID
    #[serde(
        rename = "playlistSound",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub playlist_sound: ::std::option::Option<FoundryScenePlaylistSound>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sort: ::std::option::Option<f64>,
    #[serde(
        rename = "_stats",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub stats: ::std::option::Option<FoundryDocumentStatistics>,
    ///Image file URL or path
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thumb: ::std::option::Option<::std::string::String>,
    ///Whether token vision is enabled
    #[serde(
        rename = "tokenVision",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_vision: ::std::option::Option<bool>,
    ///Weather conditions
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub weather: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::convert::From<&FoundryScene> for FoundryScene {
    fn from(value: &FoundryScene) -> Self {
        value.clone()
    }
}
impl FoundryScene {
    pub fn builder() -> builder::FoundryScene {
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
pub struct FoundrySceneFolder(::std::string::String);
impl ::std::ops::Deref for FoundrySceneFolder {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundrySceneFolder> for ::std::string::String {
    fn from(value: FoundrySceneFolder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundrySceneFolder> for FoundrySceneFolder {
    fn from(value: &FoundrySceneFolder) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundrySceneFolder {
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
impl ::std::convert::TryFrom<&str> for FoundrySceneFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundrySceneFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundrySceneFolder {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundrySceneFolder {
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
///`FoundrySceneGrid`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "alpha": {
///      "description": "Grid opacity",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0
///    },
///    "color": {
///      "title": "Color Hex",
///      "description": "6-digit hexadecimal color code",
///      "type": "string",
///      "pattern": "^#[0-9a-fA-F]{6}$",
///      "$schema": "https://json-schema.org/draft-07/schema#"
///    },
///    "distance": {
///      "description": "Distance per grid unit",
///      "type": "number",
///      "minimum": 1.0
///    },
///    "size": {
///      "description": "Grid size in pixels",
///      "type": "number",
///      "minimum": 50.0
///    },
///    "type": {
///      "description": "Grid type: 0=None, 1=Square, 2=Hex(V), 3=Hex(H), 4=Gridless, 5=Isometric",
///      "type": "integer",
///      "enum": [
///        0,
///        1,
///        2,
///        3,
///        4,
///        5
///      ]
///    },
///    "units": {
///      "description": "Distance units (ft, m, etc.)",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundrySceneGrid {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alpha: ::std::option::Option<f64>,
    ///6-digit hexadecimal color code
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<ColorHex>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub distance: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub size: ::std::option::Option<f64>,
    ///Grid type: 0=None, 1=Square, 2=Hex(V), 3=Hex(H), 4=Gridless, 5=Isometric
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<FoundrySceneGridType>,
    ///Distance units (ft, m, etc.)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub units: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundrySceneGrid> for FoundrySceneGrid {
    fn from(value: &FoundrySceneGrid) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundrySceneGrid {
    fn default() -> Self {
        Self {
            alpha: Default::default(),
            color: Default::default(),
            distance: Default::default(),
            size: Default::default(),
            type_: Default::default(),
            units: Default::default(),
        }
    }
}
impl FoundrySceneGrid {
    pub fn builder() -> builder::FoundrySceneGrid {
        Default::default()
    }
}
///Grid type: 0=None, 1=Square, 2=Hex(V), 3=Hex(H), 4=Gridless, 5=Isometric
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Grid type: 0=None, 1=Square, 2=Hex(V), 3=Hex(H), 4=Gridless, 5=Isometric",
///  "type": "integer",
///  "enum": [
///    0,
///    1,
///    2,
///    3,
///    4,
///    5
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FoundrySceneGridType(i64);
impl ::std::ops::Deref for FoundrySceneGridType {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl ::std::convert::From<FoundrySceneGridType> for i64 {
    fn from(value: FoundrySceneGridType) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundrySceneGridType> for FoundrySceneGridType {
    fn from(value: &FoundrySceneGridType) -> Self {
        value.clone()
    }
}
impl ::std::convert::TryFrom<i64> for FoundrySceneGridType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: i64,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![0_i64, 1_i64, 2_i64, 3_i64, 4_i64, 5_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundrySceneGridType {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| { <D::Error as ::serde::de::Error>::custom(e.to_string()) })
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
pub struct FoundrySceneId(::std::string::String);
impl ::std::ops::Deref for FoundrySceneId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundrySceneId> for ::std::string::String {
    fn from(value: FoundrySceneId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundrySceneId> for FoundrySceneId {
    fn from(value: &FoundrySceneId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundrySceneId {
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
impl ::std::convert::TryFrom<&str> for FoundrySceneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundrySceneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundrySceneId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundrySceneId {
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
///Associated journal entry ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Associated journal entry ID",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundrySceneJournal(::std::string::String);
impl ::std::ops::Deref for FoundrySceneJournal {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundrySceneJournal> for ::std::string::String {
    fn from(value: FoundrySceneJournal) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundrySceneJournal> for FoundrySceneJournal {
    fn from(value: &FoundrySceneJournal) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundrySceneJournal {
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
impl ::std::convert::TryFrom<&str> for FoundrySceneJournal {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundrySceneJournal {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundrySceneJournal {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundrySceneJournal {
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
pub struct FoundrySceneName(::std::string::String);
impl ::std::ops::Deref for FoundrySceneName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundrySceneName> for ::std::string::String {
    fn from(value: FoundrySceneName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundrySceneName> for FoundrySceneName {
    fn from(value: &FoundrySceneName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundrySceneName {
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
impl ::std::convert::TryFrom<&str> for FoundrySceneName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundrySceneName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundrySceneName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundrySceneName {
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
///Scene playlist ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Scene playlist ID",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryScenePlaylist(::std::string::String);
impl ::std::ops::Deref for FoundryScenePlaylist {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryScenePlaylist> for ::std::string::String {
    fn from(value: FoundryScenePlaylist) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryScenePlaylist> for FoundryScenePlaylist {
    fn from(value: &FoundryScenePlaylist) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryScenePlaylist {
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
impl ::std::convert::TryFrom<&str> for FoundryScenePlaylist {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryScenePlaylist {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryScenePlaylist {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryScenePlaylist {
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
///Background playlist sound ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Background playlist sound ID",
///  "type": "string",
///  "pattern": "^[a-zA-Z0-9]{16}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryScenePlaylistSound(::std::string::String);
impl ::std::ops::Deref for FoundryScenePlaylistSound {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryScenePlaylistSound> for ::std::string::String {
    fn from(value: FoundryScenePlaylistSound) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryScenePlaylistSound> for FoundryScenePlaylistSound {
    fn from(value: &FoundryScenePlaylistSound) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryScenePlaylistSound {
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
impl ::std::convert::TryFrom<&str> for FoundryScenePlaylistSound {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryScenePlaylistSound {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryScenePlaylistSound {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryScenePlaylistSound {
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
///X/Y coordinate position
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Position",
///  "description": "X/Y coordinate position",
///  "type": "object",
///  "properties": {
///    "x": {
///      "type": "number"
///    },
///    "y": {
///      "type": "number"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Position {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub x: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub y: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Position> for Position {
    fn from(value: &Position) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Position {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl Position {
    pub fn builder() -> builder::Position {
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
    pub struct FoundryScene {
        active: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        background_color: ::std::result::Result<
            ::std::option::Option<super::ColorHex>,
            ::std::string::String,
        >,
        darkness: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        fog_exploration: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        fog_reset: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        folder: ::std::result::Result<
            ::std::option::Option<super::FoundrySceneFolder>,
            ::std::string::String,
        >,
        foreground: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        global_light: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        global_light_threshold: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        grid: ::std::result::Result<
            ::std::option::Option<super::FoundrySceneGrid>,
            ::std::string::String,
        >,
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<super::FoundrySceneId, ::std::string::String>,
        img: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        initial: ::std::result::Result<
            ::std::option::Option<super::Position>,
            ::std::string::String,
        >,
        journal: ::std::result::Result<
            ::std::option::Option<super::FoundrySceneJournal>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::FoundrySceneName, ::std::string::String>,
        nav_order: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        navigation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        ownership: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentOwnership>,
            ::std::string::String,
        >,
        padding: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        playlist: ::std::result::Result<
            ::std::option::Option<super::FoundryScenePlaylist>,
            ::std::string::String,
        >,
        playlist_sound: ::std::result::Result<
            ::std::option::Option<super::FoundryScenePlaylistSound>,
            ::std::string::String,
        >,
        sort: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        stats: ::std::result::Result<
            ::std::option::Option<super::FoundryDocumentStatistics>,
            ::std::string::String,
        >,
        thumb: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        token_vision: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        weather: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryScene {
        fn default() -> Self {
            Self {
                active: Ok(Default::default()),
                background_color: Ok(Default::default()),
                darkness: Ok(Default::default()),
                flags: Ok(Default::default()),
                fog_exploration: Ok(Default::default()),
                fog_reset: Ok(Default::default()),
                folder: Ok(Default::default()),
                foreground: Ok(Default::default()),
                global_light: Ok(Default::default()),
                global_light_threshold: Ok(Default::default()),
                grid: Ok(Default::default()),
                height: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                img: Ok(Default::default()),
                initial: Ok(Default::default()),
                journal: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                nav_order: Ok(Default::default()),
                navigation: Ok(Default::default()),
                ownership: Ok(Default::default()),
                padding: Ok(Default::default()),
                playlist: Ok(Default::default()),
                playlist_sound: Ok(Default::default()),
                sort: Ok(Default::default()),
                stats: Ok(Default::default()),
                thumb: Ok(Default::default()),
                token_vision: Ok(Default::default()),
                weather: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl FoundryScene {
        pub fn active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.active = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for active: {}", e)
                });
            self
        }
        pub fn background_color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ColorHex>>,
            T::Error: ::std::fmt::Display,
        {
            self.background_color = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for background_color: {}", e
                    )
                });
            self
        }
        pub fn darkness<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.darkness = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for darkness: {}", e)
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
        pub fn fog_exploration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.fog_exploration = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fog_exploration: {}", e)
                });
            self
        }
        pub fn fog_reset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fog_reset = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fog_reset: {}", e)
                });
            self
        }
        pub fn folder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FoundrySceneFolder>>,
            T::Error: ::std::fmt::Display,
        {
            self.folder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for folder: {}", e)
                });
            self
        }
        pub fn foreground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.foreground = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for foreground: {}", e)
                });
            self
        }
        pub fn global_light<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.global_light = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for global_light: {}", e)
                });
            self
        }
        pub fn global_light_threshold<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.global_light_threshold = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for global_light_threshold: {}",
                        e
                    )
                });
            self
        }
        pub fn grid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FoundrySceneGrid>>,
            T::Error: ::std::fmt::Display,
        {
            self.grid = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for grid: {}", e));
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
            T: ::std::convert::TryInto<super::FoundrySceneId>,
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
        pub fn initial<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Position>>,
            T::Error: ::std::fmt::Display,
        {
            self.initial = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for initial: {}", e)
                });
            self
        }
        pub fn journal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundrySceneJournal>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.journal = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journal: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundrySceneName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nav_order<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.nav_order = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nav_order: {}", e)
                });
            self
        }
        pub fn navigation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.navigation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for navigation: {}", e)
                });
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
        pub fn padding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.padding = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for padding: {}", e)
                });
            self
        }
        pub fn playlist<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryScenePlaylist>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.playlist = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playlist: {}", e)
                });
            self
        }
        pub fn playlist_sound<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryScenePlaylistSound>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.playlist_sound = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playlist_sound: {}", e)
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
        pub fn thumb<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.thumb = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for thumb: {}", e)
                });
            self
        }
        pub fn token_vision<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.token_vision = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for token_vision: {}", e)
                });
            self
        }
        pub fn weather<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.weather = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for weather: {}", e)
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
    impl ::std::convert::TryFrom<FoundryScene> for super::FoundryScene {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryScene,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active: value.active?,
                background_color: value.background_color?,
                darkness: value.darkness?,
                flags: value.flags?,
                fog_exploration: value.fog_exploration?,
                fog_reset: value.fog_reset?,
                folder: value.folder?,
                foreground: value.foreground?,
                global_light: value.global_light?,
                global_light_threshold: value.global_light_threshold?,
                grid: value.grid?,
                height: value.height?,
                id: value.id?,
                img: value.img?,
                initial: value.initial?,
                journal: value.journal?,
                name: value.name?,
                nav_order: value.nav_order?,
                navigation: value.navigation?,
                ownership: value.ownership?,
                padding: value.padding?,
                playlist: value.playlist?,
                playlist_sound: value.playlist_sound?,
                sort: value.sort?,
                stats: value.stats?,
                thumb: value.thumb?,
                token_vision: value.token_vision?,
                weather: value.weather?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryScene> for FoundryScene {
        fn from(value: super::FoundryScene) -> Self {
            Self {
                active: Ok(value.active),
                background_color: Ok(value.background_color),
                darkness: Ok(value.darkness),
                flags: Ok(value.flags),
                fog_exploration: Ok(value.fog_exploration),
                fog_reset: Ok(value.fog_reset),
                folder: Ok(value.folder),
                foreground: Ok(value.foreground),
                global_light: Ok(value.global_light),
                global_light_threshold: Ok(value.global_light_threshold),
                grid: Ok(value.grid),
                height: Ok(value.height),
                id: Ok(value.id),
                img: Ok(value.img),
                initial: Ok(value.initial),
                journal: Ok(value.journal),
                name: Ok(value.name),
                nav_order: Ok(value.nav_order),
                navigation: Ok(value.navigation),
                ownership: Ok(value.ownership),
                padding: Ok(value.padding),
                playlist: Ok(value.playlist),
                playlist_sound: Ok(value.playlist_sound),
                sort: Ok(value.sort),
                stats: Ok(value.stats),
                thumb: Ok(value.thumb),
                token_vision: Ok(value.token_vision),
                weather: Ok(value.weather),
                width: Ok(value.width),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundrySceneGrid {
        alpha: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        color: ::std::result::Result<
            ::std::option::Option<super::ColorHex>,
            ::std::string::String,
        >,
        distance: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        size: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::FoundrySceneGridType>,
            ::std::string::String,
        >,
        units: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundrySceneGrid {
        fn default() -> Self {
            Self {
                alpha: Ok(Default::default()),
                color: Ok(Default::default()),
                distance: Ok(Default::default()),
                size: Ok(Default::default()),
                type_: Ok(Default::default()),
                units: Ok(Default::default()),
            }
        }
    }
    impl FoundrySceneGrid {
        pub fn alpha<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.alpha = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for alpha: {}", e)
                });
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ColorHex>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn distance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.distance = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for distance: {}", e)
                });
            self
        }
        pub fn size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for size: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundrySceneGridType>,
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
    }
    impl ::std::convert::TryFrom<FoundrySceneGrid> for super::FoundrySceneGrid {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundrySceneGrid,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                alpha: value.alpha?,
                color: value.color?,
                distance: value.distance?,
                size: value.size?,
                type_: value.type_?,
                units: value.units?,
            })
        }
    }
    impl ::std::convert::From<super::FoundrySceneGrid> for FoundrySceneGrid {
        fn from(value: super::FoundrySceneGrid) -> Self {
            Self {
                alpha: Ok(value.alpha),
                color: Ok(value.color),
                distance: Ok(value.distance),
                size: Ok(value.size),
                type_: Ok(value.type_),
                units: Ok(value.units),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Position {
        x: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        y: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                x: Ok(Default::default()),
                y: Ok(Default::default()),
            }
        }
    }
    impl Position {
        pub fn x<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.x = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for x: {}", e));
            self
        }
        pub fn y<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.y = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for y: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Position> for super::Position {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Position,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { x: value.x?, y: value.y? })
        }
    }
    impl ::std::convert::From<super::Position> for Position {
        fn from(value: super::Position) -> Self {
            Self {
                x: Ok(value.x),
                y: Ok(value.y),
            }
        }
    }
}
