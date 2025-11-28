Platform Crate
==============

The platform crate is one pillar of `iced-plus`. While the design system, layouts, and components ensure apps look and feel cohesive, the platform crate hides OS-specific plumbing so those experiences behave consistently across desktops. Think of it as the runtime counterpart to the UI crates.

Targets
-------

- **OSes**: macOS 13+, Windows 10+, Linux (X11 + Wayland with fallbacks).
- **Iced compatibility**: latest stable iced release; no private backend forks required.
- **Runtime**: async-agnostic at the public API level but internally uses Tokio where available (with sync fallbacks for GUIs that forbid non-main-thread runtimes).

Feature areas
-------------

1. **Tray service**
   - Show/hide menu bar or system tray icons with dynamic menus.
   - Surface click events and menu selections into iced messages.
   - Detect unsupported contexts (e.g., Wayland without StatusNotifierItem) and expose capability flags.

2. **Notifications**
   - Fire-and-forget user notifications with optional action buttons.
   - Provide delivery receipts/error callbacks so apps can fall back to in-window toasts when the OS refuses.

3. **Global hotkeys**
   - Register system-wide accelerators (e.g., `Cmd+Shift+P`) with de-duplication and automatic cleanup.
   - Handle focus-tracking policies per OS (e.g., requiring accessibility permissions on macOS).

4. **Window lifecycle helpers**
   - Helpers for primary window creation, minimization to tray, focus management, and “single instance” locks.
   - Abstractions for requesting quit, blocking close, and storing/restoring window geometry.

5. **Multi-window IPC foundation**
   - Message definitions, process spawning helpers, and async adapters (NNG- or channel-based) so future multi-window orchestration can be layered on without rethinking IPC.

API shape
---------

```rust
pub struct PlatformContext {
    tray: TrayService,
    notifications: NotificationService,
    hotkeys: HotkeyService,
    windows: WindowService,
    ipc: IpcBus, // optional feature gate
}

impl PlatformContext {
    pub fn new(app_id: &str, opts: PlatformOptions) -> Result<Self, PlatformError>;
    pub fn capabilities(&self) -> Capabilities;
}
```

Each service exposes async-friendly methods that integrate with iced via `Command::perform` or `Subscription`s:

```rust
// Register a tray icon and start a subscription that converts tray events into iced messages.
let tray_events = platform.tray.subscribe(TrayMenu::default());

// Register a global hotkey.
platform.hotkeys.register(GlobalKey::new(mods![Command, Shift], Key::P))?;

// Trigger a notification.
platform.notifications.notify(Notification::info("Build finished"))?;
```

Integration patterns
--------------------

- **Commands for fire-and-forget tasks**: notifications, hotkey registration, tray updates.
- **Subscriptions for inbound events**: tray menu clicks, hotkey presses, IPC messages.
- **Stateful helpers**: `WindowService` stores geometry/session info and exposes `Command`s to save/restore.
- **Feature flags**: `platform-ipc`, `platform-hotkeys`, etc., so smaller apps only pull what they need.

Error handling & capability reporting
-------------------------------------

- `PlatformContext::capabilities()` returns a struct detailing what the OS session supports (e.g., `tray: Supported`, `global_hotkeys: RequiresAccessibilityPermission`).
- Each service method returns a `PlatformError`. Recoverable cases (like unsupported features) should be surfaced so apps can fall back to in-app UI.

Roadmap
-------

1. **MVP**
   - Tray service for macOS + Windows, notifications via native APIs, scoped hotkeys, window geometry persistence.
2. **Linux parity**
   - Wayland + X11 tray compatibility layers, D-Bus notifications, XKB global hotkeys where allowed.
3. **IPC kit**
   - Provide message codecs, spawn helpers, and iced subscriptions for multi-process multi-window experiments.
4. **Polish & docs**
   - Expand examples, document security/permission requirements, finalize feature flags.

For additional project context, see `docs/overview.md`.
