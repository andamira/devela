// devela_base_core::text::char::digits
//
//! Defines [`Digits`].
//

#[cfg(test)]
mod tests;

mod docs; // DOC_*
use docs::*;

mod u8;
mod u16;
mod u32;
mod u64;
mod u128;
mod usize;

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Provides ASCII digit operations and conversions for unsigned integer primitives.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// Enables efficient ASCII digit extraction, counting, and conversion
/// for unsigned integer types. All operations are `const` and designed for
/// performance-critical scenarios like number formatting.
///
/// It converts **numbers → digits** for display/formatting.
///
/// # Methods
///
/// Most methods are implemented for all the unsigned integer primitives:
/// [u8](#impl-Digits<u8>),
/// [u16](#impl-Digits<u16>),
/// [u32](#impl-Digits<u32>),
/// [u64](#impl-Digits<u64>),
/// [u128](#impl-Digits<u128>),
/// [usize](#impl-Digits<usize>).
///
/// Common methods, `u8` version links:
/// - [count_digits10](#method.count_digits10),
///   [count_digits16](#method.count_digits16).
/// - [digit_at_index10](#method.digit_at_index10)
///  ([*checked*](#method.digit_at_index10_checked)),
///   [digit_at_index16](#method.digit_at_index16)
///  ([*checked*](#method.digit_at_index16_checked)).
/// - [digit_value_at_index10](#method.digit_value_at_index10)
///  ([*checked*](#method.digit_value_at_index10_checked)),
/// - [digit_value_at_index16](#method.digit_value_at_index16)
///  ([*checked*](#method.digit_value_at_index16_checked)),
/// - [digits10](#method.digits10),
///   [digits16](#method.digits16).
/// - [digits10_str](#method.digits10_str),
///   [digits16_str](#method.digits16_str).
///
/// Exclusive for `u8`:
/// - [digits10_1](#method.digits10_3),
/// - [digits10_2](#method.digits10_2),
///   [digits16_1](#method.digits16_1).
///
/// Exclusive for `u16`:
/// - [digits10_3](#method.digits10_3),
///   [digits10_4](#method.digits10_4).
///
/// # Example
/// ```
/// # use devela_base_core::{Digits, Slice};
/// let dec = Digits(12345_u32);
/// assert_eq!(dec.digit_at_index10(0), b'5');
/// assert_eq!(dec.digit_at_index10(4), b'1');
/// assert_eq!(dec.digit_value_at_index10(2), 3);
/// assert_eq!(dec.count_digits10(), 5);
/// assert_eq!(dec.digits10(), *b"0000012345");
/// assert_eq!(Slice::trim_leading(&dec.digits10(), b'0'), b"12345");
///
/// let hex = Digits(0x89ABC_u32);
/// assert_eq!(hex.digit_at_index16(1), b'B');
/// assert_eq!(hex.digit_value_at_index16(1), 11);
/// assert_eq!(hex.count_digits16(), 5);
/// assert_eq!(hex.digits16(), *b"00089ABC");
/// assert_eq!(Slice::trim_leading(&hex.digits16(), b'0'), b"89ABC");
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digits<T: Copy>(pub T);
