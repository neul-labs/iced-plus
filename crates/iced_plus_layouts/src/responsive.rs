//! Responsive layout helpers for adaptive UIs.
//!
//! Provides breakpoint-aware layouts that adapt based on container width.

use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer;
use iced::advanced::widget::{tree, Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::{event, mouse, Element, Event, Length, Rectangle, Size};

/// Standard breakpoint values in pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Breakpoints {
    /// Extra small: 0-479px (mobile portrait)
    pub xs: f32,
    /// Small: 480-767px (mobile landscape / small tablet)
    pub sm: f32,
    /// Medium: 768-1023px (tablet)
    pub md: f32,
    /// Large: 1024-1279px (desktop)
    pub lg: f32,
    /// Extra large: 1280px+ (large desktop)
    pub xl: f32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            xs: 0.0,
            sm: 480.0,
            md: 768.0,
            lg: 1024.0,
            xl: 1280.0,
        }
    }
}

impl Breakpoints {
    /// Standard breakpoints matching common device sizes.
    pub const STANDARD: Self = Self {
        xs: 0.0,
        sm: 480.0,
        md: 768.0,
        lg: 1024.0,
        xl: 1280.0,
    };

    /// Compact breakpoints for denser layouts.
    pub const COMPACT: Self = Self {
        xs: 0.0,
        sm: 360.0,
        md: 600.0,
        lg: 840.0,
        xl: 1080.0,
    };

    /// Get the current breakpoint tier for a given width.
    #[must_use]
    pub fn tier(&self, width: f32) -> BreakpointTier {
        if width >= self.xl {
            BreakpointTier::XL
        } else if width >= self.lg {
            BreakpointTier::LG
        } else if width >= self.md {
            BreakpointTier::MD
        } else if width >= self.sm {
            BreakpointTier::SM
        } else {
            BreakpointTier::XS
        }
    }
}

/// Breakpoint tier enum for pattern matching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BreakpointTier {
    /// Extra small (mobile)
    XS,
    /// Small (large mobile / small tablet)
    SM,
    /// Medium (tablet)
    MD,
    /// Large (desktop)
    LG,
    /// Extra large (large desktop)
    XL,
}

impl BreakpointTier {
    /// Check if this tier is at least the given tier.
    #[must_use]
    pub fn is_at_least(self, tier: Self) -> bool {
        self >= tier
    }

    /// Check if this tier is at most the given tier.
    #[must_use]
    pub fn is_at_most(self, tier: Self) -> bool {
        self <= tier
    }
}

/// Responsive value that changes based on breakpoint.
#[derive(Debug, Clone)]
pub struct Responsive<T> {
    xs: T,
    sm: Option<T>,
    md: Option<T>,
    lg: Option<T>,
    xl: Option<T>,
}

impl<T: Clone> Responsive<T> {
    /// Create a responsive value with a base (xs) value.
    pub fn new(base: T) -> Self {
        Self {
            xs: base,
            sm: None,
            md: None,
            lg: None,
            xl: None,
        }
    }

    /// Set the value for small breakpoint and up.
    #[must_use]
    pub fn sm(mut self, value: T) -> Self {
        self.sm = Some(value);
        self
    }

    /// Set the value for medium breakpoint and up.
    #[must_use]
    pub fn md(mut self, value: T) -> Self {
        self.md = Some(value);
        self
    }

    /// Set the value for large breakpoint and up.
    #[must_use]
    pub fn lg(mut self, value: T) -> Self {
        self.lg = Some(value);
        self
    }

    /// Set the value for extra large breakpoint and up.
    #[must_use]
    pub fn xl(mut self, value: T) -> Self {
        self.xl = Some(value);
        self
    }

    /// Get the value for a given breakpoint tier.
    #[must_use]
    pub fn get(&self, tier: BreakpointTier) -> &T {
        match tier {
            BreakpointTier::XL => self
                .xl
                .as_ref()
                .or(self.lg.as_ref())
                .or(self.md.as_ref())
                .or(self.sm.as_ref())
                .unwrap_or(&self.xs),
            BreakpointTier::LG => self
                .lg
                .as_ref()
                .or(self.md.as_ref())
                .or(self.sm.as_ref())
                .unwrap_or(&self.xs),
            BreakpointTier::MD => self.md.as_ref().or(self.sm.as_ref()).unwrap_or(&self.xs),
            BreakpointTier::SM => self.sm.as_ref().unwrap_or(&self.xs),
            BreakpointTier::XS => &self.xs,
        }
    }
}

