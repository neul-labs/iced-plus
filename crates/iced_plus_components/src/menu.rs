//! Menu and context menu components.

use std::borrow::Cow;

/// A menu item.
#[derive(Clone)]
pub enum MenuItem<'a, Message> {
    /// Regular clickable item.
    Item {
        /// Item label.
        label: Cow<'a, str>,
        /// Optional icon.
        icon: Option<Cow<'a, str>>,
        /// Message to emit on click.
        on_click: Message,
        /// Whether the item is enabled.
        enabled: bool,
        /// Optional keyboard shortcut hint.
        shortcut: Option<Cow<'a, str>>,
    },
    /// Separator line.
    Separator,
    /// Submenu.
    Submenu {
        /// Submenu label.
        label: Cow<'a, str>,
        /// Submenu items.
        items: Vec<MenuItem<'a, Message>>,
    },
}

impl<'a, Message> MenuItem<'a, Message> {
    /// Create a new menu item.
    pub fn new(label: impl Into<Cow<'a, str>>, on_click: Message) -> Self {
        Self::Item {
            label: label.into(),
            icon: None,
            on_click,
            enabled: true,
            shortcut: None,
        }
    }

    /// Create a separator.
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Create a submenu.
    pub fn submenu(label: impl Into<Cow<'a, str>>, items: Vec<MenuItem<'a, Message>>) -> Self {
        Self::Submenu {
            label: label.into(),
            items,
        }
    }

    /// Add an icon to the item.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        if let Self::Item { icon: ref mut i, .. } = self {
            *i = Some(icon.into());
        }
        self
    }

    /// Set enabled state.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        if let Self::Item { enabled: ref mut e, .. } = self {
            *e = enabled;
        }
        self
    }

    /// Add a shortcut hint.
    #[must_use]
    pub fn shortcut(mut self, shortcut: impl Into<Cow<'a, str>>) -> Self {
        if let Self::Item { shortcut: ref mut s, .. } = self {
            *s = Some(shortcut.into());
        }
        self
    }
}

/// A menu definition.
pub struct Menu<'a, Message> {
    /// Menu items.
    pub items: Vec<MenuItem<'a, Message>>,
}

impl<'a, Message> Menu<'a, Message> {
    /// Create a new empty menu.
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Add an item to the menu.
    #[must_use]
    pub fn item(mut self, item: MenuItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Add a simple labeled item.
    #[must_use]
    pub fn push(mut self, label: impl Into<Cow<'a, str>>, on_click: Message) -> Self {
        self.items.push(MenuItem::new(label, on_click));
        self
    }

    /// Add a separator.
    #[must_use]
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::Separator);
        self
    }

    /// Add a submenu.
    #[must_use]
    pub fn submenu(mut self, label: impl Into<Cow<'a, str>>, items: Vec<MenuItem<'a, Message>>) -> Self {
        self.items.push(MenuItem::submenu(label, items));
        self
    }
}

impl<'a, Message> Default for Menu<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

/// Menu bar containing multiple menus.
pub struct MenuBar<'a, Message> {
    /// Named menus in the bar.
    pub menus: Vec<(Cow<'a, str>, Menu<'a, Message>)>,
}

impl<'a, Message> MenuBar<'a, Message> {
    /// Create a new empty menu bar.
    #[must_use]
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    /// Add a menu to the bar.
    #[must_use]
    pub fn menu(mut self, label: impl Into<Cow<'a, str>>, menu: Menu<'a, Message>) -> Self {
        self.menus.push((label.into(), menu));
        self
    }
}

impl<'a, Message> Default for MenuBar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}
