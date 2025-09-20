// devela_base_core::text::str::namespace
//
//! [`Str`] namespace.
//

use crate::{AsciiDigits, Compare, InvalidUtf8, Slice, is};

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
/// # Methods
/// - [methods namespaced from `core::str`](#corestr-namespaced-methods)
/// - [extra methods](#extra-methods)
///
/// - [`range*` API methods](#range-api-methods-for-returning-substrings):</br>
///   - [**range_to**](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [_**mut**_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked),                  (`&str[..end]`)
///   - [**range_to_inclusive**](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [_**mut**_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked),        (`&str[..=end]`)
///   - [**range_from**](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [_**mut**_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked),                (`&str[start..]`)
///   - [**range**](#method.range)
///    ([*checked*](#method.range_checked),
///     [_**mut**_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked),                     (`&str[start..end]`)
///   - [**range_inclusive**](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [_**mut**_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked).           (`&str[start..=end]`)
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):</br>
///   - [**take_first**](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [_**mut**_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked),                (`&str[..n]`)
///   - [**take_last**](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [_**mut**_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked),                 (`&str[str.len() - n..]`)
///   - [**take_omit_last**](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [_**mut**_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked).            (`&str[..str.len() - n]`)
///
/// - [`*split*` API methods](#split-api-methods-for-returning-substrings):</br>
///   - [**lsplit**](#method.lsplit)
///    ([*mut*](#method.lsplit_mut)),
///   - [**rsplit**](#method.rsplit)
///    ([*mut*](#method.rsplit_mut)),
///   - [**msplit_left**](#method.msplit_left)
///    ([*mut*](#method.msplit_left_mut)),
///   - [**msplit_right**](#method.msplit_right)
///    ([*mut*](#method.msplit_right_mut)).
///
/// See also the [`std::str`] module.
#[derive(Debug)]
pub struct Str;

/// # `core::str` namespaced methods.
impl Str {
    /// Converts a slice of bytes to a string slice.
    ///
    /// See `core::str::`[`from_utf8`].
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
        { crate::unwrap![ok ::core::str::from_utf8(bytes)] }
        #[cfg(all(not(base_safe_text), unsafe··))]
        unsafe { ::core::str::from_utf8_unchecked(bytes) }
    }
}

/// # extra methods.
impl Str {
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
    /// See also [`ExtStr::repeat_into`], which should be faster,
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
        return unwrap![ok Str::from_utf8(Slice::range_to(buffer, index))];
        #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
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

            let mut num_buf = AsciiDigits(num).digits();
            let mut num_bytes = Slice::trim_leading_bytes(&num_buf, b'0');
            // IMPROVE:BENCH use NumToStr
            // let mut num_buf = [0u8; 22];
            // let mut num_bytes = num.to_bytes_base(10, &mut num_buf);

            let mut num_len = num_bytes.len();

            loop {
                if separator_turn {
                    buffer[index] = separator;
                } else {
                    is![index > 0; index -= num_len - 1];
                    // WAIT:DONE:1.87 const_copy_from_slice
                    // buffer[index..(num_len + index)].copy_from_slice(&num_bytes[..num_len]);
                    // Slice::range_mut(buffer, index, num_len + index)
                    //     .copy_from_slice(Slice::range_to(num_bytes, num_len));
                    let mut i = 0;
                    while i < num_len {
                        buffer[index + i] = num_bytes[i];
                        i += 1;
                    }

                    num = index;

                    num_buf = AsciiDigits(num).digits();
                    num_bytes = Slice::trim_leading_bytes(&num_buf, b'0');
                    // IMPROVE: use NumToStr
                    // num_bytes = num.to_bytes_base(10, &mut num_buf);

                    num_len = num_bytes.len();
                }
                is![index == 0; break; index -= 1];
                separator_turn = !separator_turn;
            }

            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            return unwrap![ok Str::from_utf8(Slice::range_to(buffer, length))];
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            // SAFETY: TODO: since `string` is a valid &str, checks are unneeded.
            sf! { unsafe { Str::from_utf8_unchecked(Slice::range_to(buffer, length)) }}
        }
    }

    /* private utilities */

    #[doc(hidden)]
    /// The cold path that returns an empty string slice.
    #[cold] #[rustfmt::skip]
    pub const fn new_cold_empty() -> &'static str { "" }
}

