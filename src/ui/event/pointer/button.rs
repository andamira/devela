// devela/src/ui/event/pointer/button.rs
//
//! Defines [`EventButton`], [`EventButtons`], [`EventButtonState`].
//

use crate::{_impl_init, impl_trait, is, set};

/* definitions */

#[doc = crate::_tags!(event interaction member)]
/// A normalized pressable pointer/mouse button.
#[doc = crate::_doc_meta!{
    location("ui/event"),
    test_size_of(EventButton = 2|16; niche Option),
}]
/// `Left`, `Middle`, and `Right` represent the three primary conventional buttons.
/// `X1..X5` represent additional auxiliary button slots when a backend can report them.
///
/// These variants name normalized button slots, not guaranteed user actions.
/// For example, `X1` is often used as “back”, `X2` as “forward”, and on
/// Pointer Events a pen eraser may be reported through an auxiliary slot.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventButton {
    /// Left mouse button.
    #[default]
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button (usually the scroll wheel button).
    Middle,
    /// The first auxiliary mouse button, commonly "back".
    X1,
    /// The second auxiliary mouse button, commonly "forward".
    X2,
    /// The third auxiliary mouse button.
    X3,
    /// The fourth auxiliary mouse button.
    X4,
    /// The fifth auxiliary mouse button.
    X5,
    /// Additional buttons (e.g., side buttons on advanced mice).
    Other(u8),
}
_impl_init! { Self::Left => EventButton }
impl EventButton {
    /// Returns some button as long as it's not 0.
    pub const fn new(number: u8) -> Option<Self> {
        match number {
            1 => Some(Self::Left),
            2 => Some(Self::Middle),
            3 => Some(Self::Right),
            _ => Some(Self::Other(number)),
        }
    }

    /// Returns the button represented by the mask, if exactly one button is set.
    #[inline(always)]
    pub const fn from_one_bit_mask(mask: EventButtons) -> Option<EventButton> {
        match mask.bits() {
            1 => Some(EventButton::Left),
            2 => Some(EventButton::Right),
            4 => Some(EventButton::Middle),
            8 => Some(EventButton::X1),
            16 => Some(EventButton::X2),
            32 => Some(EventButton::X3),
            64 => Some(EventButton::X4),
            128 => Some(EventButton::X5),
            _ => None,
        }
    }
    /// Returns this button as a held-button mask, if it has a normalized role.
    #[inline(always)]
    pub const fn to_mask(self) -> EventButtons {
        match self {
            Self::Left => EventButtons::new().with(EventButtons::LEFT),
            Self::Right => EventButtons::new().with(EventButtons::RIGHT),
            Self::Middle => EventButtons::new().with(EventButtons::MIDDLE),
            Self::X1 => EventButtons::new().with(EventButtons::X1),
            Self::X2 => EventButtons::new().with(EventButtons::X2),
            Self::X3 => EventButtons::new().with(EventButtons::X3),
            Self::X4 => EventButtons::new().with(EventButtons::X4),
            Self::X5 => EventButtons::new().with(EventButtons::X5),
            Self::Other(_) => EventButtons::new(),
        }
    }
}

#[doc = crate::_tags!(event interaction)]
/// Represents the state of a button.
#[doc = crate::_doc_meta!{location("ui/event")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventButtonState {
    /// The button was pressed.
    ///
    /// This is the default.
    #[default]
    Pressed,
    /// The button was released.
    Released,
    /// The pointer moved without a button press/release.
    Moved,
}
_impl_init! { Self::Pressed => EventButtonState }

set! {
    #[doc = crate::_tags!(event interaction set)]
    /// A semantic bitmask of currently held pressable buttons.
    #[doc = crate::_doc_meta!{
        location("ui/event"),
        test_size_of(EventButtons = 1|8), // option = 2|16
    }]
    ///
    /// The bits represent normalized button roles, not raw backend button numbers.
    ///
    /// This means:
    /// - `LEFT`, `RIGHT`, and `MIDDLE` are the three primary buttons.
    /// - `X1..X5` are additional auxiliary buttons when a backend can report them.
    /// - wheel motion is **not** represented here; it belongs in
    ///   [`EventWheel`][crate::EventWheel].
    ///
    /// Unsupported buttons are left cleared.
    ///
    /// # Backend notes
    /// - **Web** can naturally carry `LEFT`, `RIGHT`, `MIDDLE`, `X1`, and `X2`.
    /// - **X11 core** currently sets only `LEFT`, `RIGHT`, and `MIDDLE`;
    ///   buttons 4 and 5 are treated as wheel input in the current backend.
    /// - **Terminal SGR** reports primary buttons, wheel motion, and some
    ///   extended button encodings. This library normalizes the first supported
    ///   non-primary button encodings into `X1..X5` when unambiguous.
    /// - Other backends may populate only the subset they can observe.
    ///
    /// This type is semantic and cross-platform.
    /// Backend-specific numbering should be translated at the backend edge.
    //
    // Firefox may consume browser Back/Forward mouse buttons before dispatching
    // DOM mouse events, so button=3/4 and buttons=8/16 may never reach web code.
    // WAIT: [firefox-back-forward-buttons](https://bugzilla.mozilla.org/show_bug.cgi?id=1933746)
    pub struct EventButtons(u8) {
        /// The primary left button.
        LEFT = 0;
        /// The primary right button.
        RIGHT = 1;
        /// The primary middle button.
        MIDDLE = 2;
        /// The first auxiliary button, often “back”.
        X1 = 3;
        /// The second auxiliary button, often “forward”.
        X2 = 4;
        /// The third auxiliary button, when available.
        X3 = 5;
        /// The fourth auxiliary button, when available.
        X4 = 6;
        /// The fifth auxiliary button, when available.
        X5 = 7;
    }
    traits(!Debug);
}
impl_trait! { fmt::Debug for EventButtons |self, f| {
    let l = is![self.has_left(), "L", "-"];
    let r = is![self.has_right(), "R", "-"];
    let m = is![self.has_middle(), "M", "-"];
    let x1 = is![self.has_x1(), "1", "-"];
    let x2 = is![self.has_x2(), "2", "-"];
    let x3 = is![self.has_x3(), "3", "-"];
    let x4 = is![self.has_x4(), "4", "-"];
    let x5 = is![self.has_x5(), "5", "-"];
    write![f, "{l}{r}{m}{x1}{x2}{x3}{x4}{x5}"]
} }
