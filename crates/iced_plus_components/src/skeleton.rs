//! Skeleton loader/placeholder component.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{mouse, Color, Element, Length, Rectangle, Size};

/// Skeleton shape variant.
#[derive(Debug, Clone, Copy, Default)]
pub enum SkeletonShape {
    /// Rectangular shape.
    #[default]
    Rectangle,
    /// Circular shape.
    Circle,
    /// Rounded rectangle.
    Rounded,
    /// Text-like with rounded ends.
    Text,
}

/// A skeleton placeholder component for loading states.
///
/// # Example
///
/// ```rust,ignore
/// // Text placeholder
/// Skeleton::text().width(Length::Fixed(200.0))
///
/// // Avatar placeholder
/// Skeleton::circle().size(40.0)
///
/// // Card placeholder
/// Skeleton::new()
///     .width(Length::Fill)
///     .height(Length::Fixed(100.0))
/// ```
pub struct Skeleton {
    width: Length,
    height: Length,
    shape: SkeletonShape,
}

impl Default for Skeleton {
    fn default() -> Self {
        Self {
            width: Length::Fill,
            height: Length::Fixed(20.0),
            shape: SkeletonShape::default(),
        }
    }
}

impl Skeleton {
    /// Create a new skeleton placeholder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a text-line skeleton.
    #[must_use]
    pub fn text() -> Self {
        Self {
            shape: SkeletonShape::Text,
            height: Length::Fixed(16.0),
            ..Default::default()
        }
    }

    /// Create a circular skeleton (for avatars).
    #[must_use]
    pub fn circle() -> Self {
        Self {
            shape: SkeletonShape::Circle,
            width: Length::Fixed(40.0),
            height: Length::Fixed(40.0),
        }
    }

    /// Create a rounded rectangle skeleton.
    #[must_use]
    pub fn rounded() -> Self {
        Self {
            shape: SkeletonShape::Rounded,
            ..Default::default()
        }
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

    /// Set both width and height (for squares/circles).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.width = Length::Fixed(size);
        self.height = Length::Fixed(size);
        self
    }

    /// Set the shape.
    #[must_use]
    pub fn shape(mut self, shape: SkeletonShape) -> Self {
        self.shape = shape;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Skeleton
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
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
        let width = match self.width {
            Length::Fill => limits.max().width,
            Length::Fixed(w) => w,
            Length::Shrink => 100.0,
            Length::FillPortion(_) => limits.max().width,
        };

        let height = match self.height {
            Length::Fill => limits.max().height,
            Length::Fixed(h) => h,
            Length::Shrink => 20.0,
            Length::FillPortion(_) => limits.max().height,
        };

        Node::new(Size::new(width, height))
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

        let radius = match self.shape {
            SkeletonShape::Rectangle => 0.0,
            SkeletonShape::Circle => bounds.width.min(bounds.height) / 2.0,
            SkeletonShape::Rounded => 8.0,
            SkeletonShape::Text => bounds.height / 2.0,
        };

        // Skeleton color with slight shimmer effect feel
        let color = Color::from_rgba(0.0, 0.0, 0.0, 0.08);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
            },
            color,
        );
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer> From<Skeleton> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(skeleton: Skeleton) -> Self {
        Element::new(skeleton)
    }
}
