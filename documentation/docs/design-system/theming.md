# Theming

The theme bridge connects design tokens to iced's styling system, providing type-safe theming with compile-time variant resolution.

## AppTheme

`AppTheme` is the main theme type that wraps tokens and implements iced's Catalog traits:

```rust
use iced_plus_theme::AppTheme;

// Built-in themes
let light = AppTheme::light();
let dark = AppTheme::dark();
```

## Widget Classes

iced-plus provides style classes for common widgets:

### ButtonClass

```rust
use iced_plus_theme::ButtonClass;
use iced::widget::button;

button("Click me")
    .class(ButtonClass::Primary)
```

Available variants:
- `Primary` - Main call-to-action
- `Secondary` - Secondary actions
- `Ghost` - Minimal, transparent
- `Outline` - Bordered style
- `Destructive` - Dangerous actions

### ContainerClass

```rust
use iced_plus_theme::ContainerClass;
use iced::widget::container;

container(content)
    .class(ContainerClass::Card)
```

Available variants:
- `Default` - No styling
- `Card` - Elevated card with shadow
- `Surface` - Subtle background
- `Bordered` - With border

### TextInputClass

```rust
use iced_plus_theme::TextInputClass;
use iced::widget::text_input;

text_input("Placeholder", &value)
    .class(TextInputClass::Default)
```

## Component Sizes

Widgets support standardized sizes:

```rust
use iced_plus_theme::{ComponentSize, Small, Medium, Large};

// Type-safe sizes
Button::<Primary, Small, _>::new("Small")
Button::<Primary, Medium, _>::new("Medium")
Button::<Primary, Large, _>::new("Large")

// Or using methods
Button::primary("Click").small()
Button::primary("Click").large()
```

## Status Colors

Status variants for feedback:

```rust
use iced_plus_theme::Status;

match status {
    Status::Default => { /* neutral */ }
    Status::Success => { /* green */ }
    Status::Warning => { /* yellow */ }
    Status::Error => { /* red */ }
    Status::Info => { /* blue */ }
}
```

## Converting Tokens to iced Colors

Use `token_to_iced` to convert token colors:

```rust
use iced_plus_theme::token_to_iced;
use iced_plus_tokens::presets;

let theme = presets::light();
let primary = token_to_iced(theme.tokens.colors.primary.s500);
```

## Custom Themes

Create custom themes from your tokens:

```rust
use iced_plus_theme::AppTheme;
use iced_plus_tokens::ThemePreset;

let custom_tokens = ThemePreset {
    // ... custom token values
};

let custom_theme = AppTheme::from_tokens(custom_tokens);
```
