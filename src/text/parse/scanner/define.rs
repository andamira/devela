// devela::text::parse::scanner
//
//! Defines [`TextScanner`].
//

#![allow(unused, missing_docs)]

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{_impl_init, Slice, Str, is, unwrap, whilst};
use crate::{InvalidUtf8, TextCursor, TextIndex, TextParseError, TextRange, TextUnit};

#[must_use]
#[doc = crate::_tags!(text parser)]
/// A byte scanner over source text.
#[doc = crate::_doc_meta!{location("text/parse")}]
///
/// `TextScanner` provides incremental, allocation-free traversal over a borrowed
/// text source, exposing byte-oriented operations suitable for building parsers.
///
/// # Methods
///
/// - Construction and views:
///   - [new](#method.new).
///   - [from_bytes](#method.from_bytes).
///   - [slice](#method.slice).
///   - [slice_str](#method.slice_str)
///     ([*unchecked*](#method.slice_str_unchecked)<sup title="unsafe method">⚠</sup>).
///   - [rest](#method.rest).
///   - [take_rest](#method.take_rest).
///
/// - Cursor state and range construction:
///   - [pos](#method.pos).
///   - [mark](#method.mark).
///   - [remaining_len](#method.remaining_len).
///   - [advance](#method.advance).
///   - [is_eof](#method.is_eof).
///   - [range_from](#method.range_from).
///
/// - Predicate-driven scanning adapters:
///   - [eat_if](#method.eat_if).
///   - [skip_while](#method.skip_while).
///   - [take_while](#method.take_while).
///
/// - Byte inspection and exact consumption:
///   - [peek_byte](#method.peek_byte) ([*at*](#method.peek_byte_at)).
///   - [starts_with](#method.starts_with).
///   - [next_byte](#method.next_byte).
///   - [skip_byte](#method.skip_byte).
///   - [eat_byte](#method.eat_byte) ([*s*](#method.eat_bytes)).
///   - [expect_byte](#method.expect_byte) ([*s*](#method.expect_bytes)).
///
/// - Byte-delimited range scanning:
///   - [take_until_byte](#method.take_until_byte) ([*s*](#method.take_until_bytes)).
///   - [take_until_any](#method.take_until_any)
///     ([*2*](#method.take_until_any2), [*3*](#method.take_until_any3)).
///
/// - Quoted string scanning and decoding:
///   - [take_quoted_basic](#method.take_quoted_basic)
///     ([*or_rest*](#method.take_quoted_basic_or_rest)).
///   - [take_quoted_literal](#method.take_quoted_literal).
///   - [decode_quoted_basic_into](#method.decode_quoted_basic_into).
///   - [decode_quoted_basic_str_into](#method.decode_quoted_basic_str_into).
///
/// - ASCII scanning and range-taking operations:
///   - [skip_ascii_ws](#method.skip_ascii_ws).
///   - [skip_ascii_hws](#method.skip_ascii_hws).
///   - [take_ascii_ident](#method.take_ascii_ident) ([*tail*](#method.take_ascii_ident_tail)).
///   - [trim_ascii_ws](#method.trim_ascii_ws).
///   - [trim_ascii_hws](#method.trim_ascii_hws).
///
/// - `AsciiSet` scanning:
///   - [eat_ascii_set](#method.eat_ascii_set).
///   - [skip_ascii_set](#method.skip_ascii_set).
///   - [skip_until_ascii_set](#method.skip_until_ascii_set).
///   - [take_ascii_set](#method.take_ascii_set).
///   - [take_ascii_run](#method.take_ascii_run).
///   - [take_until_ascii_set](#method.take_until_ascii_set).
///
/// - ASCII numeric parsing:
///   - [take_ascii_u64](#method.take_ascii_u64).
///   - [expect_ascii_u64](#method.expect_ascii_u64).
///
/// - EOL and line-oriented scanning:
///   - [eat_eol](#method.eat_eol).
///   - [take_until_eol](#method.take_until_eol).
///   - [next_line](#method.next_line) ([*trimmed*](#method.next_line_trimmed),
///     [*trimmed_before*](#method.next_line_trimmed_before)).
///
/// - UTF-8 Unicode scalar scanning:
///   - [peek_char](#method.peek_char) ([*u*](#method.peek_charu)).
///   - [next_char](#method.next_char) ([*u*](#method.next_charu)).
///   - [take_char](#method.take_char).
///   - [eat_char](#method.eat_char) ([*u*](#method.eat_charu)).
///   - [take_char_if](#method.take_char_if) ([*u*](#method.take_charu_if)).
///   - [skip_char_while](#method.skip_char_while) ([*u*](#method.skip_charu_while)).
///   - [take_char_while](#method.take_char_while) ([*u*](#method.take_charu_while)).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextScanner<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) cursor: TextCursor,
}
_impl_init![Self::new("") => TextScanner<'_>];
