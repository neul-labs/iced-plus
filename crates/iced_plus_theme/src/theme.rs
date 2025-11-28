//! Main theme type that wraps tokens and integrates with iced.

use std::borrow::Cow;

use iced_plus_tokens::{presets, Color as TokenColor, Shade, ThemeTokens};

/// The main theme type for iced-plus applications.
///
/// `AppTheme` wraps design tokens and implements iced's styling traits,
/// providing a unified theming experience.
///
/// # Example
///
/// ```rust
/// use iced_plus_theme::AppTheme;
///
/// // Create a light theme
/// let theme = AppTheme::light();
///
/// // Access spacing tokens
/// let spacing = theme.spacing().lg();
/// ```
#[derive(Debug, Clone)]
pub struct AppTheme<'a> {
    tokens: Cow<'a, ThemeTokens>,
    name: &'static str,
}

impl AppTheme<'static> {
    /// Create a light theme.
    #[must_use]
    pub fn light() -> Self {
        Self {
            tokens: Cow::Owned(presets::light_tokens()),
            name: "light",
        }
    }

    /// Create a dark theme.
    #[must_use]
    pub fn dark() -> Self {
        Self {
            tokens: Cow::Owned(presets::dark_tokens()),
            name: "dark",
        }
    }
}

impl<'a> AppTheme<'a> {
    /// Create a theme from custom tokens.
    #[must_use]
    pub fn custom(tokens: ThemeTokens, name: &'static str) -> Self {
        Self {
            tokens: Cow::Owned(tokens),
            name,
        }
    }

    /// Create a theme from borrowed tokens (zero-copy).
    #[must_use]
    pub fn from_ref(tokens: &'a ThemeTokens, name: &'static str) -> Self {
        Self {
            tokens: Cow::Borrowed(tokens),
            name,
        }
    }

    /// Get the theme name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Get the underlying tokens.
    #[must_use]
    pub fn tokens(&self) -> &ThemeTokens {
        &self.tokens
    }

    /// Get the spacing scale.
    #[must_use]
    pub fn spacing(&self) -> &iced_plus_tokens::SpacingScale {
        &self.tokens.spacing
    }

    /// Get the color palette.
    #[must_use]
    pub fn colors(&self) -> &iced_plus_tokens::ColorPalette {
        &self.tokens.colors
    }

    /// Get the typography scale.
    #[must_use]
    pub fn typography(&self) -> &iced_plus_tokens::TypographyScale {
        &self.tokens.typography
    }

    /// Get the radius scale.
    #[must_use]
    pub fn radius(&self) -> &iced_plus_tokens::RadiusScale {
        &self.tokens.radius
    }

    /// Get the elevation scale.
    #[must_use]
    pub fn elevation(&self) -> &iced_plus_tokens::ElevationScale {
        &self.tokens.elevation
    }

    /// Get the motion scale.
    #[must_use]
    pub fn motion(&self) -> &iced_plus_tokens::MotionScale {
        &self.tokens.motion
    }

    // Color helpers

    /// Get the primary color at a specific shade.
    #[must_use]
    pub fn primary(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.primary.get(shade))
    }

    /// Get the secondary color at a specific shade.
    #[must_use]
    pub fn secondary(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.secondary.get(shade))
    }

    /// Get the neutral color at a specific shade.
    #[must_use]
    pub fn neutral(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.neutral.get(shade))
    }

    /// Get the success color at a specific shade.
    #[must_use]
    pub fn success(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.semantic.success.get(shade))
    }

    /// Get the warning color at a specific shade.
    #[must_use]
    pub fn warning(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.semantic.warning.get(shade))
    }

    /// Get the destructive color at a specific shade.
    #[must_use]
    pub fn destructive(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.semantic.destructive.get(shade))
    }

    /// Get the info color at a specific shade.
    #[must_use]
    pub fn info(&self, shade: Shade) -> iced::Color {
        token_to_iced(self.tokens.colors.semantic.info.get(shade))
    }

    /// Get the background color (neutral shade 50 for light, 900 for dark).
    #[must_use]
    pub fn background(&self) -> iced::Color {
        if self.name == "dark" {
            self.neutral(Shade::S900)
        } else {
            self.neutral(Shade::S50)
        }
    }

    /// Get the surface color (slightly elevated background).
    #[must_use]
    pub fn surface(&self) -> iced::Color {
        if self.name == "dark" {
            self.neutral(Shade::S800)
        } else {
            iced::Color::WHITE
        }
    }

    /// Get the text color (high contrast against background).
    #[must_use]
    pub fn text(&self) -> iced::Color {
        if self.name == "dark" {
            self.neutral(Shade::S100)
        } else {
            self.neutral(Shade::S900)
        }
    }

    /// Get the muted text color (lower contrast).
    #[must_use]
    pub fn text_muted(&self) -> iced::Color {
        if self.name == "dark" {
            self.neutral(Shade::S400)
        } else {
            self.neutral(Shade::S500)
        }
    }

    /// Get the border color.
    #[must_use]
    pub fn border(&self) -> iced::Color {
        if self.name == "dark" {
            self.neutral(Shade::S700)
        } else {
            self.neutral(Shade::S200)
        }
    }
}

impl Default for AppTheme<'static> {
    fn default() -> Self {
        Self::light()
    }
}

/// Convert a token color to an iced color.
#[must_use]
pub fn token_to_iced(color: TokenColor) -> iced::Color {
    iced::Color::from_rgba(color.r, color.g, color.b, color.a)
}
