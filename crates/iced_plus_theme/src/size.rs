//! Size phantom types for type-safe component sizing.

use crate::private::Sealed;

/// Extra small size.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraSmall;
impl Sealed for ExtraSmall {}

/// Small size.
#[derive(Debug, Clone, Copy, Default)]
pub struct Small;
impl Sealed for Small {}

/// Medium size (default).
#[derive(Debug, Clone, Copy, Default)]
pub struct Medium;
impl Sealed for Medium {}

/// Large size.
#[derive(Debug, Clone, Copy, Default)]
pub struct Large;
impl Sealed for Large {}

/// Extra large size.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraLarge;
impl Sealed for ExtraLarge {}

/// Trait for component sizes with compile-time resolution.
pub trait ComponentSize: Sealed + Copy + Default {
    /// Human-readable name of this size.
    const NAME: &'static str;

    /// Height in pixels for this size.
    const HEIGHT: f32;

    /// Horizontal padding in pixels.
    const PADDING_H: f32;

    /// Vertical padding in pixels.
    const PADDING_V: f32;

    /// Font size in pixels.
    const FONT_SIZE: f32;
}

impl ComponentSize for ExtraSmall {
    const NAME: &'static str = "xs";
    const HEIGHT: f32 = 24.0;
    const PADDING_H: f32 = 8.0;
    const PADDING_V: f32 = 4.0;
    const FONT_SIZE: f32 = 12.0;
}

impl ComponentSize for Small {
    const NAME: &'static str = "sm";
    const HEIGHT: f32 = 32.0;
    const PADDING_H: f32 = 12.0;
    const PADDING_V: f32 = 6.0;
    const FONT_SIZE: f32 = 14.0;
}

impl ComponentSize for Medium {
    const NAME: &'static str = "md";
    const HEIGHT: f32 = 40.0;
    const PADDING_H: f32 = 16.0;
    const PADDING_V: f32 = 8.0;
    const FONT_SIZE: f32 = 14.0;
}

impl ComponentSize for Large {
    const NAME: &'static str = "lg";
    const HEIGHT: f32 = 48.0;
    const PADDING_H: f32 = 20.0;
    const PADDING_V: f32 = 10.0;
    const FONT_SIZE: f32 = 16.0;
}

impl ComponentSize for ExtraLarge {
    const NAME: &'static str = "xl";
    const HEIGHT: f32 = 56.0;
    const PADDING_H: f32 = 24.0;
    const PADDING_V: f32 = 12.0;
    const FONT_SIZE: f32 = 18.0;
}
