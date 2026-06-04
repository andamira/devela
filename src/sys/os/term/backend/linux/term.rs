// devela::sys::os::term::backend::term
//
//! Defines [`TermLinux`].
//
// TOC
// - struct TermLinux
// - impl TermLinux (public, private)
// - impl traits

use crate::{
    Ansi, TermCap, TermCaps, TermDecModeStatus, TermInputParser, TermLinuxRestore, TermMode,
    TermParsed, TermReply, TermSession, TermSize,
};
use crate::{ColorDepth, Debug, EventKind, NonMaxU16, VersionFull, is};
use crate::{Linux, LinuxError, LinuxResult};
use crate::{
    RunCap, RunCapColor, RunCapImage, RunCapInput, RunCapText, RunCapWindow, RunService,
    RunServiceProbe,
};

#[doc = crate::_tags!(term linux)]
/// Linux terminal frontend.
#[doc = crate::_doc_meta!{
    location("sys/os/term/backend"),
    test_size_of(TermLinux = 64|512),
}]
///
/// Coordinates terminal input parsing, scoped sessions, cached terminal
/// capabilities, cached runtime capabilities, and the last known terminal size
/// over the lower-level [`Linux`] terminal helpers.
#[derive(Clone, Debug)]
pub struct TermLinux {
    parser: TermInputParser,
    term_caps: TermCaps,
    run_cap: RunCap,
    size: Option<TermSize>,
}

// Public methods
impl TermLinux {
    /// Opens a Linux terminal frontend.
    ///
    /// This does not enter raw mode and does not install terminal cleanup guards.
    /// Session discipline belongs to the future terminal session layer.
    pub fn open() -> LinuxResult<Self> {
        let size = Linux::terminal_size().ok();
        let mut term = Self {
            parser: TermInputParser::new(),
            term_caps: Self::default_term_caps(),
            run_cap: RunCap::default(),
            size,
        };
        term.run_cap = Self::derive_run_capabilities(term.term_caps, term.size);
        Ok(term)
    }

    /// Enters a scoped terminal session.
    ///
    /// Applies the requested terminal `mode` changes and
    /// returns a guard that restores the changed state when dropped.
    ///
    /// The requested mode is translated by the backend restore payload,
    /// which records only the changes that must be undone on drop.
    ///
    /// Only successfully applied changes are registered for restoration.
    /// This makes partial setup failures safe: if entering a later mode fails,
    /// earlier changes are still undone as the restore payload is dropped.
    ///
    /// The returned guard does not borrow `self`, so the terminal
    /// frontend can continue to be used while the session is active.
    pub fn session(
        &mut self,
        mode: TermMode,
    ) -> LinuxResult<TermSession<impl Drop + Debug + use<>>> {
        Ok(TermSession::new(TermLinuxRestore::enter(mode)?))
    }
    /// Enters a scoped raw terminal session.
    ///
    /// This is a convenience wrapper around [`session`][Self::session] with [`TermMode::raw`].
    pub fn session_raw(&mut self) -> LinuxResult<TermSession<impl Drop + Debug + use<>>> {
        Ok(TermSession::new(TermLinuxRestore::raw()?))
    }

    /// Polls and parses one pending terminal event, if available.
    ///
    /// This method is non-blocking. It returns `Ok(None)` when no byte is
    /// currently available on stdin.
    pub fn poll_event(&mut self) -> LinuxResult<Option<EventKind>> {
        is! { Linux::available_bytes()? == 0, return Ok(None) }
        let byte = Linux::get_byte()?;
        Ok(self.parser.feed(byte))
    }
    // pub fn wait_event(&mut self) -> LinuxResult<EventKind> { todo![] } // TODO

    /// Writes bytes to stdout.
    pub fn print(&mut self, bytes: &[u8]) -> LinuxResult<()> {
        Linux::print_bytes(bytes)
    }

