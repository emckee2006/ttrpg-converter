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
///Currency amounts
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Currency",
///  "description": "Currency amounts",
///  "type": "object",
///  "properties": {
///    "cp": {
///      "description": "Copper pieces",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "ep": {
///      "description": "Electrum pieces",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "gp": {
///      "description": "Gold pieces",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "pp": {
///      "description": "Platinum pieces",
///      "type": "number",
///      "minimum": 0.0
///    },
///    "sp": {
///      "description": "Silver pieces",
///      "type": "number",
///      "minimum": 0.0
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Currency {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ep: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pp: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sp: ::std::option::Option<f64>,
}
impl ::std::convert::From<&Currency> for Currency {
    fn from(value: &Currency) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Currency {
    fn default() -> Self {
        Self {
            cp: Default::default(),
            ep: Default::default(),
            gp: Default::default(),
            pp: Default::default(),
            sp: Default::default(),
        }
    }
}
impl Currency {
    pub fn builder() -> builder::Currency {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Currency {
        cp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        ep: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        gp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        pp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        sp: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for Currency {
        fn default() -> Self {
            Self {
                cp: Ok(Default::default()),
                ep: Ok(Default::default()),
                gp: Ok(Default::default()),
                pp: Ok(Default::default()),
                sp: Ok(Default::default()),
            }
        }
    }
    impl Currency {
        pub fn cp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.cp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cp: {}", e));
            self
        }
        pub fn ep<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.ep = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ep: {}", e));
            self
        }
        pub fn gp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.gp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gp: {}", e));
            self
        }
        pub fn pp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.pp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pp: {}", e));
            self
        }
        pub fn sp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.sp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Currency> for super::Currency {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Currency,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cp: value.cp?,
                ep: value.ep?,
                gp: value.gp?,
                pp: value.pp?,
                sp: value.sp?,
            })
        }
    }
    impl ::std::convert::From<super::Currency> for Currency {
        fn from(value: super::Currency) -> Self {
            Self {
                cp: Ok(value.cp),
                ep: Ok(value.ep),
                gp: Ok(value.gp),
                pp: Ok(value.pp),
                sp: Ok(value.sp),
            }
        }
    }
}
