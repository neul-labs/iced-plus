//! Tooltip component.

use iced::widget::tooltip;
use iced::Element;

/// Tooltip position.
#[derive(Debug, Clone, Copy, Default)]
pub enum TooltipPosition {
    /// Above the element.
    Top,
    /// Below the element.
    #[default]
    Bottom,
    /// To the left.
    Left,
    /// To the right.
    Right,
    /// Follow the cursor.
    FollowCursor,
}

impl From<TooltipPosition> for tooltip::Position {
    fn from(pos: TooltipPosition) -> Self {
        match pos {
            TooltipPosition::Top => tooltip::Position::Top,
            TooltipPosition::Bottom => tooltip::Position::Bottom,
            TooltipPosition::Left => tooltip::Position::Left,
            TooltipPosition::Right => tooltip::Position::Right,
            TooltipPosition::FollowCursor => tooltip::Position::FollowCursor,
        }
    }
}

/// A tooltip wrapper that shows text on hover.
///
/// # Example
///
/// ```rust,ignore
/// Tooltip::new(button, "Click to submit")
///     .position(TooltipPosition::Top)
/// ```
pub struct Tooltip<'a, Message> {
    content: Element<'a, Message, iced::Theme>,
    tooltip_text: String,
    position: TooltipPosition,
    gap: f32,
}

impl<'a, Message> Tooltip<'a, Message> {
    /// Create a new tooltip.
    pub fn new(
        content: impl Into<Element<'a, Message, iced::Theme>>,
        tooltip_text: impl Into<String>,
    ) -> Self {
        Self {
            content: content.into(),
            tooltip_text: tooltip_text.into(),
            position: TooltipPosition::default(),
            gap: 4.0,
        }
    }

    /// Set the tooltip position.
    #[must_use]
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the gap between content and tooltip.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
}

impl<'a, Message: 'a> From<Tooltip<'a, Message>> for Element<'a, Message, iced::Theme> {
    fn from(t: Tooltip<'a, Message>) -> Self {
        let tooltip_text = t.tooltip_text;
        tooltip(
            t.content,
            iced::widget::text(tooltip_text).size(12),
            t.position.into(),
        )
        .gap(t.gap)
        .style(|theme: &iced::Theme| {
            let palette = theme.extended_palette();
            iced::widget::container::Style {
                background: Some(iced::Background::Color(palette.background.strong.color)),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                text_color: Some(palette.background.strong.text),
                ..Default::default()
            }
        })
        .into()
    }
}
