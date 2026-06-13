// devela/src/sys/os/browser/web/event/key/key.rs
//
//! Defines [`WebEventKey`].
//

use crate::{JsInstant, KeyMods, WebEventKind, WebKeyLocation, js_bool, js_uint32};

#[doc = crate::_tags!(event web)]
/// A web API keyboard event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventKey = 24|192),
}]
///
/// Represents a JavaScript `KeyboardEvent` in a compact callback-friendly form.
///
/// `key` is the Unicode scalar value of `KeyboardEvent.key` when it contains
/// exactly one Unicode scalar, or `0` for non-text keys such as `Enter`.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventKey {
    /// Unicode scalar from `KeyboardEvent.key`, or `0` for non-text keys.
    pub key: js_uint32,

    /// Which part of the keyboard produced the event.
    pub location: WebKeyLocation,

    /// Whether this is an auto-repeat keydown.
    pub repeat: js_bool,

    /// Active keyboard modifiers.
    pub mods: KeyMods,

    /// The DOM event kind, usually `KeyDown` or `KeyUp`.
    pub etype: WebEventKind,

    /// The JavaScript event timestamp.
    pub timestamp: JsInstant,
}

impl WebEventKey {
    /// Returns a new keyboard event.
    pub const fn new(
        key: js_uint32,
        location: WebKeyLocation,
        repeat: js_bool,
        mods: KeyMods,
        etype: WebEventKind,
        timestamp: JsInstant,
    ) -> Self {
        Self { key, location, repeat, mods, etype, timestamp }
    }

    /// Returns whether this event carries a text scalar.
    pub const fn has_key_scalar(self) -> bool {
        self.key != 0
    }
}
