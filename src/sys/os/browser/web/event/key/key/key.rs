// devela/src/sys/os/browser/web/event/key/key/key.rs
//
//! Defines [`WebEventKey`].
//

use crate::{EventKey, EventKind};
#[cfg(feature = "time")]
use crate::{EventKindTimed, EventTimestamp, JsInstant, Timed, is};
use crate::{KeyFfi, KeyMods, KeyState};
crate::_use! { basic::from_utf8 }

#[doc = crate::_tags!(event web)]
/// A web API keyboard event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    #[cfg(all(feature = "time", target_pointer_width = "64"))]
    test_size_of(WebEventKey = 32|256; niche Option),
    #[cfg(all(feature = "time", target_pointer_width = "32"))]
    test_size_of(WebEventKey = 28|224; niche Option),
    #[cfg(not(feature = "time"))]
    test_size_of(WebEventKey = 20|160; niche Option),
}]
///
/// Represents a JavaScript `KeyboardEvent` in a callback-friendly form.
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
    #[cfg(feature = "time")]
    pub timestamp: JsInstant, // 8 bytes
}

impl WebEventKey {
    /// Returns a new keyboard event.
    pub const fn new(
        semantic: KeyFfi,
        physical: KeyFfi,
        mods: KeyMods,
        state: KeyState,
        #[cfg(feature = "time")] timestamp: JsInstant,
    ) -> Self {
        Self {
            semantic,
            physical,
            mods,
            state,
            #[cfg(feature = "time")]
            timestamp,
        }
    }
}
impl WebEventKey {
    /// Converts `WebEventKey` to a normalized `EventKey`.
    pub const fn to_event_key(self) -> EventKey {
        EventKey::new(self.semantic.to_key(), self.physical.to_key(), self.mods, self.state)
    }
    /// Converts `WebEventKey` to `EventKind`.
    pub const fn to_event_kind(self) -> EventKind {
        EventKind::Key(self.to_event_key())
    }
    /// Converts a normalized `EventKey` to `WebEventKey`.
    pub const fn from_event_key(from: EventKey) -> Self {
        Self {
            semantic: from.semantic.to_ffi(),
            physical: from.physical.to_ffi(),
            mods: from.mods,
            state: from.state,
            #[cfg(feature = "time")]
            timestamp: JsInstant::ZERO,
        }
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl WebEventKey {
    /// Converts `WebEventKey` to `EventKindTimed`.
    pub const fn to_event_kind_timed(self) -> EventKindTimed {
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(self.to_event_kind(), timestamp)
    }
    /// Converts a timed normalized `EventKey` to `WebEventKey`.
    pub const fn from_event_key_timed(from: Timed<EventKey, Option<EventTimestamp>>) -> Self {
        let Timed { value, time } = from;
        let timestamp = is![let Some(t) = time, t.to_js(), JsInstant::ZERO];
        Self { timestamp, ..Self::from_event_key(value) }
    }
}
impl From<WebEventKey> for EventKey {
    fn from(from: WebEventKey) -> Self {
        from.to_event_key()
    }
}
impl From<WebEventKey> for EventKind {
    fn from(from: WebEventKey) -> Self {
        from.to_event_kind()
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<WebEventKey> for EventKindTimed {
    fn from(from: WebEventKey) -> Self {
        from.to_event_kind_timed()
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<Timed<EventKey, Option<EventTimestamp>>> for WebEventKey {
    fn from(from: Timed<EventKey, Option<EventTimestamp>>) -> Self {
        Self::from_event_key_timed(from)
    }
}
