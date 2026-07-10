// devela/src/sys/os/browser/web/event/wheel.rs
//
//! Defines [`WebEventWheel`].
//

use crate::impl_trait;
use crate::{EventButtons, EventKind, EventWheel, EventWheelUnit, KeyMods};
#[cfg(feature = "time")]
use crate::{EventKindTimed, EventTimestamp, JsInstant, Timed, is};
use crate::{JsNumFmt, js_number};

#[doc = crate::_tags!(event web)]
/// A web API Wheel Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    #[cfg(feature = "time")]
    test_size_of(WebEventWheel = 48|384; niche Option),
    #[cfg(not(feature = "time"))]
    test_size_of(WebEventWheel = 40|320; niche Option),
}]
/// Represents a JavaScript wheel event with browser-native deltas and unit.
///
/// # Notes
/// - `delta_x` and `delta_y` preserve the browser-provided wheel deltas.
/// - `unit` preserves the browser-reported delta unit.
/// - `buttons` preserves the DOM buttons bitmask as observed during the wheel event.
/// - `x` and `y` are viewport-relative coordinates.
///
/// See also [`EventWheel`], [`EventWheelUnit`].
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent>
//
// WAIT: [Wheel-scrolling missing 1st event](https://bugzilla.mozilla.org/show_bug.cgi?id=1969373)
// WAIT: […stops working with XInput2](https://bugzilla.mozilla.org/show_bug.cgi?id=1182700)
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct WebEventWheel {
    /// The X-coordinate of the wheel event relative to the viewport.
    pub x: js_number, // 8 bytes
    /// The Y-coordinate of the wheel event relative to the viewport.
    pub y: js_number, // 8 bytes
    /// The horizontal wheel delta reported by the browser.
    pub delta_x: js_number, // 8 bytes
    /// The vertical wheel delta reported by the browser.
    pub delta_y: js_number, // 8 bytes

    /// A bitmask of currently held buttons during the wheel event.
    pub buttons: u8, // 1 byte
    /// A bitmask of active keyboard modifiers during the mouse event.
    pub mods: KeyMods, // 2 bytes
    /// The browser-reported unit associated to `delta_x` and `delta_y`.
    pub unit: EventWheelUnit, // 1 byte
    // 4 byte gap
    /// The JavaScript event timestamp.
    #[cfg(feature = "time")]
    pub timestamp: JsInstant, // 8 bytes
}
impl_trait! { fmt::Debug for WebEventWheel |self, f| {
    let mut d = f.debug_struct("WebEventWheel");
    d.field("x", &JsNumFmt::<2>(self.x))
        .field("y", &JsNumFmt::<2>(self.y))
        .field("delta_x", &JsNumFmt::<2>(self.delta_x))
        .field("delta_y", &JsNumFmt::<2>(self.delta_y))
        .field("buttons", &EventButtons::from_bits(self.buttons))
        .field("mods", &self.mods)
        .field("unit", &self.unit);
    #[cfg(feature = "time")]
    d.field("timestamp", &self.timestamp);
    d.finish()
}}
impl WebEventWheel {
    /// Returns a new [`WebEventWheel`].
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        x: js_number,
        y: js_number,
        delta_x: js_number,
        delta_y: js_number,
        buttons: u8,
        mods: KeyMods,
        unit: EventWheelUnit,
        #[cfg(feature = "time")] timestamp: JsInstant,
    ) -> Self {
        Self {
            x,
            y,
            delta_x,
            delta_y,
            buttons,
            mods,
            unit,
            #[cfg(feature = "time")]
            timestamp,
        }
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

/* conversion */

impl WebEventWheel {
    /// Converts `WebEventWheel` to `EventWheel`.
    pub const fn to_event_wheel(self) -> EventWheel {
        EventWheel {
            delta_x: self.delta_x as i32,
            delta_y: self.delta_y as i32,
            unit: self.unit,
            x: self.x as i32,
            y: self.y as i32,
            buttons: EventButtons::from_bits(self.buttons),
            mods: self.mods,
        }
    }
    /// Converts `WebEventWheel` to `EventKind`.
    pub const fn to_event_kind(self) -> EventKind {
        EventKind::Wheel(self.to_event_wheel())
    }
    /// Converts a normalized `EventWheel` to `WebEventWheel`.
    pub const fn from_event_wheel(from: EventWheel) -> Self {
        Self {
            x: from.x as js_number,
            y: from.y as js_number,
            delta_x: from.delta_x as js_number,
            delta_y: from.delta_y as js_number,
            buttons: from.buttons.bits(),
            mods: from.mods,
            unit: from.unit,
            #[cfg(feature = "time")]
            timestamp: JsInstant::ZERO,
        }
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl WebEventWheel {
    /// Converts `WebEventWheel` to `EventKindTimed`.
    pub const fn to_event_kind_timed(self) -> EventKindTimed {
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(self.to_event_kind(), timestamp)
    }
    /// Converts a timed normalized `EventWheel` back to `WebEventWheel`.
    pub const fn from_event_wheel_timed(from: Timed<EventWheel, Option<EventTimestamp>>) -> Self {
        let Timed { value, time } = from;
        let timestamp = is![let Some(t) = time, t.to_js(), JsInstant::ZERO];
        Self { timestamp, ..Self::from_event_wheel(value) }
    }
}
impl From<WebEventWheel> for EventWheel {
    fn from(from: WebEventWheel) -> Self {
        from.to_event_wheel()
    }
}
impl From<WebEventWheel> for EventKind {
    fn from(from: WebEventWheel) -> Self {
        from.to_event_kind()
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<WebEventWheel> for EventKindTimed {
    fn from(from: WebEventWheel) -> Self {
        from.to_event_kind_timed()
    }
}
#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
impl From<Timed<EventWheel, Option<EventTimestamp>>> for WebEventWheel {
    fn from(from: Timed<EventWheel, Option<EventTimestamp>>) -> Self {
        Self::from_event_wheel_timed(from)
    }
}
