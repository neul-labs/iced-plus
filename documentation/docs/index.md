# iced-plus

An opinionated companion toolkit for building ambitious desktop apps with [iced](https://github.com/iced-rs/iced).

## What is iced-plus?

iced-plus makes iced feel like a complete framework by providing batteries that cover real-world desktop needs:

- **Design Tokens** - Tailwind/Chakra-inspired design tokens with preset light/dark themes
- **Theme Bridge** - Adapters that wire tokens into iced's styling system
- **Layouts** - Type-safe layout primitives: stacks, shells, split panes, responsive rules
- **Components** - 30+ pre-built widgets with ergonomic builder APIs
- **Platform APIs** - Unified APIs for system tray, notifications, hotkeys, and window management

## Quick Start

Add iced-plus to your `Cargo.toml`:

```toml
[dependencies]
iced_plus = "0.1"
```

Then use it in your application:

```rust
use iced_plus::prelude::*;

// Use design tokens for consistent styling
let theme = AppTheme::light();

// Build layouts with stacks
let content = VStack::new()
    .spacing(16.0)
    .push(Button::primary("Click me"))
    .push(TextInput::default("Enter text..."));
```

## Feature Status

| Feature | Status | Notes |
|---------|--------|-------|
| Design Tokens | **Stable** | Full token system with light/dark presets |
| Theme Bridge | **Stable** | Adapts tokens to iced styling |
| Layouts | **Stable** | HStack, VStack, AppShell, Split, Responsive |
| Core Components | **Stable** | Button, Text, Input, Card, Badge, etc. |
| Audio Controls UI | **Stable** | UI controls only |
| Video Controls UI | **Stable** | Audio controls + fullscreen button |
| Audio/Video Player | **Not Implemented** | UI only - requires manual backend integration |
| Audio/Video Recorder | **Experimental** | UI ready, needs platform mic/camera integration |
| WebView | **Experimental** | System browser works, embedded needs `wry` feature |
| System Tray | **Stable** | Cross-platform tray icon and menu |
| Notifications | **Stable** | Native OS notifications |
| Hotkeys | **Stable** | Global hotkey registration |

!!! warning "Media Playback"
    Audio and video playback is **not built-in**. The library provides UI controls and state management, but actual media playback must be integrated manually using libraries like [rodio](https://crates.io/crates/rodio) for audio. See the [Media Components](components/media.md) page for details.

## Crates

| Crate | Description |
|-------|-------------|
| `iced_plus` | Umbrella crate that re-exports all sub-crates |
| `iced_plus_tokens` | Design tokens: colors, typography, spacing, elevation |
| `iced_plus_theme` | Theme bridge adapting tokens to iced styling |
| `iced_plus_layouts` | Layout primitives: stacks, shells, split panes |
| `iced_plus_components` | 30+ pre-built UI components |
| `iced_plus_platform` | Desktop platform APIs: tray, notifications, hotkeys |

## Platform Support

| Platform | Status |
|----------|--------|
| Linux (X11) | Supported |
| Linux (Wayland) | Supported |
| macOS | Supported |
| Windows | Supported |
