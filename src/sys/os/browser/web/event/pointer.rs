// devela/src/sys/os/browser/web/event/pointer.rs
//
//! Defines [`WebEventPointer`].
//
// TOC
// struct WebEventPointer
// impl WebEventPointer
// impl EventPointerKind
// struct WebPointerCode

use crate::lang::prog::ffi::js::{JsInstant, JsNumFmt, js_int32, js_number};
use crate::{
    EventButton, EventButtonState, EventButtons, EventKind, EventKindTimed, EventPointer,
    EventPointerKind, EventTimestamp, KeyMods, Timed, WebEventKind, f32bits_niche,
};
use crate::{impl_trait, is};

#[doc = crate::_tags!(event web)]
/// A web API Pointer Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventPointer = 48|384; niche Option),
}]
///
/// Represents a JavaScript pointer event containing relevant properties.
///
/// - <https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent>
///
/// # Compatibility
/// Some Firefox/Linux configurations may report stylus input as
/// `pointerType = "mouse"` with `pressure = 0.5` and zero tilt.
///
/// That is the browser/platform fallback stream.
//
// related to MOZ_USE_XINPUT2 in X11:
// DONE? [pen recognized as mouse in firefox](https://bugzilla.mozilla.org/show_bug.cgi?id=1606832)
// WAIT: - [Use XInput2 manager on GTK3](https://bugzilla.mozilla.org/show_bug.cgi?id=1207700)
// WAIT:   - [repeat always false in X11](https://bugzilla.mozilla.org/show_bug.cgi?id=1594003)
//
// WAIT: [META stylus/pen handling issues](https://bugzilla.mozilla.org/show_bug.cgi?id=1631377)
// WAIT: - [spurious pen pointer evens](https://bugzilla.mozilla.org/show_bug.cgi?id=1906968)
// WAIT: - […incorrect pointerType](https://bugzilla.mozilla.org/show_bug.cgi?id=1449660)
// WAIT: - […events broken for pen devices](https://bugzilla.mozilla.org/show_bug.cgi?id=1487509)
// WAIT: - […impl has too many errors](https://bugzilla.mozilla.org/show_bug.cgi?id=1583519)
// WAIT: - […button state incorrect value](https://bugzilla.mozilla.org/show_bug.cgi?id=1425384)
// WAIT: - [sActivePointersIds gets 0](https://bugzilla.mozilla.org/show_bug.cgi?id=1904865)
// WAIT: - [pen input is misrepresented](https://bugzilla.mozilla.org/show_bug.cgi?id=1953665)
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct WebEventPointer {
    //: 24 bytes
    /// The X-coordinate of the pointer event relative to the viewport.
    pub x: js_number, // 8 bytes
    /// The Y-coordinate of the pointer event relative to the viewport.
    pub y: js_number, // 8 bytes
    /// The pressure applied to the pointer (0.0 to 1.0 for most devices).
    pub pressure: js_number, // 8 bytes

    //: 8 bytes
    /// Unique identifier for the pointer device.
    pub id: js_int32, // 4 bytes
    /// The tilt of the stylus along the X-axis (-90° to 90°).
    pub tilt_x: i8, // 1 bytes
    /// The tilt of the stylus along the Y-axis (-90° to 90°).
    pub tilt_y: i8, // 1 bytes
    /// The rotation of the stylus around its own axis (0° to 359°).
    pub twist: u16, // 2 bytes, 9 bits

    //: 16 bytes
    // Encodes pointer kind and triggering button in one byte.
    // This is a compact internal representation, not the raw DOM shape.
    code: WebPointerCode, // 1 byte
    /// A bitmask of buttons currently being held down during the pointer event.
    pub buttons: u8, // 1 byte
    /// A bitmask of active keyboard modifiers during the pointer event.
    pub mods: KeyMods, // 2 bytes
    /// The type of pointer event (PointerDown, PointerMove, etc.).
    pub etype: WebEventKind, // 4 bytes
    /// The JavaScript event timestamp.
    pub timestamp: JsInstant, // 8 bytes
}
impl_trait! { fmt::Debug for WebEventPointer |self, f| {
    f.debug_struct("WebEventPointer")
        .field("x", &JsNumFmt::<2>(self.x))
        .field("y", &JsNumFmt::<2>(self.y))
        .field("pressure", &JsNumFmt::<3>(self.pressure))
        .field("id", &self.id)
        .field("tilt_x", &self.tilt_x)
        .field("tilt_y", &self.tilt_y)
        .field("twist", &self.twist)
        .field("kind", &self.kind())
        .field("button", &self.button())
        .field("buttons", &EventButtons::from_bits(self.buttons))
        .field("mods", &self.mods)
        .field("etype", &self.etype)
        .field("timestamp", &self.timestamp)
        .finish()
}}
impl WebEventPointer {
    /// Returns a new [`WebEventPointer`].
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub const fn new(
        x: js_number, y: js_number,
        pressure: js_number,
        id: js_int32,
        tilt_x: i8, tilt_y: i8,
        twist: u16,
        kind: EventPointerKind,
        button: u8, buttons: u8,
        mods: KeyMods,
        etype: WebEventKind,
        timestamp: JsInstant,
    ) -> Self {
        let code = WebPointerCode::new()
            .with_kind(kind.to_web())
            .with_button(Self::encode_web_button(button));
        Self { x, y, pressure, id, tilt_x, tilt_y, twist, code, buttons, mods, etype, timestamp }
    }
    /// Returns the pointer-device kind.
    pub const fn kind(self) -> EventPointerKind {
        EventPointerKind::from_web(self.code.get_kind())
    }
    /// Returns the compact web pointer-kind code.
    pub const fn web_kind(self) -> u8 {
        self.code.get_kind()
    }
    /// Returns `self` with the pointer-device kind replaced.
    pub const fn with_kind(mut self, kind: EventPointerKind) -> Self {
        self.code.set_kind(kind.to_web());
        self
    }

