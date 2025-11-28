//! Sealed trait pattern for internal extensibility control.
//!
//! This module prevents external crates from implementing certain traits,
//! allowing us to evolve internal APIs without breaking changes.

/// Marker trait that cannot be implemented outside this crate.
///
/// Used to seal traits that should only have implementations defined
/// within the iced-plus ecosystem.
pub trait Sealed {}

// Implement Sealed for internal marker types
// These implementations are added in the modules that define the types
