// devela::num::ops::counting
//
//!
//

use crate::num::{Int, NumResult as Result};

/// Numeric counting-related functionality.
///
/// These methods are implemented as const in the [`Int`][crate::num::Int] wrapper.
pub trait NumOpsCounting: Sized {
    /// Returns the factorial.
    fn factorial(&self) -> Result<Self>;

    /// Permutations of `n` items taken `r` at a time, ordered.
    fn permute(&self, r: &Self) -> Result<Self>;

    /// Permutations of `n` items taken `r` at a time with repetitions, ordered.
    fn permute_rep(&self, r: &Self) -> Result<Self>;

    /// Combinations of `n` items taken `r` at a time, ordered.
    fn combine(&self, r: &Self) -> Result<Self>;

    /// Combinations of `n` items taken `r` at a time with repetitions, unordered.
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
