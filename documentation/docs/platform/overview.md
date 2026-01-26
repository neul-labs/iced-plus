# Platform APIs

Desktop platform integration for iced-plus applications.

## Overview

The `iced_plus_platform` crate provides cross-platform APIs for desktop features:

| Feature | Description |
|---------|-------------|
| System Tray | Tray icons and menus |
| Notifications | Native OS notifications |
| Hotkeys | Global keyboard shortcuts |
| Window | Extended window management |
| Audio | Audio playback APIs |
| Recording | Audio/video recording |
| WebView | Embedded web browser |

## Installation

Platform APIs are opt-in. Enable features you need:

```toml
[dependencies]
iced_plus_platform = { version = "0.1", features = ["tray", "notifications"] }

# Or use the umbrella crate
iced_plus = { version = "0.1", features = ["platform"] }
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `tray` | System tray integration | Yes |
| `notifications` | Desktop notifications | Yes |
| `hotkeys` | Global hotkeys | No |
| `window` | Window management | No |
| `audio` | Audio playback | No |
| `recording` | Audio/video recording | No |
| `webview` | Embedded webview | No |
| `full` | All features | No |

## Platform Support

| Platform | Tray | Notifications | Hotkeys | Window |
|----------|------|---------------|---------|--------|
| Linux (X11) | Yes | Yes | Yes | Yes |
| Linux (Wayland) | Yes | Yes | Partial | Yes |
| macOS | Yes | Yes | Yes | Yes |
| Windows | Yes | Yes | Yes | Yes |

## Usage Pattern

Platform APIs follow a consistent pattern:

```rust
use iced_plus_platform::tray::{TrayIcon, TrayMenu};

// Create platform resources
let tray = TrayIcon::new("My App")
    .icon(icon_data)
    .menu(TrayMenu::new()
        .item("Show", Message::Show)
        .separator()
        .item("Quit", Message::Quit));

// Handle events in update
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::TrayEvent(event) => {
            // Handle tray interactions
        }
        // ...
    }
}
```

## Quick Links

- [System Tray](tray.md) - Tray icons and menus
- [Notifications](notifications.md) - Desktop notifications
- [WebView](webview.md) - Embedded browser
