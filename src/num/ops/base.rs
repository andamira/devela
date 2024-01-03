// devela::num::ops::base
//
//!
//

use crate::num::Int;

/// Numeric base-related functionality.
///
/// These methods are implemented as *const* in the [`Int`][crate::num::Int] wrapper.
pub trait NumOpsBase {
    /// Returns the number of digits in base 10.
    #[must_use]
    fn count_digits(&self) -> Self;

    /// Returns the number of digits in base 10 including the negative sign.
    #[must_use]
    fn count_digits_sign(&self) -> Self;

    /// Returns the number of digits in the given `base`.
    #[must_use]
    fn count_digits_base(&self, base: &Self) -> Self;

    /// Returns the number of digits in the given `base` including the negative sign.
    #[must_use]
    fn count_digits_base_sign(&self, base: &Self) -> Self;

    /// Returns the digital root in base 10.
    #[must_use]
    fn digital_root(&self) -> Self;

    /// Returns the digital root in the given `base`.
    #[must_use]
    fn digital_root_base(&self, base: &Self) -> Self;
}

macro_rules! impl_base {
    () => { impl_base![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize]; };
    ($($t:ty),+) => { $( impl_base![@$t]; )+ };
    (@$t:ty) => {
        impl NumOpsBase for $t {
            #[inline]
            fn count_digits(&self) -> Self {
                Int(*self).count_digits()
            }
            #[inline]
            fn count_digits_sign(&self) -> Self {
                Int(*self).count_digits_sign()
            }
            #[inline]
            fn count_digits_base(&self, base: &Self) -> Self {
                Int(*self).count_digits_base(*base)
            }
            #[inline]
            fn count_digits_base_sign(&self, base: &Self) -> Self {
                Int(*self).count_digits_base_sign(*base)
            }
            #[inline]
            fn digital_root(&self) -> Self {
                Int(*self).digital_root()
            }
            #[inline]
            fn digital_root_base(&self, base: &Self) -> Self {
                Int(*self).digital_root_base(*base)
            }
        }
    };
}
impl_base![];
