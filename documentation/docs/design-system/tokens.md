# Design Tokens

Design tokens are the foundational building blocks of a consistent design system. iced-plus provides a comprehensive token system inspired by Tailwind CSS and Chakra UI.

## Token Categories

### Colors

The color system provides semantic color scales and palettes:

```rust
use iced_plus_tokens::presets;

let theme = presets::light();

// Access color scales
let primary = theme.tokens.colors.primary.s500;
let gray = theme.tokens.colors.gray.s200;

// Semantic colors
let text = theme.tokens.colors.text;
let background = theme.tokens.colors.background;
```

#### Color Scales

Each color has 11 shades (s50 through s950):

| Shade | Usage |
|-------|-------|
| s50 | Lightest backgrounds |
| s100-s200 | Subtle backgrounds |
| s300-s400 | Borders and dividers |
| s500 | Base/primary shade |
| s600-s700 | Hover states |
| s800-s900 | Text on light backgrounds |
| s950 | Darkest text |

### Typography

Typography tokens define text styles:

```rust
let theme = presets::light();

// Access typography scale
let heading = theme.tokens.typography.h1;
let body = theme.tokens.typography.body;

// Properties
let font_size = heading.size;      // f32
let line_height = heading.height;  // f32
let weight = heading.weight;       // FontWeight
```

Available styles: `h1`, `h2`, `h3`, `h4`, `body`, `small`, `caption`

### Spacing

Modular spacing scale for margins, padding, and gaps:

```rust
let theme = presets::light();

// Access spacing scale
let sm = theme.tokens.spacing.sm();   // 8.0
let md = theme.tokens.spacing.md();   // 12.0
let lg = theme.tokens.spacing.lg();   // 16.0
let xl = theme.tokens.spacing.xl();   // 24.0
```

| Size | Value |
|------|-------|
| `xs` | 4.0 |
| `sm` | 8.0 |
| `md` | 12.0 |
| `lg` | 16.0 |
| `xl` | 24.0 |
| `2xl` | 32.0 |
| `3xl` | 48.0 |
| `4xl` | 64.0 |

### Radius

Border radius presets:

```rust
let theme = presets::light();

let rounded = theme.tokens.radius.md;    // 8.0
let pill = theme.tokens.radius.full;     // 9999.0
```

| Size | Value |
|------|-------|
| `none` | 0.0 |
| `sm` | 4.0 |
| `md` | 8.0 |
| `lg` | 12.0 |
| `xl` | 16.0 |
| `full` | 9999.0 |

### Elevation

Shadow and depth definitions for layered interfaces:

```rust
let theme = presets::light();

let shadow = theme.tokens.elevation.md;
```

| Level | Description |
|-------|-------------|
| `none` | No shadow |
| `sm` | Subtle shadow |
| `md` | Standard shadow |
| `lg` | Prominent shadow |
| `xl` | Heavy shadow |

### Motion

Animation duration and easing presets:

```rust
let theme = presets::light();

let fast = theme.tokens.motion.fast;       // 150ms
let normal = theme.tokens.motion.normal;   // 300ms
```

## Theme Presets

iced-plus includes built-in light and dark themes:

```rust
use iced_plus_tokens::presets;

let light = presets::light();
let dark = presets::dark();
```

## Custom Tokens

You can create custom token sets:

```rust
use iced_plus_tokens::{ThemeTokens, ColorPalette, SpacingScale};

let custom_tokens = ThemeTokens {
    colors: ColorPalette::custom(/* ... */),
    spacing: SpacingScale::default(),
    // ...
};
```
