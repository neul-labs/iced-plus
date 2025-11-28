//! Direction phantom types for type-safe layout orientation.
//!
//! These zero-sized types encode the layout direction at compile time,
//! enabling direction-aware alignment constraints.

use crate::private::Sealed;
use iced::Alignment;

/// Horizontal direction (left to right).
#[derive(Debug, Clone, Copy, Default)]
pub struct Horizontal;
impl Sealed for Horizontal {}

/// Vertical direction (top to bottom).
#[derive(Debug, Clone, Copy, Default)]
pub struct Vertical;
impl Sealed for Vertical {}

/// Trait for layout directions with associated cross-axis alignment.
///
/// This trait enables type-safe alignment constraints:
/// - `HStack` (Horizontal) uses `Vertical` alignment for cross-axis
/// - `VStack` (Vertical) uses `Horizontal` alignment for cross-axis
pub trait Direction: Sealed + Copy + Default {
    /// The alignment type for the cross axis.
    type CrossAlign: Copy + Into<Alignment>;

    /// Default cross-axis alignment.
    fn default_align() -> Self::CrossAlign;

    /// Get the main axis length from a size.
    fn main_axis(width: f32, height: f32) -> f32;

    /// Get the cross axis length from a size.
    fn cross_axis(width: f32, height: f32) -> f32;

    /// Check if this direction is horizontal.
    fn is_horizontal() -> bool;
}

impl Direction for Horizontal {
    type CrossAlign = iced::alignment::Vertical;

    fn default_align() -> Self::CrossAlign {
        iced::alignment::Vertical::Center
    }

    fn main_axis(width: f32, _height: f32) -> f32 {
        width
    }

    fn cross_axis(_width: f32, height: f32) -> f32 {
        height
    }

    fn is_horizontal() -> bool {
        true
    }
}

impl Direction for Vertical {
    type CrossAlign = iced::alignment::Horizontal;

    fn default_align() -> Self::CrossAlign {
        iced::alignment::Horizontal::Center
    }

    fn main_axis(_width: f32, height: f32) -> f32 {
        height
    }

    fn cross_axis(width: f32, _height: f32) -> f32 {
        width
    }

    fn is_horizontal() -> bool {
        false
    }
}
