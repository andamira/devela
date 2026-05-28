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
        test_size_of(TermMode = 1|8),
    }]
    ///
    /// A compact set of terminal state changes requested for a scoped
    /// [`TermSession`].
    ///
    /// Backends may ignore unsupported flags, but should restore any state they
    /// successfully changed.
    pub struct TermMode(u8) {
        /// Raw line discipline.
        RAW             = 0;
        /// Alternate screen buffer.
        ALT_SCREEN      = 1;
        /// Hidden cursor.
        HIDE_CURSOR     = 2;
        /// Bracketed paste reporting.
        BRACKETED_PASTE = 3;
        /// Mouse reporting.
        MOUSE           = 4;
        /// Clear terminal on session entry.
        CLEAR_ON_ENTER  = 5;
        /// Clear terminal on session drop.
        CLEAR_ON_DROP   = 6;
        /// Synchronized terminal output updates.
        SYNC_UPDATE     = 7;
    }
    /// # Presets
    impl {
        /// Normal terminal session request.
        pub const DEFAULT: Self = Self::new();

        /// Raw line discipline.
        pub const RAW_MODE: Self = Self::new().with(Self::RAW);

        /// Raw mode for editor-like input.
        pub const RAW_EDITOR: Self = Self::new()
            .with(Self::RAW)
            .with(Self::BRACKETED_PASTE);

        /// Raw mode for full-screen terminal applications.
        pub const FULLSCREEN_APP: Self = Self::new()
            .with(Self::RAW)
            .with(Self::ALT_SCREEN)
            .with(Self::HIDE_CURSOR)
            .with(Self::CLEAR_ON_ENTER);

        /// Returns a normal terminal session request.
        #[must_use]
        pub const fn default_mode() -> Self { Self::DEFAULT }

        /// Returns a raw terminal session request.
        #[must_use]
        pub const fn raw() -> Self { Self::RAW_MODE }

        /// Returns an editor-like raw terminal session request.
        #[must_use]
        pub const fn raw_editor() -> Self { Self::RAW_EDITOR }

        /// Returns a full-screen terminal application session request.
        #[must_use]
        pub const fn fullscreen_app() -> Self { Self::FULLSCREEN_APP }
    }
}
