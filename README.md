# iced-plus

[![Crates.io](https://img.shields.io/crates/v/iced_plus.svg)](https://crates.io/crates/iced_plus)
[![Documentation](https://docs.rs/iced_plus/badge.svg)](https://docs.rs/iced_plus)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/neul-labs/iced-plus/actions/workflows/ci.yml/badge.svg)](https://github.com/neul-labs/iced-plus/actions/workflows/ci.yml)

An opinionated companion toolkit for building ambitious desktop apps with [iced](https://github.com/iced-rs/iced). The goal is to make iced feel like a complete framework by pairing it with batteries that cover real-world desktop needs—cohesive styling primitives, scalable layouts, pre-built widgets, and consistent system integration.

## Features

- **Design Tokens**: Tailwind/Chakra-inspired design tokens with preset light/dark themes
- **Theme Bridge**: Adapters that wire tokens into iced `Theme` + widget `StyleSheet`s
- **Layouts**: Shells, split panes, stacks (HStack/VStack), overlay managers, and responsive rules
- **Components**: 30+ pre-built widgets including buttons, inputs, cards, modals, tabs, and more
- **Platform Integration**: Unified APIs for system tray, notifications, hotkeys, and window management

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
iced_plus = "0.1"
```

Or pick individual crates:

```toml
[dependencies]
iced_plus_tokens = "0.1"      # Design tokens only
iced_plus_theme = "0.1"       # Theme bridge
iced_plus_layouts = "0.1"     # Layout primitives
iced_plus_components = "0.1"  # UI components
iced_plus_platform = "0.1"    # Desktop platform APIs
```

## Quick Start

```rust
use iced_plus::prelude::*;

// Use design tokens for consistent styling
let theme = Theme::light();

// Build layouts with stacks
let content = VStack::new()
    .push(Button::primary("Click me"))
    .push(TextInput::new("Enter text..."))
    .spacing(Spacing::md());
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
| Audio/Video Player | **Not Implemented** | UI only - requires manual backend integration (see [rodio example](examples/kitchen_sink/src/audio_backend.rs)) |
| Audio/Video Recorder | **Experimental** | UI ready, needs platform mic/camera integration |
| WebView | **Experimental** | System browser works, embedded needs `wry` feature |
| System Tray | **Stable** | Cross-platform tray icon and menu |
| Notifications | **Stable** | Native OS notifications |
| Hotkeys | **Stable** | Global hotkey registration |

> **Note**: Audio and video playback is not built-in. The library provides UI controls and state management, but actual media playback must be integrated manually using libraries like [rodio](https://crates.io/crates/rodio) for audio. See the [kitchen sink example](examples/kitchen_sink/) for reference implementations.

## Crates

| Crate | Description |
|-------|-------------|
| [`iced_plus`](https://crates.io/crates/iced_plus) | Umbrella crate that re-exports all sub-crates |
| [`iced_plus_tokens`](https://crates.io/crates/iced_plus_tokens) | Design tokens: colors, typography, spacing, elevation |
| [`iced_plus_theme`](https://crates.io/crates/iced_plus_theme) | Theme bridge adapting tokens to iced styling |
| [`iced_plus_layouts`](https://crates.io/crates/iced_plus_layouts) | Layout primitives: stacks, shells, split panes |
| [`iced_plus_components`](https://crates.io/crates/iced_plus_components) | 30+ pre-built UI components |
| [`iced_plus_platform`](https://crates.io/crates/iced_plus_platform) | Desktop platform APIs: tray, notifications, hotkeys |

## Documentation

- [User Guide](https://docs.neullabs.com/iced-plus) - Comprehensive documentation
- [API Reference](https://docs.rs/iced_plus) - Generated Rust docs
- [Examples](examples/) - Working code examples

## Platform Support

| Platform | Status |
|----------|--------|
| Linux (X11) | Supported |
| Linux (Wayland) | Supported |
| macOS | Supported |
| Windows | Supported |

## Examples

Run the kitchen sink demo to see all features:

```bash
cargo run -p kitchen_sink

# With optional features
cargo run -p kitchen_sink --features audio,webcam,webview
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
