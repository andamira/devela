// devela_base_core::text::char::ascii::digits
//
//! Defines [`AsciiDigits`].
//

#[cfg(test)]
mod tests;

mod docs; // DOC_*
use docs::*;

mod usize;
mod u8;
mod u16;
mod u32;
mod u64;
mod u128;

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
/// # Example
/// ```
/// # use devela_base_core::{AsciiDigits, Slice};
/// let ascii_num = AsciiDigits(12345_u32);
/// assert_eq!(ascii_num.digit_at_power10(100), b'3');
/// assert_eq!(ascii_num.count_digits10(), 5);
/// assert_eq!(ascii_num.digits10(), *b"0000012345");
/// assert_eq!(Slice::trim_leading(&ascii_num.digits10(), b'0'), b"12345");
// IMPROVE example
/// ```
// # Methods
// TODO
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiDigits<T: Copy>(pub T);
