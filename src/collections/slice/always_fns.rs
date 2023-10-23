// devela::collections::slice::always_fns
//
//! `slice` standalone functions.
//!
//! Always available for internal use.
//

#![allow(unused)]

/// Returns a subslice without the given leading `byte`s.
#[inline]
#[must_use]
pub fn slice_trim_leading_bytes(slice: &[u8], byte: u8) -> &[u8] {
    let mut start = 0;
    while start < slice.len() && slice[start] == byte {
        start += 1;
    }
    &slice[start..]
}
