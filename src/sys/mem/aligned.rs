// devela::sys::mem::aligned
//
//! Defines [`MemAligned`].
//

#[cfg(doc)]
use crate::Mem;

/// Marker trait to prevent downstream implementations of the [`MemAligned`] trait.
trait Sealed {}
impl<Candidate, Requirement> Sealed for (Candidate, Requirement) {}

/// Marker trait to verify memory alignment between two types.
///
/// This trait ensures that a value of the `Candidate` type can be safely placed
/// into a storage medium designed for the `Requirement` type, adhering to
/// alignment requirements.
///
/// For runtime alignment adjustments, see [`Mem::align_down`] and [`Mem::align_up`].
//
// WAIT:DONE:1.79 [ptr.is_aligned](https://github.com/rust-lang/rust/pull/121948)
// WAIT: [pointer_is_aligned_to](https://github.com/rust-lang/rust/issues/96284)
// WAIT: [const_pointer_is_aligned](https://github.com/rust-lang/rust/issues/104203)
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait MemAligned: Sealed {
    /// Checks if the `Candidate`'s alignment is compatible with the `Requirement`'s alignment.
    ///
    /// Returns `true` if `Candidate` type's alignment is less than or equal to the
    /// `Requirement` type's alignment.
    ///
    /// Alignment represents the byte boundary to which a type must be aligned
    /// in memory. It is crucial to ensure that types are placed in memory
    /// locations that respect their alignment requirements.
    #[must_use]
    fn is_compatible() -> bool;

    /// Asserts that the memory alignment of the `Candidate` type meets or
    /// exceeds the `Requirement` type's alignment.
    ///
    /// # Panics
    /// Panics if the `Candidate` type's alignment is greater than the
    /// `Requirement` type's alignment, indicating an incompatibility that could
    /// lead to undefined behavior or performance penalties.
    fn assert_compatibility();
}

// #[cfg(not(feature = "nightly_const"))]
impl<Candidate, Requirement> MemAligned for (Candidate, Requirement) {
    fn is_compatible() -> bool {
        align_of::<Candidate>() <= align_of::<Requirement>()
    }

    fn assert_compatibility() {
        assert!(
            Self::is_compatible(),
            "`Candidate`'s alignment ({}) exceeds `Requirement`'s alignment ({}).",
            align_of::<Candidate>(),
            align_of::<Requirement>(),
        );
    }
}
// IMPROVE: compile-time check
// WAIT: [generic_const_exprs](https://github.com/rust-lang/rust/issues/76560)
// #[cfg(feature = "nightly_const")]
// impl<Candidate, Requirement> MemAligned for (Candidate, Requirement)
// where
//     [(); align_of::<Requirement>() - align_of::<Candidate>()]: Sized,
// {
//     fn check() {}
// }
