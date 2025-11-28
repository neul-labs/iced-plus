Design System
=============

The design-system workstream gives iced apps consistent visuals without string-based styling. It spans three layers: tokens, theme bridge, and components.

1. Tokens
---------

Typed tokens capture the primitives shared across crates.

- **Colors**: palettes + scales (50–900) for primary/secondary/neutral + semantic roles (success, warning, destructive).
- **Typography**: named text styles (body-sm/md/lg, heading-sm/md/lg, code, label) with fonts, sizes, weights, and line heights.
- **Spacing**: modular scale (2/4/6/8/12/16/24/32/48/64) plus layout gutters.
- **Radii & borders**: XS → Full radii + border thickness preset.
- **Elevation & shadow**: layered elevations for surfaces (card, popover, modal).
- **Timing & motion**: duration + curve presets to reuse in future animations.

Tokens live in `iced_plus_tokens` (name TBD) and expose:

```rust
pub struct ThemeTokens {
    pub colors: ColorTokens,
    pub typography: TypographyTokens,
    pub spacing: SpacingScale,
    pub radii: RadiusScale,
    pub elevation: ElevationScale,
    pub motion: MotionScale,
}

pub struct ThemePreset {
    pub id: &'static str,
    pub name: &'static str,
    pub tokens: ThemeTokens,
}
```

2. Theme bridge
---------------

The theme bridge adapts tokens into iced-friendly styling.

- `AppTheme` wraps `ThemeTokens` and implements iced `Theme` plus widget `StyleSheet`s.
- Style traits resolve variants/sizes (e.g., `ButtonVariant::Primary`, `ButtonSize::Lg`) into `Appearance`.
- Per-component trait hooks allow local overrides:

```rust
pub trait ButtonThemeExt {
    fn button_tokens(&self) -> &ButtonTokens;
}

impl ButtonThemeExt for AppTheme { ... }
```

Consumers either pass `&AppTheme` directly or use helpers like `ThemeContext::provide(app_theme, |ctx| { ... })`.

3. Component integration
------------------------

`iced_plus_components` consumes `AppTheme` to render widgets, layouts, and extras that stay in sync with tokens. See `docs/components.md` for the detailed catalog and kitchen-sink demo strategy. The key idea: every component receives a reference to the current `AppTheme`, picks a variant/size, and defers spacing/elevation values to the shared tokens. Layout primitives live in the dedicated layout crate (`docs/layouts.md`) but follow the same token-driven approach.

Usage pattern
-------------

```rust
use iced_plus_components::prelude::*;
use iced_plus_theme::{AppTheme, presets};

struct App {
    theme: AppTheme,
}

impl App {
    fn view(&self) -> Element<Message> {
        VStack::new()
            .spacing(self.theme.spacing().lg)
            .push(Heading::new("Dashboard"))
            .push(
                HStack::new()
                    .push(Button::primary("Sync").on_press(Message::Sync))
                    .push(Button::ghost("Cancel").on_press(Message::Cancel)),
            )
            .into()
    }
}
```

Roadmap
-------

1. Ship token crate + default light/dark presets.
2. Implement `AppTheme` + core widget style traits (button, container, text input, scrollable).
3. Release MVP components (button, text, input, card, alert, modal, toast, stack layout).
4. Add responsive helpers + split panes, overlay manager integration.
5. Expand to data-heavy widgets and extras like command palette & inspector shells.
