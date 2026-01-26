# Buttons

Type-safe button components with variants and sizes.

## Basic Usage

```rust
use iced_plus_components::Button;

Button::primary("Click me")
    .on_press(Message::Clicked)
```

## Variants

### Primary

Main call-to-action buttons:

```rust
Button::primary("Save Changes")
```

### Secondary

Secondary actions:

```rust
Button::secondary("Cancel")
```

### Ghost

Minimal, transparent background:

```rust
Button::ghost("Learn More")
```

### Outline

Bordered style:

```rust
Button::outline("Options")
```

### Destructive

Dangerous or irreversible actions:

```rust
Button::destructive("Delete")
```

## Sizes

Buttons support multiple sizes:

```rust
// Using methods
Button::primary("Small").small()
Button::primary("Medium")           // default
Button::primary("Large").large()

// Extra sizes
Button::primary("XS").extra_small()
Button::primary("XL").extra_large()
```

| Size | Padding | Font |
|------|---------|------|
| Extra Small | Compact | 12px |
| Small | Tight | 14px |
| Medium | Normal | 16px |
| Large | Generous | 18px |
| Extra Large | Spacious | 20px |

## States

### Disabled

```rust
Button::primary("Submit")
    .disabled(true)
```

### Loading

```rust
Button::primary("Saving...")
    .loading(true)
```

## With Icons

```rust
use iced_plus_components::{Button, IconName, icon};

Button::primary("")
    .push(icon(IconName::Plus))
    .push(" Add Item")
```

## Full Width

```rust
Button::primary("Full Width")
    .width(Length::Fill)
```

## Type Safety

Button types encode variant and size at compile time:

```rust
// Full type annotation
let btn: Button<'_, Primary, Medium, Message> = Button::primary("Save");

// Type changes with methods
let btn: Button<'_, Primary, Small, Message> = Button::primary("Save").small();
let btn: Button<'_, Destructive, Large, Message> = Button::destructive("Delete").large();
```

## Event Handling

```rust
Button::primary("Submit")
    .on_press(Message::Submit)
    .on_press_if(self.is_valid, Message::Submit)  // Conditional
```
