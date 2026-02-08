// devela_base_core::num::dom::int::int::wrapper::namespace
//
//! Defines the [`Int`] namespace wrapper.
//

#[doc = crate::_tags!(num namespace)]
/// Provides comprehensive integer operations on `T`, all of them *const*.
#[doc = crate::_doc_location!("num/dom")]
///
/// It's implemented for:
/// - all the integer primitives: `i8`, …, `i128`, `u8`, …, `u128`.
///
/// Specific implementations can vary between signed and signed numeric types,
/// but documentation is the same for all bit sizes:
/// - `i64` methods documentation related to:
/// [base][Self#integer-base-related-methods-for-i64],
/// [core][Self#integer-core-methods-for-i64],
/// [combinatorics][Self#integer-combinatorics-related-methods-for-i64],
/// [division][Self#integer-division-related-methods-for-i64],
/// [factors][Self#integer-factors-related-methods-for-i64],
/// [modulo][Self#integer-modulo-related-methods-for-i64],
/// [primes][Self#integer-prime-related-methods-for-i64],
/// [root][Self#integer-root-related-methods-for-i64].
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
/// See also the related trait [`NumInt`].
#[doc = crate::doclink!(custom devela "[`NumInt`]" "num/trait.NumInt.html")]
#[must_use]
#[repr(transparent)]
pub struct Int<T>(pub T);

crate::impl_ops![Int: i8, i16, i32, i64, i128, isize];
crate::impl_ops![Int: (no_neg) u8, u16, u32, u64, u128, usize];
#[rustfmt::skip]
mod core_impls {
    use crate::{impl_trait, Int, Ordering, ValueQuant};

    impl<T: Clone> Clone for Int<T> {
        fn clone(&self) -> Self { Self(self.0.clone()) }
    }
    impl<T: Copy> Copy for Int<T> {}

    impl_trait![fmt::Debug for Int[T][T] where T |self, f|
        f.debug_tuple("Int").field(&self.0).finish()
    ];
    impl_trait![fmt::Display for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Binary for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::Octal for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerHex for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperHex for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::LowerExp for Int[T][T] where T |self, f| self.0.fmt(f)];
    impl_trait![fmt::UpperExp for Int[T][T] where T |self, f| self.0.fmt(f)];

    /* eq */

    impl<T: PartialEq> PartialEq for Int<T> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Int<T> {}
    // with the inner value:
    impl<T: PartialEq> PartialEq<T> for Int<T> {
        fn eq(&self, other: &T) -> bool { self.0.eq(other) }
    }

    /* ord*/

    impl<T: PartialOrd> PartialOrd for Int<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }
    impl<T: Ord> Ord for Int<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }
    // with the inner value:
    impl<T: PartialOrd> PartialOrd<T> for Int<T> {
        fn partial_cmp(&self, other: &T) -> Option<Ordering> {
            self.0.partial_cmp(other)
        }
    }

    impl_trait![Hash for Int[T][T] where T |self, s| self.0.hash(s)];

    // with ValueQuant:
    impl<T: PartialEq> PartialEq<ValueQuant<T, T>> for ValueQuant<Int<T>, Int<T>> {
        fn eq(&self, other: &ValueQuant<T, T>) -> bool {
            self.v.eq(&other.v) && self.q.eq(&other.q)
        }
    }
    impl<T: PartialEq> PartialEq<ValueQuant<Int<T>, Int<T>>> for ValueQuant<T, T> {
        fn eq(&self, other: &ValueQuant<Int<T>, Int<T>>) -> bool {
            self.v.eq(&other.v.0) && self.q.eq(&other.q.0)
        }
    }
    // with ValueQuant and tuple:
    impl<T: PartialEq> PartialEq<(T, T)> for ValueQuant<Int<T>, Int<T>> {
        fn eq(&self, other: &(T, T)) -> bool {
            self.v.eq(&other.0) && self.q.eq(&other.1)
        }
    }
    impl<T: PartialEq> PartialEq<(Int<T>, Int<T>)> for ValueQuant<T, T> {
        fn eq(&self, other: &(Int<T>, Int<T>)) -> bool {
            self.v.eq(&other.0.0) && self.q.eq(&other.1.0)
        }
    }
}
