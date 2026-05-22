// devela::sys::os::term::event::input::_helper

use crate::{_impl_init, EventKind, Position2, is, slice, unwrap, whilst};

/// Internal parser state.
#[derive(Clone, Debug, Default)]
pub(super) enum TermInputState {
    /// No partial sequence is active.
    #[default]
    Ground,
    /// A single `ESC` byte has been received.
    Esc,
    /// An SS3 sequence introduced by `ESC O` is waiting for its final byte.
    Ss3,
    /// A CSI sequence introduced by `ESC [` is being collected.
    Csi { buf: [u8; 16], len: u8 },
    /// A UTF-8 scalar is being collected.
    Utf8 { buf: [u8; 4], len: u8, need: u8 },
}
_impl_init! { Self::Ground => TermInputState }

/// Internal parser result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TermParsed {
    /// A normalized user-facing event.
    Event(EventKind),
    /// A terminal reply, usually produced by a query sequence.
    Reply(TermReply),
    /// The current sequence is valid so far but incomplete.
    Pending,
    /// The sequence is invalid, unsupported, or intentionally unrecognized.
    Unknown,
}

/// Terminal reply parsed from the input stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TermReply {
    /// Cursor-position report: `ESC [ row ; col R`.
    ///
    /// The position is in terminal cells, meaning x = column and y = row.
    CursorPosition(Position2<u16>),
    /// Device-attributes reply: `ESC [ ... c`.
    DeviceAttributes,
}

pub(super) const fn is_csi_final(byte: u8) -> bool {
    byte >= 0x40 && byte <= 0x7E
}
pub(super) const fn parse_two_u16(bytes: &[u8], sep: u8) -> Option<(u16, u16)> {
    let mut split = None;
    whilst! { i in 0..bytes.len(); {
        is! { bytes[i] == sep, { split = Some(i); break; }}
    }}
    let split = unwrap![some? split];
    let a = unwrap![some? parse_u16(slice!(bytes, ..split))];
    let b = unwrap![some? parse_u16(slice!(bytes, split + 1, ..))];
    Some((a, b))
}
pub(super) const fn parse_u16(bytes: &[u8]) -> Option<u16> {
    is! { bytes.is_empty(), return None }
    let mut n = 0u16;
    whilst! { b in 0..bytes.len(); {
        let byte = bytes[b];
        is!{ !byte.is_ascii_digit(), return None }
        n = unwrap![some?
            unwrap![some? n.checked_mul(10)]
                .checked_add((byte - b'0') as u16)];
    }}
    Some(n)
}
