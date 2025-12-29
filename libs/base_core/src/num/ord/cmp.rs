// devela_base_core::num::ord::cmp
//
//! Helper wrapper for comparing.
//
// TOC
// - Cmp definition
// - impl core traits
// - impl Cmp for T: PartialOrd
// - impl Cmp for primitives
//   - int
//   - float
// - impl Cmp for Ordering
// - tests

use crate::Ordering::{self, Equal, Greater, Less};
#[allow(unused_imports)]
use crate::{is, paste};

#[allow(unused_imports)]
#[cfg(nightly_float)]
use ::core::{f16, f128};

#[doc = crate::_TAG_NAMESPACE!()]
/// Provides comparing methods for `T`.
///
/// It provides the non-*const* methods `pclamp`, `pmax`, `pmin`
/// for comparing [`PartialOrd`]ered values.
///
/// It provides the following *const* methods for comparing primitives:
/// `clamp`, `max`, `min`, `eq`, `ne`, `lt`, `le`, `gt`, `ge`.
///
/// And it also allows compile-time comparison between [`Ordering`]s.
///
/// In the case of floating-point primitives:
/// - total ordering is used.
/// - additional methods are provided:
///  `is_positive`, `is_negative`, `is_finite`, `is_infinite`, `is_nan`.
//
// DESIGN NOTES
// Due to Rust's inherent method resolution rules, primitive const fn comparison
// operations are provided as value methods (Cmp(x).min(y)) rather than static
// functions, as this is the only form that allows concise, monomorphized, and
// const-evaluated comparisons without traits while having full type-inference.
#[repr(transparent)]
pub struct Cmp<T>(pub T);

crate::_impl_init![ConstInitCore: <T: ConstInitCore> Self(T::INIT) => Cmp<T>];

#[rustfmt::skip]
mod core_impls {
    use {super::{Cmp, Ordering}, core::fmt};

    impl<T: Clone> Clone for Cmp<T> { fn clone(&self) -> Self { Self(self.0.clone()) } }
    impl<T: Copy> Copy for Cmp<T> {}

    impl<T: PartialEq> PartialEq for Cmp<T> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Cmp<T> {}
    impl<T: PartialOrd> PartialOrd for Cmp<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.0.partial_cmp(&other.0) }
    }
    impl<T: Ord> Ord for Cmp<T> {
        fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
    }

    impl<T: fmt::Display> fmt::Display for Cmp<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }
    impl<T: fmt::Debug> fmt::Debug for Cmp<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Cmp").field(&self.0).finish()
        }
    }
}

#[rustfmt::skip]
impl<T: PartialOrd> Cmp<T> {
    /// Compares and returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
    ///
    /// Returns `None` if comparisons are indeterminate.
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::Cmp;
    /// assert_eq![Some(0.4), Cmp(1.0).pclamp(0.2, 0.4)];
    /// assert_eq![Some(0.2), Cmp(0.0).pclamp(0.2, 0.4)];
    /// //
    /// assert_eq![None, Cmp(1.0).pclamp(f32::NAN, f32::NAN)];
    /// assert_eq![None, Cmp(1.0).pclamp(f32::NAN, 0.4)];
    /// assert_eq![None, Cmp(1.0).pclamp(0.2, f32::NAN)];
    /// ```
    #[must_use]
    pub fn pclamp(self, min: T, max: T) -> Option<T> {
        match self.0.partial_cmp(&min) {
            Some(Less) => Some(min),
            Some(Greater | Equal) => match self.0.partial_cmp(&max) {
                Some(Greater) => Some(max),
                Some(Less | Equal) => Some(self.0),
                None => None,
            },
            None => None,
        }
    }

    /// Compares and returns the maximum of two [`PartialOrd`]ered values.
    ///
    /// Returns `None` if comparisons are indeterminate.
    ///
    /// Complements `core::cmp::`[`max`][`core::cmp::max] which requires [`Ord`]
    /// # Examples
    /// ```
    /// # use devela_base_core::Cmp;
    /// assert_eq![Some(0.4), Cmp(0.2).pmax(0.4)];
    /// //
    /// assert_eq![None, Cmp(0.2).pmax(f32::NAN)];
    /// assert_eq![None, Cmp(f32::NAN).pmax(0.4)];
    /// ```
    #[must_use]
    pub fn pmax(self, other: T) -> Option<T> {
        match self.0.partial_cmp(&other) {
            Some(Less) => Some(other),
            Some(Greater | Equal) => Some(self.0),
            None => None,
        }
    }

