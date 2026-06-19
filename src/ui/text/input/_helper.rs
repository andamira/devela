// devela/src/ui/text/_helper.rs
//
//! Internal helper methods for `TextInput`.
//

use crate::{
    Str, TextInput, TextInputAction, TextInputConfig, TextInputOutcome, TextInputReject, is, whilst,
};
use TextInputOutcome::{Accepted, Cancelled, Changed, Rejected, Unchanged};

impl<B> TextInput<B> {
    pub(crate) fn valid_cursor(s: &str, cursor: usize) -> bool {
        cursor <= s.len() && s.is_char_boundary(cursor)
    }
    pub(crate) fn valid_cursor_bytes(bytes: &[u8], len: usize, cursor: usize) -> bool {
        Self::valid_cursor(Self::valid_str(bytes, len), cursor)
    }
    pub(crate) fn valid_str(bytes: &[u8], len: usize) -> &str {
        match Str::from_utf8(&bytes[..len]) {
            Ok(s) => s,
            Err(_) => panic!("invalid TextInput UTF-8 invariant"),
        }
    }
    #[rustfmt::skip]
    pub(crate) fn apply_bytes(
        bytes: &mut [u8],
        len: &mut usize,
        cursor: &mut usize,
        config: TextInputConfig,
        action: TextInputAction,
    ) -> TextInputOutcome {
        match action {
            TextInputAction::Insert(ch) => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                let need = ch.len_utf8();
                if Self::exceeds_limit(*len, need, bytes.len(), config) {
                    return Rejected(TextInputReject::Full);
                }
                Self::insert_bytes(bytes, len, cursor, ch);
                Changed
            }
            TextInputAction::Backspace => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                if *cursor == 0 { return Unchanged; }
                let prev = Self::prev_char_boundary(Self::valid_str(bytes, *len), *cursor);
                Self::delete_bytes(bytes, len, prev, *cursor);
                *cursor = prev;
                Changed
            }
            TextInputAction::Delete => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                if *cursor == *len { return Unchanged; }
                let next = Self::next_char_boundary(Self::valid_str(bytes, *len), *cursor);
                Self::delete_bytes(bytes, len, *cursor, next);
                Changed
            }
            TextInputAction::MoveLeft => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                let prev = Self::prev_char_boundary(Self::valid_str(bytes, *len), *cursor);
                if prev == *cursor { Unchanged } else { *cursor = prev; Changed }
            }
            TextInputAction::MoveRight => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                let next = Self::next_char_boundary(Self::valid_str(bytes, *len), *cursor);
                if next == *cursor { Unchanged } else { *cursor = next; Changed }
            }
            TextInputAction::MoveStart => {
                if *cursor == 0 { Unchanged } else { *cursor = 0; Changed }
            }
            TextInputAction::MoveEnd => {
                if *cursor == *len { Unchanged } else { *cursor = *len; Changed }
            }
            TextInputAction::Clear => {
                if *len == 0 { Unchanged } else { Self::clear_bytes(bytes, len, cursor); Changed }
            }
            TextInputAction::Accept => {
                if !Self::valid_cursor_bytes(bytes, *len, *cursor) {
                    return Rejected(TextInputReject::InvalidCursor);
                }
                if *len == 0 && !config.can_be_empty { Rejected(TextInputReject::Empty) }
                else { Accepted }
            }
            TextInputAction::Cancel => Cancelled,
        }
    }
    pub(crate) fn exceeds_limit(
        len: usize,
        add: usize,
        capacity: usize,
        config: TextInputConfig,
    ) -> bool {
        let Some(new_len) = len.checked_add(add) else {
            return true;
        };
        is! { new_len > capacity, return true }
        if let Some(max) = config.max_bytes {
            is! { new_len > max, return true }
        }
        false
    }
    fn insert_bytes(bytes: &mut [u8], len: &mut usize, cursor: &mut usize, ch: char) {
        let need = ch.len_utf8();
        let old_len = *len;
        let at = *cursor;
        whilst! { i in rev at,..old_len; {
            bytes[i + need] = bytes[i];
        }}
        let mut tmp = [0u8; 4];
        let encoded = ch.encode_utf8(&mut tmp).as_bytes();
        whilst! { j in 0..need; {
            bytes[at + j] = encoded[j];
        }}
        *len += need;
        *cursor += need;
    }
    fn delete_bytes(bytes: &mut [u8], len: &mut usize, start: usize, end: usize) {
        let old_len = *len;
        let removed = end - start;
        whilst! { i in end,..old_len; {
            bytes[i - removed] = bytes[i];
        }}
        whilst! { j in old_len - removed,..old_len; {
            bytes[j] = 0;
        }}
        *len -= removed;
    }
    fn clear_bytes(bytes: &mut [u8], len: &mut usize, cursor: &mut usize) {
        whilst! { i in 0..*len; {
            bytes[i] = 0;
        }}
        *len = 0;
        *cursor = 0;
    }
    pub(crate) fn prev_char_boundary(s: &str, cursor: usize) -> usize {
        let mut prev = 0;
        for (i, _) in s.char_indices() {
            is! { i >= cursor, break }
            prev = i;
        }
        prev
    }
    pub(crate) fn next_char_boundary(s: &str, cursor: usize) -> usize {
        is! { cursor >= s.len(), return cursor }
        for (i, _) in s[cursor..].char_indices().skip(1) {
            return cursor + i;
        }
        s.len()
    }
}
