// devela::sys::os::term::backend::linux
//
//! Linux terminal backend.
//
// TOC
// - struct TermLinux
// - impl TermLinux (public, private)
// - impl traits
// - internal helpers:
//   - set TermLinuxRestoreFlags
//   - struct TermLinuxRestore

use crate::LinuxTermModeGuard;
use crate::{Ansi, TermCaps, TermInputParser, TermLineMode, TermMode, TermSession, TermSize};
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
        term.run_cap = Self::run_cap_from_term(term.term_caps, term.size);
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

    /// Probes and refreshes the known terminal capabilities.
    ///
    /// This first implementation is conservative and does not yet perform
    /// interactive terminal queries. It refreshes size and rebuilds the cached
    /// runtime capability snapshot from the current terminal capability set.
    pub fn probe_capabilities(&mut self) -> LinuxResult<RunCap> {
        self.size = Linux::terminal_size().ok();
        // TODO
        // - send DA / DSR / color queries
        // - parse replies through TermInputParser
        // - update self.term_caps from observed replies
        self.run_cap = Self::run_cap_from_term(self.term_caps, self.size);
        Ok(self.run_cap)
    }
    /// Refreshes and returns the terminal size.
    pub fn refresh_size(&mut self) -> LinuxResult<TermSize> {
        let size = Linux::terminal_size()?;
        self.size = Some(size);
        Ok(size)
    }
    /// Writes bytes to stdout.
    pub fn print(&mut self, bytes: &[u8]) -> LinuxResult<()> {
        Linux::print_bytes(bytes)
    }
    /// Returns the cached terminal capabilities.
    pub const fn term_capabilities(&self) -> TermCaps {
        self.term_caps
    }
    /// Returns the cached terminal size, if known.
    #[must_use]
    pub const fn size(&self) -> Option<TermSize> {
        self.size
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
    /// Projects terminal capabilities into runtime capabilities.
    const fn run_cap_from_term(caps: TermCaps, _size: Option<TermSize>) -> RunCap {
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
        self.probe_capabilities()
    }
}

/* internal helpers */

crate::set! {
    /// Terminal escape-sequence restoration flags.
    ///
    /// These flags only track ANSI/reporting/presentation cleanup.
    /// Termios state is restored separately by the stored line-discipline guard.
    struct TermLinuxRestoreFlags(u16) {
        RESET_STYLE              = 0;
        SHOW_CURSOR              = 1;
        LEAVE_ALT_SCREEN         = 2;
        DISABLE_BRACKETED_PASTE  = 3;
        DISABLE_FOCUS_EVENTS     = 4;

        DISABLE_MOUSE            = 5;
        DISABLE_MOUSE_DRAG       = 6;
        DISABLE_MOUSE_MOTION     = 7;
        DISABLE_MOUSE_SGR        = 8;
        DISABLE_MOUSE_SGR_PIXELS = 9;

        DISABLE_SYNC_UPDATE      = 10;
        CLEAR_ON_DROP            = 11;
    }
}

