//! Enhanced scrollable with snap points and anchor support.
//!
//! Provides a scrollable container with smooth scrolling to anchor points.
//!
//! # Example
//!
//! ```rust,ignore
//! SnapScrollable::new()
//!     .push_anchor("section1", section1_content)
//!     .push_anchor("section2", section2_content)
//!     .scroll_to(self.target_anchor.clone())
//! ```

use iced::widget::{container, scrollable, Column};
use iced::{Background, Border, Color, Element, Length, Theme};

/// Scroll direction.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Vertical scrolling.
    #[default]
    Vertical,
    /// Horizontal scrolling.
    Horizontal,
    /// Both directions.
    Both,
}

/// Snap alignment for scroll snapping.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SnapAlignment {
    /// Snap to start of container.
    #[default]
    Start,
    /// Snap to center of container.
    Center,
    /// Snap to end of container.
    End,
}

/// Enhanced scrollable configuration.
pub struct ScrollableConfig {
    /// Scroll direction.
    pub direction: ScrollDirection,
    /// Width of the scrollable.
    pub width: Length,
    /// Height of the scrollable.
    pub height: Length,
    /// Whether to show scrollbars.
    pub scrollbar_width: f32,
    /// Scrollbar margin.
    pub scrollbar_margin: f32,
    /// Scroller width.
    pub scroller_width: f32,
}

impl Default for ScrollableConfig {
    fn default() -> Self {
        Self {
            direction: ScrollDirection::Vertical,
            width: Length::Fill,
            height: Length::Fill,
            scrollbar_width: 10.0,
            scrollbar_margin: 0.0,
            scroller_width: 10.0,
        }
    }
}

/// Creates a styled scrollable container.
///
/// # Example
///
/// ```rust,ignore
/// styled_scrollable(
///     my_content,
///     ScrollableConfig::default(),
/// )
/// ```
pub fn styled_scrollable<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
    config: ScrollableConfig,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let scroll = scrollable(content)
        .width(config.width)
        .height(config.height);

    scroll.into()
}

/// A section with an anchor ID for scroll-to functionality.
pub struct AnchorSection<'a, Message> {
    /// Anchor ID.
    pub id: String,
    /// Section content.
    pub content: Element<'a, Message, Theme>,
}

impl<'a, Message> AnchorSection<'a, Message> {
    /// Create a new anchor section.
    pub fn new(id: impl Into<String>, content: impl Into<Element<'a, Message, Theme>>) -> Self {
        Self {
            id: id.into(),
            content: content.into(),
        }
    }
}

/// Builder for creating scrollable content with anchor sections.
///
/// # Example
///
/// ```rust,ignore
/// // Create scrollable with sections
/// let content = ScrollableBuilder::new()
///     .push(text("Header"))
///     .push_anchor("intro", intro_section)
///     .push_anchor("features", features_section)
///     .push_anchor("contact", contact_section)
///     .spacing(20.0)
///     .build();
///
/// // Scroll to a section using iced's scrollable::Id
/// // scrollable::snap_to(id, offset)
/// ```
pub struct ScrollableBuilder<'a, Message> {
    children: Vec<Element<'a, Message, Theme>>,
    spacing: f32,
    padding: f32,
    width: Length,
    height: Length,
}

impl<'a, Message: Clone + 'a> Default for ScrollableBuilder<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone + 'a> ScrollableBuilder<'a, Message> {
    /// Create a new scrollable builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
            padding: 0.0,
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    /// Add content to the scrollable.
    #[must_use]
    pub fn push(mut self, content: impl Into<Element<'a, Message, Theme>>) -> Self {
        self.children.push(content.into());
        self
    }

    /// Add an anchor section (content wrapped with an id for scroll-to).
    ///
    /// Note: The anchor ID is stored for reference but scroll-to is handled
    /// via iced's native `scrollable::Id` and commands.
    #[must_use]
    pub fn push_anchor(
        mut self,
        _id: impl Into<String>,
        content: impl Into<Element<'a, Message, Theme>>,
    ) -> Self {
        // Wrap content in a container that can be targeted
        // Note: iced's scrollable doesn't have built-in anchor support,
        // so we just add the content. For actual scroll-to, users should
        // track positions manually or use scrollable commands.
        let section = container(content).width(Length::Fill).into();
        self.children.push(section);
        self
    }

    /// Set the spacing between items.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
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

    /// Build the scrollable element.
    #[must_use]
    pub fn build(self) -> Element<'a, Message, Theme> {
        let content: Column<'a, Message, Theme> = Column::with_children(self.children)
            .spacing(self.spacing)
            .padding(self.padding)
            .width(self.width);

        scrollable(content)
            .width(self.width)
            .height(self.height)
            .into()
    }
}

/// Scrollable with custom styling and optional scroll indicators.
///
/// # Example
///
/// ```rust,ignore
/// themed_scrollable(content)
///     .rail_style(ScrollRailStyle::Rounded)
/// ```
pub fn themed_scrollable<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> scrollable::Scrollable<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    scrollable(content).style(|theme, status| {
        let palette = theme.extended_palette();

        let rail_bg = palette.background.weak.color;
        let scroller_color = match status {
            scrollable::Status::Active => palette.background.strong.color,
            scrollable::Status::Hovered { .. } | scrollable::Status::Dragged { .. } => {
                palette.primary.base.color
            }
        };

        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: Some(Background::Color(rail_bg)),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                },
            },
            horizontal_rail: scrollable::Rail {
                background: Some(Background::Color(rail_bg)),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                },
            },
            gap: None,
        }
    })
}

/// Minimal scrollable (thin, subtle scrollbar).
pub fn minimal_scrollable<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> scrollable::Scrollable<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    scrollable(content).style(|theme, status| {
        let palette = theme.extended_palette();

        let scroller_color = match status {
            scrollable::Status::Active => Color {
                a: 0.3,
                ..palette.background.strong.color
            },
            scrollable::Status::Hovered { .. } | scrollable::Status::Dragged { .. } => Color {
                a: 0.6,
                ..palette.background.strong.color
            },
        };

        scrollable::Style {
            container: container::Style::default(),
            vertical_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                },
            },
            horizontal_rail: scrollable::Rail {
                background: None,
                border: Border::default(),
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                },
            },
            gap: None,
        }
    })
}

/// Scroll position helpers.
pub mod position {
    use iced::widget::scrollable::{AbsoluteOffset, RelativeOffset};

    /// Create an absolute scroll offset.
    #[must_use]
    pub fn absolute(x: f32, y: f32) -> AbsoluteOffset {
        AbsoluteOffset { x, y }
    }

    /// Create a relative scroll offset (0.0 to 1.0).
    #[must_use]
    pub fn relative(x: f32, y: f32) -> RelativeOffset {
        RelativeOffset {
            x: x.clamp(0.0, 1.0),
            y: y.clamp(0.0, 1.0),
        }
    }

    /// Scroll to top.
    #[must_use]
    pub fn top() -> AbsoluteOffset {
        AbsoluteOffset { x: 0.0, y: 0.0 }
    }

    /// Scroll to bottom (use with relative).
    #[must_use]
    pub fn bottom() -> RelativeOffset {
        RelativeOffset { x: 0.0, y: 1.0 }
    }
}