    /// Compares and returns the minimum of two [`PartialOrd`]ered values.
    ///
    /// Returns `None` if comparisons are indeterminate.
    ///
    /// Complements `core::cmp::`[`min`][`core::cmp::min] which requires [`Ord`]
    /// # Example
    /// ```
    /// # use devela_base_core::Cmp;
    /// assert_eq![Some(0.2), Cmp(0.2).pmin(0.4)];
    /// //
    /// assert_eq![None, Cmp(0.2).pmin(f32::NAN)];
    /// assert_eq![None, Cmp(f32::NAN).pmin(0.4)];
    /// ```
    #[must_use]
    pub fn pmin(self, other: T) -> Option<T> {
        match self.0.partial_cmp(&other) {
            Some(Greater) => Some(other),
            Some(Less | Equal) => Some(self.0),
            None => None,
        }
    }
}

/// Implement [`Comparing`] for primitives.
macro_rules! impl_comparing {
    () => {
        impl_comparing![int: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
        impl_comparing![float: f32:32:31, f64:64:63];
        #[cfg(nightly_float)]
        // impl_comparing![float: f16:16:15:"_cmp_f16", f128:128:127:"_cmp_f128"];
        impl_comparing![float: f16:16:15, f128:128:127];
    };
    (
    // $p: the integer type
    // $cap: the optional capability feature associated with the `$p` type. E.g "_cmp_u8".
    int: $($p:ty $(: $cap:literal)? ),+ $(,)?) => { $( impl_comparing![@int: $p $(:$cap)? ]; )+ };
    (@int: $p:ty $(: $cap:literal)? ) => {
        $( #[cfg(feature = $cap)] )?
        impl Cmp<$p> {
            /// Compares and returns `self` clamped between `min` and `max`.
            #[must_use] #[inline(always)]
            pub const fn clamp(self, min: $p, max: $p) -> $p {
                if self.0 < min { min } else if self.0 > max { max } else { self.0 }
            }
            /// Compares and returns the maximum between `self` and `other`.
            #[must_use] #[inline(always)]
            pub const fn max(self, other: $p) -> $p { if self.0 > other { self.0 } else { other } }
            /// Compares and returns the minimum between `self` and `other`.
            #[must_use] #[inline(always)]
            pub const fn min(self, other: $p) -> $p { if self.0 < other { self.0 } else { other } }

            /// Returns `true` if `self == other`.
            #[must_use] #[inline(always)]
            pub const fn eq(self, other: $p) -> bool { self.0 == other }
            /// Returns `true` if `self != other`.
            #[must_use] #[inline(always)]
            pub const fn ne(self, other: $p) -> bool { self.0 != other }
            /// Returns `true` if `self < other`.
            #[must_use] #[inline(always)]
            pub const fn lt(self, other: $p) -> bool { self.0 <  other }
            /// Returns `true` if `self <= other`.
            #[must_use] #[inline(always)]
            pub const fn le(self, other: $p) -> bool { self.0 <= other }
            /// Returns `true` if `self > other`.
            #[must_use] #[inline(always)]
            pub const fn gt(self, other: $p) -> bool { self.0 >  other }
            /// Returns `true` if `self >= other`.
            #[must_use] #[inline(always)]
            pub const fn ge(self, other: $p) -> bool { self.0 >= other }
        }
    };
    (
    // $f:    the floating-point type
    // $fcap: the capability feature associated with the `$f` type. E.g "_cmp_f32".
    // $b:    the bits of the floating-point primitive
    // $sh:   the shift amount for the given bits ($b - 1)

    // int: $($p:ty $(: $cap:literal)? ),+) => { $( impl_comparing![@int: $p $(:$cap)? ]; )+ };
    // (@int: $p:ty $(: $cap:literal)? ) => {
    //     $( #[cfg(feature = $cap)] )?
    float: $($f:ty:$b:literal:$sh:literal $(:$fcap:literal )? ),+ $(,)?) => {
        $( impl_comparing![@float: $f:$b:$sh $(:$fcap)?]; )+
    };
    (@float: $f:ty:$b:literal:$sh:literal $( :$fcap:literal )?) => { paste! {
        $( #[cfg(feature = $fcap)] )?
        impl Cmp<$f> {
            #[doc = "A (`const`) port of `" $f "::`[`total_cmp`][prim@" $f "#method.total_cmp]."]
            #[must_use]
            pub const fn total_cmp(self, other: $f) -> Ordering {
                let mut left = self.0.to_bits() as [<i $b>];
                let mut right = other.to_bits() as [<i $b>];

                left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
                right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

                is![left < right; Less; is![left > right; Greater; Equal]]
            }

            /// Compares and returns a clamped *total ordered* `self` between `min` and `max`.
            ///
            /// # Examples
            /// ```
            #[cfg_attr(nightly_float, doc = "# #![feature(f16, f128)]")]
            /// # use devela_base_core::Cmp;
            #[doc = "assert_eq![2.0, Cmp(5.0" $f ").clamp(-1.0, 2.0)];"]
            #[doc = "assert_eq![-1.0, Cmp(-5.0" $f ").clamp(-1.0, 2.0)];"]
            /// ```
            #[must_use] #[inline(always)]
            pub const fn clamp(self, min: $f, max: $f) -> $f { self.0.clamp(min, max) }

            /// Compares and returns the *total ordered* maximum between `self` and `other`.
            ///
            /// # Examples
            /// ```
            #[cfg_attr(nightly_float, doc = "# #![feature(f16, f128)]")]
            /// # use devela_base_core::Cmp;
            #[doc = "assert_eq![2.0, Cmp(2.0" $f ").max(-1.0)];"]
            #[doc = "assert_eq![2.0, Cmp(1.0" $f ").max(2.0)];"]
            #[doc = "assert_eq![0.0, Cmp(-0.0" $f ").max(0.0)];"]
            #[doc = "assert_eq![" $f "::INFINITY, Cmp(" $f "::INFINITY).max("
                $f "::NEG_INFINITY)];"]
            /// ```
            #[must_use] #[inline(always)]
            pub const fn max(self, other: $f) -> $f { self.0.max(other) }

            /// Compares and returns the *total ordered* minimum between `self` and `other`.
            ///
            /// # Examples
            /// ```
            #[cfg_attr(nightly_float, doc = "# #![feature(f16, f128)]")]
            /// # use devela_base_core::Cmp;
            #[doc = "assert_eq![-1.0, Cmp(2.0" $f ").min(-1.0)];"]
            #[doc = "assert_eq![1.0, Cmp(1.0" $f ").min(2.0)];"]
            #[doc = "assert_eq![-0.0, Cmp(-0.0" $f ").min(0.0)];"]
            #[doc = "assert_eq![" $f "::NEG_INFINITY, Cmp(" $f "::INFINITY).min("
                $f "::NEG_INFINITY)];"]
            /// ```
            #[must_use] #[inline(always)]
            pub const fn min(self, other: $f) -> $f { self.0.min(other) }

            /// Returns `true` if `self == other` using total order.
            #[must_use] #[inline(always)]
            pub const fn eq(self, other: $f) -> bool { Cmp(self.total_cmp(other)).eq(Equal) }

            /// Returns `true` if `self != other` using total order.
            #[must_use] #[inline(always)]
            pub const fn ne(self, other: $f) -> bool { Cmp(self.total_cmp(other)).ne(Equal) }

            /// Returns `true` if `self < other` using total order.
            #[must_use] #[inline(always)]
            pub const fn lt(self, other: $f) -> bool { Cmp(self.total_cmp(other)).eq(Less) }

            /// Returns `true` if `self <= other` using total order.
            #[must_use] #[inline(always)]
            pub const fn le(self, other: $f) -> bool { Cmp(self.total_cmp(other)).ne(Greater) }

            /// Returns `true` if `self > other` using total order.
            #[must_use] #[inline(always)]
            pub const fn gt(self, other: $f) -> bool { Cmp(self.total_cmp(other)).eq(Greater) }

            /// Returns `true` if `self >= other` using total order.
            #[must_use] #[inline(always)]
            pub const fn ge(self, other:$f) -> bool { Cmp(self.total_cmp(other)).ne(Less) }

            /// Returns `true` if `self` is sign positive.
            #[must_use] #[inline(always)]
            pub const fn is_positive(self) -> bool { self.0.is_sign_positive() }

            /// Returns `true` if `self` is sign negative.
            #[must_use] #[inline(always)]
            pub const fn is_negative(self) -> bool { self.0.is_sign_negative() }

            /// Returns `true` if `self` is infinite (either negative or positive).
            #[must_use] #[inline(always)]
            pub const fn is_infinite(self) -> bool { self.0.is_infinite() }

            /// Returns `true` if `self` is neither infinite nor NaN.
            #[must_use] #[inline(always)]
            pub const fn is_finite(self) -> bool { self.0.is_finite() }

            /// Returns `true` if `self` is NaN.
            #[must_use] #[inline(always)]
            pub const fn is_nan(self) -> bool { self.0.is_nan() }

            /// Returns `true` if `self` is subnormal.
            #[must_use] #[inline(always)]
            pub const fn is_subnormal(self) -> bool { self.0.is_subnormal() }

            /// Returns `true` if `self` is neither zero, infinite, subnormal, or NaN.
            #[must_use] #[inline(always)]
            pub const fn is_normal(self) -> bool { self.0.is_normal() }
        }
    }};
}
impl_comparing!();

#[rustfmt::skip]
impl Cmp<Ordering> {
    /// Compares and returns `self` clamped between `min` and `max`.
    #[must_use]
    pub const fn clamp(self, min: Ordering, max: Ordering) -> Ordering {
        if (self.0 as i8) < min as i8 { min }
        else if self.0 as i8 > max as i8 { max }
        else { self.0 }
    }
    /// Compares and returns the maximum between `self` and `other`.
    #[must_use]
    pub const fn max(self, other: Ordering) -> Ordering {
        if self.0 as i8 > other as i8 { self.0 } else { other }
    }
    /// Compares and returns the minimum between `self` and `other`.
    #[must_use]
    pub const fn min(self, other: Ordering) -> Ordering {
        if (self.0 as i8) < other as i8 { self.0 } else { other }
    }
    /// Returns `true` if `self == other`.
    #[must_use]
    pub const fn eq(self, other: Ordering) -> bool { self.0 as i8 == other as i8 }
    /// Returns `true` if `self != other`.
    #[must_use]
    pub const fn ne(self, other: Ordering) -> bool { self.0 as i8 != other as i8 }
    /// Returns `true` if `self < other`.
    #[must_use]
    pub const fn lt(self, other: Ordering) -> bool { (self.0 as i8) < other as i8 }
    /// Returns `true` if `self <= other`.
    #[must_use]
    pub const fn le(self, other: Ordering) -> bool { self.0 as i8 <= other as i8 }
    /// Returns `true` if `self > other`.
    #[must_use]
    pub const fn gt(self, other: Ordering) -> bool { self.0 as i8 > other as i8 }
    /// Returns `true` if `self >= other`.
    #[must_use]
    pub const fn ge(self, other: Ordering) -> bool { self.0 as i8 >= other as i8 }
}

#[cfg(test)]
mod test_min_max_clamp {
    use super::Cmp;

    #[test]
    fn min_max_clamp() {
        assert_eq![Some(2), Cmp(2).pmin(5)];
        assert_eq![Some(2), Cmp(5).pmin(2)];
        assert_eq![Some(2.), Cmp(2.).pmin(5.)];

        assert_eq![Some(5), Cmp(2).pmax(5)];
        assert_eq![Some(5), Cmp(5).pmax(2)];
        assert_eq![Some(5.), Cmp(2.).pmax(5.)];

        assert_eq![Some(3), Cmp(3).pclamp(2, 5)];
        assert_eq![Some(3.), Cmp(3.).pclamp(2., 5.)];
        assert_eq![Some(2), Cmp(1).pclamp(2, 5)];
        assert_eq![Some(5), Cmp(7).pclamp(2, 5)];
    }

    #[test]
    fn float() {
        let (zero, negzero, one, negone) =
            (Cmp(0.0_f32), Cmp(-0.0_f32), Cmp(1.0_f32), Cmp(-1.0_f32));
        let (nan1, nan2) = (Cmp(f32::NAN), Cmp(0.0_f32 / 0.0_f32));
        let (inf, neginf) = (Cmp(f32::INFINITY), Cmp(f32::NEG_INFINITY));
        let sub = Cmp(1.401298464e-45_f32);
        let (min, negmin) = (Cmp(f32::MIN_POSITIVE), Cmp(-f32::MIN_POSITIVE));

        assert![nan1.0.is_nan()];
        assert![nan2.0.is_nan()];
        assert![!zero.0.is_nan()];
        assert![!negzero.0.is_nan()];
        assert![!one.0.is_nan()];
        assert![!negone.0.is_nan()];
        assert![!inf.0.is_nan()];
        assert![!neginf.0.is_nan()];
        assert![!min.0.is_nan()];
        assert![!negmin.0.is_nan()];

        assert![negone.0.is_sign_negative()];
        assert![negzero.0.is_sign_negative()];
        assert![neginf.0.is_sign_negative()];
        assert![!negone.0.is_sign_positive()];
        assert![!negzero.0.is_sign_positive()];
        assert![!neginf.0.is_sign_positive()];

        assert![sub.0.is_subnormal() && !sub.0.is_normal()];
        assert![!zero.0.is_subnormal() && !zero.0.is_normal()];
        assert![one.0.is_normal() && !one.0.is_subnormal()];
        assert![min.0.is_normal() && !min.0.is_subnormal()];
        assert![negmin.0.is_normal() && !negmin.0.is_subnormal()];
    }
}
