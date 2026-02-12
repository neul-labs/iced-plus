//! Button styling for iced.

use iced::widget::button;
use iced::{Background, Border, Color};
use iced_plus_tokens::Shade;

use crate::theme::AppTheme;

/// Button style class for iced's Catalog system.
#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonClass {
    /// Primary button - main call-to-action.
    #[default]
    Primary,
    /// Secondary button - alternative actions.
    Secondary,
    /// Ghost button - minimal styling.
    Ghost,
    /// Destructive button - dangerous actions.
    Destructive,
    /// Outline button - bordered with transparent background.
    Outline,
}

impl<'a> button::Catalog for AppTheme<'a> {
    type Class<'b> = ButtonClass;

    fn default<'b>() -> Self::Class<'b> {
        ButtonClass::Primary
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let is_dark = self.name() == "dark";

        match class {
            ButtonClass::Primary => primary_style(self, status, is_dark),
            ButtonClass::Secondary => secondary_style(self, status, is_dark),
            ButtonClass::Ghost => ghost_style(self, status, is_dark),
            ButtonClass::Destructive => destructive_style(self, status, is_dark),
            ButtonClass::Outline => outline_style(self, status, is_dark),
        }
    }
}

fn primary_style(theme: &AppTheme, status: button::Status, _is_dark: bool) -> button::Style {
    let (bg_shade, text_color) = match status {
        button::Status::Active => (Shade::S500, Color::WHITE),
        button::Status::Hovered => (Shade::S600, Color::WHITE),
        button::Status::Pressed => (Shade::S700, Color::WHITE),
        button::Status::Disabled => (Shade::S300, Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
    };

    button::Style {
        background: Some(Background::Color(theme.primary(bg_shade))),
        text_color,
        border: Border {
            radius: theme.radius().md().into(),
            ..Border::default()
        },
        shadow: iced::Shadow::default(),
    }
}

fn secondary_style(theme: &AppTheme, status: button::Status, is_dark: bool) -> button::Style {
    let (bg_shade, text_shade) = match status {
        button::Status::Active => {
            if is_dark {
                (Shade::S700, Shade::S100)
            } else {
                (Shade::S100, Shade::S700)
            }
        }
        button::Status::Hovered => {
            if is_dark {
                (Shade::S600, Shade::S50)
            } else {
                (Shade::S200, Shade::S800)
            }
        }
        button::Status::Pressed => {
            if is_dark {
                (Shade::S500, Shade::S50)
            } else {
                (Shade::S300, Shade::S900)
            }
        }
        button::Status::Disabled => (Shade::S200, Shade::S400),
    };

    button::Style {
        background: Some(Background::Color(theme.neutral(bg_shade))),
        text_color: theme.neutral(text_shade),
        border: Border {
            radius: theme.radius().md().into(),
            ..Border::default()
        },
        shadow: iced::Shadow::default(),
    }
}

fn ghost_style(theme: &AppTheme, status: button::Status, is_dark: bool) -> button::Style {
    let (bg, text_shade) = match status {
        button::Status::Active => (
            Color::TRANSPARENT,
            if is_dark { Shade::S100 } else { Shade::S700 },
        ),
        button::Status::Hovered => {
            let bg = if is_dark {
                Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.05)
            };
            (bg, if is_dark { Shade::S50 } else { Shade::S900 })
        }
        button::Status::Pressed => {
            let bg = if is_dark {
                Color::from_rgba(1.0, 1.0, 1.0, 0.15)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.1)
            };
            (bg, if is_dark { Shade::S50 } else { Shade::S900 })
        }
        button::Status::Disabled => (Color::TRANSPARENT, Shade::S400),
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: theme.neutral(text_shade),
        border: Border {
            radius: theme.radius().md().into(),
            ..Border::default()
        },
        shadow: iced::Shadow::default(),
    }
}

fn destructive_style(theme: &AppTheme, status: button::Status, _is_dark: bool) -> button::Style {
    let (bg_shade, text_color) = match status {
        button::Status::Active => (Shade::S500, Color::WHITE),
        button::Status::Hovered => (Shade::S600, Color::WHITE),
        button::Status::Pressed => (Shade::S700, Color::WHITE),
        button::Status::Disabled => (Shade::S300, Color::from_rgba(1.0, 1.0, 1.0, 0.5)),
    };

    button::Style {
        background: Some(Background::Color(theme.destructive(bg_shade))),
        text_color,
        border: Border {
            radius: theme.radius().md().into(),
            ..Border::default()
        },
        shadow: iced::Shadow::default(),
    }
}

fn outline_style(theme: &AppTheme, status: button::Status, is_dark: bool) -> button::Style {
    let (bg, border_shade, text_shade) = match status {
        button::Status::Active => (
            Color::TRANSPARENT,
            if is_dark { Shade::S600 } else { Shade::S300 },
            if is_dark { Shade::S100 } else { Shade::S700 },
        ),
        button::Status::Hovered => {
            let bg = if is_dark {
                Color::from_rgba(1.0, 1.0, 1.0, 0.05)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.02)
            };
            (
                bg,
                if is_dark { Shade::S500 } else { Shade::S400 },
                if is_dark { Shade::S50 } else { Shade::S900 },
            )
        }
        button::Status::Pressed => {
            let bg = if is_dark {
                Color::from_rgba(1.0, 1.0, 1.0, 0.1)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.05)
            };
            (
                bg,
                if is_dark { Shade::S400 } else { Shade::S500 },
                if is_dark { Shade::S50 } else { Shade::S900 },
            )
        }
        button::Status::Disabled => (Color::TRANSPARENT, Shade::S300, Shade::S400),
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: theme.neutral(text_shade),
        border: Border {
            radius: theme.radius().md().into(),
            width: 1.0,
            color: theme.neutral(border_shade),
        },
        shadow: iced::Shadow::default(),
    }
}
