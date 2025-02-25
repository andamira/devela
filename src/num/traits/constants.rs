// devela::num::traits::constants
//
//! Defines `ExtNumConst` and implements it for primitives.
//

use crate::{
    unwrap, ExtFloatConst, NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8,
    NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

/// Fundamental numeric constants for both integer and floating-point types.
pub trait ExtNumConst {
    /// The underlying numeric type implementing this trait.
    type Num;

    /// The additive identity (`0`), if applicable.
    const NUM_ZERO: Option<Self::Num>;

    /// The multiplicative identity (`1`).
    const NUM_ONE: Self::Num;

    /// The only even prime and the fundamental doubling factor (`2`).
    const NUM_TWO: Self::Num;

    /// The smallest odd prime and the first nontrivial divisor (`3`).
    const NUM_THREE: Self::Num;

    /// The additive inverse of `ONE` (`-1`), if applicable.
    const NUM_NEG_ONE: Option<Self::Num>;

    /// The smallest representable positive value.
    const NUM_MIN_POSITIVE: Self::Num;

    /// The greatest representable negative value, if applicable.
    const NUM_MAX_NEGATIVE: Option<Self::Num>;

    /// The maximum power of two within the type’s range.
    const NUM_MAX_POWER_OF_TWO: Self::Num;
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
    };
    ($T:ty | $U:ty: $ZERO:expr, $ONE:expr, $TWO:expr, $THREE:expr,
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr) => {
        impl_ext_num_const![@$T|$U: $ZERO, $ONE, $TWO, $THREE,
        $NEG_ONE, $MIN_POS, $MAX_NEG, $MAX_POW2];
    };
    (@$T:ty | $U:ty: $ZERO:expr, $ONE:expr, $TWO:expr, $THREE:expr,
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr) => {
        impl ExtNumConst for $T {
            type Num = $T;
            const NUM_ZERO: Option<$T> = $ZERO;
            const NUM_ONE: $T = $ONE;
            const NUM_TWO: $T = $TWO;
            const NUM_THREE: $T = $THREE;
            const NUM_NEG_ONE: Option<$T> = $NEG_ONE;
            const NUM_MIN_POSITIVE: $T = $MIN_POS;
            const NUM_MAX_NEGATIVE: Option<$T> = $MAX_NEG;
            const NUM_MAX_POWER_OF_TWO: $T = $MAX_POW2;
        }
    };

    /* specific impls */

    (float: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0.0), 1.0, 2.0, 3.0,
            Some(-1.0),         // NEG_ONE
            <$T>::MIN_POSITIVE, // MIN_POS
            Some(-0.0),         // MAX_NEG // ↓ MAX_POW2
            <$T>::from_bits(((<$T>::EXPONENT_BIAS as $U << 1) << (<$T>::SIGNIFICAND_BITS)))
        ];
    )+};
    (int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0), 1, 2, 3,
            Some(-1),   // NEG_ONE
            1,          // MIN_POS
            Some(-1),   // MAX_NEG
            <$T>::MAX - (<$T>::MAX >> 1) // MAX_POW2
        ];
    )+};
    (uint: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0), 1, 2, 3,
            None,   // NEG_ONE
            1,      // MIN_POS
            None,   // MAX_NEG
            <$T>::MAX ^ (<$T>::MAX >> 1) // MAX_POW2
          ];
    )+};
    (non0int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            None,
            unwrap![some <$T>::new(1)],
            unwrap![some <$T>::new(2)],
            unwrap![some <$T>::new(3)],
            <$T>::new(-1),              // NEG_ONE
            unwrap![some <$T>::new(1)], // MIN_POS
            <$T>::new(-1),              // MAX_NEG
            unwrap![some <$T>::new(<$T>::MAX.get() - (<$T>::MAX.get() >> 1))] // MAX_POW2
        ];
    )+};
    (non0uint: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            None,
            unwrap![some <$T>::new(1)],
            unwrap![some <$T>::new(2)],
            unwrap![some <$T>::new(3)],
            None,                       // NEG_ONE
            unwrap![some <$T>::new(1)], // MIN_POS
            None,                       // MAX_NEG
            unwrap![some <$T>::new(<$T>::MAX.get() ^ (<$T>::MAX.get() >> 1))] // MAX_POW2
        ];
    )+};
}
impl_ext_num_const![];

