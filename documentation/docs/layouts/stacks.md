# Stack Layouts

Stack layouts arrange children along a single axis with consistent spacing.

## HStack

Horizontal stack arranges children from left to right:

```rust
use iced_plus_layouts::HStack;
use iced::alignment;

let row = HStack::new()
    .spacing(16.0)
    .align(alignment::Vertical::Center)
    .push(text("Left"))
    .push(text("Right"));
```

### Type-Safe Alignment

HStack only accepts vertical cross-axis alignment:

```rust
// Correct: Vertical alignment on horizontal stack
HStack::new().align(alignment::Vertical::Center)

// Compiler error: Horizontal alignment not allowed on HStack
// HStack::new().align(alignment::Horizontal::Center)
```

## VStack

Vertical stack arranges children from top to bottom:

```rust
use iced_plus_layouts::VStack;
use iced::alignment;

let column = VStack::new()
    .spacing(16.0)
    .align(alignment::Horizontal::Center)
    .push(text("Top"))
    .push(text("Bottom"));
```

### Type-Safe Alignment

VStack only accepts horizontal cross-axis alignment:

```rust
// Correct: Horizontal alignment on vertical stack
VStack::new().align(alignment::Horizontal::Center)

// Compiler error: Vertical alignment not allowed on VStack
// VStack::new().align(alignment::Vertical::Center)
```

## Common Methods

Both stacks share these methods:

| Method | Description |
|--------|-------------|
| `spacing(f32)` | Gap between children |
| `padding(f32)` | Inner padding |
| `width(Length)` | Stack width |
| `height(Length)` | Stack height |
| `push(impl Into<Element>)` | Add a child |
| `push_maybe(Option<impl Into<Element>>)` | Conditionally add |

## Examples

### Form Layout

```rust
VStack::new()
    .spacing(16.0)
    .push(TextInput::default("Name"))
    .push(TextInput::default("Email"))
    .push(
        HStack::new()
            .spacing(8.0)
            .push(Button::secondary("Cancel"))
            .push(Button::primary("Submit"))
    )
```

### Card with Actions

```rust
Card::new(
    VStack::new()
        .spacing(12.0)
        .push(Heading::h3("Card Title"))
        .push(Text::body("Card description goes here."))
        .push(
            HStack::new()
                .spacing(8.0)
                .push(Button::ghost("Learn More"))
                .push(Button::primary("Action"))
        )
)
```

### Centered Content

```rust
VStack::new()
    .width(Length::Fill)
    .height(Length::Fill)
    .align(alignment::Horizontal::Center)
    .push(Spinner::default())
    .push(Text::body("Loading..."))
```
