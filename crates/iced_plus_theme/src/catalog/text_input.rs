//! Text input styling for iced.

use iced::widget::text_input;
use iced::{Background, Border, Color};
use iced_plus_tokens::Shade;

use crate::theme::AppTheme;

/// Text input style class for iced's Catalog system.
#[derive(Debug, Clone, Copy, Default)]
pub enum TextInputClass {
    /// Default bordered input.
    #[default]
    Default,
    /// Filled input with background.
    Filled,
}

impl<'a> text_input::Catalog for AppTheme<'a> {
    type Class<'b> = TextInputClass;

    fn default<'b>() -> Self::Class<'b> {
        TextInputClass::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let is_dark = self.name() == "dark";

        match class {
            TextInputClass::Default => default_style(self, status, is_dark),
            TextInputClass::Filled => filled_style(self, status, is_dark),
        }
    }
}

fn default_style(theme: &AppTheme, status: text_input::Status, is_dark: bool) -> text_input::Style {
    let bg = if is_dark {
        theme.neutral(Shade::S900)
    } else {
        Color::WHITE
    };

    let (border_color, border_width) = match status {
        text_input::Status::Active => (
            if is_dark { theme.neutral(Shade::S700) } else { theme.neutral(Shade::S300) },
            1.0,
        ),
        text_input::Status::Hovered => (
            if is_dark { theme.neutral(Shade::S600) } else { theme.neutral(Shade::S400) },
            1.0,
        ),
        text_input::Status::Focused => (theme.primary(Shade::S500), 2.0),
        text_input::Status::Disabled => (
            if is_dark { theme.neutral(Shade::S800) } else { theme.neutral(Shade::S200) },
            1.0,
        ),
    };

    let (text_color, placeholder_color) = match status {
        text_input::Status::Disabled => (
            if is_dark { theme.neutral(Shade::S500) } else { theme.neutral(Shade::S400) },
            if is_dark { theme.neutral(Shade::S600) } else { theme.neutral(Shade::S300) },
        ),
        _ => (
            if is_dark { theme.neutral(Shade::S100) } else { theme.neutral(Shade::S900) },
            if is_dark { theme.neutral(Shade::S500) } else { theme.neutral(Shade::S400) },
        ),
    };

    text_input::Style {
        background: Background::Color(bg),
        border: Border {
            radius: theme.radius().md().into(),
            width: border_width,
            color: border_color,
        },
        icon: text_color,
        placeholder: placeholder_color,
        value: text_color,
        selection: theme.primary(Shade::S200),
    }
}

fn filled_style(theme: &AppTheme, status: text_input::Status, is_dark: bool) -> text_input::Style {
    let bg = match status {
        text_input::Status::Active | text_input::Status::Hovered => {
            if is_dark {
                theme.neutral(Shade::S800)
            } else {
                theme.neutral(Shade::S100)
            }
        }
        text_input::Status::Focused => {
            if is_dark {
                theme.neutral(Shade::S800)
            } else {
                theme.neutral(Shade::S50)
            }
        }
        text_input::Status::Disabled => {
            if is_dark {
                theme.neutral(Shade::S900)
            } else {
                theme.neutral(Shade::S100)
            }
        }
    };

    let border_color = match status {
        text_input::Status::Focused => theme.primary(Shade::S500),
        _ => Color::TRANSPARENT,
    };

    let border_width = match status {
        text_input::Status::Focused => 2.0,
        _ => 0.0,
    };

    let (text_color, placeholder_color) = match status {
        text_input::Status::Disabled => (
            if is_dark { theme.neutral(Shade::S500) } else { theme.neutral(Shade::S400) },
            if is_dark { theme.neutral(Shade::S600) } else { theme.neutral(Shade::S300) },
        ),
        _ => (
            if is_dark { theme.neutral(Shade::S100) } else { theme.neutral(Shade::S900) },
            if is_dark { theme.neutral(Shade::S500) } else { theme.neutral(Shade::S400) },
        ),
    };

    text_input::Style {
        background: Background::Color(bg),
        border: Border {
            radius: theme.radius().md().into(),
            width: border_width,
            color: border_color,
        },
        icon: text_color,
        placeholder: placeholder_color,
        value: text_color,
        selection: theme.primary(Shade::S200),
    }
}