/// # `range*` API methods for returning substrings.
///
/// Similar to `Slice::`[`range*` API methods](crate::Slice#range-api-methods-for-subslicing)
impl Str {
    // range_to

    /// Returns a substring up to the given `end` index.
    ///
    /// Equivalent to `&string[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to(string: &str, end: usize) -> &str {
        string.split_at(end).0
    }

    /// Returns a substring up to the given `end` index.
    ///
    /// Equivalent to `&string[..end]`.
    ///
    /// Returns `None` if `end` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_checked(string: &str, end: usize) -> Option<&str> {
        match string.split_at_checked(end) {
            Some((substring, _)) => Some(substring),
            None => None,
        }
    }

    /// Returns an exclusive substring up to the given `end` index.
    ///
    /// Equivalent to `&mut string[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_mut(string: &mut str, end: usize) -> &mut str {
        string.split_at_mut(end).0
    }

    /// Returns an exclusive substring up to the given `end` index.
    ///
    /// Equivalent to `&mut string[..end]`.
    ///
    /// Returns `None` if `end` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_mut_checked(string: &mut str, end: usize) -> Option<&mut str> {
        match string.split_at_mut_checked(end) {
            Some((substring, _)) => Some(substring),
            None => None,
        }
    }

    // range_to_inclusive

    /// Returns a substring up to and including the given `end` index.
    ///
    /// Equivalent to `&string[..=end]`.
    ///
    /// # Panics
    /// Panics if `end` >= `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive(string: &str, end: usize) -> &str {
        string.split_at(end + 1).0
    }

    /// Returns a substring up to and including the given `end` index.
    ///
    /// Equivalent to `&string[..=end]`.
    ///
    /// Returns `None` if `end` >= `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_checked(string: &str, end: usize) -> Option<&str> {
        is![end < string.len(); Some(string.split_at(end + 1).0); None]
    }

    /// Returns an exclusive substring up to and including the given `end` index.
    ///
    /// Equivalent to `&string[..=end]`.
    ///
    /// # Panics
    /// Panics if `end` >= `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_mut(string: &mut str, end: usize) -> &mut str {
        string.split_at_mut(end + 1).0
    }

    /// Returns an exclusive substring up to and including the given `end` index.
    ///
    /// Equivalent to `&string[..=end]`.
    ///
    /// Returns `None` if `end` >= `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_to_inclusive_mut_checked(string: &mut str, end: usize) -> Option<&mut str> {
        is![end < string.len(); Some(string.split_at_mut(end + 1).0); None]
    }

    // range_from

    /// Returns a substring starting from the given `start` index.
    ///
    /// Equivalent to `&string[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_from(string: &str, start: usize) -> &str {
        string.split_at(start).1
    }

    /// Returns a substring starting from the given `start` index.
    ///
    /// Equivalent to `&string[start..]`.
    ///
    /// Returns `None` if `start` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_checked(string: &str, start: usize) -> Option<&str> {
        match string.split_at_checked(start) {
            Some((_, substring)) => Some(substring),
            None => None,
        }
    }

    /// Returns an exclusive substring starting from the given `start` index.
    ///
    /// Equivalent to `&mut string[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_mut(string: &mut str, start: usize) -> &mut str {
        string.split_at_mut(start).1
    }

    /// Returns an exclusive substring starting from the given `start` index.
    ///
    /// Equivalent to `&mut string[start..]`.
    ///
    /// Returns `None` if `start` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_from_mut_checked(string: &mut str, start: usize) -> Option<&mut str> {
        match string.split_at_mut_checked(start) {
            Some((_, substring)) => Some(substring),
            None => None,
        }
    }

    // range

    /// Returns a substring from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&string[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` > `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range(string: &str, start: usize, end: usize) -> &str {
        string.split_at(start).1.split_at(end - start).0
    }

    /// Returns a substring from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&string[start..end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_checked(string: &str, start: usize, end: usize) -> Option<&str> {
        Some(
            unwrap![some?
                unwrap![some? string.split_at_checked(start)].1
                    .split_at_checked(end - start)]
            .0,
        )
    }

    /// Returns an exclusive substring from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut string[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` > `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_mut(string: &mut str, start: usize, end: usize) -> &mut str {
        string.split_at_mut(start).1.split_at_mut(end - start).0
    }

    /// Returns an exclusive substring from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut string[start..end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_mut_checked(string: &mut str, start: usize, end: usize) -> Option<&mut str> {
        Some(
            unwrap![some?
                unwrap![some? string.split_at_mut_checked(start)].1
                    .split_at_mut_checked(end - start)]
            .0,
        )
    }

    // range_inclusive

    /// Returns a substring from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&string[start..=end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` >= `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive(string: &str, start: usize, end: usize) -> &str {
        string.split_at(start).1.split_at(end - start + 1).0
    }

    /// Returns a substring from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&string[start..=end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_checked(string: &str, start: usize, end: usize) -> Option<&str> {
        Some(
            unwrap![some?
                unwrap![some? string.split_at_checked(start)].1
                    .split_at_checked(end - start + 1)]
            .0,
        )
    }

    /// Returns a substring from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&mut string[start..=end]`.
    ///
    /// # Panics
    /// Panics if `start` > `end` or `end` >= `string.len()`
    /// or if any split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_mut(string: &mut str, start: usize, end: usize) -> &mut str {
        string.split_at_mut(start).1.split_at_mut(end - start + 1).0
    }

    /// Returns a substring from `start` (inclusive) to `end` (inclusive).
    ///
    /// Equivalent to `&mut string[start..=end]`.
    ///
    /// Returns `None` if `start` > `end` or `end` > `string.len()`
    /// or if a split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn range_inclusive_mut_checked(
        string: &mut str,
        start: usize,
        end: usize,
    ) -> Option<&mut str> {
        Some(
            unwrap![some?
                unwrap![some? string.split_at_mut_checked(start)].1
                    .split_at_mut_checked(end - start + 1)]
            .0,
        )
    }
}

