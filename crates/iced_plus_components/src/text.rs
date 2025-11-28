//! Text and heading components.

use std::borrow::Cow;

use iced::widget::text;
use iced::{Color, Element, Length};
use iced_plus_theme::AppTheme;

/// Text style variants.
#[derive(Debug, Clone, Copy, Default)]
pub enum TextStyle {
    /// Normal body text.
    #[default]
    Body,
    /// Muted/secondary text.
    Muted,
    /// Success text (green).
    Success,
    /// Warning text (amber).
    Warning,
    /// Error text (red).
    Error,
    /// Custom color.
    Custom(Color),
}

/// A styled text component.
pub struct Text<'a> {
    content: Cow<'a, str>,
    size: f32,
    style: TextStyle,
    width: Length,
}

impl<'a> Text<'a> {
    /// Create a new text component.
    #[must_use]
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            size: 14.0,
            style: TextStyle::Body,
            width: Length::Shrink,
        }
    }

    /// Set the font size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the text style.
    #[must_use]
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Make the text muted.
    #[must_use]
    pub fn muted(mut self) -> Self {
        self.style = TextStyle::Muted;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a> From<Text<'a>> for Element<'a, Message, AppTheme<'a>> {
    fn from(t: Text<'a>) -> Self {
        let content: String = t.content.into_owned();
        text(content).size(t.size).width(t.width).into()
    }
}

impl<'a, Message: 'a> From<Text<'a>> for Element<'a, Message, iced::Theme> {
    fn from(t: Text<'a>) -> Self {
        let content: String = t.content.into_owned();
        text(content).size(t.size).width(t.width).into()
    }
}

/// Heading level.
#[derive(Debug, Clone, Copy, Default)]
pub enum HeadingLevel {
    /// H1 - largest heading.
    H1,
    /// H2 - second level heading.
    #[default]
    H2,
    /// H3 - third level heading.
    H3,
    /// H4 - fourth level heading.
    H4,
}

impl HeadingLevel {
    /// Get the font size for this heading level.
    #[must_use]
    pub const fn size(self) -> f32 {
        match self {
            Self::H1 => 32.0,
            Self::H2 => 24.0,
            Self::H3 => 20.0,
            Self::H4 => 16.0,
        }
    }
}

/// A heading component with semantic levels.
pub struct Heading<'a> {
    content: Cow<'a, str>,
    level: HeadingLevel,
    width: Length,
}

impl<'a> Heading<'a> {
    /// Create a new heading.
    #[must_use]
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            level: HeadingLevel::H2,
            width: Length::Shrink,
        }
    }

    /// Create an H1 heading.
    #[must_use]
    pub fn h1(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(content).level(HeadingLevel::H1)
    }

    /// Create an H2 heading.
    #[must_use]
    pub fn h2(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(content).level(HeadingLevel::H2)
    }

    /// Create an H3 heading.
    #[must_use]
    pub fn h3(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(content).level(HeadingLevel::H3)
    }

    /// Create an H4 heading.
    #[must_use]
    pub fn h4(content: impl Into<Cow<'a, str>>) -> Self {
        Self::new(content).level(HeadingLevel::H4)
    }

    /// Set the heading level.
    #[must_use]
    pub fn level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: 'a> From<Heading<'a>> for Element<'a, Message, AppTheme<'a>> {
    fn from(h: Heading<'a>) -> Self {
        let content: String = h.content.into_owned();
        text(content).size(h.level.size()).width(h.width).into()
    }
}

impl<'a, Message: 'a> From<Heading<'a>> for Element<'a, Message, iced::Theme> {
    fn from(h: Heading<'a>) -> Self {
        let content: String = h.content.into_owned();
        text(content).size(h.level.size()).width(h.width).into()
    }
}
