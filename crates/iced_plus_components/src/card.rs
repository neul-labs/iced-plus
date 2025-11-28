//! Card component for elevated content containers.

use iced::widget::container;
use iced::{Background, Border, Color, Element, Length, Shadow, Vector};

/// Elevation level for cards.
#[derive(Debug, Clone, Copy, Default)]
pub enum Elevation {
    /// No elevation (flat).
    Flat,
    /// Low elevation (subtle shadow).
    #[default]
    Low,
    /// Medium elevation.
    Medium,
    /// High elevation (prominent shadow).
    High,
}

impl Elevation {
    /// Get shadow configuration for this elevation.
    fn shadow(self) -> Shadow {
        match self {
            Self::Flat => Shadow::default(),
            Self::Low => Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            Self::Medium => Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 6.0,
            },
            Self::High => Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
        }
    }
}

/// A card container with elevation and styling.
///
/// # Example
///
/// ```rust,ignore
/// Card::new(content)
///     .elevation(Elevation::Medium)
///     .padding(16.0)
///     .width(Length::Fixed(300.0))
/// ```
pub struct Card<'a, Message> {
    content: Element<'a, Message, iced::Theme>,
    elevation: Elevation,
    padding: f32,
    width: Length,
    height: Length,
    radius: f32,
}

impl<'a, Message: 'a> Card<'a, Message> {
    /// Create a new card with content.
    pub fn new(content: impl Into<Element<'a, Message, iced::Theme>>) -> Self {
        Self {
            content: content.into(),
            elevation: Elevation::default(),
            padding: 16.0,
            width: Length::Shrink,
            height: Length::Shrink,
            radius: 8.0,
        }
    }

    /// Set the elevation level.
    #[must_use]
    pub fn elevation(mut self, elevation: Elevation) -> Self {
        self.elevation = elevation;
        self
    }

    /// Set the padding.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the border radius.
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Make the card fill the available width.
    #[must_use]
    pub fn fill_width(self) -> Self {
        self.width(Length::Fill)
    }
}

impl<'a, Message: 'a> From<Card<'a, Message>> for Element<'a, Message, iced::Theme> {
    fn from(card: Card<'a, Message>) -> Self {
        let elevation = card.elevation;
        let radius = card.radius;

        container(card.content)
            .padding(card.padding)
            .width(card.width)
            .height(card.height)
            .style(move |theme: &iced::Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.base.color)),
                    border: Border {
                        radius: radius.into(),
                        width: 1.0,
                        color: palette.background.weak.color,
                    },
                    shadow: elevation.shadow(),
                    ..Default::default()
                }
            })
            .into()
    }
}

// Note: For AppTheme support, use the iced::Theme version and configure theming at the app level
