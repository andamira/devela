// devela_base_core::text::str::str::take
//
//! Implements take methods for [`Str`].
//

use crate::{Str, unwrap};

/// # `take*` API methods for subslicing.
#[rustfmt::skip]
impl Str {
    // take_first

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&string[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `string.len()`
    /// or if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hello world!";
    /// assert_eq!(Str::take_first(s, 0), "");
    /// assert_eq!(Str::take_first(s, 3), "Hel");
    /// assert_eq!(Str::take_first(s, 12), "Hello world!");
    /// // assert_eq!(Str::take_first(s, 13), "Hello world!"); // panics
    /// ```
    #[must_use] #[inline(always)]
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
