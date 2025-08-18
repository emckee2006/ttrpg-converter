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
///Extracted handout definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Handout",
///  "description": "Extracted handout definition",
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "archived": {
///      "type": "boolean"
///    },
///    "gmnotes": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "inplayerjournals": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string",
///      "minLength": 1
///    },
///    "notes": {
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Handout {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub archived: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gmnotes: ::std::option::Option<::std::string::String>,
    pub id: HandoutId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inplayerjournals: ::std::option::Option<::std::string::String>,
    pub name: HandoutName,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Handout> for Handout {
    fn from(value: &Handout) -> Self {
        value.clone()
    }
}
impl Handout {
    pub fn builder() -> builder::Handout {
        Default::default()
    }
}
///`HandoutId`
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
pub struct HandoutId(::std::string::String);
impl ::std::ops::Deref for HandoutId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<HandoutId> for ::std::string::String {
    fn from(value: HandoutId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HandoutId> for HandoutId {
    fn from(value: &HandoutId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for HandoutId {
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
impl ::std::convert::TryFrom<&str> for HandoutId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HandoutId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HandoutId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for HandoutId {
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
///`HandoutName`
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
pub struct HandoutName(::std::string::String);
impl ::std::ops::Deref for HandoutName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<HandoutName> for ::std::string::String {
    fn from(value: HandoutName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HandoutName> for HandoutName {
    fn from(value: &HandoutName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for HandoutName {
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
impl ::std::convert::TryFrom<&str> for HandoutName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for HandoutName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for HandoutName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for HandoutName {
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
    pub struct Handout {
        archived: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        gmnotes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::HandoutId, ::std::string::String>,
        inplayerjournals: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<super::HandoutName, ::std::string::String>,
        notes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Handout {
        fn default() -> Self {
            Self {
                archived: Ok(Default::default()),
                gmnotes: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                inplayerjournals: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                notes: Ok(Default::default()),
            }
        }
    }
    impl Handout {
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
        pub fn gmnotes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.gmnotes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for gmnotes: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HandoutId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn inplayerjournals<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.inplayerjournals = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for inplayerjournals: {}", e
                    )
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::HandoutName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn notes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.notes = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for notes: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Handout> for super::Handout {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Handout,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                archived: value.archived?,
                gmnotes: value.gmnotes?,
                id: value.id?,
                inplayerjournals: value.inplayerjournals?,
                name: value.name?,
                notes: value.notes?,
            })
        }
    }
    impl ::std::convert::From<super::Handout> for Handout {
        fn from(value: super::Handout) -> Self {
            Self {
                archived: Ok(value.archived),
                gmnotes: Ok(value.gmnotes),
                id: Ok(value.id),
                inplayerjournals: Ok(value.inplayerjournals),
                name: Ok(value.name),
                notes: Ok(value.notes),
            }
        }
    }
}
