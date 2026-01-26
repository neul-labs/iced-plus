# System Tray

System tray (notification area) integration for desktop applications.

## Basic Usage

```rust
use iced_plus_platform::tray::{TrayIcon, TrayMenu, TrayMenuItem};

// Create tray icon with menu
let tray = TrayIcon::new("My App")
    .tooltip("My Application")
    .icon(icon_bytes)
    .menu(
        TrayMenu::new()
            .item("Show Window", Message::Show)
            .item("Settings", Message::OpenSettings)
            .separator()
            .item("Quit", Message::Quit)
    );
```

## TrayIcon

The main tray component:

```rust
TrayIcon::new("App Name")
    .tooltip("Hover text")           // Tooltip on hover
    .icon(icon_data)                 // Icon bytes (PNG recommended)
    .menu(menu)                      // Context menu
    .on_click(Message::TrayClicked)  // Left click handler
```

### Icon Format

Icons should be PNG format. Recommended sizes:
- Windows: 16x16 or 32x32
- macOS: 22x22 (template image)
- Linux: 24x24 or 48x48

```rust
let icon_bytes = include_bytes!("../assets/tray-icon.png");
TrayIcon::new("App").icon(icon_bytes)
```

## TrayMenu

Context menu for the tray icon:

```rust
TrayMenu::new()
    .item("Label", Message::Action)       // Simple item
    .item_with_icon("Open", icon, msg)    // Item with icon
    .separator()                          // Divider line
    .submenu("More", submenu)             // Nested menu
    .checkbox("Enabled", checked, msg)    // Checkbox item
```

### Menu Items

```rust
// Simple item
TrayMenuItem::new("Open", Message::Open)

// With keyboard shortcut hint
TrayMenuItem::new("Quit", Message::Quit)
    .shortcut("Ctrl+Q")

// Disabled item
TrayMenuItem::new("Unavailable", Message::None)
    .disabled(true)
```

## Event Handling

```rust
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::TrayClicked => {
            // Toggle window visibility
            self.window_visible = !self.window_visible;
        }
        Message::Show => {
            self.window_visible = true;
        }
        Message::Quit => {
            return iced::exit();
        }
        // ...
    }
    Task::none()
}
```

## Dynamic Updates

Update tray at runtime:

```rust
// Update tooltip
self.tray.set_tooltip("New Status");

// Update icon
self.tray.set_icon(new_icon_bytes);

// Update menu
self.tray.set_menu(new_menu);
```

## Platform Notes

### macOS

- Use template images (black with transparency) for proper dark mode support
- Icons appear in the menu bar

### Windows

- Icons appear in the notification area
- May need to be "promoted" from overflow area

### Linux

- Requires a system tray implementation (e.g., KDE, GNOME with extensions)
- Some desktop environments may not support system trays
