//! Tab navigation component.

use std::borrow::Cow;

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{event, mouse, Border, Color, Element, Event, Length, Point, Rectangle, Size};

/// A single tab definition.
#[derive(Clone)]
pub struct Tab<'a> {
    /// Tab label.
    pub label: Cow<'a, str>,
    /// Optional icon (as text/emoji for now).
    pub icon: Option<Cow<'a, str>>,
}

impl<'a> Tab<'a> {
    /// Create a new tab.
    #[must_use]
    pub fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            icon: None,
        }
    }

    /// Add an icon to the tab.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Tab bar component for switching between views.
///
/// # Example
///
/// ```rust,ignore
/// Tabs::new(active_tab, Message::TabSelected)
///     .push(Tab::new("Overview"))
///     .push(Tab::new("Settings"))
///     .push(Tab::new("Help"))
/// ```
pub struct Tabs<'a, Message> {
    tabs: Vec<Tab<'a>>,
    active: usize,
    on_select: Box<dyn Fn(usize) -> Message + 'a>,
    tab_width: TabWidth,
    height: f32,
}

/// Tab width mode.
#[derive(Debug, Clone, Copy, Default)]
pub enum TabWidth {
    /// Tabs shrink to fit content.
    #[default]
    Shrink,
    /// Tabs have equal width filling the container.
    Equal,
    /// Fixed width per tab.
    Fixed(f32),
}

impl<'a, Message> Tabs<'a, Message> {
    /// Create a new tab bar.
    pub fn new<F>(active: usize, on_select: F) -> Self
    where
        F: Fn(usize) -> Message + 'a,
    {
        Self {
            tabs: Vec::new(),
            active,
            on_select: Box::new(on_select),
            tab_width: TabWidth::default(),
            height: 40.0,
        }
    }

    /// Add a tab.
    #[must_use]
    pub fn push(mut self, tab: Tab<'a>) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Set the tab width mode.
    #[must_use]
    pub fn tab_width(mut self, width: TabWidth) -> Self {
        self.tab_width = width;
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    fn tab_bounds(&self, total_width: f32) -> Vec<(f32, f32)> {
        let tab_count = self.tabs.len();
        if tab_count == 0 {
            return Vec::new();
        }

        match self.tab_width {
            TabWidth::Equal => {
                let w = total_width / tab_count as f32;
                (0..tab_count).map(|i| (i as f32 * w, w)).collect()
            }
            TabWidth::Fixed(w) => (0..tab_count).map(|i| (i as f32 * w, w)).collect(),
            TabWidth::Shrink => {
                // Estimate width based on label length - use wider characters estimate
                let mut x = 0.0;
                self.tabs
                    .iter()
                    .map(|tab| {
                        // More generous width calculation: ~10px per char + padding
                        let w = (tab.label.len() as f32 * 10.0 + 48.0).max(80.0);
                        let result = (x, w);
                        x += w;
                        result
                    })
                    .collect()
            }
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Tabs<'a, Message>
where
    Message: Clone,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height))
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
        let width = limits.max().width;
        Node::new(Size::new(width, self.height))
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
                let bounds = layout.bounds();
                if bounds.contains(position) {
                    let tab_bounds = self.tab_bounds(bounds.width);
                    let rel_x = position.x - bounds.x;

                    for (i, (x, w)) in tab_bounds.iter().enumerate() {
                        if rel_x >= *x && rel_x < x + w {
                            if i != self.active {
                                shell.publish((self.on_select)(i));
                            }
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
        let tab_bounds = self.tab_bounds(bounds.width);

        // Draw background - more visible
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            },
            Color::from_rgba(0.95, 0.95, 0.97, 1.0),
        );

        // Draw tabs
        for (i, (tab, (x, w))) in self.tabs.iter().zip(tab_bounds.iter()).enumerate() {
            let tab_rect = Rectangle {
                x: bounds.x + x,
                y: bounds.y,
                width: *w,
                height: bounds.height,
            };

            let is_active = i == self.active;
            let is_hovered = cursor
                .position()
                .map(|p| tab_rect.contains(p))
                .unwrap_or(false);

            // Tab background
            let bg_color = if is_active {
                Color::WHITE
            } else if is_hovered {
                Color::from_rgba(1.0, 1.0, 1.0, 0.5)
            } else {
                Color::TRANSPARENT
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: tab_rect,
                    border: Border {
                        radius: 6.0.into(),
                        ..Default::default()
                    },
                    shadow: iced::Shadow::default(),
                },
                bg_color,
            );

            // Active indicator
            if is_active {
                let indicator = Rectangle {
                    x: tab_rect.x,
                    y: tab_rect.y + tab_rect.height - 2.0,
                    width: tab_rect.width,
                    height: 2.0,
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: indicator,
                        border: Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                    Color::from_rgb(0.22, 0.47, 0.87),
                );
            }

            // Tab label
            let text_color = if is_active {
                Color::from_rgb(0.1, 0.1, 0.1)
            } else {
                Color::from_rgb(0.4, 0.4, 0.4)
            };

            let label: String = tab.label.clone().into_owned();
            renderer.fill_text(
                iced::advanced::text::Text {
                    content: label,
                    bounds: Size::new(tab_rect.width, tab_rect.height),
                    size: iced::Pixels(14.0),
                    line_height: iced::advanced::text::LineHeight::default(),
                    font: iced::Font::default(),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    shaping: iced::advanced::text::Shaping::Basic,
                    wrapping: iced::advanced::text::Wrapping::None,
                },
                Point::new(tab_rect.x, tab_rect.y),
                text_color,
                tab_rect,
            );
        }

        // Bottom border
        let border_line = Rectangle {
            x: bounds.x,
            y: bounds.y + bounds.height - 1.0,
            width: bounds.width,
            height: 1.0,
        };
        renderer.fill_quad(
            renderer::Quad {
                bounds: border_line,
                border: Border::default(),
                shadow: iced::Shadow::default(),
            },
            Color::from_rgba(0.0, 0.0, 0.0, 0.1),
        );
    }
}

impl<'a, Message, Theme, Renderer> From<Tabs<'a, Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font> + 'a,
{
    fn from(tabs: Tabs<'a, Message>) -> Self {
        Element::new(tabs)
    }
}
