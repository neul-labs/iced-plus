//! Divider component for visual separation.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{mouse, Color, Element, Length, Rectangle, Size};

/// Orientation of the divider.
#[derive(Debug, Clone, Copy, Default)]
pub enum DividerOrientation {
    /// Horizontal divider (full width, thin height).
    #[default]
    Horizontal,
    /// Vertical divider (thin width, full height).
    Vertical,
}

/// A simple divider line for visual separation.
///
/// # Example
///
/// ```rust,ignore
/// // Horizontal divider
/// Divider::horizontal()
///
/// // Vertical divider with custom color
/// Divider::vertical()
///     .color(Color::from_rgb(0.8, 0.8, 0.8))
/// ```
pub struct Divider {
    orientation: DividerOrientation,
    thickness: f32,
    color: Color,
    spacing: f32,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            spacing: 0.0,
        }
    }
}

impl Divider {
    /// Create a new horizontal divider.
    #[must_use]
    pub fn horizontal() -> Self {
        Self::default()
    }

    /// Create a new vertical divider.
    #[must_use]
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            ..Default::default()
        }
    }

    /// Set the divider thickness.
    #[must_use]
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    /// Set the divider color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Add spacing (margin) around the divider.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Divider
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        match self.orientation {
            DividerOrientation::Horizontal => {
                Size::new(Length::Fill, Length::Fixed(self.thickness + self.spacing * 2.0))
            }
            DividerOrientation::Vertical => {
                Size::new(Length::Fixed(self.thickness + self.spacing * 2.0), Length::Fill)
            }
        }
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn state(&self) -> tree::State {
        tree::State::None
    }

    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut Tree) {}

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, limits: &Limits) -> Node {
        let size = match self.orientation {
            DividerOrientation::Horizontal => Size::new(
                limits.max().width,
                self.thickness + self.spacing * 2.0,
            ),
            DividerOrientation::Vertical => Size::new(
                self.thickness + self.spacing * 2.0,
                limits.max().height,
            ),
        };

        Node::new(limits.resolve(Length::Shrink, Length::Shrink, size))
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let line_bounds = match self.orientation {
            DividerOrientation::Horizontal => Rectangle {
                x: bounds.x,
                y: bounds.y + self.spacing,
                width: bounds.width,
                height: self.thickness,
            },
            DividerOrientation::Vertical => Rectangle {
                x: bounds.x + self.spacing,
                y: bounds.y,
                width: self.thickness,
                height: bounds.height,
            },
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: line_bounds,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            self.color,
        );
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer> From<Divider> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(divider: Divider) -> Self {
        Element::new(divider)
    }
}
