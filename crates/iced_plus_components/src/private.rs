//! Sealed trait pattern for internal extensibility control.

/// Marker trait that cannot be implemented outside this crate.
pub trait Sealed {}
