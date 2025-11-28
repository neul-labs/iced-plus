iced-plus
=========

`iced-plus` is an opinionated companion toolkit for building ambitious desktop apps with [`iced`](https://github.com/iced-rs/iced). The goal is to make iced feel like a complete framework by pairing it with batteries that cover real-world desktop needs—cohesive styling primitives, scalable layouts, pre-built widgets, and consistent system integration.

Project pillars
---------------

- **Styling & themes**: Tailwind-/Chakra-inspired design tokens, preset themes, and adapters that wire those tokens into iced `Theme` + widget `StyleSheet`s so teams can skin entire apps with predictable variants.
- **Layouts that scale**: Shells, split panes, resizable stacks, overlay managers, docking-inspired primitives, and responsive rules that keep sophisticated tools inside a single OS window.
- **Component library**: Ergonomic building blocks (buttons, inputs, surfaces, data displays, feedback, overlay widgets) and higher-level composites (command palette, inspector, property sheets).
- **Platform crate**: Unified APIs for system tray menus, notifications, global hotkeys, window/process management, and IPC plumbing for multi-window scenarios. Linux (X11/Wayland), macOS, and Windows are targeted from day one.

What we're building
-------------------

- **Platform crate** *(in progress)*: cross-desktop shim for tray icons, notifications, hotkeys, window lifecycle, and IPC scaffolding.
- **Tokens + theme bridge**: design token structs, preset palettes, and bindings into iced theme/style traits.
- **Layouts & shells**: layout crate with stacks, grids, auto-resizing panes, overlay manager, and opinionated `AppShell`.
- **Components & extras**: widget crate for buttons/inputs/feedback/data plus an extras crate for command palettes, inspectors, consoles, etc.

Repository layout
-----------------

```
README.md                # high-level orientation (this file)
specs.md                 # exploratory notes & backlog ideas
docs/
  overview.md            # product positioning & goals
  platform.md            # platform crate architecture and roadmap
  design-system.md       # styling + component strategy
  layouts.md             # scalable layout + overlay plans
  components.md          # pre-built widget roadmap
```

Status
------

- **Active focus**: fleshing out platform crate APIs *and* finalizing the design-token + theme bridge that feeds upcoming components.
- **Near-term**: bootstrap the layout crate (overlay stack + AppShell), ship an MVP component gallery, and wire everything into a shared kitchen-sink demo.
- **Mid-term**: layer on extras (command palette, inspector) and experiment with multi-window orchestration once single-window UX is polished.

Contributing
------------

Contributions are welcome once the platform crate API stabilizes. Until then feel free to open issues to discuss requirements, desktop integration edge cases, or coordination on downstream apps that want to test drive the crates early.

License
-------

MIT or Apache-2.0 (TBD). The license choice will be finalized before the first public crate release.
