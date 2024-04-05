// devela::num::int::wrapper
//
//! Integer wrapper struct.
//
// TOC
// - define Int struct
// - implement core traits

mod impl_base;
mod impl_combinatorics;
mod impl_core;
mod impl_div;
mod impl_factors;
mod impl_modulo;
mod impl_prime;
mod impl_root;

/// Provides comprehensive integer operations on `T`, most of them *const*.
///
/// It's implemented for:
/// - all the integer primitives: `i8`, …, `i128`, `u8`, …, `u128`.
///
/// Specific implementations can vary between signed and signed numeric types,
/// but documentation is the same for all bit sizes:
/// - `i32` methods documentation related to:
/// [base][Self#integer-base-related-methods-for-i32],
/// [core][Self#integer-core-methods-for-i32],
/// [combinatorics][Self#integer-combinatorics-related-methods-for-i32],
/// [division][Self#integer-division-related-methods-for-i32],
/// [factors][Self#integer-factors-related-methods-for-i32],
/// [modulo][Self#integer-modulo-related-methods-for-i32],
/// [primes][Self#integer-prime-related-methods-for-i32],
/// [root][Self#integer-root-related-methods-for-i32].
/// - `u32` methods documentation related to:
/// [base][Self#integer-base-related-methods-for-u32],
/// [core][Self#integer-core-methods-for-u32],
/// [combinatorics][Self#integer-combinatorics-related-methods-for-u32],
/// [division][Self#integer-division-related-methods-for-u32],
/// [factors][Self#integer-factors-related-methods-for-u32],
/// [modulo][Self#integer-modulo-related-methods-for-u32],
/// [primes][Self#integer-prime-related-methods-for-u32],
/// [root][Self#integer-root-related-methods-for-u32].
///
/// See also the related trait [`NumInt`][super::NumInt].
#[repr(transparent)]
pub struct Int<T>(pub T);

crate::num::impl_ops![Int: i8:"_int_i8", i16:"_int_i16", i32:"_int_i32",
    i64:"_int_i64", i128:"_int_i128", isize:"_int_isize"];
crate::num::impl_ops![Int: (no_neg) u8:"_int_i8", u16:"_int_u16", u32:"_int_u32",
    u64:"_int_u64", u128:"_int_u128", usize:"_int_usize"];

#[rustfmt::skip]
mod core_impls {
    use super::Int;
    use core::{fmt, cmp, hash};
    use crate::code::ValueQuant;

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

    /* eq */

    impl<T: PartialEq> PartialEq for Int<T> {
        #[inline] #[must_use]
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Int<T> {}
    // with the inner value:
    impl<T: PartialEq> PartialEq<T> for Int<T> {
        #[inline] #[must_use]
        fn eq(&self, other: &T) -> bool { self.0.eq(other) }
    }
    // with ValueQuant:
    impl<T: PartialEq> PartialEq<ValueQuant<T, T>> for ValueQuant<Int<T>, Int<T>> {
        #[inline] #[must_use]
        fn eq(&self, other: &ValueQuant<T, T>) -> bool {
            self.v.eq(&other.v) && self.q.eq(&other.q)
        }
    }
    impl<T: PartialEq> PartialEq<ValueQuant<Int<T>, Int<T>>> for ValueQuant<T, T> {
        #[inline] #[must_use]
        fn eq(&self, other: &ValueQuant<Int<T>, Int<T>>) -> bool {
            self.v.eq(&other.v.0) && self.q.eq(&other.q.0)
        }
    }
    // with ValueQuant and tuple:
    impl<T: PartialEq> PartialEq<(T, T)> for ValueQuant<Int<T>, Int<T>> {
        #[inline] #[must_use]
        fn eq(&self, other: &(T, T)) -> bool {
            self.v.eq(&other.0) && self.q.eq(&other.1)
        }
    }
    impl<T: PartialEq> PartialEq<(Int<T>, Int<T>)> for ValueQuant<T, T> {
        #[inline] #[must_use]
        fn eq(&self, other: &(Int<T>, Int<T>)) -> bool {
            self.v.eq(&other.0.0) && self.q.eq(&other.1.0)
        }
    }

    /* ord*/

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
    // with the inner value:
    impl<T: PartialOrd> PartialOrd<T> for Int<T> {
        #[inline] #[must_use]
        fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
            self.0.partial_cmp(other)
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
