// devela::num::dom::frac::wrapper
//
//! Fraction-related wrapper struct.
//
// TOC
// - define Frac struct
// - implement core traits

#[cfg(doc)]
use crate::Int;

mod impl_frac;

#[doc = crate::_tags!(num namespace)]
/// Provides comprehensive fractional operations on `T`, most of them *const*.
#[doc = crate::_doc_location!("num/dom")]
///
/// It's implemented for:
/// - arrays: `[i8; 2]`… `[u128; 2]`; `[Int<i8>; 2]`… `[Int<u128>; 2]`.
///
/// The documentation is the same for all bit sizes. For example these are:
/// - Methods for `[i32; 2]`:
/// [core][Self#fraction-related-methods-for-i32-2];
/// for `[Int<i32>; 2]`:
/// [core][Self#fraction-related-methods-for-inti32-2].
#[must_use]
#[repr(transparent)]
pub struct Frac<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use crate::{impl_trait, Frac};

    impl<T: Clone> Clone for Frac<T> {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Frac<T> {}

    impl_trait![fmt::Debug for Frac[T][T] where T |self, f|
        f.debug_tuple("Frac").field(&self.0).finish()
    ];
    impl_trait![fmt::Display for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Binary for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Octal for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerHex for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperHex for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerExp for Frac[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperExp for Frac[T][T] where T |self, f| self.0.fmt(f)];

    impl_trait![Hash for Frac[T][T] where T |self, s| self.0.hash(s)];
}