/// # `take*` API methods for subslicing.
impl Str {
    // take_first

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&string[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_first(string: &str, n: usize) -> &str {
        string.split_at(n).0
    }

    /// Returns the first `n` elements of the string.
    ///
    /// Equivalent to `&string[..n]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_checked(string: &str, n: usize) -> Option<&str> {
        match string.split_at_checked(n) {
            Some((substring, _)) => Some(substring),
            None => None,
        }
    }

    /// Returns the first `n` elements of the exclusive string.
    ///
    /// Equivalent to `&mut string[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_mut(string: &mut str, n: usize) -> &mut str {
        string.split_at_mut(n).0
    }

    /// Returns the first `n` elements of the exclusive string.
    ///
    /// Equivalent to `&mut string[..n]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_first_mut_checked(string: &mut str, n: usize) -> Option<&mut str> {
        match string.split_at_mut_checked(n) {
            Some((substring, _)) => Some(substring),
            None => None,
        }
    }

    // take_last

    /// Returns the last `n` elements of the string.
    ///
    /// Equivalent to `&string[string.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_last(string: &str, n: usize) -> &str {
        string.split_at(string.len() - n).1
    }

    /// Returns the last `n` elements of the string.
    ///
    /// Equivalent to `&string[string.len() - n..]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_checked(string: &str, n: usize) -> Option<&str> {
        Some(unwrap![some? string.split_at_checked(string.len() - n)].1)
    }

    /// Returns the last `n` elements of the exclusive string.
    ///
    /// Equivalent to `&mut string[string.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_mut(string: &mut str, n: usize) -> &mut str {
        string.split_at_mut(string.len() - n).1
    }

    /// Returns the last `n` elements of the exclusive string.
    ///
    /// Equivalent to `&mut string[string.len() - n..]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_last_mut_checked(string: &mut str, n: usize) -> Option<&mut str> {
        Some(unwrap![some? string.split_at_mut_checked(string.len() - n)].1)
    }

    // take_omit_last

    /// Returns the string omitting the last `n` elements.
    ///
    /// Equivalent to `&string[..string.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last(string: &str, n: usize) -> &str {
        string.split_at(string.len() - n).0
    }

    /// Returns the string omitting the last `n` elements.
    ///
    /// Equivalent to `&string[..string.len() - n]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_checked(string: &str, n: usize) -> Option<&str> {
        Some(unwrap![some? string.split_at_checked(string.len() - n)].0)
    }

    /// Returns the exclusive string omitting the last `n` elements.
    ///
    /// Equivalent to `&mut string[..string.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_mut(string: &mut str, n: usize) -> &mut str {
        string.split_at_mut(string.len() - n).0
    }

    /// Returns the exclusive string omitting the last `n` elements.
    ///
    /// Equivalent to `&mut string[..string.len() - n]`.
    ///
    /// Returns `None` if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    #[must_use]
    #[inline(always)]
    pub const fn take_omit_last_mut_checked(string: &mut str, n: usize) -> Option<&mut str> {
        Some(unwrap![some? string.split_at_mut_checked(string.len() - n)].0)
    }
}

