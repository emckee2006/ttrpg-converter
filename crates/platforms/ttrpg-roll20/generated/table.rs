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
///Extracted table definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Table",
///  "description": "Extracted table definition",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "showplayers": {
///      "type": "boolean"
///    },
///    "tableitems": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "properties": {
///          "avatar": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "weight": {
///            "type": "number"
///          }
///        }
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Table {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub showplayers: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tableitems: ::std::vec::Vec<TableTableitemsItem>,
}
impl ::std::convert::From<&Table> for Table {
    fn from(value: &Table) -> Self {
        value.clone()
    }
}
impl Table {
    pub fn builder() -> builder::Table {
        Default::default()
    }
}
///`TableTableitemsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "avatar": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "weight": {
///      "type": "number"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TableTableitemsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub weight: ::std::option::Option<f64>,
}
impl ::std::convert::From<&TableTableitemsItem> for TableTableitemsItem {
    fn from(value: &TableTableitemsItem) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TableTableitemsItem {
    fn default() -> Self {
        Self {
            avatar: Default::default(),
            name: Default::default(),
            weight: Default::default(),
        }
    }
}
impl TableTableitemsItem {
    pub fn builder() -> builder::TableTableitemsItem {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Table {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        showplayers: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        tableitems: ::std::result::Result<
            ::std::vec::Vec<super::TableTableitemsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Table {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                showplayers: Ok(Default::default()),
                tableitems: Ok(Default::default()),
            }
        }
    }
    impl Table {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
        pub fn showplayers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.showplayers = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for showplayers: {}", e)
                });
            self
        }
        pub fn tableitems<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TableTableitemsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.tableitems = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tableitems: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Table> for super::Table {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Table,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                showplayers: value.showplayers?,
                tableitems: value.tableitems?,
            })
        }
    }
    impl ::std::convert::From<super::Table> for Table {
        fn from(value: super::Table) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                showplayers: Ok(value.showplayers),
                tableitems: Ok(value.tableitems),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TableTableitemsItem {
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        weight: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for TableTableitemsItem {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                name: Ok(Default::default()),
                weight: Ok(Default::default()),
            }
        }
    }
    impl TableTableitemsItem {
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
        pub fn weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
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
    impl ::std::convert::TryFrom<TableTableitemsItem> for super::TableTableitemsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TableTableitemsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                name: value.name?,
                weight: value.weight?,
            })
        }
    }
    impl ::std::convert::From<super::TableTableitemsItem> for TableTableitemsItem {
        fn from(value: super::TableTableitemsItem) -> Self {
            Self {
                avatar: Ok(value.avatar),
                name: Ok(value.name),
                weight: Ok(value.weight),
            }
        }
    }
}
