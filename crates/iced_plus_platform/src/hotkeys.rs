//! Global keyboard shortcuts (hotkeys).
//!
//! Provides cross-platform global keyboard shortcut registration.

use std::collections::HashMap;

/// Modifier keys for hotkey combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    /// Control key (Cmd on macOS).
    pub ctrl: bool,
    /// Alt/Option key.
    pub alt: bool,
    /// Shift key.
    pub shift: bool,
    /// Super/Meta/Windows key.
    pub super_key: bool,
}

impl Modifiers {
    /// No modifiers.
    pub const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
        super_key: false,
    };

    /// Control key only.
    pub const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
        super_key: false,
    };

    /// Alt key only.
    pub const ALT: Self = Self {
        ctrl: false,
        alt: true,
        shift: false,
        super_key: false,
    };

    /// Shift key only.
    pub const SHIFT: Self = Self {
        ctrl: false,
        alt: false,
        shift: true,
        super_key: false,
    };

    /// Control + Shift.
    pub const CTRL_SHIFT: Self = Self {
        ctrl: true,
        alt: false,
        shift: true,
        super_key: false,
    };

    /// Control + Alt.
    pub const CTRL_ALT: Self = Self {
        ctrl: true,
        alt: true,
        shift: false,
        super_key: false,
    };
}

impl Default for Modifiers {
    fn default() -> Self {
        Self::NONE
    }
}

/// A keyboard key code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    /// Letter keys A-Z.
    Key(char),
    /// Function keys F1-F12.
    F(u8),
    /// Escape key.
    Escape,
    /// Enter/Return key.
    Enter,
    /// Space bar.
    Space,
    /// Backspace key.
    Backspace,
    /// Tab key.
    Tab,
    /// Arrow keys.
    Up,
    /// Arrow down.
    Down,
    /// Arrow left.
    Left,
    /// Arrow right.
    Right,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up.
    PageUp,
    /// Page Down.
    PageDown,
    /// Insert key.
    Insert,
    /// Delete key.
    Delete,
}

/// A hotkey combination (modifiers + key).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hotkey {
    /// Modifier keys.
    pub modifiers: Modifiers,
    /// The main key.
    pub key: KeyCode,
}

impl Hotkey {
    /// Create a new hotkey.
    #[must_use]
    pub const fn new(modifiers: Modifiers, key: KeyCode) -> Self {
        Self { modifiers, key }
    }

    /// Create a hotkey with just a key (no modifiers).
    #[must_use]
    pub const fn key(key: KeyCode) -> Self {
        Self::new(Modifiers::NONE, key)
    }

    /// Create a Ctrl+key hotkey.
    #[must_use]
    pub const fn ctrl(key: KeyCode) -> Self {
        Self::new(Modifiers::CTRL, key)
    }

    /// Create an Alt+key hotkey.
    #[must_use]
    pub const fn alt(key: KeyCode) -> Self {
        Self::new(Modifiers::ALT, key)
    }

    /// Create a Ctrl+Shift+key hotkey.
    #[must_use]
    pub const fn ctrl_shift(key: KeyCode) -> Self {
        Self::new(Modifiers::CTRL_SHIFT, key)
    }
}

/// Hotkey registry for managing global shortcuts.
///
/// # Example
///
/// ```rust,ignore
/// let mut registry = HotkeyRegistry::new();
/// registry.register(Hotkey::ctrl(KeyCode::Key('N')), Message::NewFile);
/// registry.register(Hotkey::ctrl(KeyCode::Key('S')), Message::Save);
/// ```
pub struct HotkeyRegistry<Message> {
    hotkeys: HashMap<Hotkey, Message>,
}

impl<Message> HotkeyRegistry<Message> {
    /// Create a new empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hotkeys: HashMap::new(),
        }
    }

    /// Register a hotkey with a message.
    pub fn register(&mut self, hotkey: Hotkey, message: Message) {
        self.hotkeys.insert(hotkey, message);
    }

    /// Unregister a hotkey.
    pub fn unregister(&mut self, hotkey: &Hotkey) -> Option<Message> {
        self.hotkeys.remove(hotkey)
    }

    /// Get the message for a hotkey if registered.
    pub fn get(&self, hotkey: &Hotkey) -> Option<&Message> {
        self.hotkeys.get(hotkey)
    }

    /// Check if a hotkey is registered.
    #[must_use]
    pub fn contains(&self, hotkey: &Hotkey) -> bool {
        self.hotkeys.contains_key(hotkey)
    }

    /// Get all registered hotkeys.
    pub fn iter(&self) -> impl Iterator<Item = (&Hotkey, &Message)> {
        self.hotkeys.iter()
    }
}

impl<Message> Default for HotkeyRegistry<Message> {
    fn default() -> Self {
        Self::new()
    }
}