/// A wrapper that shows content only at certain breakpoints.
pub struct ShowOn<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    min_tier: Option<BreakpointTier>,
    max_tier: Option<BreakpointTier>,
    breakpoints: Breakpoints,
    current_width: f32,
}

impl<'a, Message, Theme, Renderer> ShowOn<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a ShowOn wrapper for content.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            min_tier: None,
            max_tier: None,
            breakpoints: Breakpoints::default(),
            current_width: 1024.0, // Default to desktop
        }
    }

    /// Only show at this breakpoint tier or larger.
    #[must_use]
    pub fn min(mut self, tier: BreakpointTier) -> Self {
        self.min_tier = Some(tier);
        self
    }

    /// Only show at this breakpoint tier or smaller.
    #[must_use]
    pub fn max(mut self, tier: BreakpointTier) -> Self {
        self.max_tier = Some(tier);
        self
    }

    /// Set custom breakpoints.
    #[must_use]
    pub fn breakpoints(mut self, breakpoints: Breakpoints) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    fn should_show(&self, width: f32) -> bool {
        let tier = self.breakpoints.tier(width);

        let min_ok = self.min_tier.map_or(true, |min| tier >= min);
        let max_ok = self.max_tier.map_or(true, |max| tier <= max);

        min_ok && max_ok
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ShowOn<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        if self.should_show(self.current_width) {
            self.content.as_widget().size()
        } else {
            Size::new(Length::Fixed(0.0), Length::Fixed(0.0))
        }
    }

    fn tag(&self) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let width = limits.max().width;

        if self.should_show(width) {
            self.content.as_widget().layout(tree, renderer, limits)
        } else {
            Node::new(Size::ZERO)
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if layout.bounds().width > 0.0 {
            self.content
                .as_widget()
                .operate(tree, layout, renderer, operation);
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
        if layout.bounds().width > 0.0 {
            self.content.as_widget_mut().on_event(
                tree, event, layout, cursor, renderer, clipboard, shell, viewport,
            )
        } else {
            event::Status::Ignored
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        if layout.bounds().width > 0.0 {
            self.content
                .as_widget()
                .mouse_interaction(tree, layout, cursor, viewport, renderer)
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
        if layout.bounds().width > 0.0 {
            self.content
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ShowOn<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(show_on: ShowOn<'a, Message, Theme, Renderer>) -> Self {
        Element::new(show_on)
    }
}

/// Convenience function to create a ShowOn wrapper.
pub fn show_on<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> ShowOn<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    ShowOn::new(content)
}

/// Convenience function to hide content on certain breakpoints.
pub fn hide_on<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    tier: BreakpointTier,
) -> ShowOn<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    // Hide on this tier means show on everything else
    // This is a simplification - for complex cases, use ShowOn directly
    ShowOn::new(content).min(match tier {
        BreakpointTier::XS => BreakpointTier::SM,
        BreakpointTier::SM => BreakpointTier::MD,
        BreakpointTier::MD => BreakpointTier::LG,
        BreakpointTier::LG => BreakpointTier::XL,
        BreakpointTier::XL => BreakpointTier::XL, // Can't hide XL only with this
    })
}

/// A responsive row that stacks vertically on small screens.
///
/// On screens smaller than the `stack_below` tier, children are arranged
/// vertically. On larger screens, they're arranged horizontally.
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_layouts::{ResponsiveRow, BreakpointTier};
///
/// ResponsiveRow::new()
///     .stack_below(BreakpointTier::MD)  // Stack on mobile/tablet
///     .spacing(16.0)
///     .push(card1)
///     .push(card2)
///     .push(card3)
/// ```
pub struct ResponsiveRow<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer> {
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    spacing: f32,
    stack_below: BreakpointTier,
    breakpoints: Breakpoints,
}

impl<'a, Message, Theme, Renderer> ResponsiveRow<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Create a new responsive row.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 8.0,
            stack_below: BreakpointTier::MD,
            breakpoints: Breakpoints::default(),
        }
    }

    /// Add a child element.
    #[must_use]
    pub fn push(mut self, child: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Set the spacing between children.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the breakpoint below which to stack vertically.
    #[must_use]
    pub fn stack_below(mut self, tier: BreakpointTier) -> Self {
        self.stack_below = tier;
        self
    }

    /// Use custom breakpoints.
    #[must_use]
    pub fn breakpoints(mut self, breakpoints: Breakpoints) -> Self {
        self.breakpoints = breakpoints;
        self
    }

    fn should_stack(&self, width: f32) -> bool {
        let tier = self.breakpoints.tier(width);
        tier < self.stack_below
    }
}

