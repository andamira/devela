// devela::ui::event::key::key
//
//! Defines [`KeyFfi`].
//

use super::*;
use crate::{ConstInit, unwrap};

#[doc = crate::_tags!(interaction ffi)]
/// An FFI-safe version of [`Key`], used in [`EventKeyFfi`][crate::EventKeyFfi].
// (The main difference is in the Char variant.)
#[doc = crate::_doc_location!("ui/event")]
#[repr(C)]
#[non_exhaustive]
#[allow(missing_docs)] #[rustfmt::skip]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyFfi {
    // common control keys
    Backspace, Enter, Tab, Escape, Space,
    // navigation keys
    Left, Right, Up, Down,
    Home, End, PageUp, PageDown,
    // editing keys
    Delete, Insert,
    // lock keys
    CapsLock, ScrollLock, NumLock,
    // special system kesy
    PrintScreen, Pause, Menu,

    /// A keypad key.
    Pad(KeyPad),
    /// A multimedia key.
    Media(KeyMedia),
    /// A modifier key.
    Mod(KeyMod),

    /// Function keys (F1-F48).
    Fn(u8),

    #[doc = crate::_tags!(ffi)]
    /// A unicode character (FFI-safe, stored as `u32`).
    Char(u32),

    /// A dead key (accent prefix, composition).
    Dead(KeyDead),

    // Letters
    A, B, C, D, E, F, G, H, I, J,
    K, L, M, N, O, P, Q, R, S, T,
    U, V, W, X, Y, Z,

    // Digits (E row)
    Digit1, Digit2, Digit3, Digit4, Digit5,
    Digit6, Digit7, Digit8, Digit9, Digit0,

    // ANSI punctuation / symbol positions
    // E row
    /// Physical key left of '1'
    Backquote,
    /// Physical `-` key.
    Minus,
    /// Physical `=` key.
    Equal,
    // D row
    /// Physical `[` key.
    BracketLeft,
    /// Physical `]` key.
    BracketRight,
    // C (home) row
    /// Physical `;` key.
    Semicolon,
    /// Physical `'` key.
    Quote,
    // B row
    /// Physical `\` key.
    Backslash,
    /// ISO-102 extra key.
    IntlBackslash,
    // A row
    /// Physical `,` key.
    Comma,
    /// Physical `.` key.
    Dot,
    /// Physical `/` key.
    Slash,

    /// A physical scancode key.
    Scancode(u16),

    #[default]
    Unknown,
}
impl ConstInit for KeyFfi {
    const INIT: Self = Self::Unknown;
}

impl Key {
    /// Converts `Key` to `KeyFfi`.
    pub const fn to_ffi(self) -> KeyFfi {
        use {Key as K, KeyFfi as F};
        match self {
            // common control
            K::Backspace => F::Backspace,
            K::Enter => F::Enter,
            K::Tab => F::Tab,
            K::Escape => F::Escape,
            K::Space => F::Space,
            // navigation
            K::Left => F::Left,
            K::Right => F::Right,
            K::Up => F::Up,
            K::Down => F::Down,
            K::Home => F::Home,
            K::End => F::End,
            K::PageUp => F::PageUp,
            K::PageDown => F::PageDown,
            // editing
            K::Delete => F::Delete,
            K::Insert => F::Insert,
            // lock
            K::CapsLock => F::CapsLock,
            K::ScrollLock => F::ScrollLock,
            K::NumLock => F::NumLock,
            // special system
            K::PrintScreen => F::PrintScreen,
            K::Pause => F::Pause,
            K::Menu => F::Menu,
            //
            K::Pad(p) => F::Pad(p),
            K::Mod(m) => F::Mod(m),
            K::Media(m) => F::Media(m),
            K::Fn(f) => F::Fn(f),
            K::Char(c) => F::Char(c as u32), // Convert char to u32
            K::Dead(d) => F::Dead(d),
            // Letters
            K::A => F::A,
            K::B => F::B,
            K::C => F::C,
            K::D => F::D,
            K::E => F::E,
            K::F => F::F,
            K::G => F::G,
            K::H => F::H,
            K::I => F::I,
            K::J => F::J,
            K::K => F::K,
            K::L => F::L,
            K::M => F::M,
            K::N => F::N,
            K::O => F::O,
            K::P => F::P,
            K::Q => F::Q,
            K::R => F::R,
            K::S => F::S,
            K::T => F::T,
            K::U => F::U,
            K::V => F::V,
            K::W => F::W,
            K::X => F::X,
            K::Y => F::Y,
            K::Z => F::Z,
            // Digits (E row)
            K::Digit1 => F::Digit1,
            K::Digit2 => F::Digit2,
            K::Digit3 => F::Digit3,
            K::Digit4 => F::Digit4,
            K::Digit5 => F::Digit5,
            K::Digit6 => F::Digit6,
            K::Digit7 => F::Digit7,
            K::Digit8 => F::Digit8,
            K::Digit9 => F::Digit9,
            K::Digit0 => F::Digit0,

            // ANSI punctuation / symbol positions
            // E row
            K::Backquote => F::Backquote,
            K::Minus => F::Minus,
            K::Equal => F::Equal,
            // D row
            K::BracketLeft => F::BracketLeft,
            K::BracketRight => F::BracketRight,
            // C (home) row
            K::Semicolon => F::Semicolon,
            K::Quote => F::Quote,
            // B row
            K::Backslash => F::Backslash,
            K::IntlBackslash => F::IntlBackslash,
            // A row
            K::Comma => F::Comma,
            K::Dot => F::Dot,
            K::Slash => F::Slash,
            //
            K::Scancode(sc) => F::Scancode(sc),
            K::Unknown => F::Unknown,
        }
    }

