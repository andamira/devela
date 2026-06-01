// devela::sys::os::browser::web::event::pointer
//
//! Defines [`WebEventMouse`], [`WebEventPointer`].
//

use crate::lang::prog::ffi::js::{JsInstant, js_int32, js_number};
use crate::{EventPointerKind, KeyMods, WebEventKind};

#[doc = crate::_tags!(event web)]
/// A web API Mouse Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventMouse = 32|256; niche Option),
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventMouse {
    /// The X-coordinate of the mouse event relative to the viewport.
    pub x: js_number, // 8 bytes
    /// The Y-coordinate of the mouse event relative to the viewport.
    pub y: js_number, // 8 bytes

    /// The mouse button that triggered the event (`0`: left, `1`: middle, `2`: right).
    ///
    /// If the event was a movement without a button click, this is `-1` (255)
    pub button: u8, // 1 byte
    /// A bitmask of buttons currently being held down (`1`: left, `2`: right, `4`: middle).
    pub buttons: u8, // 1 byte
    /// A bitmask of active keyboard modifiers during the mouse event.
    pub mods: KeyMods, // 2 bytes
    /// The type of mouse event (Click, MouseDown, MouseMove, etc.).
    pub etype: WebEventKind, // 4 bytes

    /// The JavaScript event timestamp.
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
        timestamp: JsInstant,
    ) -> Self {
        Self { x, y, button, buttons, mods, etype, timestamp }
    }
}

#[doc = crate::_tags!(event web)]
/// A web API Pointer Event.
#[doc = crate::_doc_meta!{
    location("sys/os/browser/web"),
    test_size_of(WebEventPointer = 56|448; niche Option),
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WebEventPointer {
    /// The X-coordinate of the pointer event relative to the viewport.
    pub x: js_number, // 8 bytes
    /// The Y-coordinate of the pointer event relative to the viewport.
    pub y: js_number, // 8 bytes
    /// The pressure applied to the pointer (0.0 to 1.0 for most devices).
    pub pressure: js_number, // 8 bytes

    /// Unique identifier for the pointer device.
    pub id: js_int32, // 4 bytes
    /// The kind of pointer device.
    pub kind: EventPointerKind,
    /// The tilt of the stylus along the X-axis (-90° to 90°).
    pub tilt_x: i8, // 1 bytes
    /// The tilt of the stylus along the Y-axis (-90° to 90°).
    pub tilt_y: i8, // 1 bytes
    /// The rotation of the stylus around its own axis (0° to 359°).
    pub twist: u16, // 2 bytes

    /// The pointer button that triggered the event.
    pub button: u8, // 1 byte
    /// A bitmask of buttons currently being held down during the pointer event.
    pub buttons: u8, // 1 byte
    /// A bitmask of active keyboard modifiers during the pointer event.
    pub mods: KeyMods, // 2 bytes
    /// The type of pointer event (PointerDown, PointerMove, etc.).
    pub etype: WebEventKind, // 4 bytes

    /// The JavaScript event timestamp.
    pub timestamp: JsInstant, // 8 bytes
}
impl WebEventPointer {
    /// Returns a new [`WebEventPointer`].
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub const fn new(
        x: js_number, y: js_number,
        pressure: js_number,
        id: js_int32,
        kind: EventPointerKind,
        tilt_x: i8, tilt_y: i8,
        twist: u16,
        button: u8, buttons: u8,
        mods: KeyMods,
        etype: WebEventKind,
        timestamp: JsInstant,
    ) -> Self {
        Self { x, y, pressure, id, kind, tilt_x, tilt_y, twist,
            button, buttons, mods, etype, timestamp, }
    }
}

#[rustfmt::skip]
mod impls {
    use crate::{
        EventButton, EventButtons, EventButtonState, EventKind, EventKindTimed, EventMouse,
        EventPointer, EventPointerKind, EventTimestamp, JsInstant, KeyMods, Timed, WebEventKind,
        WebEventMouse, WebEventPointer, f32bits_niche, is, js_int32, js_number,
    };

