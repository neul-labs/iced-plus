Components & Extras
===================

`iced-plus` ships a batteries-included widget layer so teams can author feature-rich apps without hand-rolling every control. Components consume the shared design tokens + layout primitives and expose ergonomic APIs that feel native to iced.

Philosophy
----------

- **Variant-based**: each component offers semantic variants (primary, secondary, destructive, info, subtle) plus sizes; no string-based styling.
- **Builder ergonomics**: fluent builders make configuration obvious (`Button::primary("Save").size(ButtonSize::Sm)`).
- **Layout-aware**: spacing, alignment, and overlay behavior defer to layout tokens and the overlay manager, so widgets coexist cleanly.
- **Accessibility-first**: components expose focus states, keyboard interaction, and ARIA-like semantics wherever possible.
- **Composable**: low-level primitives (e.g., `Surface`, `Badge`, `IconButton`) can be mixed with high-level composites (e.g., `CommandPalette`, `InspectorPanel`).

Component families
------------------

### Inputs & controls

- Buttons (solid/outline/ghost/icon/fab), Segmented controls, Toggle group
- Text input, Text area, Password input, Code input
- Select, Combobox, Auto-complete, Slider, Range slider, Checkbox, Switch, Radio group, Stepper
- Form helpers: Label, Description, Validation message

### Feedback & status

- Alert, Toast, Snackbar, Progress bar, Spinner, Skeleton loader, Badge, Chip, Tag, Status pill
- Empty states / placeholders with icon + copy presets

### Data display

- Table (non-virtual + virtual follow-up), List, Tree, Card collection, Stat widget, Timeline, Activity feed, Code block, Log viewer

### Overlay widgets

- Modal, Drawer, Popover, Tooltip, Context menu, Floating action buttons, Notification tray
- All integrate with the overlay manager defined in `docs/layouts.md`

### Navigation & chrome

- Tabs, Breadcrumbs, Sidebar nav (collapsible sections), Top nav bar, Pagination, Command palette trigger, Search bar

### Extras

- Command palette (fuzzy search + actions)
- Inspector panel (dockable/resizable, with property groups)
- Console/log view with filtering + levels
- Notifications center, Quick actions, Onboarding checklist

API sketches
------------

```rust
let actions = HStack::new()
    .push(Button::primary("Deploy").on_press(Message::Deploy))
    .push(Button::ghost("Cancel").on_press(Message::Cancel));

let form = Form::new("Database")
    .field(
        TextInput::new("Name", &self.state.name)
            .on_input(Message::NameChanged)
            .label("Cluster Name")
            .helper("Shown in the sidebar"),
    )
    .field(
        Select::new(
            &self.state.region,
            REGION_OPTIONS,
            Message::RegionChanged,
        )
        .label("Region"),
    )
    .validation(self.state.validation_errors());
```

Roadmap
-------

1. **MVP set**: Button, Text, Heading, Input, Select, Checkbox, Switch, Card, Surface, Alert, Badge, Modal, Toast, Stack layout components.
2. **Forms & feedback**: Form builder, validation helpers, progress indicators, skeletons.
3. **Data & navigation**: Table/List/Tree, tabs, sidebar, breadcrumb.
4. **Overlay polish**: Popover, tooltip, drawer, context menu, integrated overlay stack examples.
5. **Extras**: Command palette, inspector, log console, notifications panel.

Each phase lands alongside updates to a unified “kitchen sink” demo app that exercises every new component, layout pattern, and platform hook so developers can copy working combinations.

Component catalog
-----------------

To mirror Skeleton UI’s “copy & tweak” style, iced-plus ships curated component patterns that combine tokens, layout structures, and bundles. Use this catalog as a quick reference when assembling screens.

### Token presets

