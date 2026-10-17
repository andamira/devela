//
//! Defines [`bitwise!`].
//
// TOC
// - bitwise!
// - __bitwise_backend!
// - __bitwise_inline!
// - __bitwise_methods!
// - _test/

#[cfg(all(doc, feature = "bitwise"))]
use crate::{BitOps, Bitwise};

#[rustfmt::skip]
#[doc = crate::_tags!(num bit)]
/// A mnemonic vocabulary of bitwise operations, with selectable range backends.
#[doc = crate::_doc_meta!{
    location("num/fin", macro bitwise),
}]
/// This macro requires no capability feature or `Bitwise` implementation.
/// It operates on unsigned integer primitives; all bit indices are zero-based.
/// Operations return values without mutating the inputs and evaluate each argument once.
///
/// Range operations use an inline backend by default, or delegate to `Bitwise`
/// methods when the `bitwise` feature is enabled. The implementation can also be
/// selected per invocation, without changing the operation's semantics:
/// - `bitwise![@inline mask u8; 2..=5]` forces primitive expressions.
/// - `bitwise![@methods mask u8; 2..=5]` requires the `bitwise` feature.
///
/// The same prefixes work with `extract` and `insert`. Selection and validation
/// take place within the macro; neither backend is required to validate ranges.
///
/// The extended `Bitwise` namespace and `BitOps` trait require the `bitwise` feature.
///
/// # Single bits
/// - `test`, `set`, `clear`, `toggle` interpret the last argument as a bit *index*.
///
/// # Masks
/// - `any`, `all` interpret the last argument as a bit *mask*.
/// - An empty mask makes `any` false and `all` true.
///
/// # Hacker's bit tricks
/// - `clear_lowest`: remove the lowest set bit (zero stays zero).
/// - `isolate_lowest`: isolate the lowest set bit (zero stays zero).
/// - `isolate_trailing`: isolate the lowest *unset* bit (all ones yields zero).
///
/// # Inclusive ranges
/// - `mask T; START..=END`: construct a `T` mask covering the range.
/// - `extract BITS; START..=END`: shift the selected bits down to bit zero.
/// - `insert BITS, VALUE; START..=END`: replace the region, truncating `VALUE`.
///
/// For range forms, both bounds must be within the integer width and
/// `START <= END`. Invalid ranges panic in all build modes. Full-width
/// ranges are supported without an overflowing shift.
///
/// Single-bit forms follow Rust's ordinary shift behavior for invalid indices
/// (debug overflow checks; do not pass an out-of-range index).
///
/// Range bounds must be a single token tree (a literal, identifier, or a
/// parenthesized expression), matching the syntax of `set!` and `bitfield!`.
///
/// # Corresponding `Bitwise` methods
///
/// These are **semantic counterparts**. With the `bitwise` feature, the three range
/// operations delegate to these methods by default, but `@inline` can override that.
/// The links use the `u8` implementation; all six unsigned primitives are supported.
///
/// | `bitwise!` arm | `Bitwise` method |
/// |---|---|
/// | `test BITS; bit N` | [`is_set`](Bitwise::<u8>::is_set) |
/// | `set BITS; bit N` | [`set`](Bitwise::<u8>::set) |
/// | `clear BITS; bit N` | [`unset`](Bitwise::<u8>::unset) |
/// | `toggle BITS; bit N` | [`flip`](Bitwise::<u8>::flip) |
/// | `any BITS; mask M` | [`is_set_mask`](Bitwise::<u8>::is_set_mask) |
/// | `mask T; START..=END` | [`mask_range`](Bitwise::<u8>::mask_range) |
/// | `extract BITS; START..=END` | [`get_value_range`](Bitwise::<u8>::get_value_range) |
/// | `insert BITS, VALUE; START..=END` | [`set_value_range`](Bitwise::<u8>::set_value_range) |
///
/// `all` checks an arbitrary mask and has no direct `Bitwise` equivalent;
/// [`is_set_range`](Bitwise::<u8>::is_set_range) instead checks a contiguous range.
/// The three hacker idioms likewise have no direct counterparts.
/// Unlike the unchecked `Bitwise` range methods, `bitwise!` range arms
/// panic for invalid ranges in **all** build modes.
///
/// # Examples
/// ```
/// use devela::bitwise;
///
/// let bits = 0b1010_0000_u8;
/// assert!(bitwise![test bits; bit 7]);
/// assert_eq!(bitwise![set bits; bit 0], 0b1010_0001);
/// assert_eq!(bitwise![clear bits; bit 7], 0b0010_0000);
/// assert_eq!(bitwise![toggle bits; bit 5], 0b1000_0000);
///
/// assert!(bitwise![any bits; mask 0b1000_0001]);
/// assert!(!bitwise![all bits; mask 0b1000_0001]);
///
/// assert_eq!(bitwise![mask u8; 3..=6], 0b0111_1000);
/// assert_eq!(bitwise![extract 0b1011_0110_u8; 1..=4], 0b1011);
/// assert_eq!(bitwise![insert 0b1000_0001_u8, 0b101_u8; 1..=3], 0b1000_1011);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! bitwise· {
    (

    /* Single-bit operations */

     test $bits:expr; bit $index:expr) => {{
        let (bits, index) = ($bits, $index);
        (bits & (1 << index)) != 0
    }};
    (set $bits:expr; bit $index:expr) => {{
        let (bits, index) = ($bits, $index);
        bits | (1 << index)
    }};
    (clear $bits:expr; bit $index:expr) => {{
        let (bits, index) = ($bits, $index);
        bits & !(1 << index)
    }};
    (toggle $bits:expr; bit $index:expr) => {{
        let (bits, index) = ($bits, $index);
        bits ^ (1 << index)
    }};
    (

    /* Mask predicates */

     any $bits:expr; mask $mask:expr) => {{
        let (bits, mask) = ($bits, $mask);
        (bits & mask) != 0
    }};
    (all $bits:expr; mask $mask:expr) => {{
        let (bits, mask) = ($bits, $mask);
        (bits & mask) == mask
    }};
    (

    /* Hacker's bit tricks */

     clear_lowest $bits:expr) => {{
        let bits = $bits;
        bits & bits.wrapping_sub(1)
    }};
    (isolate_lowest $bits:expr) => {{
        let bits = $bits;
        bits & bits.wrapping_neg()
    }};
    (isolate_trailing $bits:expr) => {{
        let bits = $bits;
        !bits & bits.wrapping_add(1)
    }};
    (

    /* Inclusive bit ranges: feature-selected default. */

     mask $T:ty; $start:tt ..= $end:tt) => {
        $crate::__bitwise!(% dispatch __bitwise_backend; mask $T; $start ..= $end)
    };
    (extract $bits:expr; $start:tt ..= $end:tt) => {
        $crate::__bitwise!(% dispatch __bitwise_backend; extract $bits; $start ..= $end)
    };
    (insert $bits:expr, $value:expr; $start:tt ..= $end:tt) => {
        $crate::__bitwise!(% dispatch __bitwise_backend; insert $bits, $value; $start ..= $end)
    };
    (

    /* Range backend selection: available per invocation. */

     @inline $($range:tt)+) => {
            $crate::__bitwise!(% dispatch __bitwise_inline; $($range)+)
    };
    (@methods $($range:tt)+) => {
        $crate::__bitwise!(% dispatch __bitwise_methods; $($range)+)
    };
}
#[doc(inline)]
pub use bitwise· as bitwise;

