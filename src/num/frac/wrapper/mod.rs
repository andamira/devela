// devela::num::frac::wrapper
//
//! Fraction-related wrapper struct.
//
// TOC
// - define Frac struct
// - implement core traits

#[cfg(doc)]
use crate::Int;

mod impl_frac;

#[doc = crate::TAG_NUM!()]
#[doc = crate::TAG_NAMESPACE!()]
/// Provides comprehensive fractional operations on `T`, most of them *const*.
///
/// It's implemented for:
/// - arrays: `[i8; 2]`… `[u128; 2]`; `[Int<i8>; 2]`… `[Int<u128>; 2]`.
///
/// The documentation is the same for all bit sizes. For example these are:
/// - Methods for `[i32; 2]`:
/// [core][Self#fraction-related-methods-for-i32-2];
/// for `[Int<i32>; 2]`:
/// [core][Self#fraction-related-methods-for-inti32-2].
#[repr(transparent)]
pub struct Frac<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use {super::Frac, core::{fmt, hash}};

    impl<T: Clone> Clone for Frac<T> {
        #[must_use]
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Frac<T> {}
    impl<T: fmt::Debug> fmt::Debug for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Frac").field(&self.0).finish()
        }
    }
    impl<T: fmt::Display> fmt::Display for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }
    impl<T: fmt::Binary> fmt::Binary for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Binary::fmt(&self.0, f) }
    }
    impl<T: fmt::Octal> fmt::Octal for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Octal::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerHex> fmt::LowerHex for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperHex> fmt::UpperHex for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperExp> fmt::UpperExp for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperExp::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerExp> fmt::LowerExp for Frac<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerExp::fmt(&self.0, f) }
    }
    impl<T: hash::Hash> hash::Hash for Frac<T> {
        fn hash<H: hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
    }
    impl<T: hash::Hasher> hash::Hasher for Frac<T> {
        #[must_use]
        fn finish(&self) -> u64 { self.0.finish() }
        fn write(&mut self, bytes: &[u8]) { self.0.write(bytes) }
    }
}
