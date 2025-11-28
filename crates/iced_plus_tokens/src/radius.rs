//! Border radius tokens.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Named radius sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum RadiusSize {
    /// No radius (0px)
    None = 0,
    /// Extra small (2px)
    Xs = 1,
    /// Small (4px)
    Sm = 2,
    /// Medium (6px)
    Md = 3,
    /// Large (8px)
    Lg = 4,
    /// Extra large (12px)
    Xl = 5,
    /// 2x extra large (16px)
    Xl2 = 6,
    /// Full/pill (9999px)
    Full = 7,
}

impl RadiusSize {
    /// Get the index for array access.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// Border radius scale with 8 predefined values.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RadiusScale {
    values: [f32; 8],
}

impl RadiusScale {
    /// Default radius scale values.
    pub const DEFAULT: Self = Self {
        values: [0.0, 2.0, 4.0, 6.0, 8.0, 12.0, 16.0, 9999.0],
    };

    /// Create a custom radius scale.
    #[must_use]
    pub const fn new(values: [f32; 8]) -> Self {
        Self { values }
    }

    /// Get a radius value by size.
    #[inline(always)]
    #[must_use]
    pub const fn get(&self, size: RadiusSize) -> f32 {
        self.values[size.index()]
    }

    /// No radius.
    #[inline(always)]
    #[must_use]
    pub const fn none(&self) -> f32 {
        self.values[0]
    }

    /// Extra small radius.
    #[inline(always)]
    #[must_use]
    pub const fn xs(&self) -> f32 {
        self.values[1]
    }

    /// Small radius.
    #[inline(always)]
    #[must_use]
    pub const fn sm(&self) -> f32 {
        self.values[2]
    }

    /// Medium radius.
    #[inline(always)]
    #[must_use]
    pub const fn md(&self) -> f32 {
        self.values[3]
    }

    /// Large radius.
    #[inline(always)]
    #[must_use]
    pub const fn lg(&self) -> f32 {
        self.values[4]
    }

    /// Extra large radius.
    #[inline(always)]
    #[must_use]
    pub const fn xl(&self) -> f32 {
        self.values[5]
    }

    /// 2x extra large radius.
    #[inline(always)]
    #[must_use]
    pub const fn xl2(&self) -> f32 {
        self.values[6]
    }

    /// Full/pill radius.
    #[inline(always)]
    #[must_use]
    pub const fn full(&self) -> f32 {
        self.values[7]
    }
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self::DEFAULT
    }
}
