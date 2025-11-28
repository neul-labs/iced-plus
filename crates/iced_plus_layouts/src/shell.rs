//! Application shell layout with sidebar and main content areas.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{event, mouse, Element, Event, Length, Point, Rectangle, Size};

/// Application shell with sidebar and main content areas.
///
/// The shell provides a common desktop application layout with:
/// - Optional left sidebar
/// - Main content area
/// - Optional header
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_layouts::AppShell;
///
/// let shell = AppShell::new(main_content)
///     .sidebar(sidebar_content)
///     .sidebar_width(250.0)
///     .header(header_content);
/// ```
pub struct AppShell<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    sidebar: Option<Element<'a, Message, Theme, Renderer>>,
    header: Option<Element<'a, Message, Theme, Renderer>>,
    sidebar_width: f32,
    header_height: f32,
}

impl<'a, Message, Theme, Renderer> AppShell<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new app shell with main content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            sidebar: None,
            header: None,
            sidebar_width: 240.0,
            header_height: 48.0,
        }
    }

    /// Set the sidebar content.
    #[must_use]
    pub fn sidebar(mut self, sidebar: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.sidebar = Some(sidebar.into());
        self
    }

    /// Set the header content.
    #[must_use]
    pub fn header(mut self, header: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set the sidebar width in pixels.
    #[must_use]
    pub fn sidebar_width(mut self, width: f32) -> Self {
        self.sidebar_width = width;
        self
    }

    /// Set the header height in pixels.
    #[must_use]
    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = height;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AppShell<'a, Message, Theme, Renderer>
where
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
        let mut children = vec![Tree::new(&self.content)];

        if let Some(ref sidebar) = self.sidebar {
            children.push(Tree::new(sidebar));
        }

        if let Some(ref header) = self.header {
            children.push(Tree::new(header));
        }

        children
    }

    fn diff(&self, tree: &mut Tree) {
        let mut children_iter = tree.children.iter_mut();

        if let Some(content_tree) = children_iter.next() {
            content_tree.diff(&self.content);
        }

        if let Some(ref sidebar) = self.sidebar {
            if let Some(sidebar_tree) = children_iter.next() {
                sidebar_tree.diff(sidebar);
            }
        }

        if let Some(ref header) = self.header {
            if let Some(header_tree) = children_iter.next() {
                header_tree.diff(header);
            }
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let bounds = limits.max();

        let sidebar_width = if self.sidebar.is_some() {
            self.sidebar_width
        } else {
            0.0
        };

        let header_height = if self.header.is_some() {
            self.header_height
        } else {
            0.0
        };

        let mut children = Vec::new();
        let mut child_index = 0;

        // Content area
        let content_x = sidebar_width;
        let content_y = header_height;
        let content_width = (bounds.width - sidebar_width).max(0.0);
        let content_height = (bounds.height - header_height).max(0.0);

        let content_limits =
            Limits::new(Size::ZERO, Size::new(content_width, content_height)).shrink(Size::ZERO);

        let content_node = self
            .content
            .as_widget()
            .layout(&mut tree.children[child_index], renderer, &content_limits)
            .move_to(Point::new(content_x, content_y));
        children.push(content_node);
        child_index += 1;

        // Sidebar
        if self.sidebar.is_some() {
            let sidebar_height = bounds.height - header_height;
            let sidebar_limits = Limits::new(
                Size::ZERO,
                Size::new(self.sidebar_width, sidebar_height.max(0.0)),
            );

            let sidebar_node = self
                .sidebar
                .as_ref()
                .unwrap()
                .as_widget()
                .layout(&mut tree.children[child_index], renderer, &sidebar_limits)
                .move_to(Point::new(0.0, header_height));
            children.push(sidebar_node);
            child_index += 1;
        }

        // Header
        if self.header.is_some() {
            let header_limits =
                Limits::new(Size::ZERO, Size::new(bounds.width, self.header_height));

            let header_node = self
                .header
                .as_ref()
                .unwrap()
                .as_widget()
                .layout(&mut tree.children[child_index], renderer, &header_limits)
                .move_to(Point::ORIGIN);
            children.push(header_node);
        }

        Node::with_children(bounds, children)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut children_iter = layout.children();
        let mut tree_iter = tree.children.iter_mut();

        // Content
        if let (Some(content_layout), Some(content_tree)) =
            (children_iter.next(), tree_iter.next())
        {
            self.content
                .as_widget()
                .operate(content_tree, content_layout, renderer, operation);
        }

        // Sidebar
        if let Some(ref sidebar) = self.sidebar {
            if let (Some(sidebar_layout), Some(sidebar_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                sidebar
                    .as_widget()
                    .operate(sidebar_tree, sidebar_layout, renderer, operation);
            }
        }

        // Header
        if let Some(ref header) = self.header {
            if let (Some(header_layout), Some(header_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                header
                    .as_widget()
                    .operate(header_tree, header_layout, renderer, operation);
            }
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
        let mut children_iter = layout.children();
        let mut tree_iter = tree.children.iter_mut();

        let mut status = event::Status::Ignored;

        // Content
        if let (Some(content_layout), Some(content_tree)) =
            (children_iter.next(), tree_iter.next())
        {
            status = status.merge(self.content.as_widget_mut().on_event(
                content_tree,
                event.clone(),
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            ));
        }

        // Sidebar
        if let Some(ref mut sidebar) = self.sidebar {
            if let (Some(sidebar_layout), Some(sidebar_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                status = status.merge(sidebar.as_widget_mut().on_event(
                    sidebar_tree,
                    event.clone(),
                    sidebar_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                ));
            }
        }

        // Header
        if let Some(ref mut header) = self.header {
            if let (Some(header_layout), Some(header_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                status = status.merge(header.as_widget_mut().on_event(
                    header_tree,
                    event,
                    header_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                ));
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let mut children_iter = layout.children();
        let mut tree_iter = tree.children.iter();

        // Check each child for mouse interaction
        let mut interaction = mouse::Interaction::default();

        if let (Some(content_layout), Some(content_tree)) =
            (children_iter.next(), tree_iter.next())
        {
            interaction = self.content.as_widget().mouse_interaction(
                content_tree,
                content_layout,
                cursor,
                viewport,
                renderer,
            );
        }

        if let Some(ref sidebar) = self.sidebar {
            if let (Some(sidebar_layout), Some(sidebar_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                let sidebar_interaction = sidebar.as_widget().mouse_interaction(
                    sidebar_tree,
                    sidebar_layout,
                    cursor,
                    viewport,
                    renderer,
                );
                if sidebar_interaction != mouse::Interaction::default() {
                    interaction = sidebar_interaction;
                }
            }
        }

        if let Some(ref header) = self.header {
            if let (Some(header_layout), Some(header_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                let header_interaction = header.as_widget().mouse_interaction(
                    header_tree,
                    header_layout,
                    cursor,
                    viewport,
                    renderer,
                );
                if header_interaction != mouse::Interaction::default() {
                    interaction = header_interaction;
                }
            }
        }

        interaction
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
        let mut children_iter = layout.children();
        let mut tree_iter = tree.children.iter();

        // Draw header first (top layer)
        // Then sidebar
        // Then content (behind)

        // We need to draw in reverse order to get proper layering
        // But for this shell, the order doesn't matter as they don't overlap

        // Content
        if let (Some(content_layout), Some(content_tree)) =
            (children_iter.next(), tree_iter.next())
        {
            self.content.as_widget().draw(
                content_tree,
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        }

        // Sidebar
        if let Some(ref sidebar) = self.sidebar {
            if let (Some(sidebar_layout), Some(sidebar_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                sidebar.as_widget().draw(
                    sidebar_tree,
                    renderer,
                    theme,
                    style,
                    sidebar_layout,
                    cursor,
                    viewport,
                );
            }
        }

        // Header
        if let Some(ref header) = self.header {
            if let (Some(header_layout), Some(header_tree)) =
                (children_iter.next(), tree_iter.next())
            {
                header.as_widget().draw(
                    header_tree,
                    renderer,
                    theme,
                    style,
                    header_layout,
                    cursor,
                    viewport,
                );
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> From<AppShell<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(shell: AppShell<'a, Message, Theme, Renderer>) -> Self {
        Element::new(shell)
    }
}
