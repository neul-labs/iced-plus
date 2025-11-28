//! Styled radio button component.

use iced::widget::radio;
use iced::{Element, Length};

/// A styled radio button component.
///
/// # Example
///
/// ```rust,ignore
/// Radio::new("Option A", Choice::A, selected, Message::Select)
///     .size(20.0)
/// ```
pub struct Radio<'a, Message, V> {
    label: &'a str,
    value: V,
    selected: Option<V>,
    on_select: Box<dyn Fn(V) -> Message + 'a>,
    size: f32,
    spacing: f32,
    width: Length,
}

impl<'a, Message, V> Radio<'a, Message, V>
where
    Message: Clone + 'a,
    V: Copy + Eq + 'a,
{
    /// Create a new radio button.
    pub fn new<F>(label: &'a str, value: V, selected: Option<V>, on_select: F) -> Self
    where
        F: Fn(V) -> Message + 'a,
    {
        Self {
            label,
            value,
            selected,
            on_select: Box::new(on_select),
            size: 20.0,
            spacing: 10.0,
            width: Length::Shrink,
        }
    }

    /// Set the radio button size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the spacing between radio and label.
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

impl<'a, Message, V> From<Radio<'a, Message, V>> for Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
    V: Copy + Eq + 'a,
{
    fn from(r: Radio<'a, Message, V>) -> Self {
        radio(r.label, r.value, r.selected, r.on_select)
            .size(r.size)
            .spacing(r.spacing)
            .width(r.width)
            .into()
    }
}

/// A group of radio buttons for selecting one option.
///
/// # Example
///
/// ```rust,ignore
/// RadioGroup::new(&[
///     ("Small", Size::Small),
///     ("Medium", Size::Medium),
///     ("Large", Size::Large),
/// ], selected_size, Message::SizeChanged)
/// ```
pub struct RadioGroup<'a, Message, V> {
    options: Vec<(&'a str, V)>,
    selected: Option<V>,
    on_select: Box<dyn Fn(V) -> Message + 'a>,
    size: f32,
    spacing: f32,
    gap: f32,
    horizontal: bool,
}

impl<'a, Message, V> RadioGroup<'a, Message, V>
where
    Message: Clone + 'a,
    V: Copy + Eq + 'a,
{
    /// Create a new radio group.
    pub fn new<F>(options: &[(&'a str, V)], selected: Option<V>, on_select: F) -> Self
    where
        F: Fn(V) -> Message + 'a,
    {
        Self {
            options: options.to_vec(),
            selected,
            on_select: Box::new(on_select),
            size: 20.0,
            spacing: 10.0,
            gap: 12.0,
            horizontal: false,
        }
    }

    /// Set the radio button size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set spacing between radio and label.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the gap between radio buttons.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Layout horizontally instead of vertically.
    #[must_use]
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
}

impl<'a, Message, V> From<RadioGroup<'a, Message, V>> for Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
    V: Copy + Eq + 'a,
{
    fn from(group: RadioGroup<'a, Message, V>) -> Self {
        use iced::widget::{column, row};

        let radios: Vec<Element<'a, Message, iced::Theme>> = group
            .options
            .into_iter()
            .map(|(label, value)| {
                let selected = group.selected;
                let on_select = &group.on_select;
                radio(label, value, selected, |v| on_select(v))
                    .size(group.size)
                    .spacing(group.spacing)
                    .into()
            })
            .collect();

        if group.horizontal {
            row(radios).spacing(group.gap).into()
        } else {
            column(radios).spacing(group.gap).into()
        }
    }
}
