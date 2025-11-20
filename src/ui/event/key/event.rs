// devela::ui::event::key::event
//
//! Defines [`EventKey`], [`EventKeyFfi`], [`KeyState`].
//
// TOC
// - struct EventKey
// - struct KeyState
// - ffi
//   - struct EventKeyFfi
//   - impls

#[cfg(all(feature = "js", not(windows)))]
use crate::WebEventKind;
use crate::{EventTimestamp, Key, KeyMods};

/// Represents a keyboard event.
///
#[doc = "See also [`EventKeyFfi`]."]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventKey {
    /// The key representing the human-readable code.
    ///
    /// This corresponds to X11's keysym.
    pub semantic: Key,
    /// The key representing the hardware scan code.
    ///
    /// This corresponds to X11's scancode XKB mapped.
    pub physical: Key,
    /// The state of the key (pressed or released).
    pub state: KeyState,
    /// The active modifiers of the key (e.g., Shift, Ctrl).
    pub mods: KeyMods,
    /// The time stamp of when the event occurred.
    pub timestamp: Option<EventTimestamp>,
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

#[cfg(ffi··)]
#[cfg_attr(nightly_doc, doc(cfg(ffi··)))]
mod ffi {
    use super::*;
    use crate::{ConstInit, KeyFfi, f32bits};

    #[doc = crate::_TAG_FFI!()]
    /// An FFI-safe version of [`EventKey`].
    #[repr(C)]
    #[allow(missing_docs)]
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EventKeyFfi {
        #[doc = crate::_TAG_FFI!()]
        /// The key representing the human-readable code.
        pub semantic: KeyFfi,
        #[doc = crate::_TAG_FFI!()]
        /// The key representing the hardware scan code.
        pub physical: KeyFfi,
        /// The state of the key (pressed or released).
        pub state: KeyState,
        /// The active modifiers of the key (e.g., Shift, Ctrl).
        pub mods: KeyMods,
        #[doc = crate::_TAG_FFI!()]
        /// The time stamp of when the event occurred.
        pub timestamp: f32bits,
    }

    impl EventKey {
        /// Converts `EventKey` to `EventKeyFfi`.
        pub const fn to_ffi(&self) -> EventKeyFfi {
            EventKeyFfi {
                semantic: self.semantic.to_ffi(),
                physical: self.physical.to_ffi(),
                state: self.state,
                mods: self.mods,
                timestamp: if let Some(t) = self.timestamp {
                    t.get_non_niche()
                } else {
                    f32bits::INIT
                },
            }
        }
        /// Converts `EventKeyFfi` to `EventKey`.
        pub const fn from_ffi(from: &EventKeyFfi) -> EventKey {
            EventKey {
                semantic: Key::from_ffi(from.semantic),
                physical: Key::from_ffi(from.physical),
                state: from.state,
                mods: from.mods,
                timestamp: Some(EventTimestamp::from_non_niche(from.timestamp)),
            }
        }
    }
    crate::items! {
        impl From<&EventKey> for EventKeyFfi { fn from(e: &EventKey) -> Self { EventKey::to_ffi(e) } }
        impl From<&EventKeyFfi> for EventKey { fn from(e: &EventKeyFfi) -> Self { Self::from_ffi(e) } }
        impl From<EventKey> for EventKeyFfi { fn from(e: EventKey) -> Self { EventKey::to_ffi(&e) } }
        impl From<EventKeyFfi> for EventKey { fn from(e: EventKeyFfi) -> Self { Self::from_ffi(&e) } }
    }

    #[cfg(all(feature = "js", not(windows)))]
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
}
#[cfg(ffi··)]
pub use ffi::*;
