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
///Extracted deck definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Deck",
///  "description": "Extracted deck definition",
///  "type": "object",
///  "properties": {
///    "avatar": {
///      "type": "string"
///    },
///    "cards": {
///      "type": "array"
///    },
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "playerscandraw": {
///      "type": "boolean"
///    },
///    "shown": {
///      "type": "boolean"
///    },
///    "showplayers": {
///      "type": "boolean"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Deck {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub cards: ::std::vec::Vec<::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub playerscandraw: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shown: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub showplayers: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Deck> for Deck {
    fn from(value: &Deck) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Deck {
    fn default() -> Self {
        Self {
            avatar: Default::default(),
            cards: Default::default(),
            id: Default::default(),
            name: Default::default(),
            playerscandraw: Default::default(),
            shown: Default::default(),
            showplayers: Default::default(),
        }
    }
}
impl Deck {
    pub fn builder() -> builder::Deck {
        Default::default()
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
///Extracted pdf definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "PDF",
///  "description": "Extracted pdf definition",
///  "type": "object",
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string",
///      "format": "uri"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Pdf {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Pdf> for Pdf {
    fn from(value: &Pdf) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Pdf {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            url: Default::default(),
        }
    }
}
impl Pdf {
    pub fn builder() -> builder::Pdf {
        Default::default()
    }
}
///Extracted player definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Player",
///  "description": "Extracted player definition",
///  "type": "object",
///  "properties": {
///    "color": {
///      "type": "string"
///    },
///    "displayname": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "showname": {
///      "type": "boolean"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Player {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub displayname: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub showname: ::std::option::Option<bool>,
}
impl ::std::convert::From<&Player> for Player {
    fn from(value: &Player) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Player {
    fn default() -> Self {
        Self {
            color: Default::default(),
            displayname: Default::default(),
            id: Default::default(),
            showname: Default::default(),
        }
    }
}
impl Player {
    pub fn builder() -> builder::Player {
        Default::default()
    }
}
///Schema for Roll20 campaign export format (based on R20Exporter)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Campaign Export",
///  "description": "Schema for Roll20 campaign export format (based on R20Exporter)",
///  "type": "object",
///  "required": [
///    "characters",
///    "handouts",
///    "pages"
///  ],
///  "properties": {
///    "characters": {
///      "description": "Character sheets with attributes and abilities",
///      "type": "array",
///      "items": {
///        "title": "Roll20 Character Sheet",
///        "description": "Schema for Roll20 character sheet with attributes and abilities",
///        "type": "object",
///        "required": [
///          "id",
///          "name"
///        ],
///        "properties": {
///          "abilities": {
///            "description": "Character sheet abilities/macros",
///            "type": "array",
///            "items": {
///              "title": "Roll20 Character Ability",
///              "description": "Character sheet ability/macro",
///              "type": "object",
///              "required": [
///                "name"
///              ],
///              "properties": {
///                "action": {
///                  "description": "Macro commands/actions",
///                  "type": "string"
///                },
///                "description": {
///                  "description": "Ability description",
///                  "type": "string"
///                },
///                "id": {
///                  "type": "string",
///                  "pattern": "^-[A-Za-z0-9_-]{19}$"
///                },
///                "istokenaction": {
///                  "description": "Whether this appears as token action",
///                  "default": false,
///                  "type": "boolean"
///                },
///                "name": {
///                  "description": "Ability name",
///                  "type": "string",
///                  "minLength": 1
///                }
///              },
///              "$schema": "https://json-schema.org/draft-07/schema#"
///            }
///          },
///          "archived": {
///            "description": "Whether character is archived",
///            "default": false,
///            "type": "boolean"
///          },
///          "attribs": {
///            "description": "Character sheet attributes",
///            "type": "array",
///            "items": {
///              "title": "Roll20 Character Attribute",
///              "description": "Character sheet attribute (e.g., strength, hp)",
///              "type": "object",
///              "required": [
///                "name"
///              ],
///              "properties": {
///                "current": {
///                  "description": "Current value as string",
///                  "type": "string"
///                },
///                "id": {
///                  "type": "string",
///                  "pattern": "^-[A-Za-z0-9_-]{19}$"
///                },
///                "max": {
///                  "description": "Maximum value as string",
///                  "type": "string"
///                },
///                "name": {
///                  "description": "Attribute name (e.g., 'strength', 'hp')",
///                  "type": "string",
///                  "minLength": 1
///                }
///              },
///              "$schema": "https://json-schema.org/draft-07/schema#"
///            }
///          },
///          "avatar": {
///            "description": "Character avatar image URL",
///            "type": "string",
///            "format": "uri"
///          },
///          "bio": {
///            "description": "Character biography/notes",
///            "type": "string"
///          },
///          "controlledby": {
///            "description": "Comma-separated player IDs who can control this character",
///            "type": "string"
///          },
///          "defaulttoken": {
///            "description": "Default token settings JSON",
///            "type": "string"
///          },
///          "gmnotes": {
///            "description": "GM-only notes",
///            "type": "string"
///          },
///          "id": {
///            "description": "Unique Roll20 character ID",
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "inplayerjournals": {
///            "description": "Comma-separated player IDs who can see this character",
///            "type": "string"
///          },
///          "name": {
///            "description": "Character name",
///            "type": "string",
///            "minLength": 1
///          },
///          "tags": {
///            "description": "Character tags",
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "decks": {
///      "description": "Card decks",
///      "type": "array",
///      "items": {
///        "title": "Deck",
///        "description": "Extracted deck definition",
///        "type": "object",
///        "properties": {
///          "avatar": {
///            "type": "string"
///          },
///          "cards": {
///            "type": "array"
///          },
///          "id": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "playerscandraw": {
///            "type": "boolean"
///          },
///          "shown": {
///            "type": "boolean"
///          },
///          "showplayers": {
///            "type": "boolean"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "handouts": {
///      "description": "Journal entries and handouts",
///      "type": "array",
///      "items": {
///        "title": "Handout",
///        "description": "Extracted handout definition",
///        "type": "object",
///        "required": [
///          "id",
///          "name"
///        ],
///        "properties": {
///          "archived": {
///            "type": "boolean"
///          },
///          "gmnotes": {
///            "type": "string"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "inplayerjournals": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string",
///            "minLength": 1
///          },
///          "notes": {
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "journalfolder": {
///      "description": "Journal folder organization",
///      "type": "array"
///    },
///    "jukebox": {
///      "description": "Audio playlists",
///      "type": "array",
///      "items": {
///        "title": "Audio Track",
///        "description": "Extracted audio_track definition",
///        "type": "object",
///        "properties": {
///          "id": {
///            "type": "string"
///          },
///          "loop": {
///            "type": "boolean"
///          },
///          "playing": {
///            "type": "boolean"
///          },
///          "title": {
///            "type": "string"
///          },
///          "track_id": {
///            "type": "string"
///          },
///          "volume": {
///            "type": "number"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "jukeboxfolder": {
///      "description": "Audio folder organization",
///      "type": "array"
///    },
///    "macros": {
///      "description": "Campaign macros",
///      "type": "array",
///      "items": {
///        "title": "Macro",
///        "description": "Extracted macro definition",
///        "type": "object",
///        "required": [
///          "action",
///          "name"
///        ],
///        "properties": {
///          "action": {
///            "type": "string"
///          },
///          "id": {
///            "type": "string"
///          },
///          "istokenaction": {
///            "type": "boolean"
///          },
///          "name": {
///            "type": "string"
///          },
///          "visibleto": {
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "pages": {
///      "description": "Maps/scenes with tokens, paths, lighting",
///      "type": "array",
///      "items": {
///        "title": "Page",
///        "description": "Extracted page definition",
///        "type": "object",
///        "required": [
///          "id",
///          "name"
///        ],
///        "properties": {
///          "archived": {
///            "type": "boolean"
///          },
///          "background_color": {
///            "type": "string"
///          },
///          "fog_opacity": {
///            "type": "number"
///          },
///          "grid_opacity": {
///            "type": "number"
///          },
///          "grid_size": {
///            "type": "number"
///          },
///          "height": {
///            "type": "number"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "lightenforcelos": {
///            "type": "boolean"
///          },
///          "lightglobalillum": {
///            "type": "boolean"
///          },
///          "lightrestrictmovement": {
///            "type": "boolean"
///          },
///          "lightupdatedrop": {
///            "type": "boolean"
///          },
///          "name": {
///            "type": "string",
///            "minLength": 1
///          },
///          "snapping_increment": {
///            "type": "number"
///          },
///          "width": {
///            "type": "number"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "pdfs": {
///      "description": "PDF attachments",
///      "type": "array",
///      "items": {
///        "title": "PDF",
///        "description": "Extracted pdf definition",
///        "type": "object",
///        "properties": {
///          "id": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string",
///            "format": "uri"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "players": {
///      "description": "Campaign players",
///      "type": "array",
///      "items": {
///        "title": "Player",
///        "description": "Extracted player definition",
///        "type": "object",
///        "properties": {
///          "color": {
///            "type": "string"
///          },
///          "displayname": {
///            "type": "string"
///          },
///          "id": {
///            "type": "string"
///          },
///          "showname": {
///            "type": "boolean"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "tables": {
///      "description": "Random tables (rollabletables)",
///      "type": "array",
///      "items": {
///        "title": "Table",
///        "description": "Extracted table definition",
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "id": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "showplayers": {
///            "type": "boolean"
///          },
///          "tableitems": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "properties": {
///                "avatar": {
///                  "type": "string"
///                },
///                "name": {
///                  "type": "string"
///                },
///                "weight": {
///                  "type": "number"
///                }
///              }
///            }
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "turnorder": {
///      "description": "Combat turn order",
///      "type": "array",
///      "items": {
///        "title": "Turn Entry",
///        "description": "Extracted turn_entry definition",
///        "type": "object",
///        "properties": {
///          "_pageid": {
///            "type": "string"
///          },
///          "custom": {
///            "type": "string"
///          },
///          "id": {
///            "type": "string"
///          },
///          "pr": {
///            "type": "string"
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CampaignExport {
    ///Character sheets with attributes and abilities
    pub characters: ::std::vec::Vec<Roll20CharacterSheet>,
    ///Card decks
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub decks: ::std::vec::Vec<Deck>,
    ///Journal entries and handouts
    pub handouts: ::std::vec::Vec<Handout>,
    ///Journal folder organization
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub journalfolder: ::std::vec::Vec<::serde_json::Value>,
    ///Audio playlists
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub jukebox: ::std::vec::Vec<AudioTrack>,
    ///Audio folder organization
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub jukeboxfolder: ::std::vec::Vec<::serde_json::Value>,
    ///Campaign macros
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub macros: ::std::vec::Vec<Macro>,
    ///Maps/scenes with tokens, paths, lighting
    pub pages: ::std::vec::Vec<Page>,
    ///PDF attachments
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub pdfs: ::std::vec::Vec<Pdf>,
    ///Campaign players
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub players: ::std::vec::Vec<Player>,
    ///Random tables (rollabletables)
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub tables: ::std::vec::Vec<Table>,
    ///Combat turn order
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub turnorder: ::std::vec::Vec<TurnEntry>,
}
impl ::std::convert::From<&Roll20CampaignExport> for Roll20CampaignExport {
    fn from(value: &Roll20CampaignExport) -> Self {
        value.clone()
    }
}
impl Roll20CampaignExport {
    pub fn builder() -> builder::Roll20CampaignExport {
        Default::default()
    }
}
///Character sheet ability/macro
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Ability",
///  "description": "Character sheet ability/macro",
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "action": {
///      "description": "Macro commands/actions",
///      "type": "string"
///    },
///    "description": {
///      "description": "Ability description",
///      "type": "string"
///    },
///    "id": {
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "istokenaction": {
///      "description": "Whether this appears as token action",
///      "default": false,
///      "type": "boolean"
///    },
///    "name": {
///      "description": "Ability name",
///      "type": "string",
///      "minLength": 1
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterAbility {
    ///Macro commands/actions
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action: ::std::option::Option<::std::string::String>,
    ///Ability description
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Roll20CharacterAbilityId>,
    ///Whether this appears as token action
    #[serde(default)]
    pub istokenaction: bool,
    ///Ability name
    pub name: Roll20CharacterAbilityName,
}
impl ::std::convert::From<&Roll20CharacterAbility> for Roll20CharacterAbility {
    fn from(value: &Roll20CharacterAbility) -> Self {
        value.clone()
    }
}
impl Roll20CharacterAbility {
    pub fn builder() -> builder::Roll20CharacterAbility {
        Default::default()
    }
}
///`Roll20CharacterAbilityId`
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
pub struct Roll20CharacterAbilityId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAbilityId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAbilityId> for ::std::string::String {
    fn from(value: Roll20CharacterAbilityId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAbilityId> for Roll20CharacterAbilityId {
    fn from(value: &Roll20CharacterAbilityId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAbilityId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAbilityId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAbilityId {
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
///Ability name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Ability name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterAbilityName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterAbilityName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterAbilityName> for ::std::string::String {
    fn from(value: Roll20CharacterAbilityName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterAbilityName> for Roll20CharacterAbilityName {
    fn from(value: &Roll20CharacterAbilityName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterAbilityName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterAbilityName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterAbilityName {
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
///Schema for Roll20 character sheet with attributes and abilities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Roll20 Character Sheet",
///  "description": "Schema for Roll20 character sheet with attributes and abilities",
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "abilities": {
///      "description": "Character sheet abilities/macros",
///      "type": "array",
///      "items": {
///        "title": "Roll20 Character Ability",
///        "description": "Character sheet ability/macro",
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "action": {
///            "description": "Macro commands/actions",
///            "type": "string"
///          },
///          "description": {
///            "description": "Ability description",
///            "type": "string"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "istokenaction": {
///            "description": "Whether this appears as token action",
///            "default": false,
///            "type": "boolean"
///          },
///          "name": {
///            "description": "Ability name",
///            "type": "string",
///            "minLength": 1
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "archived": {
///      "description": "Whether character is archived",
///      "default": false,
///      "type": "boolean"
///    },
///    "attribs": {
///      "description": "Character sheet attributes",
///      "type": "array",
///      "items": {
///        "title": "Roll20 Character Attribute",
///        "description": "Character sheet attribute (e.g., strength, hp)",
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "current": {
///            "description": "Current value as string",
///            "type": "string"
///          },
///          "id": {
///            "type": "string",
///            "pattern": "^-[A-Za-z0-9_-]{19}$"
///          },
///          "max": {
///            "description": "Maximum value as string",
///            "type": "string"
///          },
///          "name": {
///            "description": "Attribute name (e.g., 'strength', 'hp')",
///            "type": "string",
///            "minLength": 1
///          }
///        },
///        "$schema": "https://json-schema.org/draft-07/schema#"
///      }
///    },
///    "avatar": {
///      "description": "Character avatar image URL",
///      "type": "string",
///      "format": "uri"
///    },
///    "bio": {
///      "description": "Character biography/notes",
///      "type": "string"
///    },
///    "controlledby": {
///      "description": "Comma-separated player IDs who can control this character",
///      "type": "string"
///    },
///    "defaulttoken": {
///      "description": "Default token settings JSON",
///      "type": "string"
///    },
///    "gmnotes": {
///      "description": "GM-only notes",
///      "type": "string"
///    },
///    "id": {
///      "description": "Unique Roll20 character ID",
///      "type": "string",
///      "pattern": "^-[A-Za-z0-9_-]{19}$"
///    },
///    "inplayerjournals": {
///      "description": "Comma-separated player IDs who can see this character",
///      "type": "string"
///    },
///    "name": {
///      "description": "Character name",
///      "type": "string",
///      "minLength": 1
///    },
///    "tags": {
///      "description": "Character tags",
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Roll20CharacterSheet {
    ///Character sheet abilities/macros
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub abilities: ::std::vec::Vec<Roll20CharacterAbility>,
    ///Whether character is archived
    #[serde(default)]
    pub archived: bool,
    ///Character sheet attributes
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub attribs: ::std::vec::Vec<Roll20CharacterAttribute>,
    ///Character avatar image URL
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub avatar: ::std::option::Option<::std::string::String>,
    ///Character biography/notes
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bio: ::std::option::Option<::std::string::String>,
    ///Comma-separated player IDs who can control this character
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub controlledby: ::std::option::Option<::std::string::String>,
    ///Default token settings JSON
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub defaulttoken: ::std::option::Option<::std::string::String>,
    ///GM-only notes
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gmnotes: ::std::option::Option<::std::string::String>,
    ///Unique Roll20 character ID
    pub id: Roll20CharacterSheetId,
    ///Comma-separated player IDs who can see this character
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inplayerjournals: ::std::option::Option<::std::string::String>,
    ///Character name
    pub name: Roll20CharacterSheetName,
    ///Character tags
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tags: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Roll20CharacterSheet> for Roll20CharacterSheet {
    fn from(value: &Roll20CharacterSheet) -> Self {
        value.clone()
    }
}
impl Roll20CharacterSheet {
    pub fn builder() -> builder::Roll20CharacterSheet {
        Default::default()
    }
}
///Unique Roll20 character ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Unique Roll20 character ID",
///  "type": "string",
///  "pattern": "^-[A-Za-z0-9_-]{19}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterSheetId(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterSheetId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterSheetId> for ::std::string::String {
    fn from(value: Roll20CharacterSheetId) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterSheetId> for Roll20CharacterSheetId {
    fn from(value: &Roll20CharacterSheetId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterSheetId {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterSheetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterSheetId {
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
///Character name
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Character name",
///  "type": "string",
///  "minLength": 1
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Roll20CharacterSheetName(::std::string::String);
impl ::std::ops::Deref for Roll20CharacterSheetName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Roll20CharacterSheetName> for ::std::string::String {
    fn from(value: Roll20CharacterSheetName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Roll20CharacterSheetName> for Roll20CharacterSheetName {
    fn from(value: &Roll20CharacterSheetName) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for Roll20CharacterSheetName {
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
impl ::std::convert::TryFrom<&str> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Roll20CharacterSheetName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Roll20CharacterSheetName {
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
///Extracted turn_entry definition
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "Turn Entry",
///  "description": "Extracted turn_entry definition",
///  "type": "object",
///  "properties": {
///    "_pageid": {
///      "type": "string"
///    },
///    "custom": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "pr": {
///      "type": "string"
///    }
///  },
///  "$schema": "https://json-schema.org/draft-07/schema#"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TurnEntry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub custom: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "_pageid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pageid: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pr: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TurnEntry> for TurnEntry {
    fn from(value: &TurnEntry) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TurnEntry {
    fn default() -> Self {
        Self {
            custom: Default::default(),
            id: Default::default(),
            pageid: Default::default(),
            pr: Default::default(),
        }
    }
}
impl TurnEntry {
    pub fn builder() -> builder::TurnEntry {
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
    #[derive(Clone, Debug)]
    pub struct Deck {
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cards: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        playerscandraw: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        shown: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        showplayers: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Deck {
        fn default() -> Self {
            Self {
                avatar: Ok(Default::default()),
                cards: Ok(Default::default()),
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                playerscandraw: Ok(Default::default()),
                shown: Ok(Default::default()),
                showplayers: Ok(Default::default()),
            }
        }
    }
    impl Deck {
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
        pub fn cards<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.cards = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for cards: {}", e)
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
        pub fn playerscandraw<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.playerscandraw = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for playerscandraw: {}", e)
                });
            self
        }
        pub fn shown<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.shown = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for shown: {}", e)
                });
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
    }
    impl ::std::convert::TryFrom<Deck> for super::Deck {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Deck,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                avatar: value.avatar?,
                cards: value.cards?,
                id: value.id?,
                name: value.name?,
                playerscandraw: value.playerscandraw?,
                shown: value.shown?,
                showplayers: value.showplayers?,
            })
        }
    }
    impl ::std::convert::From<super::Deck> for Deck {
        fn from(value: super::Deck) -> Self {
            Self {
                avatar: Ok(value.avatar),
                cards: Ok(value.cards),
                id: Ok(value.id),
                name: Ok(value.name),
                playerscandraw: Ok(value.playerscandraw),
                shown: Ok(value.shown),
                showplayers: Ok(value.showplayers),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct Pdf {
        id: ::std::result::Result<
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
    impl ::std::default::Default for Pdf {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                name: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Pdf {
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
    impl ::std::convert::TryFrom<Pdf> for super::Pdf {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Pdf,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Pdf> for Pdf {
        fn from(value: super::Pdf) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Player {
        color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        displayname: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        showname: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Player {
        fn default() -> Self {
            Self {
                color: Ok(Default::default()),
                displayname: Ok(Default::default()),
                id: Ok(Default::default()),
                showname: Ok(Default::default()),
            }
        }
    }
    impl Player {
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for color: {}", e)
                });
            self
        }
        pub fn displayname<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.displayname = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for displayname: {}", e)
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
        pub fn showname<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.showname = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for showname: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Player> for super::Player {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Player,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                color: value.color?,
                displayname: value.displayname?,
                id: value.id?,
                showname: value.showname?,
            })
        }
    }
    impl ::std::convert::From<super::Player> for Player {
        fn from(value: super::Player) -> Self {
            Self {
                color: Ok(value.color),
                displayname: Ok(value.displayname),
                id: Ok(value.id),
                showname: Ok(value.showname),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Roll20CampaignExport {
        characters: ::std::result::Result<
            ::std::vec::Vec<super::Roll20CharacterSheet>,
            ::std::string::String,
        >,
        decks: ::std::result::Result<
            ::std::vec::Vec<super::Deck>,
            ::std::string::String,
        >,
        handouts: ::std::result::Result<
            ::std::vec::Vec<super::Handout>,
            ::std::string::String,
        >,
        journalfolder: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        jukebox: ::std::result::Result<
            ::std::vec::Vec<super::AudioTrack>,
            ::std::string::String,
        >,
        jukeboxfolder: ::std::result::Result<
            ::std::vec::Vec<::serde_json::Value>,
            ::std::string::String,
        >,
        macros: ::std::result::Result<
            ::std::vec::Vec<super::Macro>,
            ::std::string::String,
        >,
        pages: ::std::result::Result<
            ::std::vec::Vec<super::Page>,
            ::std::string::String,
        >,
        pdfs: ::std::result::Result<::std::vec::Vec<super::Pdf>, ::std::string::String>,
        players: ::std::result::Result<
            ::std::vec::Vec<super::Player>,
            ::std::string::String,
        >,
        tables: ::std::result::Result<
            ::std::vec::Vec<super::Table>,
            ::std::string::String,
        >,
        turnorder: ::std::result::Result<
            ::std::vec::Vec<super::TurnEntry>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CampaignExport {
        fn default() -> Self {
            Self {
                characters: Err("no value supplied for characters".to_string()),
                decks: Ok(Default::default()),
                handouts: Err("no value supplied for handouts".to_string()),
                journalfolder: Ok(Default::default()),
                jukebox: Ok(Default::default()),
                jukeboxfolder: Ok(Default::default()),
                macros: Ok(Default::default()),
                pages: Err("no value supplied for pages".to_string()),
                pdfs: Ok(Default::default()),
                players: Ok(Default::default()),
                tables: Ok(Default::default()),
                turnorder: Ok(Default::default()),
            }
        }
    }
    impl Roll20CampaignExport {
        pub fn characters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Roll20CharacterSheet>>,
            T::Error: ::std::fmt::Display,
        {
            self.characters = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for characters: {}", e)
                });
            self
        }
        pub fn decks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Deck>>,
            T::Error: ::std::fmt::Display,
        {
            self.decks = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decks: {}", e)
                });
            self
        }
        pub fn handouts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Handout>>,
            T::Error: ::std::fmt::Display,
        {
            self.handouts = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for handouts: {}", e)
                });
            self
        }
        pub fn journalfolder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.journalfolder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for journalfolder: {}", e)
                });
            self
        }
        pub fn jukebox<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AudioTrack>>,
            T::Error: ::std::fmt::Display,
        {
            self.jukebox = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for jukebox: {}", e)
                });
            self
        }
        pub fn jukeboxfolder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.jukeboxfolder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for jukeboxfolder: {}", e)
                });
            self
        }
        pub fn macros<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Macro>>,
            T::Error: ::std::fmt::Display,
        {
            self.macros = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for macros: {}", e)
                });
            self
        }
        pub fn pages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Page>>,
            T::Error: ::std::fmt::Display,
        {
            self.pages = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pages: {}", e)
                });
            self
        }
        pub fn pdfs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Pdf>>,
            T::Error: ::std::fmt::Display,
        {
            self.pdfs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pdfs: {}", e));
            self
        }
        pub fn players<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Player>>,
            T::Error: ::std::fmt::Display,
        {
            self.players = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for players: {}", e)
                });
            self
        }
        pub fn tables<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Table>>,
            T::Error: ::std::fmt::Display,
        {
            self.tables = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for tables: {}", e)
                });
            self
        }
        pub fn turnorder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TurnEntry>>,
            T::Error: ::std::fmt::Display,
        {
            self.turnorder = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for turnorder: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CampaignExport> for super::Roll20CampaignExport {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CampaignExport,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                characters: value.characters?,
                decks: value.decks?,
                handouts: value.handouts?,
                journalfolder: value.journalfolder?,
                jukebox: value.jukebox?,
                jukeboxfolder: value.jukeboxfolder?,
                macros: value.macros?,
                pages: value.pages?,
                pdfs: value.pdfs?,
                players: value.players?,
                tables: value.tables?,
                turnorder: value.turnorder?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CampaignExport> for Roll20CampaignExport {
        fn from(value: super::Roll20CampaignExport) -> Self {
            Self {
                characters: Ok(value.characters),
                decks: Ok(value.decks),
                handouts: Ok(value.handouts),
                journalfolder: Ok(value.journalfolder),
                jukebox: Ok(value.jukebox),
                jukeboxfolder: Ok(value.jukeboxfolder),
                macros: Ok(value.macros),
                pages: Ok(value.pages),
                pdfs: Ok(value.pdfs),
                players: Ok(value.players),
                tables: Ok(value.tables),
                turnorder: Ok(value.turnorder),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Roll20CharacterAbility {
        action: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<super::Roll20CharacterAbilityId>,
            ::std::string::String,
        >,
        istokenaction: ::std::result::Result<bool, ::std::string::String>,
        name: ::std::result::Result<
            super::Roll20CharacterAbilityName,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterAbility {
        fn default() -> Self {
            Self {
                action: Ok(Default::default()),
                description: Ok(Default::default()),
                id: Ok(Default::default()),
                istokenaction: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl Roll20CharacterAbility {
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for action: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
            T: ::std::convert::TryInto<
                ::std::option::Option<super::Roll20CharacterAbilityId>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn istokenaction<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
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
            T: ::std::convert::TryInto<super::Roll20CharacterAbilityName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterAbility>
    for super::Roll20CharacterAbility {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterAbility,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                action: value.action?,
                description: value.description?,
                id: value.id?,
                istokenaction: value.istokenaction?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterAbility> for Roll20CharacterAbility {
        fn from(value: super::Roll20CharacterAbility) -> Self {
            Self {
                action: Ok(value.action),
                description: Ok(value.description),
                id: Ok(value.id),
                istokenaction: Ok(value.istokenaction),
                name: Ok(value.name),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct Roll20CharacterSheet {
        abilities: ::std::result::Result<
            ::std::vec::Vec<super::Roll20CharacterAbility>,
            ::std::string::String,
        >,
        archived: ::std::result::Result<bool, ::std::string::String>,
        attribs: ::std::result::Result<
            ::std::vec::Vec<super::Roll20CharacterAttribute>,
            ::std::string::String,
        >,
        avatar: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        bio: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        controlledby: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        defaulttoken: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        gmnotes: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<super::Roll20CharacterSheetId, ::std::string::String>,
        inplayerjournals: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            super::Roll20CharacterSheetName,
            ::std::string::String,
        >,
        tags: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Roll20CharacterSheet {
        fn default() -> Self {
            Self {
                abilities: Ok(Default::default()),
                archived: Ok(Default::default()),
                attribs: Ok(Default::default()),
                avatar: Ok(Default::default()),
                bio: Ok(Default::default()),
                controlledby: Ok(Default::default()),
                defaulttoken: Ok(Default::default()),
                gmnotes: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                inplayerjournals: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                tags: Ok(Default::default()),
            }
        }
    }
    impl Roll20CharacterSheet {
        pub fn abilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Roll20CharacterAbility>>,
            T::Error: ::std::fmt::Display,
        {
            self.abilities = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for abilities: {}", e)
                });
            self
        }
        pub fn archived<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.archived = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for archived: {}", e)
                });
            self
        }
        pub fn attribs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Roll20CharacterAttribute>>,
            T::Error: ::std::fmt::Display,
        {
            self.attribs = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for attribs: {}", e)
                });
            self
        }
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
        pub fn bio<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bio = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bio: {}", e));
            self
        }
        pub fn controlledby<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.controlledby = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for controlledby: {}", e)
                });
            self
        }
        pub fn defaulttoken<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.defaulttoken = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for defaulttoken: {}", e)
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
            T: ::std::convert::TryInto<super::Roll20CharacterSheetId>,
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
            T: ::std::convert::TryInto<super::Roll20CharacterSheetName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Roll20CharacterSheet> for super::Roll20CharacterSheet {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Roll20CharacterSheet,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                abilities: value.abilities?,
                archived: value.archived?,
                attribs: value.attribs?,
                avatar: value.avatar?,
                bio: value.bio?,
                controlledby: value.controlledby?,
                defaulttoken: value.defaulttoken?,
                gmnotes: value.gmnotes?,
                id: value.id?,
                inplayerjournals: value.inplayerjournals?,
                name: value.name?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::Roll20CharacterSheet> for Roll20CharacterSheet {
        fn from(value: super::Roll20CharacterSheet) -> Self {
            Self {
                abilities: Ok(value.abilities),
                archived: Ok(value.archived),
                attribs: Ok(value.attribs),
                avatar: Ok(value.avatar),
                bio: Ok(value.bio),
                controlledby: Ok(value.controlledby),
                defaulttoken: Ok(value.defaulttoken),
                gmnotes: Ok(value.gmnotes),
                id: Ok(value.id),
                inplayerjournals: Ok(value.inplayerjournals),
                name: Ok(value.name),
                tags: Ok(value.tags),
            }
        }
    }
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
    #[derive(Clone, Debug)]
    pub struct TurnEntry {
        custom: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pageid: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pr: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TurnEntry {
        fn default() -> Self {
            Self {
                custom: Ok(Default::default()),
                id: Ok(Default::default()),
                pageid: Ok(Default::default()),
                pr: Ok(Default::default()),
            }
        }
    }
    impl TurnEntry {
        pub fn custom<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.custom = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for custom: {}", e)
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
        pub fn pageid<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pageid = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for pageid: {}", e)
                });
            self
        }
        pub fn pr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pr: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TurnEntry> for super::TurnEntry {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TurnEntry,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                custom: value.custom?,
                id: value.id?,
                pageid: value.pageid?,
                pr: value.pr?,
            })
        }
    }
    impl ::std::convert::From<super::TurnEntry> for TurnEntry {
        fn from(value: super::TurnEntry) -> Self {
            Self {
                custom: Ok(value.custom),
                id: Ok(value.id),
                pageid: Ok(value.pageid),
                pr: Ok(value.pr),
            }
        }
    }
}
