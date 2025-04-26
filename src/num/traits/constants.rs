// devela::num::traits::constants
//
//! Defines [`NumConst`] and implements it for primitives.
//
// TODO:RETHINK: implement for NonValue*
// - we would need to implement Num for it.
// - we also have to make ONE, TWO and THREE optional. (not a problem for NonExtreme)

use crate::{
    FloatConst, NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, unwrap,
};

#[doc = crate::TAG_NUM!()]
/// Fundamental numeric constants for both integer and floating-point types.
pub trait NumConst {
    /// The underlying numeric type implementing this trait.
    type Num;

    /* introspection */

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

    /* constant values */

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

    /// The smallest representable value.
    const NUM_MIN: Self::Num;

    /// The smallest representable positive value.
    const NUM_MIN_POSITIVE: Option<Self::Num>;

    /// The greatest representable value.
    const NUM_MAX: Self::Num;

    /// The greatest representable negative value, if applicable.
    const NUM_MAX_NEGATIVE: Option<Self::Num>;

    /// The maximum power of two within the type's range.
    const NUM_MAX_POWER_OF_TWO: Option<Self::Num>;
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
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr,
     $IS_BIG:literal, $IS_INT:literal, $IS_FLOAT:literal, $IS_FIXED:literal,
     $IS_SIGNED:literal, $IS_NICHE:literal) => {
        impl_ext_num_const![@$T|$U: $ZERO, $ONE, $TWO, $THREE,
        $NEG_ONE, $MIN_POS, $MAX_NEG, $MAX_POW2,
        $IS_BIG, $IS_INT, $IS_FLOAT, $IS_FIXED, $IS_SIGNED, $IS_NICHE];
    };
    (@$T:ty | $U:ty: $ZERO:expr, $ONE:expr, $TWO:expr, $THREE:expr,
     $NEG_ONE:expr, $MIN_POS:expr, $MAX_NEG:expr, $MAX_POW2:expr,
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
            const NUM_ONE: $T = $ONE;
            const NUM_TWO: $T = $TWO;
            const NUM_THREE: $T = $THREE;
            const NUM_NEG_ONE: Option<$T> = $NEG_ONE;
            const NUM_MIN: $T = <$T>::MIN;
            const NUM_MIN_POSITIVE: Option<$T> = $MIN_POS;
            const NUM_MAX: $T = <$T>::MAX;
            const NUM_MAX_NEGATIVE: Option<$T> = $MAX_NEG;
            const NUM_MAX_POWER_OF_TWO: Option<$T> = $MAX_POW2;
        }
    };

    /* specific impls */

    (float: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            Some(0.0), 1.0, 2.0, 3.0,
            Some(-1.0),               // NEG_ONE
            Some(<$T>::MIN_POSITIVE), // MIN_POS
            Some(-0.0),               // MAX_NEG // ↓ MAX_POW2
            Some(<$T>::from_bits(((<$T>::EXPONENT_BIAS as $U << 1) << (<$T>::SIGNIFICAND_BITS)))),
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
            Some(0), 1, 2, 3,
            Some(-1),   // NEG_ONE
            Some(1),    // MIN_POS
            Some(-1),   // MAX_NEG
            Some(<$T>::MAX - (<$T>::MAX >> 1)), // MAX_POW2
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
            Some(0), 1, 2, 3,
            None,    // NEG_ONE
            Some(1), // MIN_POS
            None,    // MAX_NEG
            Some(<$T>::MAX ^ (<$T>::MAX >> 1)), // MAX_POW2
            false, // IS_BIG
            true,  // IS_INT
            false, // IS_FLOAT
            false, // IS_FIXED
            false, // IS_SIGNED
            false  // IS_NICHE
          ];
    )+};
    (non0int: $( $T:ty | $U:ty ),+) => { $(
        impl_ext_num_const![$T|$U:
            None,
            unwrap![some <$T>::new(1)],
            unwrap![some <$T>::new(2)],
            unwrap![some <$T>::new(3)],
            <$T>::new(-1), // NEG_ONE
            <$T>::new(1),  // MIN_POS
            <$T>::new(-1), // MAX_NEG
            <$T>::new(<$T>::MAX.get() - (<$T>::MAX.get() >> 1)), // MAX_POW2
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
            None,
            unwrap![some <$T>::new(1)],
            unwrap![some <$T>::new(2)],
            unwrap![some <$T>::new(3)],
            None,          // NEG_ONE
            <$T>::new(1),  // MIN_POS
            None,          // MAX_NEG
            <$T>::new(<$T>::MAX.get() ^ (<$T>::MAX.get() >> 1)), // MAX_POW2
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
        assert_eq!(f32::NUM_ONE, 1.0);
        assert_eq!(f32::NUM_TWO, 2.0);
        assert_eq!(f32::NUM_THREE, 3.0);
        assert_eq!(f32::NUM_NEG_ONE, Some(-1.0));
        assert_eq!(f32::NUM_MIN_POSITIVE, Some(f32::MIN_POSITIVE));
        assert_eq!(f32::NUM_MAX_NEGATIVE, Some(-0.0));
        assert_eq!(f32::NUM_MAX_POWER_OF_TWO, Some(2.0_f32.powi(f32::EXPONENT_BIAS as i32)));
        assert_eq!(f64::NUM_MAX_POWER_OF_TWO, Some(2.0_f64.powi(f64::EXPONENT_BIAS as i32)));
    }
    #[test]
    fn int() {
        assert_eq!(i8::NUM_ZERO, Some(0));
        assert_eq!(i8::NUM_ONE, 1);
        assert_eq!(i8::NUM_TWO, 2);
        assert_eq!(i8::NUM_THREE, 3);
        assert_eq!(i8::NUM_NEG_ONE, Some(-1));
        assert_eq!(i8::NUM_MIN_POSITIVE, Some(1));
        assert_eq!(i8::NUM_MAX_NEGATIVE, Some(-1));
        assert_eq!(i8::NUM_MAX_POWER_OF_TWO, Some(64));
    }
    #[test]
    fn uint() {
        assert_eq!(u8::NUM_ZERO, Some(0));
        assert_eq!(u8::NUM_ONE, 1);
        assert_eq!(u8::NUM_TWO, 2);
        assert_eq!(u8::NUM_THREE, 3);
        assert_eq!(u8::NUM_NEG_ONE, None);
        assert_eq!(u8::NUM_MIN_POSITIVE, Some(1));
        assert_eq!(u8::NUM_MAX_NEGATIVE, None);
        assert_eq!(u8::NUM_MAX_POWER_OF_TWO, Some(128));
    }
    #[test]
    fn non0int() {
        assert_eq!(NonZeroI8::NUM_ZERO, None);
        assert_eq!(NonZeroI8::NUM_ONE, NonZeroI8::new(1).unwrap());
        assert_eq!(NonZeroI8::NUM_TWO, NonZeroI8::new(2).unwrap());
        assert_eq!(NonZeroI8::NUM_THREE, NonZeroI8::new(3).unwrap());
        assert_eq!(NonZeroI8::NUM_NEG_ONE, NonZeroI8::new(-1));
        assert_eq!(NonZeroI8::NUM_MIN_POSITIVE, NonZeroI8::new(1));
        assert_eq!(NonZeroI8::NUM_MAX_NEGATIVE, NonZeroI8::new(-1));
        assert_eq!(NonZeroI8::NUM_MAX_POWER_OF_TWO, NonZeroI8::new(64));
    }
    #[test]
    fn non0uint() {
        assert_eq!(NonZeroU8::NUM_ZERO, None);
        assert_eq!(NonZeroU8::NUM_ONE, NonZeroU8::new(1).unwrap());
        assert_eq!(NonZeroU8::NUM_TWO, NonZeroU8::new(2).unwrap());
        assert_eq!(NonZeroU8::NUM_THREE, NonZeroU8::new(3).unwrap());
        assert_eq!(NonZeroU8::NUM_NEG_ONE, None);
        assert_eq!(NonZeroU8::NUM_MIN_POSITIVE, NonZeroU8::new(1));
        assert_eq!(NonZeroU8::NUM_MAX_NEGATIVE, None);
        assert_eq!(NonZeroU8::NUM_MAX_POWER_OF_TWO, NonZeroU8::new(128));
    }
}
