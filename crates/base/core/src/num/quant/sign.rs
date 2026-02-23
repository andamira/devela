// devela_base_core::num::quant::sign
//
//! Defines the [`Sign`] of a number.
//
// TOC
// - enum Sign
// - impls
// - impl Into<Sign>
// - impl From<Sign> TryFrom<Sign>

use crate::{ConstInitCore, InvalidValue, is, whilst};

#[doc = crate::_tags!(quant)]
/// The three-valued sign of a number: negative (−1), zero (0), or positive (+1).
#[doc = crate::_doc_location!("num/quant")]
///
/// This enum models the mathematical signum function and is useful when
/// representing or manipulating a value's sign independently of its magnitude.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Sign {
    /// A negative sign (-).
    Negative = -1,

    /// The zero sign (0). (The default)
    #[default]
    Zero = 0,

    /// A positive sign (+).
    Positive = 1,
}

impl ConstInitCore for Sign {
    /// No sign.
    const INIT: Self = Sign::Zero;
}

impl Sign {
    /// True if `self` == `other`.
    #[inline(always)]
    pub const fn eq(self, other: Self) -> bool {
        self as i8 == other as i8
    }

    /// True if `self` is `Negative`.
    #[inline(always)]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }

    /// True if `self` is `Positive`.
    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::Positive)
    }

    /// True if `self` is `Zero`.
    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        matches!(self, Self::Zero)
    }

    /// True if `self` is not `Zero`.
    #[inline(always)]
    pub const fn is_nonzero(self) -> bool {
        !self.is_zero()
    }

    /// Flips `Positive` ↔ `Negative`, keeps `Zero`.
    #[inline(always)]
    pub const fn invert(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
            Self::Zero => Self::Zero,
        }
    }

    /// Returns true if both have the same non-zero direction.
    #[inline(always)]
    pub const fn same_direction(self, other: Self) -> bool {
        matches!((self, other), (Self::Positive, Self::Positive) | (Self::Negative, Self::Negative))
    }

    /// Multiplies two signs with zero-dominance.
    pub const fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            (Self::Positive, s) | (s, Self::Positive) => s,
            (Self::Negative, Self::Negative) => Self::Positive,
        }
    }

    /// Raises the sign to `n` with zero-dominance: odd→same, even→`Positive`.
    pub const fn pow(self, n: u32) -> Self {
        match self {
            Self::Zero => Self::Zero,
            Self::Positive => Self::Positive,
            Self::Negative => is![n & 1 == 0, Self::Positive, Self::Negative],
        }
    }

    /// Converts `Negative`→`Positive`; keeps `Zero`/`Positive`.
    #[inline(always)]
    pub const fn abs(self) -> Self {
        match self {
            Self::Negative => Self::Positive,
            _ => self,
        }
    }

    /// Converts `Positive`→`Negative`; keeps `Zero`/`Negative`.
    #[inline(always)]
    pub const fn neg_abs(self) -> Self {
        match self {
            Self::Positive => Self::Negative,
            _ => self,
        }
    }

    /// Combines all signs in `iter` with zero-dominance.
    pub fn fold<I: IntoIterator<Item = Sign>>(iter: I) -> Self {
        let mut acc = Self::Positive;
        for s in iter.into_iter() {
            acc = acc.combine(s);
            if matches!(acc, Self::Zero) {
                return Self::Zero;
            }
        }
        acc
    }

    /// Combines all signs in `slice` with zero-dominance.
    pub const fn fold_slice(slice: &[Sign]) -> Self {
        let mut acc = Self::Positive;
        whilst! { i in 0..slice.len(); {
            let s = slice[i];
            acc = acc.combine(s);
            if matches!(acc, Self::Zero) {
                return Self::Zero;
            }
        }}
        acc
    }
}

/* Into<Sign> */

