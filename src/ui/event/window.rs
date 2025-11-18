// devela::ui::event::window
//
//! Defines [`EventWindow`].
//

use super::{Event, EventKind};

#[cfg(feature = "alloc")]
use crate::String;

/// Events related to the window.
//
// - https://docs.rs/sdl2/latest/sdl2/event/enum.WindowEvent.html
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventWindow {
    ///
    FocusGained,

    ///
    FocusLost,

    ///
    // crossterm and sdl has this
    Resized(Option<[usize; 2]>),

    // NOTE the difference in SDL: https://stackoverflow.com/a/55076700/940200
    // SizeChanged(UiSize),
    /// Signal to continue (SIGCONT).
    ///
    /// This typically indicates that the program has been restarted after being
    /// paused or placed in the background.
    //
    // https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_signal
    Continue,

    /// Signal of end of input.
    ///
    /// Notcurses specific. [see doc][0].
    ///
    /// [0]: https://github.com/dankamongmen/notcurses/blob/master/doc/man/man3/notcurses_input.3.md#nckey_eof
    EndOfInput,

    /// Paste action.
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    Paste(String),
    // Moved(Position),

    // Shown,
    // Hidden,
    // Exposed,
    // Minimized,
    // Maximized,
    // Restored,
    // Enter,
    // Leave,
    // Close,
    // TakeFocus,
    // HitTest,
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
