//! Progress bar and indicators.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{mouse, Color, Element, Length, Rectangle, Size};

/// Progress bar variant.
#[derive(Debug, Clone, Copy, Default)]
pub enum ProgressVariant {
    /// Default/primary color.
    #[default]
    Default,
    /// Success (green).
    Success,
    /// Warning (amber).
    Warning,
    /// Error (red).
    Error,
}

impl ProgressVariant {
    fn color(self) -> Color {
        match self {
            Self::Default => Color::from_rgb(0.22, 0.47, 0.87),
            Self::Success => Color::from_rgb(0.13, 0.70, 0.40),
            Self::Warning => Color::from_rgb(0.95, 0.65, 0.15),
            Self::Error => Color::from_rgb(0.87, 0.24, 0.24),
        }
    }
}

/// A progress bar component.
///
/// # Example
///
/// ```rust,ignore
/// Progress::new(0.75) // 75% progress
///     .variant(ProgressVariant::Success)
///     .height(8.0)
/// ```
pub struct Progress {
    value: f32,
    variant: ProgressVariant,
    height: f32,
    width: Length,
    track_color: Color,
    radius: f32,
}

impl Progress {
    /// Create a new progress bar with a value between 0.0 and 1.0.
    #[must_use]
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            variant: ProgressVariant::default(),
            height: 6.0,
            width: Length::Fill,
            track_color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            radius: 3.0,
        }
    }

    /// Set the progress variant.
    #[must_use]
    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self.radius = height / 2.0;
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the track (background) color.
    #[must_use]
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }

    /// Use success variant.
    #[must_use]
    pub fn success(self) -> Self {
        self.variant(ProgressVariant::Success)
    }

    /// Use warning variant.
    #[must_use]
    pub fn warning(self) -> Self {
        self.variant(ProgressVariant::Warning)
    }

    /// Use error variant.
    #[must_use]
    pub fn error(self) -> Self {
        self.variant(ProgressVariant::Error)
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Progress
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Fixed(self.height))
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

        Node::new(Size::new(width, self.height))
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

        // Draw track (background)
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    radius: self.radius.into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
            },
            self.track_color,
        );

        // Draw progress fill
        if self.value > 0.0 {
            let fill_width = bounds.width * self.value;
            let fill_bounds = Rectangle {
                width: fill_width,
                ..bounds
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: fill_bounds,
                    border: iced::Border {
                        radius: self.radius.into(),
                        ..Default::default()
                    },
                    shadow: iced::Shadow::default(),
                },
                self.variant.color(),
            );
        }
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer> From<Progress> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(progress: Progress) -> Self {
        Element::new(progress)
    }
}
