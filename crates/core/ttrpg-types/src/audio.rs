//! Audio and playlist data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core playlist data structure - unified representation across all systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    /// Playlist unique identifier
    pub id: String,
    /// Playlist name
    pub name: String,
    /// Playlist description
    pub description: Option<String>,
    /// Audio tracks in the playlist
    pub tracks: Vec<String>, // AudioTrack IDs
    /// Whether playlist is currently playing
    pub playing: Option<bool>,
    /// Current track index
    pub current_track: Option<usize>,
    /// Playlist volume (0.0 - 1.0)
    pub volume: Option<f64>,
    /// Loop playlist when finished
    pub repeat: Option<bool>,
    /// Shuffle tracks
    pub shuffle: Option<bool>,
    /// System-specific data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Core audio track data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrack {
    /// Track unique identifier
    pub id: String,
    /// Track name/title
    pub name: String,
    /// Track description
    pub description: Option<String>,
    /// Audio file path or URL
    pub source: String,
    /// Track duration in seconds
    pub duration: Option<f64>,
    /// Track volume (0.0 - 1.0)
    pub volume: Option<f64>,
    /// Loop this track
    pub repeat: Option<bool>,
    /// Fade in/out settings
    pub fade: Option<AudioFade>,
    /// Track is currently playing
    pub playing: Option<bool>,
    /// Current playback position in seconds
    pub position: Option<f64>,
    /// System-specific data
    pub system_data: HashMap<String, serde_json::Value>,
}

/// Audio fade configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFade {
    /// Fade in duration in seconds
    pub fade_in: Option<f64>,
    /// Fade out duration in seconds
    pub fade_out: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_creation() {
        let playlist = Playlist {
            id: "playlist_001".to_string(),
            name: "Tavern Ambiance".to_string(),
            description: Some("Background music for tavern scenes".to_string()),
            tracks: vec!["track_001".to_string(), "track_002".to_string()],
            playing: Some(false),
            current_track: Some(0),
            volume: Some(0.7),
            repeat: Some(true),
            shuffle: Some(false),
            system_data: HashMap::new(),
        };

        assert_eq!(playlist.name, "Tavern Ambiance");
        assert_eq!(playlist.tracks.len(), 2);
        assert_eq!(playlist.volume, Some(0.7));
    }

    #[test]
    fn test_audio_track_creation() {
        let track = AudioTrack {
            id: "track_001".to_string(),
            name: "Tavern Chatter".to_string(),
            description: Some("Ambient tavern background noise".to_string()),
            source: "audio/tavern_chatter.mp3".to_string(),
            duration: Some(180.0),
            volume: Some(0.5),
            repeat: Some(true),
            fade: Some(AudioFade {
                fade_in: Some(2.0),
                fade_out: Some(3.0),
            }),
            playing: Some(false),
            position: Some(0.0),
            system_data: HashMap::new(),
        };

        assert_eq!(track.name, "Tavern Chatter");
        assert_eq!(track.duration, Some(180.0));
        assert!(track.fade.is_some());
    }

    #[test]
    fn test_audio_fade_configuration() {
        let fade = AudioFade {
            fade_in: Some(1.5),
            fade_out: Some(2.5),
        };

        assert_eq!(fade.fade_in, Some(1.5));
        assert_eq!(fade.fade_out, Some(2.5));
    }
}
