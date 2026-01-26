# Getting Started

This guide will help you set up iced-plus in your project and build your first application.

## Prerequisites

- Rust 1.75 or later
- A working iced development environment

## Installation

### Using the umbrella crate (recommended)

Add iced-plus to your `Cargo.toml`:

```toml
[dependencies]
iced = "0.13"
iced_plus = "0.1"
```

### Using individual crates

For more control, depend on only what you need:

```toml
[dependencies]
iced = "0.13"
iced_plus_tokens = "0.1"      # Design tokens only
iced_plus_theme = "0.1"       # Theme bridge
iced_plus_layouts = "0.1"     # Layout primitives
iced_plus_components = "0.1"  # UI components
iced_plus_platform = "0.1"    # Desktop platform APIs
```

## Feature Flags

The umbrella crate (`iced_plus`) supports these feature flags:

| Feature | Default | Description |
|---------|---------|-------------|
| `tokens` | Yes | Design tokens and color scales |
| `theme` | Yes | Theme bridge adapting tokens to iced |
| `layouts` | Yes | Layout primitives (stacks, shells, responsive) |
| `components` | Yes | Pre-built UI components |
| `platform` | No | Desktop platform APIs (opt-in) |
| `full` | No | All features including platform APIs |

Example with all features:

```toml
[dependencies]
iced_plus = { version = "0.1", features = ["full"] }
```

## Your First App

Here's a minimal example using iced-plus:

```rust
use iced::widget::container;
use iced::{Element, Task};
use iced_plus::prelude::*;

fn main() -> iced::Result {
    iced::application("My App", App::update, App::view)
        .run()
}

struct App {
    count: i32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

impl Default for App {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let content = VStack::new()
            .spacing(16.0)
            .push(Heading::h1(format!("Count: {}", self.count)))
            .push(
                HStack::new()
                    .spacing(8.0)
                    .push(Button::primary("+").on_press(Message::Increment))
                    .push(Button::secondary("-").on_press(Message::Decrement))
            );

        container(content)
            .center_x(iced::Fill)
            .center_y(iced::Fill)
            .into()
    }
}
```

## Next Steps

- Learn about [Design Tokens](design-system/tokens.md)
- Explore [Layout Primitives](layouts/stacks.md)
- Browse the [Component Library](components/overview.md)
- Check out the [Kitchen Sink Example](examples/kitchen-sink.md)
