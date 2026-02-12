//! Motion and animation timing tokens.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Named duration presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DurationPreset {
    /// Instant (0ms) - no animation
    Instant,
    /// Fast (100ms) - micro-interactions
    Fast,
    /// Normal (200ms) - standard transitions
    Normal,
    /// Slow (300ms) - emphasis transitions
    Slow,
    /// Slower (500ms) - dramatic transitions
    Slower,
}

impl DurationPreset {
    /// Get the duration in milliseconds.
    #[must_use]
    pub const fn ms(self) -> u32 {
        match self {
            Self::Instant => 0,
            Self::Fast => 100,
            Self::Normal => 200,
            Self::Slow => 300,
            Self::Slower => 500,
        }
    }

    /// Get the duration in seconds.
    #[must_use]
    pub const fn seconds(self) -> f32 {
        self.ms() as f32 / 1000.0
    }
}

/// Easing function type.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Easing {
    /// Linear easing
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in-out (slow start and end)
    EaseInOut,
    /// Custom cubic bezier
    CubicBezier(f32, f32, f32, f32),
}

impl Easing {
    /// Standard ease for UI transitions.
    pub const STANDARD: Self = Self::CubicBezier(0.4, 0.0, 0.2, 1.0);

    /// Decelerate ease for entering elements.
    pub const DECELERATE: Self = Self::CubicBezier(0.0, 0.0, 0.2, 1.0);

    /// Accelerate ease for exiting elements.
    pub const ACCELERATE: Self = Self::CubicBezier(0.4, 0.0, 1.0, 1.0);

    /// Sharp ease for elements that need to feel snappy.
    pub const SHARP: Self = Self::CubicBezier(0.4, 0.0, 0.6, 1.0);
}

impl Default for Easing {
    fn default() -> Self {
        Self::STANDARD
    }
}

/// A complete motion definition combining duration and easing.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Motion {
    /// Duration in milliseconds
    pub duration_ms: u32,
    /// Easing function
    pub easing: Easing,
}

impl Motion {
    /// Create a new motion definition.
    #[must_use]
    pub const fn new(duration_ms: u32, easing: Easing) -> Self {
        Self {
            duration_ms,
            easing,
        }
    }

    /// Create motion from a duration preset.
    #[must_use]
    pub const fn from_preset(preset: DurationPreset, easing: Easing) -> Self {
        Self::new(preset.ms(), easing)
    }

    /// Get the duration in seconds.
    #[must_use]
    pub const fn duration_seconds(&self) -> f32 {
        self.duration_ms as f32 / 1000.0
    }

    /// No motion (instant).
    pub const NONE: Self = Self::new(0, Easing::Linear);
}

impl Default for Motion {
    fn default() -> Self {
        Self::new(200, Easing::STANDARD)
    }
}

/// Motion scale with predefined animation presets.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MotionScale {
    /// Instant feedback (0ms)
    pub instant: Motion,
    /// Fast micro-interactions (100ms)
    pub fast: Motion,
    /// Standard transitions (200ms)
    pub normal: Motion,
    /// Emphasis transitions (300ms)
    pub slow: Motion,
    /// Dramatic transitions (500ms)
    pub slower: Motion,
}

impl MotionScale {
    /// Get motion by duration preset.
    #[must_use]
    pub const fn get(&self, preset: DurationPreset) -> Motion {
        match preset {
            DurationPreset::Instant => self.instant,
            DurationPreset::Fast => self.fast,
            DurationPreset::Normal => self.normal,
            DurationPreset::Slow => self.slow,
            DurationPreset::Slower => self.slower,
        }
    }
}

impl Default for MotionScale {
    fn default() -> Self {
        Self {
            instant: Motion::NONE,
            fast: Motion::new(100, Easing::SHARP),
            normal: Motion::new(200, Easing::STANDARD),
            slow: Motion::new(300, Easing::STANDARD),
            slower: Motion::new(500, Easing::DECELERATE),
        }
    }
}
