// devela::str::ext
//
//!
//

#[cfg(not(feature = "unsafe_str"))]
use core::str::from_utf8;
#[cfg(feature = "unsafe_str")]
use core::str::from_utf8_unchecked;

#[cfg(all(feature = "ascii", feature = "unsafe_fmt"))]
use crate::fmt::IntBuf;
#[cfg(all(feature = "ascii", not(feature = "unsafe_fmt")))]
use crate::{ascii::ascii_usize_digits, slice::slice_trim_leading_bytes};
#[cfg(feature = "ascii")]
use crate::{ascii::AsciiChar, codegen::iif};

// Marker trait to prevent downstream implementations of the `StrExt` trait.
impl private::Sealed for str {}
mod private {
    pub trait Sealed {}
}

/// Extension trait providing additional methods for `str`.
pub trait StrExt {
    /// Repeats a string a given number of times into the provided `buffer`.
    /// and returns a reference to the new `&str`.
    ///
    /// # Examples
    /// ```
    /// use devela::str::StrExt;
    ///
    /// let mut buf = [0_u8; 12];
    /// let repeated = "ay".repeat_into(3, &mut buf);
    /// assert_eq![repeated, "ayayay"];
    /// ```
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
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
    ///
    /// # Examples
    /// ```
    /// use devela::{ascii::AsciiChar, str::StrExt};
    ///
    /// let mut buf = [0; 15];
    /// assert_eq!("2*4*6*8*11*14*", str::new_counter(&mut buf, 14, AsciiChar::Asterisk));
    /// assert_eq!("_3_5_7_9_12_15_", str::new_counter(&mut buf, 15, AsciiChar::LowLine));
    /// ```
    /// # Panics
    /// Panics if `buffer.len() < length`
    ///
    /// # Features
    /// Makes use of the `unsafe_str` and `unsafe_fmt` features if enabled.
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    #[cfg(feature = "ascii")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "ascii")))]
    fn new_counter(buffer: &mut [u8], length: usize, separator: AsciiChar) -> &str;
}

impl StrExt for str {
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

        // SAFETY: since self is a valid &str, checks are unneeded.
        #[cfg(feature = "unsafe_str")]
        unsafe {
            from_utf8_unchecked(&buffer[..index])
        }
        #[cfg(not(feature = "unsafe_str"))]
        from_utf8(&buffer[..index]).unwrap()
    }

    #[cfg(feature = "ascii")]
    fn new_counter(buffer: &mut [u8], length: usize, separator: AsciiChar) -> &str {
        assert![buffer.len() >= length];
        if length == 0 {
            cold_empty_string()
        } else {
            let separator = separator.as_u8();
            let mut index = length - 1; // start writing from the end
            let mut num = length; // the first number to write is the length
            let mut separator_turn = true; // start writing the separator

            // safe:
            #[cfg(not(feature = "unsafe_fmt"))]
            let mut num_buf = ascii_usize_digits(num);
            #[cfg(not(feature = "unsafe_fmt"))]
            let mut num_bytes = slice_trim_leading_bytes(&num_buf, b'0');

            // unsafe:
            #[cfg(feature = "unsafe_fmt")]
            let mut num_buf = IntBuf::new();
            #[cfg(feature = "unsafe_fmt")]
            let mut num_bytes = num_buf.to_bytes(num);

            let mut num_len = num_bytes.len();

            loop {
                if separator_turn {
                    buffer[index] = separator;
                } else {
                    iif![index > 0; index -= num_len - 1];
                    buffer[index..(num_len + index)].copy_from_slice(&num_bytes[..num_len]);

                    num = index;

                    // safe
                    #[cfg(not(feature = "unsafe_fmt"))]
                    {
                        num_buf = ascii_usize_digits(num);
                        num_bytes = slice_trim_leading_bytes(&num_buf, b'0');
                    }
                    // unsafe
                    #[cfg(feature = "unsafe_fmt")]
                    {
                        num_bytes = num_buf.to_bytes(num);
                    }

                    num_len = num_bytes.len();
                }
                iif![index == 0; break; index -= 1];
                separator_turn = !separator_turn;
            }

            #[cfg(feature = "unsafe_string")]
            return unsafe { from_utf8_unchecked(&buffer[..length]) };
            #[cfg(not(feature = "unsafe_string"))]
            return from_utf8(&buffer[..length]).unwrap();
        }
    }
}

// the cold path that returns an empty string slice
#[cold]
#[inline]
#[cfg(feature = "ascii")]
fn cold_empty_string() -> &'static str {
    ""
}
