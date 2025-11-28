//! Layout primitives for iced-plus.
//!
//! This crate provides type-safe layout components with direction-aware
//! alignment constraints and compile-time guarantees.
//!
//! # Key Types
//!
//! - [`HStack`] - Horizontal stack layout (left to right)
//! - [`VStack`] - Vertical stack layout (top to bottom)
//! - [`AppShell`] - Application shell with sidebar and header areas
//! - [`Modal`] - Modal overlay with backdrop
//! - [`Breakpoints`] - Responsive breakpoint definitions
//! - [`ShowOn`] - Show content only at certain breakpoints
//!
//! # Type Safety
//!
//! The direction is encoded in the type system:
//! - `HStack` only accepts `Vertical` cross-axis alignment
//! - `VStack` only accepts `Horizontal` cross-axis alignment
//!
//! This prevents mistakes like trying to set horizontal alignment on an HStack.
//!
//! # Responsive Layouts
//!
//! Use breakpoint helpers to create adaptive UIs:
//!
//! ```rust,ignore
//! use iced_plus_layouts::{ShowOn, BreakpointTier};
//!
//! // Only show on medium screens and up
//! ShowOn::new(sidebar).min(BreakpointTier::MD)
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_layouts::{HStack, VStack};
//! use iced::alignment;
//!
//! let layout = VStack::new()
//!     .spacing(16.0)
//!     .align(alignment::Horizontal::Center)  // Type-safe!
//!     .push(text("Hello"))
//!     .push(text("World"));
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod direction;
mod private;
#[cfg(feature = "stacks")]
mod stack;
#[cfg(feature = "shell")]
mod shell;
#[cfg(feature = "overlay")]
mod overlay;
#[cfg(feature = "responsive")]
mod responsive;

pub use direction::{Direction, Horizontal, Vertical};
#[cfg(feature = "stacks")]
pub use stack::{HStack, Stack, VStack};
#[cfg(feature = "shell")]
pub use shell::AppShell;
#[cfg(feature = "overlay")]
pub use overlay::{drawer, drawer_left, drawer_right, modal, modal_with_opacity, Modal};
#[cfg(feature = "responsive")]
pub use responsive::{
    hide_on, responsive_row, show_on, BreakpointTier, Breakpoints, Responsive, ResponsiveRow,
    ShowOn,
};
