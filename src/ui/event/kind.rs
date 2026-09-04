// devela/src/ui/event/kind.rs
//
//! Defines [`EventTag`], [`EventTagSet`], [`EventKindTimed`], [`EventKind`].
//

use crate::{AppControl, ConstInit, MaybeTimed};
use crate::{EventKey, EventMouse, EventPointer, EventTimestamp, EventWheel, EventWindow};

crate::enumset! {
    #[doc = crate::_tags!(event uid member)]
    /// A lightweight, data-less identifier for [`EventKind`].
    #[doc = crate::_doc_meta!{
        location("ui/event", enum EventTag),
        test_size_of(EventTag = 1|8; niche Option),
    }]
    /// Used when only the *category* of the event is relevant and the
    /// payload of the variant is not needed.
    ///
    /// This avoids carrying the full `EventKind` and is suitable for
    /// quick routing, filtering, or statistics.
    #[repr(u8)]
    #[non_exhaustive]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum EventTag(
        #[doc = crate::_tags!(event set)]
        /// A compact set of [`EventTag`]s.
        #[doc = crate::_doc_meta!{
            location("ui/event", struct EventTagSet),
            test_size_of(EventTagSet = 1|8; niche !Option),
        }]
        ///
        /// Used to declare, filter, and query coarse event categories.
        pub EventTagSet: u8
    ) {
        #[doc = crate::_tags!(init)]
        /// No event or an empty placeholder.
        None,
        /// A keyboard event.
        Key,
        /// A mouse button or motion event.
        Mouse,
        /// A pointing-device event (mouse, pen, stylus, touch).
        Pointer,
        /// A wheel event.
        Wheel,
        /// An event related to a window, viewport, or presentation surface.
        Window,
        /// Application-level control notice.
        Control,
    }
    impl set #[doc = "Common event category groups."] {
        /// Events caused by direct user input.
        pub const INPUT: Self = Self::Key
            .with(Self::Mouse)
            .with(Self::Pointer)
            .with(Self::Wheel);
        /// Events related to pointing devices.
        pub const POINTING: Self = Self::Mouse
            .with(Self::Pointer)
            .with(Self::Wheel);
    }
}
impl Default for EventTag {
    /// Returns [`EventTag::None`].
    fn default() -> Self {
        Self::None
    }
}

#[doc = crate::_tags!(event time maybe)]
/// A convenience helper for optionally timed event kinds.
#[doc = crate::_doc_meta!{
    location("ui/event", type EventKindTimed),
    #[cfg(not(feature = "alloc"))]
    test_size_of(EventKindTimed = 40|320; niche Option),
    #[cfg(feature = "alloc")] #[cfg(target_pointer_size = "32")]
    test_size_of(EventKindTimed = 40|320; niche Option),
    #[cfg(feature = "alloc")] #[cfg(target_pointer_size = "64")]
    test_size_of(EventKindTimed = 48|384; niche Option),
}]
pub type EventKindTimed = MaybeTimed<EventKind, EventTimestamp>;

#[doc = crate::_tags!(event)]
/// An enumeration of concrete event variants.
#[doc = crate::_doc_meta!{
    location("ui/event", enum EventKind),
    #[cfg(not(feature = "alloc"))]
    test_size_of(EventKind = 36|288; niche Option),
    #[cfg(feature = "alloc")] #[cfg(target_pointer_size = "32")]
    test_size_of(EventKind = 36|288; niche Option),
    #[cfg(feature = "alloc")] #[cfg(target_pointer_size = "64")]
    test_size_of(EventKind = 40|320; niche Option),
}]
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
    #[doc = crate::_tags!(init)]
    /// An unknown, empty or absent event.
    #[default]
    None,

    /// A keyboard event.
    Key(EventKey),

    /// A mouse event.
    Mouse(EventMouse),

    /// A pointing-device event (mouse, pen, stylus, or multi-touch).
    Pointer(EventPointer),

    /// A wheel event.
    Wheel(EventWheel),

    /// A window event.
    Window(EventWindow),

    /// External control notice directed at the application.
    Control(AppControl),
    /* */
    // /// A gamepad event.
    // Gamepad(GamepadEvent),

    // /// A midi event.
    // Midi(MidiEvent),
}
impl ConstInit for EventKind {
    const INIT: Self = Self::None;
}
impl EventKind {
    #[must_use]
    /// Returns whether this event kind has `tag`.
    pub const fn has_tag(&self, tag: EventTag) -> bool {
        self.tag().to_set().contains(tag.to_set())
    }
    #[must_use]
    /// Returns whether this event kind belongs to `set`.
    pub const fn is_in(&self, set: EventTagSet) -> bool {
        self.tag().is_in(set)
    }

