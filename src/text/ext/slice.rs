// devela::text::ext::slice
//
//! trait ExtStr
//

#[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
use core::str::from_utf8;
#[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
use core::str::from_utf8_unchecked;

use crate::{code::iif, text::AsciiChar};

// IMPROVE: use NumToStr
use crate::{mem::Slice, text::Ascii};

// the cold path that returns an empty string slice
#[cold] #[rustfmt::skip]
const fn cold_empty_string() -> &'static str { "" }

// Marker trait to prevent downstream implementations of the `ExtStr` trait.
trait Sealed {}
impl Sealed for str {}

/// Extension trait providing additional methods for [`&str`].
#[cfg_attr(feature = "nightly_doc", doc(notable_trait))]
#[allow(private_bounds, reason = "Sealed")]
pub trait ExtStr: Sealed {
    /// Repeats a string a given number of times into the provided `buffer`.
    /// and returns a reference to the new `&str`.
    /// # Examples
    /// ```
    /// use devela::text::ExtStr;
    ///
    /// let mut buf = [0_u8; 12];
    /// let repeated = "ay".repeat_into(3, &mut buf);
    /// assert_eq![repeated, "ayayay"];
    /// ```
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[must_use]
    fn repeat_into<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str;

    /// Returns a [`&str`] backed by a `buffer`, where you always know each
    /// character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    /// # Examples
    /// ```
    /// use devela::text::{AsciiChar, ExtStr};
    ///
    /// let mut buf = [0; 15];
    /// assert_eq!("2*4*6*8*11*14*", str::new_counter(&mut buf, 14, AsciiChar::Asterisk));
    /// assert_eq!("_3_5_7_9_12_15_", str::new_counter(&mut buf, 15, AsciiChar::LowLine));
    /// ```
    /// # Panics
    /// Panics if `buffer.len() < length`
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[must_use]
    fn new_counter(buffer: &mut [u8], length: usize, separator: AsciiChar) -> &str;
}

impl ExtStr for str {
    fn repeat_into<'input, const CAP: usize>(
        &self,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str {
        let s_bytes = self.as_bytes();

        let mut index = 0;
        for _ in 0..n {
            for &b in s_bytes {
                if index == CAP {
                    break;
                }
                buffer[index] = b;
                index += 1;
            }
        }

        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return from_utf8(&buffer[..index]).unwrap();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: since self is a valid &str, checks are unneeded.
        unsafe {
            from_utf8_unchecked(&buffer[..index])
        }
    }

    fn new_counter(buffer: &mut [u8], length: usize, separator: AsciiChar) -> &str {
        assert![buffer.len() >= length];
        if length == 0 {
            cold_empty_string()
        } else {
            let separator = separator.as_u8();
            let mut index = length - 1; // start writing from the end
            let mut num = length; // the first number to write is the length
            let mut separator_turn = true; // start writing the separator

            let mut num_buf = Ascii(num).digits();
            let mut num_bytes = Slice::trim_leading_bytes(&num_buf, b'0');
            // IMPROVE:BENCH use NumToStr
            // let mut num_buf = [0u8; 22];
            // let mut num_bytes = num.to_bytes_base(10, &mut num_buf);

            let mut num_len = num_bytes.len();

            loop {
                if separator_turn {
                    buffer[index] = separator;
                } else {
                    iif![index > 0; index -= num_len - 1];
                    buffer[index..(num_len + index)].copy_from_slice(&num_bytes[..num_len]);

                    num = index;

                    num_buf = Ascii(num).digits();
                    num_bytes = Slice::trim_leading_bytes(&num_buf, b'0');
                    // IMPROVE: use NumToStr
                    // num_bytes = num.to_bytes_base(10, &mut num_buf);

                    num_len = num_bytes.len();
                }
                iif![index == 0; break; index -= 1];
                separator_turn = !separator_turn;
            }

            #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
            return from_utf8(&buffer[..length]).unwrap();
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: TODO
            return unsafe { from_utf8_unchecked(&buffer[..length]) };
        }
    }
}
