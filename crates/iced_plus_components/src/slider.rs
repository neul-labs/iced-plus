//! Styled slider component.

use iced::widget::slider;
use iced::{Element, Length};

/// A styled slider component for f32 values.
///
/// # Example
///
/// ```rust,ignore
/// Slider::new(0.0..=100.0, value, Message::ValueChanged)
///     .step(1.0)
///     .width(200.0)
/// ```
pub struct Slider<'a, Message> {
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
    on_release: Option<Message>,
    step: f32,
    width: Length,
}

impl<'a, Message> Slider<'a, Message>
where
    Message: Clone,
{
    /// Create a new slider.
    pub fn new<F>(range: std::ops::RangeInclusive<f32>, value: f32, on_change: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        Self {
            range,
            value,
            on_change: Box::new(on_change),
            on_release: None,
            step: 1.0,
            width: Length::Fill,
        }
    }

    /// Set the step value.
    #[must_use]
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set a message to emit when the slider is released.
    #[must_use]
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }
}

impl<'a, Message> From<Slider<'a, Message>> for Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(s: Slider<'a, Message>) -> Self {
        let mut slider = slider(s.range, s.value, s.on_change)
            .step(s.step)
            .width(s.width);

        if let Some(on_release) = s.on_release {
            slider = slider.on_release(on_release);
        }

        slider.into()
    }
}

/// A vertical slider component for f32 values.
///
/// # Example
///
/// ```rust,ignore
/// VerticalSlider::new(0.0..=100.0, value, Message::ValueChanged)
///     .height(200.0)
/// ```
pub struct VerticalSlider<'a, Message> {
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
    on_release: Option<Message>,
    step: f32,
    height: Length,
}

impl<'a, Message> VerticalSlider<'a, Message>
where
    Message: Clone,
{
    /// Create a new vertical slider.
    pub fn new<F>(range: std::ops::RangeInclusive<f32>, value: f32, on_change: F) -> Self
    where
        F: Fn(f32) -> Message + 'a,
    {
        Self {
            range,
            value,
            on_change: Box::new(on_change),
            on_release: None,
            step: 1.0,
            height: Length::Fill,
        }
    }

    /// Set the step value.
    #[must_use]
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set a message to emit when the slider is released.
    #[must_use]
    pub fn on_release(mut self, message: Message) -> Self {
        self.on_release = Some(message);
        self
    }
}

impl<'a, Message> From<VerticalSlider<'a, Message>> for Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(s: VerticalSlider<'a, Message>) -> Self {
        use iced::widget::vertical_slider;

        let mut slider = vertical_slider(s.range, s.value, s.on_change)
            .step(s.step)
            .height(s.height);

        if let Some(on_release) = s.on_release {
            slider = slider.on_release(on_release);
        }

        slider.into()
    }
}
