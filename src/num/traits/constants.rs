// devela::num::traits::constants
//
//! Defines [`NumConst`] and implements it for primitives.
//
// TODO:
// - add NUM_FOUR
// - implement for NonValue*

#[rustfmt::skip]
use crate::{
    FloatConst,
    // NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize,
    // NonValueU8, NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};

#[doc = crate::_tags!(num)]
/// Fundamental numeric constants for both integer and floating-point types.
#[doc = crate::_doc_location!("num")]
pub trait NumConst: PartialEq<Self::Num> {
    /// The underlying numeric type implementing this trait.
    type Num;

    /* numeric type introspection */

    /// Whether the number can represent big quantities.
    const NUM_IS_BIG: bool;

    /// Whether the number uses an integer representation.
    const NUM_IS_INT: bool;

    /// Whether the number uses a floating-point representation.
    const NUM_IS_FLOAT: bool;

    /// Whether the number uses a fixed-point representation.
    const NUM_IS_FIXED: bool;

    /// Whether the number includes the sign.
    const NUM_IS_SIGNED: bool;

    /// Whether the number has a memory niche optimization.
    const NUM_IS_NICHE: bool;

    /* numeric constant values */

    /// The additive identity (`0`), if applicable.
    const NUM_ZERO: Option<Self::Num>;

    /// The multiplicative identity (`1`).
    const NUM_ONE: Option<Self::Num>;

    /// The only even prime and the fundamental doubling factor (`2`).
    const NUM_TWO: Option<Self::Num>;

    /// The smallest odd prime and the first nontrivial divisor (`3`).
    const NUM_THREE: Option<Self::Num>;

    /// The additive inverse of `ONE` (`-1`), if applicable.
    const NUM_NEG_ONE: Option<Self::Num>;

    /// The smallest representable value.
    const NUM_MIN: Option<Self::Num>;

    /// The greatest representable value.
    const NUM_MAX: Option<Self::Num>;

    /// The smallest representable positive value.
    const NUM_MIN_POSITIVE: Option<Self::Num>;

    /// The greatest representable negative value, if applicable.
    const NUM_MAX_NEGATIVE: Option<Self::Num>;

    /// The smallest normalized value (e.g. 0.0 for float, `MIN` for integers).
    const NUM_MIN_NORM: Option<Self::Num>;

    /// The greatest normalized value (e.g. 1.0 for float, `MAX` for integers).
    const NUM_MAX_NORM: Option<Self::Num>;

    /// The maximum representable power of two within the type's range.
    ///
    /// This is constructed using exact bit manipulation rather than arithmetic
    /// operations to ensure consistent results across all platforms.
    const NUM_MAX_POWER_OF_TWO: Option<Self::Num>;

    /* auto-implemented convenience methods */

    /// Whether `self` is equal to [`NUM_ZERO`][`Self::NUM_ZERO`].
    fn is_num_zero(&self) -> bool {
        Self::NUM_ZERO.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_ONE`][`Self::NUM_ONE`].
    fn is_num_one(&self) -> bool {
        Self::NUM_ONE.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_TWO`][`Self::NUM_TWO`].
    fn is_num_two(&self) -> bool {
        Self::NUM_TWO.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_THREE`][`Self::NUM_THREE`].
    fn is_num_three(&self) -> bool {
        Self::NUM_THREE.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_NEG_ONE`][`Self::NUM_NEG_ONE`].
    fn is_num_neg_one(&self) -> bool {
        Self::NUM_NEG_ONE.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MIN`][`Self::NUM_MIN`].
    fn is_num_min(&self) -> bool {
        Self::NUM_MIN.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MAX`][`Self::NUM_MAX`].
    fn is_num_max(&self) -> bool {
        Self::NUM_MAX.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to `Some(`[`NUM_MIN_POSITIVE`][`Self::NUM_MIN_POSITIVE`]`)`.
    fn is_num_min_positive(&self) -> bool {
        Self::NUM_MIN_POSITIVE.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MAX_NEGATIVE`][`Self::NUM_MAX_NEGATIVE`].
    fn is_num_max_negative(&self) -> bool {
        Self::NUM_MAX_NEGATIVE.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MIN_NORM`][`Self::NUM_MIN_NORM`].
    fn is_num_min_norm(&self) -> bool {
        Self::NUM_MIN_NORM.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MAX_NORM`][`Self::NUM_MAX_NORM`].
    fn is_num_max_norm(&self) -> bool {
        Self::NUM_MAX_NORM.is_some_and(|n| self == &n)
    }

    /// Whether `self` is equal to [`NUM_MAX_POWER_OF_TWO`][`Self::NUM_MAX_POWER_OF_TWO`].
    fn is_num_max_power_of_two(&self) -> bool {
        Self::NUM_MAX_POWER_OF_TWO.is_some_and(|n| self == &n)
    }
}

///
macro_rules! impl_ext_num_const {
    () => {
        impl_ext_num_const![float: f32|u32, f64|u64];
        impl_ext_num_const![int: i8|u8, i16|u16, i32|u32, i64|u64, i128|u128, isize|usize];
        impl_ext_num_const![uint: u8|u8, u16|u16, u32|u32, u64|u64, u128|u128, usize|usize];
        impl_ext_num_const![non0int: NonZeroI8|u8, NonZeroI16|u16, NonZeroI32|u32,
            NonZeroI64|u64, NonZeroI128|u128, NonZeroIsize|usize];
        impl_ext_num_const![non0uint: NonZeroU8|u8, NonZeroU16|u16, NonZeroU32|u32,
            NonZeroU64|u64, NonZeroU128|u128, NonZeroUsize|usize];
        // impl_ext_num_const![nonval_int: NonValueI8|u8, NonValueI16|u16, NonValueI32|u32,
        //     NonValueI64|u64, NonValueI128|u128, NonValueIsize|usize];
    };
    ($T:ty | $U:ty: $ZERO:expr, $ONE:expr, $TWO:expr, $THREE:expr,
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr, $MIN_NORM:expr, $MAX_NORM:expr,
     $IS_BIG:literal, $IS_INT:literal, $IS_FLOAT:literal, $IS_FIXED:literal,
     $IS_SIGNED:literal, $IS_NICHE:literal) => {
        impl_ext_num_const![@$T|$U: $ZERO, $ONE, $TWO, $THREE,
        $NEG_ONE, $MIN_POS, $MAX_NEG, $MAX_POW2, $MIN_NORM, $MAX_NORM,
        $IS_BIG, $IS_INT, $IS_FLOAT, $IS_FIXED, $IS_SIGNED, $IS_NICHE];
    };
    (@$T:ty | $U:ty: $ZERO:expr, $ONE:expr, $TWO:expr, $THREE:expr,
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr, $MIN_NORM:expr, $MAX_NORM:expr,
     $IS_BIG:literal, $IS_INT:literal, $IS_FLOAT:literal, $IS_FIXED:literal,
     $IS_SIGNED:literal, $IS_NICHE:literal) => {
        impl NumConst for $T {
            type Num = $T;
            // introspection
            const NUM_IS_BIG: bool = $IS_BIG;
            const NUM_IS_INT: bool = $IS_INT;
            const NUM_IS_FLOAT: bool = $IS_FLOAT;
            const NUM_IS_FIXED: bool = $IS_FIXED;
            const NUM_IS_SIGNED: bool = $IS_SIGNED;
            const NUM_IS_NICHE: bool = $IS_NICHE;
            // constant values
            const NUM_ZERO: Option<$T> = $ZERO;
            const NUM_ONE: Option<$T> = $ONE;
            const NUM_TWO: Option<$T> = $TWO;
            const NUM_THREE: Option<$T> = $THREE;
            const NUM_NEG_ONE: Option<$T> = $NEG_ONE;
            const NUM_MIN: Option<$T> = Some(<$T>::MIN); // FIXME: receive the some
            const NUM_MAX: Option<$T> = Some(<$T>::MAX); // FIXME
            const NUM_MIN_POSITIVE: Option<$T> = $MIN_POS;
            const NUM_MAX_NEGATIVE: Option<$T> = $MAX_NEG;
            const NUM_MIN_NORM: Option<$T> = $MIN_NORM;
            const NUM_MAX_NORM: Option<$T> = $MAX_NORM;
            const NUM_MAX_POWER_OF_TWO: Option<$T> = $MAX_POW2;
        }
    };

    /* specific impls */

    (float: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0.0), Some(1.0), Some(2.0), Some(3.0), // 0, 1, 2, 3
            Some(-1.0),               // NEG_ONE
            Some(<$T>::MIN_POSITIVE), // MIN_POS
            Some(-0.0),               // MAX_NEG // ↓ MAX_POW2
            Some(<$T>::from_bits((<$T>::EXPONENT_BIAS as $U << 1) << (<$T>::SIGNIFICAND_BITS))),
            Some(0.0), Some(1.0), // MIN_NORM, MAX_NORM
            false, // IS_BIG
            false, // IS_INT
            true,  // IS_FLOAT
            false, // IS_FIXED
            true,  // IS_SIGNED
            false  // IS_NICHE
        ];
    )+};
    (int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0), Some(1), Some(2), Some(3), // 0, 1, 2, 3
            Some(-1),   // NEG_ONE
            Some(1),    // MIN_POS
            Some(-1),   // MAX_NEG
            Some(<$T>::MAX - (<$T>::MAX >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            true,  // IS_SIGNED
            false  // IS_NICHE
        ];
    )+};
    (uint: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0), Some(1), Some(2), Some(3), // 0, 1, 2, 3
            None,    // NEG_ONE
            Some(1), // MIN_POS
            None,    // MAX_NEG
            Some(<$T>::MAX ^ (<$T>::MAX >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            false, // IS_SIGNED
            false  // IS_NICHE
          ];
    )+};
    // TODO: parameterize, for…
    // RENAME: niche (the new constructor returns Option)
    (non0int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            <$T>::new(0), <$T>::new(1), <$T>::new(2), <$T>::new(3), // 0, 1, 2, 3
            <$T>::new(-1), // NEG_ONE
            <$T>::new(1),  // MIN_POS
            <$T>::new(-1), // MAX_NEG
            <$T>::new(<$T>::MAX.get() - (<$T>::MAX.get() >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            true,  // IS_SIGNED
            true   // IS_NICHE
        ];
    )+};
    (non0uint: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            <$T>::new(0), <$T>::new(1), <$T>::new(2), <$T>::new(3), // 0, 1, 2, 3
            None,          // NEG_ONE
            <$T>::new(1),  // MIN_POS
            None,          // MAX_NEG
            <$T>::new(<$T>::MAX.get() ^ (<$T>::MAX.get() >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            false, // IS_SIGNED
            true   // IS_NICHE
        ];
    )+};

    // TODO
    (nonval_int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            <$T>::new(0), <$T>::new(1), <$T>::new(2), <$T>::new(3), // 0, 1, 2, 3
            <$T>::new(-1), // NEG_ONE
            <$T>::new(1),  // MIN_POS
            <$T>::new(-1), // MAX_NEG
            <$T>::new(<$T>::MAX.get() - (<$T>::MAX.get() >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            true,  // IS_SIGNED
            true   // IS_NICHE
        ];
    )+};
    (nonval_uint: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            <$T>::new(0), <$T>::new(1), <$T>::new(2), <$T>::new(3), // 0, 1, 2, 3
            None,          // NEG_ONE
            <$T>::new(1),  // MIN_POS
            None,          // MAX_NEG
            <$T>::new(<$T>::MAX.get() ^ (<$T>::MAX.get() >> 1)), // MAX_POW2
            Some(<$T>::MIN), // MIN_NORM
            Some(<$T>::MAX), // MAX_NORM
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            false, // IS_SIGNED
            true   // IS_NICHE
        ];
    )+};
}
impl_ext_num_const![];

#[cfg(test)]
mod tests {
    use super::{FloatConst, NonZeroI8, NonZeroU8, NumConst};

    #[test]
    fn float() {
        assert_eq!(f32::NUM_ZERO, Some(0.0));
        assert_eq!(f32::NUM_ONE, Some(1.0));
        assert_eq!(f32::NUM_TWO, Some(2.0));
        assert_eq!(f32::NUM_THREE, Some(3.0));
        assert_eq!(f32::NUM_NEG_ONE, Some(-1.0));
        assert_eq!(f32::NUM_MIN_POSITIVE, Some(f32::MIN_POSITIVE));
        assert_eq!(f32::NUM_MAX_NEGATIVE, Some(-0.0));
        assert_eq!(f32::NUM_MIN_NORM, Some(0.0));
        assert_eq!(f32::NUM_MAX_NORM, Some(1.0));

        // NOTE: The powi floating-point operation is non-deterministic.
        // 1.70141183e38_f32, 8.98846567431158e307_f64, …
        //
        // #[cfg(all(target_arch = "x86_64", not(miri)))]
        // {
        //     assert_eq!(f32::NUM_MAX_POWER_OF_TWO, Some(2.0_f32.powi(f32::EXPONENT_BIAS as i32)));
        //     assert_eq!(f64::NUM_MAX_POWER_OF_TWO, Some(2.0_f64.powi(f64::EXPONENT_BIAS as i32)));
        // }
        let expected_bits = ((f32::EXPONENT_BIAS as u32) << 1) << f32::SIGNIFICAND_BITS;
        let actual_bits = f32::NUM_MAX_POWER_OF_TWO.unwrap().to_bits();
        assert_eq!(actual_bits, expected_bits);
        //
        let expected_bits = ((f64::EXPONENT_BIAS as u64) << 1) << f64::SIGNIFICAND_BITS;
        let actual_bits = f64::NUM_MAX_POWER_OF_TWO.unwrap().to_bits();
        assert_eq!(actual_bits, expected_bits);
    }
    #[test]
    fn int() {
        assert_eq!(i8::NUM_ZERO, Some(0));
        assert_eq!(i8::NUM_ONE, Some(1));
        assert_eq!(i8::NUM_TWO, Some(2));
        assert_eq!(i8::NUM_THREE, Some(3));
        assert_eq!(i8::NUM_NEG_ONE, Some(-1));
        assert_eq!(i8::NUM_MIN_POSITIVE, Some(1));
        assert_eq!(i8::NUM_MAX_NEGATIVE, Some(-1));
        assert_eq!(i8::NUM_MIN_NORM, Some(i8::MIN));
        assert_eq!(i8::NUM_MAX_NORM, Some(i8::MAX));
        assert_eq!(i8::NUM_MAX_POWER_OF_TWO, Some(64));
    }
    #[test]
    fn uint() {
        assert_eq!(u8::NUM_ZERO, Some(0));
        assert_eq!(u8::NUM_ONE, Some(1));
        assert_eq!(u8::NUM_TWO, Some(2));
        assert_eq!(u8::NUM_THREE, Some(3));
        assert_eq!(u8::NUM_NEG_ONE, None);
        assert_eq!(u8::NUM_MIN_POSITIVE, Some(1));
        assert_eq!(u8::NUM_MAX_NEGATIVE, None);
        assert_eq!(u8::NUM_MIN_NORM, Some(u8::MIN));
        assert_eq!(u8::NUM_MAX_NORM, Some(u8::MAX));
        assert_eq!(u8::NUM_MAX_POWER_OF_TWO, Some(128));
    }
    #[test]
    fn non0int() {
        assert_eq!(NonZeroI8::NUM_ZERO, None);
        assert_eq!(NonZeroI8::NUM_ONE, NonZeroI8::new(1));
        assert_eq!(NonZeroI8::NUM_TWO, NonZeroI8::new(2));
        assert_eq!(NonZeroI8::NUM_THREE, NonZeroI8::new(3));
        assert_eq!(NonZeroI8::NUM_NEG_ONE, NonZeroI8::new(-1));
        assert_eq!(NonZeroI8::NUM_MIN_POSITIVE, NonZeroI8::new(1));
        assert_eq!(NonZeroI8::NUM_MAX_NEGATIVE, NonZeroI8::new(-1));
        assert_eq!(NonZeroI8::NUM_MIN_NORM, Some(NonZeroI8::MIN));
        assert_eq!(NonZeroI8::NUM_MAX_NORM, Some(NonZeroI8::MAX));
        assert_eq!(NonZeroI8::NUM_MAX_POWER_OF_TWO, NonZeroI8::new(64));
    }
    #[test]
    fn non0uint() {
        assert_eq!(NonZeroU8::NUM_ZERO, None);
        assert_eq!(NonZeroU8::NUM_ONE, NonZeroU8::new(1));
        assert_eq!(NonZeroU8::NUM_TWO, NonZeroU8::new(2));
        assert_eq!(NonZeroU8::NUM_THREE, NonZeroU8::new(3));
        assert_eq!(NonZeroU8::NUM_NEG_ONE, None);
        assert_eq!(NonZeroU8::NUM_MIN_POSITIVE, NonZeroU8::new(1));
        assert_eq!(NonZeroU8::NUM_MAX_NEGATIVE, None);
        assert_eq!(NonZeroU8::NUM_MIN_NORM, Some(NonZeroU8::MIN));
        assert_eq!(NonZeroU8::NUM_MAX_NORM, Some(NonZeroU8::MAX));
        assert_eq!(NonZeroU8::NUM_MAX_POWER_OF_TWO, NonZeroU8::new(128));
    }
}
