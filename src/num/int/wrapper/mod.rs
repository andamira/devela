// devela::num::int::wrapper
//
//! Integer wrapper struct.
//
// TOC
// - define Int struct
// - implement core traits

mod impl_base;
mod impl_core;
mod impl_count;
mod impl_div;
mod impl_factors;
mod impl_prime;
mod impl_sqrt;

/// Provides integer operations on `T`, most of them *const*.
///
/// It's implemented for:
/// `i8`, …, `i128`, `u8`, …, `u128`.
///
/// The documentation is the same for all bit sizes. For example these are:
/// - Methods for `i32` related to:
/// [base][Self#integer-base-related-methods-for-i32],
/// [core][Self#integer-core-methods-for-i32],
/// [counting][Self#integer-counting-related-methods-for-i32],
/// [division][Self#integer-division-related-methods-for-i32],
/// [factors][Self#integer-factors-related-methods-for-i32],
/// [primes][Self#integer-prime-related-methods-for-i32],
/// [square root][Self#integer-square-root-related-methods-for-i32].
/// - Methods for `u32` related to:
/// [base][Self#integer-base-related-methods-for-u32],
/// [core][Self#integer-core-methods-for-u32],
/// [counting][Self#integer-counting-related-methods-for-u32],
/// [division][Self#integer-division-related-methods-for-u32],
/// [factors][Self#integer-factors-related-methods-for-u32],
/// [primes][Self#integer-prime-related-methods-for-u32],
/// [square root][Self#integer-square-root-related-methods-for-u32].
///
/// See also the related trait [`NumInt`][super::NumInt].
#[repr(transparent)]
pub struct Int<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use {super::Int, core::{fmt, cmp, hash}};

    impl<T: Clone> Clone for Int<T> {
        #[inline] #[must_use]
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Int<T> {}
    impl<T: fmt::Debug> fmt::Debug for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Int").field(&self.0).finish()
        }
    }
    impl<T: fmt::Display> fmt::Display for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }
    impl<T: fmt::Binary> fmt::Binary for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Binary::fmt(&self.0, f) }
    }
    impl<T: fmt::Octal> fmt::Octal for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Octal::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerHex> fmt::LowerHex for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperHex> fmt::UpperHex for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperHex::fmt(&self.0, f) }
    }
    impl<T: fmt::UpperExp> fmt::UpperExp for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::UpperExp::fmt(&self.0, f) }
    }
    impl<T: fmt::LowerExp> fmt::LowerExp for Int<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::LowerExp::fmt(&self.0, f) }
    }

    impl<T: PartialEq> PartialEq for Int<T> {
        #[inline] #[must_use]
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Int<T> {}
    impl<T: PartialOrd> PartialOrd for Int<T> {
        #[inline] #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Int<T> {
        #[inline] #[must_use]
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl<T: hash::Hash> hash::Hash for Int<T> {
        #[inline]
        fn hash<H: hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl<T: hash::Hasher> hash::Hasher for Int<T> {
        #[inline] #[must_use]
        fn finish(&self) -> u64 {
            self.0.finish()
        }
        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            self.0.write(bytes)
        }
    }
}
