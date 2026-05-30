// devela::sys::os::browser::web::event::pointer
//
//! Defines [`WebEventWheel`].
//

use crate::lang::prog::ffi::js::{JsInstant, js_number};
use crate::ui::event::{
    EventButtons, EventKind, EventKindTimed, EventTimestamp, EventWheel, EventWheelUnit, KeyMods,
};
use crate::{Timed, is};

#[doc = crate::_tags!(event web)]
/// A web API Wheel Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventWheel = 48|384; niche Option),
}]
///
/// Represents a JavaScript wheel event with browser-native deltas and unit.
///
/// This is a raw web-side event carrier.
/// Use [`to_kind_timed`](Self::to_kind_timed) to normalize it into
/// [`EventKindTimed`].
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
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub timestamp: JsInstant, // 8 bytes
}
impl WebEventWheel {
    /// Returns a new [`WebEventWheel`].
    pub const fn new(
        x: js_number,
        y: js_number,
        delta_x: js_number,
        delta_y: js_number,
        buttons: u8,
        mods: KeyMods,
        unit: EventWheelUnit,
        timestamp: JsInstant,
    ) -> Self {
        Self {
            x,
            y,
            delta_x,
            delta_y,
            buttons,
            mods,
            unit,
            timestamp,
        }
    }

    /// Converts `WebEventWheel` to `EventKindTimed`.
    pub const fn to_kind_timed(self) -> EventKindTimed {
        let kind = EventKind::Wheel(EventWheel {
            delta_x: self.delta_x as i32,
            delta_y: self.delta_y as i32,
            unit: self.unit,
            x: self.x as i32,
            y: self.y as i32,
            buttons: EventButtons::from_bits(self.buttons),
            mods: self.mods,
        });
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(kind, timestamp)
    }

    /// Converts a timed normalized `EventWheel` back to `WebEventWheel`.
    pub const fn from_event_wheel_timed(
        from: Timed<EventWheel, Option<EventTimestamp>>,
    ) -> WebEventWheel {
        let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
        WebEventWheel {
            x: from.value.x as js_number,
            y: from.value.y as js_number,
            delta_x: from.value.delta_x as js_number,
            delta_y: from.value.delta_y as js_number,
            buttons: from.value.buttons.bits(),
            mods: from.value.mods,
            unit: from.value.unit,
            timestamp,
        }
    }
}

impl From<WebEventWheel> for EventKindTimed {
    fn from(from: WebEventWheel) -> Self {
        from.to_kind_timed()
    }
}
impl From<Timed<EventWheel, Option<EventTimestamp>>> for WebEventWheel {
    fn from(from: Timed<EventWheel, Option<EventTimestamp>>) -> Self {
        Self::from_event_wheel_timed(from)
    }
}
