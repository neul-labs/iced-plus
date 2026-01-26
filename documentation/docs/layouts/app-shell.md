# App Shell

`AppShell` provides a flexible application layout with optional sidebar, header, and main content areas.

## Basic Usage

```rust
use iced_plus_layouts::AppShell;

AppShell::new()
    .sidebar(sidebar_content)
    .header(header_content)
    .main(main_content)
```

## Layout Regions

### Sidebar

Fixed-width panel on the left:

```rust
AppShell::new()
    .sidebar(
        VStack::new()
            .width(Length::Fixed(250.0))
            .push(logo)
            .push(nav_items)
    )
```

### Header

Full-width bar at the top:

```rust
AppShell::new()
    .header(
        HStack::new()
            .padding(16.0)
            .push(title)
            .push(Space::with_width(Length::Fill))
            .push(user_menu)
    )
```

### Main Content

The primary content area fills remaining space:

```rust
AppShell::new()
    .main(
        container(page_content)
            .padding(24.0)
    )
```

## Complete Example

```rust
use iced_plus_layouts::{AppShell, VStack, HStack};
use iced_plus_components::{Button, Heading, Text};

fn view(&self) -> Element<Message> {
    let sidebar = VStack::new()
        .width(Length::Fixed(240.0))
        .padding(16.0)
        .push(Heading::h2("My App"))
        .push(Button::ghost("Dashboard").on_press(Message::Nav(Page::Dashboard)))
        .push(Button::ghost("Settings").on_press(Message::Nav(Page::Settings)));

    let header = HStack::new()
        .padding(16.0)
        .push(Text::body("Welcome back"))
        .push(Space::with_width(Length::Fill))
        .push(Button::secondary("Logout"));

    let main = container(self.current_page())
        .padding(24.0)
        .width(Length::Fill)
        .height(Length::Fill);

    AppShell::new()
        .sidebar(sidebar)
        .header(header)
        .main(main)
        .into()
}
```

## Responsive Sidebar

Combine with responsive helpers to hide sidebar on small screens:

```rust
use iced_plus_layouts::{AppShell, ShowOn, BreakpointTier};

let sidebar = ShowOn::new(sidebar_content)
    .min(BreakpointTier::MD);

AppShell::new()
    .sidebar(sidebar)
    .main(main_content)
```
