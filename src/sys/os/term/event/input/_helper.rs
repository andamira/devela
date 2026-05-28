// devela::sys::os::term::event::input::_helper
//
//! Defines ([`TermInputState`], [`TermParsed`], [`TermParsedCsi`], [`TermReply`]).
//

use crate::{_impl_init, EventKind, EventWindow, Key, Position2, is, slice, unwrap, whilst};

crate::test_size_of!(TermInputState = 18 | 144);
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

crate::test_size_of!(TermParsed = 32 | 256);
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

crate::test_size_of!(TermParsedCsi = 8 | 64);
/// Const-safe CSI parser result.
///
/// Keeps CSI dispatch free of drop-bearing event types
/// until the final conversion to [`TermParsed`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TermParsedCsi {
    /// No CSI match; continue with the next parser layer.
    Continue,
    /// The sequence was handled, but produced no user-facing event.
    Pending,
    /// A keyboard event.
    Key(Key),
    /// Terminal focus was gained.
    FocusGained,
    /// Terminal focus was lost.
    FocusLost,
    /// An internal terminal reply.
    Reply(TermReply),
    /// A complete but unsupported CSI sequence.
    Unknown,
}
impl TermParsedCsi {
    /// Converts this handled CSI result into the general parser result.
    ///
    /// `Continue` maps to `Unknown`; callers should normally handle it before conversion.
    pub(super) const fn to_term_parsed(self) -> TermParsed {
        match self {
            TermParsedCsi::Continue => TermParsed::Unknown,
            TermParsedCsi::Pending => TermParsed::Pending,
            TermParsedCsi::Key(key) => TermParsed::Event(super::TermInputParser::key(key)),
            TermParsedCsi::FocusGained => {
                TermParsed::Event(EventKind::Window(EventWindow::FocusGained))
            }
            TermParsedCsi::FocusLost => {
                TermParsed::Event(EventKind::Window(EventWindow::FocusLost))
            }
            TermParsedCsi::Reply(reply) => TermParsed::Reply(reply),
            TermParsedCsi::Unknown => TermParsed::Unknown,
        }
    }
}

crate::test_size_of!(TermReply = 6 | 48);
/// Terminal reply parsed from the input stream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
