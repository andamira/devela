// devela::text::fmt::num_to_str
//
//! Defines the [`NumToStr`] trait.
//

#[allow(unused_imports, reason = "±unsafe")]
use crate::_core::str::from_utf8_unchecked;

#[allow(unused_imports)]
#[cfg(not(feature = "dep_simdutf8"))]
use ::core::str::from_utf8;
#[allow(unused_imports)]
#[cfg(feature = "dep_simdutf8")]
use ::simdutf8::basic::from_utf8;

#[cfg(test)]
mod tests;

/// Converts a number into a string representation, storing it into a byte slice.
///
/// # Features
/// It makes use of the `unsafe_str` feature for faster unchecked conversion of
/// the resulting bytes to a string slice, and of the `dep_simdutf8` dependency.
///
#[doc = crate::doc_!(vendor: "numtoa")]
pub trait NumToStr<T> {
    /// Given a base for encoding and a mutable byte slice, write the number
    /// into the byte slice and return the indice where the inner string begins.
    /// The inner string can be extracted by slicing the byte slice from
    /// that indice.
    ///
    /// # Panics
    /// If the supplied buffer is smaller than the number of bytes needed to
    /// write the integer, this will panic. On debug builds, this function will
    /// perform a check on base 10 conversions to ensure that the input array
    /// is large enough to hold the largest possible value in digits.
    ///
    /// # Example
    /// ```
    /// use devela::NumToStr;
    /// use std::io::{self, Write};
    ///
    /// let stdout = io::stdout();
    /// let stdout = &mut io::stdout();
    ///
    /// // Allocate a buffer that will be reused in each iteration.
    /// let mut buffer = [0u8; 20];
    ///
    /// let number = 15325;
    /// let _ = stdout.write(number.to_bytes_base(10, &mut buffer));
    ///
    /// let number = 1241;
    /// let _ = stdout.write(number.to_bytes_base(10, &mut buffer));
    ///
    /// assert_eq!(12345.to_bytes_base(10, &mut buffer), b"12345");
    /// ```
    fn to_bytes_base(self, base: T, string: &mut [u8]) -> &[u8];

    /// Convenience method for quickly getting a string from the input's array buffer.
    fn to_str_base(self, base: T, buf: &mut [u8]) -> &str;
}

// A lookup table to prevent the need for conditional branching
// The value of the remainder of each step will be used as the index
const LOOKUP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// A lookup table optimized for decimal lookups.
// Each two indices represents one possible number.
const DEC_LOOKUP: &[u8; 200] = b"0001020304050607080910111213141516171819\
                                 2021222324252627282930313233343536373839\
                                 4041424344454647484950515253545556575859\
                                 6061626364656667686970717273747576777879\
                                 8081828384858687888990919293949596979899";

macro_rules! base_10 {
    ($number:ident, $index:ident, $string:ident) => {
        // Decode four characters at the same time
        while $number > 9999 {
            let rem = ($number % 10000) as u16;
            let (frst, scnd) = ((rem / 100) * 2, (rem % 100) * 2);
            $string[$index - 3..$index - 1]
                .copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
            $string[$index - 1..$index + 1]
                .copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
            $index = $index.wrapping_sub(4);
            $number /= 10000;
        }

        if $number > 999 {
            let (frst, scnd) = (($number / 100) * 2, ($number % 100) * 2);
            $string[$index - 3..$index - 1]
                .copy_from_slice(&DEC_LOOKUP[frst as usize..frst as usize + 2]);
            $string[$index - 1..$index + 1]
                .copy_from_slice(&DEC_LOOKUP[scnd as usize..scnd as usize + 2]);
            $index = $index.wrapping_sub(4);
        } else if $number > 99 {
            let section = ($number as u16 / 10) * 2;
            $string[$index - 2..$index]
                .copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
            $string[$index] = LOOKUP[($number % 10) as usize];
            $index = $index.wrapping_sub(3);
        } else if $number > 9 {
            $number *= 2;
            $string[$index - 1..$index + 1]
                .copy_from_slice(&DEC_LOOKUP[$number as usize..$number as usize + 2]);
            $index = $index.wrapping_sub(2);
        } else {
            $string[$index] = LOOKUP[$number as usize];
            $index = $index.wrapping_sub(1);
        }
    };
}

