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
///Extracted page definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Page",
///  "description": "Extracted page definition",
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "archived": {
///      "type": "boolean"
///    },
///    "background_color": {
///      "type": "string"
///    },
///    "fog_opacity": {
///      "type": "number"
///    },
///    "grid_opacity": {
///      "type": "number"
///    },
///    "grid_size": {
///      "type": "number"
///    },
///    "height": {
///      "type": "number"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "lightenforcelos": {
///      "type": "boolean"
///    },
///    "lightglobalillum": {
///      "type": "boolean"
///    },
///    "lightrestrictmovement": {
///      "type": "boolean"
///    },
///    "lightupdatedrop": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string",
///      "minLength": 1
///    },
///    "snapping_increment": {
///      "type": "number"
///    },
///    "width": {
///      "type": "number"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Page {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub archived: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fog_opacity: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grid_opacity: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grid_size: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<f64>,
    pub id: PageId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lightenforcelos: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lightglobalillum: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lightrestrictmovement: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lightupdatedrop: ::std::option::Option<bool>,
    pub name: PageName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub snapping_increment: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Page> for Page {
    fn from(value: &Page) -> Self {
        value.clone()
    }
}
impl Page {
    pub fn builder() -> builder::Page {
        Default::default()
    }
}
///`PageId`
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
pub struct PageId(::std::string::String);
impl ::std::ops::Deref for PageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PageId> for ::std::string::String {
    fn from(value: PageId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PageId> for PageId {
    fn from(value: &PageId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PageId {
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
impl ::std::convert::TryFrom<&str> for PageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageId {
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
///`PageName`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PageName(::std::string::String);
impl ::std::ops::Deref for PageName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PageName> for ::std::string::String {
    fn from(value: PageName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PageName> for PageName {
    fn from(value: &PageName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for PageName {
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
impl ::std::convert::TryFrom<&str> for PageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PageName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PageName {
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
    pub struct Page {
        archived: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        background_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        fog_opacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        grid_opacity: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        grid_size: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        height: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        id: ::std::result::Result<super::PageId, ::std::string::String>,
        lightenforcelos: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        lightglobalillum: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        lightrestrictmovement: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        lightupdatedrop: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::PageName, ::std::string::String>,
        snapping_increment: ::std::result::Result<
            ::std::option::Option<f64>,
            ::std::string::String,
        >,
        width: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Page {
        fn default() -> Self {
            Self {
                archived: Ok(Default::default()),
                background_color: Ok(Default::default()),
                fog_opacity: Ok(Default::default()),
                grid_opacity: Ok(Default::default()),
                grid_size: Ok(Default::default()),
                height: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                lightenforcelos: Ok(Default::default()),
                lightglobalillum: Ok(Default::default()),
                lightrestrictmovement: Ok(Default::default()),
                lightupdatedrop: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                snapping_increment: Ok(Default::default()),
                width: Ok(Default::default()),
            }
        }
    }
    impl Page {
        pub fn archived<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.archived = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for archived: {}", e)
                });
            self
        }
        pub fn background_color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
        pub fn fog_opacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.fog_opacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for fog_opacity: {}", e)
                });
            self
        }
        pub fn grid_opacity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.grid_opacity = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grid_opacity: {}", e)
                });
            self
        }
        pub fn grid_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.grid_size = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for grid_size: {}", e)
                });
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
            T: ::std::convert::TryInto<super::PageId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn lightenforcelos<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lightenforcelos = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for lightenforcelos: {}", e)
                });
            self
        }
        pub fn lightglobalillum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lightglobalillum = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for lightglobalillum: {}", e
                    )
                });
            self
        }
        pub fn lightrestrictmovement<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lightrestrictmovement = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for lightrestrictmovement: {}",
                        e
                    )
                });
            self
        }
        pub fn lightupdatedrop<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.lightupdatedrop = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for lightupdatedrop: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PageName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn snapping_increment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.snapping_increment = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for snapping_increment: {}", e
                    )
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
    impl ::std::convert::TryFrom<Page> for super::Page {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Page,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                archived: value.archived?,
                background_color: value.background_color?,
                fog_opacity: value.fog_opacity?,
                grid_opacity: value.grid_opacity?,
                grid_size: value.grid_size?,
                height: value.height?,
                id: value.id?,
                lightenforcelos: value.lightenforcelos?,
                lightglobalillum: value.lightglobalillum?,
                lightrestrictmovement: value.lightrestrictmovement?,
                lightupdatedrop: value.lightupdatedrop?,
                name: value.name?,
                snapping_increment: value.snapping_increment?,
                width: value.width?,
            })
        }
    }
    impl ::std::convert::From<super::Page> for Page {
        fn from(value: super::Page) -> Self {
            Self {
                archived: Ok(value.archived),
                background_color: Ok(value.background_color),
                fog_opacity: Ok(value.fog_opacity),
                grid_opacity: Ok(value.grid_opacity),
                grid_size: Ok(value.grid_size),
                height: Ok(value.height),
                id: Ok(value.id),
                lightenforcelos: Ok(value.lightenforcelos),
                lightglobalillum: Ok(value.lightglobalillum),
                lightrestrictmovement: Ok(value.lightrestrictmovement),
                lightupdatedrop: Ok(value.lightupdatedrop),
                name: Ok(value.name),
                snapping_increment: Ok(value.snapping_increment),
                width: Ok(value.width),
            }
        }
    }
}
