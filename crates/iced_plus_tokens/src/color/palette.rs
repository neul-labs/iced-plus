//! Color palette with semantic roles.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::scale::{Color, ColorScale};

/// Complete color palette for a theme.
///
/// Contains all color scales organized by their semantic role.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColorPalette {
    /// Primary brand color scale
    pub primary: ColorScale,
    /// Secondary brand color scale
    pub secondary: ColorScale,
    /// Neutral/gray color scale for text, borders, backgrounds
    pub neutral: ColorScale,
    /// Semantic colors for status and feedback
    pub semantic: SemanticColors,
}

impl ColorPalette {
    /// Create a new color palette.
    #[must_use]
    pub const fn new(
        primary: ColorScale,
        secondary: ColorScale,
        neutral: ColorScale,
        semantic: SemanticColors,
    ) -> Self {
        Self {
            primary,
            secondary,
            neutral,
            semantic,
        }
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            secondary: default_secondary(),
            neutral: ColorScale::default(),
            semantic: SemanticColors::default(),
        }
    }
}

/// Semantic colors for status and feedback.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SemanticColors {
    /// Success state color scale (typically green)
    pub success: ColorScale,
    /// Warning state color scale (typically yellow/amber)
    pub warning: ColorScale,
    /// Error/destructive state color scale (typically red)
    pub destructive: ColorScale,
    /// Informational state color scale (typically blue)
    pub info: ColorScale,
}

impl SemanticColors {
    /// Create new semantic colors.
    #[must_use]
    pub const fn new(
        success: ColorScale,
        warning: ColorScale,
        destructive: ColorScale,
        info: ColorScale,
    ) -> Self {
        Self {
            success,
            warning,
            destructive,
            info,
        }
    }
}

impl Default for SemanticColors {
    fn default() -> Self {
        Self {
            success: default_success(),
            warning: default_warning(),
            destructive: default_destructive(),
            info: default_info(),
        }
    }
}

// Default color scales based on Tailwind CSS colors

fn default_primary() -> ColorScale {
    // Blue scale
    ColorScale::new(
        Color::from_hex(0xEFF6FF),
        Color::from_hex(0xDBEAFE),
        Color::from_hex(0xBFDBFE),
        Color::from_hex(0x93C5FD),
        Color::from_hex(0x60A5FA),
        Color::from_hex(0x3B82F6),
        Color::from_hex(0x2563EB),
        Color::from_hex(0x1D4ED8),
        Color::from_hex(0x1E40AF),
        Color::from_hex(0x1E3A8A),
    )
}

fn default_secondary() -> ColorScale {
    // Violet scale
    ColorScale::new(
        Color::from_hex(0xF5F3FF),
        Color::from_hex(0xEDE9FE),
        Color::from_hex(0xDDD6FE),
        Color::from_hex(0xC4B5FD),
        Color::from_hex(0xA78BFA),
        Color::from_hex(0x8B5CF6),
        Color::from_hex(0x7C3AED),
        Color::from_hex(0x6D28D9),
        Color::from_hex(0x5B21B6),
        Color::from_hex(0x4C1D95),
    )
}

fn default_success() -> ColorScale {
    // Green scale
    ColorScale::new(
        Color::from_hex(0xF0FDF4),
        Color::from_hex(0xDCFCE7),
        Color::from_hex(0xBBF7D0),
        Color::from_hex(0x86EFAC),
        Color::from_hex(0x4ADE80),
        Color::from_hex(0x22C55E),
        Color::from_hex(0x16A34A),
        Color::from_hex(0x15803D),
        Color::from_hex(0x166534),
        Color::from_hex(0x14532D),
    )
}

fn default_warning() -> ColorScale {
    // Amber scale
    ColorScale::new(
        Color::from_hex(0xFFFBEB),
        Color::from_hex(0xFEF3C7),
        Color::from_hex(0xFDE68A),
        Color::from_hex(0xFCD34D),
        Color::from_hex(0xFBBF24),
        Color::from_hex(0xF59E0B),
        Color::from_hex(0xD97706),
        Color::from_hex(0xB45309),
        Color::from_hex(0x92400E),
        Color::from_hex(0x78350F),
    )
}

fn default_destructive() -> ColorScale {
    // Red scale
    ColorScale::new(
        Color::from_hex(0xFEF2F2),
        Color::from_hex(0xFEE2E2),
        Color::from_hex(0xFECACA),
        Color::from_hex(0xFCA5A5),
        Color::from_hex(0xF87171),
        Color::from_hex(0xEF4444),
        Color::from_hex(0xDC2626),
        Color::from_hex(0xB91C1C),
        Color::from_hex(0x991B1B),
        Color::from_hex(0x7F1D1D),
    )
}

fn default_info() -> ColorScale {
    // Cyan scale
    ColorScale::new(
        Color::from_hex(0xECFEFF),
        Color::from_hex(0xCFFAFE),
        Color::from_hex(0xA5F3FC),
        Color::from_hex(0x67E8F9),
        Color::from_hex(0x22D3EE),
        Color::from_hex(0x06B6D4),
        Color::from_hex(0x0891B2),
        Color::from_hex(0x0E7490),
        Color::from_hex(0x155E75),
        Color::from_hex(0x164E63),
    )
}
