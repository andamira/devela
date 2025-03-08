// devela::text::str::namespace
//
//! [`Str`] namespace.
//

#[cfg(doc)]
use crate::ExtStr;
use crate::{iif, Ascii, InvalidUtf8, Slice};
#[allow(unused_imports, reason = "unsafe")]
#[cfg(feature = "alloc")]
use crate::{Box, _dep::_alloc::str::from_boxed_utf8_unchecked};
#[allow(unused_imports, reason = "unsafe")]
use crate::{
    _core::str::{from_utf8_unchecked, from_utf8_unchecked_mut},
    sf, unwrap,
};
// TODO: IMPROVE:
// - one default, (simd == api if possible)
// - other faster-simdversion if possible (no care about api, errors)
// can't import either or, has to be both, for this module…
use ::core::str::from_utf8_mut;
// crate::_use! {basic::from_utf8} // MAYBE not needed

#[doc = crate::TAG_TEXT!()]
#[doc = crate::TAG_NAMESPACE!()]
/// A string slice namespace.
///
/// See also the [`std::str`] module.
pub struct Str;

impl Str {
    /// Converts a slice of bytes to a string slice.
    ///
    /// See `core::str::`[`from_utf8`].
    //
    // WAIT:[const_methods](https://github.com/rusticstuff/simdutf8/pull/111)
    // /// # Features
    // /// if the `dep_simdutf8` is enabled
    // /// then `simdutf8::compat::`[`from_utf8`] is called instead.
    pub const fn from_utf8(v: &[u8]) -> Result<&str, InvalidUtf8> {
        // #[cfg(not(feature = "dep_simdutf8"))]
        match ::core::str::from_utf8(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(InvalidUtf8::from_utf8_error(e)),
        }
        // #[cfg(feature = "dep_simdutf8")]
        // match ::simdutf8::compat::from_utf8(v) {
        //     Ok(v) => Ok(v),
        //     Err(e) => Err(InvalidUtf8::from_compat_utf8_error(e)),
        // }
    }

    /// Converts a mutable slice of bytes to a mutable string slice.
    ///
    /// See [`from_utf8_mut`].
    // WAIT: [const_str_from_utf8](https://github.com/rust-lang/rust/pull/136668)
    pub fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, InvalidUtf8> {
        match from_utf8_mut(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(InvalidUtf8::from_utf8_error(e)),
        }
    }

    /// Converts a slice of bytes to a string slice without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked(v) }
    }

    /// Converts a mutable slice of bytes to a mutable string slice without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked_mut`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    pub const unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked_mut(v) }
    }

    /// Converts a boxed slice of bytes to a boxed string slice without checking valid UTF-8.
    ///
    /// See [`from_boxed_utf8_unchecked`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg(all(not(feature = "safe_text"), unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", unsafe··))))]
    pub unsafe fn from_boxed_utf8_unchecked(v: Box<[u8]>) -> Box<str> {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_boxed_utf8_unchecked(v) }
    }

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
    /// See also [`ExtStr::new_counter`], which should be faster,
    /// because it uses `copy_from_slice`.
    #[must_use]
    pub const fn repeat_into<'input, const CAP: usize>(
        string: &str,
        n: usize,
        buffer: &'input mut [u8; CAP],
    ) -> &'input str {
        let s_bytes = string.as_bytes();
        let mut index = 0;
        // for _ in 0..n {
        //     for &b in s_bytes {
        //         iif![index == CAP; break];
        //         buffer[index] = b;
        //         index += 1;
        //     }
        // } // const loop:
        let mut outer_count = 0;
        while outer_count < n {
            let mut inner_index = 0;
            while inner_index < s_bytes.len() {
                iif![index == CAP; break];
                buffer[index] = s_bytes[inner_index];
                index += 1;
                inner_index += 1;
            }
            outer_count += 1;
        }

        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return unwrap![ok Str::from_utf8(Slice::range_to(buffer, index))];
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        // SAFETY: since `string` is a valid &str, checks are unneeded.
        sf! { unsafe { Str::from_utf8_unchecked(Slice::range_to(buffer, index)) }}
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
    /// Panics if `buffer.len() < length`, or if `!char.is_ascii()`.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    ///
    /// See also [`ExtStr::new_counter`].
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
                    // buffer[index..(num_len + index)].copy_from_slice(&num_bytes[..num_len]);
                    // Slice::range_mut(buffer, index, num_len + index)
                    //     .copy_from_slice(Slice::range_to(num_bytes, num_len));
                    let mut i = 0;
                    while i < num_len {
                        buffer[index + i] = num_bytes[i];
                        i += 1;
                    }

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
            return unwrap![ok Str::from_utf8(Slice::range_to(buffer, length))];
            #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
            // SAFETY: TODO: since `string` is a valid &str, checks are unneeded.
            sf! { unsafe { Str::from_utf8_unchecked(Slice::range_to(buffer, length)) }}
        }
    }

    /* private utilities */

    /// The cold path that returns an empty string slice.
    #[cold] #[rustfmt::skip]
    pub(crate) const fn new_cold_empty() -> &'static str { "" }
}
