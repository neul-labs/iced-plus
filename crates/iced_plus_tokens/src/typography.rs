//! Typography tokens for text styling.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Font weight values following CSS font-weight specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u16)]
pub enum FontWeight {
    /// Thin (100)
    Thin = 100,
    /// Extra Light (200)
    ExtraLight = 200,
    /// Light (300)
    Light = 300,
    /// Regular/Normal (400)
    Regular = 400,
    /// Medium (500)
    Medium = 500,
    /// Semi Bold (600)
    SemiBold = 600,
    /// Bold (700)
    Bold = 700,
    /// Extra Bold (800)
    ExtraBold = 800,
    /// Black (900)
    Black = 900,
}

impl FontWeight {
    /// Get the numeric weight value.
    #[must_use]
    pub const fn value(self) -> u16 {
        self as u16
    }
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Regular
    }
}

/// A complete text style definition.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextStyle {
    /// Font family name (e.g., "Inter", "system-ui")
    pub font_family: &'static str,
    /// Font size in pixels
    pub size: f32,
    /// Font weight
    pub weight: FontWeight,
    /// Line height as a multiplier (e.g., 1.5 = 150%)
    pub line_height: f32,
    /// Letter spacing in pixels (can be negative)
    pub letter_spacing: f32,
}

impl TextStyle {
    /// Create a new text style.
    #[must_use]
    pub const fn new(
        font_family: &'static str,
        size: f32,
        weight: FontWeight,
        line_height: f32,
    ) -> Self {
        Self {
            font_family,
            size,
            weight,
            line_height,
            letter_spacing: 0.0,
        }
    }

    /// Create a text style with custom letter spacing.
    #[must_use]
    pub const fn with_letter_spacing(mut self, letter_spacing: f32) -> Self {
        self.letter_spacing = letter_spacing;
        self
    }

    /// Get the computed line height in pixels.
    #[must_use]
    pub const fn line_height_px(&self) -> f32 {
        self.size * self.line_height
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new("Inter", 16.0, FontWeight::Regular, 1.5)
    }
}

/// Named text style categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextStyleName {
    /// Extra large display text
    DisplayXl,
    /// Large display text
    DisplayLg,
    /// Large heading
    HeadingLg,
    /// Medium heading
    HeadingMd,
    /// Small heading
    HeadingSm,
    /// Large body text
    BodyLg,
    /// Medium body text (default)
    BodyMd,
    /// Small body text
    BodySm,
    /// Monospace/code text
    Code,
    /// Label text
    Label,
    /// Micro text (smallest)
    Micro,
}

/// Typography scale containing all text styles.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypographyScale {
    /// Extra large display text
    pub display_xl: TextStyle,
    /// Large display text
    pub display_lg: TextStyle,
    /// Large heading
    pub heading_lg: TextStyle,
    /// Medium heading
    pub heading_md: TextStyle,
    /// Small heading
    pub heading_sm: TextStyle,
    /// Large body text
    pub body_lg: TextStyle,
    /// Medium body text
    pub body_md: TextStyle,
    /// Small body text
    pub body_sm: TextStyle,
    /// Code/monospace text
    pub code: TextStyle,
    /// Label text
    pub label: TextStyle,
    /// Micro text
    pub micro: TextStyle,
}

impl TypographyScale {
    /// Get a text style by name.
    #[must_use]
    pub fn get(&self, name: TextStyleName) -> &TextStyle {
        match name {
            TextStyleName::DisplayXl => &self.display_xl,
            TextStyleName::DisplayLg => &self.display_lg,
            TextStyleName::HeadingLg => &self.heading_lg,
            TextStyleName::HeadingMd => &self.heading_md,
            TextStyleName::HeadingSm => &self.heading_sm,
            TextStyleName::BodyLg => &self.body_lg,
            TextStyleName::BodyMd => &self.body_md,
            TextStyleName::BodySm => &self.body_sm,
            TextStyleName::Code => &self.code,
            TextStyleName::Label => &self.label,
            TextStyleName::Micro => &self.micro,
        }
    }
}

impl Default for TypographyScale {
    fn default() -> Self {
        const FONT: &str = "Inter";
        const MONO: &str = "JetBrains Mono";

        Self {
            display_xl: TextStyle::new(FONT, 60.0, FontWeight::Bold, 1.1),
            display_lg: TextStyle::new(FONT, 48.0, FontWeight::Bold, 1.1),
            heading_lg: TextStyle::new(FONT, 32.0, FontWeight::SemiBold, 1.2),
            heading_md: TextStyle::new(FONT, 24.0, FontWeight::SemiBold, 1.3),
            heading_sm: TextStyle::new(FONT, 20.0, FontWeight::SemiBold, 1.4),
            body_lg: TextStyle::new(FONT, 18.0, FontWeight::Regular, 1.6),
            body_md: TextStyle::new(FONT, 16.0, FontWeight::Regular, 1.5),
            body_sm: TextStyle::new(FONT, 14.0, FontWeight::Regular, 1.5),
            code: TextStyle::new(MONO, 14.0, FontWeight::Regular, 1.6),
            label: TextStyle::new(FONT, 12.0, FontWeight::Medium, 1.4),
            micro: TextStyle::new(FONT, 10.0, FontWeight::Medium, 1.4),
        }
    }
}