/// # `*split*` API methods for subslicing.
impl Str {
    /* left split */

    /// Returns the leftmost sub-`string` with the given maximum `len`.
    ///
    /// If `len > self.len()` it returns the full string.
    ///
    /// # Panics
    /// Panics if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hello world!";
    /// assert_eq!(Str::lsplit(s, 0), "");
    /// assert_eq!(Str::lsplit(s, 3), "Hel");
    /// assert_eq!(Str::lsplit(s, 20), "Hello world!");
    /// ```
    pub const fn lsplit(string: &str, len: usize) -> &str {
        let end_idx = Compare(len).clamp(0, string.len());
        let (left, _) = string.split_at(end_idx);
        left
    }

    /// Returns the leftmost exclusive sub-`string` with the given maximum `len`.
    ///
    /// If `left_len > string.len()` it returns the full string.
    ///
    /// # Panics
    /// Panics if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<16>::with("Hello world!").unwrap();
    /// assert_eq!(&*Str::lsplit_mut(&mut s, 0), "");
    /// assert_eq!(&*Str::lsplit_mut(&mut s, 3), "Hel");
    /// assert_eq!(&*Str::lsplit_mut(&mut s, 20), "Hello world!");
    /// ```
    /// See also [`string::lsplit_mut`].
    pub const fn lsplit_mut(string: &mut str, len: usize) -> &mut str {
        let end_idx = Compare(len).clamp(0, string.len());
        let (left, _) = string.split_at_mut(end_idx);
        left
    }

    /* right split */

    /// Returns the rightmost sub-`string` with the given maximum `len`.
    ///
    /// If `left_len > string.len()` it returns the full string.
    ///
    /// # Panics
    /// Panics if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hello world!";
    /// assert_eq!(Str::rsplit(s, 0), "");
    /// assert_eq!(Str::rsplit(s, 3), "ld!");
    /// assert_eq!(Str::rsplit(s, 20), "Hello world!");
    /// ```
    #[must_use]
    pub const fn rsplit(string: &str, len: usize) -> &str {
        let start_idx = string.len().saturating_sub(len);
        let (_, right) = string.split_at(start_idx);
        right
    }

