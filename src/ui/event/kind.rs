// devela::ui::event::kind
//
//! Defines [`EventTag`] and [`EventKind`].
//

use crate::{ConstInit, EventKey, EventMouse, EventPointer, EventWindow};

#[doc = crate::_tags!(event uid)]
/// A lightweight, data-less identifier for `EventKind`.
#[doc = crate::_doc_location!("ui/event")]
///
/// Used when only the *category* of the event is relevant and the
/// payload of the variant is not needed.
///
/// This avoids carrying the full `EventKind` and is suitable for
/// quick routing, filtering, or statistics.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventTag {
    /// No event or an empty placeholder.
    #[default]
    None,
    /// A keyboard event.
    Key,
    /// A mouse button or motion event.
    Mouse,
    /// A pointing-device event (mouse, pen, stylus, touch).
    Pointer,
    /// A window-related event (focus, resize, etc.).
    Window,
}

#[doc = crate::_tags!(event)]
/// An enumeration of concrete event variants.
#[doc = crate::_doc_location!("ui/event")]
///
/// High-level, typed grouping of input and window interactions.
/// Backends normalize their raw events into these variants.
//
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
//   - https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs
// - https://docs.rs/notcurses/latest/notcurses/struct.Input.html
//   - https://docs.rs/notcurses/latest/notcurses/enum.Received.html
// - https://docs.rs/sdl2/latest/sdl2/event/enum.Event.html
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum EventKind {
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A keyboard event.
    Key(EventKey),

    /// A mouse event.
    Mouse(EventMouse),

    /// A pointing-device event (mouse, pen, stylus, or multi-touch).
    Pointer(EventPointer),

    /// A window event.
    Window(EventWindow),
    // /// A gamepad event.
    // Gamepad(GamepadEvent),

    // /// A midi event.
    // Midi(MidiEvent),
}
impl ConstInit for EventKind {
    const INIT: Self = Self::None;
}
impl EventKind {
    /// Returns the coarse category of this event without its associated data.
    ///
    /// Useful for fast dispatch or matching on *type* rather than *content*.
    pub const fn tag(&self) -> EventTag {
        match self {
            Self::None => EventTag::None,
            Self::Key(_) => EventTag::Key,
            Self::Mouse(_) => EventTag::Mouse,
            Self::Pointer(_) => EventTag::Pointer,
            Self::Window(_) => EventTag::Window,
        }
    }
}

#[rustfmt::skip]
impl EventKind {
    /// Whether there's no event.
    #[must_use] #[inline(always)]
    pub const fn is_none(&self) -> bool { matches![self, EventKind::None] }

    /// Whether there's some event.
    #[must_use] #[inline(always)]
    pub const fn is_some(&self) -> bool { !matches![self, EventKind::None] }

    /// Whether it's a window event.
    #[must_use] #[inline(always)]
    pub const fn is_window(&self) -> bool { matches![self, EventKind::Window(_)] }

    /// Whether it's a keyboard event.
    #[must_use] #[inline(always)]
    pub const fn is_key(&self) -> bool { matches![self, EventKind::Key(_)] }

    /// Whether it's a mouse event.
    #[must_use] #[inline(always)]
    pub const fn is_mouse(&self) -> bool { matches![self, EventKind::Mouse(_)] }

    /// Whether it's a pointer event.
    #[must_use] #[inline(always)]
    pub const fn is_pointer(&self) -> bool { matches![self, EventKind::Pointer(_)] }

    // /// Returns `true` if it's a keyboard event.
    // pub const fn is_midi(&self) -> bool { matches![self, EventKind::Midi(_)] }

    // /// Returns `true` if it's a mouse event.
    // pub const fn is_gamepad(&self) -> bool { matches![self, EventKind::Gamepad(_)] }

    /// Returns some window event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_window(&self) -> Option<&EventWindow> {
        if let EventKind::Window(e) = &self { Some(e) } else { None }
    }

    /// Returns some keyboard event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_key(&self) -> Option<&EventKey> {
        if let EventKind::Key(e) = &self { Some(e) } else { None }
    }

    /// Returns some mouse event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_mouse(&self) -> Option<&EventMouse> {
        if let EventKind::Mouse(e) = &self { Some(e) } else { None }
    }

    /// Returns some pointer event, if that's the kind.
    #[must_use] #[inline(always)]
    pub const fn some_pointer(&self) -> Option<&EventPointer> {
        if let EventKind::Pointer(e) = &self { Some(e) } else { None }
    }

    // /// Returns some gamepad event, if that's the kind.
    // pub const fn some_gamepad(&self) -> Option<&GamepadEvent> {
    //     if let EventKind::Gamepad(e) = &self { Some(e) } else { None }
    // }

    // /// Returns some midi event, if that's the kind.
    // pub const fn some_midi(&self) -> Option<&MidiEvent> {
    //     if let EventKind::Midi(e) = &self { Some(e) } else { None }
    // }
}
