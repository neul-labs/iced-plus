//! Media player components (audio/video).
//!
//! Note: iced doesn't have built-in audio/video support.
//! These components provide UI controls and state management.
//! Actual playback requires platform-specific integration.

use std::time::Duration;

/// Playback state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PlaybackState {
    /// Not started or stopped.
    #[default]
    Stopped,
    /// Currently playing.
    Playing,
    /// Paused.
    Paused,
    /// Loading/buffering.
    Loading,
    /// Error occurred.
    Error,
}

/// Media player state (for audio or video).
///
/// This struct manages playback state. Actual media playback
/// must be handled by platform-specific code or external libraries.
///
/// # Example
///
/// ```rust,ignore
/// struct App {
///     player: MediaPlayerState,
/// }
///
/// impl App {
///     fn update(&mut self, message: Message) -> Command<Message> {
///         match message {
///             Message::Play => {
///                 self.player.play();
///                 // Start actual playback...
///             }
///             Message::Pause => {
///                 self.player.pause();
///                 // Pause actual playback...
///             }
///             Message::Seek(position) => {
///                 self.player.seek(position);
///                 // Seek in actual player...
///             }
///             Message::UpdatePosition(pos) => {
///                 self.player.set_position(pos);
///             }
///         }
///         Command::none()
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MediaPlayerState {
    /// Current playback state.
    pub state: PlaybackState,
    /// Current position.
    pub position: Duration,
    /// Total duration (if known).
    pub duration: Option<Duration>,
    /// Volume (0.0 to 1.0).
    pub volume: f32,
    /// Whether muted.
    pub muted: bool,
    /// Playback speed (1.0 = normal).
    pub speed: f32,
    /// Whether looping is enabled.
    pub looping: bool,
}

impl Default for MediaPlayerState {
    fn default() -> Self {
        Self::new()
    }
}

impl MediaPlayerState {
    /// Create a new player state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: PlaybackState::Stopped,
            position: Duration::ZERO,
            duration: None,
            volume: 1.0,
            muted: false,
            speed: 1.0,
            looping: false,
        }
    }

    /// Start or resume playback.
    pub fn play(&mut self) {
        self.state = PlaybackState::Playing;
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    /// Toggle play/pause.
    pub fn toggle(&mut self) {
        match self.state {
            PlaybackState::Playing => self.pause(),
            PlaybackState::Paused | PlaybackState::Stopped => self.play(),
            _ => {}
        }
    }

    /// Stop playback and reset position.
    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
        self.position = Duration::ZERO;
    }

    /// Seek to a position.
    pub fn seek(&mut self, position: Duration) {
        if let Some(duration) = self.duration {
            self.position = position.min(duration);
        } else {
            self.position = position;
        }
    }

    /// Update the current position (called from playback updates).
    pub fn set_position(&mut self, position: Duration) {
        self.position = position;
    }

    /// Set the total duration.
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = Some(duration);
    }

    /// Set volume (clamped to 0.0-1.0).
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Toggle mute.
    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    /// Set playback speed.
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.max(0.25);
    }

    /// Toggle looping.
    pub fn toggle_loop(&mut self) {
        self.looping = !self.looping;
    }

    /// Get progress as a percentage (0.0-1.0).
    #[must_use]
    pub fn progress(&self) -> f32 {
        if let Some(duration) = self.duration {
            if duration.as_secs_f32() > 0.0 {
                return self.position.as_secs_f32() / duration.as_secs_f32();
            }
        }
        0.0
    }

    /// Check if currently playing.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.state == PlaybackState::Playing
    }

    /// Format time as MM:SS or HH:MM:SS.
    #[must_use]
    pub fn format_time(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            format!("{hours}:{mins:02}:{secs:02}")
        } else {
            format!("{mins}:{secs:02}")
        }
    }

    /// Get formatted position string.
    #[must_use]
    pub fn position_str(&self) -> String {
        Self::format_time(self.position)
    }

    /// Get formatted duration string.
    #[must_use]
    pub fn duration_str(&self) -> String {
        self.duration
            .map(Self::format_time)
            .unwrap_or_else(|| "--:--".to_string())
    }
}

