// devela::ui::event::key::key
//
//! Defines [`Key`].
//

use crate::{KeyAlpha, KeyMedia, KeyMod, KeyPad};

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
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Key {
    /// Unknown key code (default).
    // mapped to:
    // - crossterm: Null,
    #[default]
    Unknown,

    /* common control keys */
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Tab key.
    Tab,
    // NOTE crossterm returns this from Shift + Tab, but we deconstruct it.
    // BackTab,
    /// Escape key.
    Escape,

    ///
    // NOTE: from crossterm this is received as ' ' character, but we convert it.
    Space,

    /* navigation keys */
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

    /* editing keys */
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,

    /// A function key.
    ///
    /// - Normal: F1-F12
    /// - +Shift: F13-F24
    /// - +Control: F25-F36
    /// - +Shift+Control: F37-F48
    F(u8),

    /* lock keys */
    /// Caps Lock key.
    CapsLock,
    /// Scroll Lock key.
    ScrollLock,
    /// Num Lock key.
    NumLock,

    /* Special system keys */
    /// Print Screen key.
    PrintScreen,
    /// Pause key.
    Pause,
    /// Menu key.
    Menu,

    /// An alphanumeric key (A-Z, 0-9).
    Alpha(KeyAlpha),

    /// A Unicode character (text input, international layouts, fallback).
    Char(char),

    /// A keypad key.
    Pad(KeyPad),
    // - https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.KeypadBegin
    // - WAIT; https://github.com/crossterm-rs/crossterm/issues/860
    // KeypadBegin,
    /// A multimedia key.
    Media(KeyMedia),
    /// A modifer key.
    Mod(KeyMod),
}
#[allow(non_upper_case_globals)]
impl Key {
    /// Alias of [`Escape`][Key::Escape].
    pub const Esc: Key = Key::Escape;
    /// Alias of [`Insert`][Key::Insert].
    pub const Ins: Key = Key::Insert;
    /// Alias of [`Delete`][Key::Delete].
    pub const Del: Key = Key::Delete;
}

crate::sf! {
    impl From<KeyAlpha> for Key { fn from(code: KeyAlpha) -> Key { Key::Alpha(code) } }
    impl From<KeyMedia> for Key { fn from(code: KeyMedia) -> Key { Key::Media(code) } }
    impl From<KeyMod> for Key { fn from(code: KeyMod) -> Key { Key::Mod(code) } }
    impl From<KeyPad> for Key { fn from(code: KeyPad) -> Key { Key::Pad(code) } }
}
