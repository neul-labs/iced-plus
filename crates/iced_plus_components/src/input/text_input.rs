//! Enhanced text input component.

use std::borrow::Cow;

use iced::widget::text_input;
use iced::{Element, Length};
use iced_plus_theme::{AppTheme, TextInputClass};

/// A styled text input component with label and helper text support.
pub struct TextInput<'a, Message> {
    id: Option<text_input::Id>,
    placeholder: Cow<'a, str>,
    value: &'a str,
    label: Option<Cow<'a, str>>,
    helper: Option<Cow<'a, str>>,
    error: Option<Cow<'a, str>>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    width: Length,
    padding: f32,
    size: f32,
    class: TextInputClass,
    secure: bool,
}

impl<'a, Message> TextInput<'a, Message> {
    /// Create a new text input.
    #[must_use]
    pub fn new(placeholder: impl Into<Cow<'a, str>>, value: &'a str) -> Self {
        Self {
            id: None,
            placeholder: placeholder.into(),
            value,
            label: None,
            helper: None,
            error: None,
            on_input: None,
            on_submit: None,
            width: Length::Fill,
            padding: 10.0,
            size: 14.0,
            class: TextInputClass::Default,
            secure: false,
        }
    }

    /// Set the input ID for focusing.
    #[must_use]
    pub fn id(mut self, id: text_input::Id) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the label text.
    #[must_use]
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the helper text.
    #[must_use]
    pub fn helper(mut self, helper: impl Into<Cow<'a, str>>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    /// Set the error message.
    #[must_use]
    pub fn error(mut self, error: impl Into<Cow<'a, str>>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// Set the callback for input changes.
    #[must_use]
    pub fn on_input<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_input = Some(Box::new(f));
        self
    }

    /// Set the message to emit on submit (Enter key).
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the padding.
    #[must_use]
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the font size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Use the filled style variant.
    #[must_use]
    pub fn filled(mut self) -> Self {
        self.class = TextInputClass::Filled;
        self
    }

    /// Make this a password input.
    #[must_use]
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Make this a password input (alias for secure(true)).
    #[must_use]
    pub fn password(self) -> Self {
        self.secure(true)
    }
}

impl<'a, Message: Clone + 'a> From<TextInput<'a, Message>> for Element<'a, Message, AppTheme<'a>> {
    fn from(input: TextInput<'a, Message>) -> Self {
        let mut widget = text_input(&input.placeholder, input.value)
            .padding(input.padding)
            .size(input.size)
            .width(input.width)
            .class(input.class)
            .secure(input.secure);

        if let Some(id) = input.id {
            widget = widget.id(id);
        }

        if let Some(on_input) = input.on_input {
            widget = widget.on_input(on_input);
        }

        if let Some(msg) = input.on_submit {
            widget = widget.on_submit(msg);
        }

        // TODO: Wrap with label and helper text using VStack
        widget.into()
    }
}

impl<'a, Message: Clone + 'a> From<TextInput<'a, Message>> for Element<'a, Message, iced::Theme> {
    fn from(input: TextInput<'a, Message>) -> Self {
        let mut widget = text_input(&input.placeholder, input.value)
            .padding(input.padding)
            .size(input.size)
            .width(input.width)
            .secure(input.secure);

        if let Some(id) = input.id {
            widget = widget.id(id);
        }

        if let Some(on_input) = input.on_input {
            widget = widget.on_input(on_input);
        }

        if let Some(msg) = input.on_submit {
            widget = widget.on_submit(msg);
        }

        widget.into()
    }
}
