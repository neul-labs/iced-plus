//! Spacing scale tokens.
//!
//! Provides a modular spacing scale for consistent layout across the design system.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Named spacing sizes for semantic usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u8)]
pub enum SpacingSize {
    /// Extra extra small (2px)
    Xxs = 0,
    /// Extra small (4px)
    Xs = 1,
    /// Small (8px)
    Sm = 2,
    /// Medium (12px)
    Md = 3,
    /// Large (16px)
    Lg = 4,
    /// Extra large (24px)
    Xl = 5,
    /// 2x extra large (32px)
    Xl2 = 6,
    /// 3x extra large (48px)
    Xl3 = 7,
    /// 4x extra large (64px)
    Xl4 = 8,
    /// 5x extra large (96px)
    Xl5 = 9,
}

impl SpacingSize {
    /// All spacing sizes in order from smallest to largest.
    pub const ALL: [Self; 10] = [
        Self::Xxs,
        Self::Xs,
        Self::Sm,
        Self::Md,
        Self::Lg,
        Self::Xl,
        Self::Xl2,
        Self::Xl3,
        Self::Xl4,
        Self::Xl5,
    ];

    /// Get the index for array access.
    #[must_use]
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// A modular spacing scale with 10 predefined values.
///
/// The scale follows a geometric progression for visual harmony:
/// - xxs: 2px
/// - xs:  4px
/// - sm:  8px
/// - md:  12px
/// - lg:  16px
/// - xl:  24px
/// - 2xl: 32px
/// - 3xl: 48px
/// - 4xl: 64px
/// - 5xl: 96px
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpacingScale {
    values: [f32; 10],
}

impl SpacingScale {
    /// Default spacing scale values.
    pub const DEFAULT: Self = Self {
        values: [2.0, 4.0, 8.0, 12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0],
    };

    /// Create a custom spacing scale from an array.
    #[must_use]
    pub const fn new(values: [f32; 10]) -> Self {
        Self { values }
    }

    /// Get a spacing value by size.
    #[inline(always)]
    #[must_use]
    pub const fn get(&self, size: SpacingSize) -> f32 {
        self.values[size.index()]
    }

    /// Extra extra small spacing (2px default).
    #[inline(always)]
    #[must_use]
    pub const fn xxs(&self) -> f32 {
        self.values[0]
    }

    /// Extra small spacing (4px default).
    #[inline(always)]
    #[must_use]
    pub const fn xs(&self) -> f32 {
        self.values[1]
    }

    /// Small spacing (8px default).
    #[inline(always)]
    #[must_use]
    pub const fn sm(&self) -> f32 {
        self.values[2]
    }

    /// Medium spacing (12px default).
    #[inline(always)]
    #[must_use]
    pub const fn md(&self) -> f32 {
        self.values[3]
    }

    /// Large spacing (16px default).
    #[inline(always)]
    #[must_use]
    pub const fn lg(&self) -> f32 {
        self.values[4]
    }

    /// Extra large spacing (24px default).
    #[inline(always)]
    #[must_use]
    pub const fn xl(&self) -> f32 {
        self.values[5]
    }

    /// 2x extra large spacing (32px default).
    #[inline(always)]
    #[must_use]
    pub const fn xl2(&self) -> f32 {
        self.values[6]
    }

    /// 3x extra large spacing (48px default).
    #[inline(always)]
    #[must_use]
    pub const fn xl3(&self) -> f32 {
        self.values[7]
    }

    /// 4x extra large spacing (64px default).
    #[inline(always)]
    #[must_use]
    pub const fn xl4(&self) -> f32 {
        self.values[8]
    }

    /// 5x extra large spacing (96px default).
    #[inline(always)]
    #[must_use]
    pub const fn xl5(&self) -> f32 {
        self.values[9]
    }

    /// Get the raw values array.
    #[must_use]
    pub const fn values(&self) -> &[f32; 10] {
        &self.values
    }
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_scale_values() {
        let scale = SpacingScale::DEFAULT;
        assert!((scale.xxs() - 2.0).abs() < f32::EPSILON);
        assert!((scale.xs() - 4.0).abs() < f32::EPSILON);
        assert!((scale.sm() - 8.0).abs() < f32::EPSILON);
        assert!((scale.md() - 12.0).abs() < f32::EPSILON);
        assert!((scale.lg() - 16.0).abs() < f32::EPSILON);
        assert!((scale.xl() - 24.0).abs() < f32::EPSILON);
    }

    #[test]
    fn get_by_size() {
        let scale = SpacingScale::DEFAULT;
        assert!((scale.get(SpacingSize::Lg) - 16.0).abs() < f32::EPSILON);
    }
}
