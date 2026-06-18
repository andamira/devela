// devela/src/text/str/namespace/define.rs
//
//! Defines [`Str`].
//

#[doc = crate::_tags!(string namespace)]
/// A string slice namespace.
#[doc = crate::_doc_meta!{location("text/str")}]
///
/// Provides stateless, non-allocating operations over valid UTF-8 string slices.
///
/// - For scalar-level Unicode operations see [`Char`][crate::Char].
/// - For cursor-based parsing see [`TextScanner`][crate::TextScanner].
///
/// # Methods
///
/// - [UTF-8 conversion methods](#utf-8-conversion-methods)
///   - [from_utf8](#method.from_utf8) ([*mut*](#method.from_utf8_mut),
///     [*unchecked*](#method.from_utf8_unchecked),
///     [*unchecked_mut*](#method.from_utf8_unchecked_mut)).
///
/// - [text traversal methods](#text-traversal-methods)
///   - [chars](#method.chars),
///   - [char_count](#method.char_count),
///   - [graphemes_in](#method.graphemes_in), ([*charu*](#method.graphemes_charu_in)),
///   - [grapheme_count](#method.grapheme_count).
///
/// - [equality and boundary methods](#equality-and-boundary-methods)
///   - [eq](#method.eq),
///   - [starts_with](#method.starts_with) ([*char*](#method.starts_with_char)),
///   - [ends_with](#method.ends_with) ([*char*](#method.ends_with_char)),
///   - [strip_prefix](#method.strip_prefix) ([*char*](#method.strip_prefix_char)),
///   - [strip_suffix](#method.strip_suffix) ([*char*](#method.strip_suffix_char)),
///   - [strip_circumfix](#method.strip_circumfix) ([*chars*](#method.strip_circumfix_chars)).
///
/// - [text writing methods](#text-writing-methods)
///   - [repeat_into](#method.repeat_into) ([*slice*](#method.repeat_into_slice)),
///   - [new_counter](#method.new_counter).
///   - [translit_ascii_into](#method.translit_ascii_into)
///     ([*or*](#method.translit_ascii_into_or)).
///
/// - [`range*` API methods](#range-api-methods-for-returning-substrings):<br/>
///   - [range](#method.range)
///    ([*checked*](#method.range_checked),
///     [_mut_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked)),                     ≈ `&str[start..end]`
///   - [range_inclusive](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [_mut_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked)),           ≈ `&str[start..=end]`
///   - [range_from](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [_mut_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked)),                ≈ `&str[start..]`
///   - [range_to](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [_mut_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked)),                  ≈ `&str[..end]`
///   - [range_to_inclusive](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [_mut_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked)).        ≈ `&str[..=end]`
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):<br/>
///   - [take_first](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [_mut_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked)),                ≈ `&str[..n]`
///   - [take_last](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [_mut_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked)),                 ≈ `&str[len - n..]`
///   - [take_omit_last](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [_mut_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked)).            ≈ `&str[..len - n]`
///
/// - [`*split*` API methods](#split-api-methods-for-returning-substrings):<br/>
///   - [lsplit](#method.lsplit)
///    ([*checked*](#method.lsplit_checked),
///    [mut](#method.lsplit_mut),
///    [*mut_checked*](#method.lsplit_mut_checked)),
///   - [rsplit](#method.rsplit)
///    ([*checked*](#method.rsplit_checked),
///    [*mut*](#method.rsplit_mut),
///    [*mut_checked*](#method.rsplit_mut_checked)),
///   - [msplit_left](#method.msplit_left)
///    ([*checked*](#method.msplit_left_checked),
///    [*mut*](#method.msplit_left_mut),
///    [*mut_checked*](#method.msplit_left_mut_checked)),
///   - [msplit_right](#method.msplit_right)
///    ([*checked*](#method.msplit_right_checked),
///    [*mut*](#method.msplit_right_mut).
///    [*mut_checked*](#method.msplit_right_mut_checked)).
///
/// See also the [`core::str`] module.
#[derive(Debug)]
pub struct Str;
