//! Aggregate theme tokens structure.

use crate::{
    color::ColorPalette,
    elevation::ElevationScale,
    motion::MotionScale,
    radius::RadiusScale,
    spacing::SpacingScale,
    typography::TypographyScale,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Complete set of design tokens for a theme.
///
/// This is the primary structure that contains all token categories
/// needed to style an application consistently.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThemeTokens {
    /// Color palette including primary, secondary, neutral, and semantic colors
    pub colors: ColorPalette,
    /// Typography scale with all text styles
    pub typography: TypographyScale,
    /// Spacing scale for margins, padding, and gaps
    pub spacing: SpacingScale,
    /// Border radius scale
    pub radius: RadiusScale,
    /// Elevation/shadow scale
    pub elevation: ElevationScale,
    /// Motion/animation scale
    pub motion: MotionScale,
}

impl ThemeTokens {
    /// Create a new theme tokens set.
    #[must_use]
    pub fn new(
        colors: ColorPalette,
        typography: TypographyScale,
        spacing: SpacingScale,
        radius: RadiusScale,
        elevation: ElevationScale,
        motion: MotionScale,
    ) -> Self {
        Self {
            colors,
            typography,
            spacing,
            radius,
            elevation,
            motion,
        }
    }

    /// Get the color palette.
    #[must_use]
    pub fn colors(&self) -> &ColorPalette {
        &self.colors
    }

    /// Get the typography scale.
    #[must_use]
    pub fn typography(&self) -> &TypographyScale {
        &self.typography
    }

    /// Get the spacing scale.
    #[must_use]
    pub fn spacing(&self) -> &SpacingScale {
        &self.spacing
    }

    /// Get the radius scale.
    #[must_use]
    pub fn radius(&self) -> &RadiusScale {
        &self.radius
    }

    /// Get the elevation scale.
    #[must_use]
    pub fn elevation(&self) -> &ElevationScale {
        &self.elevation
    }

    /// Get the motion scale.
    #[must_use]
    pub fn motion(&self) -> &MotionScale {
        &self.motion
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
            typography: TypographyScale::default(),
            spacing: SpacingScale::default(),
            radius: RadiusScale::default(),
            elevation: ElevationScale::default(),
            motion: MotionScale::default(),
        }
    }
}

/// A named theme preset containing tokens and metadata.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThemePreset {
    /// Unique identifier for the preset
    pub id: &'static str,
    /// Human-readable name
    pub name: &'static str,
    /// The design tokens
    pub tokens: ThemeTokens,
}

impl ThemePreset {
    /// Create a new theme preset.
    #[must_use]
    pub const fn new(id: &'static str, name: &'static str, tokens: ThemeTokens) -> Self {
        Self { id, name, tokens }
    }
}
