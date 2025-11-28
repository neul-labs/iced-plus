//! Color scale with typed shade access.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An RGBA color with f32 components in the range 0.0..=1.0.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    /// Red component (0.0..=1.0)
    pub r: f32,
    /// Green component (0.0..=1.0)
    pub g: f32,
    /// Blue component (0.0..=1.0)
    pub b: f32,
    /// Alpha component (0.0..=1.0)
    pub a: f32,
}

impl Color {
    /// Create a new color from RGBA components.
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque color from RGB components.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create a color from 8-bit RGB values.
    #[must_use]
    pub const fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    /// Create a color from a hex value (0xRRGGBB).
    #[must_use]
    pub const fn from_hex(hex: u32) -> Self {
        Self::from_rgb8(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    /// Return this color with a different alpha value.
    #[must_use]
    pub const fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    /// Transparent color.
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Black color.
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);

    /// White color.
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

/// Shade level for color scales (50-900).
///
/// Each shade represents a specific lightness level in the color scale,
/// following the convention used by Tailwind CSS and Material Design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum Shade {
    /// Lightest shade (50)
    S50 = 0,
    /// Very light shade (100)
    S100 = 1,
    /// Light shade (200)
    S200 = 2,
    /// Light-medium shade (300)
    S300 = 3,
    /// Medium shade (400)
    S400 = 4,
    /// Base shade (500)
    S500 = 5,
    /// Medium-dark shade (600)
    S600 = 6,
    /// Dark shade (700)
    S700 = 7,
    /// Very dark shade (800)
    S800 = 8,
    /// Darkest shade (900)
    S900 = 9,
}

impl Shade {
    /// All shades in order from lightest to darkest.
    pub const ALL: [Self; 10] = [
        Self::S50,
        Self::S100,
        Self::S200,
        Self::S300,
        Self::S400,
        Self::S500,
        Self::S600,
        Self::S700,
        Self::S800,
        Self::S900,
    ];

    /// Get the numeric value of the shade (50, 100, 200, etc.).
    #[must_use]
    pub const fn value(self) -> u16 {
        match self {
            Self::S50 => 50,
            Self::S100 => 100,
            Self::S200 => 200,
            Self::S300 => 300,
            Self::S400 => 400,
            Self::S500 => 500,
            Self::S600 => 600,
            Self::S700 => 700,
            Self::S800 => 800,
            Self::S900 => 900,
        }
    }

    /// Get the index of this shade (0-9).
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// A 10-step color scale from light (50) to dark (900).
///
/// Provides type-safe access to color shades with compile-time guarantees.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ColorScale {
    /// Shade 50 - lightest
    pub s50: Color,
    /// Shade 100
    pub s100: Color,
    /// Shade 200
    pub s200: Color,
    /// Shade 300
    pub s300: Color,
    /// Shade 400
    pub s400: Color,
    /// Shade 500 - base
    pub s500: Color,
    /// Shade 600
    pub s600: Color,
    /// Shade 700
    pub s700: Color,
    /// Shade 800
    pub s800: Color,
    /// Shade 900 - darkest
    pub s900: Color,
}

impl ColorScale {
    /// Create a new color scale from individual shades.
    #[must_use]
    pub const fn new(
        s50: Color,
        s100: Color,
        s200: Color,
        s300: Color,
        s400: Color,
        s500: Color,
        s600: Color,
        s700: Color,
        s800: Color,
        s900: Color,
    ) -> Self {
        Self {
            s50,
            s100,
            s200,
            s300,
            s400,
            s500,
            s600,
            s700,
            s800,
            s900,
        }
    }

    /// Create a color scale from an array of 10 colors.
    #[must_use]
    pub const fn from_array(colors: [Color; 10]) -> Self {
        Self {
            s50: colors[0],
            s100: colors[1],
            s200: colors[2],
            s300: colors[3],
            s400: colors[4],
            s500: colors[5],
            s600: colors[6],
            s700: colors[7],
            s800: colors[8],
            s900: colors[9],
        }
    }

    /// Get the color at a specific shade.
    #[must_use]
    pub const fn get(&self, shade: Shade) -> Color {
        match shade {
            Shade::S50 => self.s50,
            Shade::S100 => self.s100,
            Shade::S200 => self.s200,
            Shade::S300 => self.s300,
            Shade::S400 => self.s400,
            Shade::S500 => self.s500,
            Shade::S600 => self.s600,
            Shade::S700 => self.s700,
            Shade::S800 => self.s800,
            Shade::S900 => self.s900,
        }
    }

    /// Get the base color (shade 500).
    #[must_use]
    pub const fn base(&self) -> Color {
        self.s500
    }

    /// Get a light variant (shade 100).
    #[must_use]
    pub const fn light(&self) -> Color {
        self.s100
    }

    /// Get a dark variant (shade 700).
    #[must_use]
    pub const fn dark(&self) -> Color {
        self.s700
    }

    /// Convert to an array of colors.
    #[must_use]
    pub const fn to_array(&self) -> [Color; 10] {
        [
            self.s50, self.s100, self.s200, self.s300, self.s400, self.s500, self.s600, self.s700,
            self.s800, self.s900,
        ]
    }
}

impl Default for ColorScale {
    fn default() -> Self {
        // Default to a neutral gray scale
        Self::new(
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
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_hex() {
        let color = Color::from_hex(0xFF0000);
        assert!((color.r - 1.0).abs() < f32::EPSILON);
        assert!(color.g.abs() < f32::EPSILON);
        assert!(color.b.abs() < f32::EPSILON);
    }

    #[test]
    fn shade_value() {
        assert_eq!(Shade::S50.value(), 50);
        assert_eq!(Shade::S500.value(), 500);
        assert_eq!(Shade::S900.value(), 900);
    }

    #[test]
    fn color_scale_get() {
        let scale = ColorScale::default();
        let color = scale.get(Shade::S500);
        assert_eq!(color, scale.s500);
    }
}
