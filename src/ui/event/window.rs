// devela::ui::event::window
//
//! Defines [`EventWindow`].
//

use crate::{ConstInit, Event, EventKind};

#[cfg(feature = "alloc")]
use crate::String;

/// Events related to the window.
//
// - https://docs.rs/sdl2/latest/sdl2/event/enum.WindowEvent.html
// - https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EventWindow {
    /// The window gained input focus.
    ///
    /// Triggered when the system directs keyboard input to this window.
    FocusGained,

    /// The window lost input focus.
    ///
    /// Triggered when the system stops directing keyboard input to this window.
    FocusLost,

    /// Window size changed.
    ///
    /// Carries the new width–height pair when available.
    // Some backends (like X11 expose sequences) may not provide it.
    Resized(Option<[u32; 2]>),

    /// Window position changed.
    ///
    /// Carries the new x–y coordinates when available.
    // Some backends emit movement without precise coordinates.
    Moved(Option<[u32; 2]>),

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

    /// The window requests a repaint of its contents.
    ///
    /// Emitted when the system notifies that some region needs redrawing.
    RedrawRequested,

    /// The window manager requested that this window should close.
    CloseRequested,

    /// Window became visible on screen.
    ///
    /// Triggered when the system maps or shows the window.
    Shown,

    /// Window became hidden or obscured.
    ///
    /// Triggered when the system unmaps or hides the window.
    Hidden,
    // Minimized,
    // Maximized,
    // Restored,
    // Enter,
    // Leave,
    // Close,
    // TakeFocus,
    // HitTest,
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