/// Audio player controls widget.
///
/// This creates standard audio player controls (play/pause, seek bar, volume).
/// Connect the messages to your actual audio playback implementation.
///
/// # Example
///
/// ```rust,ignore
/// AudioControls::new(&self.player)
///     .on_play(Message::Play)
///     .on_pause(Message::Pause)
///     .on_seek(Message::Seek)
///     .on_volume(Message::Volume)
/// ```
pub struct AudioControls<'a, Message> {
    state: &'a MediaPlayerState,
    on_play: Option<Message>,
    on_pause: Option<Message>,
    on_stop: Option<Message>,
    on_seek: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_volume: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_mute: Option<Message>,
    show_time: bool,
    compact: bool,
}

impl<'a, Message> AudioControls<'a, Message>
where
    Message: Clone,
{
    /// Create new audio controls.
    pub fn new(state: &'a MediaPlayerState) -> Self {
        Self {
            state,
            on_play: None,
            on_pause: None,
            on_stop: None,
            on_seek: None,
            on_volume: None,
            on_mute: None,
            show_time: true,
            compact: false,
        }
    }

    /// Set the play message.
    #[must_use]
    pub fn on_play(mut self, message: Message) -> Self {
        self.on_play = Some(message);
        self
    }

    /// Set the pause message.
    #[must_use]
    pub fn on_pause(mut self, message: Message) -> Self {
        self.on_pause = Some(message);
        self
    }

    /// Set the stop message.
    #[must_use]
    pub fn on_stop(mut self, message: Message) -> Self {
        self.on_stop = Some(message);
        self
    }

    /// Set the seek callback (receives progress 0.0-1.0).
    #[must_use]
    pub fn on_seek<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_seek = Some(Box::new(f));
        self
    }

    /// Set the volume callback (receives volume 0.0-1.0).
    #[must_use]
    pub fn on_volume<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.on_volume = Some(Box::new(f));
        self
    }

    /// Set the mute toggle message.
    #[must_use]
    pub fn on_mute(mut self, message: Message) -> Self {
        self.on_mute = Some(message);
        self
    }

    /// Hide time display.
    #[must_use]
    pub fn hide_time(mut self) -> Self {
        self.show_time = false;
        self
    }

    /// Use compact layout.
    #[must_use]
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
}

