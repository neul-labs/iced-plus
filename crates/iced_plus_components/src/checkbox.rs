//! Styled checkbox component.

use iced::widget::checkbox;
use iced::{Element, Length};

/// A styled checkbox component.
///
/// # Example
///
/// ```rust,ignore
/// Checkbox::new("Accept terms", is_checked, Message::ToggleTerms)
///     .size(20.0)
/// ```
pub struct Checkbox<'a, Message> {
    label: &'a str,
    is_checked: bool,
    on_toggle: Box<dyn Fn(bool) -> Message + 'a>,
    size: f32,
    spacing: f32,
    width: Length,
}

impl<'a, Message> Checkbox<'a, Message>
where
    Message: Clone + 'a,
{
    /// Create a new checkbox.
    pub fn new<F>(label: &'a str, is_checked: bool, on_toggle: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        Self {
            label,
            is_checked,
            on_toggle: Box::new(on_toggle),
            size: 20.0,
            spacing: 10.0,
            width: Length::Shrink,
        }
    }

    /// Set the checkbox size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spacing between checkbox and label.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message> From<Checkbox<'a, Message>> for Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(cb: Checkbox<'a, Message>) -> Self {
        checkbox(cb.label, cb.is_checked)
            .on_toggle(cb.on_toggle)
            .size(cb.size)
            .spacing(cb.spacing)
            .width(cb.width)
            .into()
    }
}
