// devela_base_core::text::str::namespace::split
//
//! Implements split methods for [`Str`].
//

use crate::{Cmp, Str};

/// # `*split*` API methods for subslicing.
///
/// These methods always clamp the length to avoid overflow.
#[rustfmt::skip]
impl Str {
    /* left split */

    /// Returns the leftmost sub-`string` with the given maximum clamped `len`.
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
    #[must_use] #[inline(always)]
    pub const fn lsplit(string: &str, len: usize) -> &str {
        Str::range_to(string, Cmp(len).min(string.len()))
    }

    /// Returns the leftmost sub-`string` with the given maximum clamped `len`.
    ///
    /// Returns `None` if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cmp, Str};
    /// let s = "Hellø wørld!";
    /// assert_eq!(Str::lsplit_checked(s, 0), Some(""));
    /// assert_eq!(Str::lsplit_checked(s, 3), Some("Hel"));
    /// assert!(Str::lsplit_checked(s, 5).is_none()); // attempts to split `ø`
    /// assert_eq!(Str::lsplit_checked(s, 20), Some("Hellø wørld!"));
    /// ```
    #[must_use] #[inline(always)]
    pub const fn lsplit_checked(string: &str, len: usize) -> Option<&str> {
        Str::range_to_checked(string, Cmp(len).min(string.len()))
    }

    /// Returns the leftmost sub-`string` with the given maximum clamped `len`.
    ///
    /// If `len > self.len()` it returns the full string.
    ///
    /// # Panics
    /// Panics if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<16>::with("Hello world!").unwrap();
    /// assert_eq!(&*Str::lsplit(&mut s, 0), "");
    /// assert_eq!(&*Str::lsplit(&mut s, 3), "Hel");
    /// assert_eq!(&*Str::lsplit(&mut s, 20), "Hello world!");
    /// ```
    #[must_use] #[inline(always)]
    pub const fn lsplit_mut(string: &mut str, len: usize) -> &mut str {
        Str::range_to_mut(string, Cmp(len).min(string.len()))
    }

    /// Returns the leftmost sub-`string` with the given maximum clamped `len`.
    ///
    /// Returns `None` if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cmp, Str, StringU8};
    /// let mut s = StringU8::<16>::with("Hellø wørld!").unwrap();
    /// assert!(Str::lsplit_mut_checked(&mut s, 0).is_some_and(|s| &*s == ""));
    /// assert!(Str::lsplit_mut_checked(&mut s, 3).is_some_and(|s| &*s == "Hel"));
    /// assert!(Str::lsplit_mut_checked(&mut s, 5).is_none()); // attempts to split `ø`
    /// assert!(Str::lsplit_mut_checked(&mut s, 20).is_some_and(|s| &*s == "Hellø wørld!"));
    /// ```
    #[must_use] #[inline(always)]
    pub const fn lsplit_mut_checked(string: &mut str, len: usize) -> Option<&mut str> {
        Str::range_to_mut_checked(string, Cmp(len).min(string.len()))
    }

    /* right split */

