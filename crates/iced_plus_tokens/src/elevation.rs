//! Elevation and shadow tokens.

use crate::color::Color;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Shadow definition for elevation effects.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Shadow {
    /// Horizontal offset
    pub offset_x: f32,
    /// Vertical offset
    pub offset_y: f32,
    /// Blur radius
    pub blur: f32,
    /// Spread radius
    pub spread: f32,
    /// Shadow color
    pub color: Color,
}

impl Shadow {
    /// Create a new shadow.
    #[must_use]
    pub const fn new(offset_x: f32, offset_y: f32, blur: f32, spread: f32, color: Color) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            spread,
            color,
        }
    }

    /// No shadow.
    pub const NONE: Self = Self::new(0.0, 0.0, 0.0, 0.0, Color::TRANSPARENT);
}

impl Default for Shadow {
    fn default() -> Self {
        Self::NONE
    }
}

/// Named elevation levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum ElevationLevel {
    /// Flat - no elevation
    Flat = 0,
    /// Raised - slight elevation (cards, buttons)
    Raised = 1,
    /// Overlay - medium elevation (dropdowns, popovers)
    Overlay = 2,
    /// Floating - high elevation (floating buttons, toasts)
    Floating = 3,
    /// Modal - highest elevation (modals, dialogs)
    Modal = 4,
}

impl ElevationLevel {
    /// Get the index for array access.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Get the z-index value for this elevation level.
    #[must_use]
    pub const fn z_index(self) -> u32 {
        match self {
            Self::Flat => 0,
            Self::Raised => 10,
            Self::Overlay => 20,
            Self::Floating => 30,
            Self::Modal => 40,
        }
    }
}

/// Complete elevation definition including shadow and border.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Elevation {
    /// Primary shadow
    pub shadow: Shadow,
    /// Optional secondary shadow for more depth
    pub shadow_secondary: Option<Shadow>,
    /// Border width (for subtle elevation without shadow)
    pub border_width: f32,
    /// Border color
    pub border_color: Color,
}

impl Elevation {
    /// Create a new elevation.
    #[must_use]
    pub const fn new(shadow: Shadow) -> Self {
        Self {
            shadow,
            shadow_secondary: None,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    /// Create an elevation with border only (no shadow).
    #[must_use]
    pub const fn with_border(border_width: f32, border_color: Color) -> Self {
        Self {
            shadow: Shadow::NONE,
            shadow_secondary: None,
            border_width,
            border_color,
        }
    }

    /// Flat elevation (no visual elevation).
    pub const FLAT: Self = Self::new(Shadow::NONE);
}

impl Default for Elevation {
    fn default() -> Self {
        Self::FLAT
    }
}

/// Elevation scale with 5 predefined levels.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElevationScale {
    /// Flat - no elevation
    pub flat: Elevation,
    /// Raised - slight elevation
    pub raised: Elevation,
    /// Overlay - medium elevation
    pub overlay: Elevation,
    /// Floating - high elevation
    pub floating: Elevation,
    /// Modal - highest elevation
    pub modal: Elevation,
}

impl ElevationScale {
    /// Get elevation by level.
    #[must_use]
    pub fn get(&self, level: ElevationLevel) -> &Elevation {
        match level {
            ElevationLevel::Flat => &self.flat,
            ElevationLevel::Raised => &self.raised,
            ElevationLevel::Overlay => &self.overlay,
            ElevationLevel::Floating => &self.floating,
            ElevationLevel::Modal => &self.modal,
        }
    }
}

impl Default for ElevationScale {
    fn default() -> Self {
        let shadow_color = Color::new(0.0, 0.0, 0.0, 0.1);
        let shadow_color_strong = Color::new(0.0, 0.0, 0.0, 0.15);

        Self {
            flat: Elevation::FLAT,
            raised: Elevation::new(Shadow::new(0.0, 1.0, 3.0, 0.0, shadow_color)),
            overlay: Elevation::new(Shadow::new(0.0, 4.0, 6.0, -1.0, shadow_color)),
            floating: Elevation::new(Shadow::new(0.0, 10.0, 15.0, -3.0, shadow_color_strong)),
            modal: Elevation::new(Shadow::new(0.0, 25.0, 50.0, -12.0, shadow_color_strong)),
        }
    }
}
