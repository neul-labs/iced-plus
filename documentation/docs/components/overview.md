# Components Overview

iced-plus provides 30+ pre-styled components with ergonomic builder APIs and type-safe variants.

## Design Principles

- **Type-safety**: Variants and sizes are encoded in the type signature
- **Zero-cost abstractions**: Style resolution happens at compile time
- **Ergonomic API**: Builder pattern with method chaining

## Component Categories

### Buttons & Inputs

| Component | Description |
|-----------|-------------|
| `Button` | Type-safe buttons with variants (Primary, Secondary, Ghost, etc.) |
| `TextInput` | Enhanced text input with label and helper text |
| `TextArea` | Multi-line text input |
| `Checkbox` | Styled checkbox |
| `Radio` | Radio buttons for single selection |
| `Switch` | Toggle switches |
| `Slider` | Range sliders |
| `Select` | Dropdown selection |

### Layout & Display

| Component | Description |
|-----------|-------------|
| `Text` | Typography component |
| `Heading` | Heading levels (h1-h6) |
| `Card` | Elevated content container |
| `Divider` | Visual separators |
| `Avatar` | User/entity avatars |
| `Skeleton` | Loading placeholders |
| `Image` | Image display with loading states |

### Feedback

| Component | Description |
|-----------|-------------|
| `Badge` | Status indicators and counts |
| `Alert` | Contextual feedback messages |
| `Progress` | Progress indicators |
| `Spinner` | Loading spinners |
| `Toast` | Toast notifications |
| `Tooltip` | Hover tooltips |

### Navigation & Overlays

| Component | Description |
|-----------|-------------|
| `Tabs` | Tab navigation |
| `Menu` | Menus and menu bars |
| `Drawer` | Side panel overlays |
| `Modal` | Modal dialogs |

### Media

| Component | Description | Status |
|-----------|-------------|--------|
| `AudioControls` | Audio player controls | UI only |
| `VideoControls` | Video player controls | UI only |
| `MediaPlayerState` | Playback state management | Ready |

!!! warning "Media Components"
    Media components provide **UI only**. Actual playback requires manual backend integration. See [Media Components](media.md) for details.

## Usage Pattern

All components follow a consistent pattern:

```rust
use iced_plus_components::Button;

// Create with variant
let btn = Button::primary("Save");

// Chain configuration
let btn = Button::primary("Save")
    .on_press(Message::Save)
    .large();

// Convert to Element
let element: Element<Message> = btn.into();
```

## Type-Safe Variants

Button variants are encoded in the type:

```rust
// Type: Button<'_, Primary, Medium, Message>
let primary = Button::primary("Save");

// Type: Button<'_, Destructive, Small, Message>
let delete = Button::destructive("Delete").small();
```

This provides compile-time guarantees and enables IDE autocompletion.
