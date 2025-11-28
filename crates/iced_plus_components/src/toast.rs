//! Toast notification component.
//!
//! Provides toast notifications that appear as overlays.
//!
//! # Example
//!
//! ```rust,ignore
//! // In your app state:
//! toasts: Vec<(usize, String, ToastVariant)>,
//! toast_id_counter: usize,
//!
//! // In your view:
//! toast_container(
//!     your_content,
//!     &self.toasts,
//!     Message::CloseToast,
//!     ToastPosition::TopRight,
//! )
//!
//! // In your update:
//! Message::ShowToast(msg, variant) => {
//!     self.toasts.push((self.toast_id_counter, msg, variant));
//!     self.toast_id_counter += 1;
//! }
//! Message::CloseToast(id) => {
//!     self.toasts.retain(|(i, _, _)| *i != id);
//! }
//! ```

use iced::widget::{button, column, container, row, text, Space};
use iced::{
    alignment, Background, Border, Color, Element, Length, Padding, Theme,
};
use std::borrow::Cow;
use std::time::Duration;

/// Toast variant/severity.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastVariant {
    /// Informational toast.
    #[default]
    Info,
    /// Success toast.
    Success,
    /// Warning toast.
    Warning,
    /// Error toast.
    Error,
}

impl ToastVariant {
    /// Get the icon name for this variant.
    #[must_use]
    pub fn icon_name(&self) -> crate::icons::IconName {
        match self {
            Self::Info => crate::icons::IconName::Info,
            Self::Success => crate::icons::IconName::Success,
            Self::Warning => crate::icons::IconName::Warning,
            Self::Error => crate::icons::IconName::Error,
        }
    }

    /// Get the background color for this variant.
    #[must_use]
    pub fn background_color(&self) -> Color {
        match self {
            Self::Info => Color::from_rgb(0.2, 0.5, 0.8),
            Self::Success => Color::from_rgb(0.2, 0.7, 0.3),
            Self::Warning => Color::from_rgb(0.9, 0.7, 0.2),
            Self::Error => Color::from_rgb(0.8, 0.2, 0.2),
        }
    }

    /// Get the text color for this variant (light for most).
    #[must_use]
    pub fn text_color(&self) -> Color {
        match self {
            Self::Warning => Color::from_rgb(0.1, 0.1, 0.1),
            _ => Color::WHITE,
        }
    }
}

/// Toast position on screen.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToastPosition {
    /// Top right corner.
    #[default]
    TopRight,
    /// Top left corner.
    TopLeft,
    /// Bottom right corner.
    BottomRight,
    /// Bottom left corner.
    BottomLeft,
    /// Top center.
    TopCenter,
    /// Bottom center.
    BottomCenter,
}

/// A toast notification definition.
///
/// Toasts are typically managed by a toast manager/container at the app level.
///
/// # Example
///
/// ```rust,ignore
/// let toast = Toast::success("File saved successfully!")
///     .duration(Duration::from_secs(3));
/// ```
#[derive(Clone)]
pub struct Toast<'a> {
    /// Toast message.
    pub message: Cow<'a, str>,
    /// Optional title.
    pub title: Option<Cow<'a, str>>,
    /// Toast variant.
    pub variant: ToastVariant,
    /// Auto-dismiss duration (None = manual dismiss).
    pub duration: Option<Duration>,
    /// Whether the toast is dismissible.
    pub dismissible: bool,
}

impl<'a> Toast<'a> {
    /// Create a new toast.
    #[must_use]
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
            title: None,
            variant: ToastVariant::default(),
            duration: Some(Duration::from_secs(5)),
            dismissible: true,
        }
    }

    /// Create an info toast.
    #[must_use]
    pub fn info(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).variant(ToastVariant::Info)
    }

    /// Create a success toast.
    #[must_use]
    pub fn success(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).variant(ToastVariant::Success)
    }

    /// Create a warning toast.
    #[must_use]
    pub fn warning(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).variant(ToastVariant::Warning)
    }

    /// Create an error toast.
    #[must_use]
    pub fn error(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).variant(ToastVariant::Error)
    }

    /// Set the variant.
    #[must_use]
    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Set a title.
    #[must_use]
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the auto-dismiss duration.
    #[must_use]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Make the toast persist until manually dismissed.
    #[must_use]
    pub fn persistent(mut self) -> Self {
        self.duration = None;
        self
    }

    /// Set whether the toast is dismissible.
    #[must_use]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }
}

/// Toast manager state for tracking active toasts.
pub struct ToastManager<'a> {
    /// Active toasts.
    pub toasts: Vec<Toast<'a>>,
    /// Maximum number of visible toasts.
    pub max_visible: usize,
}

