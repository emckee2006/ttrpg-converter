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
///Schema for Foundry VTT module.json manifest files
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry VTT Module Manifest",
///  "description": "Schema for Foundry VTT module.json manifest files",
///  "type": "object",
///  "required": [
///    "authors",
///    "compatibility",
///    "description",
///    "id",
///    "title",
///    "version"
///  ],
///  "properties": {
///    "authors": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "discord": {
///            "description": "Author Discord username",
///            "type": "string"
///          },
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
///            "description": "Author URL",
///            "type": "string",
///            "format": "uri"
///          }
///        }
///      }
///    },
///    "bugs": {
///      "description": "Bug report URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "changelog": {
///      "description": "Changelog URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "compatibility": {
///      "type": "object",
///      "required": [
///        "minimum",
///        "verified"
///      ],
///      "properties": {
///        "maximum": {
///          "description": "Maximum compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        },
///        "minimum": {
///          "description": "Minimum compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        },
///        "verified": {
///          "description": "Verified compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        }
///      }
///    },
///    "conflicts": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "id"
///        ],
///        "properties": {
///          "id": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string",
///            "enum": [
///              "module",
///              "system",
///              "world"
///            ]
///          },
///          "versionMax": {
///            "type": "string"
///          },
///          "versionMin": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "coreTranslation": {
///      "description": "Whether module provides core translations",
///      "type": "boolean"
///    },
///    "dependencies": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "id",
///          "type"
///        ],
///        "properties": {
///          "compatibility": {
///            "type": "object",
///            "required": [
///              "minimum",
///              "verified"
///            ],
///            "properties": {
///              "maximum": {
///                "description": "Maximum compatible Foundry version",
///                "type": "string",
///                "pattern": "^\\d+\\.\\d+\\.\\d+$"
///              },
///              "minimum": {
///                "description": "Minimum compatible Foundry version",
///                "type": "string",
///                "pattern": "^\\d+\\.\\d+\\.\\d+$"
///              },
///              "verified": {
///                "description": "Verified compatible Foundry version",
///                "type": "string",
///                "pattern": "^\\d+\\.\\d+\\.\\d+$"
///              }
///            }
///          },
///          "id": {
///            "type": "string"
///          },
///          "manifest": {
///            "type": "string",
///            "format": "uri"
///          },
///          "type": {
///            "type": "string",
///            "enum": [
///              "module",
///              "system",
///              "world"
///            ]
///          }
///        }
///      }
///    },
///    "description": {
///      "description": "Module description",
///      "type": "string"
///    },
///    "download": {
///      "description": "Module download URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "esmodules": {
///      "description": "ES module files",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "id": {
///      "description": "Module identifier",
///      "type": "string",
///      "pattern": "^[a-z0-9-_]+$"
///    },
///    "languages": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "lang",
///          "name",
///          "path"
///        ],
///        "properties": {
///          "lang": {
///            "type": "string",
///            "pattern": "^[a-z]{2}(-[A-Z]{2})?$"
///          },
///          "name": {
///            "type": "string"
///          },
///          "path": {
///            "type": "string"
///          }
///        }
///      }
///    },
///    "library": {
///      "description": "Whether module is a library",
///      "type": "boolean"
///    },
///    "license": {
///      "description": "Module license",
///      "type": "string"
///    },
///    "manifest": {
///      "description": "Module manifest URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "media": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "caption": {
///            "type": "string"
///          },
///          "loop": {
///            "type": "boolean"
///          },
///          "thumbnail": {
///            "type": "string",
///            "format": "uri"
///          },
///          "type": {
///            "type": "string",
///            "enum": [
///              "icon",
///              "screenshot",
///              "video"
///            ]
///          },
///          "url": {
///            "type": "string",
///            "format": "uri"
///          }
///        }
///      }
///    },
///    "packs": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "label",
///          "name",
///          "path",
///          "type"
///        ],
///        "properties": {
///          "flags": {
///            "type": "object"
///          },
///          "label": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "path": {
///            "type": "string"
///          },
///          "private": {
///            "type": "boolean"
///          },
///          "system": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string",
///            "enum": [
///              "Actor",
///              "Item",
///              "Scene",
///              "JournalEntry",
///              "Macro",
///              "RollTable",
///              "Playlist",
///              "Adventure",
///              "Card",
///              "Cards"
///            ]
///          }
///        }
///      }
///    },
///    "protected": {
///      "description": "Whether module is DRM protected",
///      "type": "boolean"
///    },
///    "readme": {
///      "description": "README file URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "scripts": {
///      "description": "Script files",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "socket": {
///      "description": "Whether module uses socket functionality",
///      "type": "boolean"
///    },
///    "styles": {
///      "description": "CSS files",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "system": {
///      "description": "Compatible systems",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "title": {
///      "description": "Module display name",
///      "type": "string"
///    },
///    "url": {
///      "description": "Module homepage URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "version": {
///      "description": "Semantic version string",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifest {
    pub authors: ::std::vec::Vec<FoundryVttModuleManifestAuthorsItem>,
    ///Bug report URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bugs: ::std::option::Option<::std::string::String>,
    ///Changelog URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub changelog: ::std::option::Option<::std::string::String>,
    pub compatibility: FoundryVttModuleManifestCompatibility,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub conflicts: ::std::vec::Vec<FoundryVttModuleManifestConflictsItem>,
    ///Whether module provides core translations
    #[serde(
        rename = "coreTranslation",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub core_translation: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub dependencies: ::std::vec::Vec<FoundryVttModuleManifestDependenciesItem>,
    ///Module description
    pub description: ::std::string::String,
    ///Module download URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub download: ::std::option::Option<::std::string::String>,
    ///ES module files
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub esmodules: ::std::vec::Vec<::std::string::String>,
    ///Module identifier
    pub id: FoundryVttModuleManifestId,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub languages: ::std::vec::Vec<FoundryVttModuleManifestLanguagesItem>,
    ///Whether module is a library
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub library: ::std::option::Option<bool>,
    ///Module license
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub license: ::std::option::Option<::std::string::String>,
    ///Module manifest URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub manifest: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub media: ::std::vec::Vec<FoundryVttModuleManifestMediaItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub packs: ::std::vec::Vec<FoundryVttModuleManifestPacksItem>,
    ///Whether module is DRM protected
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protected: ::std::option::Option<bool>,
    ///README file URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub readme: ::std::option::Option<::std::string::String>,
    ///Script files
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scripts: ::std::vec::Vec<::std::string::String>,
    ///Whether module uses socket functionality
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub socket: ::std::option::Option<bool>,
    ///CSS files
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub styles: ::std::vec::Vec<::std::string::String>,
    ///Compatible systems
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub system: ::std::vec::Vec<::std::string::String>,
    ///Module display name
    pub title: ::std::string::String,
    ///Module homepage URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    ///Semantic version string
    pub version: FoundryVttModuleManifestVersion,
}
impl ::std::convert::From<&FoundryVttModuleManifest> for FoundryVttModuleManifest {
    fn from(value: &FoundryVttModuleManifest) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifest {
    pub fn builder() -> builder::FoundryVttModuleManifest {
        Default::default()
    }
}
///`FoundryVttModuleManifestAuthorsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "discord": {
///      "description": "Author Discord username",
///      "type": "string"
///    },
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
///      "description": "Author URL",
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestAuthorsItem {
    ///Author Discord username
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub discord: ::std::option::Option<::std::string::String>,
    ///Author email
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    ///Author name
    pub name: ::std::string::String,
    ///Author URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryVttModuleManifestAuthorsItem>
for FoundryVttModuleManifestAuthorsItem {
    fn from(value: &FoundryVttModuleManifestAuthorsItem) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestAuthorsItem {
    pub fn builder() -> builder::FoundryVttModuleManifestAuthorsItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestCompatibility`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "minimum",
///    "verified"
///  ],
///  "properties": {
///    "maximum": {
///      "description": "Maximum compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    },
///    "minimum": {
///      "description": "Minimum compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    },
///    "verified": {
///      "description": "Verified compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestCompatibility {
    ///Maximum compatible Foundry version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<FoundryVttModuleManifestCompatibilityMaximum>,
    ///Minimum compatible Foundry version
    pub minimum: FoundryVttModuleManifestCompatibilityMinimum,
    ///Verified compatible Foundry version
    pub verified: FoundryVttModuleManifestCompatibilityVerified,
}
impl ::std::convert::From<&FoundryVttModuleManifestCompatibility>
for FoundryVttModuleManifestCompatibility {
    fn from(value: &FoundryVttModuleManifestCompatibility) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestCompatibility {
    pub fn builder() -> builder::FoundryVttModuleManifestCompatibility {
        Default::default()
    }
}
///Maximum compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Maximum compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestCompatibilityMaximum(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestCompatibilityMaximum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestCompatibilityMaximum>
for ::std::string::String {
    fn from(value: FoundryVttModuleManifestCompatibilityMaximum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestCompatibilityMaximum>
for FoundryVttModuleManifestCompatibilityMaximum {
    fn from(value: &FoundryVttModuleManifestCompatibilityMaximum) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestCompatibilityMaximum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestCompatibilityMaximum {
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
///Minimum compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Minimum compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestCompatibilityMinimum(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestCompatibilityMinimum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestCompatibilityMinimum>
for ::std::string::String {
    fn from(value: FoundryVttModuleManifestCompatibilityMinimum) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestCompatibilityMinimum>
for FoundryVttModuleManifestCompatibilityMinimum {
    fn from(value: &FoundryVttModuleManifestCompatibilityMinimum) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestCompatibilityMinimum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestCompatibilityMinimum {
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
///Verified compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Verified compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestCompatibilityVerified(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestCompatibilityVerified {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestCompatibilityVerified>
for ::std::string::String {
    fn from(value: FoundryVttModuleManifestCompatibilityVerified) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestCompatibilityVerified>
for FoundryVttModuleManifestCompatibilityVerified {
    fn from(value: &FoundryVttModuleManifestCompatibilityVerified) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestCompatibilityVerified {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestCompatibilityVerified {
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
///`FoundryVttModuleManifestConflictsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "module",
///        "system",
///        "world"
///      ]
///    },
///    "versionMax": {
///      "type": "string"
///    },
///    "versionMin": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestConflictsItem {
    pub id: ::std::string::String,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<FoundryVttModuleManifestConflictsItemType>,
    #[serde(
        rename = "versionMax",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_max: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "versionMin",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub version_min: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryVttModuleManifestConflictsItem>
for FoundryVttModuleManifestConflictsItem {
    fn from(value: &FoundryVttModuleManifestConflictsItem) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestConflictsItem {
    pub fn builder() -> builder::FoundryVttModuleManifestConflictsItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestConflictsItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "module",
///    "system",
///    "world"
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
pub enum FoundryVttModuleManifestConflictsItemType {
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "world")]
    World,
}
impl ::std::convert::From<&Self> for FoundryVttModuleManifestConflictsItemType {
    fn from(value: &FoundryVttModuleManifestConflictsItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttModuleManifestConflictsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Module => f.write_str("module"),
            Self::System => f.write_str("system"),
            Self::World => f.write_str("world"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestConflictsItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "module" => Ok(Self::Module),
            "system" => Ok(Self::System),
            "world" => Ok(Self::World),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestConflictsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestConflictsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestConflictsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryVttModuleManifestDependenciesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "type"
///  ],
///  "properties": {
///    "compatibility": {
///      "type": "object",
///      "required": [
///        "minimum",
///        "verified"
///      ],
///      "properties": {
///        "maximum": {
///          "description": "Maximum compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        },
///        "minimum": {
///          "description": "Minimum compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        },
///        "verified": {
///          "description": "Verified compatible Foundry version",
///          "type": "string",
///          "pattern": "^\\d+\\.\\d+\\.\\d+$"
///        }
///      }
///    },
///    "id": {
///      "type": "string"
///    },
///    "manifest": {
///      "type": "string",
///      "format": "uri"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "module",
///        "system",
///        "world"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestDependenciesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub compatibility: ::std::option::Option<
        FoundryVttModuleManifestDependenciesItemCompatibility,
    >,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub manifest: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FoundryVttModuleManifestDependenciesItemType,
}
impl ::std::convert::From<&FoundryVttModuleManifestDependenciesItem>
for FoundryVttModuleManifestDependenciesItem {
    fn from(value: &FoundryVttModuleManifestDependenciesItem) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestDependenciesItem {
    pub fn builder() -> builder::FoundryVttModuleManifestDependenciesItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestDependenciesItemCompatibility`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "minimum",
///    "verified"
///  ],
///  "properties": {
///    "maximum": {
///      "description": "Maximum compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    },
///    "minimum": {
///      "description": "Minimum compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    },
///    "verified": {
///      "description": "Verified compatible Foundry version",
///      "type": "string",
///      "pattern": "^\\d+\\.\\d+\\.\\d+$"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestDependenciesItemCompatibility {
    ///Maximum compatible Foundry version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<
        FoundryVttModuleManifestDependenciesItemCompatibilityMaximum,
    >,
    ///Minimum compatible Foundry version
    pub minimum: FoundryVttModuleManifestDependenciesItemCompatibilityMinimum,
    ///Verified compatible Foundry version
    pub verified: FoundryVttModuleManifestDependenciesItemCompatibilityVerified,
}
impl ::std::convert::From<&FoundryVttModuleManifestDependenciesItemCompatibility>
for FoundryVttModuleManifestDependenciesItemCompatibility {
    fn from(value: &FoundryVttModuleManifestDependenciesItemCompatibility) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestDependenciesItemCompatibility {
    pub fn builder() -> builder::FoundryVttModuleManifestDependenciesItemCompatibility {
        Default::default()
    }
}
///Maximum compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Maximum compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestDependenciesItemCompatibilityMaximum(
    ::std::string::String,
);
impl ::std::ops::Deref for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestDependenciesItemCompatibilityMaximum>
for ::std::string::String {
    fn from(
        value: FoundryVttModuleManifestDependenciesItemCompatibilityMaximum,
    ) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestDependenciesItemCompatibilityMaximum>
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    fn from(
        value: &FoundryVttModuleManifestDependenciesItemCompatibilityMaximum,
    ) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for FoundryVttModuleManifestDependenciesItemCompatibilityMaximum {
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
///Minimum compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Minimum compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestDependenciesItemCompatibilityMinimum(
    ::std::string::String,
);
impl ::std::ops::Deref for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestDependenciesItemCompatibilityMinimum>
for ::std::string::String {
    fn from(
        value: FoundryVttModuleManifestDependenciesItemCompatibilityMinimum,
    ) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestDependenciesItemCompatibilityMinimum>
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    fn from(
        value: &FoundryVttModuleManifestDependenciesItemCompatibilityMinimum,
    ) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for FoundryVttModuleManifestDependenciesItemCompatibilityMinimum {
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
///Verified compatible Foundry version
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Verified compatible Foundry version",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestDependenciesItemCompatibilityVerified(
    ::std::string::String,
);
impl ::std::ops::Deref
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestDependenciesItemCompatibilityVerified>
for ::std::string::String {
    fn from(
        value: FoundryVttModuleManifestDependenciesItemCompatibilityVerified,
    ) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestDependenciesItemCompatibilityVerified>
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    fn from(
        value: &FoundryVttModuleManifestDependenciesItemCompatibilityVerified,
    ) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str>
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de>
for FoundryVttModuleManifestDependenciesItemCompatibilityVerified {
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
///`FoundryVttModuleManifestDependenciesItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "module",
///    "system",
///    "world"
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
pub enum FoundryVttModuleManifestDependenciesItemType {
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "world")]
    World,
}
impl ::std::convert::From<&Self> for FoundryVttModuleManifestDependenciesItemType {
    fn from(value: &FoundryVttModuleManifestDependenciesItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttModuleManifestDependenciesItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Module => f.write_str("module"),
            Self::System => f.write_str("system"),
            Self::World => f.write_str("world"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestDependenciesItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "module" => Ok(Self::Module),
            "system" => Ok(Self::System),
            "world" => Ok(Self::World),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestDependenciesItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestDependenciesItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestDependenciesItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Module identifier
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Module identifier",
///  "type": "string",
///  "pattern": "^[a-z0-9-_]+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestId(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestId> for ::std::string::String {
    fn from(value: FoundryVttModuleManifestId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestId> for FoundryVttModuleManifestId {
    fn from(value: &FoundryVttModuleManifestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z0-9-_]+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9-_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FoundryVttModuleManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttModuleManifestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestId {
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
///`FoundryVttModuleManifestLanguagesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "lang",
///    "name",
///    "path"
///  ],
///  "properties": {
///    "lang": {
///      "type": "string",
///      "pattern": "^[a-z]{2}(-[A-Z]{2})?$"
///    },
///    "name": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestLanguagesItem {
    pub lang: FoundryVttModuleManifestLanguagesItemLang,
    pub name: ::std::string::String,
    pub path: ::std::string::String,
}
impl ::std::convert::From<&FoundryVttModuleManifestLanguagesItem>
for FoundryVttModuleManifestLanguagesItem {
    fn from(value: &FoundryVttModuleManifestLanguagesItem) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestLanguagesItem {
    pub fn builder() -> builder::FoundryVttModuleManifestLanguagesItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestLanguagesItemLang`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[a-z]{2}(-[A-Z]{2})?$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestLanguagesItemLang(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestLanguagesItemLang {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestLanguagesItemLang>
for ::std::string::String {
    fn from(value: FoundryVttModuleManifestLanguagesItemLang) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestLanguagesItemLang>
for FoundryVttModuleManifestLanguagesItemLang {
    fn from(value: &FoundryVttModuleManifestLanguagesItemLang) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestLanguagesItemLang {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-z]{2}(-[A-Z]{2})?$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z]{2}(-[A-Z]{2})?$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestLanguagesItemLang {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestLanguagesItemLang {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestLanguagesItemLang {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestLanguagesItemLang {
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
///`FoundryVttModuleManifestMediaItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "caption": {
///      "type": "string"
///    },
///    "loop": {
///      "type": "boolean"
///    },
///    "thumbnail": {
///      "type": "string",
///      "format": "uri"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "icon",
///        "screenshot",
///        "video"
///      ]
///    },
///    "url": {
///      "type": "string",
///      "format": "uri"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestMediaItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub caption: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "loop",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub loop_: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub thumbnail: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<FoundryVttModuleManifestMediaItemType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryVttModuleManifestMediaItem>
for FoundryVttModuleManifestMediaItem {
    fn from(value: &FoundryVttModuleManifestMediaItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryVttModuleManifestMediaItem {
    fn default() -> Self {
        Self {
            caption: Default::default(),
            loop_: Default::default(),
            thumbnail: Default::default(),
            type_: Default::default(),
            url: Default::default(),
        }
    }
}
impl FoundryVttModuleManifestMediaItem {
    pub fn builder() -> builder::FoundryVttModuleManifestMediaItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestMediaItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "icon",
///    "screenshot",
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
pub enum FoundryVttModuleManifestMediaItemType {
    #[serde(rename = "icon")]
    Icon,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "video")]
    Video,
}
impl ::std::convert::From<&Self> for FoundryVttModuleManifestMediaItemType {
    fn from(value: &FoundryVttModuleManifestMediaItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttModuleManifestMediaItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Icon => f.write_str("icon"),
            Self::Screenshot => f.write_str("screenshot"),
            Self::Video => f.write_str("video"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestMediaItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "icon" => Ok(Self::Icon),
            "screenshot" => Ok(Self::Screenshot),
            "video" => Ok(Self::Video),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestMediaItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestMediaItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestMediaItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`FoundryVttModuleManifestPacksItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "label",
///    "name",
///    "path",
///    "type"
///  ],
///  "properties": {
///    "flags": {
///      "type": "object"
///    },
///    "label": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "path": {
///      "type": "string"
///    },
///    "private": {
///      "type": "boolean"
///    },
///    "system": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "Actor",
///        "Item",
///        "Scene",
///        "JournalEntry",
///        "Macro",
///        "RollTable",
///        "Playlist",
///        "Adventure",
///        "Card",
///        "Cards"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryVttModuleManifestPacksItem {
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub flags: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub label: ::std::string::String,
    pub name: ::std::string::String,
    pub path: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub private: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: FoundryVttModuleManifestPacksItemType,
}
impl ::std::convert::From<&FoundryVttModuleManifestPacksItem>
for FoundryVttModuleManifestPacksItem {
    fn from(value: &FoundryVttModuleManifestPacksItem) -> Self {
        value.clone()
    }
}
impl FoundryVttModuleManifestPacksItem {
    pub fn builder() -> builder::FoundryVttModuleManifestPacksItem {
        Default::default()
    }
}
///`FoundryVttModuleManifestPacksItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Actor",
///    "Item",
///    "Scene",
///    "JournalEntry",
///    "Macro",
///    "RollTable",
///    "Playlist",
///    "Adventure",
///    "Card",
///    "Cards"
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
pub enum FoundryVttModuleManifestPacksItemType {
    Actor,
    Item,
    Scene,
    JournalEntry,
    Macro,
    RollTable,
    Playlist,
    Adventure,
    Card,
    Cards,
}
impl ::std::convert::From<&Self> for FoundryVttModuleManifestPacksItemType {
    fn from(value: &FoundryVttModuleManifestPacksItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for FoundryVttModuleManifestPacksItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Actor => f.write_str("Actor"),
            Self::Item => f.write_str("Item"),
            Self::Scene => f.write_str("Scene"),
            Self::JournalEntry => f.write_str("JournalEntry"),
            Self::Macro => f.write_str("Macro"),
            Self::RollTable => f.write_str("RollTable"),
            Self::Playlist => f.write_str("Playlist"),
            Self::Adventure => f.write_str("Adventure"),
            Self::Card => f.write_str("Card"),
            Self::Cards => f.write_str("Cards"),
        }
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestPacksItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Actor" => Ok(Self::Actor),
            "Item" => Ok(Self::Item),
            "Scene" => Ok(Self::Scene),
            "JournalEntry" => Ok(Self::JournalEntry),
            "Macro" => Ok(Self::Macro),
            "RollTable" => Ok(Self::RollTable),
            "Playlist" => Ok(Self::Playlist),
            "Adventure" => Ok(Self::Adventure),
            "Card" => Ok(Self::Card),
            "Cards" => Ok(Self::Cards),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestPacksItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestPacksItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for FoundryVttModuleManifestPacksItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Semantic version string
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Semantic version string",
///  "type": "string",
///  "pattern": "^\\d+\\.\\d+\\.\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FoundryVttModuleManifestVersion(::std::string::String);
impl ::std::ops::Deref for FoundryVttModuleManifestVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FoundryVttModuleManifestVersion> for ::std::string::String {
    fn from(value: FoundryVttModuleManifestVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FoundryVttModuleManifestVersion>
for FoundryVttModuleManifestVersion {
    fn from(value: &FoundryVttModuleManifestVersion) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FoundryVttModuleManifestVersion {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+\\.\\d+\\.\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+\\.\\d+\\.\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FoundryVttModuleManifestVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for FoundryVttModuleManifestVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FoundryVttModuleManifestVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FoundryVttModuleManifestVersion {
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
    pub struct FoundryVttModuleManifest {
        authors: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestAuthorsItem>,
            ::std::string::String,
        >,
        bugs: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        changelog: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        compatibility: ::std::result::Result<
            super::FoundryVttModuleManifestCompatibility,
            ::std::string::String,
        >,
        conflicts: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestConflictsItem>,
            ::std::string::String,
        >,
        core_translation: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        dependencies: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestDependenciesItem>,
            ::std::string::String,
        >,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        download: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        esmodules: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            super::FoundryVttModuleManifestId,
            ::std::string::String,
        >,
        languages: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestLanguagesItem>,
            ::std::string::String,
        >,
        library: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        license: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        manifest: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        media: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestMediaItem>,
            ::std::string::String,
        >,
        packs: ::std::result::Result<
            ::std::vec::Vec<super::FoundryVttModuleManifestPacksItem>,
            ::std::string::String,
        >,
        protected: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        readme: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scripts: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        socket: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        styles: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version: ::std::result::Result<
            super::FoundryVttModuleManifestVersion,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifest {
        fn default() -> Self {
            Self {
                authors: Err("no value supplied for authors".to_string()),
                bugs: Ok(Default::default()),
                changelog: Ok(Default::default()),
                compatibility: Err("no value supplied for compatibility".to_string()),
                conflicts: Ok(Default::default()),
                core_translation: Ok(Default::default()),
                dependencies: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                download: Ok(Default::default()),
                esmodules: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                languages: Ok(Default::default()),
                library: Ok(Default::default()),
                license: Ok(Default::default()),
                manifest: Ok(Default::default()),
                media: Ok(Default::default()),
                packs: Ok(Default::default()),
                protected: Ok(Default::default()),
                readme: Ok(Default::default()),
                scripts: Ok(Default::default()),
                socket: Ok(Default::default()),
                styles: Ok(Default::default()),
                system: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                url: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifest {
        pub fn authors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestAuthorsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.authors = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for authors: {}", e)
                });
            self
        }
        pub fn bugs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bugs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bugs: {}", e));
            self
        }
        pub fn changelog<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.changelog = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for changelog: {}", e)
                });
            self
        }
        pub fn compatibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttModuleManifestCompatibility>,
            T::Error: ::std::fmt::Display,
        {
            self.compatibility = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for compatibility: {}", e)
                });
            self
        }
        pub fn conflicts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestConflictsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.conflicts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for conflicts: {}", e)
                });
            self
        }
        pub fn core_translation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.core_translation = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for core_translation: {}", e
                    )
                });
            self
        }
        pub fn dependencies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestDependenciesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.dependencies = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dependencies: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn download<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.download = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for download: {}", e)
                });
            self
        }
        pub fn esmodules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.esmodules = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for esmodules: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttModuleManifestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn languages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestLanguagesItem>,
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
        pub fn library<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.library = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for library: {}", e)
                });
            self
        }
        pub fn license<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.license = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for license: {}", e)
                });
            self
        }
        pub fn manifest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.manifest = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for manifest: {}", e)
                });
            self
        }
        pub fn media<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestMediaItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.media = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for media: {}", e)
                });
            self
        }
        pub fn packs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::FoundryVttModuleManifestPacksItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.packs = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for packs: {}", e)
                });
            self
        }
        pub fn protected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.protected = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for protected: {}", e)
                });
            self
        }
        pub fn readme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.readme = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for readme: {}", e)
                });
            self
        }
        pub fn scripts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scripts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for scripts: {}", e)
                });
            self
        }
        pub fn socket<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.socket = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for socket: {}", e)
                });
            self
        }
        pub fn styles<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.styles = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for styles: {}", e)
                });
            self
        }
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.system = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
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
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttModuleManifestVersion>,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifest>
    for super::FoundryVttModuleManifest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authors: value.authors?,
                bugs: value.bugs?,
                changelog: value.changelog?,
                compatibility: value.compatibility?,
                conflicts: value.conflicts?,
                core_translation: value.core_translation?,
                dependencies: value.dependencies?,
                description: value.description?,
                download: value.download?,
                esmodules: value.esmodules?,
                id: value.id?,
                languages: value.languages?,
                library: value.library?,
                license: value.license?,
                manifest: value.manifest?,
                media: value.media?,
                packs: value.packs?,
                protected: value.protected?,
                readme: value.readme?,
                scripts: value.scripts?,
                socket: value.socket?,
                styles: value.styles?,
                system: value.system?,
                title: value.title?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifest>
    for FoundryVttModuleManifest {
        fn from(value: super::FoundryVttModuleManifest) -> Self {
            Self {
                authors: Ok(value.authors),
                bugs: Ok(value.bugs),
                changelog: Ok(value.changelog),
                compatibility: Ok(value.compatibility),
                conflicts: Ok(value.conflicts),
                core_translation: Ok(value.core_translation),
                dependencies: Ok(value.dependencies),
                description: Ok(value.description),
                download: Ok(value.download),
                esmodules: Ok(value.esmodules),
                id: Ok(value.id),
                languages: Ok(value.languages),
                library: Ok(value.library),
                license: Ok(value.license),
                manifest: Ok(value.manifest),
                media: Ok(value.media),
                packs: Ok(value.packs),
                protected: Ok(value.protected),
                readme: Ok(value.readme),
                scripts: Ok(value.scripts),
                socket: Ok(value.socket),
                styles: Ok(value.styles),
                system: Ok(value.system),
                title: Ok(value.title),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestAuthorsItem {
        discord: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestAuthorsItem {
        fn default() -> Self {
            Self {
                discord: Ok(Default::default()),
                email: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                url: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttModuleManifestAuthorsItem {
        pub fn discord<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.discord = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for discord: {}", e)
                });
            self
        }
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
            T: ::std::convert::TryInto<::std::string::String>,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifestAuthorsItem>
    for super::FoundryVttModuleManifestAuthorsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestAuthorsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                discord: value.discord?,
                email: value.email?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestAuthorsItem>
    for FoundryVttModuleManifestAuthorsItem {
        fn from(value: super::FoundryVttModuleManifestAuthorsItem) -> Self {
            Self {
                discord: Ok(value.discord),
                email: Ok(value.email),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestCompatibility {
        maximum: ::std::result::Result<
            ::std::option::Option<super::FoundryVttModuleManifestCompatibilityMaximum>,
            ::std::string::String,
        >,
        minimum: ::std::result::Result<
            super::FoundryVttModuleManifestCompatibilityMinimum,
            ::std::string::String,
        >,
        verified: ::std::result::Result<
            super::FoundryVttModuleManifestCompatibilityVerified,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestCompatibility {
        fn default() -> Self {
            Self {
                maximum: Ok(Default::default()),
                minimum: Err("no value supplied for minimum".to_string()),
                verified: Err("no value supplied for verified".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifestCompatibility {
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FoundryVttModuleManifestCompatibilityMaximum,
                >,
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
                super::FoundryVttModuleManifestCompatibilityMinimum,
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
                super::FoundryVttModuleManifestCompatibilityVerified,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifestCompatibility>
    for super::FoundryVttModuleManifestCompatibility {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestCompatibility,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                maximum: value.maximum?,
                minimum: value.minimum?,
                verified: value.verified?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestCompatibility>
    for FoundryVttModuleManifestCompatibility {
        fn from(value: super::FoundryVttModuleManifestCompatibility) -> Self {
            Self {
                maximum: Ok(value.maximum),
                minimum: Ok(value.minimum),
                verified: Ok(value.verified),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestConflictsItem {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<
            ::std::option::Option<super::FoundryVttModuleManifestConflictsItemType>,
            ::std::string::String,
        >,
        version_max: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        version_min: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestConflictsItem {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                type_: Ok(Default::default()),
                version_max: Ok(Default::default()),
                version_min: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttModuleManifestConflictsItem {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttModuleManifestConflictsItemType>,
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
        pub fn version_max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_max = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_max: {}", e)
                });
            self
        }
        pub fn version_min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.version_min = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for version_min: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttModuleManifestConflictsItem>
    for super::FoundryVttModuleManifestConflictsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestConflictsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                type_: value.type_?,
                version_max: value.version_max?,
                version_min: value.version_min?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestConflictsItem>
    for FoundryVttModuleManifestConflictsItem {
        fn from(value: super::FoundryVttModuleManifestConflictsItem) -> Self {
            Self {
                id: Ok(value.id),
                type_: Ok(value.type_),
                version_max: Ok(value.version_max),
                version_min: Ok(value.version_min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestDependenciesItem {
        compatibility: ::std::result::Result<
            ::std::option::Option<
                super::FoundryVttModuleManifestDependenciesItemCompatibility,
            >,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        manifest: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::FoundryVttModuleManifestDependenciesItemType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestDependenciesItem {
        fn default() -> Self {
            Self {
                compatibility: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                manifest: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifestDependenciesItem {
        pub fn compatibility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FoundryVttModuleManifestDependenciesItemCompatibility,
                >,
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn manifest<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.manifest = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for manifest: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                super::FoundryVttModuleManifestDependenciesItemType,
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
    }
    impl ::std::convert::TryFrom<FoundryVttModuleManifestDependenciesItem>
    for super::FoundryVttModuleManifestDependenciesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestDependenciesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                compatibility: value.compatibility?,
                id: value.id?,
                manifest: value.manifest?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestDependenciesItem>
    for FoundryVttModuleManifestDependenciesItem {
        fn from(value: super::FoundryVttModuleManifestDependenciesItem) -> Self {
            Self {
                compatibility: Ok(value.compatibility),
                id: Ok(value.id),
                manifest: Ok(value.manifest),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestDependenciesItemCompatibility {
        maximum: ::std::result::Result<
            ::std::option::Option<
                super::FoundryVttModuleManifestDependenciesItemCompatibilityMaximum,
            >,
            ::std::string::String,
        >,
        minimum: ::std::result::Result<
            super::FoundryVttModuleManifestDependenciesItemCompatibilityMinimum,
            ::std::string::String,
        >,
        verified: ::std::result::Result<
            super::FoundryVttModuleManifestDependenciesItemCompatibilityVerified,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default
    for FoundryVttModuleManifestDependenciesItemCompatibility {
        fn default() -> Self {
            Self {
                maximum: Ok(Default::default()),
                minimum: Err("no value supplied for minimum".to_string()),
                verified: Err("no value supplied for verified".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifestDependenciesItemCompatibility {
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<
                    super::FoundryVttModuleManifestDependenciesItemCompatibilityMaximum,
                >,
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
                super::FoundryVttModuleManifestDependenciesItemCompatibilityMinimum,
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
                super::FoundryVttModuleManifestDependenciesItemCompatibilityVerified,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifestDependenciesItemCompatibility>
    for super::FoundryVttModuleManifestDependenciesItemCompatibility {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestDependenciesItemCompatibility,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                maximum: value.maximum?,
                minimum: value.minimum?,
                verified: value.verified?,
            })
        }
    }
    impl ::std::convert::From<
        super::FoundryVttModuleManifestDependenciesItemCompatibility,
    > for FoundryVttModuleManifestDependenciesItemCompatibility {
        fn from(
            value: super::FoundryVttModuleManifestDependenciesItemCompatibility,
        ) -> Self {
            Self {
                maximum: Ok(value.maximum),
                minimum: Ok(value.minimum),
                verified: Ok(value.verified),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestLanguagesItem {
        lang: ::std::result::Result<
            super::FoundryVttModuleManifestLanguagesItemLang,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FoundryVttModuleManifestLanguagesItem {
        fn default() -> Self {
            Self {
                lang: Err("no value supplied for lang".to_string()),
                name: Err("no value supplied for name".to_string()),
                path: Err("no value supplied for path".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifestLanguagesItem {
        pub fn lang<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FoundryVttModuleManifestLanguagesItemLang>,
            T::Error: ::std::fmt::Display,
        {
            self.lang = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for lang: {}", e));
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
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FoundryVttModuleManifestLanguagesItem>
    for super::FoundryVttModuleManifestLanguagesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestLanguagesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                lang: value.lang?,
                name: value.name?,
                path: value.path?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestLanguagesItem>
    for FoundryVttModuleManifestLanguagesItem {
        fn from(value: super::FoundryVttModuleManifestLanguagesItem) -> Self {
            Self {
                lang: Ok(value.lang),
                name: Ok(value.name),
                path: Ok(value.path),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestMediaItem {
        caption: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        loop_: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        thumbnail: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            ::std::option::Option<super::FoundryVttModuleManifestMediaItemType>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestMediaItem {
        fn default() -> Self {
            Self {
                caption: Ok(Default::default()),
                loop_: Ok(Default::default()),
                thumbnail: Ok(Default::default()),
                type_: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl FoundryVttModuleManifestMediaItem {
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
        pub fn thumbnail<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.thumbnail = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for thumbnail: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::FoundryVttModuleManifestMediaItemType>,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifestMediaItem>
    for super::FoundryVttModuleManifestMediaItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestMediaItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                caption: value.caption?,
                loop_: value.loop_?,
                thumbnail: value.thumbnail?,
                type_: value.type_?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestMediaItem>
    for FoundryVttModuleManifestMediaItem {
        fn from(value: super::FoundryVttModuleManifestMediaItem) -> Self {
            Self {
                caption: Ok(value.caption),
                loop_: Ok(value.loop_),
                thumbnail: Ok(value.thumbnail),
                type_: Ok(value.type_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FoundryVttModuleManifestPacksItem {
        flags: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        private: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        system: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::FoundryVttModuleManifestPacksItemType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryVttModuleManifestPacksItem {
        fn default() -> Self {
            Self {
                flags: Ok(Default::default()),
                label: Err("no value supplied for label".to_string()),
                name: Err("no value supplied for name".to_string()),
                path: Err("no value supplied for path".to_string()),
                private: Ok(Default::default()),
                system: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl FoundryVttModuleManifestPacksItem {
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
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn private<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.private = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for private: {}", e)
                });
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
            T: ::std::convert::TryInto<super::FoundryVttModuleManifestPacksItemType>,
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
    impl ::std::convert::TryFrom<FoundryVttModuleManifestPacksItem>
    for super::FoundryVttModuleManifestPacksItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryVttModuleManifestPacksItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                flags: value.flags?,
                label: value.label?,
                name: value.name?,
                path: value.path?,
                private: value.private?,
                system: value.system?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryVttModuleManifestPacksItem>
    for FoundryVttModuleManifestPacksItem {
        fn from(value: super::FoundryVttModuleManifestPacksItem) -> Self {
            Self {
                flags: Ok(value.flags),
                label: Ok(value.label),
                name: Ok(value.name),
                path: Ok(value.path),
                private: Ok(value.private),
                system: Ok(value.system),
                type_: Ok(value.type_),
            }
        }
    }
}
