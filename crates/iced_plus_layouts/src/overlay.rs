//! Overlay layouts for modals, dialogs, and floating elements.

use iced::widget::{center, container, mouse_area, opaque, stack};
use iced::{Color, Element, Length};

/// Creates a modal overlay that displays content over a backdrop.
///
/// Uses iced's built-in `stack`, `opaque`, `mouse_area`, and `center` widgets
/// for proper overlay rendering with a semi-transparent backdrop.
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_layouts::modal;
///
/// if show_modal {
///     modal(base_content, modal_content, Message::CloseModal)
/// } else {
///     base_content.into()
/// }
/// ```
pub fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    modal_with_opacity(base, content, on_blur, 0.8)
}

/// Creates a modal overlay with custom backdrop opacity.
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_layouts::modal_with_opacity;
///
/// modal_with_opacity(base, modal_content, Message::Close, 0.5)
/// ```
pub fn modal_with_opacity<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
    opacity: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let opacity = opacity.clamp(0.0, 1.0);

    stack![
        base.into(),
        opaque(
            mouse_area(
                center(opaque(content))
                    .style(move |_theme| {
                        container::Style {
                            background: Some(
                                Color {
                                    a: opacity,
                                    ..Color::BLACK
                                }
                                .into(),
                            ),
                            ..container::Style::default()
                        }
                    })
            )
            .on_press(on_blur)
        )
    ]
    .into()
}

/// Modal builder for more control over the overlay.
///
/// # Example
///
/// ```rust,ignore
/// Modal::new(base_content, modal_content)
///     .backdrop_opacity(0.7)
///     .on_backdrop_press(Message::CloseModal)
///     .into()
/// ```
pub struct Modal<'a, Message> {
    base: Element<'a, Message>,
    content: Element<'a, Message>,
    backdrop_opacity: f32,
    on_backdrop_press: Option<Message>,
}

impl<'a, Message> Modal<'a, Message>
where
    Message: Clone + 'a,
{
    /// Create a new modal overlay.
    pub fn new(
        base: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self {
            base: base.into(),
            content: content.into(),
            backdrop_opacity: 0.8,
            on_backdrop_press: None,
        }
    }

    /// Set the backdrop opacity (0.0 - 1.0).
    #[must_use]
    pub fn backdrop_opacity(mut self, opacity: f32) -> Self {
        self.backdrop_opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Set the message to emit when the backdrop is pressed.
    #[must_use]
    pub fn on_backdrop_press(mut self, message: Message) -> Self {
        self.on_backdrop_press = Some(message);
        self
    }
}

impl<'a, Message> From<Modal<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(modal: Modal<'a, Message>) -> Self {
        let opacity = modal.backdrop_opacity;

        // Create the backdrop with centered content
        let backdrop_content: Element<'a, Message> = center(opaque(modal.content))
            .style(move |_theme| container::Style {
                background: Some(
                    Color {
                        a: opacity,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            })
            .into();

        // Wrap with mouse_area if we have a blur handler
        let overlay: Element<'a, Message> = if let Some(on_blur) = modal.on_backdrop_press {
            opaque(mouse_area(backdrop_content).on_press(on_blur)).into()
        } else {
            opaque(backdrop_content).into()
        };

        stack![modal.base, overlay].into()
    }
}

/// Creates a drawer overlay that slides in from the side.
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_layouts::drawer;
///
/// if show_drawer {
///     drawer(base_content, drawer_content, Message::CloseDrawer)
/// } else {
///     base_content.into()
/// }
/// ```
pub fn drawer<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    drawer_left(base, content, on_close, 280.0)
}

/// Creates a drawer overlay from the left with custom width.
pub fn drawer_left<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
    width: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    use iced::widget::row;

    let drawer_content: Element<'a, Message> = container(content)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(|theme: &iced::Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(palette.background.base.color.into()),
                ..container::Style::default()
            }
        })
        .into();

    let backdrop: Element<'a, Message> = mouse_area(
        container(iced::widget::Space::new(Length::Fill, Length::Fill)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.5,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }),
    )
    .on_press(on_close)
    .into();

    let overlay: Element<'a, Message> = row![opaque(drawer_content), backdrop]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    stack![base.into(), overlay].into()
}

/// Creates a drawer overlay from the right with custom width.
pub fn drawer_right<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
    width: f32,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    use iced::widget::row;

    let drawer_content: Element<'a, Message> = container(content)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(|theme: &iced::Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(palette.background.base.color.into()),
                ..container::Style::default()
            }
        })
        .into();

    let backdrop: Element<'a, Message> = mouse_area(
        container(iced::widget::Space::new(Length::Fill, Length::Fill)).style(|_theme| {
            container::Style {
                background: Some(
                    Color {
                        a: 0.5,
                        ..Color::BLACK
                    }
                    .into(),
                ),
                ..container::Style::default()
            }
        }),
    )
    .on_press(on_close)
    .into();

    let overlay: Element<'a, Message> = row![backdrop, opaque(drawer_content)]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    stack![base.into(), overlay].into()
}