impl<'a, Message> From<AudioControls<'a, Message>>
    for iced::Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(controls: AudioControls<'a, Message>) -> Self {
        use iced::widget::{button, column, row, slider, text, Space};
        use iced::Length;

        let play_pause: iced::Element<'a, Message, iced::Theme> = {
            let label = if controls.state.is_playing() {
                "⏸"
            } else {
                "▶"
            };

            let mut btn = button(text(label).size(16));

            if controls.state.is_playing() {
                if let Some(msg) = controls.on_pause {
                    btn = btn.on_press(msg);
                }
            } else if let Some(msg) = controls.on_play {
                btn = btn.on_press(msg);
            }

            btn.into()
        };

        let stop: Option<iced::Element<'a, Message, iced::Theme>> =
            controls.on_stop.map(|msg| {
                button(text("⏹").size(16)).on_press(msg).into()
            });

        let seek_bar: Option<iced::Element<'a, Message, iced::Theme>> =
            controls.on_seek.map(|on_seek| {
                slider(0.0..=1.0, controls.state.progress(), on_seek)
                    .step(0.01)
                    .width(Length::Fill)
                    .into()
            });

        let time_display: Option<iced::Element<'a, Message, iced::Theme>> =
            if controls.show_time {
                Some(
                    text(format!(
                        "{} / {}",
                        controls.state.position_str(),
                        controls.state.duration_str()
                    ))
                    .size(12)
                    .into(),
                )
            } else {
                None
            };

        let volume: Option<iced::Element<'a, Message, iced::Theme>> =
            controls.on_volume.map(|on_volume| {
                let vol = if controls.state.muted {
                    0.0
                } else {
                    controls.state.volume
                };
                slider(0.0..=1.0, vol, on_volume)
                    .step(0.05)
                    .width(Length::Fixed(80.0))
                    .into()
            });

        let mute_btn: Option<iced::Element<'a, Message, iced::Theme>> =
            controls.on_mute.map(|msg| {
                let icon = if controls.state.muted { "🔇" } else { "🔊" };
                button(text(icon).size(14)).on_press(msg).into()
            });

        if controls.compact {
            let mut items: Vec<iced::Element<'a, Message, iced::Theme>> = vec![play_pause];
            if let Some(seek) = seek_bar {
                items.push(seek);
            }
            if let Some(time) = time_display {
                items.push(time);
            }
            row(items).spacing(8).align_y(iced::Alignment::Center).into()
        } else {
            let mut top_row: Vec<iced::Element<'a, Message, iced::Theme>> = vec![play_pause];
            if let Some(stop) = stop {
                top_row.push(stop);
            }
            top_row.push(Space::with_width(Length::Fill).into());
            if let Some(mute) = mute_btn {
                top_row.push(mute);
            }
            if let Some(vol) = volume {
                top_row.push(vol);
            }

            let mut content: Vec<iced::Element<'a, Message, iced::Theme>> = vec![
                row(top_row)
                    .spacing(8)
                    .align_y(iced::Alignment::Center)
                    .into(),
            ];

            if let Some(seek) = seek_bar {
                content.push(seek);
            }

            if let Some(time) = time_display {
                content.push(time);
            }

            column(content).spacing(8).into()
        }
    }
}

/// Recording state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum RecordingState {
    /// Not recording.
    #[default]
    Idle,
    /// Recording in progress.
    Recording,
    /// Recording paused.
    Paused,
    /// Processing/saving the recording.
    Processing,
    /// Error occurred.
    Error,
}

/// State for audio/video recorder.
///
/// # Example
///
/// ```rust,ignore
/// struct App {
///     recorder: RecorderState,
/// }
///
/// impl App {
///     fn update(&mut self, message: Message) -> Command<Message> {
///         match message {
///             Message::StartRecording => {
///                 self.recorder.start();
///                 // Start platform recording...
///             }
///             Message::StopRecording => {
///                 self.recorder.stop();
///                 // Stop and save recording...
///             }
///         }
///         Command::none()
///     }
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct RecorderState {
    /// Current recording state.
    pub state: RecordingState,
    /// Recording duration.
    pub duration: Duration,
    /// Audio input level (0.0-1.0) for visualization.
    pub audio_level: f32,
    /// Whether audio is enabled.
    pub audio_enabled: bool,
    /// Whether video is enabled (for video recorder).
    pub video_enabled: bool,
    /// Max recording duration (None = unlimited).
    pub max_duration: Option<Duration>,
}

