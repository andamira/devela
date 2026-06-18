// devela/src/text/str/namespace/writing.rs

#[allow(unused_imports, reason = "±unsafe")]
use crate::unwrap;
use crate::{Digits, Str, is, slice, whilst};

/// # Writing and repetition methods
impl Str {
    /// Repeats a `string` a given number of times into the provided `buffer`.
    /// and returns a reference to the new `&str`.
    /// # Examples
    /// ```
    /// # use devela::Str;
    /// let mut buf = [0_u8; 12];
    /// let repeated = Str::repeat_into("ay", 3, &mut buf);
    /// assert_eq![repeated, "ayayay"];
    /// ```
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// See also [`StrExt::repeat_into`], which should be faster, because it uses `copy_from_slice`.
    #[doc = crate::doclink!(custom devela "[`StrExt::repeat_into`]"
        "text/str/trait.StrExt.html#method.repeat_into")]
    #[must_use]
    pub const fn repeat_into<'input, const CAP: usize>(
        string: &str,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str {
        Self::repeat_into_slice(string, n, buffer)
    }

    /// Repeats `string` up to `n` times into `buffer`,
    /// stopping early if it does not fit,
    /// and returns the written prefix as `&str`.
    ///
    /// Like [`repeat_into`][Self::repeat_into], but accepts a dynamically sized buffer.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[must_use]
    pub const fn repeat_into_slice<'input>(
        string: &str,
        n: usize,
        buffer: &'input mut [u8],
    ) -> &'input str {
        let s_bytes = string.as_bytes();
        let s_len = s_bytes.len();
        let mut index = 0;
        whilst! { count in 0..n; {
            is![index + s_len > buffer.len(), break];
            // buffer[index..index + s_len].copy_from_slice(s_bytes);
            slice![mut buffer, index, ..index + s_len].copy_from_slice(s_bytes);
            index += s_len;
        }}
        cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
            // SAFETY: since `string` is a valid &str, checks are unneeded.
            unsafe { Str::from_utf8_unchecked(slice![buffer, ..index]) }
        } _ => { unwrap![ok Str::from_utf8(slice![buffer, ..index])] }}
    }

    /// Returns a [`&str`] backed by a `buffer`, where you always know each
    /// character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    /// # Examples
    /// ```
    /// # use devela::Str;
    /// let mut buf = [0; 15];
    /// assert_eq!("2*4*6*8*11*14*", Str::new_counter(&mut buf, 14, '*'));
    /// assert_eq!("_3_5_7_9_12_15_", Str::new_counter(&mut buf, 15, '_'));
    /// ```
    /// # Panics
    /// Panics if `buffer.len() < length`, or if `!separator.is_ascii()`.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// See also [`StrExt::new_counter`].
    #[doc = crate::doclink!(custom devela "[`StrExt::new_counter`]"
        "text/str/trait.StrExt.html#method.new_counter")]
    ///
    /// [0]: https://www.satisfice.com/blog/archives/22
    pub const fn new_counter(buffer: &mut [u8], length: usize, separator: char) -> &str {
        assert![buffer.len() >= length];
        assert![separator.is_ascii()];
        if length == 0 {
            Str::new_cold_empty()
        } else {
            let separator = separator as u8;
            let mut index = length - 1; // start writing from the end
            let mut num = length; // the first number to write is the length
            let mut separator_turn = true; // start writing the separator
            let mut digits = Digits(num).count_digits10() as usize;
            loop {
                if separator_turn {
                    buffer[index] = separator;
                } else {
                    let start = index + 1 - digits;
                    let _ = Digits(num).write_digits10(buffer, start);
                    index = start;
                    num = index;
                    digits = Digits(num).count_digits10() as usize;
                }
                is![index == 0, break, index -= 1];
                separator_turn = !separator_turn;
            }
            cfg_select! { all(feature = "unsafe_str", not(feature = "safe_text")) => {
                // SAFETY: only ASCII bytes are written
                unsafe { Str::from_utf8_unchecked(slice![buffer, ..length]) }
            } _ => { unwrap![ok Str::from_utf8(slice![buffer, ..length])] }}
        }
    }

    /* private utilities */

    #[doc(hidden)]
    /// The cold path that returns an empty string slice.
    #[cold] #[rustfmt::skip]
    pub const fn new_cold_empty() -> &'static str { "" }
}
