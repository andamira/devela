// devela::ui::event::key::event
//
//! Defines [`EventKey`], [`EventKeyFfi`].
//

use crate::{ConstInit, Key, KeyMods, KeyState};

#[doc = crate::_tags!(event interaction)]
/// Represents a keyboard event.
#[doc = crate::_doc_location!("ui/event")]
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

    /// The active modifiers of the key (e.g., Shift, Ctrl).
    pub mods: KeyMods,

    /// The state of the key (pressed or released).
    pub state: KeyState,
}
impl ConstInit for EventKey {
    const INIT: Self = Self {
        semantic: Key::INIT,
        physical: Key::INIT,
        state: KeyState::INIT,
        mods: KeyMods::INIT,
    };
}

#[cfg(ffi··)]
pub use ffi::*;
#[cfg(ffi··)]
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(ffi··)))]
mod ffi {
    use super::*;
    use crate::KeyFfi;

    #[doc = crate::_tags!(event interaction ffi)]
    /// An FFI-safe version of [`EventKey`].
    #[doc = crate::_doc_location!("ui/event")]
    #[repr(C)]
    #[allow(missing_docs)]
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EventKeyFfi {
        #[doc = crate::_tags!(ffi)]
        /// The key representing the human-readable code.
        pub semantic: KeyFfi,

        #[doc = crate::_tags!(ffi)]
        /// The key representing the hardware scan code.
        pub physical: KeyFfi,

        /// The state of the key (pressed or released).
        pub state: KeyState,

        /// The active modifiers of the key (e.g., Shift, Ctrl).
        pub mods: KeyMods,
    }

    impl EventKey {
        /// Converts `EventKey` to `EventKeyFfi`.
        pub const fn to_ffi(&self) -> EventKeyFfi {
            EventKeyFfi {
                semantic: self.semantic.to_ffi(),
                physical: self.physical.to_ffi(),
                state: self.state,
                mods: self.mods,
            }
        }
        /// Converts `EventKeyFfi` to `EventKey`.
        pub const fn from_ffi(from: &EventKeyFfi) -> EventKey {
            EventKey {
                semantic: Key::from_ffi(from.semantic),
                physical: Key::from_ffi(from.physical),
                state: from.state,
                mods: from.mods,
            }
        }
    }
    impl From<&EventKey> for EventKeyFfi { fn from(e: &EventKey) -> Self { EventKey::to_ffi(e) } }
    impl From<&EventKeyFfi> for EventKey { fn from(e: &EventKeyFfi) -> Self { Self::from_ffi(e) } }
    impl From<EventKey> for EventKeyFfi { fn from(e: EventKey) -> Self { EventKey::to_ffi(&e) } }
    impl From<EventKeyFfi> for EventKey { fn from(e: EventKeyFfi) -> Self { Self::from_ffi(&e) } }
}