    /// Returns the rightmost sub-`string` with the given maximum clamped `len`.
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
        Str::range_from(string, string.len().saturating_sub(len))
    }

    /// Returns the rightmost sub-`string` with the given maximum clamped `len`.
    ///
    /// Returns `None` if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cmp, Str};
    /// let s = "Hellø wørld!";
    /// assert_eq!(Str::rsplit_checked(s, 0), Some(""));
    /// assert_eq!(Str::rsplit_checked(s, 3), Some("ld!"));
    /// assert!(Str::rsplit_checked(s, 5).is_none()); // attempts to split `ø`
    /// assert_eq!(Str::rsplit_checked(s, 20), Some("Hellø wørld!"));
    /// ```
    #[must_use] #[inline(always)]
    pub const fn rsplit_checked(string: &str, len: usize) -> Option<&str> {
        Str::range_from_checked(string, string.len().saturating_sub(len))
    }

    /// Returns the rightmost exclusive sub-`string` with the given maximum clamped `len`.
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
    #[must_use]
    pub const fn rsplit_mut(string: &mut str, len: usize) -> &mut str {
        Str::range_from_mut(string, string.len().saturating_sub(len))
    }

    /// Returns the righttmost sub-`string` with the given maximum clamped `len`.
    ///
    /// Returns `None` if the split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Cmp, Str, StringU8};
    /// let mut s = StringU8::<16>::with("Hellø wørld!").unwrap();
    /// assert!(Str::rsplit_mut_checked(&mut s, 0).is_some_and(|s| &*s == ""));
    /// assert!(Str::rsplit_mut_checked(&mut s, 3).is_some_and(|s| &*s == "ld!"));
    /// assert!(Str::rsplit_mut_checked(&mut s, 5).is_none()); // attempts to split `ø`
    /// assert!(Str::rsplit_mut_checked(&mut s, 20).is_some_and(|s| &*s == "Hellø wørld!"));
    /// ```
    #[must_use] #[inline(always)]
    pub const fn rsplit_mut_checked(string: &mut str, len: usize) -> Option<&mut str> {
        Str::range_from_mut_checked(string, string.len().saturating_sub(len))
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
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Cmp(mid_idx + half_len).min(string.len());
        Str::range(string, start_idx, end_idx)
    }

    /// Returns the middle sub-`string` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// Returns `None` if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hellø wørld!";
    /// assert_eq!(Str::msplit_left_checked(s, 0), Some(""));
    /// assert_eq!(Str::msplit_left_checked(s, 1), Some(" "));
    /// assert_eq!(Str::msplit_left_checked(s, 2), Some(" w"));
    /// assert_eq!(Str::msplit_left_checked(s, 3), None); // attempts to split ø
    /// assert_eq!(Str::msplit_left_checked(s, 4), None); // "
    /// assert_eq!(Str::msplit_left_checked(s, 5), None); // "
    /// assert_eq!(Str::msplit_left_checked(s, 6), Some("ø wø"));
    /// assert_eq!(Str::msplit_left_checked(s, 7), Some("lø wø"));
    /// assert_eq!(Str::msplit_left_checked(s, 20), Some("Hellø wørld!"));
    /// ```
    /// See also [`Str::msplit_right_checked`].
    #[must_use]
    pub const fn msplit_left_checked(string: &str, len: usize) -> Option<&str> {
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Cmp(mid_idx + half_len).min(string.len());
        Str::range_checked(string, start_idx, end_idx)
    }

    /// Returns the middle exclusive sub-`string` with the given maximum `len` and a left bias.
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
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Cmp(mid_idx + half_len).min(string.len());
        Str::range_mut(string, start_idx, end_idx)
    }

    /// Returns the middle exclusive sub-`string` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// Returns `None` if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<14>::with("Hellø wørld!").unwrap();
    /// assert!(Str::msplit_left_mut_checked(&mut s, 0).is_some_and(|s| &*s == ""));
    /// assert!(Str::msplit_left_mut_checked(&mut s, 1).is_some_and(|s| &*s == " "));
    /// assert!(Str::msplit_left_mut_checked(&mut s, 2).is_some_and(|s| &*s == " w"));
    /// assert!(Str::msplit_left_mut_checked(&mut s, 3).is_none()); // attempts to split ø
    /// assert!(Str::msplit_left_mut_checked(&mut s, 4).is_none()); // "
    /// assert!(Str::msplit_left_mut_checked(&mut s, 5).is_none()); // "
    /// assert!(Str::msplit_left_mut_checked(&mut s, 6).is_some_and(|s| &*s == "ø wø"));
    /// assert!(Str::msplit_left_mut_checked(&mut s, 7).is_some_and(|s| &*s == "lø wø"));
    /// assert!(Str::msplit_left_mut_checked(&mut s, 20).is_some_and(|s| &*s == "Hellø wørld!"));
    /// ```
    /// See also [`Str::msplit_right_mut`].
    #[must_use]
    pub const fn msplit_left_mut_checked(string: &mut str, len: usize) -> Option<&mut str> {
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Cmp(mid_idx + half_len).min(string.len());
        Str::range_mut_checked(string, start_idx, end_idx)
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
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Cmp(mid_idx + half_len + (len % 2)).min(string.len());
        Str::range(string, start_idx, end_idx)
    }

    /// Returns the middle sub-`string` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the right.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// Returns `None` if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Str;
    /// let s = "Hellø wørld!";
    /// assert_eq!(Str::msplit_right_checked(s, 0), Some(""));
    /// assert_eq!(Str::msplit_right_checked(s, 1), Some("w"));
    /// assert_eq!(Str::msplit_right_checked(s, 2), Some(" w"));
    /// assert_eq!(Str::msplit_right_checked(s, 3), None); // attempts to split ø
    /// assert_eq!(Str::msplit_right_checked(s, 4), None); // "
    /// assert_eq!(Str::msplit_right_checked(s, 5), None); // "
    /// assert_eq!(Str::msplit_right_checked(s, 6), Some("ø wø"));
    /// assert_eq!(Str::msplit_right_checked(s, 7), Some("ø wør"));
    /// assert_eq!(Str::msplit_right_checked(s, 20), Some("Hellø wørld!"));
    /// ```
    /// See also [`Str::msplit_right_checked`].
    #[must_use]
    pub const fn msplit_right_checked(string: &str, len: usize) -> Option<&str> {
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Cmp(mid_idx + half_len + (len % 2)).min(string.len());
        Str::range_checked(string, start_idx, end_idx)
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
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Cmp(mid_idx + half_len + (len % 2)).min(string.len());
        Str::range_mut(string, start_idx, end_idx)
    }

    /// Returns the middle exclusive sub-`string` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the right.
    ///
    /// If `len > string.len()` returns the full `string`.
    ///
    /// Returns `None` if any split point falls outside a UTF-8 code point boundary.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::{Str, StringU8};
    /// let mut s = StringU8::<14>::with("Hellø wørld!").unwrap();
    /// assert!(Str::msplit_right_mut_checked(&mut s, 0).is_some_and(|s| &*s == ""));
    /// assert!(Str::msplit_right_mut_checked(&mut s, 1).is_some_and(|s| &*s == "w"));
    /// assert!(Str::msplit_right_mut_checked(&mut s, 2).is_some_and(|s| &*s == " w"));
    /// assert!(Str::msplit_right_mut_checked(&mut s, 3).is_none()); // attempts to split ø
    /// assert!(Str::msplit_right_mut_checked(&mut s, 4).is_none()); // "
    /// assert!(Str::msplit_right_mut_checked(&mut s, 5).is_none()); // "
    /// assert!(Str::msplit_right_mut_checked(&mut s, 6).is_some_and(|s| &*s == "ø wø"));
    /// assert!(Str::msplit_right_mut_checked(&mut s, 7).is_some_and(|s| &*s == "ø wør"));
    /// assert!(Str::msplit_right_mut_checked(&mut s, 20).is_some_and(|s| &*s == "Hellø wørld!"));
    /// ```
    /// See also [`Str::msplit_right_mut`].
    #[must_use]
    pub const fn msplit_right_mut_checked(string: &mut str, len: usize) -> Option<&mut str> {
        let (mid_idx, half_len) = (string.len() / 2, len / 2);
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Cmp(mid_idx + half_len + (len % 2)).min(string.len());
        Str::range_mut_checked(string, start_idx, end_idx)
    }
}