    /// Returns the normalized triggering button, or `None` when no button changed.
    pub const fn button(self) -> Option<EventButton> {
        EventButton::from_web(self.web_button())
    }
    /// Returns the raw DOM-compatible `button` value.
    ///
    /// Returns `255` for DOM `button = -1`, meaning no button changed.
    pub const fn web_button(self) -> u8 {
        Self::decode_web_button(self.code.get_button())
    }
    /// Returns `self` with the normalized triggering button replaced.
    pub const fn with_button(mut self, button: Option<EventButton>) -> Self {
        let web_button = match button {
            Some(button) => button.to_web(),
            None => 255,
        };
        self.code.set_button(Self::encode_web_button(web_button));
        self
    }
    /// Returns `self` with the raw DOM-compatible `button` value replaced.
    pub const fn with_web_button(mut self, button: u8) -> Self {
        self.code.set_button(Self::encode_web_button(button));
        self
    }

    /* helpers */
    const WEB_BUTTON_NONE: u8 = 15;
    const fn encode_web_button(button: u8) -> u8 {
        match button {
            255 => Self::WEB_BUTTON_NONE,
            0..=14 => button,
            _ => Self::WEB_BUTTON_NONE,
        }
    }
    const fn decode_web_button(code: u8) -> u8 {
        match code {
            Self::WEB_BUTTON_NONE => 255,
            n => n,
        }
    }
}

impl WebEventPointer {
    /// Converts `WebEventPointer` to `EventKindTimed`.
    pub const fn to_kind_timed(self) -> EventKindTimed {
        let kind = EventKind::Pointer(EventPointer::new(
            self.kind(),
            self.id as u32,
            self.x as i32,
            self.y as i32,
            0,
            0,
            f32bits_niche::new(self.pressure as f32),
            self.tilt_x,
            self.tilt_y,
            self.twist,
            self.button(),
            EventButtonState::from_web(self.etype),
            EventButtons::from_bits(self.buttons),
            self.mods,
        ));
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(kind, timestamp)
    }
    /// Converts a timed normalized `EventPointer` back to `WebEventPointer`.
    pub const fn from_event_pointer_timed(
        from: Timed<EventPointer, Option<EventTimestamp>>,
    ) -> WebEventPointer {
        let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
        WebEventPointer::new(
            from.value.x as js_number,
            from.value.y as js_number,
            from.value.get_pressure() as js_number,
            from.value.id as js_int32,
            from.value.tilt_x,
            from.value.tilt_y,
            from.value.twist,
            from.value.kind,
            is![let Some(b) = from.value.button, b.to_web(), 255],
            from.value.buttons.bits(),
            from.value.mods,
            from.value.state.to_web_as_pointer(),
            timestamp,
        )
    }
}
impl From<WebEventPointer> for EventKindTimed {
    fn from(from: WebEventPointer) -> Self {
        from.to_kind_timed()
    }
}
impl From<Timed<EventPointer, Option<EventTimestamp>>> for WebEventPointer {
    fn from(from: Timed<EventPointer, Option<EventTimestamp>>) -> Self {
        Self::from_event_pointer_timed(from)
    }
}

impl EventPointerKind {
    /// Converts a compact web pointer kind into `EventPointerKind`.
    pub const fn from_web(kind: u8) -> Self {
        match kind {
            1 => EventPointerKind::Touch,
            2 => EventPointerKind::Pen,
            _ => EventPointerKind::Mouse,
        }
    }
    /// Converts `EventPointerKind` into a compact web pointer kind.
    pub const fn to_web(self) -> u8 {
        match self {
            EventPointerKind::Mouse => 0,
            EventPointerKind::Touch => 1,
            EventPointerKind::Pen => 2,
        }
    }
}

crate::bitfield! {
    /// Internal compact encoding for pointer kind and triggering button.
    struct WebPointerCode(u8) {
        /// Compact pointer kind: `0` mouse, `1` touch, `2` pen.
        KIND = 0..=1;
        /// Compact triggering-button code.
        ///
        /// This is not the raw DOM `button` value. In particular,
        /// DOM `button = -1` is encoded with a private sentinel.
        BUTTON = 2..=5;
        /// Reserved bits for future pointer metadata.
        RESERVED = 6..=7;
    }
}
