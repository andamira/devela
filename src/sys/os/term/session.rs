// devela::sys::os::term::session
//
//! Defines [`TermSession`], [`TermPollPolicy`], [`TermMode`].
//

use crate::{Duration, TermLineMode};

#[doc = crate::_tags!(term guard)]
/// Scoped terminal session guard.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
///
/// Keeps backend-specific restoration state alive until dropped.
///
/// The restore payload defines the actual cleanup behavior. For example, a
/// backend may use it to restore raw mode, leave the alternate screen, show the
/// cursor, or reset terminal styling.
#[derive(Debug)]
#[must_use = "dropping the session immediately restores the terminal state"]
pub struct TermSession<R> {
    restore: R,
}
impl<R> TermSession<R> {
    /// Creates a terminal session guard from a restore payload.
    pub const fn new(restore: R) -> Self {
        Self { restore }
    }
    /// Consumes the session and returns its restore payload.
    ///
    /// The payload's own drop behavior is preserved.
    pub fn into_restore(self) -> R {
        self.restore
    }
}

#[doc = crate::_tags!(term event)]
/// Terminal event polling policy.
#[doc = crate::_doc_meta!{location("sys/os/term")}]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TermPollPolicy {
    /// Non-blocking. Preserves pending partial input sequences.
    ///
    /// A lone `ESC` remains pending until more bytes arrive
    /// or it is flushed by another policy.
    Pending,

    /// Non-blocking. Resolves pending lone `ESC` after ready bytes are drained.
    ///
    /// Best for games and frame loops.
    #[default]
    Immediate,

    /// Non-blocking. Resolves pending lone `ESC` after `Duration`.
    ///
    /// Best for interactive text UIs that want to distinguish fast escape
    /// sequences from a deliberate Escape key.
    Timeout(Duration),

    /// Blocking. Waits until an event is available.
    Blocking,
}

