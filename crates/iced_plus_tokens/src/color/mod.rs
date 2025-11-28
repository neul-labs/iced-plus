//! Color tokens for the design system.
//!
//! Provides type-safe color scales and semantic palettes with compile-time
//! guarantees for valid color access.

mod palette;
mod scale;

pub use palette::{ColorPalette, SemanticColors};
pub use scale::{Color, ColorScale, Shade};
