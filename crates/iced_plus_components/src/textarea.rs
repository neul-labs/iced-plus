//! Textarea component for multi-line text input.
//!
//! # Example
//!
//! ```rust,ignore
//! TextArea::new("Enter description...", &self.text)
//!     .on_input(Message::TextChanged)
//!     .rows(5)
//! ```

use iced::widget::{container, scrollable, text_editor, TextEditor};
use iced::{Background, Border, Element, Length, Padding, Theme};

/// Text area content state.
///
/// This wraps iced's text_editor::Content for multi-line editing.
pub type TextAreaContent = text_editor::Content;

/// A styled multi-line text area component.
///
/// Uses iced's `text_editor` widget under the hood.
///
/// # Example
///
/// ```rust,ignore
/// // In your state:
/// content: text_editor::Content::new(),
///
/// // In your view:
/// TextArea::new(&self.content)
///     .on_action(Message::TextAreaAction)
///     .placeholder("Enter text...")
///     .height(200.0)
/// ```
pub struct TextArea<'a, Message> {
    content: &'a TextAreaContent,
    on_action: Option<Box<dyn Fn(text_editor::Action) -> Message + 'a>>,
    placeholder: Option<&'a str>,
    width: Length,
    height: Length,
    padding: Padding,
}

impl<'a, Message: Clone + 'a> TextArea<'a, Message> {
    /// Create a new text area.
    pub fn new(content: &'a TextAreaContent) -> Self {
        Self {
            content,
            on_action: None,
            placeholder: None,
            width: Length::Fill,
            height: Length::Fixed(150.0),
            padding: Padding::new(12.0),
        }
    }

    /// Set the action handler for text changes.
    #[must_use]
    pub fn on_action<F>(mut self, on_action: F) -> Self
    where
        F: Fn(text_editor::Action) -> Message + 'a,
    {
        self.on_action = Some(Box::new(on_action));
        self
    }

    /// Set placeholder text (shown when empty).
    #[must_use]
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set height by number of visible rows (approximate).
    #[must_use]
    pub fn rows(self, rows: usize) -> Self {
        // Approximate 20px per row + padding
        let height = (rows as f32 * 22.0) + 24.0;
        self.height(Length::Fixed(height))
    }

    /// Set the padding.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<TextArea<'a, Message>> for Element<'a, Message, Theme> {
    fn from(textarea: TextArea<'a, Message>) -> Self {
        let mut editor = TextEditor::new(textarea.content);

        if let Some(on_action) = textarea.on_action {
            editor = editor.on_action(on_action);
        }

        if let Some(placeholder) = textarea.placeholder {
            editor = editor.placeholder(placeholder);
        }

        editor = editor
            .padding(textarea.padding)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();

                let (background, border_color) = match status {
                    text_editor::Status::Active => (
                        palette.background.base.color,
                        palette.background.strong.color,
                    ),
                    text_editor::Status::Hovered => {
                        (palette.background.base.color, palette.primary.weak.color)
                    }
                    text_editor::Status::Focused => {
                        (palette.background.base.color, palette.primary.base.color)
                    }
                    text_editor::Status::Disabled => (
                        palette.background.weak.color,
                        palette.background.strong.color,
                    ),
                };

                text_editor::Style {
                    background: Background::Color(background),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    icon: palette.background.weak.text,
                    placeholder: palette.background.weak.text,
                    value: palette.background.base.text,
                    selection: palette.primary.weak.color,
                }
            });

        container(editor)
            .width(textarea.width)
            .height(textarea.height)
            .into()
    }
}

/// Simple multi-line text input using scrollable text input approach.
///
/// This is an alternative that uses a simpler approach for basic multi-line needs.
///
/// # Example
///
/// ```rust,ignore
/// SimpleTextArea::new("Description", &self.text)
///     .on_input(Message::TextChanged)
///     .rows(4)
/// ```
pub struct SimpleTextArea<'a, Message> {
    placeholder: &'a str,
    value: &'a str,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    width: Length,
    height: Length,
}

impl<'a, Message: Clone + 'a> SimpleTextArea<'a, Message> {
    /// Create a new simple text area.
    pub fn new(placeholder: &'a str, value: &'a str) -> Self {
        Self {
            placeholder,
            value,
            on_input: None,
            width: Length::Fill,
            height: Length::Fixed(100.0),
        }
    }

    /// Set the input handler.
    #[must_use]
    pub fn on_input<F>(mut self, on_input: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_input = Some(Box::new(on_input));
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set height by rows.
    #[must_use]
    pub fn rows(self, rows: usize) -> Self {
        self.height(Length::Fixed((rows as f32 * 22.0) + 20.0))
    }
}

impl<'a, Message: Clone + 'a> From<SimpleTextArea<'a, Message>> for Element<'a, Message, Theme> {
    fn from(textarea: SimpleTextArea<'a, Message>) -> Self {
        use iced::widget::text_input;

        let mut input = text_input(textarea.placeholder, textarea.value)
            .width(textarea.width)
            .padding(12);

        if let Some(on_input) = textarea.on_input {
            input = input.on_input(on_input);
        }

        // Wrap in scrollable container for multi-line feel
        container(
            scrollable(container(input).width(Length::Fill).padding(Padding {
                top: 0.0,
                right: 4.0,
                bottom: 0.0,
                left: 0.0,
            }))
            .height(textarea.height),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.base.color)),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..container::Style::default()
            }
        })
        .width(textarea.width)
        .height(textarea.height)
        .into()
    }
}
