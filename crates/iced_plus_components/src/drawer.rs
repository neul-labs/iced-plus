//! Drawer/side panel component.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{event, mouse, Color, Element, Event, Length, Point, Rectangle, Size};

/// Drawer position.
#[derive(Debug, Clone, Copy, Default)]
pub enum DrawerPosition {
    /// Slides in from the left.
    #[default]
    Left,
    /// Slides in from the right.
    Right,
}

/// A drawer/side panel that overlays content.
///
/// # Example
///
/// ```rust,ignore
/// if show_drawer {
///     Drawer::new(base_content, drawer_content)
///         .position(DrawerPosition::Left)
///         .width(300.0)
///         .on_close(Message::CloseDrawer)
/// } else {
///     base_content.into()
/// }
/// ```
pub struct Drawer<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    base: Element<'a, Message, Theme, Renderer>,
    drawer: Element<'a, Message, Theme, Renderer>,
    position: DrawerPosition,
    width: f32,
    backdrop_opacity: f32,
    on_close: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Drawer<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new drawer.
    pub fn new(
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
        drawer: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            base: base.into(),
            drawer: drawer.into(),
            position: DrawerPosition::default(),
            width: 300.0,
            backdrop_opacity: 0.6,
            on_close: None,
        }
    }

    /// Set the drawer position.
    #[must_use]
    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the drawer width.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the backdrop opacity.
    #[must_use]
    pub fn backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the message to emit when backdrop is clicked.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Drawer<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn state(&self) -> tree::State {
        tree::State::None
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.base), Tree::new(&self.drawer)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.children[0].diff(&self.base);
        tree.children[1].diff(&self.drawer);
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let bounds = limits.max();

        // Layout base content
        let base_node = self.base.as_widget().layout(
            &mut tree.children[0],
            renderer,
            &Limits::new(Size::ZERO, bounds),
        );

        // Layout drawer
        let drawer_limits = Limits::new(Size::ZERO, Size::new(self.width, bounds.height));
        let drawer_node = self.drawer.as_widget().layout(
            &mut tree.children[1],
            renderer,
            &drawer_limits,
        );

        // Position drawer based on side
        let drawer_x = match self.position {
            DrawerPosition::Left => 0.0,
            DrawerPosition::Right => bounds.width - self.width,
        };
        let drawer_node = drawer_node.move_to(Point::new(drawer_x, 0.0));

        Node::with_children(bounds, vec![base_node, drawer_node])
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut children = layout.children();

        if let Some(base_layout) = children.next() {
            self.base
                .as_widget()
                .operate(&mut tree.children[0], base_layout, renderer, operation);
        }

        if let Some(drawer_layout) = children.next() {
            self.drawer
                .as_widget()
                .operate(&mut tree.children[1], drawer_layout, renderer, operation);
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut children = layout.children();
        let _base_layout = children.next();
        let drawer_layout = children.next();

        // First, let drawer handle events
        if let Some(drawer_layout) = drawer_layout {
            let status = self.drawer.as_widget_mut().on_event(
                &mut tree.children[1],
                event.clone(),
                drawer_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            if status == event::Status::Captured {
                return event::Status::Captured;
            }

            // Check for backdrop click
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                if let Some(position) = cursor.position() {
                    let drawer_bounds = drawer_layout.bounds();
                    if !drawer_bounds.contains(position) {
                        if let Some(message) = self.on_close.clone() {
                            shell.publish(message);
                            return event::Status::Captured;
                        }
                    }
                }
            }
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let _base_layout = children.next();

        if let Some(drawer_layout) = children.next() {
            self.drawer.as_widget().mouse_interaction(
                &tree.children[1],
                drawer_layout,
                cursor,
                viewport,
                renderer,
            )
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();

        // Draw base content
        if let Some(base_layout) = children.next() {
            self.base.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                base_layout,
                cursor,
                viewport,
            );
        }

        // Draw backdrop
        let bounds = layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            Color::from_rgba(0.0, 0.0, 0.0, self.backdrop_opacity),
        );

        // Draw drawer
        if let Some(drawer_layout) = children.next() {
            // Drawer background
            renderer.fill_quad(
                renderer::Quad {
                    bounds: drawer_layout.bounds(),
                    border: iced::Border::default(),
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                        offset: iced::Vector::new(2.0, 0.0),
                        blur_radius: 8.0,
                    },
                },
                Color::WHITE,
            );

            self.drawer.as_widget().draw(
                &tree.children[1],
                renderer,
                theme,
                style,
                drawer_layout,
                cursor,
                viewport,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Drawer<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(drawer: Drawer<'a, Message, Theme, Renderer>) -> Self {
        Element::new(drawer)
    }
}