    /// Returns the rightmost exclusive sub-`string` with the given maximum `len`.
    ///
    /// If `left_len > string.len()` it returns the full string.
    ///
    /// # Panics
    /// Panics if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<12>::with("Hello world!").unwrap();
    /// assert_eq!(&*Str::rsplit_mut(&mut s, 0), "");
    /// assert_eq!(&*Str::rsplit_mut(&mut s, 3), "ld!");
    /// assert_eq!(&*Str::rsplit_mut(&mut s, 20), "Hello world!");
    /// ```
    /// See also [`Str::lsplit_mut`].
    #[must_use]
    pub const fn rsplit_mut(string: &mut str, len: usize) -> &mut str {
        let start_idx = string.len().saturating_sub(len);
        let (_, right) = string.split_at_mut(start_idx);
        right
    }

    /* middle split left biased */

    /// Returns the middle sub-`string` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// # Panics
    /// Panics if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hello world!";
    /// assert_eq!(Str::msplit_left(s, 0), "");
    /// assert_eq!(Str::msplit_left(s, 1), " ");
    /// assert_eq!(Str::msplit_left(s, 2), " w");
    /// assert_eq!(Str::msplit_left(s, 3), "o w");
    /// assert_eq!(Str::msplit_left(s, 4), "o wo");
    /// assert_eq!(Str::msplit_left(s, 5), "lo wo");
    /// assert_eq!(Str::msplit_left(s, 20), "Hello world!");

    /// ```
    /// See also [`Str::msplit_right`].
    #[must_use]
    pub const fn msplit_left(string: &str, len: usize) -> &str {
        let mid_idx = string.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Compare(mid_idx + half_len).min(string.len());
        let (_, right) = string.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }

    /// Returns the middle exclusive sub-`string` with the given maximum `len` and a
    /// left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// # Panics
    /// Panics if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<12>::with("Hello world!").unwrap();
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 0), "");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 1), " ");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 2), " w");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 3), "o w");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 4), "o wo");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 5), "lo wo");
    /// assert_eq!(&*Str::msplit_left_mut(&mut s, 20), "Hello world!");
    /// ```
    /// See also [`Str::msplit_right_mut`].
    #[must_use]
    pub const fn msplit_left_mut(string: &mut str, len: usize) -> &mut str {
        let mid_idx = string.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Compare(mid_idx + half_len).min(string.len());
        let (_, right) = string.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }

    /* middle split right biased */

    /// Returns the middle sub-`string` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the right.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// # Panics
    /// Panics if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let s = "Hello world!";
    /// assert_eq!(Str::msplit_right(&s, 0), "");
    /// assert_eq!(Str::msplit_right(&s, 1), "w");
    /// assert_eq!(Str::msplit_right(&s, 2), " w");
    /// assert_eq!(Str::msplit_right(&s, 3), " wo");
    /// assert_eq!(Str::msplit_right(&s, 4), "o wo");
    /// assert_eq!(Str::msplit_right(&s, 5), "o wor");
    /// assert_eq!(Str::msplit_right(&s, 20), "Hello world!");
    /// ```
    /// See also [`Str::msplit_left`].
    #[must_use]
    pub const fn msplit_right(string: &str, len: usize) -> &str {
        let mid_idx = string.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Compare(mid_idx + half_len + (len % 2)).min(string.len());
        let (_, right) = string.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }

    /// Returns the middle exclusive sub-`string` with the given maximum `len` and a
    /// right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// # Panics
    /// Panics if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<12>::with("Hello world!").unwrap();
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 0), "");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 1), "w");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 2), " w");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 3), " wo");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 4), "o wo");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 5), "o wor");
    /// assert_eq!(&*Str::msplit_right_mut(&mut s, 20), "Hello world!");
    /// ```
    /// See also [`Str::msplit_left_mut`].
    #[must_use]
    pub const fn msplit_right_mut(string: &mut str, len: usize) -> &mut str {
        let mid_idx = string.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Compare(mid_idx + half_len + (len % 2)).min(string.len());
        let (_, right) = string.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }
}
