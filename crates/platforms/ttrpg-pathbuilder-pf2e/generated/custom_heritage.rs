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
///Extracted custom_heritage definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Custom Heritage",
///  "description": "Extracted custom_heritage definition",
///  "type": "object",
///  "required": [
///    "ancestryID",
///    "databaseID",
///    "description",
///    "id",
///    "name",
///    "src",
///    "timestamp"
///  ],
///  "properties": {
///    "ancestryID": {
///      "description": "Parent ancestry ID",
///      "type": "string"
///    },
///    "databaseID": {
///      "type": "integer"
///    },
///    "description": {
///      "description": "Heritage description and abilities",
///      "type": "string",
///      "minLength": 1
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///    },
///    "name": {
///      "description": "Heritage name",
///      "type": "string",
///      "minLength": 1
///    },
///    "src": {
///      "description": "Source book/pack",
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "string",
///      "pattern": "^\\d+$"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CustomHeritage {
    ///Parent ancestry ID
    #[serde(rename = "ancestryID")]
    pub ancestry_id: ::std::string::String,
    #[serde(rename = "databaseID")]
    pub database_id: i64,
    ///Heritage description and abilities
    pub description: CustomHeritageDescription,
    pub id: CustomHeritageId,
    ///Heritage name
    pub name: CustomHeritageName,
    ///Source book/pack
    pub src: ::std::string::String,
    pub timestamp: CustomHeritageTimestamp,
}
impl ::std::convert::From<&CustomHeritage> for CustomHeritage {
    fn from(value: &CustomHeritage) -> Self {
        value.clone()
    }
}
impl CustomHeritage {
    pub fn builder() -> builder::CustomHeritage {
        Default::default()
    }
}
///Heritage description and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Heritage description and abilities",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageDescription(::std::string::String);
impl ::std::ops::Deref for CustomHeritageDescription {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageDescription> for ::std::string::String {
    fn from(value: CustomHeritageDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageDescription> for CustomHeritageDescription {
    fn from(value: &CustomHeritageDescription) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageDescription {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageDescription {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageDescription {
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
///`CustomHeritageId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageId(::std::string::String);
impl ::std::ops::Deref for CustomHeritageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageId> for ::std::string::String {
    fn from(value: CustomHeritageId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageId> for CustomHeritageId {
    fn from(value: &CustomHeritageId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageId {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageId {
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
///Heritage name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Heritage name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageName(::std::string::String);
impl ::std::ops::Deref for CustomHeritageName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageName> for ::std::string::String {
    fn from(value: CustomHeritageName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageName> for CustomHeritageName {
    fn from(value: &CustomHeritageName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageName {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageName {
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
///`CustomHeritageTimestamp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^\\d+$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CustomHeritageTimestamp(::std::string::String);
impl ::std::ops::Deref for CustomHeritageTimestamp {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CustomHeritageTimestamp> for ::std::string::String {
    fn from(value: CustomHeritageTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomHeritageTimestamp> for CustomHeritageTimestamp {
    fn from(value: &CustomHeritageTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CustomHeritageTimestamp {
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
impl ::std::convert::TryFrom<&str> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CustomHeritageTimestamp {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CustomHeritageTimestamp {
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
    pub struct CustomHeritage {
        ancestry_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        database_id: ::std::result::Result<i64, ::std::string::String>,
        description: ::std::result::Result<
            super::CustomHeritageDescription,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::CustomHeritageId, ::std::string::String>,
        name: ::std::result::Result<super::CustomHeritageName, ::std::string::String>,
        src: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<
            super::CustomHeritageTimestamp,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CustomHeritage {
        fn default() -> Self {
            Self {
                ancestry_id: Err("no value supplied for ancestry_id".to_string()),
                database_id: Err("no value supplied for database_id".to_string()),
                description: Err("no value supplied for description".to_string()),
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                src: Err("no value supplied for src".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl CustomHeritage {
        pub fn ancestry_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ancestry_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ancestry_id: {}", e)
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
            T: ::std::convert::TryInto<super::CustomHeritageDescription>,
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
            T: ::std::convert::TryInto<super::CustomHeritageId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomHeritageName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
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
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CustomHeritageTimestamp>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for timestamp: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CustomHeritage> for super::CustomHeritage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CustomHeritage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ancestry_id: value.ancestry_id?,
                database_id: value.database_id?,
                description: value.description?,
                id: value.id?,
                name: value.name?,
                src: value.src?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::CustomHeritage> for CustomHeritage {
        fn from(value: super::CustomHeritage) -> Self {
            Self {
                ancestry_id: Ok(value.ancestry_id),
                database_id: Ok(value.database_id),
                description: Ok(value.description),
                id: Ok(value.id),
                name: Ok(value.name),
                src: Ok(value.src),
                timestamp: Ok(value.timestamp),
            }
        }
    }
}
