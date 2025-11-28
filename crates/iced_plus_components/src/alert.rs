//! Alert component for contextual feedback messages.

use std::borrow::Cow;

use iced::widget::{container, row, text};
use iced::{Background, Border, Color, Element, Length};

/// Alert severity/type.
#[derive(Debug, Clone, Copy, Default)]
pub enum AlertType {
    /// Informational alert.
    #[default]
    Info,
    /// Success alert.
    Success,
    /// Warning alert.
    Warning,
    /// Error/danger alert.
    Error,
}

impl AlertType {
    fn colors(self) -> (Color, Color, Color) {
        // (background, border, text)
        match self {
            Self::Info => (
                Color::from_rgb(0.93, 0.95, 0.99),
                Color::from_rgb(0.22, 0.47, 0.87),
                Color::from_rgb(0.15, 0.33, 0.60),
            ),
            Self::Success => (
                Color::from_rgb(0.92, 0.98, 0.94),
                Color::from_rgb(0.13, 0.70, 0.40),
                Color::from_rgb(0.10, 0.50, 0.30),
            ),
            Self::Warning => (
                Color::from_rgb(1.0, 0.98, 0.92),
                Color::from_rgb(0.95, 0.65, 0.15),
                Color::from_rgb(0.60, 0.40, 0.10),
            ),
            Self::Error => (
                Color::from_rgb(0.99, 0.93, 0.93),
                Color::from_rgb(0.87, 0.24, 0.24),
                Color::from_rgb(0.60, 0.15, 0.15),
            ),
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Info => "ℹ",
            Self::Success => "✓",
            Self::Warning => "⚠",
            Self::Error => "✕",
        }
    }
}

/// An alert banner for contextual feedback.
///
/// # Example
///
/// ```rust,ignore
/// Alert::info("This is an informational message.")
/// Alert::error("Something went wrong!")
///     .title("Error")
/// ```
pub struct Alert<'a> {
    message: Cow<'a, str>,
    title: Option<Cow<'a, str>>,
    alert_type: AlertType,
    show_icon: bool,
}

impl<'a> Alert<'a> {
    /// Create a new alert.
    #[must_use]
    pub fn new(message: impl Into<Cow<'a, str>>) -> Self {
        Self {
            message: message.into(),
            title: None,
            alert_type: AlertType::default(),
            show_icon: true,
        }
    }

    /// Create an info alert.
    #[must_use]
    pub fn info(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).alert_type(AlertType::Info)
    }

    /// Create a success alert.
    #[must_use]
    pub fn success(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).alert_type(AlertType::Success)
    }

    /// Create a warning alert.
    #[must_use]
    pub fn warning(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).alert_type(AlertType::Warning)
    }

    /// Create an error alert.
    #[must_use]
    pub fn error(message: impl Into<Cow<'a, str>>) -> Self {
        Self::new(message).alert_type(AlertType::Error)
    }

    /// Set the alert type.
    #[must_use]
    pub fn alert_type(mut self, alert_type: AlertType) -> Self {
        self.alert_type = alert_type;
        self
    }

    /// Set a title for the alert.
    #[must_use]
    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Hide the icon.
    #[must_use]
    pub fn no_icon(mut self) -> Self {
        self.show_icon = false;
        self
    }
}

impl<'a, Message: 'a> From<Alert<'a>> for Element<'a, Message, iced::Theme> {
    fn from(alert: Alert<'a>) -> Self {
        let (bg_color, border_color, text_color) = alert.alert_type.colors();
        let message: String = alert.message.into_owned();

        let mut content_row = row![].spacing(8);

        if alert.show_icon {
            content_row = content_row.push(
                text(alert.alert_type.icon())
                    .size(16)
                    .color(border_color),
            );
        }

        let text_content = if let Some(title) = alert.title {
            let title_str: String = title.into_owned();
            iced::widget::column![
                text(title_str).size(14).color(text_color),
                text(message).size(13).color(text_color),
            ]
            .spacing(4)
        } else {
            iced::widget::column![text(message).size(14).color(text_color)]
        };

        content_row = content_row.push(text_content);

        container(content_row)
            .padding(12)
            .width(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(Background::Color(bg_color)),
                border: Border {
                    radius: 6.0.into(),
                    width: 1.0,
                    color: border_color,
                },
                ..Default::default()
            })
            .into()
    }
}