// Internal API
#[doc(hidden)]
#[macro_export]
macro_rules! __bitwise· {
    /* Shared validation and u32 conversion, independent of the backend. */
    (% dispatch $backend:ident; mask $T:ty; $start:tt ..= $end:tt) => {{
        let (start, end) = (($start) as u128, ($end) as u128);
        assert!(start <= end && end < <$T>::BITS as u128, "bitwise!: invalid bit range");
        $crate::$backend!(mask $T; start as u32, end as u32)
    }};
    (% dispatch $backend:ident; extract $bits:expr; $start:tt ..= $end:tt) => {{
        let (bits, start, end) = ($bits, ($start) as u128, ($end) as u128);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u128;
        assert!(start <= end && end < bit_width, "bitwise!: invalid bit range");
        $crate::$backend!(extract bits; start as u32, end as u32)
    }};
    (% dispatch $backend:ident; insert $bits:expr, $value:expr; $start:tt ..= $end:tt) => {{
        let (bits, value, start, end) =
            ($bits, $value, ($start) as u128, ($end) as u128);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u128;
        assert!(start <= end && end < bit_width, "bitwise!: invalid bit range");
        $crate::$backend!(insert bits, value; start as u32, end as u32)
    }};
}
#[doc(hidden)]
pub use __bitwise· as __bitwise;

#[doc(hidden)]
#[cfg(not(feature = "bitwise"))]
pub use __bitwise_inline as __bitwise_backend;
#[doc(hidden)]
#[cfg(feature = "bitwise")]
pub use __bitwise_methods as __bitwise_backend;