    /// Converts `KeyFfi` to `Key`.
    pub const fn from_ffi(from: KeyFfi) -> Key {
        use {Key as K, KeyFfi as F};
        match from {
            // common control
            F::Backspace => K::Backspace,
            F::Enter => K::Enter,
            F::Tab => K::Tab,
            F::Escape => K::Escape,
            F::Space => K::Space,
            // navigation
            F::Left => K::Left,
            F::Right => K::Right,
            F::Up => K::Up,
            F::Down => K::Down,
            F::Home => K::Home,
            F::End => K::End,
            F::PageUp => K::PageUp,
            // editing
            F::PageDown => K::PageDown,
            F::Delete => K::Delete,
            F::Insert => K::Insert,
            // lock
            F::CapsLock => K::CapsLock,
            F::ScrollLock => K::ScrollLock,
            F::NumLock => K::NumLock,
            // special system
            F::PrintScreen => K::PrintScreen,
            F::Pause => K::Pause,
            F::Menu => K::Menu,
            F::Pad(p) => K::Pad(p),
            F::Mod(m) => K::Mod(m),
            F::Media(m) => K::Media(m),
            F::Fn(f) => K::Fn(f),
            F::Char(c) => K::Char(unwrap![some_or char::from_u32(c), char::REPLACEMENT_CHARACTER]),
            F::Dead(d) => K::Dead(d),
            // Letters
            F::A => K::A,
            F::B => K::B,
            F::C => K::C,
            F::D => K::D,
            F::E => K::E,
            F::F => K::F,
            F::G => K::G,
            F::H => K::H,
            F::I => K::I,
            F::J => K::J,
            F::K => K::K,
            F::L => K::L,
            F::M => K::M,
            F::N => K::N,
            F::O => K::O,
            F::P => K::P,
            F::Q => K::Q,
            F::R => K::R,
            F::S => K::S,
            F::T => K::T,
            F::U => K::U,
            F::V => K::V,
            F::W => K::W,
            F::X => K::X,
            F::Y => K::Y,
            F::Z => K::Z,
            // Digits (E row)
            F::Digit1 => K::Digit1,
            F::Digit2 => K::Digit2,
            F::Digit3 => K::Digit3,
            F::Digit4 => K::Digit4,
            F::Digit5 => K::Digit5,
            F::Digit6 => K::Digit6,
            F::Digit7 => K::Digit7,
            F::Digit8 => K::Digit8,
            F::Digit9 => K::Digit9,
            F::Digit0 => K::Digit0,

            // ANSI punctuation / symbol positions
            // E row
            F::Backquote => K::Backquote,
            F::Minus => K::Minus,
            F::Equal => K::Equal,
            // D row
            F::BracketLeft => K::BracketLeft,
            F::BracketRight => K::BracketRight,
            // C (home) row
            F::Semicolon => K::Semicolon,
            F::Quote => K::Quote,
            // B row
            F::Backslash => K::Backslash,
            F::IntlBackslash => K::IntlBackslash,
            // A row
            F::Comma => K::Comma,
            F::Dot => K::Dot,
            F::Slash => K::Slash,
            //
            F::Scancode(sc) => K::Scancode(sc),
            F::Unknown => K::Unknown,
        }
    }
}

crate::items! {
    impl From<Key> for KeyFfi { fn from(k: Key) -> Self { Key::to_ffi(k) } }
    impl From<KeyFfi> for Key { fn from(k: KeyFfi) -> Self { Key::from_ffi(k) } }
}
