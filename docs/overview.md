Overview
========

`iced-plus` is a collection of crates that round out [`iced`](https://github.com/iced-rs/iced) for desktop-class apps. Rather than bolting CSS semantics onto iced, we lean into Rust-friendly primitives—typed design tokens, explicit component variants, predictable layouts, and consistent platform services—so teams can treat iced as a full framework.

Goals
-----

- **Design-system ready**: reusable tokens (color, typography, spacing, radii, elevation) plus preset themes that map cleanly to iced widgets.
- **Layouts that scale**: higher-level shells, split panes, responsive stacks, overlay managers, and docking-like regions to keep complex tools manageable.
- **Pre-built widgets**: ergonomic components and extras covering common desktop patterns (forms, tables, toasts, palettes, inspectors).
- **Platform parity**: consistent tray, notification, hotkey, window, and IPC APIs across macOS, Windows, and Linux (Wayland + X11).
- **Pragmatic scope**: deliver the pieces with the most impact first; advanced multi-window or animation tooling comes after the core experience is solid.

Non-goals
---------

- Re-creating a CSS cascade or runtime class strings.
- Promising browser-grade typography or layout engines.
- Depending on nightly Rust or unstable iced forks.

Roadmap snapshot
----------------

1. **Design system core** *(in progress)*
   - Token definitions, theme bridge, typography + color presets, style traits.
2. **Platform crate** *(in progress)*
   - Tray, notifications, global hotkeys, window lifecycle helpers, IPC building blocks.
3. **Layouts & overlays**
   - `AppShell`, stacks/grids/split panes, overlay manager, responsive helpers.
4. **Components crate**
   - Buttons, inputs, surfaces, feedback widgets, data displays, overlay primitives, bundled patterns, and showcase integration.
5. **Extras crate**
   - Command palette, inspector/docking utilities, console/log views, rich dialogs.
6. **Multi-window (experimental)**
   - Multi-process orchestration via async IPC once the rest of the stack is stable.

See `docs/design-system.md`, `docs/layouts.md`, `docs/components.md`, and `docs/platform.md` for detailed plans on each pillar plus the shared kitchen-sink demo strategy.