macro_rules! impl_primitive {
    ( signed $($t:ty),+ ) => { $( impl_primitive![@signed $t]; )+ };
    ( unsigned $($t:ty),+ ) => { $( impl_primitive![@unsigned $t]; )+ };
    (@signed $t:ty) => {
        impl NumToStr<$t> for $t {
            fn to_bytes_base(mut self, base: $t, string: &mut [u8]) -> &[u8] {
                if cfg!(debug_assertions) {
                    if base == 10 {
                        match size_of::<$t>() {
                            2 => debug_assert![string.len() >= 6,
                                "i16 base 10 conversions require at least 6 bytes"],
                            4 => debug_assert![string.len() >= 11,
                                "i32 base 10 conversions require at least 11 bytes"],
                            8 => debug_assert![string.len() >= 20,
                                "i64 base 10 conversions require at least 20 bytes"],
                            _ => unreachable![],
                        }
                    }
                }
                let mut index = string.len() - 1;
                let mut is_negative = false;
                if self < 0 {
                    is_negative = true;
                    self = match self.checked_abs() {
                        Some(value) => value,
                        None => {
                            let value = <$t>::MAX;
                            string[index] = LOOKUP[((value % base + 1) % base) as usize];
                            index -= 1;
                            value / base + <$t>::from(value % base == base -1)
                        }
                    };
                } else if self == 0 {
                    string[index] = b'0';
                    return &string[index..];
                }
                if base == 10 {
                    // Convert using optimized base 10 algorithm
                    base_10!(self, index, string);
                } else {
                    while self != 0 {
                        let rem = self % base;
                        string[index] = LOOKUP[rem as usize];
                        index = index.wrapping_sub(1);
                        self /= base;
                    }
                }
                if is_negative {
                    string[index] = b'-';
                    index = index.wrapping_sub(1);
                }
                &string[index.wrapping_add(1)..]
            }
            fn to_str_base(self, base: $t, buf: &mut [u8]) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                return from_utf8(self.to_bytes_base(base, buf)).unwrap();

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: the bytes are valid utf-8
                unsafe { from_utf8_unchecked(self.to_bytes_base(base, buf)) }
            }
        }
    };
    (@unsigned $t:ty) => {
        impl NumToStr<$t> for $t {
            fn to_bytes_base(mut self, base: $t, string: &mut [u8]) -> &[u8] {
                // Check if the buffer is large enough and panic on debug builds if it isn't
                if cfg!(debug_assertions) {
                    if base == 10 {
                        match size_of::<$t>() {
                            2 => debug_assert![ string.len() >= 5,
                                "u16 base 10 conversions require at least 5 bytes"],
                            4 => debug_assert![ string.len() >= 10,
                                "u32 base 10 conversions require at least 10 bytes"],
                            8 => debug_assert![ string.len() >= 20,
                                "u64 base 10 conversions require at least 20 bytes"],
                            _ => unreachable![],
                        }
                    }
                }
                let mut index = string.len() - 1;
                if self == 0 {
                    string[index] = b'0';
                    return &string[index..];
                }
                if base == 10 {
                    // Convert using optimized base 10 algorithm
                    base_10!(self, index, string);
                } else {
                    while self != 0 {
                        let rem = self % base;
                        string[index] = LOOKUP[rem as usize];
                        index = index.wrapping_sub(1);
                        self /= base;
                    }
                }
                &string[index.wrapping_add(1)..]
            }
            fn to_str_base(self, base: $t, buf: &mut [u8]) -> &str {
                #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
                return from_utf8(self.to_bytes_base(base, buf)).unwrap();

                #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
                // SAFETY: the bytes are valid utf-8
                unsafe { from_utf8_unchecked(self.to_bytes_base(base, buf)) }
            }
        }
    };
}
impl_primitive!(signed i16, i32, i64, isize);
impl_primitive!(unsigned u16, u32, u64, usize);

