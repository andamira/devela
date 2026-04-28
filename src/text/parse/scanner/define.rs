// devela::text::parse::scanner
//
//! Defines [`TextScanner`].
//

#![allow(unused, missing_docs)]

#[cfg(doc)]
use crate::TextParseErrorKind;
use crate::{_impl_init, ConstInit, Slice, Str, is, unwrap, whilst};
use crate::{InvalidUtf8, TextCursor, TextIndex, TextParseError, TextRange, TextUnit};

#[must_use]
#[doc = crate::_tags!(text)]
/// A byte scanner over source text.
#[doc = crate::_doc_location!("text/parse")]
///
/// `TextScanner` provides incremental, allocation-free traversal over a borrowed
/// text source, exposing byte-oriented operations suitable for building parsers.
//
// # Methods
// - Construction and views:
//   - new
//   - from_bytes
//   - slice
//   - slice_str
//   - slice_str_unchecked
//   - rest
//   - take_rest
// - Cursor state and range construction:
//   - pos
//   - mark
//   - remaining_len
//   - advance
//   - is_eof
//   - range_from
// - Predicate-driven scanning adapters:
//   - eat_if
//   - skip_while
//   - take_while
// - Byte inspection and exact consumption:
//   - peek_byte
//   - peek_byte_at
//   - starts_with
//   - next_byte
//   - eat_byte
//   - eat_bytes
//   - expect_byte
//   - expect_bytes
// - Byte-delimited range scanning:
//   - take_until_byte
//   - take_until_bytes
//   - take_until_any
//   - take_until_any2
//   - take_until_any3
// - Quoted string scanning and decoding:
//   - take_quoted_basic
//   - take_quoted_basic_or_rest
//   - take_quoted_literal
//   - decode_quoted_basic_into
//   - decode_quoted_basic_str_into
// - ASCII scanning and range-taking operations:
//   - skip_ascii_ws
//   - skip_ascii_hws
//   - take_ascii_ident
//   - take_ascii_ident_tail
//   - trim_ascii_ws
//   - trim_ascii_hws
// - `AsciiSet` scanning:
//   - skip_ascii_set
//   - take_ascii_set
//   - take_ascii_run
// - ASCII numeric parsing:
//   - take_ascii_u64
//   - expect_ascii_u64
// - EOL and line-oriented scanning:
//   - eat_eol
//   - take_until_eol
//   - next_line
//   - next_line_trimmed
//   - next_line_trimmed_before
// - Quoted string scanning and decoding:
//   - take_quoted_basic
//   - take_quoted_basic_or_rest
//   - take_quoted_literal
//   - decode_quoted_basic_into
//   - decode_quoted_basic_str_into
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextScanner<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) cursor: TextCursor,
}
_impl_init![ConstInit: Self::new("") => TextScanner<'_>];
