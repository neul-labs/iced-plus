//! Toggle switch component.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{event, mouse, Color, Element, Event, Length, Rectangle, Size};

/// A toggle switch component.
///
/// # Example
///
/// ```rust,ignore
/// Switch::new(is_enabled, Message::ToggleEnabled)
///     .label("Enable feature")
/// ```
pub struct Switch<'a, Message> {
    is_on: bool,
    on_toggle: Box<dyn Fn(bool) -> Message + 'a>,
    label: Option<String>,
    width: f32,
    height: f32,
}

impl<'a, Message> Switch<'a, Message> {
    /// Create a new switch.
    pub fn new<F>(is_on: bool, on_toggle: F) -> Self
    where
        F: Fn(bool) -> Message + 'a,
    {
        Self {
            is_on,
            on_toggle: Box::new(on_toggle),
            label: None,
            width: 44.0,
            height: 24.0,
        }
    }

    /// Set a label for the switch.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set custom dimensions.
    #[must_use]
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Switch<'a, Message>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
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
        Node::new(Size::new(self.width, self.height))
    }

    fn on_event(
        &mut self,
        _tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            if let Some(position) = cursor.position() {
                if layout.bounds().contains(position) {
                    shell.publish((self.on_toggle)(!self.is_on));
                    return event::Status::Captured;
                }
            }
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let is_hovered = cursor.is_over(bounds);

        // Track colors
        let track_color = if self.is_on {
            if is_hovered {
                Color::from_rgb(0.18, 0.40, 0.75)
            } else {
                Color::from_rgb(0.22, 0.47, 0.87)
            }
        } else if is_hovered {
            Color::from_rgba(0.0, 0.0, 0.0, 0.25)
        } else {
            Color::from_rgba(0.0, 0.0, 0.0, 0.2)
        };

        // Draw track
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    radius: (self.height / 2.0).into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
            },
            track_color,
        );

        // Draw thumb
        let thumb_size = self.height - 4.0;
        let thumb_x = if self.is_on {
            bounds.x + bounds.width - thumb_size - 2.0
        } else {
            bounds.x + 2.0
        };

        let thumb_bounds = Rectangle {
            x: thumb_x,
            y: bounds.y + 2.0,
            width: thumb_size,
            height: thumb_size,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: thumb_bounds,
                border: iced::Border {
                    radius: (thumb_size / 2.0).into(),
                    ..Default::default()
                },
                shadow: iced::Shadow::default(),
            },
            Color::WHITE,
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Switch<'a, Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(switch: Switch<'a, Message>) -> Self {
        Element::new(switch)
    }
}