#[doc(hidden)]
#[macro_export]
macro_rules! __bitwise_inline· {
    (mask $T:ty; $start:expr, $end:expr) => {{
        let (start, end): (u32, u32) = ($start, $end);
        let high = if end == <$T>::BITS - 1 {
            !0 as $T
        } else {
            (1 as $T).wrapping_shl(end.wrapping_add(1)).wrapping_sub(1)
        };
        let low = if start == 0 { 0 as $T } else { (1 as $T).wrapping_shl(start).wrapping_sub(1) };
        high & !low
    }};
    (extract $bits:expr; $start:expr, $end:expr) => {{
        let (bits, start, end) = ($bits, $start, $end);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u32;
        let width = end.wrapping_sub(start).wrapping_add(1);
        let all = bits | !bits;
        let mask = if width == bit_width { all } else { !all.wrapping_shl(width) };
        bits.wrapping_shr(start) & mask
    }};
    (insert $bits:expr, $value:expr; $start:expr, $end:expr) => {{
        let (bits, value, start, end) = ($bits, $value, $start, $end);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u32;
        let width = end.wrapping_sub(start).wrapping_add(1);
        let all = bits | !bits;
        let low_mask = if width == bit_width { all } else { !all.wrapping_shl(width) };
        let mask = low_mask.wrapping_shl(start);
        (bits & !mask) | ((value & low_mask).wrapping_shl(start))
    }};
}
#[doc(hidden)]
pub use __bitwise_inline· as __bitwise_inline;

#[doc(hidden)]
#[cfg(feature = "bitwise")]
#[macro_export]
macro_rules! __bitwise_methods· {
    (mask $T:ty; $start:expr, $end:expr) => {
        $crate::Bitwise::<$T>::mask_range($start, $end).0
    };
    (extract $bits:expr; $start:expr, $end:expr) => {
        $crate::Bitwise($bits).get_value_range($start, $end).0
    };
    (insert $bits:expr, $value:expr; $start:expr, $end:expr) => {
        $crate::Bitwise($bits).set_value_range($value, $start, $end).0
    };
}
#[doc(hidden)]
#[cfg(not(feature = "bitwise"))]
#[macro_export]
macro_rules! __bitwise_methods· {
    ($($tt:tt)*) => {
        compile_error!("bitwise![@methods ...] requires the `bitwise` feature")
    };
}
#[doc(hidden)]
pub use __bitwise_methods· as __bitwise_methods;

#[cfg(test)]
mod _test {
    use super::*;

