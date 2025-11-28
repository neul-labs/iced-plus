//! Theme bridge for iced-plus.
//!
//! This crate adapts design tokens from `iced_plus_tokens` to iced's styling
//! system, providing type-safe theming with compile-time variant resolution.
//!
//! # Key Types
//!
//! - [`AppTheme`] - The main theme type that wraps tokens and implements iced's Catalog traits
//! - [`ButtonClass`] - Button styling variants (Primary, Secondary, Ghost, etc.)
//! - [`ContainerClass`] - Container styling variants (Card, Surface, Bordered)
//! - [`TextInputClass`] - Text input styling variants (Default, Filled)
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_theme::{AppTheme, ButtonClass};
//! use iced::widget::button;
//!
//! let theme = AppTheme::light();
//!
//! // Use in iced application
//! button("Click me")
//!     .class(ButtonClass::Primary)
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod catalog;
mod private;
pub mod size;
pub mod status;
pub mod theme;
pub mod variant;

// Re-export main types
pub use catalog::*;
pub use size::{ComponentSize, ExtraLarge, ExtraSmall, Large, Medium, Small};
pub use status::Status;
pub use theme::{token_to_iced, AppTheme};
pub use variant::{ButtonVariant, Destructive, Ghost, Outline, Primary, Secondary};
