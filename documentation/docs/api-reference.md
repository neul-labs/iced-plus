# API Reference

Complete API documentation is available on docs.rs.

## Crate Documentation

| Crate | docs.rs |
|-------|---------|
| `iced_plus` | [docs.rs/iced_plus](https://docs.rs/iced_plus) |
| `iced_plus_tokens` | [docs.rs/iced_plus_tokens](https://docs.rs/iced_plus_tokens) |
| `iced_plus_theme` | [docs.rs/iced_plus_theme](https://docs.rs/iced_plus_theme) |
| `iced_plus_layouts` | [docs.rs/iced_plus_layouts](https://docs.rs/iced_plus_layouts) |
| `iced_plus_components` | [docs.rs/iced_plus_components](https://docs.rs/iced_plus_components) |
| `iced_plus_platform` | [docs.rs/iced_plus_platform](https://docs.rs/iced_plus_platform) |

## Building Docs Locally

Generate documentation locally:

```bash
# All crates
cargo doc --workspace --no-deps --open

# Specific crate
cargo doc -p iced_plus_components --no-deps --open

# With all features
cargo doc --workspace --all-features --no-deps --open
```

## Key Types

### Design Tokens (`iced_plus_tokens`)

- `ThemeTokens` - Complete token set
- `ThemePreset` - Pre-configured theme
- `ColorPalette` - Color scales and semantics
- `SpacingScale` - Spacing values
- `TypographyScale` - Text styles

### Theme (`iced_plus_theme`)

- `AppTheme` - Main theme type
- `ButtonClass` - Button style variants
- `ContainerClass` - Container style variants
- `ComponentSize` - Size markers

### Layouts (`iced_plus_layouts`)

- `HStack` - Horizontal stack
- `VStack` - Vertical stack
- `AppShell` - Application shell
- `Modal` - Modal overlay
- `ShowOn` - Responsive visibility
- `Breakpoints` - Breakpoint definitions

### Components (`iced_plus_components`)

- `Button` - Type-safe buttons
- `TextInput` - Enhanced text input
- `Card` - Content container
- `Badge` - Status indicator
- `Alert` - Feedback message
- `Tabs` - Tab navigation
- `AudioControls` - Media player UI
- Many more...

### Platform (`iced_plus_platform`)

- `TrayIcon` - System tray
- `Notification` - Desktop notification
- `Hotkey` - Global shortcut
- `WebView` - Embedded browser

## Feature Flags Reference

### `iced_plus`

| Feature | Default | Dependencies |
|---------|---------|--------------|
| `tokens` | Yes | `iced_plus_tokens` |
| `theme` | Yes | `iced_plus_theme` + tokens |
| `layouts` | Yes | `iced_plus_layouts` |
| `components` | Yes | `iced_plus_components` + theme + layouts |
| `platform` | No | `iced_plus_platform` |
| `full` | No | All of the above |

### `iced_plus_layouts`

| Feature | Default |
|---------|---------|
| `stacks` | Yes |
| `shell` | Yes |
| `overlay` | Yes |
| `responsive` | Yes |
| `split` | No |

### `iced_plus_platform`

| Feature | Default |
|---------|---------|
| `tray` | Yes |
| `notifications` | Yes |
| `hotkeys` | No |
| `window` | No |
| `audio` | No |
| `recording` | No |
| `webview` | No |