    /// Returns the last known terminal size.
    pub const fn size(&self) -> Option<TermSize> {
        self.size
    }
    /// Refreshes and returns the terminal size.
    pub fn refresh_size(&mut self) -> LinuxResult<Option<TermSize>> {
        self.size = Linux::terminal_size().ok();
        Ok(self.size)
    }

    /// Returns the last known terminal protocol capabilities.
    pub const fn term_capabilities(&self) -> TermCaps {
        self.term_caps
    }
    /// Probes terminal protocol capabilities exposed by the current terminal endpoint.
    ///
    /// When running inside a terminal multiplexer such as `tmux`, this probes the
    /// multiplexer pseudo-terminal, not necessarily the outer terminal emulator.
    // IMPROVE: add more queries
    pub fn probe_term_capabilities(&mut self) -> LinuxResult<TermCaps> {
        let _guard = Linux::scoped_event_mode()?;
        let mut caps = TermCaps::new();
        /* input */
        // Keyboard
        is! { self.query_dec_private_mode_supported(1000)?, caps.set_cap(TermCap::Mouse) }
        is! { self.query_dec_private_mode_supported(1002)?, caps.set_cap(TermCap::MouseDrag) }
        is! { self.query_dec_private_mode_supported(1003)?, caps.set_cap(TermCap::MouseMotion) }
        is! { self.query_dec_private_mode_supported(1006)?, caps.set_cap(TermCap::MouseSgr); }
        is! { self.query_dec_private_mode_supported(1016)?, caps.set_cap(TermCap::MouseSgrPixels) }
        is! { self.query_dec_private_mode_supported(1004)?, caps.set_cap(TermCap::Focus) }
        is! { self.query_dec_private_mode_supported(2004)?, caps.set_cap(TermCap::BracketedPaste) }
        // resize
        /* output */
        // Ansi
        // Cursor
        // Style
        // x scroll 1007
        // x screen_only 1047
        is! { self.query_dec_private_mode_supported(1049)?, caps.set_cap(TermCap::AltScreen) }
        is! { self.query_dec_private_mode_supported(2026)?, caps.set_cap(TermCap::SyncUpdate) }
        /* image */
        is! { self.query_dec_private_mode_supported(80)?, caps.set_cap(TermCap::Sixel) }
        // KittyImage
        // ItermImage
        /* query */
        // QueryDeviceAttrs,
        // QueryCursorPos,
        // QueryColor,
        self.term_caps = caps;
        Ok(caps)
    }

    /// Returns the last known runtime capabilities.
    pub const fn run_capabilities(&self) -> RunCap {
        self.run_cap
    }
    /// Refreshes runtime capabilities from cached terminal capabilities and current size.
    pub fn refresh_run_capabilities(&mut self) -> LinuxResult<RunCap> {
        self.refresh_size()?;
        self.run_cap = Self::derive_run_capabilities(self.term_caps, self.size);
        Ok(self.run_cap)
    }
    /// Probes terminal capabilities, refreshes size, and recomputes runtime capabilities.
    pub fn probe_run_capabilities(&mut self) -> LinuxResult<RunCap> {
        let _ = self.probe_term_capabilities()?;
        self.refresh_run_capabilities()
    }
}

// Private methods
impl TermLinux {
    /// Returns the conservative baseline assumed before interactive probing.
    const fn default_term_caps() -> TermCaps {
        TermCaps::ANSI_BASE
    }
    const fn palette_size(depth: ColorDepth) -> Option<NonMaxU16> {
        match depth {
            ColorDepth::Palette2 => NonMaxU16::new(2),
            ColorDepth::Palette8 => NonMaxU16::new(8),
            ColorDepth::Palette16 => NonMaxU16::new(16),
            ColorDepth::Palette256 => NonMaxU16::new(256),
            _ => None,
        }
    }

