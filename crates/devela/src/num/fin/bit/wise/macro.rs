//
//! Defines [`bitwise!`].
//

#[rustfmt::skip]
#[doc = crate::_tags!(num bit)]
/// Bitwise idioms expanded directly into primitive integer expressions.
#[doc = crate::_doc_meta!{
    location("num/fin", macro bitwise),
}]
/// This macro requires no capability feature or `Bitwise` implementation.
/// It operates on unsigned integer primitives; all bit indices are zero-based.
/// Operations return values without mutating the inputs and evaluate each argument once.
///
/// The extended `Bitwise` namespace and `BitOps` trait require the `bitwise` feature.
#[cfg_attr(feature = "bitwise",
    doc = "See also: [`Bitwise`][crate::Bitwise] and [`BitOps`][crate::BitOps].")]
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
#[cfg_attr(feature = "bitwise", doc = r#"
# Corresponding `Bitwise` methods

The following are **semantic counterparts**, not method calls performed by the macro.
The links use the `u8` implementation; `Bitwise` supports all six unsigned primitives.

| `bitwise!` arm | `Bitwise` method |
|---|---|
| `test BITS; bit N` | [`is_set`](crate::Bitwise::<u8>::is_set) |
| `set BITS; bit N` | [`set`](crate::Bitwise::<u8>::set) |
| `clear BITS; bit N` | [`unset`](crate::Bitwise::<u8>::unset) |
| `toggle BITS; bit N` | [`flip`](crate::Bitwise::<u8>::flip) |
| `any BITS; mask M` | [`is_set_mask`](crate::Bitwise::<u8>::is_set_mask) |
| `mask T; START..=END` | [`mask_range`](crate::Bitwise::<u8>::mask_range) |
| `extract BITS; START..=END` | [`get_value_range`](crate::Bitwise::<u8>::get_value_range) |
| `insert BITS, VALUE; START..=END` | [`set_value_range`](crate::Bitwise::<u8>::set_value_range) |

`all` tests an arbitrary mask and has no direct `Bitwise` equivalent;
[`is_set_range`](crate::Bitwise::<u8>::is_set_range) instead tests a contiguous range.
The three hacker idioms likewise have no direct counterparts.
Unlike the unchecked `Bitwise` range methods, `bitwise!` range arms
panic for invalid ranges in **all** build modes.
"#)]
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
/// assert!(bitwise![any bits; mask 0b1000_0001]);
/// assert!(!bitwise![all bits; mask 0b1000_0001]);
/// assert_eq!(bitwise![mask u8; 3..=6], 0b0111_1000);
/// assert_eq!(bitwise![extract 0b1011_0110_u8; 1..=4], 0b1011);
/// assert_eq!(bitwise![insert 0b1000_0001_u8, 0b101_u8; 1..=3], 0b1000_1011);
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! bitwise· {
    /* Single-bit operations */
    (test $bits:expr; bit $index:expr) => {{
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

    /* Mask predicates */
    (any $bits:expr; mask $mask:expr) => {{
        let (bits, mask) = ($bits, $mask);
        (bits & mask) != 0
    }};
    (all $bits:expr; mask $mask:expr) => {{
        let (bits, mask) = ($bits, $mask);
        (bits & mask) == mask
    }};

    /* Hacker's bit tricks */
    (clear_lowest $bits:expr) => {{
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

    /* Inclusive bit ranges */
    (mask $T:ty; $start:tt ..= $end:tt) => {{
        let (start, end) = (($start) as u32, ($end) as u32);
        assert!(start <= end && end < <$T>::BITS, "bitwise!: invalid bit range");
        let high = if end == <$T>::BITS - 1 {
            !0 as $T
        } else {
            ((1 as $T) << (end + 1)) - 1
        };
        let low = if start == 0 { 0 as $T } else { ((1 as $T) << start) - 1 };
        high & !low
    }};
    (extract $bits:expr; $start:tt ..= $end:tt) => {{
        let (bits, start, end) = ($bits, ($start) as u32, ($end) as u32);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u32;
        assert!(start <= end && end < bit_width, "bitwise!: invalid bit range");
        let width = end - start + 1;
        let mask = if width == bit_width { !0 } else { (1 << width) - 1 };
        (bits >> start) & mask
    }};
    (insert $bits:expr, $value:expr; $start:tt ..= $end:tt) => {{
        let (bits, value, start, end) = ($bits, $value, ($start) as u32, ($end) as u32);
        let bit_width = (::core::mem::size_of_val(&bits) * 8) as u32;
        assert!(start <= end && end < bit_width, "bitwise!: invalid bit range");
        let width = end - start + 1;
        let low_mask = if width == bit_width { !0 } else { (1 << width) - 1 };
        let mask = low_mask << start;
        (bits & !mask) | ((value & low_mask) << start)
    }};
}
#[doc(inline)]
pub use bitwise· as bitwise;

#[cfg(test)]
mod _test {
    use super::bitwise;

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
        assert_eq!(MASK, 0b0011_1100);
        assert_eq!(EXTRACTED, 3);
    }
}
