// devela/src/sys/os/browser/web/event/mouse.rs
//
//! Defines [`WebEventMouse`].
//

use crate::impl_trait;
use crate::{EventButton, EventButtons, KeyMods, WebEventKind};
#[cfg(feature = "time")]
use crate::{
    EventButtonState, EventKind, EventKindTimed, EventMouse, EventTimestamp, JsInstant, Timed, is,
};
use crate::{JsNumFmt, js_number};

#[doc = crate::_tags!(event web)]
/// A web API Mouse Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    #[cfg(feature = "time")]
    test_size_of(WebEventMouse = 32|256; niche Option),
    #[cfg(not(feature = "time"))]
    test_size_of(WebEventMouse = 24|192; niche Option),
}]
///
/// Represents a JavaScript mouse event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent>
///
/// # Compatibility
///
/// Browser Back/Forward mouse buttons are backend-dependent.
///
/// Chromium exposes them as DOM mouse events (`button` 3/4, `buttons` 8/16).
/// Firefox currently may consume them for history navigation before the page
/// receives `mousedown`/`mouseup`.
//
// WAIT: [firefox-back-forward-buttons](https://bugzilla.mozilla.org/show_bug.cgi?id=1933746)
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct WebEventMouse {
    /// The X-coordinate of the mouse event relative to the viewport.
    pub x: js_number, // 8 bytes
    /// The Y-coordinate of the mouse event relative to the viewport.
    pub y: js_number, // 8 bytes

    /// Raw DOM-compatible `button` value.
    ///
    /// Uses `255` to represent DOM `button = -1`, meaning no button changed.
    pub button: u8, // 1 byte
    /// Raw DOM `buttons` bitmask for buttons currently held down.
    pub buttons: u8, // 1 byte
    /// A bitmask of active keyboard modifiers during the mouse event.
    pub mods: KeyMods, // 2 bytes
    /// The type of mouse event (Click, MouseDown, MouseMove, etc.).
    pub etype: WebEventKind, // 4 bytes

    /// The JavaScript event timestamp.
    #[cfg(feature = "time")]
    pub timestamp: JsInstant, // 8 bytes
}
impl WebEventMouse {
    /// Returns a new [`WebEventMouse`].
    pub const fn new(
        x: js_number,
        y: js_number,
        button: u8,
        buttons: u8,
        mods: KeyMods,
        etype: WebEventKind,
        #[cfg(feature = "time")] timestamp: JsInstant,
    ) -> Self {
        Self {
            x,
            y,
            button,
            buttons,
            mods,
            etype,
            #[cfg(feature = "time")]
            timestamp,
        }
    }
}
impl_trait! { fmt::Debug for WebEventMouse |self, f| {
    let mut d = f.debug_struct("WebEventMouse");
    d.field("x", &JsNumFmt::<2>(self.x))
        .field("y", &JsNumFmt::<2>(self.y))
        .field("button", &self.button())
        .field("buttons", &EventButtons::from_bits(self.buttons))
        .field("mods", &self.mods)
        .field("etype", &self.etype);
    #[cfg(feature = "time")]
    d.field("timestamp", &self.timestamp);
    d.finish()
}}
impl WebEventMouse {
    /// Returns the normalized triggering button, or `None` when no button changed.
    pub const fn button(self) -> Option<EventButton> {
        EventButton::from_web(self.button)
    }
    /// Returns the raw DOM-compatible `button` value.
    ///
    /// Returns `255` for DOM `button = -1`, meaning no button changed.
    pub const fn web_button(self) -> u8 {
        self.button
    }
    /// Returns the normalized held-button set.
    pub const fn buttons(self) -> EventButtons {
        EventButtons::from_bits(self.buttons)
    }
    /// Returns the raw DOM `buttons` bitmask.
    pub const fn web_buttons(self) -> u8 {
        self.buttons
    }
}

#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl WebEventMouse {
    /// Converts `WebEventMouse` to `EventKindTimed`.
    pub const fn to_kind_timed(self) -> EventKindTimed {
        let kind = EventKind::Mouse(EventMouse {
            x: self.x as i32,
            y: self.y as i32,
            button: EventButton::from_web(self.button),
            state: EventButtonState::from_web(self.etype),
            buttons: EventButtons::from_bits(self.buttons),
            mods: self.mods,
        });
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(kind, timestamp)
    }
    /// Converts a timed normalized `EventMouse` back to `WebEventMouse`.
    pub const fn from_event_mouse_timed(
        from: Timed<EventMouse, Option<EventTimestamp>>,
    ) -> WebEventMouse {
        let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
        WebEventMouse {
            x: from.value.x as js_number,
            y: from.value.y as js_number,
            button: is![let Some(b) = from.value.button, b.to_web(), 255],
            buttons: from.value.buttons.bits(), // already a bitmask, directly compatible
            mods: from.value.mods,
            etype: from.value.state.to_web_as_mouse(),
            timestamp,
        }
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<WebEventMouse> for EventKindTimed {
    fn from(from: WebEventMouse) -> Self {
        from.to_kind_timed()
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<Timed<EventMouse, Option<EventTimestamp>>> for WebEventMouse {
    fn from(from: Timed<EventMouse, Option<EventTimestamp>>) -> Self {
        Self::from_event_mouse_timed(from)
    }
}
