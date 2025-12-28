// devela_base_core::text::fmt::num::conf
//
//! Defines [`FmtNumConf`], [`FmtNumSign`].
//
// TOC
// struct FmtNumConf
// enum FmtNumSign

use crate::{_TAG_FMT, _TAG_NUM, ConstInitCore, Sign};

#[doc = _TAG_FMT!()]
#[doc = _TAG_NUM!()]
/// Configuration for numeric formatting.
///
/// This configuration applies uniformly to integers and floating-point numbers.
/// Fields that do not apply to a given number type are ignored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumConf {
    /// Whether to emit a sign and which one (+ / - / none).
    pub sign: FmtNumSign,

    /// Minimum number of digits in the integral part (zero-padded).
    pub int: u16,

    /// Number of fractional digits to emit.
    ///
    /// For integers, this is ignored.
    /// For floats, this specifies fixed-point precision.
    pub fract: u16,
}

impl ConstInitCore for FmtNumConf {
    const INIT: Self = FmtNumConf { sign: FmtNumSign::INIT, int: 0, fract: 0 };
}

#[rustfmt::skip]
impl FmtNumConf {
    /// Creates a default formatting configuration.
    ///
    /// This sets:
    /// - the default sign policy,
    /// - no minimum integral digit count,
    /// - and zero fractional digits.
    #[inline(always)]
    pub const fn new() -> Self { Self::INIT }

    /// Creates a formatting configuration for integers.
    ///
    /// This sets:
    /// - the default sign policy,
    /// - a minimum integral digit count,
    /// - and zero fractional digits.
    #[inline(always)]
    pub const fn new_int(int: u16) -> Self { Self::INIT.with_int(int) }

    /// Creates a formatting configuration for fixed-point floating-point numbers.
    ///
    /// This sets:
    /// - the default sign policy,
    /// - a minimum integral digit count,
    /// - and a fixed number of fractional digits.
    #[inline(always)]
    pub const fn new_float(int: u16, fract: u16) -> Self { Self::new_int(int).with_fract(fract) }

    //

    /// Sets the sign formatting policy.
    #[inline(always)]
    pub const fn set_sign(&mut self, sign: FmtNumSign) { self.sign = sign }
    /// Returns a copy with the given sign formatting policy.
    #[inline(always)]
    pub const fn with_sign(mut self, sign: FmtNumSign) -> Self { self.sign = sign; self }

    /// Sets the minimum number of integral digits (zero-padded).
    #[inline(always)]
    pub const fn set_int(&mut self, int: u16) { self.int = int }
    /// Returns a copy with the given minimum integral digit count.
    #[inline(always)]
    pub const fn with_int(mut self, int: u16) -> Self { self.int = int; self }

    /// Sets the number of fractional digits to emit.
    ///
    /// For integers, this value is ignored.
    #[inline(always)]
    pub const fn set_fract(&mut self, fract: u16) { self.fract = fract }
    /// Returns a copy with the given fractional digit count.
    ///
    /// For integers, this value is ignored.
    #[inline(always)]
    pub const fn with_fract(mut self, fract: u16) -> Self { self.fract = fract; self }
}

#[doc = _TAG_FMT!()]
#[doc = _TAG_NUM!()]
/// Controls how the sign of a number is formatted.
///
/// This enum specifies whether a sign glyph (`'-'` or `'+'`) is emitted,
/// independently of the numeric magnitude.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FmtNumSign {
    /// Only print '-' for negative numbers.
    /// (`-42 → -42`, `42 → 42`)
    ///
    /// This is the default.
    #[default]
    NegativeOnly,

    /// Always print a sign: '-' or '+'.
    /// (`-42 → -42`, `42 → +42`)
    Always,

    /// Never print a sign (absolute value).
    /// (`-42 → 42`, `42 → 42`)
    Never,

    /// Only print '+' for positive numbers.
    /// (`-42 → 42`, `42 → +42`)
    PositiveOnly,
}

impl ConstInitCore for FmtNumSign {
    const INIT: Self = FmtNumSign::NegativeOnly;
}

impl FmtNumSign {
    /// Returns a formatting sign policy derived from a numeric sign.
    ///
    /// This maps:
    /// - `Negative` → `NegativeOnly`
    /// - `Zero | Positive` → `PositiveOnly`
    pub const fn from_quant(sign: Sign) -> Self {
        match sign {
            Sign::Negative => Self::NegativeOnly,
            Sign::Zero | Sign::Positive => Self::PositiveOnly,
        }
    }
}