impl RecorderState {
    /// Create a new recorder state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            duration: Duration::ZERO,
            audio_level: 0.0,
            audio_enabled: true,
            video_enabled: true,
            max_duration: None,
        }
    }

    /// Create an audio-only recorder.
    #[must_use]
    pub fn audio_only() -> Self {
        Self {
            video_enabled: false,
            ..Self::new()
        }
    }

    /// Set maximum recording duration.
    #[must_use]
    pub fn with_max_duration(mut self, duration: Duration) -> Self {
        self.max_duration = Some(duration);
        self
    }

    /// Start recording.
    pub fn start(&mut self) {
        self.state = RecordingState::Recording;
        self.duration = Duration::ZERO;
    }

    /// Pause recording.
    pub fn pause(&mut self) {
        if self.state == RecordingState::Recording {
            self.state = RecordingState::Paused;
        }
    }

    /// Resume recording.
    pub fn resume(&mut self) {
        if self.state == RecordingState::Paused {
            self.state = RecordingState::Recording;
        }
    }

    /// Stop recording.
    pub fn stop(&mut self) {
        self.state = RecordingState::Processing;
    }

    /// Mark as finished processing.
    pub fn finish(&mut self) {
        self.state = RecordingState::Idle;
        self.duration = Duration::ZERO;
    }

    /// Mark as error.
    pub fn error(&mut self) {
        self.state = RecordingState::Error;
    }

    /// Update duration (call periodically during recording).
    pub fn update_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Update audio level for visualization.
    pub fn update_audio_level(&mut self, level: f32) {
        self.audio_level = level.clamp(0.0, 1.0);
    }

    /// Toggle audio.
    pub fn toggle_audio(&mut self) {
        self.audio_enabled = !self.audio_enabled;
    }

    /// Toggle video.
    pub fn toggle_video(&mut self) {
        self.video_enabled = !self.video_enabled;
    }

    /// Check if recording.
    #[must_use]
    pub fn is_recording(&self) -> bool {
        self.state == RecordingState::Recording
    }

    /// Check if can start.
    #[must_use]
    pub fn can_start(&self) -> bool {
        self.state == RecordingState::Idle || self.state == RecordingState::Error
    }

    /// Format duration.
    #[must_use]
    pub fn duration_str(&self) -> String {
        MediaPlayerState::format_time(self.duration)
    }

    /// Get remaining time if max duration set.
    #[must_use]
    pub fn remaining(&self) -> Option<Duration> {
        self.max_duration.map(|max| max.saturating_sub(self.duration))
    }
}

/// Audio recorder controls widget.
///
/// # Example
///
/// ```rust,ignore
/// AudioRecorder::new(&self.recorder)
///     .on_start(Message::StartRecording)
///     .on_stop(Message::StopRecording)
///     .on_pause(Message::PauseRecording)
/// ```
pub struct AudioRecorder<'a, Message> {
    state: &'a RecorderState,
    on_start: Option<Message>,
    on_stop: Option<Message>,
    on_pause: Option<Message>,
    on_resume: Option<Message>,
    on_toggle_audio: Option<Message>,
    show_level: bool,
    show_time: bool,
}

impl<'a, Message> AudioRecorder<'a, Message>
where
    Message: Clone,
{
    /// Create new audio recorder controls.
    pub fn new(state: &'a RecorderState) -> Self {
        Self {
            state,
            on_start: None,
            on_stop: None,
            on_pause: None,
            on_resume: None,
            on_toggle_audio: None,
            show_level: true,
            show_time: true,
        }
    }

    /// Set start recording message.
    #[must_use]
    pub fn on_start(mut self, message: Message) -> Self {
        self.on_start = Some(message);
        self
    }

    /// Set stop recording message.
    #[must_use]
    pub fn on_stop(mut self, message: Message) -> Self {
        self.on_stop = Some(message);
        self
    }

    /// Set pause recording message.
    #[must_use]
    pub fn on_pause(mut self, message: Message) -> Self {
        self.on_pause = Some(message);
        self
    }

    /// Set resume recording message.
    #[must_use]
    pub fn on_resume(mut self, message: Message) -> Self {
        self.on_resume = Some(message);
        self
    }

    /// Set audio toggle message.
    #[must_use]
    pub fn on_toggle_audio(mut self, message: Message) -> Self {
        self.on_toggle_audio = Some(message);
        self
    }

    /// Hide audio level indicator.
    #[must_use]
    pub fn hide_level(mut self) -> Self {
        self.show_level = false;
        self
    }

    /// Hide time display.
    #[must_use]
    pub fn hide_time(mut self) -> Self {
        self.show_time = false;
        self
    }
}

