//! Cross-platform audio and video recording support.
//!
//! This module provides platform-agnostic recording APIs. For actual recording,
//! you'll need to integrate with platform-specific APIs or cross-platform
//! libraries like `cpal` (audio) or `nokhwa` (video).
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_platform::recording::{AudioRecorder, RecordingCommand};
//!
//! // Create recorder
//! let recorder = AudioRecorder::new();
//!
//! // Start recording
//! recorder.send(RecordingCommand::Start);
//!
//! // Stop and get the recording
//! recorder.send(RecordingCommand::Stop);
//! ```

use std::path::PathBuf;
use std::time::Duration;

/// Recording commands for both audio and video recorders.
#[derive(Debug, Clone)]
pub enum RecordingCommand {
    /// Start recording.
    Start,
    /// Pause recording.
    Pause,
    /// Resume recording from pause.
    Resume,
    /// Stop recording.
    Stop,
    /// Cancel recording (discard data).
    Cancel,
    /// Set the output file path.
    SetOutputPath(PathBuf),
    /// Set recording quality (0.0 = low, 1.0 = high).
    SetQuality(f32),
    /// Select input device by ID.
    SelectDevice(String),
}

/// Audio-specific recording commands.
#[derive(Debug, Clone)]
pub enum AudioRecordingCommand {
    /// Base recording command.
    Base(RecordingCommand),
    /// Set sample rate.
    SetSampleRate(u32),
    /// Set number of channels (1 = mono, 2 = stereo).
    SetChannels(u16),
    /// Set output format.
    SetFormat(AudioFormat),
    /// Enable/disable noise reduction.
    SetNoiseReduction(bool),
    /// Set input gain.
    SetGain(f32),
}

/// Video-specific recording commands.
#[derive(Debug, Clone)]
pub enum VideoRecordingCommand {
    /// Base recording command.
    Base(RecordingCommand),
    /// Set resolution.
    SetResolution { width: u32, height: u32 },
    /// Set frame rate.
    SetFrameRate(u32),
    /// Set output format/codec.
    SetFormat(VideoFormat),
    /// Enable/disable audio recording with video.
    SetRecordAudio(bool),
    /// Select camera by ID.
    SelectCamera(String),
}

/// Recording state.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum RecordingState {
    /// Recorder is idle, not recording.
    #[default]
    Idle,
    /// Recording is in progress.
    Recording,
    /// Recording is paused.
    Paused,
    /// Recording is being processed/saved.
    Processing,
    /// Recording completed successfully.
    Completed,
    /// Recording failed.
    Failed,
}

/// Audio recording formats.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AudioFormat {
    /// WAV format (uncompressed).
    #[default]
    Wav,
    /// MP3 format (compressed).
    Mp3,
    /// OGG Vorbis format.
    Ogg,
    /// FLAC format (lossless).
    Flac,
    /// WebM audio.
    WebM,
    /// AAC format.
    Aac,
}

impl AudioFormat {
    /// Get the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Mp3 => "mp3",
            Self::Ogg => "ogg",
            Self::Flac => "flac",
            Self::WebM => "webm",
            Self::Aac => "aac",
        }
    }

    /// Get the MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Wav => "audio/wav",
            Self::Mp3 => "audio/mpeg",
            Self::Ogg => "audio/ogg",
            Self::Flac => "audio/flac",
            Self::WebM => "audio/webm",
            Self::Aac => "audio/aac",
        }
    }
}

/// Video recording formats/codecs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum VideoFormat {
    /// MP4 with H.264.
    #[default]
    Mp4H264,
    /// WebM with VP8.
    WebMVp8,
    /// WebM with VP9.
    WebMVp9,
    /// AVI format.
    Avi,
    /// MOV format.
    Mov,
}

impl VideoFormat {
    /// Get the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp4H264 => "mp4",
            Self::WebMVp8 | Self::WebMVp9 => "webm",
            Self::Avi => "avi",
            Self::Mov => "mov",
        }
    }

    /// Get the MIME type for this format.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Mp4H264 => "video/mp4",
            Self::WebMVp8 | Self::WebMVp9 => "video/webm",
            Self::Avi => "video/avi",
            Self::Mov => "video/quicktime",
        }
    }
}

/// Audio recorder state information.
#[derive(Debug, Clone, Default)]
pub struct AudioRecorderState {
    /// Current recording state.
    pub state: RecordingState,
    /// Recording duration so far.
    pub duration: Duration,
    /// Current audio level (0.0 to 1.0).
    pub level: f32,
    /// Peak audio level.
    pub peak_level: f32,
    /// Selected input device name.
    pub device: Option<String>,
    /// Output file path if set.
    pub output_path: Option<PathBuf>,
    /// Recording format.
    pub format: AudioFormat,
    /// Sample rate.
    pub sample_rate: u32,
    /// Number of channels.
    pub channels: u16,
    /// Error message if any.
    pub error: Option<String>,
}

impl AudioRecorderState {
    /// Create a new default audio recorder state.
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            ..Default::default()
        }
    }

    /// Check if currently recording.
    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }

    /// Check if paused.
    pub fn is_paused(&self) -> bool {
        self.state == RecordingState::Paused
    }
}

