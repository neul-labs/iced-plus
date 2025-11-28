Layouts & Overlays
==================

Building ambitious desktop tooling usually means juggling navigation, inspector panes, dialogs, and transient overlays inside a single OS window. The `iced-plus` layout stack handles that with structured primitives rather than ad-hoc containers.

Principles
----------

- **Single-window first**: keep complex workflows inside one window via panes, drawers, and modals; multi-window comes later.
- **Deterministic layers**: define explicit z-order buckets (content, chrome, floating controls, blocking overlays, notifications) to avoid “widget fights.”
- **Responsive scaling**: expose breakpoint-aware APIs so layouts adapt to laptop vs. ultrawide vs. portrait monitors.
- **Stateful panes**: remember user-adjusted sizes (split ratios, collapsed panels) and provide persistence helpers.

Core pieces
-----------

### Layout primitives

- `HStack` / `VStack`: flex-like stacks with spacing, alignment, and wrap options.
- `Grid`: CSS-like grid with track definitions (fixed / fr units) and gap tokens.
- `Split`: resizable split panes (horizontal/vertical) with min/max constraints and persisted ratios.
- `Surface` / `Card`: elevation-aware containers for grouping content.
- `Spacer`, `Divider`, `Section`: utility widgets for breathing room and labeling.

### App shell

`AppShell` orchestrates:

- Top bar (title, actions), sidebar (nav tree, collapsible), content area, detail pane, bottom status bar.
- Integrated overlay slots (e.g., command palette, modal, toast stack).
- Responsive behavior: collapse sidebar on narrow widths, auto-hide detail panel, convert top bar actions into overflow menu.

### Overlay manager

- Maintains layered stacks:
  1. Base content
  2. Persistent chrome (fab, floating buttons)
  3. Drawers / popovers
  4. Modals (blocking)
  5. Toasts / notifications
- Handles focus trapping & inert background for blocking overlays.
- Provides API like:

```rust
let overlay = overlay::Manager::new()
    .with_modal(modal_view)
    .with_toasts(toasts.iter());

overlay.compose(app_content)
```

### Responsive helpers

- `Breakpoints` struct (xs/sm/md/lg/xl) with customizable pixel values.
- `ResponsiveStack` chooses orientation/spacing per breakpoint.
- `ShowOn` / `HideOn` wrappers for conditional rendering.

Persistence & integration
-------------------------

- Layout components expose serialization-friendly state (e.g., `SplitState`, `SidebarState`) so apps can store user preferences in settings files.
- Platform crate’s window service can hook into the same persistence to restore window geometry plus layout state.

Roadmap
-------

1. Release `HStack`, `VStack`, `Surface`, `Card`, `Section`, `Divider`.
2. Add `Split` with persisted state + example.
3. Ship `AppShell` MVP (top bar + sidebar + overlay slot) and responsive helpers.
4. Implement overlay manager + modal/toast/drawer integration.
5. Expand to docking-like behaviors (tabbed panes, detachable inspectors) after core experience is stable.
