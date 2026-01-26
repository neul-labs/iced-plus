# Responsive Layouts

iced-plus provides helpers for building adaptive interfaces that respond to window size.

## Breakpoints

Standard breakpoint tiers:

| Tier | Min Width | Typical Device |
|------|-----------|----------------|
| `XS` | 0px | Small phones |
| `SM` | 640px | Large phones |
| `MD` | 768px | Tablets |
| `LG` | 1024px | Small laptops |
| `XL` | 1280px | Desktops |
| `XXL` | 1536px | Large screens |

## ShowOn

Conditionally show content based on breakpoint:

```rust
use iced_plus_layouts::{ShowOn, BreakpointTier};

// Only show on medium and larger
ShowOn::new(sidebar)
    .min(BreakpointTier::MD)

// Only show on small screens
ShowOn::new(mobile_menu)
    .max(BreakpointTier::SM)

// Show between specific breakpoints
ShowOn::new(content)
    .min(BreakpointTier::SM)
    .max(BreakpointTier::LG)
```

## Responsive Row

A row that adapts based on screen width:

```rust
use iced_plus_layouts::{responsive_row, BreakpointTier};

responsive_row(
    children,
    |tier| match tier {
        BreakpointTier::XS | BreakpointTier::SM => 1,  // 1 column
        BreakpointTier::MD => 2,                       // 2 columns
        _ => 3,                                        // 3 columns
    }
)
```

## Custom Breakpoints

Define custom breakpoint values:

```rust
use iced_plus_layouts::Breakpoints;

let custom = Breakpoints {
    sm: 600.0,
    md: 900.0,
    lg: 1200.0,
    xl: 1500.0,
    xxl: 1800.0,
};
```

## Responsive Pattern Examples

### Sidebar Toggle

```rust
fn view(&self) -> Element<Message> {
    let sidebar = ShowOn::new(self.sidebar())
        .min(BreakpointTier::MD);

    let mobile_menu = ShowOn::new(self.mobile_menu())
        .max(BreakpointTier::SM);

    VStack::new()
        .push(mobile_menu)
        .push(
            HStack::new()
                .push(sidebar)
                .push(self.main_content())
        )
        .into()
}
```

### Adaptive Grid

```rust
fn gallery(&self) -> Element<Message> {
    Responsive::new(|tier| {
        let columns = match tier {
            BreakpointTier::XS => 1,
            BreakpointTier::SM => 2,
            BreakpointTier::MD => 3,
            _ => 4,
        };

        // Create grid with `columns` items per row
        create_grid(self.items.clone(), columns)
    })
}
```
