// devela::ui::event::key::state
//
//! Defines [`KeyState`].
//

use crate::ConstInit;
#[cfg(all(feature = "js", not(windows)))]
use crate::{WebEventKind, is};

#[doc = crate::_tags!(interaction)]
/// Represents the state of a [`Key`][crate::Key].
#[doc = crate::_doc_location!("ui/event")]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum KeyState {
    /// The key was pressed.
    #[default]
    Press,
    /// The key was released.
    Release,
    /// The key was repeated.
    Repeat,
}
impl ConstInit for KeyState {
    const INIT: Self = Self::Press;
}

#[cfg(all(feature = "js", not(windows)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "js")))]
impl KeyState {
    /// Converts a `WebEventKind` to `KeyState`, if applicable.
    // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/repeat
    #[must_use]
    pub const fn from_js(from: WebEventKind, repeat: bool) -> Option<KeyState> {
        match from {
            WebEventKind::KeyDown => Some(is![repeat; KeyState::Repeat; KeyState::Press]),
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
            KeyState::Repeat => WebEventKind::KeyDown,
        }
    }
}
