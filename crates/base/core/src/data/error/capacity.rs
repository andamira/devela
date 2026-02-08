// devela_base_core::data::error::capacity
//
//! Defines [`MismatchedCapacity`].
//

use crate::{_TAG_DATA, Boundary1d, define_error};

define_error! { individual:
    /// Represents an absolute mismatch: the operation cannot succeed
    /// for the given value, regardless of the current state or any retry.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::{Boundary1d, MismatchedCapacity};
    /// // Upper-bound violation. Capacity is fixed at 8, value tried was 12:
    /// let err = MismatchedCapacity::too_large(12, 8);
    /// assert_eq!(err.bound, Boundary1d::Upper);
    /// assert_eq!(err.excess(), Some(4));
    /// assert_eq!(err.missing(), None);
    ///
    /// // Lower-bound violation. At least 16 bytes required, only 10 provided:
    /// let err = MismatchedCapacity::too_small(10, 16);
    /// assert_eq!(err.bound, Boundary1d::Lower);
    /// assert_eq!(err.missing(), Some(6));
    /// assert_eq!(err.excess(), None);
    ///
    /// // Direction known, quantities unknown:
    /// let err = MismatchedCapacity::none(Boundary1d::Upper);
    /// assert_eq!(err.excess(), None);
    /// assert_eq!(err.missing(), None);
    /// ```
    pub struct MismatchedCapacity {
        /// Which boundary of the capacity constraint applies.
        pub bound: Boundary1d,
        /// The value involved in the capacity check, if known.
        pub value: Option<usize>,
        /// The capacity limit involved in the check, if known.
        pub limit: Option<usize>,
    }
    +location: "data/error",
    +tag: _TAG_DATA!(),
    DOC_MISMATCHED_CAPACITY = "The operation did not satisfy a finite capacity constraint.",
    self+f => match (self.bound, self.value, self.limit) {
        (Boundary1d::Upper, Some(v), Some(l)) =>
            write!(f, "The value {v} exceeded the available capacity {l}."),
        (Boundary1d::Upper, Some(v), None) =>
            write!(f, "The value {v} exceeded an available finite capacity."),
        (Boundary1d::Upper, None, Some(l)) =>
            write!(f, "The operation exceeded the available capacity {l}."),

        (Boundary1d::Lower, Some(v), Some(l)) =>
            write!(f, "The value {v} was insufficient for the required capacity {l}."),
        (Boundary1d::Lower, Some(v), None) =>
            write!(f, "The value {v} was insufficient for the required capacity."),
        (Boundary1d::Lower, None, Some(l)) =>
            write!(f, "The operation did not meet the required capacity {l}."),
        _ => f.write_str(DOC_MISMATCHED_CAPACITY!())
    }
}
/// Constructors
#[rustfmt::skip]
impl MismatchedCapacity {
    /// Creates a capacity mismatch with no additional information.
    pub const fn none(bound: Boundary1d) -> Self {
        Self { bound, value: None, limit: None }
    }
    /// Creates a mismatch with a known value.
    pub const fn value(bound: Boundary1d, value: usize) -> Self {
        Self { bound, value: Some(value), limit: None }
    }
    /// Creates a mismatch with a known limit.
    pub const fn limit(bound: Boundary1d, limit: usize) -> Self {
        Self { bound, value: None, limit: Some(limit) }
    }
    /// Creates a mismatch with both value and limit known.
    pub const fn value_limit(bound: Boundary1d, value: usize, limit: usize) -> Self {
        Self { bound, value: Some(value), limit: Some(limit) }
    }
    /// Creates a capacity mismatch indicating that a value was
    /// **below** the required capacity.
    ///
    /// This corresponds to a violation of the **lower** boundary.
    pub const fn too_small(value: usize, limit: usize) -> Self {
        Self::value_limit(Boundary1d::Lower, value, limit)
    }
    /// Creates a capacity mismatch indicating that a value
    /// **exceeded** an available capacity.
    ///
    /// This corresponds to a violation of the **upper** boundary.
    pub const fn too_large(value: usize, limit: usize) -> Self {
        Self::value_limit(Boundary1d::Upper, value, limit)
    }
    /// Creates an upper-bound capacity mismatch from a fallible value computation.
    ///
    /// If the value is available, it is recorded.
    /// Otherwise, the mismatch is created without a concrete value.
    pub const fn too_large_try<E: Copy>(value: Result<usize, E>, limit: usize) -> Self {
        match value {
            Ok(v) => Self::too_large(v, limit),
            Err(_) => Self::limit(Boundary1d::Upper, limit),
        }
    }
}
/// Queries
impl MismatchedCapacity {
    /// Returns the amount by which the value exceeded the limit,
    /// if this is an upper-bound mismatch and both quantities are known.
    pub const fn excess(&self) -> Option<usize> {
        match (self.bound, self.value, self.limit) {
            (Boundary1d::Upper, Some(v), Some(l)) if v > l => Some(v - l),
            _ => None,
        }
    }
    /// Returns the amount by which the value fell short of the limit,
    /// if this is a lower-bound mismatch and both quantities are known.
    pub const fn missing(&self) -> Option<usize> {
        match (self.bound, self.value, self.limit) {
            (Boundary1d::Lower, Some(v), Some(l)) if v < l => Some(l - v),
            _ => None,
        }
    }
}
