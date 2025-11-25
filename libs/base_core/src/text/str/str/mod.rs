// devela_base_core::text::str::str
//
//! [`Str`] namespace.
//

mod range;
mod take;
mod split;

use crate::{CharIter, Digits, InvalidUtf8, Slice, is, slice};

#[allow(unused_imports, reason = "±unsafe")]
use {
    crate::{sf, unwrap},
    ::core::str::{from_utf8_unchecked, from_utf8_unchecked_mut},
};

// TODO: IMPROVE:
// - one default, (simd == api if possible)
// - other faster-simdversion if possible (no care about api, errors)
// can't import either or, has to be both, for this module…
use ::core::str::from_utf8_mut;
// crate::_use! {basic::from_utf8} // MAYBE not needed

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// A string slice namespace.
///
#[doc = crate::_doc!(location: "text/str")]
///
/// # Methods
/// - [methods namespaced from `core::str`](#corestr-namespaced-methods)
/// - [extra methods](#extra-methods)
///
/// - [`range*` API methods](#range-api-methods-for-returning-substrings):<br/>
///   - [**range**](#method.range)
///    ([*checked*](#method.range_checked),
///     [_**mut**_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked)),                     ≈ `&str[start..end]`
///   - [**range_inclusive**](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [_**mut**_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked)),           ≈ `&str[start..=end]`
///   - [**range_from**](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [_**mut**_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked)),                ≈ `&str[start..]`
///   - [**range_to**](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [_**mut**_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked)),                  ≈ `&str[..end]`
///   - [**range_to_inclusive**](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [_**mut**_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked)).        ≈ `&str[..=end]`
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):<br/>
///   - [**take_first**](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [_**mut**_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked)),                ≈ `&str[..n]`
///   - [**take_last**](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [_**mut**_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked)),                 ≈ `&str[len - n..]`
///   - [**take_omit_last**](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [_**mut**_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked)).            ≈ `&str[..len - n]`
///
/// - [`*split*` API methods](#split-api-methods-for-returning-substrings):<br/>
///   - [**lsplit**](#method.lsplit)
///    ([*checked*](#method.lsplit_checked),
///    [**mut**](#method.lsplit_mut),
///    [*mut_checked*](#method.lsplit_mut_checked)),
///   - [**rsplit**](#method.rsplit)
///    ([*checked*](#method.rsplit_checked),
///    [*mut*](#method.rsplit_mut),
///    [*mut_checked*](#method.rsplit_mut_checked)),
///   - [**msplit_left**](#method.msplit_left)
///    ([*checked*](#method.msplit_left_checked),
///    [*mut*](#method.msplit_left_mut),
///    [*mut_checked*](#method.msplit_left_mut_checked)),
///   - [**msplit_right**](#method.msplit_right)
///    ([*checked*](#method.msplit_right_checked),
///    [*mut*](#method.msplit_right_mut).
///    [*mut_checked*](#method.msplit_right_mut_checked)).
///
/// See also the [`core::str`] module.
#[derive(Debug)]
pub struct Str;

/// # `core::str` namespaced methods.
impl Str {
    /// Converts a slice of bytes to a string slice.
    ///
    /// See `core::str::`[`from_utf8`][::core::str::from_utf8].
    //
    // WAIT: [const_methods](https://github.com/rusticstuff/simdutf8/pull/111)
    // /// # Features
    // /// if the `dep_simdutf8` is enabled
    // /// then `simdutf8::compat::`[`from_utf8`] is called instead.
    #[allow(rustdoc::broken_intra_doc_links, reason = "±unsafe")]
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

    /// Converts an exclusive slice of bytes to an exclusive string slice.
    ///
    /// See [`from_utf8_mut`].
    pub const fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, InvalidUtf8> {
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
    #[cfg(all(not(base_safe_text), unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked(v) }
    }

