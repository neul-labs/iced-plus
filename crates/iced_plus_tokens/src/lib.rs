//! Design tokens for iced-plus.
//!
//! This crate provides a comprehensive set of design tokens for building
//! consistent, themeable user interfaces with the iced GUI framework.
//!
//! # Token Categories
//!
//! - **Colors**: Color scales and semantic palettes
//! - **Typography**: Text styles with font, size, weight, and line height
//! - **Spacing**: Modular spacing scale for margins, padding, and gaps
//! - **Radius**: Border radius presets
//! - **Elevation**: Shadow and depth definitions
//! - **Motion**: Animation duration and easing presets
//!
//! # Example
//!
//! ```rust
//! use iced_plus_tokens::presets;
//!
//! // Get a pre-built theme
//! let theme = presets::light();
//!
//! // Access tokens
//! let spacing = theme.tokens.spacing.lg(); // 16.0
//! let primary = theme.tokens.colors.primary.s500; // Primary blue
//! ```
//!
//! # Feature Flags
//!
//! - `preset-light` (default): Include the light theme preset
//! - `preset-dark` (default): Include the dark theme preset
//! - `serde`: Enable serialization/deserialization support

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod color;
pub mod elevation;
pub mod motion;
pub mod presets;
mod private;
pub mod radius;
pub mod spacing;
pub mod tokens;
pub mod typography;

// Re-export main types at crate root for convenience
pub use color::{Color, ColorPalette, ColorScale, SemanticColors, Shade};
pub use elevation::{Elevation, ElevationLevel, ElevationScale, Shadow};
pub use motion::{DurationPreset, Easing, Motion, MotionScale};
pub use radius::{RadiusScale, RadiusSize};
pub use spacing::{SpacingScale, SpacingSize};
pub use tokens::{ThemePreset, ThemeTokens};
pub use typography::{FontWeight, TextStyle, TextStyleName, TypographyScale};
