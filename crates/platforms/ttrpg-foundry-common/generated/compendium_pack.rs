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
/// Types for composing complex structures.
pub mod builder {
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
}
