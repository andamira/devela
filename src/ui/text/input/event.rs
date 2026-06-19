// devela/src/ui/text/input/event.rs
//
//! Event mappings for text input.
//

use crate::{EventKey, Key, KeyMods, KeyState, TextInputAction, is};
use KeyState::{Press, Release, Repeat};

#[doc = crate::_tags!(ui text event)]
/// Preset key mapping for [`TextInputAction`].
#[doc = crate::_doc_meta!{
    location("ui/text"),
    test_size_of(TextInputKeymapPreset = 1|8; niche Option),
}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TextInputKeymapPreset {
    /// Platform-neutral single-line editing keys.
    ///
    /// Supported mappings:
    /// - text keys: insert their Unicode scalar.
    /// - `Space`: insert a space.
    /// - `Backspace`: delete before the cursor.
    /// - `Delete`: delete at the cursor.
    /// - `Left`: move left.
    /// - `Right`: move right.
    /// - `Home`: move to start.
    /// - `End`: move to end.
    /// - `Enter`: accept.
    /// - `Escape`: cancel.
    /// - `Ctrl-C`: cancel.
    ///
    /// Key releases are ignored.
    /// Text insertion is disabled while `Control`, `Alt`, or `Super` are active.
    /// Shifted text is accepted through the event's semantic key.
    #[default]
    Default,

    /// Basic Emacs-style shortcuts.
    ///
    /// Supported shortcuts:
    /// - `Ctrl-A`: move to start.
    /// - `Ctrl-E`: move to end.
    /// - `Ctrl-B`: move left.
    /// - `Ctrl-F`: move right.
    /// - `Ctrl-D`: delete character at cursor.
    /// - `Ctrl-H`: delete character before cursor.
    /// - `Ctrl-C`: cancel.
    /// - `Ctrl-G`: cancel.
    ///
    /// This preset falls back to [`Default`](Self::Default) for plain text,
    /// navigation keys, `Enter`, `Escape`, `Backspace`, and `Delete`.
    ///
    /// Word movement, kill/yank, history navigation, and mark/selection
    /// are not included yet.
    Emacs,
}

#[doc = crate::_tags!(ui text event)]
/// Maps platform-agnostic keyboard events to text input actions.
#[doc = crate::_doc_meta!{
    location("ui/text"),
    test_size_of(TextInputKeymap = 1|8; niche Option),
}]
/// Backends should translate native keyboard events into [`EventKey`] first.
/// This keeps text editing independent of terminal, desktop, web, or mobile event APIs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TextInputKeymap {
    /// The preset mapping to use.
    pub preset: TextInputKeymapPreset,
}

impl TextInputKeymap {
    /// Default platform-neutral text input keymap.
    pub const DEFAULT: Self = Self { preset: TextInputKeymapPreset::Default };

    /// Basic Emacs-style text input keymap.
    pub const EMACS: Self = Self { preset: TextInputKeymapPreset::Emacs };

    /// Creates a keymap from a preset.
    #[must_use]
    pub const fn new(preset: TextInputKeymapPreset) -> Self {
        Self { preset }
    }
    /// Maps a key event to a text input action.
    #[must_use]
    pub const fn map_key(self, key: &EventKey) -> Option<TextInputAction> {
        is! { !is_press_like(key.state), return None }
        match self.preset {
            TextInputKeymapPreset::Default => map_default(key),
            TextInputKeymapPreset::Emacs => {
                if let Some(action) = map_emacs(key) {
                    Some(action)
                } else {
                    map_default(key)
                }
            }
        }
    }
}

impl TextInputAction {
    /// Maps a platform-agnostic key event using the default keymap.
    #[must_use]
    pub const fn from_key_event(key: &EventKey) -> Option<Self> {
        TextInputKeymap::DEFAULT.map_key(key)
    }
    /// Maps a platform-agnostic key event using `keymap`.
    #[must_use]
    pub const fn from_key_event_with(key: &EventKey, keymap: TextInputKeymap) -> Option<Self> {
        keymap.map_key(key)
    }
}

/* mappings */

const fn map_default(key: &EventKey) -> Option<TextInputAction> {
    let mods = key.mods;
    if mods.has_control() && !mods.has_alt() && !mods.has_super() {
        return map_default_control(key.semantic);
    }
    if accepts_text(mods) {
        match key.semantic {
            Key::Char(c) => return Some(TextInputAction::Insert(c)),
            Key::Space => return Some(TextInputAction::Insert(' ')),
            _ => {}
        }
    }
    match key.semantic {
        Key::Backspace => Some(TextInputAction::Backspace),
        Key::Delete => Some(TextInputAction::Delete),
        //
        Key::Left => Some(TextInputAction::MoveLeft),
        Key::Right => Some(TextInputAction::MoveRight),
        Key::Home => Some(TextInputAction::MoveStart),
        Key::End => Some(TextInputAction::MoveEnd),
        //
        Key::Enter => Some(TextInputAction::Accept),
        Key::Escape => Some(TextInputAction::Cancel),
        //
        _ => None,
    }
}
crate::items! {
    const fn map_default_control(key: Key) -> Option<TextInputAction> {
        if is_key_c(key) { Some(TextInputAction::Cancel) } else { None }
    }
    const fn map_emacs(key: &EventKey) -> Option<TextInputAction> {
        let mods = key.mods;
        if !(mods.has_control() && !mods.has_alt() && !mods.has_super()) { return None; }
        let key = key.semantic;
        if is_key_a(key) { Some(TextInputAction::MoveStart) }
        else if is_key_e(key) { Some(TextInputAction::MoveEnd) }
        else if is_key_b(key) { Some(TextInputAction::MoveLeft) }
        else if is_key_f(key) { Some(TextInputAction::MoveRight) }
        else if is_key_d(key) { Some(TextInputAction::Delete) }
        else if is_key_h(key) { Some(TextInputAction::Backspace) }
        else if is_key_c(key) || is_key_g(key) { Some(TextInputAction::Cancel) }
        else { None }
    }

    /* helpers */

    const fn is_press_like(state: KeyState) -> bool {
        match state { Press | Repeat => true, Release => false }
    }
    const fn accepts_text(mods: KeyMods) -> bool {
        !mods.has_control() && !mods.has_alt() && !mods.has_super()
    }
    const fn is_key_a(key: Key) -> bool { matches!(key, Key::A | Key::Char('a' | 'A')) }
    const fn is_key_b(key: Key) -> bool { matches!(key, Key::B | Key::Char('b' | 'B')) }
    const fn is_key_c(key: Key) -> bool { matches!(key, Key::C | Key::Char('c' | 'C')) }
    const fn is_key_d(key: Key) -> bool { matches!(key, Key::D | Key::Char('d' | 'D')) }
    const fn is_key_e(key: Key) -> bool { matches!(key, Key::E | Key::Char('e' | 'E')) }
    const fn is_key_f(key: Key) -> bool { matches!(key, Key::F | Key::Char('f' | 'F')) }
    const fn is_key_g(key: Key) -> bool { matches!(key, Key::G | Key::Char('g' | 'G')) }
    const fn is_key_h(key: Key) -> bool { matches!(key, Key::H | Key::Char('h' | 'H')) }
}
