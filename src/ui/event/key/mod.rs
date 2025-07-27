// devela::ui::event::key
//
//! Defines [`EventKey`], [`EventKeyFFi`], [`KeyState`].
//
// TOC
// - struct EventKey
// - enum KeyState
// - mod ffi
//   - struct EventKeyFfi
// - tests

use crate::EventTimestamp;
#[cfg(feature = "js")]
use crate::WebEventKind;

mod alpha_pad; // KeyAlpha, KeyPad
mod key; // Key
mod media_mods; // KeyMedia, KeyMod, KeyMods
pub use {alpha_pad::*, key::*, media_mods::*};

#[cfg(ffi··)]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(ffi··)))]
    mod ffi; // EventKeyFfi, KeyFfi
    pub use ffi::*;
}

/// Represents a keyboard event.
///
#[doc = "See also [`EventKeyFfi`]."]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventKey {
    /// The key representing the human-readable code.
    pub semantic: Key,
    /// The key representing the hardware scan code.
    pub physical: Key,
    /// The state of the key (pressed or released).
    pub state: KeyState,
    /// The active modifiers of the key (e.g., Shift, Ctrl).
    pub mods: KeyMods,
    /// The time stamp of when the event occurred.
    pub time_stamp: Option<EventTimestamp>,
}

/// Represents the state of a [`Key`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyState {
    /// The key was pressed.
    #[default]
    Press,
    /// The key was released.
    Release,
}

#[cfg(feature = "js")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyState {
    /// Converts a `WebEventKind` to `KeyState`, if applicable.
    #[must_use]
    pub const fn from_js(from: WebEventKind) -> Option<KeyState> {
        match from {
            WebEventKind::KeyDown => Some(KeyState::Press),
            WebEventKind::KeyUp => Some(KeyState::Release),
            _ => None,
        }
    }
    /// Converts a `KeyState` to `WebEventKind`.
    #[must_use]
    pub const fn to_js(self) -> WebEventKind {
        match self {
            KeyState::Press => WebEventKind::KeyDown,
            KeyState::Release => WebEventKind::KeyUp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[rustfmt::skip]
    fn sizes_of() {
        assert_eq![24, size_of::<EventKey>()];        // 192 bits
        assert_eq![ 8, size_of::<Key>()];             //  64 bits
        #[cfg(ffi··)] crate::items! {
            assert_eq![24, size_of::<EventKeyFfi>()]; // 192 bits
            assert_eq![ 8, size_of::<KeyFfi>()];      //  64 bits
        }
        assert_eq![ 1, size_of::<KeyAlpha>()];        //   8 bits
        assert_eq![ 1, size_of::<KeyMedia>()];        //   8 bits
        assert_eq![ 1, size_of::<KeyMod>()];          //   8 bits
        assert_eq![ 2, size_of::<KeyMods>()];         //  16 bits
        assert_eq![ 1, size_of::<KeyPad>()];          //   8 bits
        assert_eq![ 1, size_of::<KeyState>()];        //   8 bits
    }

    #[test]
    fn key_state_to_js_event() {
        assert_eq!(KeyState::Press.to_js(), WebEventKind::KeyDown);
        assert_eq!(KeyState::Release.to_js(), WebEventKind::KeyUp);
    }
    #[test]
    fn js_event_to_key_state() {
        assert_eq!(KeyState::from_js(WebEventKind::KeyDown), Some(KeyState::Press));
        assert_eq!(KeyState::from_js(WebEventKind::KeyUp), Some(KeyState::Release));
        assert_eq!(KeyState::from_js(WebEventKind::Click), None);
    }
}
