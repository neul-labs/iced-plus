//! Select/Dropdown component.

use iced::widget::pick_list;
use iced::{Element, Length};

/// A styled select/dropdown component.
///
/// This wraps iced's pick_list with consistent styling.
///
/// # Example
///
/// ```rust,ignore
/// Select::new(
///     &["Option 1", "Option 2", "Option 3"],
///     selected,
///     Message::Selected,
/// )
/// .placeholder("Choose an option")
/// ```
pub struct Select<'a, T, Message>
where
    T: ToString + PartialEq + Clone,
{
    options: &'a [T],
    selected: Option<T>,
    on_select: Box<dyn Fn(T) -> Message + 'a>,
    placeholder: Option<&'a str>,
    width: Length,
}

impl<'a, T, Message> Select<'a, T, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    /// Create a new select component.
    pub fn new<F>(options: &'a [T], selected: Option<T>, on_select: F) -> Self
    where
        F: Fn(T) -> Message + 'a,
    {
        Self {
            options,
            selected,
            on_select: Box::new(on_select),
            placeholder: None,
            width: Length::Fill,
        }
    }

    /// Set the placeholder text.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, T, Message> From<Select<'a, T, Message>> for Element<'a, Message, iced::Theme>
where
    T: ToString + PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    fn from(select: Select<'a, T, Message>) -> Self {
        let mut widget = pick_list(select.options, select.selected, select.on_select)
            .width(select.width)
            .padding(10);

        if let Some(placeholder) = select.placeholder {
            widget = widget.placeholder(placeholder);
        }

        widget.into()
    }
}
