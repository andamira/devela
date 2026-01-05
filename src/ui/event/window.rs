// devela::ui::event::window
//
//! Defines [`EventWindow`].
//

use crate::{ConstInit, Event, EventKind};
// use crate::{Extent, Position};

#[cfg(feature = "alloc")]
use crate::String;

#[doc = crate::_TAG_EVENT!()]
#[doc = crate::_TAG_INTERACTION!()]
/// Events related to a window.
#[doc = crate::_doc_location!("ui/event")]
///
/// Names and payloads are backend-agnostic and focus on
/// the stable cross-platform meaning of each event.
//
// - https://docs.rs/sdl2/latest/sdl2/event/enum.WindowEvent.html
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventWindow {
    /* geometry */
    /// The window's size changed.
    ///
    /// Carries the new `[width, height]` when the backend provides it.
    // NOTE in SDL: https://stackoverflow.com/a/55076700/940200
    // SizeChanged(UiSize),
    Resized(Option<[u32; 2]>),
    // Resized(Option<Extent<u32, 2>>),
    /// The window's position changed.
    ///
    /// Carries the new `[x, y]` when available.
    Moved(Option<[i32; 2]>),
    // Moved(Option<Position<i32, 2>>),

    /* focus*/
    /// The window gained keyboard focus.
    FocusGained,

    /// The window lost keyboard focus.
    FocusLost,

    /* life cycle / visibility transitions */
    /// The system or window manager requested that the window close.
    CloseRequested,

    /// The window became visible (mapped / shown).
    Shown,

    /// The window became hidden (unmapped / obscured).
    Hidden,

    /// The window needs its contents repainted.
    RedrawRequested,

    /// The window was minimized.
    Minimized,

    /// The window was maximized.
    Maximized,

    /// The window was restored to its normal state.
    Restored,

    /// The window entered fullscreen mode.
    FullscreenEntered,

    /// The window exited fullscreen mode.
    FullscreenExited,

    /* pointer enter/leave */
    /// The pointer entered the window region.
    Entered,

    /// The pointer left the window region.
    Left,

    /* text / clipboard */
    /// Paste text.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    Paste(String),

    /* input stream signals (terminal) */
    /// The application resumed after being paused (e.g. SIGCONT).
    // https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_signal
    Continue,

    /// End-of-input condition (EOF).
    // https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_eof
    EndOfInput,
}
impl ConstInit for EventWindow {
    const INIT: Self = Self::FocusLost;
}
impl From<EventWindow> for EventKind {
    fn from(window_event: EventWindow) -> EventKind {
        EventKind::Window(window_event)
    }
}
impl From<EventWindow> for Event {
    fn from(window_event: EventWindow) -> Event {
        EventKind::from(window_event).into()
    }
}

impl EventWindow {
    /* geometry */

    /// True if this event indicates a change in window size or position.
    pub const fn is_geometry(&self) -> bool {
        matches!(self, Self::Resized(_) | Self::Moved(_))
    }

    /// True if this is a resize event (with or without data).
    pub const fn is_resize(&self) -> bool {
        matches!(self, Self::Resized(_))
    }
    /// Returns some resize event, if that's the kind.
    pub const fn resize_coord(&self) -> Option<[u32; 2]> {
        // pub const fn resize_coord(&self) -> Option<Extent<u32, 2>> {
        if let EventWindow::Resized(e) = self { *e } else { None }
    }

    /// True if this is a move event (with or without data).
    pub const fn is_move(&self) -> bool {
        matches!(self, Self::Moved(_))
    }
    /// Returns some move event, if that's the kind.
    pub const fn move_coords(&self) -> Option<[i32; 2]> {
        // pub const fn move_coord(&self) -> Option<Position<i32, 2>> {
        if let EventWindow::Moved(e) = self { *e } else { None }
    }

    /* focus */

    /// True if this event indicates a change in keyboard focus.
    pub const fn is_focus(&self) -> bool {
        matches!(self, Self::FocusGained | Self::FocusLost)
    }

    /* life cycle / visibility */

    /// True if this event indicates that the system wants the window to close.
    pub const fn is_close(&self) -> bool {
        matches!(self, Self::CloseRequested)
    }

    /// True if this event requests a redraw.
    pub const fn is_redraw(&self) -> bool {
        matches!(self, Self::RedrawRequested)
    }

    /// True if this event indicates window visibility or lifecycle transitions.
    pub const fn is_visibility(&self) -> bool {
        matches!(
            self,
            Self::Shown
                | Self::Hidden
                | Self::Minimized
                | Self::Restored
                | Self::Maximized
                | Self::FullscreenEntered
                | Self::FullscreenExited
        )
    }

    /* pointer enter/leave */

    /// True if the pointer crossed into or out of the window.
    pub const fn is_pointer_crossing(&self) -> bool {
        matches!(self, Self::Entered | Self::Left)
    }

    /* text / clipboard */

    /// True if the event involves clipboard/text input.
    #[cfg(feature = "alloc")]
    pub const fn is_text_input(&self) -> bool {
        matches!(self, Self::Paste(_))
    }

    /* input stream signals (terminal) */

    /// True if this event is a terminal/stream-control event.
    pub const fn is_stream_signal(&self) -> bool {
        matches!(self, Self::Continue | Self::EndOfInput)
    }
}
