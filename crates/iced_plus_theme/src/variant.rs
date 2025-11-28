//! Variant phantom types for type-safe styling.
//!
//! These zero-sized types encode component variants at compile time,
//! enabling monomorphization for optimal performance.

use crate::private::Sealed;

/// Primary variant - main call-to-action.
#[derive(Debug, Clone, Copy, Default)]
pub struct Primary;
impl Sealed for Primary {}

/// Secondary variant - alternative actions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Secondary;
impl Sealed for Secondary {}

/// Ghost variant - subtle, minimal styling.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ghost;
impl Sealed for Ghost {}

/// Destructive variant - dangerous/irreversible actions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Destructive;
impl Sealed for Destructive {}

/// Outline variant - bordered with transparent background.
#[derive(Debug, Clone, Copy, Default)]
pub struct Outline;
impl Sealed for Outline {}

/// Trait for button variants with compile-time style resolution.
pub trait ButtonVariant: Sealed + Copy + Default {
    /// Human-readable name of this variant.
    const NAME: &'static str;
}

impl ButtonVariant for Primary {
    const NAME: &'static str = "primary";
}

impl ButtonVariant for Secondary {
    const NAME: &'static str = "secondary";
}

impl ButtonVariant for Ghost {
    const NAME: &'static str = "ghost";
}

impl ButtonVariant for Destructive {
    const NAME: &'static str = "destructive";
}

impl ButtonVariant for Outline {
    const NAME: &'static str = "outline";
}

/// Trait for input variants.
pub trait InputVariant: Sealed + Copy + Default {
    /// Human-readable name of this variant.
    const NAME: &'static str;
}

/// Default input variant.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultInput;
impl Sealed for DefaultInput {}
impl InputVariant for DefaultInput {
    const NAME: &'static str = "default";
}

/// Filled input variant.
#[derive(Debug, Clone, Copy, Default)]
pub struct FilledInput;
impl Sealed for FilledInput {}
impl InputVariant for FilledInput {
    const NAME: &'static str = "filled";
}
