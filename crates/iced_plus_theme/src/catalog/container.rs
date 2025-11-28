//! Container styling for iced.

use iced::widget::container;
use iced::{Background, Border};
use iced_plus_tokens::Shade;

use crate::theme::AppTheme;

/// Container style class for iced's Catalog system.
#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerClass {
    /// Transparent container (no styling).
    #[default]
    Transparent,
    /// Card with background and subtle border.
    Card,
    /// Surface with background color.
    Surface,
    /// Bordered container.
    Bordered,
}

impl<'a> container::Catalog for AppTheme<'a> {
    type Class<'b> = ContainerClass;

    fn default<'b>() -> Self::Class<'b> {
        ContainerClass::Transparent
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        let is_dark = self.name() == "dark";

        match class {
            ContainerClass::Transparent => container::Style::default(),
            ContainerClass::Card => card_style(self, is_dark),
            ContainerClass::Surface => surface_style(self, is_dark),
            ContainerClass::Bordered => bordered_style(self, is_dark),
        }
    }
}

fn card_style(theme: &AppTheme, is_dark: bool) -> container::Style {
    let bg = if is_dark {
        theme.neutral(Shade::S800)
    } else {
        iced::Color::WHITE
    };

    let border_color = if is_dark {
        theme.neutral(Shade::S700)
    } else {
        theme.neutral(Shade::S200)
    };

    container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            radius: theme.radius().lg().into(),
            width: 1.0,
            color: border_color,
        },
        ..container::Style::default()
    }
}

fn surface_style(theme: &AppTheme, is_dark: bool) -> container::Style {
    let bg = if is_dark {
        theme.neutral(Shade::S800)
    } else {
        theme.neutral(Shade::S50)
    };

    container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            radius: theme.radius().md().into(),
            ..Border::default()
        },
        ..container::Style::default()
    }
}

fn bordered_style(theme: &AppTheme, is_dark: bool) -> container::Style {
    let border_color = if is_dark {
        theme.neutral(Shade::S700)
    } else {
        theme.neutral(Shade::S200)
    };

    container::Style {
        background: None,
        border: Border {
            radius: theme.radius().md().into(),
            width: 1.0,
            color: border_color,
        },
        ..container::Style::default()
    }
}
