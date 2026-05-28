// devela::sys::os::term::session
//
//! Defines [`TermSession`], [`TermMode`].
//

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

        /// Raw line discipline.
        RAW                 = 0;

        /* presentation */

        /// Alternate screen buffer.
        ALT_SCREEN          = 1;
        /// Hidden cursor.
        HIDE_CURSOR         = 2;
        /// Clear terminal on session entry.
        CLEAR_ON_ENTER      = 3;
        /// Clear terminal on session drop.
        CLEAR_ON_DROP       = 4;
        /// Reset styling on session drop.
        RESET_STYLE_ON_DROP = 5;

        /* input reporting */

        /// Bracketed paste reporting.
        BRACKETED_PASTE     = 6;
        /// Focus in/out reporting.
        FOCUS_EVENTS        = 7;
        /// Mouse button press/release reporting.
        MOUSE = 8;
        /// Mouse drag reporting while a button is held.
        MOUSE_DRAG = 9;
        /// Mouse motion reporting, including movement without buttons.
        MOUSE_MOTION = 10;
        /// SGR mouse encoding, with cell coordinates.
        MOUSE_SGR = 11;
        /// SGR mouse encoding, with pixel coordinates.
        MOUSE_SGR_PIXELS = 12;

        /* output transaction */

        /// Synchronized terminal output updates.
        SYNC_UPDATE         = 13;

        /* future/protocol possible extensions */

        // /// Kitty progressive keyboard protocol.
        // KITTY_KEYBOARD      = 14;
        // /// Kitty drag-and-drop protocol support.
        // KITTY_DND           = 15;
    }
    /// # Preset constructors
    impl {
        /// Returns a normal terminal session request.
        #[must_use]
        pub const fn default_mode() -> Self {
            Self::new()
        }
        /// Returns a raw line discipline terminal session request.
        #[must_use]
        pub const fn raw() -> Self {
            Self::new().with(Self::RAW)
        }
        /// Returns an editor-like raw terminal session request.
        pub const fn raw_editor() -> Self {
            Self::raw()
                .with(Self::BRACKETED_PASTE)
                .with(Self::FOCUS_EVENTS)
                .with(Self::RESET_STYLE_ON_DROP)
        }
        /// Returns a full-screen terminal application session request.
        pub const fn fullscreen_app() -> Self {
            Self::raw_editor()
                .with(Self::ALT_SCREEN)
                .with(Self::HIDE_CURSOR)
                .with(Self::CLEAR_ON_ENTER)
                .with(Self::RESET_STYLE_ON_DROP)
        }
    }
    /// # Preset modifiers
    impl {
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
