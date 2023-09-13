// devela::str
//
//! String slices, extends [`core::str`].
//

#[cfg(not(feature = "unsafe_str"))]
use core::str::from_utf8;
#[cfg(feature = "unsafe_str")]
use core::str::from_utf8_unchecked;

// Marker trait to prevent downstream implementations of the `StrExt` trait.
impl private::Sealed for str {}
mod private {
    pub trait Sealed {}
}

/// Extension trait providing additional methods for `str`.
pub trait StrExt {
    /// Repeats a string a given number of times into the provided `buffer`.
    /// and returns a reference to the new string.
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

        #[cfg(feature = "unsafe_str")]
        unsafe {
            from_utf8_unchecked(&buffer[..index])
        }
        #[cfg(not(feature = "unsafe_str"))]
        from_utf8(&buffer[..index]).unwrap()
    }
}