// helper macro to implement conversion from numbers to Sign
macro_rules! impl_into_sign {
    // integer primitives
    (int: $($int:ty),+) => { $( impl_into_sign![@int: $int]; )+ };
    (@int: $int:ty) => {
        impl From<$int> for Sign {
            /// Returns `Zero` if 0, `Positive` if > 0 and `Negative` if < 0.
            fn from(n: $int) -> Sign {
                match n {
                    0 => Sign::Zero,
                    1.. => Sign::Positive,
                    #[allow(unreachable_patterns, reason = "for unsigned")]
                    _ => Sign::Negative,
                }
            }
        }
    };
    // floating-point primitives
    (float: $($float:ty),+) => { $( impl_into_sign![@float: $float]; )+ };
    (@float: $float:ty) => {
        impl From<$float> for Sign {
            /// Returns `Zero` if 0.0, `Positive` if > 0 and `Negative` if < 0.
            fn from(n: $float) -> Sign {
                if n.is_sign_positive() {
                    Sign::Positive
                } else {
                    Sign::Negative
                }
            }
        }
    };
    // boolean primitive
    (bool) => {
        impl From<bool> for Sign {
            /// Returns `Positive` if `true` and `Negative` if `false`.
            fn from(n: bool) -> Sign {
                match n {
                    true => Sign::Positive,
                    false => Sign::Negative,
                }
            }
        }
    };
}
impl_into_sign![int: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
impl_into_sign![float: f32, f64];
impl_into_sign![bool];

/* impl From<Sign> TryFrom<Sign> */

// helper macro to implement conversion from Sign to numbers (1, 0, -1)
macro_rules! impl_from_sign {
    // signed integer primitives
    (sint: $($sint:ty),+) => { $( impl_from_sign![@sint: $sint]; )+ };
    (@sint: $sint:ty) => {
        impl From<Sign> for $sint {
            /// Returns 0 if `Zero`, 1 if `Positive` and -1 if `Negative`.
            fn from(s: Sign) -> $sint {
                match s {
                    Sign::Zero => 0,
                    Sign::Positive => 1,
                    Sign::Negative => -1,
                }
            }
        }
    };
    // unsigned integer primitives
    (uint: $($uint:ty),+) => { $( impl_from_sign![@uint: $uint]; )+ };
    (@uint: $uint:ty) => {
        impl TryFrom<Sign> for $uint {
            type Error = InvalidValue;

            /// Returns 0 if `Zero` and 1 if `Positive`.
            ///
            /// # Errors
            /// Returns [`InvalidValue`] if the sign is `Negative`.
            fn try_from(s: Sign) -> Result<$uint, InvalidValue> {
                match s {
                    Sign::Zero => Ok(0),
                    Sign::Positive => Ok(1),
                    Sign::Negative => Err(InvalidValue),
                }
            }
        }
    };
    // floating-point primitives
    (float: $($float:ty),+) => { $( impl_from_sign![@float: $float]; )+ };
    (@float: $float:ty) => {
        impl From<Sign> for $float {
            /// Returns 0.0 if `Zero`, 1.0 if `Positive` and -1.0 if `Negative`.
            fn from(s: Sign) -> $float {
                match s {
                    Sign::Zero => 0.0,
                    Sign::Positive => 1.0,
                    Sign::Negative => -1.0,
                }
            }
        }
    };
    // boolean primitive
    (bool) => {
        impl TryFrom<Sign> for bool {
            type Error = InvalidValue;

            /// Returns `true` if `Zero` and `false` if `Negative`.
            ///
            /// # Errors
            /// Returns [`InvalidValue`] if the sign is `Zero`.
            fn try_from(s: Sign) -> Result<bool, InvalidValue> {
                match s {
                    Sign::Positive => Ok(true),
                    Sign::Negative => Ok(false),
                    Sign::Zero => Err(InvalidValue),
                }
            }
        }
    };
}
impl_from_sign![sint: i8, i16, i32, i64, i128, isize];
impl_from_sign![uint: u8, u16, u32, u64, u128, usize];
impl_from_sign![float: f32, f64];
impl_from_sign![bool];
