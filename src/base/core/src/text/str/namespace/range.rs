// devela_base_core::text::str::namespace::range
//
//! Implements range methods for [`Str`].
//

use crate::{Str, is, unwrap};

/// # `range*` API methods for returning substrings.
///
/// Similar to `Slice::`[`range*` API methods](crate::Slice#range-api-methods-for-subslicing)
impl Str {
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
}
