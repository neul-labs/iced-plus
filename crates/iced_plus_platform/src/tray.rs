//! System tray icon integration.
//!
//! Provides an abstraction for system tray/notification area icons
//! with context menus.

use std::borrow::Cow;

/// System tray icon builder.
///
/// # Example
///
/// ```rust,ignore
/// let tray = TrayIcon::new("My App")
///     .tooltip("My Application")
///     .menu(TrayMenu::new()
///         .item("Show", Message::Show)
///         .item("Quit", Message::Quit));
/// ```
pub struct TrayIcon<'a, Message> {
    /// Application name for identification.
    pub name: Cow<'a, str>,
    /// Tooltip text shown on hover.
    pub tooltip: Option<Cow<'a, str>>,
    /// Icon data (PNG bytes).
    pub icon: Option<&'a [u8]>,
    /// Context menu.
    pub menu: Option<TrayMenu<'a, Message>>,
}

impl<'a, Message> TrayIcon<'a, Message> {
    /// Create a new tray icon.
    #[must_use]
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self {
            name: name.into(),
            tooltip: None,
            icon: None,
            menu: None,
        }
    }

    /// Set the tooltip text.
    #[must_use]
    pub fn tooltip(mut self, tooltip: impl Into<Cow<'a, str>>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set the icon data (PNG bytes).
    #[must_use]
    pub fn icon(mut self, icon: &'a [u8]) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set the context menu.
    #[must_use]
    pub fn menu(mut self, menu: TrayMenu<'a, Message>) -> Self {
        self.menu = Some(menu);
        self
    }
}

/// Tray context menu.
pub struct TrayMenu<'a, Message> {
    /// Menu items.
    pub items: Vec<TrayMenuItem<'a, Message>>,
}

impl<'a, Message> TrayMenu<'a, Message> {
    /// Create a new empty menu.
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add a menu item.
    #[must_use]
    pub fn item(mut self, label: impl Into<Cow<'a, str>>, message: Message) -> Self {
        self.items.push(TrayMenuItem::Item {
            label: label.into(),
            message,
            enabled: true,
        });
        self
    }

    /// Add a disabled menu item.
    #[must_use]
    pub fn item_disabled(mut self, label: impl Into<Cow<'a, str>>, message: Message) -> Self {
        self.items.push(TrayMenuItem::Item {
            label: label.into(),
            message,
            enabled: false,
        });
        self
    }

    /// Add a separator.
    #[must_use]
    pub fn separator(mut self) -> Self {
        self.items.push(TrayMenuItem::Separator);
        self
    }

    /// Add a submenu.
    #[must_use]
    pub fn submenu(mut self, label: impl Into<Cow<'a, str>>, submenu: TrayMenu<'a, Message>) -> Self {
        self.items.push(TrayMenuItem::Submenu {
            label: label.into(),
            menu: submenu,
        });
        self
    }
}

impl<'a, Message> Default for TrayMenu<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual tray menu item.
pub enum TrayMenuItem<'a, Message> {
    /// Regular menu item.
    Item {
        /// Display label.
        label: Cow<'a, str>,
        /// Message to emit on click.
        message: Message,
        /// Whether the item is enabled.
        enabled: bool,
    },
    /// Menu separator.
    Separator,
    /// Submenu.
    Submenu {
        /// Submenu label.
        label: Cow<'a, str>,
        /// Nested menu.
        menu: TrayMenu<'a, Message>,
    },
}
