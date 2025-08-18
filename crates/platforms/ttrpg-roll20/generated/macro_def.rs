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
///Extracted macro definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Macro",
///  "description": "Extracted macro definition",
///  "type": "object",
///  "required": [
///    "action",
///    "name"
///  ],
///  "properties": {
///    "action": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "istokenaction": {
///      "type": "boolean"
///    },
///    "name": {
///      "type": "string"
///    },
///    "visibleto": {
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Macro {
    pub action: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub istokenaction: ::std::option::Option<bool>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub visibleto: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Macro> for Macro {
    fn from(value: &Macro) -> Self {
        value.clone()
    }
}
impl Macro {
    pub fn builder() -> builder::Macro {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Macro {
        action: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        istokenaction: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        visibleto: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Macro {
        fn default() -> Self {
            Self {
                action: Err("no value supplied for action".to_string()),
                id: Ok(Default::default()),
                istokenaction: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                visibleto: Ok(Default::default()),
            }
        }
    }
    impl Macro {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for action: {}", e)
                });
            self
        }
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
        pub fn istokenaction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.istokenaction = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for istokenaction: {}", e)
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
        pub fn visibleto<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.visibleto = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for visibleto: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Macro> for super::Macro {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Macro,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                id: value.id?,
                istokenaction: value.istokenaction?,
                name: value.name?,
                visibleto: value.visibleto?,
            })
        }
    }
    impl ::std::convert::From<super::Macro> for Macro {
        fn from(value: super::Macro) -> Self {
            Self {
                action: Ok(value.action),
                id: Ok(value.id),
                istokenaction: Ok(value.istokenaction),
                name: Ok(value.name),
                visibleto: Ok(value.visibleto),
            }
        }
    }
}
