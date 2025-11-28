//! Extended window management.
//!
//! Provides additional window management capabilities beyond iced's built-in support.

/// Window position on screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowPosition {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl WindowPosition {
    /// Create a new position.
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Position at origin (0, 0).
    pub const ORIGIN: Self = Self { x: 0, y: 0 };
}

/// Window size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowSize {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl WindowSize {
    /// Create a new size.
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Window state (normal, minimized, maximized, fullscreen).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WindowState {
    /// Normal window.
    #[default]
    Normal,
    /// Minimized to taskbar/dock.
    Minimized,
    /// Maximized to fill screen.
    Maximized,
    /// Fullscreen mode.
    Fullscreen,
}

/// Window settings for creation or modification.
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// Window title.
    pub title: String,
    /// Initial size.
    pub size: WindowSize,
    /// Initial position (None = system default).
    pub position: Option<WindowPosition>,
    /// Minimum size constraint.
    pub min_size: Option<WindowSize>,
    /// Maximum size constraint.
    pub max_size: Option<WindowSize>,
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Whether to show window decorations (title bar, borders).
    pub decorations: bool,
    /// Whether the window should be transparent.
    pub transparent: bool,
    /// Whether the window should always be on top.
    pub always_on_top: bool,
    /// Initial window state.
    pub state: WindowState,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            title: String::new(),
            size: WindowSize::new(800, 600),
            position: None,
            min_size: None,
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            state: WindowState::Normal,
        }
    }
}

impl WindowSettings {
    /// Create new window settings with a title.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Set the window size.
    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = WindowSize::new(width, height);
        self
    }

    /// Set the window position.
    #[must_use]
    pub fn position(mut self, x: i32, y: i32) -> Self {
        self.position = Some(WindowPosition::new(x, y));
        self
    }

    /// Center the window on screen.
    #[must_use]
    pub fn centered(mut self) -> Self {
        // Setting position to None typically results in centered placement
        self.position = None;
        self
    }

    /// Set minimum size constraint.
    #[must_use]
    pub fn min_size(mut self, width: u32, height: u32) -> Self {
        self.min_size = Some(WindowSize::new(width, height));
        self
    }

    /// Set maximum size constraint.
    #[must_use]
    pub fn max_size(mut self, width: u32, height: u32) -> Self {
        self.max_size = Some(WindowSize::new(width, height));
        self
    }

    /// Set whether the window is resizable.
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Set whether to show decorations.
    #[must_use]
    pub fn decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    /// Set whether the window is transparent.
    #[must_use]
    pub fn transparent(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }

    /// Set whether the window is always on top.
    #[must_use]
    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.always_on_top = always_on_top;
        self
    }

    /// Set the initial window state.
    #[must_use]
    pub fn state(mut self, state: WindowState) -> Self {
        self.state = state;
        self
    }

    /// Start maximized.
    #[must_use]
    pub fn maximized(self) -> Self {
        self.state(WindowState::Maximized)
    }

    /// Start in fullscreen.
    #[must_use]
    pub fn fullscreen(self) -> Self {
        self.state(WindowState::Fullscreen)
    }
}
