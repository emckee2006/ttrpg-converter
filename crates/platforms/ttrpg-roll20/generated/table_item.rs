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
///Extracted table_item definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Table Item",
///  "description": "Extracted table_item definition",
///  "type": "object",
///  "required": [
///    "name",
///    "weight"
///  ],
///  "properties": {
///    "avatar": {
///      "description": "Optional image URL for this result",
///      "type": "string",
///      "format": "uri"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "name": {
///      "description": "Table item name/result text",
///      "type": "string",
///      "minLength": 1
///    },
///    "weight": {
///      "description": "Relative weight for random selection",
///      "type": "number",
///      "minimum": 1.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TableItem {
    ///Optional image URL for this result
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<TableItemId>,
    ///Table item name/result text
    pub name: TableItemName,
    pub weight: f64,
}
impl ::std::convert::From<&TableItem> for TableItem {
    fn from(value: &TableItem) -> Self {
        value.clone()
    }
}
impl TableItem {
    pub fn builder() -> builder::TableItem {
        Default::default()
    }
}
///`TableItemId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^-[A-Za-z0-9_-]{19}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TableItemId(::std::string::String);
impl ::std::ops::Deref for TableItemId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TableItemId> for ::std::string::String {
    fn from(value: TableItemId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TableItemId> for TableItemId {
    fn from(value: &TableItemId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TableItemId {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^-[A-Za-z0-9_-]{19}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^-[A-Za-z0-9_-]{19}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TableItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TableItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TableItemId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TableItemId {
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
///Table item name/result text
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Table item name/result text",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TableItemName(::std::string::String);
impl ::std::ops::Deref for TableItemName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TableItemName> for ::std::string::String {
    fn from(value: TableItemName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TableItemName> for TableItemName {
    fn from(value: &TableItemName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TableItemName {
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
impl ::std::convert::TryFrom<&str> for TableItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TableItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TableItemName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TableItemName {
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
    pub struct TableItem {
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::TableItemId>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::TableItemName, ::std::string::String>,
        weight: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for TableItem {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                weight: Err("no value supplied for weight".to_string()),
            }
        }
    }
    impl TableItem {
        pub fn avatar<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.avatar = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for avatar: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TableItemId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TableItemName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
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
    impl ::std::convert::TryFrom<TableItem> for super::TableItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TableItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                id: value.id?,
                name: value.name?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::TableItem> for TableItem {
        fn from(value: super::TableItem) -> Self {
            Self {
                avatar: Ok(value.avatar),
                id: Ok(value.id),
                name: Ok(value.name),
                weight: Ok(value.weight),
            }
        }
    }
}