    /* mouse */

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
        pub const fn from_event_mouse_timed(from: Timed<EventMouse, Option<EventTimestamp>>)
            -> WebEventMouse {
            let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
            WebEventMouse {
                x: from.value.x as js_number,
                y: from.value.y as js_number,
                button: is![let Some(b) = from.value.button, b.to_web(), 255], // IMPROVE to_web
                buttons: from.value.buttons.bits(), // already a bitmask, directly compatible
                mods: from.value.mods,
                etype: from.value.state.to_web_as_mouse(),
                timestamp,
            }
        }
    }
    impl From<WebEventMouse> for EventKindTimed {
        fn from(from: WebEventMouse) -> Self { from.to_kind_timed() }
    }
    impl From<Timed<EventMouse, Option<EventTimestamp>>> for WebEventMouse {
        fn from(from: Timed<EventMouse, Option<EventTimestamp>>) -> Self {
            Self::from_event_mouse_timed(from)
        }
    }

    /* pointer */

    impl WebEventPointer {
    /// Converts `WebEventPointer` to `EventKindTimed`.
    pub const fn to_kind_timed(self) -> EventKindTimed {
        let kind = EventKind::Pointer(EventPointer::new(
            self.kind,
            self.id as u32,
            self.x as i32, self.y as i32,
            0, 0,
            f32bits_niche::new(self.pressure as f32),
            self.tilt_x, self.tilt_y,
            self.twist,
            EventButton::from_web(self.button),
            EventButtonState::from_web(self.etype),
            EventButtons::from_bits(self.buttons),
            self.mods,
        ));
        let timestamp = Some(EventTimestamp::from_js(self.timestamp));
        EventKindTimed::new(kind, timestamp)
    }
    /// Converts a timed normalized `EventPointer` back to `WebEventPointer`.
    pub const fn from_event_pointer_timed(from: Timed<EventPointer, Option<EventTimestamp>>)
        -> WebEventPointer {
            let timestamp = is![let Some(t) = from.time, t.to_js(), JsInstant { ms: 0.0 }];
            WebEventPointer {
                x: from.value.x as js_number,
                y: from.value.y as js_number,
                pressure: from.value.get_pressure() as js_number,
                id: from.value.id as js_int32,
                kind: from.value.kind,
                tilt_x: from.value.tilt_x,
                tilt_y: from.value.tilt_y,
                twist: from.value.twist,
                button: is![let Some(b) = from.value.button, b.to_web(), 255],
                buttons: from.value.buttons.bits(),
                mods: from.value.mods,
                etype: from.value.state.to_web_as_pointer(),
                timestamp,
            }
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

    /* button */

    impl EventButton {
        /// Converts a web API button index in [`WebEventMouse`] into `EventButton`.
        //
        pub const fn from_web(js_button: u8) -> Option<Self> {
            match js_button {
                0 => Some(EventButton::Left),
                1 => Some(EventButton::Middle),
                2 => Some(EventButton::Right),
                3 => Some(EventButton::X1),
                4 => Some(EventButton::X2),
                //
                5 => Some(EventButton::X3),
                6 => Some(EventButton::X4),
                7 => Some(EventButton::X5),
                255 => None, // (== -1_i8) represents "no button"
                n => Some(EventButton::Other(n)),
            }
        }
        /// Converts an EventButton to a JavaScript button index in [`WebEventMouse`].
        pub const fn to_web(self) -> u8 {
            match self {
                EventButton::Left => 0,
                EventButton::Middle => 1,
                EventButton::Right => 2,
                EventButton::X1 => 3,
                EventButton::X2 => 4,
                //
                EventButton::X3 => 5,
                EventButton::X4 => 6,
                EventButton::X5 => 7,
                EventButton::Other(n) => n,
            }
        }
    }

    // IMPROVE: MAYBE impl try_ methods
    impl EventButtonState {
        /// Converts a `WebEventKind` into `EventButtonState`.
        pub const fn from_web(js_event: WebEventKind) -> Self {
            use {WebEventKind as J, EventButtonState as E};
            match js_event {
                J::Click | J::MouseDown | J::PointerDown => E::Pressed,
                J::MouseUp | J::PointerUp => E::Released,
                _ => E::Moved,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_web_as_mouse(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::MouseDown,
                E::Released => J::MouseUp,
                E::Moved => J::MouseMove,
            }
        }
        /// Converts a `EventButtonState` into a `WebEventKind`.
        pub const fn to_web_as_pointer(self) -> WebEventKind {
            use {WebEventKind as J, EventButtonState as E};
            match self {
                E::Pressed => J::PointerDown,
                E::Released => J::PointerUp,
                E::Moved => J::PointerMove,
            }
        }
    }

    /* key modifiers */

    impl KeyMods {
        /// Constructs `KeyMods` from a compact web modifier bitmask.
        ///
        /// Bit layout:
        /// - 0: Control
        /// - 1: Shift
        /// - 2: Alt
        /// - 3: Super / Meta
        /// - 4: AltGraph
        /// - 5: CapsLock
        /// - 6: NumLock
        /// - 7: ScrollLock
        pub const fn from_web(bits: u8) -> Self {
            Self::from_bits(bits as u16)
        }

        /// Converts `KeyMods` into a compact web modifier bitmask.
        ///
        /// Only the web-representable low byte is preserved.
        pub const fn to_web(self) -> u8 {
            (self.bits() & 0x00FF) as u8
        }
    }
}
