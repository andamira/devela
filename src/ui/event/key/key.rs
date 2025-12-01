// devela::ui::event::key::key
//
//! Defines [`Key`].
//

use crate::{ConstInit, KeyMedia, KeyMod, KeyPad};

/// Keyboard codes, used in [`EventKey`][crate::EventKey].
///
#[doc = "See also [`KeyFfi`][super::KeyFfi]."]
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Code.html
// - https://docs.rs/notcurses/latest/notcurses/struct.Code.html
//   - https://github.com/dankamongmen/notcurses/blob/b0f19f9f296bed44ee2ca69b0cbff1b2b29902f0/src/lib/in.c#L180
// - https://man.archlinux.org/man/terminfo.5.en
// - https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Keycode.html (235)
//   - https://docs.rs/sdl2/latest/sdl2/keyboard/enum.Scancode.html (241)
#[rustfmt::skip]
#[non_exhaustive]
#[allow(missing_docs, reason = "alphanumerics")]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Key {
    /* common control keys
    */
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Tab key.
    Tab,
    // Note crossterm returns this from Shift + Tab, but we deconstruct it.
    // BackTab,
    /// Escape key.
    Escape,

    /// Space bar key.
    // Note: from crossterm this is received as ' ' character, but we convert it.
    Space,

    /* navigation keys
    */
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,

    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,

    /* editing keys
    */
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,

    /* lock keys
    */
    /// Caps Lock key.
    CapsLock,
    /// Scroll Lock key.
    ScrollLock,
    /// Num Lock key.
    NumLock,

    /* Special system keys
    */
    /// Print Screen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,

    /// A keypad key.
    Pad(KeyPad),

    // - https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.KeypadBegin
    // - WAIT; https://github.com/crossterm-rs/crossterm/issues/860
    // KeypadBegin,

    /// A modifer key.
    Mod(KeyMod),
    /// A multimedia key.
    Media(KeyMedia),

    /// A function key.
    ///
    /// - Normal: F1-F12
    /// - +Shift: F13-F24
    /// - +Control: F25-F36
    /// - +Shift+Control: F37-F48
    Fn(u8),

    /// A Unicode character (text input, international layouts, fallback).
    Char(char),

    /* ------------------------------------------------------
      US ANSI physical layout keys (letters/digits/symbols)
      These describe physical key positions ("code" layer).
    --------------------------------------------------------- */

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

    /// Physical keys that have no universal name.
    Scancode(u16),

    /// Unknown key code (default).
    // mapped to:
    // - crossterm: Null,
    #[default]
    Unknown,
}
impl ConstInit for Key {
    const INIT: Self = Self::Unknown;
}

/* impl aliases */

#[allow(non_upper_case_globals)]
impl Key {
    /// Alias of [`Escape`][Key::Escape].
    pub const Esc: Key = Key::Escape;
    /// Alias of [`Insert`][Key::Insert].
    pub const Ins: Key = Key::Insert;
    /// Alias of [`Delete`][Key::Delete].
    pub const Del: Key = Key::Delete;
}

/// Spanish-layout aliases for the US ANSI physical layout keys.
///
/// Each alias names the symbol normally appearing on Spanish keyboards in that physical position.
///
/// The listed characters reflect the typical unshifted and shifted outputs on standard ES layouts.
#[allow(non_upper_case_globals)]
impl Key {
    /// Spanish key for `º` and `ª`.
    pub const EsOrdinal: Self = Self::Backquote;
    /// Spanish key for `'` and `?`.
    pub const EsApostrophe: Self = Self::Minus;
    /// Spanish key for `¡` and `¿`.
    pub const EsInvertedExclamationMark: Self = Self::Equal;

    /// Spanish key for `\`` and `^`.
    pub const EsBacktick: Self = Self::BracketLeft;
    /// Spanish key for `+` and `*`.
    pub const EsPlus: Self = Self::BracketRight;

    /// Spanish key for `ñ` and `Ñ`.
    pub const EsEnye: Self = Self::Semicolon;
    /// Spanish key for `´` and `¨`.
    pub const EsAccent: Self = Self::Quote;
    /// Spanish key for `ç` and `Ç`.
    pub const EsCedilla: Self = Self::Backslash;

    /// Spanish key for `<` and `>`.
    pub const EsBracket: Self = Self::IntlBackslash;
    /// Spanish key for `,` and `;`.
    pub const EsComma: Self = Self::Comma;
    /// Spanish key for `.` and `:`.
    pub const EsDot: Self = Self::Dot;
    /// Spanish key for `-` and `_`.
    pub const EsMinus: Self = Self::Slash;
}

/* impl conversions */

crate::sf! {
    impl From<KeyMedia> for Key { fn from(code: KeyMedia) -> Key { Key::Media(code) } }
    impl From<KeyMod> for Key { fn from(code: KeyMod) -> Key { Key::Mod(code) } }
    impl From<KeyPad> for Key { fn from(code: KeyPad) -> Key { Key::Pad(code) } }
}