| Category   | Tokens / presets                                                                                  | Notes                                                      |
|------------|---------------------------------------------------------------------------------------------------|------------------------------------------------------------|
| Colors     | `primary`, `secondary`, `accent`, `neutral`, `info`, `success`, `warning`, `destructive` × 50–900 | Includes contrast-aware text colors + alpha overlays      |
| Surfaces   | `surface`, `surface-muted`, `surface-elevated`, `surface-inverse`                                 | Maps to elevation + backdrop tokens                       |
| Typography | `display-xl`, `display-lg`, `heading-lg/md/sm`, `body-lg/md/sm`, `code`, `label`, `micro`         | Each entry stores font family, size, weight, line height  |
| Spacing    | `xxs (2)`, `xs (4)`, `sm (8)`, `md (12)`, `lg (16)`, `xl (24)`, `2xl (32)`, `3xl (48)`, `4xl (64)`| Works for margin, padding, gaps                           |
| Radii      | `none`, `xs`, `sm`, `md`, `lg`, `xl`, `full`                                                      | Aligns with component variants                            |
| Elevation  | `flat`, `raised`, `overlay`, `floating`, `modal`                                                  | Packs shadow, border, and backdrop values                 |
| Motion     | `fast (120ms)`, `base (200ms)`, `slow (320ms)`, `snappy (180ms cubic)`, `gentle (250ms ease)`     | Future-friendly for animated components                   |

### Layout patterns

| Pattern           | Description                                                                | Key primitives                         |
|-------------------|----------------------------------------------------------------------------|----------------------------------------|
| Dashboard shell   | Header + collapsible sidebar + scrollable content + toast slot             | `AppShell`, `Sidebar`, `HStack`, `ToastStack` |
| Inspector layout  | Primary content + resizable inspector drawer                               | `Split`, `Drawer`, `Surface`           |
| Detail flyout     | List/detail split with responsive collapse                                 | `ResponsiveStack`, `List`, `Card`      |
| Command workspace | Full-screen content with command palette overlay + status bar              | `AppShell`, `CommandPalette`, `StatusBar` |
| Modal workflow    | Dimmed background + centered modal + stepper controls                      | `OverlayManager`, `Modal`, `Stepper`   |
| Notification HUD  | Floating action button triggering stacked toasts                           | `Fab`, `ToastStack`, `OverlayManager`  |
| Empty state page  | Centered illustration/text/action layout                                   | `VStack`, `Surface`, `ButtonSet`       |

### Component bundles

| Bundle            | Components included                                                                | Usage tip                                     |
|-------------------|-------------------------------------------------------------------------------------|-----------------------------------------------|
| Auth form         | `Heading`, `Form`, `TextInput`, `PasswordInput`, `Checkbox`, `Button::primary`      | Combine with `Card` + `Surface::raised`       |
| CRUD table        | `Toolbar`, `SearchInput`, `Table`, `Pagination`, `Toast`                            | Pair with `CommandPalette` for quick actions  |
| Settings panel    | `SidebarNav`, `FormSection`, `Switch`, `Select`, `AlertInline`                      | Persist state with layout helpers             |
| Activity feed     | `Timeline`, `Badge`, `Tag`, `Popover`                                               | Use color tokens for status variants          |
| Command palette   | `CommandPalette`, `List`, `SearchInput`, `KeyboardHint`                             | Trigger via platform hotkey + overlay manager |
| Log console       | `Tabs`, `LogView`, `FilterChips`, `StatusBadge`                                     | Hook into platform notifications for errors   |
| Inspector         | `Section`, `PropertyField`, `SegmentedControl`, `Drawer`                            | Works with `Split` to dock left/right         |

### How to use

1. **Pick tokens**: choose a preset (`light`, `dark`, `solarized`) or build scales with `ThemeTokens`.
2. **Select layout pattern**: import primitives from the layout crate and start from the provided structure (examples coming in `/examples`).
3. **Mix bundles**: `iced_plus_components::bundles::*` will expose helper modules for each bundle to reduce boilerplate.
4. **Augment with platform hooks**: connect background events (tray, hotkeys, notifications) to component behaviors (e.g., show toast when sync finishes).