    /// Converts an exclusive slice of bytes to an exclusive string slice
    /// without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked_mut`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(base_safe_text), unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    pub const unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked_mut(v) }
    }

    /// Converts a byte slice known to be valid UTF-8 to a string.
    ///
    /// # Features
    /// It can use unsafe internally to skip checks.
    ///
    /// # Safety
    /// `bytes` must contain valid utf-8.
    ///
    /// This is intended to be called by macros, where the feature-bounds
    /// would be tested against the user code.
    ///
    /// This is why it is not marked as unsafe; the macro is the one to enforce safety.
    ///
    /// # Used by
    /// [`join!`][crate::join].
    #[doc(hidden)] #[rustfmt::skip]
    pub const fn __utf8_bytes_to_str(bytes: &[u8]) -> &str {
        #[cfg(any(base_safe_text, not(unsafe··)))]
        { unwrap![ok ::core::str::from_utf8(bytes)] }
        #[cfg(all(not(base_safe_text), unsafe··))]
        unsafe { ::core::str::from_utf8_unchecked(bytes) }
    }
}

/// # extra methods.
impl Str {
    /// Returns an iterator over the Unicode scalars.
    #[inline(always)]
    pub const fn chars(string: &str) -> CharIter<'_, &str> {
        CharIter::<&str>::new(string)
    }

    /// Returns the total number of Unicode scalars.
    #[must_use]
    #[inline(always)]
    pub const fn char_count(string: &str) -> usize {
        CharIter::<&str>::new(string).count()
    }

    /// Repeats a `string` a given number of times into the provided `buffer`.
    /// and returns a reference to the new `&str`.
    /// # Examples
    /// ```
    /// # use devela_base_core::Str;
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
        let s_bytes = string.as_bytes();
        let mut index = 0;
        // for _ in 0..n {
        //     for &b in s_bytes {
        //         is![index == CAP; break];
        //         buffer[index] = b;
        //         index += 1;
        //     }
        // } // const loop:
        let mut outer_count = 0;
        while outer_count < n {
            let mut inner_index = 0;
            while inner_index < s_bytes.len() {
                is![index == CAP; break];
                buffer[index] = s_bytes[inner_index];
                index += 1;
                inner_index += 1;
            }
            outer_count += 1;
        }
        #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
        return unwrap![ok Str::from_utf8(slice![buffer, ..index])];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
        // SAFETY: since `string` is a valid &str, checks are unneeded.
        sf! { unsafe { str::from_utf8_unchecked(slice![buffer, ..index]) }}
    }

    /// Returns a [`&str`] backed by a `buffer`, where you always know each
    /// character's position.
    ///
    /// A [*counter string*][0] is a graduated string of arbitrary `length`,
    /// with a `separator` positioned after the immediately preceding number.
    /// # Examples
    /// ```
    /// # use devela_base_core::Str;
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

            let mut num_buf = Digits(num).digits10(); // IMPROVE
            let mut num_bytes = Slice::trim_leading(&num_buf, b'0');

            let mut num_len = num_bytes.len();

            loop {
                if separator_turn {
                    buffer[index] = separator;
                } else {
                    is![index > 0; index -= num_len - 1];
                    slice![mut buffer, index, ..num_len + index]
                        .copy_from_slice(slice![num_bytes, ..num_len]);
                    num = index;
                    num_buf = Digits(num).digits10(); // IMPROVE
                    num_bytes = Slice::trim_leading(&num_buf, b'0');
                    num_len = num_bytes.len();
                }
                is![index == 0; break; index -= 1];
                separator_turn = !separator_turn;
            }

            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            return unwrap![ok Str::from_utf8(slice![buffer, ..length])];
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: TODO: since `string` is a valid &str, checks are unneeded.
            sf! { unsafe { Str::from_utf8_unchecked(slice![buffer, ..length]) }}
        }
    }

    /* private utilities */

    #[doc(hidden)]
    /// The cold path that returns an empty string slice.
    #[cold] #[rustfmt::skip]
    pub const fn new_cold_empty() -> &'static str { "" }
}