crate::test_size_of!(TermLinuxRestore = 56 | 448);
/// Linux terminal session restore payload.
///
/// This is the synchronization point between [`TermMode`] and Linux-specific
/// terminal state changes. It applies termios line mode changes and ANSI
/// reporting/presentation requests, recording the corresponding restoration
/// work as each step succeeds.
#[derive(Debug)]
struct TermLinuxRestore {
    guard: Option<LinuxTermModeGuard>,
    flags: TermLinuxRestoreFlags,
}
impl TermLinuxRestore {
    fn none() -> Self {
        Self { guard: None, flags: TermLinuxRestoreFlags::new() }
    }
    fn raw() -> LinuxResult<Self> {
        Self::enter(TermMode::raw())
    }
    fn enter(mode: TermMode) -> LinuxResult<Self> {
        let mut restore = Self::none();

        /* line discipline */
        match mode.line_mode() {
            TermLineMode::Line => {}
            TermLineMode::Event => {
                restore.guard = Some(Linux::scoped_event_mode()?);
            }
            TermLineMode::Raw => {
                restore.guard = Some(Linux::scoped_termios_update(|state| {
                    state.make_raw();

                    if mode.has_processed_output() {
                        state.set_output_processing(true);
                    }
                })?);
            }
        }

        /* presentation */
        if mode.has_alt_screen() {
            Linux::print_bytes(&Ansi::ENABLE_ALT_SCREEN_B)?;
            restore.flags = restore.flags.with_leave_alt_screen();
        }
        if mode.has_hide_cursor() {
            Linux::print_bytes(&Ansi::CURSOR_INVISIBLE_B)?;
            restore.flags = restore.flags.with_show_cursor();
        }

        /* input reporting */
        if mode.has_bracketed_paste() {
            Linux::print_bytes(&Ansi::ENABLE_BRACKETED_PASTE_B)?;
            restore.flags = restore.flags.with_disable_bracketed_paste();
        }
        if mode.has_focus_events() {
            Linux::print_bytes(&Ansi::ENABLE_FOCUS_EVENTS_B)?;
            restore.flags = restore.flags.with_disable_focus_events();
        }

        /* mouse: enable encoding before reporting */
        if mode.has_mouse_sgr_pixels() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_SGR_PIXELS_B)?;
            restore.flags = restore.flags.with_disable_mouse_sgr_pixels();
        } else if mode.has_mouse_sgr() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_SGR_B)?;
            restore.flags = restore.flags.with_disable_mouse_sgr();
        }

        if mode.has_mouse_motion() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_MOTION_B)?;
            restore.flags = restore.flags.with_disable_mouse_motion();
        } else if mode.has_mouse_drag() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_DRAG_B)?;
            restore.flags = restore.flags.with_disable_mouse_drag();
        } else if mode.has_mouse() {
            Linux::print_bytes(&Ansi::ENABLE_MOUSE_B)?;
            restore.flags = restore.flags.with_disable_mouse();
        }

        /* screen clearing */
        if mode.has_clear_on_enter() {
            Linux::print_bytes(&Ansi::ERASE_SCREEN_B)?;
            Linux::print_bytes(&Ansi::CURSOR_HOME_B)?;
        }
        if mode.has_clear_on_drop() {
            restore.flags = restore.flags.with_clear_on_drop();
        }

        /* output transaction */
        if mode.has_sync_update() {
            Linux::print_bytes(&Ansi::ENABLE_SYNC_UPDATE_B)?;
            restore.flags = restore.flags.with_disable_sync_update();
        }

        Ok(restore)
    }
}
impl Drop for TermLinuxRestore {
    fn drop(&mut self) {
        /* style reset */
        if self.flags.has_reset_style() {
            let _ = Linux::print_bytes(&Ansi::RESET_B);
        }

        /* input reporting */
        if self.flags.has_disable_bracketed_paste() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_BRACKETED_PASTE_B);
        }
        if self.flags.has_disable_focus_events() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_FOCUS_EVENTS_B);
        }

        /* mouse: disable reporting before encoding */
        if self.flags.has_disable_mouse_motion() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_MOTION_B);
        }
        if self.flags.has_disable_mouse_drag() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_DRAG_B);
        }
        if self.flags.has_disable_mouse() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_B);
        }
        //
        if self.flags.has_disable_mouse_sgr_pixels() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_SGR_PIXELS_B);
        }
        if self.flags.has_disable_mouse_sgr() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_MOUSE_SGR_B);
        }

        /* screen clearing */
        if self.flags.has_clear_on_drop() {
            let _ = Linux::print_bytes(&Ansi::ERASE_SCREEN_B);
            let _ = Linux::print_bytes(&Ansi::CURSOR_HOME_B);
        }
        /* presentation */
        if self.flags.has_show_cursor() {
            let _ = Linux::print_bytes(&Ansi::CURSOR_VISIBLE_B);
        }
        if self.flags.has_leave_alt_screen() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_ALT_SCREEN_B);
        }
        /* output transaction */
        if self.flags.has_disable_sync_update() {
            let _ = Linux::print_bytes(&Ansi::DISABLE_SYNC_UPDATE_B);
        }
        // `guard` drops after this and restores termios.
    }
}
