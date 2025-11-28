//! Badge component for status indicators and counts.

use std::borrow::Cow;

use iced::widget::{container, text};
use iced::{Background, Border, Color, Element};

/// Badge variant for different semantic meanings.
#[derive(Debug, Clone, Copy, Default)]
pub enum BadgeVariant {
    /// Default/neutral badge.
    #[default]
    Default,
    /// Primary accent badge.
    Primary,
    /// Success/positive badge.
    Success,
    /// Warning badge.
    Warning,
    /// Error/destructive badge.
    Error,
}

impl BadgeVariant {
    fn colors(self) -> (Color, Color) {
        // (background, text)
        match self {
            Self::Default => (Color::from_rgb(0.9, 0.9, 0.9), Color::from_rgb(0.3, 0.3, 0.3)),
            Self::Primary => (Color::from_rgb(0.22, 0.47, 0.87), Color::WHITE),
            Self::Success => (Color::from_rgb(0.13, 0.70, 0.40), Color::WHITE),
            Self::Warning => (Color::from_rgb(0.95, 0.65, 0.15), Color::BLACK),
            Self::Error => (Color::from_rgb(0.87, 0.24, 0.24), Color::WHITE),
        }
    }
}

/// A small badge for status indicators or counts.
///
/// # Example
///
/// ```rust,ignore
/// Badge::new("New")
///     .variant(BadgeVariant::Primary)
///
/// Badge::count(42)
///     .variant(BadgeVariant::Error)
/// ```
pub struct Badge<'a> {
    content: Cow<'a, str>,
    variant: BadgeVariant,
}

impl<'a> Badge<'a> {
    /// Create a new badge with text content.
    #[must_use]
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::default(),
        }
    }

    /// Create a badge with a numeric count.
    #[must_use]
    pub fn count(count: usize) -> Self {
        Self::new(count.to_string())
    }

    /// Set the badge variant.
    #[must_use]
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Make this a primary badge.
    #[must_use]
    pub fn primary(self) -> Self {
        self.variant(BadgeVariant::Primary)
    }

    /// Make this a success badge.
    #[must_use]
    pub fn success(self) -> Self {
        self.variant(BadgeVariant::Success)
    }

    /// Make this a warning badge.
    #[must_use]
    pub fn warning(self) -> Self {
        self.variant(BadgeVariant::Warning)
    }

    /// Make this an error badge.
    #[must_use]
    pub fn error(self) -> Self {
        self.variant(BadgeVariant::Error)
    }
}

impl<'a, Message: 'a> From<Badge<'a>> for Element<'a, Message, iced::Theme> {
    fn from(badge: Badge<'a>) -> Self {
        let (bg_color, text_color) = badge.variant.colors();
        let content: String = badge.content.into_owned();

        container(text(content).size(12).color(text_color))
            .padding([2, 8])
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(Background::Color(bg_color)),
                border: Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }
}
