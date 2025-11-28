//! Text styling for iced.

use iced::widget::text;
use iced_plus_tokens::Shade;

use crate::theme::AppTheme;

/// Text style class for iced's Catalog system.
#[derive(Debug, Clone, Copy, Default)]
pub enum TextClass {
    /// Default text color.
    #[default]
    Default,
    /// Muted/secondary text.
    Muted,
    /// Success text (green).
    Success,
    /// Warning text (amber).
    Warning,
    /// Error text (red).
    Error,
}

impl<'a> text::Catalog for AppTheme<'a> {
    type Class<'b> = TextClass;

    fn default<'b>() -> Self::Class<'b> {
        TextClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        let is_dark = self.name() == "dark";

        let color = match class {
            TextClass::Default => {
                if is_dark {
                    self.neutral(Shade::S100)
                } else {
                    self.neutral(Shade::S900)
                }
            }
            TextClass::Muted => {
                if is_dark {
                    self.neutral(Shade::S400)
                } else {
                    self.neutral(Shade::S500)
                }
            }
            TextClass::Success => self.success(Shade::S500),
            TextClass::Warning => self.warning(Shade::S500),
            TextClass::Error => self.destructive(Shade::S500),
        };

        text::Style { color: Some(color) }
    }
}