#[cfg(test)]
mod tests {
    use super::{ExtFloatConst, ExtNumConst, NonZeroI8, NonZeroU8};

    #[test]
    fn float() {
        assert_eq!(f32::NUM_ZERO, Some(0.0));
        assert_eq!(f32::NUM_ONE, 1.0);
        assert_eq!(f32::NUM_TWO, 2.0);
        assert_eq!(f32::NUM_THREE, 3.0);
        assert_eq!(f32::NUM_NEG_ONE, Some(-1.0));
        assert_eq!(f32::NUM_MIN_POSITIVE, f32::MIN_POSITIVE);
        assert_eq!(f32::NUM_MAX_NEGATIVE, Some(-0.0));
        assert_eq!(f32::NUM_MAX_POWER_OF_TWO, 2.0_f32.powi(f32::EXPONENT_BIAS as i32));
        assert_eq!(f64::NUM_MAX_POWER_OF_TWO, 2.0_f64.powi(f64::EXPONENT_BIAS as i32));
    }
    #[test]
    fn int() {
        assert_eq!(i8::NUM_ZERO, Some(0));
        assert_eq!(i8::NUM_ONE, 1);
        assert_eq!(i8::NUM_TWO, 2);
        assert_eq!(i8::NUM_THREE, 3);
        assert_eq!(i8::NUM_NEG_ONE, Some(-1));
        assert_eq!(i8::NUM_MIN_POSITIVE, 1);
        assert_eq!(i8::NUM_MAX_NEGATIVE, Some(-1));
        assert_eq!(i8::NUM_MAX_POWER_OF_TWO, 64);
    }
    #[test]
    fn uint() {
        assert_eq!(u8::NUM_ZERO, Some(0));
        assert_eq!(u8::NUM_ONE, 1);
        assert_eq!(u8::NUM_TWO, 2);
        assert_eq!(u8::NUM_THREE, 3);
        assert_eq!(u8::NUM_NEG_ONE, None);
        assert_eq!(u8::NUM_MIN_POSITIVE, 1);
        assert_eq!(u8::NUM_MAX_NEGATIVE, None);
        assert_eq!(u8::NUM_MAX_POWER_OF_TWO, 128);
    }
    #[test]
    fn non0int() {
        assert_eq!(NonZeroI8::NUM_ZERO, None);
        assert_eq!(NonZeroI8::NUM_ONE, NonZeroI8::new(1).unwrap());
        assert_eq!(NonZeroI8::NUM_TWO, NonZeroI8::new(2).unwrap());
        assert_eq!(NonZeroI8::NUM_THREE, NonZeroI8::new(3).unwrap());
        assert_eq!(NonZeroI8::NUM_NEG_ONE, Some(NonZeroI8::new(-1).unwrap()));
        assert_eq!(NonZeroI8::NUM_MIN_POSITIVE, NonZeroI8::new(1).unwrap());
        assert_eq!(NonZeroI8::NUM_MAX_NEGATIVE, Some(NonZeroI8::new(-1).unwrap()));
        assert_eq!(NonZeroI8::NUM_MAX_POWER_OF_TWO, NonZeroI8::new(64).unwrap());
    }
    #[test]
    fn non0uint() {
        assert_eq!(NonZeroU8::NUM_ZERO, None);
        assert_eq!(NonZeroU8::NUM_ONE, NonZeroU8::new(1).unwrap());
        assert_eq!(NonZeroU8::NUM_TWO, NonZeroU8::new(2).unwrap());
        assert_eq!(NonZeroU8::NUM_THREE, NonZeroU8::new(3).unwrap());
        assert_eq!(NonZeroU8::NUM_NEG_ONE, None);
        assert_eq!(NonZeroU8::NUM_MIN_POSITIVE, NonZeroU8::new(1).unwrap());
        assert_eq!(NonZeroU8::NUM_MAX_NEGATIVE, None);
        assert_eq!(NonZeroU8::NUM_MAX_POWER_OF_TWO, NonZeroU8::new(128).unwrap());
    }
}