impl<'a> ToastManager<'a> {
    /// Create a new toast manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max_visible: 5,
        }
    }

    /// Set the maximum number of visible toasts.
    #[must_use]
    pub fn max_visible(mut self, max: usize) -> Self {
        self.max_visible = max;
        self
    }

    /// Add a toast.
    pub fn push(&mut self, toast: Toast<'a>) {
        self.toasts.push(toast);
        // Remove oldest if over limit
        while self.toasts.len() > self.max_visible {
            self.toasts.remove(0);
        }
    }

    /// Remove a toast by index.
    pub fn remove(&mut self, index: usize) {
        if index < self.toasts.len() {
            self.toasts.remove(index);
        }
    }

    /// Clear all toasts.
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

impl<'a> Default for ToastManager<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a single toast notification.
///
/// # Arguments
/// * `id` - Unique identifier for this toast
/// * `message` - Toast message text
/// * `variant` - Toast severity/type
/// * `on_close` - Message to emit when close button is clicked
pub fn toast_view<'a, Message>(
    id: usize,
    message: &'a str,
    variant: ToastVariant,
    on_close: impl Fn(usize) -> Message + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let bg_color = variant.background_color();
    let text_color = variant.text_color();
    let icon: Element<'a, Message, Theme> = crate::icons::Icon::new(variant.icon_name())
        .size(16.0)
        .color(text_color)
        .into();

    let close_icon: Element<'a, Message, Theme> = crate::icons::Icon::close()
        .size(14.0)
        .color(text_color)
        .into();

    let content = row![
        icon,
        Space::with_width(8),
        text(message).size(14).color(text_color),
        Space::with_width(Length::Fill),
        button(close_icon)
            .style(move |_theme, _status| button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color,
                ..button::Style::default()
            })
            .padding(Padding::from([4, 6]))
            .on_press(on_close(id)),
    ]
    .align_y(alignment::Vertical::Center)
    .spacing(4);

    container(content)
        .padding(Padding::from([12, 16]))
        .style(move |_theme| container::Style {
            background: Some(Background::Color(bg_color)),
            border: Border {
                radius: 8.0.into(),
                ..Default::default()
            },
            ..container::Style::default()
        })
        .width(Length::Fixed(320.0))
        .into()
}

/// Create a toast container that overlays toasts on top of content.
///
/// This wraps your content and displays toasts in the specified position.
///
/// # Arguments
/// * `content` - The main content to display
/// * `toasts` - List of (id, message, variant) tuples
/// * `on_close` - Function to create close message from toast id
/// * `position` - Where to position the toasts
///
/// # Example
///
/// ```rust,ignore
/// toast_container(
///     your_main_content,
///     &self.toasts,
///     Message::CloseToast,
///     ToastPosition::TopRight,
/// )
/// ```
pub fn toast_container<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
    toasts: &'a [(usize, String, ToastVariant)],
    on_close: impl Fn(usize) -> Message + Copy + 'a,
    position: ToastPosition,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    use iced::widget::{stack, Row, Column};

    if toasts.is_empty() {
        return content.into();
    }

    let toast_list: Element<'a, Message, Theme> = column(
        toasts
            .iter()
            .map(|(id, msg, variant)| toast_view(*id, msg, *variant, on_close))
            .collect::<Vec<_>>(),
    )
    .spacing(8)
    .into();

    // Create a positioned toast container that doesn't block mouse events
    // by only covering the area needed for the toasts
    let toast_container_el = container(toast_list)
        .padding(16);

    // Position the toasts using row/column alignment without a full-screen container
    // This prevents the overlay from capturing scroll events
    let toast_overlay: Element<'a, Message, Theme> = match position {
        ToastPosition::TopRight => {
            Column::new()
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(toast_container_el)
                )
                .push(Space::with_height(Length::Fill))
                .into()
        }
        ToastPosition::TopLeft => {
            Column::new()
                .push(
                    Row::new()
                        .push(toast_container_el)
                        .push(Space::with_width(Length::Fill))
                )
                .push(Space::with_height(Length::Fill))
                .into()
        }
        ToastPosition::BottomRight => {
            Column::new()
                .push(Space::with_height(Length::Fill))
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(toast_container_el)
                )
                .into()
        }
        ToastPosition::BottomLeft => {
            Column::new()
                .push(Space::with_height(Length::Fill))
                .push(
                    Row::new()
                        .push(toast_container_el)
                        .push(Space::with_width(Length::Fill))
                )
                .into()
        }
        ToastPosition::TopCenter => {
            Column::new()
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(toast_container_el)
                        .push(Space::with_width(Length::Fill))
                )
                .push(Space::with_height(Length::Fill))
                .into()
        }
        ToastPosition::BottomCenter => {
            Column::new()
                .push(Space::with_height(Length::Fill))
                .push(
                    Row::new()
                        .push(Space::with_width(Length::Fill))
                        .push(toast_container_el)
                        .push(Space::with_width(Length::Fill))
                )
                .into()
        }
    };

    stack![content.into(), toast_overlay].into()
}

/// Create a toast container with default top-right positioning.
pub fn toasts<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
    toasts: &'a [(usize, String, ToastVariant)],
    on_close: impl Fn(usize) -> Message + Copy + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    toast_container(content, toasts, on_close, ToastPosition::TopRight)
}
