# Kitchen Sink Example

The kitchen sink is a comprehensive demo showcasing all iced-plus features.

## Running

```bash
# Basic (no optional features)
cargo run -p kitchen_sink

# With audio playback
cargo run -p kitchen_sink --features audio

# With webcam support
cargo run -p kitchen_sink --features webcam

# With embedded webview
cargo run -p kitchen_sink --features webview

# All features
cargo run -p kitchen_sink --features audio,webcam,webview
```

## Features Demonstrated

### Design System

- Light/dark theme toggle
- Color palette display
- Typography scale
- Spacing showcase

### Components

- **Buttons**: All variants (Primary, Secondary, Ghost, Outline, Destructive) and sizes
- **Inputs**: TextInput, TextArea, Checkbox, Radio, Switch, Slider, Select
- **Display**: Card, Badge, Avatar, Divider, Skeleton
- **Feedback**: Alert, Progress, Spinner, Toast, Tooltip
- **Navigation**: Tabs, Menu

### Layouts

- HStack and VStack
- AppShell with sidebar
- Responsive breakpoints
- Modal overlays

### Platform

- System tray (if supported)
- Notifications
- Hotkeys

### Media

- Audio controls UI with rodio backend (`--features audio`)
- Video recorder UI with nokhwa backend (`--features webcam`)
- WebView browser bar (`--features webview`)

## Project Structure

```
examples/kitchen_sink/
├── Cargo.toml
└── src/
    ├── main.rs           # Main application
    ├── audio_backend.rs  # Rodio integration
    ├── webcam_backend.rs # Nokhwa integration
    └── webview_backend.rs # Wry integration
```

## Code Highlights

### Theme Toggle

```rust
fn theme_toggle(&self) -> Element<Message> {
    let label = if self.dark_mode { "Dark" } else { "Light" };
    Switch::new(self.dark_mode)
        .label(label)
        .on_toggle(Message::ThemeToggled)
        .into()
}
```

### Component Gallery

```rust
fn buttons_section(&self) -> Element<Message> {
    VStack::new()
        .spacing(16.0)
        .push(Heading::h3("Buttons"))
        .push(
            HStack::new()
                .spacing(8.0)
                .push(Button::primary("Primary"))
                .push(Button::secondary("Secondary"))
                .push(Button::ghost("Ghost"))
                .push(Button::outline("Outline"))
                .push(Button::destructive("Destructive"))
        )
        .into()
}
```

### Audio Player Integration

```rust
fn audio_section(&self) -> Element<Message> {
    let controls = AudioControls::new(&self.media_player)
        .on_play(Message::MediaPlay)
        .on_pause(Message::MediaPause)
        .on_seek(Message::MediaSeek)
        .on_volume(Message::MediaVolume);

    VStack::new()
        .spacing(16.0)
        .push(Heading::h3("Audio Player"))
        .push(controls)
        .push(Text::caption("Connect to actual audio backend"))
        .into()
}
```

## Dependencies

Optional dependencies are conditionally compiled:

```toml
[features]
default = ["audio", "webcam"]
audio = ["rodio"]
webcam = ["nokhwa"]
webview = ["wry", "tao", "raw-window-handle"]

[dependencies]
rodio = { version = "0.19", optional = true }
nokhwa = { version = "0.10", features = ["input-native"], optional = true }
wry = { version = "0.43", optional = true }
```

## Extending

Use the kitchen sink as a reference for your own applications:

1. Copy relevant sections
2. Adapt message types
3. Wire up your own backends
4. Customize styling with tokens
