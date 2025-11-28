//! Avatar component for user/entity representation.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Size};

/// Avatar size preset.
#[derive(Debug, Clone, Copy, Default)]
pub enum AvatarSize {
    /// Extra small (24px).
    XS,
    /// Small (32px).
    Small,
    /// Medium (40px).
    #[default]
    Medium,
    /// Large (56px).
    Large,
    /// Extra large (80px).
    XL,
    /// Custom size.
    Custom(f32),
}

impl AvatarSize {
    fn pixels(self) -> f32 {
        match self {
            Self::XS => 24.0,
            Self::Small => 32.0,
            Self::Medium => 40.0,
            Self::Large => 56.0,
            Self::XL => 80.0,
            Self::Custom(px) => px,
        }
    }
}

/// Avatar shape.
#[derive(Debug, Clone, Copy, Default)]
pub enum AvatarShape {
    /// Circular avatar.
    #[default]
    Circle,
    /// Rounded square.
    Rounded,
    /// Square corners.
    Square,
}

/// An avatar component showing initials or placeholder.
///
/// # Example
///
/// ```rust,ignore
/// Avatar::new("John Doe")
///     .size(AvatarSize::Large)
///     .color(Color::from_rgb(0.2, 0.5, 0.8))
/// ```
pub struct Avatar {
    initials: String,
    size: AvatarSize,
    shape: AvatarShape,
    background_color: Color,
    text_color: Color,
}

impl Avatar {
    /// Create a new avatar from a name (extracts initials).
    #[must_use]
    pub fn new(name: &str) -> Self {
        let initials = name
            .split_whitespace()
            .take(2)
            .filter_map(|word| word.chars().next())
            .collect::<String>()
            .to_uppercase();

        let initials = if initials.is_empty() {
            "?".to_string()
        } else {
            initials
        };

        // Generate a consistent color based on the name
        let hash = name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let hue = (hash % 360) as f32;
        let background_color = hsl_to_rgb(hue, 0.5, 0.6);

        Self {
            initials,
            size: AvatarSize::default(),
            shape: AvatarShape::default(),
            background_color,
            text_color: Color::WHITE,
        }
    }

    /// Create an avatar with specific initials.
    #[must_use]
    pub fn initials(initials: impl Into<String>) -> Self {
        Self {
            initials: initials.into().to_uppercase(),
            size: AvatarSize::default(),
            shape: AvatarShape::default(),
            background_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: Color::WHITE,
        }
    }

    /// Set the size.
    #[must_use]
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Set the shape.
    #[must_use]
    pub fn shape(mut self, shape: AvatarShape) -> Self {
        self.shape = shape;
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    /// Set the text color.
    #[must_use]
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::from_rgb(r + m, g + m, b + m)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Avatar
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> Size<Length> {
        let px = self.size.pixels();
        Size::new(Length::Fixed(px), Length::Fixed(px))
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

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        let px = self.size.pixels();
        Node::new(Size::new(px, px))
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
            AvatarShape::Circle => bounds.width / 2.0,
            AvatarShape::Rounded => 8.0,
            AvatarShape::Square => 0.0,
        };

        // Draw background
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    radius: radius.into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
            },
            self.background_color,
        );

        // Draw initials
        let font_size = bounds.width * 0.4;
        renderer.fill_text(
            iced::advanced::text::Text {
                content: self.initials.clone(),
                bounds: Size::new(bounds.width, bounds.height),
                size: iced::Pixels(font_size),
                line_height: iced::advanced::text::LineHeight::default(),
                font: iced::Font::default(),
                horizontal_alignment: iced::alignment::Horizontal::Center,
                vertical_alignment: iced::alignment::Vertical::Center,
                shaping: iced::advanced::text::Shaping::Basic,
                wrapping: iced::advanced::text::Wrapping::None,
            },
            Point::new(bounds.x, bounds.y),
            self.text_color,
            bounds,
        );
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer> From<Avatar> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font> + 'a,
{
    fn from(avatar: Avatar) -> Self {
        Element::new(avatar)
    }
}
