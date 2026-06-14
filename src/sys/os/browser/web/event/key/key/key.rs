// devela/src/sys/os/browser/web/event/key/key/key.rs
//
//! Defines [`WebEventKey`].
//

use crate::{EventKey, EventKind, EventKindTimed, EventTimestamp};
use crate::{JsInstant, Timed, is};
use crate::{KeyFfi, KeyMods, KeyState};
crate::_use! { basic::from_utf8 }

#[doc = crate::_tags!(event web)]
/// A web API keyboard event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventKey = 32|256; niche Option),
}]
///
/// Represents a JavaScript `KeyboardEvent` in a compact callback-friendly form.
///
/// `key` is the Unicode scalar value of `KeyboardEvent.key` when it contains
/// exactly one Unicode scalar, or `0` for non-text keys such as `Enter`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventKey {
    /// The key representing the human-readable code.
    ///
    /// This corresponds to the Web API KeyboardEvent's semantic [key].
    ///
    /// [key]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    pub semantic: KeyFfi, // 8 bytes

    /// The key representing the hardware scan code.
    ///
    /// This corresponds to the Web API KeyboardEvent's physical [code].
    ///
    /// [code]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    pub physical: KeyFfi, // 8 bytes

    /// Active keyboard modifiers.
    pub mods: KeyMods, // 2 bytes
    /// The key transition state.
    pub state: KeyState, // 1 byte

    /// The JavaScript event timestamp.
    pub timestamp: JsInstant, // 8 bytes
}

impl WebEventKey {
    /// Returns a new keyboard event.
    pub const fn new(
        semantic: KeyFfi,
        physical: KeyFfi,
        mods: KeyMods,
        state: KeyState,
        timestamp: JsInstant,
    ) -> Self {
        Self { semantic, physical, mods, state, timestamp }
    }

    /// Converts `WebEventKey` to a normalized `EventKey`.
    ///
    /// Returns `None` if the web event kind is not a keyboard event.
    pub const fn to_event_key(self) -> EventKey {
        EventKey::new(self.semantic.to_key(), self.physical.to_key(), self.mods, self.state)
    }

    /// Converts `WebEventKey` to `EventKindTimed`.
    ///
    /// Returns `None` if the web event kind is not a keyboard event.
    pub const fn to_kind_timed(self) -> EventKindTimed {
        EventKindTimed::new(
            EventKind::Key(self.to_event_key()),
            Some(EventTimestamp::from_js(self.timestamp)),
        )
    }

    /// Converts a timed normalized `EventKey` back to `WebEventKey`.
    pub const fn from_event_key_timed(from: Timed<EventKey, Option<EventTimestamp>>) -> Self {
        Self {
            semantic: from.value.semantic.to_ffi(),
            physical: from.value.physical.to_ffi(),
            mods: from.value.mods,
            state: from.value.state,
            timestamp: is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }],
        }
    }
}
impl From<WebEventKey> for EventKindTimed {
    fn from(from: WebEventKey) -> Self {
        from.to_kind_timed()
    }
}
impl From<Timed<EventKey, Option<EventTimestamp>>> for WebEventKey {
    fn from(from: Timed<EventKey, Option<EventTimestamp>>) -> Self {
        Self::from_event_key_timed(from)
    }
}
