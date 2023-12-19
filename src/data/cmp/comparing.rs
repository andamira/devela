// devela::data::cmp::comparing
//
//! Helper wrapper for comparing.
//
// TOC
// - Comparing definition
// - impl core traits
// - impl Comparing for T: PartialOrd
// - impl Comparing for primitives
//   - int
//   - float
// - tests
//
// - WAIT:[const_float_classify](https://github.com/rust-lang/rust/issues/72505)
// - WAIT:[const_float_bits_conv](https://github.com/rust-lang/rust/issues/72447)
// - WAIT:[const_fn_floating_point_arithmetic](https://github.com/rust-lang/rust/issues/57241)

use crate::code::{iif, paste};
use core::cmp::Ordering::{self, *};

/// Provides comparing methods for `T`.
///
/// It implements the following *const* methods for comparing primitives:
/// `clamp`, `max`, `min`, `eq`, `ne`, `lt`, `le`, `gt`, `ge`.
/// In the case of floating-point primitives it uses total ordering and the
/// methods will only be const if the `unsafe_data` feature is enabled.
///
/// It additionally provides the non-const methods `pclamp`, `pmax`, `pmin`
/// for comparing [`PartialOrd`]ered values.
#[repr(transparent)]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "data")))]
pub struct Comparing<T>(pub T);

#[rustfmt::skip]
mod core_impls {
    use {super::{Comparing, Ordering}, core::fmt};

    impl<T: Clone> Clone for Comparing<T> { fn clone(&self) -> Self { Self(self.0.clone()) } }
    impl<T: Copy> Copy for Comparing<T> {}

    impl<T: PartialEq> PartialEq for Comparing<T> {
        #[inline] #[must_use]
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<T: Eq> Eq for Comparing<T> {}
    impl<T: PartialOrd> PartialOrd for Comparing<T> {
        #[inline] #[must_use]
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.0.partial_cmp(&other.0) }
    }
    impl<T: Ord> Ord for Comparing<T> {
        #[inline] #[must_use]
        fn cmp(&self, other: &Self) -> Ordering { self.0.cmp(&other.0) }
    }

    impl<T: fmt::Display> fmt::Display for Comparing<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { fmt::Display::fmt(&self.0, f) }
    }
    impl<T: fmt::Debug> fmt::Debug for Comparing<T> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_tuple("Comparing").field(&self.0).finish()
        }
    }
}

#[rustfmt::skip]
impl<T: PartialOrd> Comparing<T> {
    /// Compares and returns a [`PartialOrd`]ered `value` clamped between `min` and `max`.
    ///
    /// # Examples
    /// ```
    /// # use devela::data::Comparing;
    /// assert_eq![0.4, Comparing(1.0).pclamp(0.2, 0.4)];
    /// assert_eq![0.2, Comparing(0.0).pclamp(0.2, 0.4)];
    /// ```
    #[inline] #[must_use]
    pub fn pclamp(self, min: T, max: T) -> T { Self(self.pmax(min)).pmin(max) }

    /// Compares and returns the maximum of two [`PartialOrd`]ered values.
    ///
    /// Complements `core::cmp::`[`max`][`core::cmp::max] which requires
    /// [`Ord`]
    ///
    /// # Examples
    /// ```
    /// # use devela::data::Comparing;
    /// assert_eq![0.4, Comparing(0.2).pmax(0.4)];
    /// ```
    #[inline] #[must_use]
    pub fn pmax(self, other: T) -> T { if self.0 > other { self.0 } else { other } }

    /// Compares and returns the minimum of two [`PartialOrd`]ered values.
    ///
    /// Complements `core::cmp::`[`min`][`core::cmp::min] which requires
    /// [`Ord`]
    ///
    /// # Example
    /// ```
    /// # use devela::data::Comparing;
    /// assert_eq![0.2, Comparing(0.2).pmin(0.4)];
    /// ```
    #[inline] #[must_use]
    pub fn pmin(self, other: T) -> T { if self.0 < other { self.0 } else { other } }
}

