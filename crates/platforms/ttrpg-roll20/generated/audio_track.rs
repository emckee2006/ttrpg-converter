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
///Extracted audio_track definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Audio Track",
///  "description": "Extracted audio_track definition",
///  "type": "object",
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "loop": {
///      "type": "boolean"
///    },
///    "playing": {
///      "type": "boolean"
///    },
///    "title": {
///      "type": "string"
///    },
///    "track_id": {
///      "type": "string"
///    },
///    "volume": {
///      "type": "number"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AudioTrack {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "loop",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub loop_: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playing: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub track_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub volume: ::std::option::Option<f64>,
}
impl ::std::convert::From<&AudioTrack> for AudioTrack {
    fn from(value: &AudioTrack) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AudioTrack {
    fn default() -> Self {
        Self {
            id: Default::default(),
            loop_: Default::default(),
            playing: Default::default(),
            title: Default::default(),
            track_id: Default::default(),
            volume: Default::default(),
        }
    }
}
impl AudioTrack {
    pub fn builder() -> builder::AudioTrack {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AudioTrack {
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        loop_: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        playing: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        track_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for AudioTrack {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                loop_: Ok(Default::default()),
                playing: Ok(Default::default()),
                title: Ok(Default::default()),
                track_id: Ok(Default::default()),
                volume: Ok(Default::default()),
            }
        }
    }
    impl AudioTrack {
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
        pub fn loop_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.loop_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for loop_: {}", e)
                });
            self
        }
        pub fn playing<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.playing = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playing: {}", e)
                });
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for title: {}", e)
                });
            self
        }
        pub fn track_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.track_id = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for track_id: {}", e)
                });
            self
        }
        pub fn volume<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.volume = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for volume: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AudioTrack> for super::AudioTrack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AudioTrack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                loop_: value.loop_?,
                playing: value.playing?,
                title: value.title?,
                track_id: value.track_id?,
                volume: value.volume?,
            })
        }
    }
    impl ::std::convert::From<super::AudioTrack> for AudioTrack {
        fn from(value: super::AudioTrack) -> Self {
            Self {
                id: Ok(value.id),
                loop_: Ok(value.loop_),
                playing: Ok(value.playing),
                title: Ok(value.title),
                track_id: Ok(value.track_id),
                volume: Ok(value.volume),
            }
        }
    }
}
