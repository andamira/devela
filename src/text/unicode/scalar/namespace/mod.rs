// devela::text::unicode::scalar::namespace
//
//! Defines the [`Char`] namespace.
//
// TOC
// - struct Char
// - methods over u16

#[cfg(test)]
mod tests;

mod char; // Char<char>
mod u16; // Char<u16>
mod u32; // Char<u32>
mod byte; // Char<u8>
mod slice; // Char<&[u8] | &[u8; N]>

#[doc = crate::_tags!(text namespace)]
/// Unicode scalars-related low-level *const* operations.
#[doc = crate::_doc_meta!{location("text/unicode/scalar")}]
///
/// # Terminology
/// - A **code point** is an integer in `U+0000..=U+10FFFF`.
/// - A **surrogate** is a code point in `U+D800..=U+DFFF`.
/// - A **Unicode scalar value** is a code point that is not a surrogate.
///   Rust's [`char`] represents exactly this set.
/// - A **scalar rank** is the zero-based position of a scalar value in the
///   ordered set of all Unicode scalar values, with the surrogate range omitted.
///
/// # Methods
/// - [over `char`](#methods-over-char)
///   - [len_utf8](#method.len_utf8).
///   - [width](#method.width) ([*common*](#method.width_common)).
///   - [is_combining](#method.is_combining) ([*common*](#method.is_combining_common)).
///   - [is_control](#method.is_control) ([*common*](#method.is_control_common)).
///   - [is_fullwidth](#method.is_fullwidth) ([*common*](#method.is_fullwidth_common)).
///   - [to_utf8_bytes](#method.to_utf8_bytes).
///   - [as_ascii](#method.as_ascii) ([*unchecked*](#method.as_ascii_unchecked)).
///   - [to_ascii_fold](#method.to_ascii_fold)
///    ([*unchecked*](#method.to_ascii_fold_unchecked)).
///   - [write_utf8_to](#method.write_utf8_to).
///
/// - [over `u16`](#methods-over-u16)
///   - [is_surrogate](#method.is_surrogate)
///    ([*high*](#method.is_surrogate_high), [*low*](#method.is_surrogate_low)).
///   - [decode_surrogate_pair](#method.decode_surrogate_pair).
///
/// - [over `u32`](#methods-over-u32)
///   - [len_bytes](#method.len_bytes).
///   - [len_utf8](#method.len_utf8-1) ([*unchecked*](#method.len_utf8_unchecked)).
///   - [width](#method.width-1) ([*common*](#method.width_common-1)).
///   - [is_valid_code](#method.is_valid_code).
///   - [is_valid_scalar](#method.is_valid_scalar).
///   - [is_noncharacter](#method.is_noncharacter).
///   - [is_combining](#method.is_combining-1) ([*common*](#method.is_combining_common-1)).
///   - [is_control](#method.is_control-1) ([*common*](#method.is_control_common-1)).
///   - [is_fullwidth](#method.is_fullwidth-1) ([*common*](#method.is_fullwidth_common-1)).
///   - [is_surrogate](#method.is_surrogate-1)
///    ([*high*](#method.is_surrogate_high-1), [*low*](#method.is_surrogate_low-1)).
///   - [as_ascii](#method.as_ascii-1) ([*unchecked*](#method.as_ascii_unchecked-1)).
///   - [to_utf8_bytes](#method.to_utf8_bytes-1)
///    ([*unchecked*](#method.to_utf8_bytes_unchecked)).
///   - [write_utf8_to_unchecked](#method.write_utf8_to_unchecked).
///
/// - [over `u8`](#methods-over-u8)
///   - [len_utf8](#method.len_utf8-2) ([*unchecked*](#method.len_utf8_unchecked-1)).
///   - [len_utf8_match](#method.len_utf8_match) ([*naive*](#method.len_utf8_match_naive)).
///   - [is_utf8_boundary](#method.is_utf8_boundary).
///   - [is_utf8_continuation](#method.is_utf8_continuation).
///   - [as_char](#method.as_char).
///
/// - [over `&[u8]`](#methods-over-u8-slice)
///   - [to_char](#method.to_char) ([*lenient*](#method.to_char_lenient),
///     [*unchecked*](#method.to_char_unchecked)<sup title="unsafe method">⚠</sup>).
///   - [to_scalar](#method.to_scalar) ([*unchecked*](#method.to_scalar_unchecked)).
///   - [has_overlong_encoding](#method.has_overlong_encoding).
///   - [has_valid_continuation](#method.has_valid_continuation).
///   - [ceil_utf8_boundary](#method.is_utf8_boundary).
///   - [floor_utf8_boundary](#method.is_utf8_boundary).
///   - [is_utf8_boundary](#method.is_utf8_boundary-1).
///
/// See also [`Str`][crate::Str].
#[derive(Clone, Copy, Debug)]
pub struct Char<T>(pub T);