// implement for primitives
macro_rules! impl_comparing {
    // $p: the integer type
    (int: $($p:ty),+) => { $( impl_comparing![@int: $p]; )+ };
    (@int: $p:ty) => {
        impl Comparing<$p> {
            /// Compares and returns `self` clamped between `min` and `max`.
            #[inline] #[must_use]
            pub const fn clamp(self, min: $p, max: $p) -> $p { Self(self.max(min)).min(max) }

            /// Compares and returns the maximum between `self` and `other`.
            #[inline] #[must_use]
            pub const fn max(self, other: $p) -> $p { if self.0 > other { self.0 } else { other } }

            /// Compares and returns the minimum between `self` and `other`.
            #[inline] #[must_use]
            pub const fn min(self, other: $p) -> $p { if self.0 < other { self.0 } else { other } }

            /// Returns `true` if `self == other`.
            #[inline] #[must_use]
            pub const fn eq(self, other: $p) -> bool { self.0 == other }
            /// Returns `true` if `self != other`.
            #[inline] #[must_use]
            pub const fn ne(self, other: $p) -> bool { self.0 != other }
            /// Returns `true` if `self < other`.
            #[inline] #[must_use]
            pub const fn lt(self, other: $p) -> bool { self.0 < other }
            /// Returns `true` if `self <= other`.
            #[inline] #[must_use]
            pub const fn le(self, other: $p) -> bool { self.0 <= other }
            /// Returns `true` if `self > other`.
            #[inline] #[must_use]
            pub const fn gt(self, other: $p) -> bool { self.0 > other }
            /// Returns `true` if `self >= other`.
            #[inline] #[must_use]
            pub const fn ge(self, other: $p) -> bool { self.0 >= other }
        }
    };

    // $f: the floating-point type
    // $b:  the bits of the floating-point primitive
    // $sh: the shift amount for the given bits ($b - 1)
    (float: $($f:ty:$b:literal:$sh:literal),+) => { $( impl_comparing![@float: $f:$b:$sh]; )+ };
    (@float: $f:ty:$b:literal:$sh:literal) => { paste! {
        impl Comparing<$f> {
            #[doc = "A (`const`) port of `" $f "::`[`total_cmp`][" $f "#method.total_cmp]."]
            ///
            /// # Features
            /// This function will only be `const` if the `unsafe_data` feature is enabled.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn total_cmp(self, other: $f) -> Ordering {
                // WAIT:const_float_bits_conv https://github.com/rust-lang/rust/issues/72447
                // let mut left = self.0.to_bits() as [<i $b>];
                // let mut right = other.to_bits() as [<i $b>];
                //
                // SAFETY: transmuting from f32 to i32 or f64 to i64 is safe.
                let mut left = unsafe { core::mem::transmute::<$f, [<i$b>]>(self.0) };
                let mut right = unsafe { core::mem::transmute::<$f, [<i$b>]>(other) };

                left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
                right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

                iif![left < right; Less; iif![left > right; Greater; Equal]]
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn total_cmp(self, other: $f) -> Ordering {
                let mut left = self.0.to_bits() as [<i $b>];
                let mut right = other.to_bits() as [<i $b>];

                left ^= (((left >> $sh) as [<u $b>]) >> 1) as [<i $b>];
                right ^= (((right >> $sh) as [<u $b>]) >> 1) as [<i $b>];

                iif![left < right; Less; iif![left > right; Greater; Equal]]
            }

            /// Compares and returns a clamped *total ordered* `self` between `min` and `max`.
            ///
            /// # Features
            /// This function will only be `const` if the `unsafe_data` feature is enabled.
            ///
            /// # Examples
            /// ```
            /// # use devela::data::Comparing;
            #[doc = "assert_eq![2.0, Comparing(5.0" $f ").clamp(-1.0, 2.0)];"]
            #[doc = "assert_eq![-1.0, Comparing(-5.0" $f ").clamp(-1.0, 2.0)];"]
            /// ```
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn clamp(self, min: $f, max: $f) -> $f { Self(self.max(min)).min(max) }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn clamp(self, min: $f, max: $f) -> $f { Self(self.max(min)).min(max) }

            /// Compares and returns the *total ordered* maximum between `self` and `other`.
            ///
            /// # Features
            /// This function will only be `const` if the `unsafe_data` feature is enabled.
            ///
            /// # Examples
            /// ```
            /// # use devela::data::Comparing;
            #[doc = "assert_eq![2.0, Comparing(2.0" $f ").max(-1.0)];"]
            #[doc = "assert_eq![2.0, Comparing(1.0" $f ").max(2.0)];"]
            #[doc = "assert_eq![0.0, Comparing(-0.0" $f ").max(0.0)];"]
            #[doc = "assert_eq![" $f "::INFINITY, Comparing(" $f "::INFINITY).max("
                $f "::NEG_INFINITY)];"]
            /// ```
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn max(self, other: $f) -> $f {
                match Self(self.0).total_cmp(other) { Greater | Equal => self.0, Less => other }
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn max(self, other: $f) -> $f {
                match self.0.total_cmp(&other) { Greater | Equal => self.0, Less => other }
            }

            /// Compares and returns the *total ordered* minimum between `self` and `other`.
            ///
            /// # Features
            /// This function will only be `const` if the `unsafe_data` feature is enabled.
            ///
            /// # Examples
            /// ```
            /// # use devela::data::Comparing;
            #[doc = "assert_eq![-1.0, Comparing(2.0" $f ").min(-1.0)];"]
            #[doc = "assert_eq![1.0, Comparing(1.0" $f ").min(2.0)];"]
            #[doc = "assert_eq![-0.0, Comparing(-0.0" $f ").min(0.0)];"]
            #[doc = "assert_eq![" $f "::NEG_INFINITY, Comparing(" $f "::INFINITY).min("
                $f "::NEG_INFINITY)];"]
            /// ```
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn min(self, other: $f) -> $f {
                match Self(self.0).total_cmp(other) { Greater | Equal => other, Less => self.0 }
            }
            // safe, non-const version (undocumented)
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn min(self, other: $f) -> $f {
                match self.0.total_cmp(&other) { Greater | Equal => other, Less => self.0 }
            }

            /// Returns `true` if `self == other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn eq(self, other: $f) -> bool { matches!(self.total_cmp(other), Equal) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn eq(self, other: $f) -> bool { matches!(self.total_cmp(other), Equal) }

            /// Returns `true` if `self != other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn ne(self, other: $f) -> bool { !matches!(self.total_cmp(other), Equal) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn ne(self, other: $f) -> bool { !matches!(self.total_cmp(other), Equal) }

            /// Returns `true` if `self < other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn lt(self, other: $f) -> bool { matches!(self.total_cmp(other), Less) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn lt(self, other: $f) -> bool { matches!(self.total_cmp(other), Less) }

            /// Returns `true` if `self <= other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn le(self, other: $f) -> bool {
                matches!(self.total_cmp(other), Less | Equal) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn le(self, other: $f) -> bool { matches!(self.total_cmp(other), Less | Equal) }

            /// Returns `true` if `self > other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn gt(self, other: $f) -> bool { matches!(self.total_cmp(other), Greater) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn gt(self, other: $f) -> bool { matches!(self.total_cmp(other), Greater) }

            /// Returns `true` if `self >= other` using total order.
            #[inline] #[must_use] #[cfg(feature = "unsafe_data")]
            pub const fn ge(self, other: $f) -> bool {
                matches!(self.total_cmp(other), Greater | Equal) }
            #[inline] #[must_use] #[allow(missing_docs)] #[cfg(not(feature = "unsafe_data"))]
            pub fn ge(self, other: $f) -> bool { matches!(self.total_cmp(other), Greater | Equal) }
        }
    }};
}
impl_comparing![int: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
impl_comparing![float: f32:32:31, f64:64:63];

#[cfg(test)]
mod test_min_max_clamp {
    use super::Comparing;

    #[test]
    fn min_max_clamp() {
        assert_eq![2, Comparing(2).pmin(5)];
        assert_eq![2, Comparing(5).pmin(2)];
        assert_eq![2., Comparing(2.).pmin(5.)];

        assert_eq![5, Comparing(2).pmax(5)];
        assert_eq![5, Comparing(5).pmax(2)];
        assert_eq![5., Comparing(2.).pmax(5.)];

        assert_eq![3, Comparing(3).pclamp(2, 5)];
        assert_eq![3., Comparing(3.).pclamp(2., 5.)];
        assert_eq![2, Comparing(1).pclamp(2, 5)];
        assert_eq![5, Comparing(7).pclamp(2, 5)];
    }
}
