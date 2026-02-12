//! Type-safe stack layouts with direction encoding.
//!
//! Provides `HStack` and `VStack` as type aliases for `Stack<Horizontal>`
//! and `Stack<Vertical>` respectively.

use std::marker::PhantomData;

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::event;
use iced::{Element, Event, Length, Point, Rectangle, Size};
use smallvec::SmallVec;

use crate::direction::{Direction, Horizontal, Vertical};

/// A stack layout that arranges children in a single direction.
///
/// The direction is encoded in the type parameter `D`:
/// - `Stack<Horizontal>` arranges children left to right (HStack)
/// - `Stack<Vertical>` arranges children top to bottom (VStack)
///
/// Cross-axis alignment is type-safe: `HStack` only accepts `Vertical` alignment,
/// and `VStack` only accepts `Horizontal` alignment.
pub struct Stack<'a, D, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    D: Direction,
{
    children: SmallVec<[Element<'a, Message, Theme, Renderer>; 8]>,
    spacing: f32,
    padding: f32,
    width: Length,
    height: Length,
    align: D::CrossAlign,
    _direction: PhantomData<D>,
}

/// Horizontal stack (left to right).
pub type HStack<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> =
    Stack<'a, Horizontal, Message, Theme, Renderer>;

/// Vertical stack (top to bottom).
pub type VStack<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> =
    Stack<'a, Vertical, Message, Theme, Renderer>;

impl<'a, D, Message, Theme, Renderer> Stack<'a, D, Message, Theme, Renderer>
where
    D: Direction,
{
    /// Create a new empty stack.
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: SmallVec::new(),
            spacing: 0.0,
            padding: 0.0,
            width: Length::Shrink,
            height: Length::Shrink,
            align: D::default_align(),
            _direction: PhantomData,
        }
    }

    /// Set the spacing between children.
    #[must_use]
    pub fn spacing(mut self, spacing: impl Into<f32>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Set the padding around all children.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<f32>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Set the width of the stack.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height of the stack.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set the cross-axis alignment.
    ///
    /// For `HStack`, this sets vertical alignment (top, center, bottom).
    /// For `VStack`, this sets horizontal alignment (left, center, right).
    #[must_use]
    pub fn align(mut self, align: D::CrossAlign) -> Self {
        self.align = align;
        self
    }

    /// Add a child element.
    #[must_use]
    pub fn push(mut self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Add a child element conditionally.
    #[must_use]
    pub fn push_if(
        self,
        condition: bool,
        child: impl FnOnce() -> Element<'a, Message, Theme, Renderer>,
    ) -> Self {
        if condition {
            self.push(child())
        } else {
            self
        }
    }

    /// Add an optional child element.
    #[must_use]
    pub fn push_maybe(
        self,
        child: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        if let Some(c) = child {
            self.push(c)
        } else {
            self
        }
    }

    /// Add multiple children from an iterator.
    #[must_use]
    pub fn extend(
        mut self,
        children: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.children.extend(children);
        self
    }
}

impl<'a, D, Message, Theme, Renderer> Default for Stack<'a, D, Message, Theme, Renderer>
where
    D: Direction,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, D, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Stack<'a, D, Message, Theme, Renderer>
where
    D: Direction,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let padding = self.padding;

        if self.children.is_empty() {
            return layout::Node::new(limits.resolve(self.width, self.height, Size::ZERO));
        }

        // For horizontal: main axis is width, cross axis is height
        // For vertical: main axis is height, cross axis is width
        let is_horizontal = D::is_horizontal();

        let mut nodes = Vec::with_capacity(self.children.len());
        let mut main_axis_used = padding * 2.0;
        let mut cross_axis_max: f32 = 0.0;

        for (i, child) in self.children.iter().enumerate() {
            let child_limits = if is_horizontal {
                layout::Limits::new(Size::ZERO, Size::new(f32::INFINITY, limits.max().height))
            } else {
                layout::Limits::new(Size::ZERO, Size::new(limits.max().width, f32::INFINITY))
            };

            let node = child
                .as_widget()
                .layout(&mut tree.children[i], renderer, &child_limits);
            let size = node.size();

            if is_horizontal {
                main_axis_used += size.width;
                cross_axis_max = cross_axis_max.max(size.height);
            } else {
                main_axis_used += size.height;
                cross_axis_max = cross_axis_max.max(size.width);
            }

            nodes.push(node);
        }

        // Add spacing between children
        main_axis_used += self.spacing * (self.children.len().saturating_sub(1)) as f32;

        // Calculate total size
        let total_size = if is_horizontal {
            Size::new(main_axis_used, cross_axis_max + padding * 2.0)
        } else {
            Size::new(cross_axis_max + padding * 2.0, main_axis_used)
        };

        let total_size = limits.resolve(self.width, self.height, total_size);

        // Position children
        let mut main_offset = padding;
        let cross_available = if is_horizontal {
            total_size.height - padding * 2.0
        } else {
            total_size.width - padding * 2.0
        };

        for node in &mut nodes {
            let size = node.size();
            let cross_size = if is_horizontal {
                size.height
            } else {
                size.width
            };

            // Calculate cross-axis offset based on alignment
            let cross_offset = match self.align.into() {
                iced::alignment::Alignment::Start => padding,
                iced::alignment::Alignment::Center => {
                    padding + (cross_available - cross_size) / 2.0
                }
                iced::alignment::Alignment::End => padding + cross_available - cross_size,
            };

            let position = if is_horizontal {
                Point::new(main_offset, cross_offset)
            } else {
                Point::new(cross_offset, main_offset)
            };

            node.move_to_mut(position);

            main_offset += if is_horizontal {
                size.width
            } else {
                size.height
            } + self.spacing;
        }

        layout::Node::with_children(total_size, nodes)
    }

    fn children(&self) -> Vec<widget::Tree> {
        self.children.iter().map(widget::Tree::new).collect()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&self.children);
    }

    fn operate(
        &self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .operate(state, layout, renderer, operation);
        }
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut status = event::Status::Ignored;

        for ((child, state), layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            let child_status = child.as_widget_mut().on_event(
                state,
                event.clone(),
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            if child_status == event::Status::Captured {
                status = event::Status::Captured;
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced::mouse::Interaction {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            let interaction = child
                .as_widget()
                .mouse_interaction(state, layout, cursor, viewport, renderer);

            if interaction != iced::mouse::Interaction::default() {
                return interaction;
            }
        }

        iced::mouse::Interaction::default()
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport);
        }
    }
}

impl<'a, D, Message, Theme, Renderer> From<Stack<'a, D, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    D: Direction + 'a,
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(stack: Stack<'a, D, Message, Theme, Renderer>) -> Self {
        Self::new(stack)
    }
}