impl<'a, Message> From<AudioRecorder<'a, Message>>
    for iced::Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(recorder: AudioRecorder<'a, Message>) -> Self {
        use iced::widget::{button, column, container, progress_bar, row, text, Space};
        use iced::{Background, Border, Color, Length};

        let state = recorder.state;

        // Record/Stop button
        let main_btn: iced::Element<'a, Message, iced::Theme> = match state.state {
            RecordingState::Idle | RecordingState::Error => {
                let mut btn = button(
                    container(Space::new(Length::Fixed(20.0), Length::Fixed(20.0)))
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(Color::from_rgb(0.9, 0.2, 0.2))),
                            border: Border {
                                radius: 10.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                )
                .padding(8);
                if let Some(msg) = recorder.on_start {
                    btn = btn.on_press(msg);
                }
                btn.into()
            }
            RecordingState::Recording | RecordingState::Paused => {
                let mut btn = button(
                    container(Space::new(Length::Fixed(20.0), Length::Fixed(20.0)))
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
                            border: Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                )
                .padding(8);
                if let Some(msg) = recorder.on_stop {
                    btn = btn.on_press(msg);
                }
                btn.into()
            }
            RecordingState::Processing => {
                button(text("...").size(14)).into()
            }
        };

        // Pause/Resume button
        let pause_btn: Option<iced::Element<'a, Message, iced::Theme>> =
            if state.state == RecordingState::Recording {
                recorder.on_pause.map(|msg| {
                    button(text("II").size(14)).on_press(msg).into()
                })
            } else if state.state == RecordingState::Paused {
                recorder.on_resume.map(|msg| {
                    button(text("Resume").size(12)).on_press(msg).into()
                })
            } else {
                None
            };

        // Audio level indicator
        let level_indicator: Option<iced::Element<'a, Message, iced::Theme>> =
            if recorder.show_level && (state.state == RecordingState::Recording || state.state == RecordingState::Paused) {
                Some(
                    progress_bar(0.0..=1.0, state.audio_level)
                        .height(8)
                        .width(Length::Fixed(100.0))
                        .into(),
                )
            } else {
                None
            };

        // Time display
        let time: Option<iced::Element<'a, Message, iced::Theme>> =
            if recorder.show_time && state.state != RecordingState::Idle {
                let time_text = if state.state == RecordingState::Recording {
                    format!("REC {}", state.duration_str())
                } else if state.state == RecordingState::Paused {
                    format!("PAUSED {}", state.duration_str())
                } else {
                    state.duration_str()
                };
                Some(text(time_text).size(14).into())
            } else {
                None
            };

        // Audio toggle
        let audio_toggle: Option<iced::Element<'a, Message, iced::Theme>> =
            recorder.on_toggle_audio.map(|msg| {
                let icon = if state.audio_enabled { "Mic ON" } else { "Mic OFF" };
                button(text(icon).size(12)).on_press(msg).into()
            });

        // Layout
        let mut items: Vec<iced::Element<'a, Message, iced::Theme>> = vec![main_btn];
        if let Some(pause) = pause_btn {
            items.push(pause);
        }
        if let Some(level) = level_indicator {
            items.push(level);
        }
        if let Some(t) = time {
            items.push(t);
        }
        if let Some(audio) = audio_toggle {
            items.push(audio);
        }

        column![
            row(items).spacing(12).align_y(iced::Alignment::Center),
        ]
        .spacing(8)
        .into()
    }
}

/// Video recorder controls widget.
///
/// # Example
///
/// ```rust,ignore
/// VideoRecorder::new(&self.recorder)
///     .on_start(Message::StartRecording)
///     .on_stop(Message::StopRecording)
/// ```
pub struct VideoRecorder<'a, Message> {
    state: &'a RecorderState,
    on_start: Option<Message>,
    on_stop: Option<Message>,
    on_pause: Option<Message>,
    on_resume: Option<Message>,
    on_toggle_audio: Option<Message>,
    on_toggle_video: Option<Message>,
    on_switch_camera: Option<Message>,
    show_preview: bool,
}

