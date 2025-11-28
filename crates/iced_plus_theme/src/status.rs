//! Widget status types for interaction states.

/// Status of an interactive widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Status {
    /// Normal/idle state.
    #[default]
    Active,
    /// Mouse is hovering over the widget.
    Hovered,
    /// Widget is being pressed/clicked.
    Pressed,
    /// Widget is disabled and non-interactive.
    Disabled,
    /// Widget has keyboard focus.
    Focused,
}

impl Status {
    /// Returns true if the widget is interactive (not disabled).
    #[must_use]
    pub const fn is_interactive(self) -> bool {
        !matches!(self, Self::Disabled)
    }

    /// Returns true if the widget should show hover effects.
    #[must_use]
    pub const fn shows_hover(self) -> bool {
        matches!(self, Self::Hovered)
    }

    /// Returns true if the widget should show pressed effects.
    #[must_use]
    pub const fn shows_pressed(self) -> bool {
        matches!(self, Self::Pressed)
    }
}

/// Convert from iced's button status.
impl From<iced::widget::button::Status> for Status {
    fn from(status: iced::widget::button::Status) -> Self {
        match status {
            iced::widget::button::Status::Active => Self::Active,
            iced::widget::button::Status::Hovered => Self::Hovered,
            iced::widget::button::Status::Pressed => Self::Pressed,
            iced::widget::button::Status::Disabled => Self::Disabled,
        }
    }
}

/// Convert from iced's text input status.
impl From<iced::widget::text_input::Status> for Status {
    fn from(status: iced::widget::text_input::Status) -> Self {
        match status {
            iced::widget::text_input::Status::Active => Self::Active,
            iced::widget::text_input::Status::Hovered => Self::Hovered,
            iced::widget::text_input::Status::Focused => Self::Focused,
            iced::widget::text_input::Status::Disabled => Self::Disabled,
        }
    }
}
