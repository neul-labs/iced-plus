//! Cross-platform audio playback support.
//!
//! This module provides a platform-agnostic audio player API. For actual audio playback,
//! you'll need to integrate with a cross-platform audio library like `rodio`, `cpal`,
//! or platform-specific APIs.
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_platform::audio::{AudioPlayer, AudioCommand};
//!
//! // Create player
//! let player = AudioPlayer::new();
//!
//! // In your update function, send commands to the player
//! player.send(AudioCommand::LoadFile("song.mp3".into()));
//! player.send(AudioCommand::Play);
//! ```

use std::path::PathBuf;
use std::time::Duration;

/// Audio playback commands that can be sent to an audio player.
#[derive(Debug, Clone)]
pub enum AudioCommand {
    /// Load an audio file from the given path.
    LoadFile(PathBuf),
    /// Load audio from raw bytes (format must be specified).
    LoadBytes {
        /// Raw audio data.
        data: Vec<u8>,
        /// Audio format (e.g., "mp3", "wav", "ogg").
        format: String,
    },
    /// Load audio from a URL.
    LoadUrl(String),
    /// Start or resume playback.
    Play,
    /// Pause playback.
    Pause,
    /// Stop playback and reset to beginning.
    Stop,
    /// Seek to a specific position.
    Seek(Duration),
    /// Set playback volume (0.0 to 1.0).
    SetVolume(f32),
    /// Set playback speed (1.0 = normal).
    SetSpeed(f32),
    /// Toggle mute state.
    ToggleMute,
    /// Enable or disable looping.
    SetLoop(bool),
}

/// Audio player state that can be reported back to the application.
#[derive(Debug, Clone, Default)]
pub struct AudioState {
    /// Whether audio is currently playing.
    pub playing: bool,
    /// Whether audio is paused.
    pub paused: bool,
    /// Whether audio is muted.
    pub muted: bool,
    /// Current playback position.
    pub position: Duration,
    /// Total duration of the loaded audio.
    pub duration: Option<Duration>,
    /// Current volume (0.0 to 1.0).
    pub volume: f32,
    /// Current playback speed.
    pub speed: f32,
    /// Whether looping is enabled.
    pub looping: bool,
    /// Error message if any.
    pub error: Option<String>,
}

impl AudioState {
    /// Create a new default audio state.
    pub fn new() -> Self {
        Self {
            volume: 1.0,
            speed: 1.0,
            ..Default::default()
        }
    }

    /// Get playback progress as a value between 0.0 and 1.0.
    pub fn progress(&self) -> f32 {
        match self.duration {
            Some(duration) if !duration.is_zero() => {
                self.position.as_secs_f32() / duration.as_secs_f32()
            }
            _ => 0.0,
        }
    }
}

/// Messages that an audio player backend can send to the application.
#[derive(Debug, Clone)]
pub enum AudioEvent {
    /// Playback started.
    Started,
    /// Playback paused.
    Paused,
    /// Playback stopped.
    Stopped,
    /// Playback finished (reached end).
    Finished,
    /// Position changed (periodic update).
    PositionChanged(Duration),
    /// Duration determined (after loading).
    DurationDetermined(Duration),
    /// Volume changed.
    VolumeChanged(f32),
    /// Error occurred.
    Error(String),
    /// State update (full state).
    StateChanged(AudioState),
}

/// Trait for audio player backends.
///
/// Implement this trait to create a custom audio player backend using
/// libraries like `rodio`, `cpal`, or platform-specific APIs.
pub trait AudioBackend: Send {
    /// Send a command to the audio player.
    fn send(&self, command: AudioCommand) -> Result<(), AudioError>;

    /// Get the current state of the audio player.
    fn state(&self) -> AudioState;

    /// Check if the player supports streaming.
    fn supports_streaming(&self) -> bool {
        false
    }

    /// Check if the player supports a given format.
    fn supports_format(&self, format: &str) -> bool;
}

/// Audio player errors.
#[derive(Debug, Clone)]
pub enum AudioError {
    /// File not found.
    FileNotFound(PathBuf),
    /// Unsupported format.
    UnsupportedFormat(String),
    /// Decoding error.
    DecodeError(String),
    /// Playback error.
    PlaybackError(String),
    /// Network error (for URL loading).
    NetworkError(String),
    /// Backend not available.
    BackendUnavailable,
    /// Generic error.
    Other(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(path) => write!(f, "Audio file not found: {}", path.display()),
            Self::UnsupportedFormat(fmt) => write!(f, "Unsupported audio format: {}", fmt),
            Self::DecodeError(msg) => write!(f, "Audio decode error: {}", msg),
            Self::PlaybackError(msg) => write!(f, "Audio playback error: {}", msg),
            Self::NetworkError(msg) => write!(f, "Network error: {}", msg),
            Self::BackendUnavailable => write!(f, "Audio backend unavailable"),
            Self::Other(msg) => write!(f, "Audio error: {}", msg),
        }
    }
}

impl std::error::Error for AudioError {}

/// A no-op audio player for when no backend is available.
///
/// This allows the UI components to work even without actual audio support,
/// providing a graceful degradation.
#[derive(Debug, Default)]
pub struct NoOpAudioPlayer {
    state: AudioState,
}

impl NoOpAudioPlayer {
    /// Create a new no-op audio player.
    pub fn new() -> Self {
        Self {
            state: AudioState::new(),
        }
    }
}

impl AudioBackend for NoOpAudioPlayer {
    fn send(&self, _command: AudioCommand) -> Result<(), AudioError> {
        // No-op: just accept commands without doing anything
        Ok(())
    }

    fn state(&self) -> AudioState {
        self.state.clone()
    }

    fn supports_format(&self, _format: &str) -> bool {
        false
    }
}

/// Helper utilities for audio integration.
pub mod utils {
    use super::*;

    /// Format duration as MM:SS.
    pub fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let minutes = total_secs / 60;
        let seconds = total_secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Format duration as HH:MM:SS for longer durations.
    pub fn format_duration_long(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }

    /// Convert progress (0.0 to 1.0) to duration.
    pub fn progress_to_duration(progress: f32, total: Duration) -> Duration {
        Duration::from_secs_f32(progress * total.as_secs_f32())
    }
}

/// Common audio formats.
pub mod formats {
    /// MP3 format identifier.
    pub const MP3: &str = "mp3";
    /// WAV format identifier.
    pub const WAV: &str = "wav";
    /// OGG Vorbis format identifier.
    pub const OGG: &str = "ogg";
    /// FLAC format identifier.
    pub const FLAC: &str = "flac";
    /// AAC format identifier.
    pub const AAC: &str = "aac";
    /// WebM format identifier.
    pub const WEBM: &str = "webm";

    /// Get all commonly supported formats.
    pub fn all() -> &'static [&'static str] {
        &[MP3, WAV, OGG, FLAC, AAC, WEBM]
    }
}
