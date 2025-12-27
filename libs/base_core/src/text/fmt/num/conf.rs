// devela_base_core::text::fmt::num::conf
//
//! Defines [`FmtNumConf`], [`FmtNumSign`].
//

#[doc = crate::_TAG_FMT!()]
#[doc = crate::_TAG_NUM!()]
/// Configuration for numeric formatting.
///
/// This configuration applies uniformly to integers and floating-point numbers.
/// Fields that do not apply to a given number type are ignored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumConf {
    /// Whether to emit a sign and which one (+ / - / none).
    pub sign: FmtNumSign,

    /// Minimum number of digits in the integral part (zero-padded).
    pub min_integral: u16,

    /// Number of fractional digits to emit.
    ///
    /// For integers, this is ignored.
    /// For floats, this specifies fixed-point precision.
    pub fract: u16,
}

#[rustfmt::skip]
impl FmtNumConf {
    /// Sets the sign formatting policy.
    #[inline(always)]
    pub const fn set_sign(&mut self, sign: FmtNumSign) { self.sign = sign }
    /// Returns a copy with the given sign formatting policy.
    #[inline(always)]
    pub const fn with_sign(mut self, sign: FmtNumSign) -> Self { self.sign = sign; self }

    /// Sets the minimum number of integral digits (zero-padded).
    #[inline(always)]
    pub const fn set_int(&mut self, min: u16) { self.min_integral = min }
    /// Returns a copy with the given minimum integral digit count.
    #[inline(always)]
    pub const fn with_int(mut self, min: u16) -> Self { self.min_integral = min; self }

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

#[doc = crate::_TAG_FMT!()]
#[doc = crate::_TAG_NUM!()]
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
