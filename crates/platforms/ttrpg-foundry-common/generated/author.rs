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
///Author information for Foundry VTT content
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Foundry Author",
///  "description": "Author information for Foundry VTT content",
///  "type": "object",
///  "properties": {
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
///      "description": "Author website",
///      "type": "string",
///      "format": "uri"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FoundryAuthor {
    ///Author email
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub email: ::std::option::Option<::std::string::String>,
    ///Author name
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Author website
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FoundryAuthor> for FoundryAuthor {
    fn from(value: &FoundryAuthor) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FoundryAuthor {
    fn default() -> Self {
        Self {
            email: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl FoundryAuthor {
    pub fn builder() -> builder::FoundryAuthor {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct FoundryAuthor {
        email: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FoundryAuthor {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl FoundryAuthor {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    impl ::std::convert::TryFrom<FoundryAuthor> for super::FoundryAuthor {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FoundryAuthor,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                email: value.email?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::FoundryAuthor> for FoundryAuthor {
        fn from(value: super::FoundryAuthor) -> Self {
            Self {
                email: Ok(value.email),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
}
