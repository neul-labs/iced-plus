# Media Components

!!! warning "Important: UI Controls Only"
    **iced-plus media components provide UI controls and state management only.**

    Actual audio/video playback is **not built-in** and must be integrated manually using external libraries. This is an intentional design choice to maintain portability and flexibility.

## Overview

| Component | Description | Status |
|-----------|-------------|--------|
| `AudioControls` | Play/pause, seek, volume controls | UI Ready |
| `VideoControls` | Audio controls + fullscreen button | UI Ready |
| `MediaPlayerState` | Playback state management | Ready |
| `AudioRecorder` | Recording controls with level meter | UI Ready |
| `VideoRecorder` | Video recording controls | UI Ready |
| `RecorderState` | Recording state management | Ready |

## Audio Controls

UI component for audio playback:

```rust
use iced_plus_components::{AudioControls, MediaPlayerState};

struct App {
    player: MediaPlayerState,
}

fn view(&self) -> Element<Message> {
    AudioControls::new(&self.player)
        .on_play(Message::Play)
        .on_pause(Message::Pause)
        .on_seek(Message::Seek)
        .on_volume(Message::Volume)
        .on_mute(Message::Mute)
        .into()
}
```

### Features

- Play/pause button
- Seek slider with time display
- Volume slider
- Mute toggle
- Current time / duration display

## Video Controls

Extends AudioControls with video-specific features:

```rust
use iced_plus_components::VideoControls;

VideoControls::new(&self.player)
    .on_play(Message::Play)
    .on_pause(Message::Pause)
    .on_seek(Message::Seek)
    .on_fullscreen(Message::Fullscreen)
    .into()
```

## MediaPlayerState

State management for playback:

```rust
use iced_plus_components::{MediaPlayerState, PlaybackState};

let mut player = MediaPlayerState::new();

// Control methods
player.play();
player.pause();
player.seek(Duration::from_secs(30));
player.set_volume(0.8);
player.toggle_mute();

// State queries
let is_playing = player.state() == PlaybackState::Playing;
let position = player.position();
let duration = player.duration();
let progress = player.progress(); // 0.0 - 1.0

// Time formatting
let time_str = MediaPlayerState::format_time(position); // "1:23" or "1:23:45"
```

## Backend Integration

You must connect the UI controls to an actual playback library. Here's an example using [rodio](https://crates.io/crates/rodio):

```rust
use rodio::{Decoder, OutputStream, Sink};
use iced_plus_components::{AudioControls, MediaPlayerState};

struct App {
    player: MediaPlayerState,
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Play => {
                self.player.play();
                if let Some(sink) = &self.sink {
                    sink.play();
                }
            }
            Message::Pause => {
                self.player.pause();
                if let Some(sink) = &self.sink {
                    sink.pause();
                }
            }
            Message::Volume(vol) => {
                self.player.set_volume(vol);
                if let Some(sink) = &self.sink {
                    sink.set_volume(vol);
                }
            }
            Message::LoadFile(path) => {
                // Load audio file with rodio
                let (stream, handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&handle).unwrap();
                let file = std::fs::File::open(path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                sink.append(source);

                self.sink = Some(sink);
                self._stream = Some(stream);
            }
            // ... handle other messages
        }
        Task::none()
    }
}
```

## Recording Components

### AudioRecorder

```rust
use iced_plus_components::{AudioRecorder, RecorderState};

struct App {
    recorder: RecorderState,
}

fn view(&self) -> Element<Message> {
    AudioRecorder::new(&self.recorder)
        .on_start(Message::StartRecording)
        .on_stop(Message::StopRecording)
        .on_pause(Message::PauseRecording)
        .audio_level(self.current_level) // 0.0 - 1.0
        .into()
}
```

### VideoRecorder

```rust
use iced_plus_components::VideoRecorder;

VideoRecorder::new(&self.recorder)
    .on_start(Message::StartRecording)
    .on_stop(Message::StopRecording)
    .preview(self.camera_frame.as_ref()) // Optional preview image
    .into()
```

## Example: Kitchen Sink

The [kitchen sink example](https://github.com/neul-labs/iced-plus/tree/main/examples/kitchen_sink) includes working implementations:

- `audio_backend.rs` - Rodio integration for audio playback
- `webcam_backend.rs` - Nokhwa integration for camera capture

Run it with:

```bash
cargo run -p kitchen_sink --features audio,webcam
```

## Recommended Libraries

| Use Case | Library |
|----------|---------|
| Audio playback | [rodio](https://crates.io/crates/rodio) |
| Low-level audio | [cpal](https://crates.io/crates/cpal) |
| Webcam capture | [nokhwa](https://crates.io/crates/nokhwa) |
| Video playback | Custom implementation required |