impl<'a, Message> VideoRecorder<'a, Message>
where
    Message: Clone,
{
    /// Create new video recorder controls.
    pub fn new(state: &'a RecorderState) -> Self {
        Self {
            state,
            on_start: None,
            on_stop: None,
            on_pause: None,
            on_resume: None,
            on_toggle_audio: None,
            on_toggle_video: None,
            on_switch_camera: None,
            show_preview: true,
        }
    }

    /// Set start recording message.
    #[must_use]
    pub fn on_start(mut self, message: Message) -> Self {
        self.on_start = Some(message);
        self
    }

    /// Set stop recording message.
    #[must_use]
    pub fn on_stop(mut self, message: Message) -> Self {
        self.on_stop = Some(message);
        self
    }

    /// Set pause recording message.
    #[must_use]
    pub fn on_pause(mut self, message: Message) -> Self {
        self.on_pause = Some(message);
        self
    }

    /// Set resume recording message.
    #[must_use]
    pub fn on_resume(mut self, message: Message) -> Self {
        self.on_resume = Some(message);
        self
    }

    /// Set audio toggle message.
    #[must_use]
    pub fn on_toggle_audio(mut self, message: Message) -> Self {
        self.on_toggle_audio = Some(message);
        self
    }

    /// Set video toggle message.
    #[must_use]
    pub fn on_toggle_video(mut self, message: Message) -> Self {
        self.on_toggle_video = Some(message);
        self
    }

    /// Set switch camera message.
    #[must_use]
    pub fn on_switch_camera(mut self, message: Message) -> Self {
        self.on_switch_camera = Some(message);
        self
    }

    /// Hide preview area.
    #[must_use]
    pub fn hide_preview(mut self) -> Self {
        self.show_preview = false;
        self
    }
}

impl<'a, Message> From<VideoRecorder<'a, Message>>
    for iced::Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(recorder: VideoRecorder<'a, Message>) -> Self {
        use iced::widget::{button, column, container, row, text, Space};
        use iced::{Background, Border, Color, Length};

        let state = recorder.state;

        // Preview area placeholder
        let preview: iced::Element<'a, Message, iced::Theme> = if recorder.show_preview {
            container(
                column![
                    text("Camera Preview").size(14),
                    text("(Connect to platform camera)").size(10),
                ]
                .spacing(4)
                .align_x(iced::Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fixed(200.0))
            .center_x(Length::Fill)
            .center_y(Length::Fixed(200.0))
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                text_color: Some(Color::from_rgb(0.7, 0.7, 0.7)),
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
        } else {
            Space::with_height(0).into()
        };

        // Record button
        let record_btn: iced::Element<'a, Message, iced::Theme> = match state.state {
            RecordingState::Idle | RecordingState::Error => {
                let mut btn = button(
                    container(Space::new(Length::Fixed(24.0), Length::Fixed(24.0)))
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(Color::from_rgb(0.9, 0.2, 0.2))),
                            border: Border {
                                radius: 12.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                )
                .padding(12);
                if let Some(msg) = recorder.on_start {
                    btn = btn.on_press(msg);
                }
                btn.into()
            }
            RecordingState::Recording | RecordingState::Paused => {
                let mut btn = button(
                    container(Space::new(Length::Fixed(24.0), Length::Fixed(24.0)))
                        .style(|_theme| container::Style {
                            background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))),
                            border: Border {
                                radius: 4.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                )
                .padding(12);
                if let Some(msg) = recorder.on_stop {
                    btn = btn.on_press(msg);
                }
                btn.into()
            }
            RecordingState::Processing => {
                button(text("Processing...").size(12)).into()
            }
        };

        // Recording time
        let time_display: iced::Element<'a, Message, iced::Theme> =
            if state.state == RecordingState::Recording {
                container(text(format!("REC {}", state.duration_str())).size(14))
                    .padding(4)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.9, 0.2, 0.2, 0.8))),
                        text_color: Some(Color::WHITE),
                        border: Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .into()
            } else if state.state == RecordingState::Paused {
                text(format!("PAUSED {}", state.duration_str())).size(14).into()
            } else {
                text("Ready").size(14).into()
            };

        // Toggle buttons
        let audio_btn: Option<iced::Element<'a, Message, iced::Theme>> =
            recorder.on_toggle_audio.map(|msg| {
                let label = if state.audio_enabled { "Mic ON" } else { "Mic OFF" };
                button(text(label).size(12)).on_press(msg).into()
            });

        let video_btn: Option<iced::Element<'a, Message, iced::Theme>> =
            recorder.on_toggle_video.map(|msg| {
                let label = if state.video_enabled { "Cam ON" } else { "Cam OFF" };
                button(text(label).size(12)).on_press(msg).into()
            });

        let switch_btn: Option<iced::Element<'a, Message, iced::Theme>> =
            recorder.on_switch_camera.map(|msg| {
                button(text("Switch Cam").size(12)).on_press(msg).into()
            });

        // Control row
        let mut controls: Vec<iced::Element<'a, Message, iced::Theme>> = Vec::new();
        if let Some(audio) = audio_btn {
            controls.push(audio);
        }
        controls.push(record_btn);
        if let Some(video) = video_btn {
            controls.push(video);
        }
        if let Some(switch) = switch_btn {
            controls.push(switch);
        }

        column![
            preview,
            Space::with_height(8),
            row![time_display].align_y(iced::Alignment::Center),
            Space::with_height(8),
            row(controls).spacing(16).align_y(iced::Alignment::Center),
        ]
        .spacing(4)
        .align_x(iced::Alignment::Center)
        .into()
    }
}