/// Video recorder state information.
#[derive(Debug, Clone, Default)]
pub struct VideoRecorderState {
    /// Current recording state.
    pub state: RecordingState,
    /// Recording duration so far.
    pub duration: Duration,
    /// Selected camera device name.
    pub camera: Option<String>,
    /// Output file path if set.
    pub output_path: Option<PathBuf>,
    /// Recording format.
    pub format: VideoFormat,
    /// Resolution width.
    pub width: u32,
    /// Resolution height.
    pub height: u32,
    /// Frame rate.
    pub frame_rate: u32,
    /// Whether audio is being recorded.
    pub recording_audio: bool,
    /// Error message if any.
    pub error: Option<String>,
}

impl VideoRecorderState {
    /// Create a new default video recorder state.
    pub fn new() -> Self {
        Self {
            width: 1280,
            height: 720,
            frame_rate: 30,
            recording_audio: true,
            ..Default::default()
        }
    }

    /// Check if currently recording.
    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }

    /// Check if paused.
    pub fn is_paused(&self) -> bool {
        self.state == RecordingState::Paused
    }
}

/// Events from a recorder.
#[derive(Debug, Clone)]
pub enum RecordingEvent {
    /// Recording started.
    Started,
    /// Recording paused.
    Paused,
    /// Recording resumed.
    Resumed,
    /// Recording stopped, file saved.
    Completed { path: PathBuf, duration: Duration },
    /// Recording cancelled.
    Cancelled,
    /// Audio level update.
    LevelUpdate(f32),
    /// Duration update.
    DurationUpdate(Duration),
    /// Error occurred.
    Error(String),
    /// Device list changed.
    DevicesChanged,
}

/// Information about an input device.
#[derive(Debug, Clone)]
pub struct InputDevice {
    /// Unique device identifier.
    pub id: String,
    /// Human-readable device name.
    pub name: String,
    /// Whether this is the default device.
    pub is_default: bool,
}

/// Trait for audio recorder backends.
pub trait AudioRecorderBackend: Send {
    /// Send a command to the recorder.
    fn send(&self, command: AudioRecordingCommand) -> Result<(), RecordingError>;

    /// Get the current state.
    fn state(&self) -> AudioRecorderState;

    /// List available input devices.
    fn list_devices(&self) -> Vec<InputDevice>;
}

/// Trait for video recorder backends.
pub trait VideoRecorderBackend: Send {
    /// Send a command to the recorder.
    fn send(&self, command: VideoRecordingCommand) -> Result<(), RecordingError>;

    /// Get the current state.
    fn state(&self) -> VideoRecorderState;

    /// List available cameras.
    fn list_cameras(&self) -> Vec<InputDevice>;

    /// Get a frame for preview (if available).
    fn preview_frame(&self) -> Option<PreviewFrame>;
}

/// A video preview frame.
#[derive(Debug, Clone)]
pub struct PreviewFrame {
    /// Raw RGBA pixel data.
    pub data: Vec<u8>,
    /// Frame width.
    pub width: u32,
    /// Frame height.
    pub height: u32,
}

/// Recording errors.
#[derive(Debug, Clone)]
pub enum RecordingError {
    /// No device available.
    NoDeviceAvailable,
    /// Device not found.
    DeviceNotFound(String),
    /// Permission denied.
    PermissionDenied,
    /// Device busy.
    DeviceBusy,
    /// Encoding error.
    EncodingError(String),
    /// IO error.
    IoError(String),
    /// Backend not available.
    BackendUnavailable,
    /// Generic error.
    Other(String),
}

impl std::fmt::Display for RecordingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDeviceAvailable => write!(f, "No recording device available"),
            Self::DeviceNotFound(id) => write!(f, "Recording device not found: {}", id),
            Self::PermissionDenied => write!(f, "Permission denied for recording"),
            Self::DeviceBusy => write!(f, "Recording device is busy"),
            Self::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::BackendUnavailable => write!(f, "Recording backend unavailable"),
            Self::Other(msg) => write!(f, "Recording error: {}", msg),
        }
    }
}

impl std::error::Error for RecordingError {}

/// No-op audio recorder for when no backend is available.
#[derive(Debug, Default)]
pub struct NoOpAudioRecorder {
    state: AudioRecorderState,
}

impl NoOpAudioRecorder {
    /// Create a new no-op audio recorder.
    pub fn new() -> Self {
        Self {
            state: AudioRecorderState::new(),
        }
    }
}

impl AudioRecorderBackend for NoOpAudioRecorder {
    fn send(&self, _command: AudioRecordingCommand) -> Result<(), RecordingError> {
        Ok(())
    }

    fn state(&self) -> AudioRecorderState {
        self.state.clone()
    }

    fn list_devices(&self) -> Vec<InputDevice> {
        Vec::new()
    }
}

/// No-op video recorder for when no backend is available.
#[derive(Debug, Default)]
pub struct NoOpVideoRecorder {
    state: VideoRecorderState,
}

impl NoOpVideoRecorder {
    /// Create a new no-op video recorder.
    pub fn new() -> Self {
        Self {
            state: VideoRecorderState::new(),
        }
    }
}

impl VideoRecorderBackend for NoOpVideoRecorder {
    fn send(&self, _command: VideoRecordingCommand) -> Result<(), RecordingError> {
        Ok(())
    }

    fn state(&self) -> VideoRecorderState {
        self.state.clone()
    }

    fn list_cameras(&self) -> Vec<InputDevice> {
        Vec::new()
    }

    fn preview_frame(&self) -> Option<PreviewFrame> {
        None
    }
}