impl NumToStr<i8> for i8 {
    fn to_bytes_base(mut self, base: i8, string: &mut [u8]) -> &[u8] {
        if cfg!(debug_assertions) && base == 10 {
            debug_assert!(string.len() >= 4, "i8 conversions need at least 4 bytes");
        }
        let mut index = string.len() - 1;
        let mut is_negative = false;
        #[allow(clippy::comparison_chain)]
        if self < 0 {
            is_negative = true;
            self = if let Some(value) = self.checked_abs() {
                value
            } else {
                let value = <i8>::MAX;
                string[index] = LOOKUP[((value % base + 1) % base) as usize];
                index -= 1;
                value / base + ((value % base == base - 1) as i8)
            };
        } else if self == 0 {
            string[index] = b'0';
            return &string[index..];
        }
        if base == 10 {
            if self > 99 {
                let section = (self / 10) * 2;
                string[index - 2..index]
                    .copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
                string[index] = LOOKUP[(self % 10) as usize];
                index = index.wrapping_sub(3);
            } else if self > 9 {
                self *= 2;
                string[index - 1..index + 1]
                    .copy_from_slice(&DEC_LOOKUP[self as usize..self as usize + 2]);
                index = index.wrapping_sub(2);
            } else {
                string[index] = LOOKUP[self as usize];
                index = index.wrapping_sub(1);
            }
        } else {
            while self != 0 {
                let rem = self % base;
                string[index] = LOOKUP[rem as usize];
                index = index.wrapping_sub(1);
                self /= base;
            }
        }
        if is_negative {
            string[index] = b'-';
            index = index.wrapping_sub(1);
        }
        &string[index.wrapping_add(1)..]
    }

    fn to_str_base(self, base: Self, buf: &mut [u8]) -> &str {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return from_utf8(self.to_bytes_base(base, buf)).unwrap();

        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            from_utf8_unchecked(self.to_bytes_base(base, buf))
        }
    }
}

impl NumToStr<u8> for u8 {
    fn to_bytes_base(mut self, base: u8, string: &mut [u8]) -> &[u8] {
        if cfg!(debug_assertions) && base == 10 {
            debug_assert!(string.len() >= 3, "u8 conversions need at least 3 bytes");
        }
        let mut index = string.len() - 1;
        if self == 0 {
            string[index] = b'0';
            return &string[index..];
        }
        if base == 10 {
            if self > 99 {
                let section = (self / 10) * 2;
                string[index - 2..index]
                    .copy_from_slice(&DEC_LOOKUP[section as usize..section as usize + 2]);
                string[index] = LOOKUP[(self % 10) as usize];
                index = index.wrapping_sub(3);
            } else if self > 9 {
                self *= 2;
                string[index - 1..index + 1]
                    .copy_from_slice(&DEC_LOOKUP[self as usize..self as usize + 2]);
                index = index.wrapping_sub(2);
            } else {
                string[index] = LOOKUP[self as usize];
                index = index.wrapping_sub(1);
            }
        } else {
            while self != 0 {
                let rem = self % base;
                string[index] = LOOKUP[rem as usize];
                index = index.wrapping_sub(1);
                self /= base;
            }
        }
        &string[index.wrapping_add(1)..]
    }

    fn to_str_base(self, base: Self, buf: &mut [u8]) -> &str {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return from_utf8(self.to_bytes_base(base, buf)).unwrap();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: the bytes are valid utf-8
        unsafe {
            from_utf8_unchecked(self.to_bytes_base(base, buf))
        }
    }
}
