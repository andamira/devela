// devela_base_core::num::quant::sign
//
//! Defines the [`Sign`] of a number.
//
// TOC
// - enum Sign
// - impl Into<Sign>
// - impl From<Sign> TryFrom<Sign>

use crate::InvalidValue;

#[doc = crate::_TAG_QUANT!()]
/// Represents the sign of a number.
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Sign {
    /// A negative sign (-).
    Negative = -1,

    /// An absence of sign, associated with Zero. (The default)
    #[default]
    None = 0,

    /// A positive sign (+).
    Positive = 1,
}

/* Into<Sign> */

// helper macro to implement conversion from numbers to Sign
macro_rules! impl_into_sign {
    // integer primitives
    (int: $($int:ty),+) => { $( impl_into_sign![@int: $int]; )+ };
    (@int: $int:ty) => {
        impl From<$int> for Sign {
            /// Returns `None` if 0, `Positive` if > 0 and `Negative` if < 0.
            fn from(n: $int) -> Sign {
                match n {
                    0 => Sign::None,
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
            /// Returns `None` if 0.0, `Positive` if > 0 and `Negative` if < 0.
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
            /// Returns 0 if `None`, 1 if `Positive` and -1 if `Negative`.
            fn from(s: Sign) -> $sint {
                match s {
                    Sign::None => 0,
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

            /// Returns 0 if `None` and 1 if `Positive`.
            ///
            /// # Errors
            /// Returns [`InvalidValue`] if the sign is `Negative`.
            fn try_from(s: Sign) -> Result<$uint, InvalidValue> {
                match s {
                    Sign::None => Ok(0),
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
            /// Returns 0.0 if `None`, 1.0 if `Positive` and -1.0 if `Negative`.
            fn from(s: Sign) -> $float {
                match s {
                    Sign::None => 0.0,
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

            /// Returns `true` if `None` and `false` if `Negative`.
            ///
            /// # Errors
            /// Returns [`InvalidValue`] if the sign is `None`.
            fn try_from(s: Sign) -> Result<bool, InvalidValue> {
                match s {
                    Sign::Positive => Ok(true),
                    Sign::Negative => Ok(false),
                    Sign::None => Err(InvalidValue),
                }
            }
        }
    };
}
impl_from_sign![sint: i8, i16, i32, i64, i128, isize];
impl_from_sign![uint: u8, u16, u32, u64, u128, usize];
impl_from_sign![float: f32, f64];
impl_from_sign![bool];
