// devela::num::ops::counting
//
//!
//

#[cfg(doc)]
use crate::num::NumErrors as E;
use crate::num::{Int, NumResult as Result};

/// Numeric counting-related functionality.
///
/// These methods are implemented as *const* in the [`Int`][crate::num::Int] wrapper.
pub trait NumOpsCounting: Sized {
    /// Returns the factorial.
    ///
    /// Permutations of *n* items, ordered, where $n = r$.
    ///
    /// # Formula
    /// $$ n! = 1 \cdot 2 \cdot 3 \cdot \ldots \cdot (n-1) \cdot n $$
    ///
    /// These are the maximum numbers whose factorials can fit within
    /// standard signed integer types:
    /// - 5 for `i8`, 7 for `i16`, 12 for `i32`, 20 for `i64` and 33 for `i128`.
    /// - 5 for `u8`, 8 for `u16`, 12 for `u32`, 20 for `u64` and 34 for `u128`.
    ///
    /// # Errors
    /// Returns [`NonNegativeRequired`][E::NonNegativeRequired] if $n<0$,
    /// and [`Overflow`][E::Overflow] if the result can't fit the type.
    fn factorial(&self) -> Result<Self>;

    /// Permutations of `n` items taken `r` at a time, ordered.
    ///
    /// When $n=r$ or $n=r-1$ the result is the same as calculating the factorial $n!$.
    ///
    /// # Formula
    /// $$ \large P(n,r) = \frac{n!}{(n−r)!} $$
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
    /// [`Overflow`][E::Overflow] if the result cant't fit the type.
    fn permute(&self, r: &Self) -> Result<Self>;

    /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
    ///
    /// # Formula
    /// $$ \large P_\text{rep}(n,r) = n_r $$
    ///
    /// # Errors
    /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
    ///
    fn permute_rep(&self, r: &Self) -> Result<Self>;

    /// Combinations of `n` items taken `r` at a time, unordered.
    ///
    /// # Formula
    /// $$ \large C(n,r) = {n \choose r} = \frac{n!}{(n−r)!r!} $$
    ///
    /// # Errors
    /// Returns [`MismatchedSizes`][E::MismatchedSizes] if $r > n$ and
    /// [`Overflow`][E::Overflow] if the result cant't fit the type.
    fn combine(&self, r: &Self) -> Result<Self>;

    /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
    ///
    /// Also known as *multichoose*.
    ///
    /// # Formula
    /// $$ \large C(n+r-1,r) = {n+k-1 \choose r} = \frac{(n+r-1)!}{(n−1)!r!} $$
    ///
    /// # Errors
    /// Returns [`Overflow`][E::Overflow] if the result cant't fit the type.
    fn combine_rep(&self, r: &Self) -> Result<Self>;
}

macro_rules! impl_counting {
    () => { impl_counting![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]; };
    ($($t:ty),+) => { $( impl_counting![@$t]; )+ };
    (@$t:ty) => {
        impl NumOpsCounting for $t {
            #[inline]
            fn factorial(&self) -> Result<Self> { Int(*self).factorial() }
            #[inline]
            fn permute(&self, r: &Self) -> Result<Self> { Int(*self).permute(*r) }
            #[inline]
            fn permute_rep(&self, r: &Self) -> Result<Self> { Int(*self).permute_rep(*r) }
            #[inline]
            fn combine(&self, r: &Self) -> Result<Self> { Int(*self).combine(*r) }
            #[inline]
            fn combine_rep(&self, r: &Self) -> Result<Self> { Int(*self).combine_rep(*r) }
        }
    };
}
impl_counting![];