impl<'a, Message, Theme, Renderer> Default for ResponsiveRow<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for ResponsiveRow<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn state(&self) -> tree::State {
        tree::State::None
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let max_width = limits.max().width;
        let should_stack = self.should_stack(max_width);

        if should_stack {
            // Vertical layout
            let mut y = 0.0;
            let mut max_w = 0.0_f32;
            let child_limits = Limits::new(Size::ZERO, Size::new(max_width, f32::INFINITY));

            let nodes: Vec<Node> = self
                .children
                .iter()
                .zip(tree.children.iter_mut())
                .map(|(child, child_tree)| {
                    let node = child
                        .as_widget()
                        .layout(child_tree, renderer, &child_limits);
                    let size = node.size();
                    let node = node.move_to(iced::Point::new(0.0, y));
                    y += size.height + self.spacing;
                    max_w = max_w.max(size.width);
                    node
                })
                .collect();

            // Remove last spacing
            if !nodes.is_empty() {
                y -= self.spacing;
            }

            Node::with_children(Size::new(max_w.max(max_width), y), nodes)
        } else {
            // Horizontal layout - equal width for all children
            let child_count = self.children.len();
            if child_count == 0 {
                return Node::new(Size::new(max_width, 0.0));
            }

            let total_spacing = self.spacing * (child_count - 1) as f32;
            let child_width = (max_width - total_spacing) / child_count as f32;
            let child_limits = Limits::new(Size::ZERO, Size::new(child_width, f32::INFINITY));

            let mut x = 0.0;
            let mut max_h = 0.0_f32;

            let nodes: Vec<Node> = self
                .children
                .iter()
                .zip(tree.children.iter_mut())
                .map(|(child, child_tree)| {
                    let node = child
                        .as_widget()
                        .layout(child_tree, renderer, &child_limits);
                    let size = node.size();
                    let node = node.move_to(iced::Point::new(x, 0.0));
                    x += child_width + self.spacing;
                    max_h = max_h.max(size.height);
                    node
                })
                .collect();

            Node::with_children(Size::new(max_width, max_h), nodes)
        }
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(tree.children.iter_mut())
            .zip(layout.children())
        {
            child
                .as_widget()
                .operate(child_tree, child_layout, renderer, operation);
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
        for ((child, child_tree), child_layout) in self
            .children
            .iter_mut()
            .zip(tree.children.iter_mut())
            .zip(layout.children())
        {
            let status = child.as_widget_mut().on_event(
                child_tree,
                event.clone(),
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );

            if status == event::Status::Captured {
                return event::Status::Captured;
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
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(tree.children.iter())
            .zip(layout.children())
        {
            let interaction = child.as_widget().mouse_interaction(
                child_tree,
                child_layout,
                cursor,
                viewport,
                renderer,
            );

            if interaction != mouse::Interaction::default() {
                return interaction;
            }
        }

        mouse::Interaction::default()
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
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(tree.children.iter())
            .zip(layout.children())
        {
            child.as_widget().draw(
                child_tree,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ResponsiveRow<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(row: ResponsiveRow<'a, Message, Theme, Renderer>) -> Self {
        Element::new(row)
    }
}

/// Convenience function to create a responsive row.
pub fn responsive_row<'a, Message, Theme, Renderer>() -> ResponsiveRow<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    ResponsiveRow::new()
}