/// Video player controls (similar to audio but with fullscreen option).
pub struct VideoControls<'a, Message> {
    audio: AudioControls<'a, Message>,
    on_fullscreen: Option<Message>,
}

impl<'a, Message> VideoControls<'a, Message>
where
    Message: Clone,
{
    /// Create new video controls.
    pub fn new(state: &'a MediaPlayerState) -> Self {
        Self {
            audio: AudioControls::new(state),
            on_fullscreen: None,
        }
    }

    /// Set the play message.
    #[must_use]
    pub fn on_play(mut self, message: Message) -> Self {
        self.audio = self.audio.on_play(message);
        self
    }

    /// Set the pause message.
    #[must_use]
    pub fn on_pause(mut self, message: Message) -> Self {
        self.audio = self.audio.on_pause(message);
        self
    }

    /// Set the seek callback.
    #[must_use]
    pub fn on_seek<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.audio = self.audio.on_seek(f);
        self
    }

    /// Set the volume callback.
    #[must_use]
    pub fn on_volume<F>(mut self, f: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        self.audio = self.audio.on_volume(f);
        self
    }

    /// Set the mute toggle message.
    #[must_use]
    pub fn on_mute(mut self, message: Message) -> Self {
        self.audio = self.audio.on_mute(message);
        self
    }

    /// Set the fullscreen toggle message.
    #[must_use]
    pub fn on_fullscreen(mut self, message: Message) -> Self {
        self.on_fullscreen = Some(message);
        self
    }
}

impl<'a, Message> From<VideoControls<'a, Message>>
    for iced::Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(controls: VideoControls<'a, Message>) -> Self {
        use iced::widget::{button, row, text};

        let audio_controls: iced::Element<'a, Message, iced::Theme> = controls.audio.into();

        if let Some(fullscreen_msg) = controls.on_fullscreen {
            let fullscreen_btn: iced::Element<'a, Message, iced::Theme> =
                button(text("⛶").size(16)).on_press(fullscreen_msg).into();

            row![audio_controls, fullscreen_btn]
                .spacing(8)
                .align_y(iced::Alignment::Center)
                .into()
        } else {
            audio_controls
        }
    }
}