    ///
    fn query_dec_private_mode_supported(&mut self, mode: u16) -> LinuxResult<bool> {
        Ok(self.query_dec_private_mode(mode)?.is_some_and(|s| s.is_supported()))
    }
    ///
    fn query_dec_private_mode(&mut self, mode: u16) -> LinuxResult<Option<TermDecModeStatus>> {
        let mut query = [0u8; 10];
        let query = Ansi::QUERY_DEC_PRIVATE_MODE_N_B(&mut query, mode);
        self.print(query)?;
        self.read_dec_private_mode_reply(mode)
    }

    /// Reads a DEC private mode reply for `mode`, if the terminal answers.
    ///
    /// Expects a DECRPM reply of the form:
    /// `ESC [ ? mode ; status $ y`.
    ///
    /// Returns `Ok(None)` on timeout or when another reply is received.
    fn read_dec_private_mode_reply(&mut self, mode: u16) -> LinuxResult<Option<TermDecModeStatus>> {
        // Conservative probe timeout:
        // 40 × 1ms keeps startup responsive while giving terminal replies time to arrive.
        //
        // This avoids blocking forever on terminals that do not implement DECRQM.
        const TRIES: usize = 40;
        let mut tries = 0;
        while tries < TRIES {
            if let Some(byte) = Linux::try_get_byte() {
                match self.parser.feed_parsed(byte) {
                    TermParsed::Reply(TermReply::DecPrivateMode { mode: reply_mode, status })
                        if reply_mode == mode =>
                    {
                        return Ok(Some(status));
                    }
                    // Other terminal replies may be from previous/proximate probes.
                    // For now we ignore them.
                    TermParsed::Reply(_) => {}
                    // User-facing input during probing.
                    //
                    // If TermLinux eventually has an event queue, this is the place
                    // to preserve it instead of dropping it.
                    TermParsed::Event(_) => {}
                    TermParsed::Pending | TermParsed::Unknown => {}
                }
            } else {
                tries += 1;
                Linux::sleep_ms(1)?;
            }
        }
        Ok(None)
    }

    /// Projects terminal capabilities into runtime capabilities.
    const fn derive_run_capabilities(caps: TermCaps, _size: Option<TermSize>) -> RunCap {
        let depth = caps.color_depth();
        RunCap {
            color: if depth.has_color() {
                Some(RunCapColor {
                    depth,
                    palette_size: Self::palette_size(depth),
                    palette_set: caps.get_query_color() != 0,
                })
            } else {
                None
            },
            image: if caps.get_sixel() != 0
                || caps.get_kitty_image() != 0
                || caps.get_iterm_image() != 0
            {
                Some(RunCapImage {
                    max_bitmap_extent: None,
                    pixel_native: caps.get_kitty_image() != 0 || caps.get_iterm_image() != 0,
                })
            } else {
                None
            },
            input: Some(
                RunCapInput::new()
                    .with_if(caps.get_keyboard() != 0, RunCapInput::KEYBOARD)
                    .with_if(caps.get_mouse() != 0, RunCapInput::MOUSE),
            ),
            text: Some(
                RunCapText::new()
                    .with(RunCapText::WRITE)
                    .with(RunCapText::CELL_GRID)
                    .with_if(caps.get_cursor() != 0, RunCapText::CURSOR)
                    .with_if(caps.get_style() != 0, RunCapText::STYLE)
                    .with_if(caps.get_keyboard() != 0, RunCapText::EDIT)
                    .with_if(depth.has_color(), RunCapText::COLOR_PAIR),
            ),
            audio: None,
            system: None,
            window: if caps.get_resize() != 0 { Some(RunCapWindow { multi: false }) } else { None },
        }
    }
}

/* impl traits */

impl RunService for TermLinux {
    fn run_capabilities(&self) -> RunCap {
        self.run_cap
    }
    fn run_version(&self) -> VersionFull<'_> {
        VersionFull::new(0, 0, 1)
    }
}
impl RunServiceProbe for TermLinux {
    type Error = LinuxError;
    fn run_capabilities_refresh(&mut self) -> Result<RunCap, Self::Error> {
        self.probe_run_capabilities()
    }
}