    /// Returns the coarse category of this event without its associated data.
    ///
    /// Useful for fast dispatch or matching on *type* rather than *content*.
    pub const fn tag(&self) -> EventTag {
        match self {
            Self::None => EventTag::None,
            Self::Key(_) => EventTag::Key,
            Self::Mouse(_) => EventTag::Mouse,
            Self::Pointer(_) => EventTag::Pointer,
            Self::Wheel(_) => EventTag::Wheel,
            Self::Window(_) => EventTag::Window,
            Self::Control(_) => EventTag::Control,
        }
    }
}

#[rustfmt::skip]
impl EventKind {
    #[must_use]
    /// Whether there's no event.
    pub const fn is_none(&self) -> bool { matches![self, EventKind::None] }

    #[must_use]
    /// Whether there's some event.
    pub const fn is_some(&self) -> bool { !matches![self, EventKind::None] }

    #[must_use]
    /// Whether it's a keyboard event.
    pub const fn is_key(&self) -> bool { matches![self, EventKind::Key(_)] }

    #[must_use]
    /// Whether it's a mouse event.
    pub const fn is_mouse(&self) -> bool { matches![self, EventKind::Mouse(_)] }

    #[must_use]
    /// Whether it's a pointer event.
    pub const fn is_pointer(&self) -> bool { matches![self, EventKind::Pointer(_)] }

    #[must_use]
    /// Whether it's a wheel event.
    pub const fn is_wheel(&self) -> bool { matches![self, EventKind::Wheel(_)] }

    #[must_use]
    /// Whether it's a window event.
    pub const fn is_window(&self) -> bool { matches![self, EventKind::Window(_)] }

    #[must_use]
    /// Whether it's a control event.
    pub const fn is_control(&self) -> bool { matches![self, EventKind::Control(_)] }

    // /// Returns `true` if it's a keyboard event.
    // pub const fn is_midi(&self) -> bool { matches![self, EventKind::Midi(_)] }

    // /// Returns `true` if it's a mouse event.
    // pub const fn is_gamepad(&self) -> bool { matches![self, EventKind::Gamepad(_)] }

    #[must_use]
    /// Returns some keyboard event, if that's the kind.
    pub const fn some_key(&self) -> Option<&EventKey> {
        if let EventKind::Key(e) = &self { Some(e) } else { None }
    }
    #[must_use]
    /// Returns some mouse event, if that's the kind.
    pub const fn some_mouse(&self) -> Option<&EventMouse> {
        if let EventKind::Mouse(e) = &self { Some(e) } else { None }
    }
    #[must_use]
    /// Returns some pointer event, if that's the kind.
    pub const fn some_pointer(&self) -> Option<&EventPointer> {
        if let EventKind::Pointer(e) = &self { Some(e) } else { None }
    }
    #[must_use]
    /// Returns some wheel event, if that's the kind.
    pub const fn some_wheel(&self) -> Option<&EventWheel> {
        if let EventKind::Wheel(e) = &self { Some(e) } else { None }
    }
    #[must_use]
    /// Returns some window event, if that's the kind.
    pub const fn some_window(&self) -> Option<&EventWindow> {
        if let EventKind::Window(e) = &self { Some(e) } else { None }
    }
    #[must_use]
    /// Returns some control event, if that's the kind.
    pub const fn some_control(&self) -> Option<&AppControl> {
        if let EventKind::Control(e) = &self { Some(e) } else { None }
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
