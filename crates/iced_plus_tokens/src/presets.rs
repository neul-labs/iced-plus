//! Built-in theme presets.

use crate::{
    color::{Color, ColorPalette, ColorScale, SemanticColors},
    elevation::{Elevation, ElevationScale, Shadow},
    motion::MotionScale,
    radius::RadiusScale,
    spacing::SpacingScale,
    tokens::{ThemePreset, ThemeTokens},
    typography::TypographyScale,
};

/// Light theme preset.
#[cfg(feature = "preset-light")]
pub fn light() -> ThemePreset {
    ThemePreset::new("light", "Light", light_tokens())
}

/// Dark theme preset.
#[cfg(feature = "preset-dark")]
pub fn dark() -> ThemePreset {
    ThemePreset::new("dark", "Dark", dark_tokens())
}

/// Get the light theme tokens.
#[cfg(feature = "preset-light")]
pub fn light_tokens() -> ThemeTokens {
    ThemeTokens {
        colors: light_palette(),
        typography: TypographyScale::default(),
        spacing: SpacingScale::DEFAULT,
        radius: RadiusScale::DEFAULT,
        elevation: light_elevation(),
        motion: MotionScale::default(),
    }
}

/// Get the dark theme tokens.
#[cfg(feature = "preset-dark")]
pub fn dark_tokens() -> ThemeTokens {
    ThemeTokens {
        colors: dark_palette(),
        typography: TypographyScale::default(),
        spacing: SpacingScale::DEFAULT,
        radius: RadiusScale::DEFAULT,
        elevation: dark_elevation(),
        motion: MotionScale::default(),
    }
}

#[cfg(feature = "preset-light")]
fn light_palette() -> ColorPalette {
    ColorPalette {
        primary: ColorScale::new(
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
        ),
        secondary: ColorScale::new(
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
        ),
        neutral: ColorScale::new(
            Color::from_hex(0xFAFAFA),
            Color::from_hex(0xF5F5F5),
            Color::from_hex(0xE5E5E5),
            Color::from_hex(0xD4D4D4),
            Color::from_hex(0xA3A3A3),
            Color::from_hex(0x737373),
            Color::from_hex(0x525252),
            Color::from_hex(0x404040),
            Color::from_hex(0x262626),
            Color::from_hex(0x171717),
        ),
        semantic: SemanticColors::default(),
    }
}

#[cfg(feature = "preset-dark")]
fn dark_palette() -> ColorPalette {
    ColorPalette {
        primary: ColorScale::new(
            Color::from_hex(0x1E3A8A),
            Color::from_hex(0x1E40AF),
            Color::from_hex(0x1D4ED8),
            Color::from_hex(0x2563EB),
            Color::from_hex(0x3B82F6),
            Color::from_hex(0x60A5FA),
            Color::from_hex(0x93C5FD),
            Color::from_hex(0xBFDBFE),
            Color::from_hex(0xDBEAFE),
            Color::from_hex(0xEFF6FF),
        ),
        secondary: ColorScale::new(
            Color::from_hex(0x4C1D95),
            Color::from_hex(0x5B21B6),
            Color::from_hex(0x6D28D9),
            Color::from_hex(0x7C3AED),
            Color::from_hex(0x8B5CF6),
            Color::from_hex(0xA78BFA),
            Color::from_hex(0xC4B5FD),
            Color::from_hex(0xDDD6FE),
            Color::from_hex(0xEDE9FE),
            Color::from_hex(0xF5F3FF),
        ),
        neutral: ColorScale::new(
            Color::from_hex(0x171717),
            Color::from_hex(0x262626),
            Color::from_hex(0x404040),
            Color::from_hex(0x525252),
            Color::from_hex(0x737373),
            Color::from_hex(0xA3A3A3),
            Color::from_hex(0xD4D4D4),
            Color::from_hex(0xE5E5E5),
            Color::from_hex(0xF5F5F5),
            Color::from_hex(0xFAFAFA),
        ),
        semantic: dark_semantic_colors(),
    }
}

#[cfg(feature = "preset-dark")]
fn dark_semantic_colors() -> SemanticColors {
    SemanticColors {
        success: ColorScale::new(
            Color::from_hex(0x14532D),
            Color::from_hex(0x166534),
            Color::from_hex(0x15803D),
            Color::from_hex(0x16A34A),
            Color::from_hex(0x22C55E),
            Color::from_hex(0x4ADE80),
            Color::from_hex(0x86EFAC),
            Color::from_hex(0xBBF7D0),
            Color::from_hex(0xDCFCE7),
            Color::from_hex(0xF0FDF4),
        ),
        warning: ColorScale::new(
            Color::from_hex(0x78350F),
            Color::from_hex(0x92400E),
            Color::from_hex(0xB45309),
            Color::from_hex(0xD97706),
            Color::from_hex(0xF59E0B),
            Color::from_hex(0xFBBF24),
            Color::from_hex(0xFCD34D),
            Color::from_hex(0xFDE68A),
            Color::from_hex(0xFEF3C7),
            Color::from_hex(0xFFFBEB),
        ),
        destructive: ColorScale::new(
            Color::from_hex(0x7F1D1D),
            Color::from_hex(0x991B1B),
            Color::from_hex(0xB91C1C),
            Color::from_hex(0xDC2626),
            Color::from_hex(0xEF4444),
            Color::from_hex(0xF87171),
            Color::from_hex(0xFCA5A5),
            Color::from_hex(0xFECACA),
            Color::from_hex(0xFEE2E2),
            Color::from_hex(0xFEF2F2),
        ),
        info: ColorScale::new(
            Color::from_hex(0x164E63),
            Color::from_hex(0x155E75),
            Color::from_hex(0x0E7490),
            Color::from_hex(0x0891B2),
            Color::from_hex(0x06B6D4),
            Color::from_hex(0x22D3EE),
            Color::from_hex(0x67E8F9),
            Color::from_hex(0xA5F3FC),
            Color::from_hex(0xCFFAFE),
            Color::from_hex(0xECFEFF),
        ),
    }
}

#[cfg(feature = "preset-light")]
fn light_elevation() -> ElevationScale {
    let shadow_color = Color::new(0.0, 0.0, 0.0, 0.1);
    let shadow_color_strong = Color::new(0.0, 0.0, 0.0, 0.15);

    ElevationScale {
        flat: Elevation::FLAT,
        raised: Elevation::new(Shadow::new(0.0, 1.0, 3.0, 0.0, shadow_color)),
        overlay: Elevation::new(Shadow::new(0.0, 4.0, 6.0, -1.0, shadow_color)),
        floating: Elevation::new(Shadow::new(0.0, 10.0, 15.0, -3.0, shadow_color_strong)),
        modal: Elevation::new(Shadow::new(0.0, 25.0, 50.0, -12.0, shadow_color_strong)),
    }
}

#[cfg(feature = "preset-dark")]
fn dark_elevation() -> ElevationScale {
    let shadow_color = Color::new(0.0, 0.0, 0.0, 0.3);
    let shadow_color_strong = Color::new(0.0, 0.0, 0.0, 0.5);

    ElevationScale {
        flat: Elevation::FLAT,
        raised: Elevation::new(Shadow::new(0.0, 1.0, 3.0, 0.0, shadow_color)),
        overlay: Elevation::new(Shadow::new(0.0, 4.0, 6.0, -1.0, shadow_color)),
        floating: Elevation::new(Shadow::new(0.0, 10.0, 15.0, -3.0, shadow_color_strong)),
        modal: Elevation::new(Shadow::new(0.0, 25.0, 50.0, -12.0, shadow_color_strong)),
    }
}
