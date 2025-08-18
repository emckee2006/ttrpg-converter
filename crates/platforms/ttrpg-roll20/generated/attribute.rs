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
///Character sheet attribute (e.g., strength, hp)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Attribute",
///  "description": "Character sheet attribute (e.g., strength, hp)",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "current": {
///      "description": "Current value as string",
///      "type": "string"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "max": {
///      "description": "Maximum value as string",
///      "type": "string"
///    },
///    "name": {
///      "description": "Attribute name (e.g., 'strength', 'hp')",
///      "type": "string",
///      "minLength": 1
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterAttribute {
    ///Current value as string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub current: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Roll20CharacterAttributeId>,
    ///Maximum value as string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<::std::string::String>,
    ///Attribute name (e.g., 'strength', 'hp')
    pub name: Roll20CharacterAttributeName,
}
impl ::std::convert::From<&Roll20CharacterAttribute> for Roll20CharacterAttribute {
    fn from(value: &Roll20CharacterAttribute) -> Self {
        value.clone()
    }
}
impl Roll20CharacterAttribute {
    pub fn builder() -> builder::Roll20CharacterAttribute {
        Default::default()
    }
}
///`Roll20CharacterAttributeId`
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
pub struct Roll20CharacterAttributeId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAttributeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAttributeId> for ::std::string::String {
    fn from(value: Roll20CharacterAttributeId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAttributeId> for Roll20CharacterAttributeId {
    fn from(value: &Roll20CharacterAttributeId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAttributeId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAttributeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAttributeId {
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
///Attribute name (e.g., 'strength', 'hp')
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Attribute name (e.g., 'strength', 'hp')",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterAttributeName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAttributeName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAttributeName> for ::std::string::String {
    fn from(value: Roll20CharacterAttributeName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAttributeName>
for Roll20CharacterAttributeName {
    fn from(value: &Roll20CharacterAttributeName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAttributeName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAttributeName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAttributeName {
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
    pub struct Roll20CharacterAttribute {
        current: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::Roll20CharacterAttributeId>,
            ::std::string::String,
        >,
        max: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::Roll20CharacterAttributeName,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterAttribute {
        fn default() -> Self {
            Self {
                current: Ok(Default::default()),
                id: Ok(Default::default()),
                max: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Roll20CharacterAttribute {
        pub fn current<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.current = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for current: {}", e)
                });
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::Roll20CharacterAttributeId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Roll20CharacterAttributeName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterAttribute>
    for super::Roll20CharacterAttribute {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterAttribute,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current: value.current?,
                id: value.id?,
                max: value.max?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterAttribute>
    for Roll20CharacterAttribute {
        fn from(value: super::Roll20CharacterAttribute) -> Self {
            Self {
                current: Ok(value.current),
                id: Ok(value.id),
                max: Ok(value.max),
                name: Ok(value.name),
            }
        }
    }
}