crate::set! {
    #[doc = crate::_tags!(term runtime set)]
    /// Terminal session mode request.
    #[doc = crate::_doc_meta!{
        location("sys/os/term"),
        test_size_of(TermMode = 4|32),
    }]
    /// This compact set describes the state changes requested for a scoped [`TermSession`].
    ///
    /// # Synchronization
    /// Line-discipline bits are decoded through [`line_mode`](Self::line_mode)
    /// and applied by backend session entry code, such as `TermLinuxRestore::enter`.
    /// Preset constructors and backend entry logic must stay in sync.
    ///
    /// In `MOUSE_SGR_PIXELS` mode, terminal mouse coordinates are reported as
    /// 0-based pixel positions in `EventMouse` and `EventWheel`.
    //
    // Only add here things that are:
    // - session-scoped,
    // - useful across ordinary terminal frontends.
    // - explicitly enabled/disabled by the client,
    // - realistically restorable on drop,
    pub struct TermMode(u32) {
        /* line discipline */

        /// Event-oriented input with normal terminal behavior mostly preserved.
        EVENT               = 0;
        /// Raw byte-oriented input with most terminal processing disabled.
        RAW                 = 1;

        /* presentation */

        /// Alternate screen buffer.
        ALT_SCREEN          = 2;
        /// Hidden cursor.
        HIDE_CURSOR         = 3;
        /// Clear terminal on session entry.
        CLEAR_ON_ENTER      = 4;
        /// Clear terminal on session drop.
        CLEAR_ON_DROP       = 5;
        /// Reset styling on session drop.
        RESET_STYLE_ON_DROP = 6;

        /* input reporting */

        /// Bracketed paste reporting.
        BRACKETED_PASTE     = 7;
        /// Focus in/out reporting.
        FOCUS_EVENTS        = 8;
        /// Mouse button press/release reporting.
        MOUSE               = 9;
        /// Mouse drag reporting.
        MOUSE_DRAG          = 10;
        /// Mouse motion reporting.
        MOUSE_MOTION        = 11;
        /// SGR mouse encoding, with cell coordinates.
        MOUSE_SGR           = 12;
        /// SGR mouse encoding, with pixel coordinates.
        MOUSE_SGR_PIXELS    = 13;

        /* output transaction */

        /// Synchronized terminal output updates.
        SYNC_UPDATE         = 14;
        /// Preserve terminal output post-processing.
        PROCESSED_OUTPUT = 15;

        /* future/protocol possible extensions */

        // /// Kitty progressive keyboard protocol.
        // KITTY_KEYBOARD      = 16;
        // /// Kitty drag-and-drop protocol support.
        // KITTY_DND           = 17;
    }
    /// # Line mode accessors
    impl {
        /// Traditional name for `EVENT`.
        pub const CBREAK: Self = Self::EVENT;

        /// Returns the requested terminal line mode.
        ///
        /// `RAW` dominates `EVENT`: raw mode is represented as
        /// event-capable input plus extra raw processing changes.
        #[must_use]
        pub const fn line_mode(self) -> TermLineMode {
            if self.has(Self::RAW) {
                TermLineMode::Raw
            } else if self.has(Self::EVENT) {
                TermLineMode::Event
            } else {
                TermLineMode::Line
            }
        }
        /// Returns `self` with the requested terminal line mode.
        #[must_use]
        pub const fn with_line_mode(self, mode: TermLineMode) -> Self {
            let cleared = self.without(Self::EVENT).without(Self::RAW);
            match mode {
                TermLineMode::Line => cleared,
                TermLineMode::Event => cleared.with(Self::EVENT),
                TermLineMode::Raw => cleared.with(Self::EVENT).with(Self::RAW),
            }
        }
    }
    /// # Preset constructors
    impl {
        /// Returns a line-buffered terminal session request.
        #[must_use]
        pub const fn line() -> Self { Self::new() }

        /// Returns an event-oriented terminal session request.
        #[must_use]
        pub const fn event() -> Self { Self::new().with_line_mode(TermLineMode::Event) }

        /// Returns a raw terminal session request.
        // pure raw, OPOST off
        #[must_use]
        pub const fn raw() -> Self { Self::new().with_line_mode(TermLineMode::Raw) }

        /// Returns an editor-like event-oriented terminal session request.
        ///
        /// Enables bracketed paste and focus reporting, and resets styling on drop.
        #[must_use]
        pub const fn editor() -> Self {
            Self::event()
                .with(Self::BRACKETED_PASTE)
                .with(Self::FOCUS_EVENTS)
                .with(Self::RESET_STYLE_ON_DROP)
        }
        /// Returns an editor-like raw terminal session request.
        ///
        /// Like [`editor`][Self::editor], but control characters are delivered as input
        /// instead of terminal-generated signals.
        // raw input + app reports + processed output
        #[must_use]
        pub const fn raw_editor() -> Self {
            Self::raw()
                .with(Self::PROCESSED_OUTPUT)
                .with(Self::BRACKETED_PASTE)
                .with(Self::FOCUS_EVENTS)
                .with(Self::RESET_STYLE_ON_DROP)
        }
        // pure raw + fullscreen/pixel-ish/manual output
        // pub const fn raw_canvas() -> Self { } // FUTURE
    }

    /// # Preset modifiers
    impl {
        /// Returns `self` with full-screen presentation flags.
        pub const fn fullscreen(self) -> Self {
            self.with(Self::ALT_SCREEN)
                .with(Self::HIDE_CURSOR)
                .with(Self::CLEAR_ON_ENTER)
                .with(Self::RESET_STYLE_ON_DROP)
        }

        /// Returns `self` with normal mouse press/release reporting.
        ///
        /// Uses SGR cell-coordinate encoding.
        /// Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse(self) -> Self {
            self.with(Self::MOUSE).with(Self::MOUSE_SGR)
        }
        /// Returns `self` with mouse press/release reporting, preferring pixel coordinates.
        ///
        /// Enables SGR cell encoding as a fallback for terminals that ignore SGR-pixels.
        /// Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse_pixels(self) -> Self {
            self.with(Self::MOUSE).with(Self::MOUSE_SGR).with(Self::MOUSE_SGR_PIXELS)
        }

        /// Returns `self` with mouse drag reporting.
        ///
        /// Reports movement while a button is held.
        /// Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse_drag(self) -> Self {
            self.with(Self::MOUSE_DRAG).with(Self::MOUSE_SGR)
        }
        /// Returns `self` with mouse drag reporting, preferring pixel coordinates.
        ///
        /// Enables SGR cell encoding as a fallback for terminals that ignore SGR-pixels.
        /// Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse_drag_pixels(self) -> Self {
            self.with(Self::MOUSE_DRAG).with(Self::MOUSE_SGR).with(Self::MOUSE_SGR_PIXELS)
        }

        /// Returns `self` with full mouse motion reporting.
        ///
        /// This can produce many events and should usually be enabled deliberately.
        #[must_use]
        pub const fn mouse_motion(self) -> Self {
            self.with(Self::MOUSE_MOTION).with(Self::MOUSE_SGR)
        }
        /// Returns `self` with full mouse motion reporting, preferring pixel coordinates.
        ///
        /// This can produce many events and should usually be enabled deliberately.
        ///
        /// Enables SGR cell encoding as a fallback for terminals that ignore SGR-pixels.
        #[must_use]
        pub const fn mouse_motion_pixels(self) -> Self {
            self.with(Self::MOUSE_MOTION).with(Self::MOUSE_SGR).with(Self::MOUSE_SGR_PIXELS)
        }
    }
}