    #[test]
    fn basic_bits_and_masks() {
        let bits = 0b1010_0000_u8;
        assert!(bitwise![test bits; bit 7]);
        assert!(!bitwise![test bits; bit 0]);
        assert_eq!(bitwise![set bits; bit 0], 0b1010_0001);
        assert_eq!(bitwise![clear bits; bit 7], 0b0010_0000);
        assert_eq!(bitwise![toggle bits; bit 5], 0b1000_0000);
        assert!(bitwise![any bits; mask 0b1000_0001]);
        assert!(!bitwise![all bits; mask 0b1000_0001]);
        assert!(!bitwise![any bits; mask 0]);
        assert!(bitwise![all bits; mask 0]);
    }
    #[test]
    fn hacker_idioms() {
        let bits = 0b1011_0000_u8;
        assert_eq!(bitwise![clear_lowest bits], 0b1010_0000);
        assert_eq!(bitwise![isolate_lowest bits], 0b0001_0000);
        assert_eq!(bitwise![isolate_trailing bits], 0b0000_0001);
        assert_eq!(bitwise![clear_lowest 0_u8], 0);
        assert_eq!(bitwise![isolate_lowest 0_u8], 0);
        assert_eq!(bitwise![isolate_trailing u8::MAX], 0);
    }
    #[test]
    #[cfg(feature = "bitwise")]
    fn range_backends_are_equivalent() {
        macro_rules! compare {
            ($T:ty) => {{
                let end = <$T>::BITS - 1;
                let bits = !0 as $T;
                let value = !0 as $T;
                let pattern = 0b1010_0110 as $T;
                let replacement = 0b1101 as $T;
                assert_eq!(
                    __bitwise_inline!(mask $T; 0_u32, end),
                    __bitwise_methods!(mask $T; 0_u32, end),
                );
                assert_eq!(
                    __bitwise_inline!(extract bits; 0_u32, end),
                    __bitwise_methods!(extract bits; 0_u32, end),
                );
                assert_eq!(
                    __bitwise_inline!(insert 0 as $T, value; 0_u32, end),
                    __bitwise_methods!(insert 0 as $T, value; 0_u32, end),
                );
                assert_eq!(
                    __bitwise_inline!(mask $T; 1_u32, 3_u32),
                    __bitwise_methods!(mask $T; 1_u32, 3_u32),
                );
                assert_eq!(
                    __bitwise_inline!(extract bits; 1_u32, 3_u32),
                    __bitwise_methods!(extract bits; 1_u32, 3_u32),
                );
                assert_eq!(
                    __bitwise_inline!(insert 0 as $T, value; 1_u32, 3_u32),
                    __bitwise_methods!(insert 0 as $T, value; 1_u32, 3_u32),
                );
                assert_eq!(
                    __bitwise_inline!(extract pattern; 1_u32, 3_u32),
                    __bitwise_methods!(extract pattern; 1_u32, 3_u32),
                );
                assert_eq!(
                    __bitwise_inline!(insert pattern, replacement; 1_u32, 3_u32),
                    __bitwise_methods!(insert pattern, replacement; 1_u32, 3_u32),
                );
            }};
        }
        compare!(u8);
        compare!(u16);
        compare!(u32);
        compare!(u64);
        compare!(u128);
        compare!(usize);
    }
    #[test]
    fn forced_inline_ranges() {
        assert_eq!(bitwise![@inline mask u8; 3..=6], 0b0111_1000);
        assert_eq!(bitwise![@inline extract 0b1011_0110_u8; 1..=4], 0b1011);
        assert_eq!(bitwise![@inline insert 0_u8, 0b101_u8; 1..=3], 0b1010);
    }
    #[test]
    #[cfg(feature = "bitwise")]
    fn forced_methods_ranges() {
        assert_eq!(bitwise![@methods mask u8; 3..=6], 0b0111_1000);
        assert_eq!(bitwise![@methods extract 0b1011_0110_u8; 1..=4], 0b1011);
        assert_eq!(bitwise![@methods insert 0_u8, 0b101_u8; 1..=3], 0b1010);
    }
    #[test]
    fn inclusive_ranges() {
        assert_eq!(bitwise![mask u8; 3..=6], 0b0111_1000);
        assert_eq!(bitwise![mask u8; 0..=7], u8::MAX);
        assert_eq!(bitwise![mask u16; 0..=15], u16::MAX);
        assert_eq!(bitwise![mask u32; 0..=31], u32::MAX);
        assert_eq!(bitwise![mask u64; 0..=63], u64::MAX);
        assert_eq!(bitwise![mask u128; 0..=127], u128::MAX);
        assert_eq!(bitwise![mask usize; 0..=(usize::BITS - 1)], usize::MAX);
        assert_eq!(bitwise![extract 0b1011_0110_u8; 1..=4], 0b1011);
        assert_eq!(bitwise![extract u8::MAX; 0..=7], u8::MAX);
        assert_eq!(bitwise![insert 0b1000_0001_u8, 0b101_u8; 1..=3], 0b1000_1011);
        assert_eq!(bitwise![insert 0_u8, u8::MAX; 3..=6], 0b0111_1000);
        assert_eq!(bitwise![insert 0_u128, u128::MAX; 0..=127], u128::MAX);
    }
    #[test]
    #[should_panic]
    fn invalid_range() {
        let _ = bitwise![mask u8; 2..=8];
    }
    #[test]
    #[should_panic(expected = "invalid bit range")]
    fn invalid_extract_range() {
        let _ = bitwise![@inline extract 0_u8; 4..=2];
    }
    #[test]
    #[should_panic(expected = "invalid bit range")]
    fn invalid_insert_range() {
        let _ = bitwise![@inline insert 0_u8, 0_u8; 1..=8];
    }
    #[test]
    #[should_panic(expected = "invalid bit range")]
    fn invalid_wide_index() {
        let _ = bitwise![@inline mask u8; 0..=(u64::from(u32::MAX) + 1)];
    }
    #[test]
    fn each_input_evaluated_once() {
        let mut calls = 0;
        let bits = bitwise![insert { calls += 1; 0_u8 }, { calls += 1; 3_u8 }; 2..=3];
        assert_eq!(bits, 0b1100);
        assert_eq!(calls, 2);
    }
    #[test]
    fn constant_evaluation() {
        const MASK: u8 = bitwise![mask u8; 2..=5];
        const VALUE: u8 = bitwise![insert 0_u8, 3_u8; 2..=5];
        const EXTRACTED: u8 = bitwise![extract VALUE; 2..=5];
        const INLINE: u8 = bitwise![@inline mask u8; 2..=5];
        assert_eq!(INLINE, MASK);
        assert_eq!(MASK, 0b0011_1100);
        assert_eq!(EXTRACTED, 3);
    }
}
