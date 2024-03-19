// devela::num::sign
//
//! the sign of a number.
//
// TOC
// - definition
// - impl Into Sign
// - impl (Try)From Sign

use crate::{
    code::ConstDefault,
    num::{NumError, NumResult as Result},
};
use NumError::Invalid;

/// Represents the sign of a number.
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

impl ConstDefault for Sign {
    /// No sign.
    const DEFAULT: Self = Sign::None;
}

/* Into Sign */

// helper macro to implement conversion from numbers to Sign
macro_rules! impl_into_sign {
    // integer primitives
    (int: $($int:ty),+) => { $( impl_into_sign![@int: $int]; )+ };
    (@int: $int:ty) => {
        impl From<$int> for Sign {
            /// Returns `None` if 0, `Positive` if > 0 and `Negative` if < 0.
            #[must_use] #[inline]
            fn from(n: $int) -> Sign {
                match n {
                    0 => Sign::None,
                    1.. => Sign::Positive,
                    #[allow(unreachable_patterns)] // for unsigned
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
            #[must_use] #[inline]
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
            #[must_use] #[inline]
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

/* (Try)From Sign */

// helper macro to implement conversion from Sign to numbers (1, 0, -1)
macro_rules! impl_from_sign {
    // signed integer primitives
    (sint: $($sint:ty),+) => { $( impl_from_sign![@sint: $sint]; )+ };
    (@sint: $sint:ty) => {
        impl From<Sign> for $sint {
            /// Returns 0 if `None`, 1 if `Positive` and -1 if `Negative`.
            #[must_use] #[inline]
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
            type Error = NumError;

            /// Returns 0 if `None` and 1 if `Positive`.
            ///
            /// # Errors
            /// Returns [`Invalid`] if the sign is `Negative`.
            #[inline]
            fn try_from(s: Sign) -> Result<$uint> {
                match s {
                    Sign::None => Ok(0),
                    Sign::Positive => Ok(1),
                    Sign::Negative => Err(Invalid),
                }
            }
        }
    };
    // floating-point primitives
    (float: $($float:ty),+) => { $( impl_from_sign![@float: $float]; )+ };
    (@float: $float:ty) => {
        impl From<Sign> for $float {
            /// Returns 0.0 if `None`, 1.0 if `Positive` and -1.0 if `Negative`.
            #[must_use] #[inline]
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
            type Error = NumError;

            /// Returns `true` if `None` and `false` if `Negative`.
            ///
            /// # Errors
            /// Returns [`Invalid`] if the sign is `None`.
            #[inline]
            fn try_from(s: Sign) -> Result<bool> {
                match s {
                    Sign::Positive => Ok(true),
                    Sign::Negative => Ok(false),
                    Sign::None => Err(Invalid),
                }
            }
        }
    };
}
impl_from_sign![sint: i8, i16, i32, i64, i128, isize];
impl_from_sign![uint: u8, u16, u32, u64, u128, usize];
impl_from_sign![float: f32, f64];
impl_from_sign![bool];
