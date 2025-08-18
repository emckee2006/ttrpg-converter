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
///Extracted custom_item definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Item",
///  "description": "Extracted custom_item definition",
///  "type": "object",
///  "required": [
///    "databaseID",
///    "description",
///    "id",
///    "level",
///    "name",
///    "src",
///    "timestamp",
///    "traits"
///  ],
///  "properties": {
///    "ac": {
///      "description": "Armor AC bonus",
///      "type": "integer"
///    },
///    "activate": {
///      "description": "Activation method",
///      "type": "string"
///    },
///    "armorGroup": {
///      "description": "Armor group classification",
///      "type": "string"
///    },
///    "bulk": {
///      "description": "Item bulk (L, 1, 2, etc.)",
///      "type": "string"
///    },
///    "category": {
///      "description": "Item category (weapon, armor, consumable, etc.)",
///      "type": "string"
///    },
///    "checkPenalty": {
///      "description": "Armor check penalty",
///      "type": "integer"
///    },
///    "damage": {
///      "description": "Weapon damage dice",
///      "type": "string"
///    },
///    "damageType": {
///      "description": "Damage type (piercing, slashing, etc.)",
///      "type": "string"
///    },
///    "databaseID": {
///      "description": "Database identifier",
///      "type": "integer"
///    },
///    "description": {
///      "description": "Item description and abilities",
///      "type": "string",
///      "minLength": 1
///    },
///    "dexCap": {
///      "description": "Maximum Dex bonus for armor",
///      "type": "integer"
///    },
///    "hands": {
///      "description": "Number of hands required",
///      "type": "string",
///      "enum": [
///        "1",
///        "2",
///        "1+"
///      ]
///    },
///    "id": {
///      "description": "Unique item ID (UUID)",
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "level": {
///      "description": "Item level",
///      "type": "integer",
///      "maximum": 25.0,
///      "minimum": 0.0
///    },
///    "name": {
///      "description": "Item name",
///      "type": "string",
///      "minLength": 1
///    },
///    "price": {
///      "description": "Item price in gold/silver/copper",
///      "type": "string"
///    },
///    "range": {
///      "description": "Weapon range increment",
///      "type": "string"
///    },
///    "reload": {
///      "description": "Reload time for ranged weapons",
///      "type": "string"
///    },
///    "speedPenalty": {
///      "description": "Speed penalty from armor",
///      "type": "integer"
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "strength": {
///      "description": "Required Strength score",
///      "type": "integer"
///    },
///    "timestamp": {
///      "description": "Creation timestamp",
///      "type": "string",
///      "pattern": "^\\d+$"
///    },
///    "traits": {
///      "description": "Item traits (comma-separated)",
///      "type": "string"
///    },
///    "usage": {
///      "description": "Item usage/activation",
///      "type": "string"
///    },
///    "weaponGroup": {
///      "description": "Weapon group classification",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomItem {
    ///Armor AC bonus
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ac: ::std::option::Option<i64>,
    ///Activation method
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub activate: ::std::option::Option<::std::string::String>,
    ///Armor group classification
    #[serde(
        rename = "armorGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub armor_group: ::std::option::Option<::std::string::String>,
    ///Item bulk (L, 1, 2, etc.)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bulk: ::std::option::Option<::std::string::String>,
    ///Item category (weapon, armor, consumable, etc.)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub category: ::std::option::Option<::std::string::String>,
    ///Armor check penalty
    #[serde(
        rename = "checkPenalty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub check_penalty: ::std::option::Option<i64>,
    ///Weapon damage dice
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub damage: ::std::option::Option<::std::string::String>,
    ///Damage type (piercing, slashing, etc.)
    #[serde(
        rename = "damageType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub damage_type: ::std::option::Option<::std::string::String>,
    ///Database identifier
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Item description and abilities
    pub description: CustomItemDescription,
    ///Maximum Dex bonus for armor
    #[serde(
        rename = "dexCap",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dex_cap: ::std::option::Option<i64>,
    ///Number of hands required
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hands: ::std::option::Option<CustomItemHands>,
    ///Unique item ID (UUID)
    pub id: CustomItemId,
    ///Item level
    pub level: i64,
    ///Item name
    pub name: CustomItemName,
    ///Item price in gold/silver/copper
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::std::string::String>,
    ///Weapon range increment
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub range: ::std::option::Option<::std::string::String>,
    ///Reload time for ranged weapons
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reload: ::std::option::Option<::std::string::String>,
    ///Speed penalty from armor
    #[serde(
        rename = "speedPenalty",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub speed_penalty: ::std::option::Option<i64>,
    ///Source book/pack
    pub src: ::std::string::String,
    ///Required Strength score
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub strength: ::std::option::Option<i64>,
    ///Creation timestamp
    pub timestamp: CustomItemTimestamp,
    ///Item traits (comma-separated)
    pub traits: ::std::string::String,
    ///Item usage/activation
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub usage: ::std::option::Option<::std::string::String>,
    ///Weapon group classification
    #[serde(
        rename = "weaponGroup",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub weapon_group: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&CustomItem> for CustomItem {
    fn from(value: &CustomItem) -> Self {
        value.clone()
    }
}
impl CustomItem {
    pub fn builder() -> builder::CustomItem {
        Default::default()
    }
}
///Item description and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Item description and abilities",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemDescription(::std::string::String);
impl ::std::ops::Deref for CustomItemDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemDescription> for ::std::string::String {
    fn from(value: CustomItemDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemDescription> for CustomItemDescription {
    fn from(value: &CustomItemDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemDescription {
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
impl ::std::convert::TryFrom<&str> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemDescription {
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
///Number of hands required
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Number of hands required",
///  "type": "string",
///  "enum": [
///    "1",
///    "2",
///    "1+"
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
pub enum CustomItemHands {
    #[serde(rename = "1")]
    X1,
    #[serde(rename = "2")]
    X2,
    #[serde(rename = "1+")]
    X1x,
}
impl ::std::convert::From<&Self> for CustomItemHands {
    fn from(value: &CustomItemHands) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CustomItemHands {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1 => f.write_str("1"),
            Self::X2 => f.write_str("2"),
            Self::X1x => f.write_str("1+"),
        }
    }
}
impl ::std::str::FromStr for CustomItemHands {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1" => Ok(Self::X1),
            "2" => Ok(Self::X2),
            "1+" => Ok(Self::X1x),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemHands {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Unique item ID (UUID)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique item ID (UUID)",
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemId(::std::string::String);
impl ::std::ops::Deref for CustomItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemId> for ::std::string::String {
    fn from(value: CustomItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemId> for CustomItemId {
    fn from(value: &CustomItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        {
            ::regress::Regex::new(
                    "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
                )
                .unwrap()
        });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemId {
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
///Item name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Item name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemName(::std::string::String);
impl ::std::ops::Deref for CustomItemName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemName> for ::std::string::String {
    fn from(value: CustomItemName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemName> for CustomItemName {
    fn from(value: &CustomItemName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemName {
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
impl ::std::convert::TryFrom<&str> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemName {
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
///Creation timestamp
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Creation timestamp",
///  "type": "string",
///  "pattern": "^\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomItemTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomItemTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomItemTimestamp> for ::std::string::String {
    fn from(value: CustomItemTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomItemTimestamp> for CustomItemTimestamp {
    fn from(value: &CustomItemTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomItemTimestamp {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^\\d+$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^\\d+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomItemTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomItemTimestamp {
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
    pub struct CustomItem {
        ac: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        activate: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        armor_group: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        bulk: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        category: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        check_penalty: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        damage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        damage_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomItemDescription,
            ::std::string::String,
        >,
        dex_cap: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        hands: ::std::result::Result<
            ::std::option::Option<super::CustomItemHands>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomItemId, ::std::string::String>,
        level: ::std::result::Result<i64, ::std::string::String>,
        name: ::std::result::Result<super::CustomItemName, ::std::string::String>,
        price: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        range: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        reload: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        speed_penalty: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        strength: ::std::result::Result<
            ::std::option::Option<i64>,
            ::std::string::String,
        >,
        timestamp: ::std::result::Result<
            super::CustomItemTimestamp,
            ::std::string::String,
        >,
        traits: ::std::result::Result<::std::string::String, ::std::string::String>,
        usage: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        weapon_group: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomItem {
        fn default() -> Self {
            Self {
                ac: Ok(Default::default()),
                activate: Ok(Default::default()),
                armor_group: Ok(Default::default()),
                bulk: Ok(Default::default()),
                category: Ok(Default::default()),
                check_penalty: Ok(Default::default()),
                damage: Ok(Default::default()),
                damage_type: Ok(Default::default()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                dex_cap: Ok(Default::default()),
                hands: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                level: Err("no value supplied for level".to_string()),
                name: Err("no value supplied for name".to_string()),
                price: Ok(Default::default()),
                range: Ok(Default::default()),
                reload: Ok(Default::default()),
                speed_penalty: Ok(Default::default()),
                src: Err("no value supplied for src".to_string()),
                strength: Ok(Default::default()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                traits: Err("no value supplied for traits".to_string()),
                usage: Ok(Default::default()),
                weapon_group: Ok(Default::default()),
            }
        }
    }
    impl CustomItem {
        pub fn ac<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ac = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ac: {}", e));
            self
        }
        pub fn activate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.activate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for activate: {}", e)
                });
            self
        }
        pub fn armor_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.armor_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for armor_group: {}", e)
                });
            self
        }
        pub fn bulk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bulk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bulk: {}", e));
            self
        }
        pub fn category<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.category = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for category: {}", e)
                });
            self
        }
        pub fn check_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.check_penalty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for check_penalty: {}", e)
                });
            self
        }
        pub fn damage<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.damage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for damage: {}", e)
                });
            self
        }
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
        pub fn database_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.database_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for database_id: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemDescription>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn dex_cap<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.dex_cap = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dex_cap: {}", e)
                });
            self
        }
        pub fn hands<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CustomItemHands>>,
            T::Error: ::std::fmt::Display,
        {
            self.hands = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for hands: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for level: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn price<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.price = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for price: {}", e)
                });
            self
        }
        pub fn range<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reload = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for reload: {}", e)
                });
            self
        }
        pub fn speed_penalty<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.speed_penalty = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for speed_penalty: {}", e)
                });
            self
        }
        pub fn src<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.src = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for src: {}", e));
            self
        }
        pub fn strength<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.strength = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for strength: {}", e)
                });
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomItemTimestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
        pub fn traits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.usage = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for usage: {}", e)
                });
            self
        }
        pub fn weapon_group<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.weapon_group = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for weapon_group: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomItem> for super::CustomItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ac: value.ac?,
                activate: value.activate?,
                armor_group: value.armor_group?,
                bulk: value.bulk?,
                category: value.category?,
                check_penalty: value.check_penalty?,
                damage: value.damage?,
                damage_type: value.damage_type?,
                database_id: value.database_id?,
                description: value.description?,
                dex_cap: value.dex_cap?,
                hands: value.hands?,
                id: value.id?,
                level: value.level?,
                name: value.name?,
                price: value.price?,
                range: value.range?,
                reload: value.reload?,
                speed_penalty: value.speed_penalty?,
                src: value.src?,
                strength: value.strength?,
                timestamp: value.timestamp?,
                traits: value.traits?,
                usage: value.usage?,
                weapon_group: value.weapon_group?,
            })
        }
    }
    impl ::std::convert::From<super::CustomItem> for CustomItem {
        fn from(value: super::CustomItem) -> Self {
            Self {
                ac: Ok(value.ac),
                activate: Ok(value.activate),
                armor_group: Ok(value.armor_group),
                bulk: Ok(value.bulk),
                category: Ok(value.category),
                check_penalty: Ok(value.check_penalty),
                damage: Ok(value.damage),
                damage_type: Ok(value.damage_type),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                dex_cap: Ok(value.dex_cap),
                hands: Ok(value.hands),
                id: Ok(value.id),
                level: Ok(value.level),
                name: Ok(value.name),
                price: Ok(value.price),
                range: Ok(value.range),
                reload: Ok(value.reload),
                speed_penalty: Ok(value.speed_penalty),
                src: Ok(value.src),
                strength: Ok(value.strength),
                timestamp: Ok(value.timestamp),
                traits: Ok(value.traits),
                usage: Ok(value.usage),
                weapon_group: Ok(value.weapon_group),
            }
        }
    }
}
