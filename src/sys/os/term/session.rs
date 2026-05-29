// devela::sys::os::term::session
//
//! Defines [`TermSession`], [`TermMode`].
//

use crate::TermLineMode;

#[doc = crate::_tags!(term guard)]
/// Scoped terminal session guard.
#[doc = crate::_doc_meta!{location("sys/os/term/session")}]
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

crate::set! {
    #[doc = crate::_tags!(term runtime)]
    /// Terminal session mode request.
    #[doc = crate::_doc_meta!{
        location("sys/os/term/session"),
        test_size_of(TermMode = 2|16),
    }]
    ///
    /// A compact set of terminal state changes requested for a scoped [`TermSession`].
    ///
    /// Backends may ignore unsupported flags,
    /// but should restore any state they successfully changed.
    //
    // Only add here things that are:
    // - session-scoped,
    // - useful across ordinary terminal frontends.
    // - explicitly enabled/disabled by the client,
    // - realistically restorable on drop,
    pub struct TermMode(u16) {
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

        /* future/protocol possible extensions */

        // /// Kitty progressive keyboard protocol.
        // KITTY_KEYBOARD      = 15;
        // /// Kitty drag-and-drop protocol support.
        // KITTY_DND           = 16;
    }
    /// # Line mode accessors
    impl {
        /// Traditional name for `EVENT`.
        pub const CBREAK: Self = Self::EVENT;

        /// Returns the requested terminal line mode.
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
            let mode_cleared = self.without(Self::RAW).without(Self::EVENT);
            match mode {
                TermLineMode::Line => mode_cleared,
                TermLineMode::Event => mode_cleared.with(Self::EVENT),
                TermLineMode::Raw => mode_cleared.with(Self::EVENT).with(Self::RAW),
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
        #[must_use]
        pub const fn raw_editor() -> Self {
            Self::raw()
                .with(Self::BRACKETED_PASTE)
                .with(Self::FOCUS_EVENTS)
                .with(Self::RESET_STYLE_ON_DROP)
        }
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
        /// Uses SGR cell-coordinate encoding. Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse(self) -> Self {
            self.with(Self::MOUSE).with(Self::MOUSE_SGR)
        }
        /// Returns `self` with normal mouse press/release reporting in pixel coordinates.
        ///
        /// Uses SGR-pixels encoding. Mouse motion is not enabled.
        #[must_use]
        pub const fn mouse_pixels(self) -> Self {
            self.with(Self::MOUSE).with(Self::MOUSE_SGR_PIXELS)
        }
        /// Returns `self` with mouse drag reporting.
        ///
        /// Reports movement while a button is held. Free mouse motion is not enabled.
        #[must_use]
        pub const fn mouse_drag(self) -> Self {
            self.with(Self::MOUSE_DRAG).with(Self::MOUSE_SGR)
        }
        /// Returns `self` with full mouse motion reporting.
        ///
        /// This can produce many events and should usually be enabled deliberately.
        #[must_use]
        pub const fn mouse_motion(self) -> Self {
            self.with(Self::MOUSE_MOTION).with(Self::MOUSE_SGR)
        }
    }
}
