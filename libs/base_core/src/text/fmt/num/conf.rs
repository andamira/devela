// devela_base_core::text::fmt::num::conf
//
//! Defines [`FmtNumConf`], [`FmtNumGroup`], [`FmtNumSign`].
//
// TOC
// struct FmtNumConf
// struct FmtNumGroup
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

    /// Whether the sign prefix participates in zero-padding.
    pub pad_sign: bool,

    /// Minimum number of digits in the integral part (zero-padded).
    pub int: u16,

    /// Number of fractional digits to emit.
    ///
    /// For integers, this is ignored.
    /// For floats, this specifies fixed-point precision.
    pub fract: u16,
}

impl ConstInitCore for FmtNumConf {
    const INIT: Self = FmtNumConf {
        sign: FmtNumSign::INIT,
        pad_sign: false,
        int: 0,
        fract: 0,
    };
}

#[rustfmt::skip]
impl FmtNumConf {
    const _SIZE: () = const { assert![size_of::<Self>() == 6]; };

    /* constructors */

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

    /// Sets the sign padding influence policy.
    #[inline(always)]
    pub const fn set_pad_sign(&mut self, pad_sign: bool) { self.pad_sign = pad_sign }
    /// Returns a copy with the given sign padding influence policy.
    #[inline(always)]
    pub const fn with_pad_sign(mut self, pad_sign: bool) -> Self { self.pad_sign = pad_sign; self }

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
/// Numeric digit grouping configuration.
///
/// This type controls structural digit grouping for formatted numbers,
/// such as thousands separators, without performing layout or localization.
///
/// Grouping is applied independently to the integral (left) and fractional
/// (right) digit sequences, counting from the radix point outward.
///
/// A grouping length of `0` disables grouping on that side,
/// as well as having a `None` separator for that side.
///
/// This configuration is numeric-decoration only:
/// it does not handle spacing, alignment, text direction, or locale rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FmtNumGroup {
    /// Number of digits per group on the integral (left) side.
    pub left_len: u8,
    /// Number of digits per group on the fractional (right) side.
    pub right_len: u8,
    /// Separator glyph inserted between digit groups on the integral (left) side.
    pub left_sep: Option<char>,
    /// Separator glyph inserted between digit groups on the fractional (right) side.
    pub right_sep: Option<char>,
}
#[rustfmt::skip]
impl Default for FmtNumGroup { fn default() -> Self { Self::INIT } }
impl ConstInitCore for FmtNumGroup {
    const INIT: Self = Self {
        left_len: 3,
        right_len: 0,
        left_sep: None,
        right_sep: None,
    };
}

#[rustfmt::skip]
impl FmtNumGroup {
    const _SIZE: () = const { assert![size_of::<Self>() == 12]; };

    /* constructors */

    /// Creates a default grouping configuration of 3 for the left part, and no separator.
    #[inline(always)]
    pub const fn new() -> Self { Self::INIT }

    /// Sets the `_` underscore for the left part, with groups of 3.
    pub const fn rust() -> Self { Self::INIT.with_sep('_') }
    ///
    /// Sets the `_` underscore for both parts, with groups of 3.
    pub const fn both_rust() -> Self { Self::INIT.with_sep('_').with_len(3) }

    /// Sets the `_` underscore for the left part, with groups of 4.
    pub const fn rust_hex() -> Self { Self::INIT.with_left(4, '_') }

    /// Sets the `_` underscore for both parts, with groups of 4.
    pub const fn both_rust_hex() -> Self { Self::INIT.with_sep('_').with_len(4) }

    /* individual lengths */

    /// Sets the left length to the given value.
    pub const fn set_left_len(&mut self, len: u8) { self.left_len = len; }
    /// Returns a copy with the left length set to the given value.
    pub const fn with_left_len(mut self, len: u8) -> Self { self.left_len = len; self }

    /// Sets the left length to the given value.
    pub const fn set_right_len(&mut self, len: u8) { self.right_len = len; }
    /// Returns a copy with the right length set to the given value.
    pub const fn with_right_len(mut self, len: u8) -> Self { self.right_len = len; self }

    /* individual separators */

    /// Sets the left separator to the given character.
    pub const fn set_left_sep(&mut self, sep: char) { self.left_sep = Some(sep); }
    /// Returns a copy with the left separator set to the given character.
    pub const fn with_left_sep(mut self, sep: char) -> Self { self.left_sep = Some(sep); self }
    /// Clears the left separator.
    pub const fn clear_left_sep(&mut self) { self.left_sep = None; }
    /// Return a copy with the left separator cleared.
    pub const fn without_left_sep(mut self) -> Self { self.left_sep = None; self }

    /// Sets the right separator to the given character.
    pub const fn set_right_sep(&mut self, sep: char) { self.right_sep = Some(sep); }
    /// Returns a copy with the right separator set to the given character.
    pub const fn with_right_sep(mut self, sep: char) -> Self { self.right_sep = Some(sep); self }
    /// Clears the right separator.
    pub const fn clear_right_sep(&mut self) { self.right_sep = None; }
    /// Return a copy with the right separator cleared.
    pub const fn without_right_sep(mut self) -> Self { self.right_sep = None; self }

    /* grouped lengths */

    /// Sets both grouping lenghts to different values.
    pub const fn set_lens(&mut self, left: u8, right: u8) {
        self.left_len = left; self.right_len = right; }
    /// Returns a copy with both lenghts set to different values.
    pub const fn with_lens(mut self, left: u8, right: u8) -> Self {
        self.left_len = left; self.right_len = right; self }

    /// Sets both grouping lenghts to different values.
    pub const fn set_len(&mut self, len: u8) {
        self.left_len = len; self.right_len = len; }
    /// Returns a copy with both lenghts set to different values.
    pub const fn with_len(mut self, len: u8) -> Self {
        self.left_len = len; self.right_len = len; self }

    /* grouped separators */

    /// Sets both separators to different characters.
    pub const fn set_seps(&mut self, left: char, right: char) {
        self.left_sep = Some(left); self.right_sep = Some(right); }
    /// Returns a copy with both separators set to different characters.
    pub const fn with_seps(mut self, left: char, right: char) -> Self {
        self.left_sep = Some(left); self.right_sep = Some(right); self }

    /// Sets both separators to the same character.
    pub const fn set_sep(&mut self, sep: char) {
        self.left_sep = Some(sep); self.right_sep = Some(sep); }
    /// Returns a copy with both separators set to the same character.
    pub const fn with_sep(mut self, sep: char) -> Self {
        self.left_sep = Some(sep); self.right_sep = Some(sep); self }

    /// Clears both separators.
    pub const fn clear_sep(&mut self) { self.left_sep = None; self.right_sep = None; }
    /// Returns a copy with both separators cleared.
    pub const fn without_sep(mut self) -> Self {
        self.left_sep = None; self.right_sep = None; self }

    /* grouped parts */

    /// Sets left length and separator to the given values.
    pub const fn set_left(&mut self, len: u8, sep: char) {
        self.left_len = len; self.right_sep = Some(sep); }
    /// Returns a copy with the left length and separator set to the given values.
    pub const fn with_left(mut self, len: u8, sep: char) -> Self {
        self.left_len = len; self.right_sep = Some(sep); self }

    /// Sets right length and separator to the given values.
    pub const fn set_right(&mut self, len: u8, sep: char) {
        self.right_len = len; self.right_sep = Some(sep); }
    /// Returns a copy with the right length and separator set to the given values.
    pub const fn with_right(mut self, len: u8, sep: char) -> Self {
        self.right_len = len; self.right_sep = Some(sep); self }
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
